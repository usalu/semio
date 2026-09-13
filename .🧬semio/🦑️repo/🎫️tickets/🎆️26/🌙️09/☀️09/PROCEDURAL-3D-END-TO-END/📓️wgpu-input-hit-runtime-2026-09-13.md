# 🖱️ wgpu INPUT / HIT / DISPATCH AT RUNTIME — what 6118 actually does, and the one hop that stops it

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane **wgpu-input-hit-runtime**, 2026-09-13.
Target: `http://127.0.0.1:6118/?plugin=generation3d&mode=generate`, the coordinator's wgpu serve.

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`);
no ticket was opened, closed or reopened, `📓️status.md` and `🎫️ticket.json` were not touched. No
git-state-modifying command was run. No dev server was started or stopped. Nothing under
`🗑️generated` that this lane did not create was touched.

---

## 1. TL;DR

| question | answer |
|---|---|
| **Was any of the six deliverable hops proven on 6118?** | **No. None of them, and none is claimed.** Hover, click-selection, wheel-orbit, chrome-control dispatch, retained tree-row dispatch and keyboard chords are all **unreachable**, for one reason that sits upstream of every one of them. |
| **Why** | `boot_shell` never returns. The page stops at `shell-boot 86%`, and the canvas's DOM pointer listeners are installed only after boot completes — so **zero** input events exist to route. Measured across five runs: `os_host handle_event` = 0, `os_host pointer hit` = 0, `dispatch_normalized_event` = 0, `dock plan` = 0, over 65–86 s of continuous pointer nudging. The entire hit/dispatch chain this lane was asked to prove is downstream of a hop that never fires. |
| **What was wedging boot when this lane started** | Two defects, stacked. **(A)** The app-static catalogue's document fails to parse, and the shell recorded its cache key only on SUCCESS — so a refresh re-fetched it, and a refresh runs on **every settled command**. The SECOND fetch of `framework.section.catalogue` never returned from its own turn and wedged boot at 4.9 s. **(B)** That failure was **invisible**: it was reported through `eprintln!`, which is a no-op in a `wasm32-unknown-unknown` Worker. 27 diagnostics in the wgpu shell were dead this way. |
| **What this lane fixed** | Both. §3: the 27 dead traces now reach `console.log` through the file's own `debug_log` sink — which named defect (A)'s root cause on the first run after the fix, quoted verbatim in §3.2. §4: `claim_app_catalogue_fetch` makes the catalogue's "once per app instance" rule hold on **every** outcome, so a failed catalogue is never re-fetched. Fixture + Rust law (**2 passed**) + TypeScript twin (**12 passed**), §5. |
| **What that bought, measured** | Boot advances past the catalogue. The second `framework.section.catalogue` render is gone (one fetch per boot, §6.3), and both refresh passes now complete cleanly where the second one used to hang mid-render. |
| **What still blocks every hop** | The **second `flowEvalTick` never settles** (§6.4). `settle_ui_chain` round 0 dispatches it, gets `settled effects=0 mutations=0`, re-renders every surface, and round 1's dispatch never returns — so `flush_deferred_actions` never returns, `settle_boot` never returns, `boot_shell` never returns. This is the guest's typed-operation retirement, **not this lane's layer**, and it is already named by `📓️audit-regression-diff-2026-09-13.md` §0 (`"registered fixture typed operation did not retire within 30 seconds"`, from `flowEvalTick`). This report adds the browser-side confirmation that audit did not have. |
| **Code this lane was asked to own** | The input/hit/dispatch/chrome-action code was **not** modified — there was no evidence on which to change it, and no hop's failure was attributable to it. Changing it would have been guessing. |

---

## 2. The hop, and where it dies

```
boot_shell ─► settle_boot ─► push_contributions ─► refresh_ui (pass 1) ✅ 7 windows + 7 panels
                  │
                  ├─ refresh_app_catalogue ──► render framework.section.catalogue
                  │        │                     turn 0,1,2 → document published ✅
                  │        └─ read back ──────► ❌ UiFixedMap cap exceeded (§3.2)
                  │              │
                  │              ├─ WAS: recorded only on success → re-fetch every refresh → the
                  │              │        SECOND fetch hangs → boot wedged at 4.9 s     ← FIXED §4
                  │              └─ NOW: attempt recorded once per instance → one fetch ✅
                  │
                  └─ flush_deferred_actions ─► settle_ui_chain
                          round 0: flowEvalTick ─► settled ✅ ─► refresh_ui (pass 2) ✅
                          round 1: flowEvalTick ─► ❌ NEVER SETTLES                    ← BLOCKER, not this layer
                                                        │
   ════════════════════════════ boot_shell never returns ═══════════════════════════
                                                        │
   canvas pointer listeners never installed ─► 0 DOM events ─► 0 handle_event
                                                        │
   ShellState::handle_pointer_* ─► InputState::hit_at ─► retained registry ─► dispatch_action
   └────────────── every hop this lane was asked to prove lives here, unreachable ──────────────┘
```

---

## 3. Deliverable 1 — the wgpu shell's diagnostics were dead, and that is why this took a rebuild to see

### 3.1 The defect

`ShellState` carries its own trace sink, `debug_log` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3497`), which
logs to `web_sys::console` under `wasm32` and to `eprintln!` natively. **27 diagnostics in the same
file bypassed it and called `eprintln!` directly** — a no-op in a `wasm32-unknown-unknown` Worker,
where nothing is attached to stderr. The file already knew this: the comment at `record_surface_fault`
said so verbatim for one call site and the rest were never swept.

The cost is not theoretical. `refresh_app_catalogue`'s two failure arms were among the 27, so **the
catalogue has been failing on every single wgpu boot with no trace whatsoever**, which is exactly why
three previous lanes on this target read the symptom as "boot is slow" rather than "a fetch failed".

### 3.2 The fix, and what it printed on the first run afterwards

All 27 converted to `Self::debug_log(&format!(…))` (paren-matched by
`<ticket>/🐍️eprintln-to-debug-log.py`, which skips `debug_log`'s own native arm by CONTEXT, never by
line number, so a concurrent edit cannot shift it). One `eprintln!` remains in the file — the one
inside `debug_log`. The comment at `record_surface_fault` was generalised into the file's rule.

First run after the rebuild, `🗑️generated/wgpu-input/boot-generate-3/console.txt:502` — the root
cause of defect (A), visible for the first time:

```
11680 log [DEBUG] wgpu shell app catalogue fetch failed: renderDocument result parse failed:
UiFixedMap requires at most 32 ascending unique entries at line 1 column 21173
headroom collections=124 items=403
near :1.5707963267948966}","cardinality":"!"}],"outputs":[{"code":"C","abbreviation":"Crv",…
```

**This is a real, separate defect and it is NOT fixed here** — the catalogue document the guest
publishes carries a map node with more than `UiFixedMap`'s 32 entries. It is named, loudly traced,
and left to whoever owns the catalogue document's encoding. What this lane fixed is the far worse
consequence it used to have (§4).

---

## 4. Deliverable 2 — "once per app instance" now holds on every outcome

### 4.1 The defect

`refresh_app_catalogue`'s own first doc line says the catalogue is fetched *once per app instance*.
The body kept that promise only on the success path: `app_catalogue_instance` was assigned **after**
a successful reassembly. With the fetch failing (§3.2), the key was never recorded, and since
`refresh_ui` runs on every settled command, every refresh re-entered the fetch.

The second fetch is where boot died. From `🗑️generated/wgpu-input/boot-generate-2/console.txt`, the
pass-1 fetch (lines 468–501) publishes at turn 2 and returns; the pass-2 fetch stops dead:

```
12616  renderSurface enter surface=framework.section.catalogue body=framework.section.catalogue actor=procedural#1
12641  renderSurface surface-visible returned surface=framework.section.catalogue patches=0 status={"tag":"more-work"}
12642  renderSurface accept begin surface=framework.section.catalogue turn=0 patches=0
12642  renderSurface accept end   surface=framework.section.catalogue turn=0 supplemental=0 intakeSteps=386187
12642  renderSurface surface=framework.section.catalogue turn=0 effects=0 carried=0 tags=- intakeSteps=386187
       ── 60.5 s of total console silence, then the probe gave up ──
```

The next statement after that log is `submitTurn(actorId, [])` (`🐚️plugin-bridge/🟦️.ts`'s turn
loop). It never resolved.

### 4.2 The fix, at the owning layer

`claim_app_catalogue_fetch` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:2829`) is now the single owner of the
rule, and `refresh_app_catalogue` (`:3965–3966`) is its only caller:

```rust
fn claim_app_catalogue_fetch(recorded: &mut Option<u32>, instance_id: u32) -> bool {
    if *recorded == Some(instance_id) {
        return false;
    }
    *recorded = Some(instance_id);
    true
}
```

`recorded` is the instance the catalogue was last **attempted** for, not the one it last succeeded
for. Claiming records it *before* the fetch runs, so the fetch's outcome cannot change how many
fetches happen. A different instance still re-claims, so switching app instance still refetches
exactly once — "once per instance" did not become "once per process".

This is deliberately **not** a workaround for §3.2: a catalogue that fails still leaves the palette
empty and still says so loudly. What it removes is the unbounded retry that turned a contained
failure into a dead application.

---

## 5. Deliverable 3 — the fixture and the two implementations that answer it

`🧑‍🎨engine/🧫️fixtures/🛍️app-catalogue-attempt/🔣️.json` — language-neutral. It carries the rule's
statement, the three outcomes, and five cases. Each case carries BOTH `expectedFetchIndices` (the
rule) and `baselineFetchIndices` (what recording-only-successes produced), plus a `discriminates`
flag — so the fixture is held to actually telling the two rules apart rather than merely asserting
the new one.

| case | pins |
|---|---|
| `a-catalogue-that-reassembles-is-fetched-once` | the unbroken path is unchanged |
| `a-catalogue-whose-reassembly-fails-is-still-fetched-once` | **the defect** — pre-fix `[0,1,2,3]`, rule `[0]` |
| `a-catalogue-whose-fetch-faults-is-still-fetched-once` | the other failure arm — pre-fix `[0,1,2]`, rule `[0]` |
| `a-new-app-instance-refetches-exactly-once` | the fix did not make it once-per-process |
| `a-new-instance-refetches-once-even-after-a-failure` | a failed instance poisons neither the next one nor itself — pre-fix `[0,1,2,3,4]`, rule `[0,2,4]` |

**Rust** — `🐚️Shell/🧪️tests/🛍️app-catalogue-attempt/🦀️.rs`, mounted at `🦀️.rs:2840`, drives the
production predicate and re-derives the pre-fix rule alongside it.

```
RUST_MIN_STACK=33554432 CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-renderer-wgpu --lib \
  -- app_catalogue_attempt_tests:: --test-threads=1 --nocapture
```

```
[DEBUG] app-catalogue-attempt a-catalogue-that-reassembles-is-fetched-once: fetches [0] recorded=Some(1)
[DEBUG] app-catalogue-attempt a-catalogue-whose-reassembly-fails-is-still-fetched-once: fetches [0] recorded=Some(1)
[DEBUG] app-catalogue-attempt a-catalogue-whose-fetch-faults-is-still-fetched-once: fetches [0] recorded=Some(1)
[DEBUG] app-catalogue-attempt a-new-app-instance-refetches-exactly-once: fetches [0, 2] recorded=Some(2)
[DEBUG] app-catalogue-attempt a-new-instance-refetches-once-even-after-a-failure: fetches [0, 2, 4] recorded=Some(1)
[DEBUG] app-catalogue-attempt a-catalogue-whose-reassembly-fails-is-still-fetched-once: pre-fix [0, 1, 2, 3] vs rule [0]
[DEBUG] app-catalogue-attempt a-catalogue-whose-fetch-faults-is-still-fetched-once: pre-fix [0, 1, 2] vs rule [0]
[DEBUG] app-catalogue-attempt a-new-instance-refetches-once-even-after-a-failure: pre-fix [0, 1, 2, 3, 4] vs rule [0, 2, 4]

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 529 filtered out
```

Compiled in `5m 38s` with **43 warnings emitted** — quoted because zero errors means nothing if the
expansion never ran.

**TypeScript twin** — `🧑‍🎨engine/🧪️tests/🛍️app-catalogue-attempt/🟦️.ts`, registered in
`🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`. Re-derives both rules independently from the same fixture.

```
bunx vitest run --config "…/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts" "🧪️tests/🛍️app-catalogue-attempt/🟦️.ts"
  Test Files  1 passed (1)
       Tests  12 passed (12)
```

---

## 6. Deliverable 4 — 6118, run by run

Probe: `<ticket>/🐍️wgpu-boot-witness-probe.mjs`, written this lane. It nudges the pointer twice a
second for the whole window — the wgpu host ticks on input, so a probe that does not nudge measures a
runtime that was never asked to run (`🐍️console-dump-probe.mjs` prints **5 lines in 170 s** on 6118
for exactly that reason, `🗑️generated/wgpu-input/wedge-1/`). It stops on `boot_shell leave` and
reports the last line before the silence and the length of that silence.

| run | binary | `boot_shell leave` | last line before silence | silence |
|---|---|---|---|---|
| `run-1` (`🐍️wgpu-hit-probe.mjs`) | 12:12 wasm | never | `renderSurface … section.catalogue turn=0` (2nd fetch) | 66 s |
| `boot-generate` | 12:12 wasm | never | same, reproduced exactly | 86 s |
| `boot-generate-2` | 12:12 wasm + peer's bridge traces | never | same, now with `accept begin/end` bracketing | 60 s |
| `boot-generate-3` | **+ §3 + §4 fixes** | never | `render leave surface=framework.panel.history` | 81 s |
| `boot-generate-4` | + temporary refresh-tail traces | never | `refresh_ui leave` (**both passes complete**) | 75 s |
| `boot-generate-final` | shipped tree, traces removed | never | `render leave surface=framework.panel.history` | 65 s |

### 6.1 Nothing in this lane's own chain ever fired

Every run, all six: `os_host handle_event` **0**, `os_host pointer hit` **0**,
`dispatch_normalized_event` **0**, `dock plan` **0**, `world3d surface` **0**, `frame build` **0**.
`shot.png` in each run directory shows `shell-boot 86%` over bare `#001117` ground.

The probe's own pointer-delivery witness in `run-1` confirms the canvas exists and is full-viewport
(`{"x":0,"y":0,"width":1440,"height":900,"tag":"CANVAS"}`) with `downs: 0` — the listeners are simply
not attached yet, because attaching them is downstream of `boot_shell`.

### 6.2 The catalogue failure is now visible

`boot-generate-3` and later: exactly **one** `wgpu shell app catalogue fetch failed` line per boot,
quoted in §3.2. Before the §3 fix this line existed in the binary's string table and reached nothing.

### 6.3 The catalogue is now fetched once

`boot-generate-final/console.txt`: one `framework.section.catalogue` render block (turns 0, 1, 2,
publishing at turn 2), and **no second one**. Before the §4 fix there were two, and the second never
returned. The `grep -c "catalogue fetch failed"` count is `1`.

### 6.4 The remaining blocker, with its trace

`boot-generate-4` bracketed `refresh_ui`'s tail with temporary traces (since removed, §7). Both
passes complete:

```
8272  [DEBUG] refresh_ui leave
8273  [DEBUG] boot-phase shell-boot:refresh-ui 1719 ms          ← settle_boot enters flush-deferred
8284  [DEBUG] wgpu-bridge typed-operation command instance=1 pages=2 terminal=true
8284  [DEBUG] wgpu-shell command flowEvalTick settled effects=0 mutations=0   ← settle round 0
8288 …8405  ── refresh_ui pass 2: 7 windows + 7 panels, all render begin/leave ──
8405  [DEBUG] refresh_ui leave
      ── 74.6 s of silence. settle round 1 dispatches flowEvalTick and it never settles. ──
```

`settle_ui_chain` (`🦀️.rs:7363`) round 1 calls `drain_deferred_actions` (`:7382`), whose
`dispatch_action` for the second `flowEvalTick` never returns. No `wgpu-shell command flowEvalTick
settled` for it, no `ui chain settled after N round(s)`, no `ui chain exhausted`, no
`deferred chain exhausted`, no `wgpu-shell invoke` — the dispatch is simply suspended.

This is the same defect `📓️audit-regression-diff-2026-09-13.md` §0 measured natively as
`flowEvalTick: Fault { message: "registered fixture typed operation did not retire within 30 seconds" }`
and ranked its TOP candidate for (`RetainedInflateHistory` demanding its full 32 KiB window in one
`try_reserve_exact` while the loader only ever offers 4 KiB, making silent zero progress forever).
**This report contributes the browser-side confirmation**: it is the SECOND `flowEvalTick` of a boot,
the first settles normally, and it is what holds `boot_shell` open.

It is not this lane's layer and was not touched.

---

## 7. Files

**Changed**

| file | what |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | 27 dead `eprintln!` diagnostics → `Self::debug_log` (61 call sites now; the 1 remaining `eprintln!` is `debug_log`'s own native arm at `:3497`); the file's trace rule written into `record_surface_fault`'s comment; new region `🛍️AppCatalogueAttempt` with `claim_app_catalogue_fetch` at `:2829`; the law mounted at `:2840`; `refresh_app_catalogue` at `:3965` claims through it and its doc records why |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts` | registers the twin |

**Added**

| file | what |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🛍️app-catalogue-attempt/🔣️.json` | the neutral oracle — 5 cases, rule + pre-fix baseline per case |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🛍️app-catalogue-attempt/🦀️.rs` | the Rust law over the production predicate |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🛍️app-catalogue-attempt/🟦️.ts` | the TypeScript twin |
| `<ticket>/🐍️wgpu-boot-witness-probe.mjs` | the nudging boot witness §6 |
| `<ticket>/🐍️eprintln-to-debug-log.py` | the paren-matching converter §3.2 |

**Temporary, added and removed again**: four `[DEBUG] refresh_ui tail:` / `[DEBUG] refresh_ui leave`
traces that produced §6.4's evidence. The shipped `dist/wasm-dev` was rebuilt after their removal, so
source and served binary agree.

**Generated (disposable, `<ticket>/🗑️generated/wgpu-input/`)**: `run-1/`, `wedge-1/`,
`boot-generate/`, `boot-generate-2/`, `boot-generate-3/`, `boot-generate-4/`, `boot-generate-final/`
(each `console.txt`, `verdict.json`, `shot.png`), plus `rust-law.txt` and `wasm-build.txt`.

### Peer breakages run through, attributed, not worked around

1. `semio-s-artifact-stdio-semio` was red at 12:48 (`cannot find type Rational`, 4 errors) — a peer's
   in-flight rationals→expansions migration in `…/🧊️brep/🧬️schema/📸️snapshot/➡️vector/⚖️predicates/🦀️.rs`.
   It completed on its own at 12:49:03. **Nothing was touched**; the build was simply re-run.
2. `semio-framework-os-renderer-wgpu` was red at 12:54 (`HitKind::{NumberStepper, Ring, IconSelect}`
   not found). Not a real break: a peer added those variants to the enum at 12:53 and used them at
   12:54, while my build had STARTED at 12:50 and compiled the tree as of its start. Re-running was
   the whole fix.
3. A peer is concurrently editing `route_retained_pointer_press` — this lane's own function — to route
   `Toggle`/`Slider`/`NumberStepper`/`Ring`/`IconSelect` presses into the retained router
   (`📓️audit-wgpu-parity-2026-09-13.md` gaps #1/#6). **Left entirely alone**; it closes this lane's
   predecessor's §8 "a Toggle dispatches nothing" gap and would have collided.

---

## 8. What is NOT claimed

* **All six brief deliverables are unproven.** Hover on the World3d mesh, click→`selectedIds`,
  wheel→`setCamera`, a chrome control's action, a retained tree row's `addGeneration`, and keyboard
  chords: **none** was observed, because no pointer, wheel or key event reaches the shell at all
  while `boot_shell` is open. Not one of them is claimed as working, and not one is claimed as broken
  either — they were never reachable enough to test.
* **No input/hit/dispatch/chrome-action code was changed.** There was no evidence implicating it.
  The predecessor lanes' native laws for that chain still stand on their own reports; this lane
  neither re-ran nor extended them.
* **The `UiFixedMap` 32-entry defect (§3.2) is not fixed**, only made visible and named. The catalogue
  is empty on wgpu until someone owns it, which means the node-graph palette/spotlight is empty too.
* **The `flowEvalTick` retirement wedge (§6.4) is not fixed**, only confirmed in the browser and
  attributed. It is the single blocker for this lane and for every runtime deliverable behind it.
* **`?mode=generate` only.** The default mode was not separately characterised.
* **The `📓️audit-regression-diff-2026-09-13.md` ranking was not verified.** §6.4 confirms the
  symptom that audit predicted; it does not confirm its `RetainedInflateHistory` root cause.
