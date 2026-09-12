# 🖼️ wgpu RASTER-WITNESS + GUEST EFFECTS — the surface paints, and the guest's chain now starts

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane "wgpu raster witness + guest effects", 2026-09-12.
Resumes `📓️wgpu-document-reconcile-2026-09-12.md` §6 (deliverable A) and §7 (deliverable B).

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`); no
ticket was opened, closed or reopened — bookkeeping is on disk. Evidence under
`🗑️generated/wgpu-raster/run-1 … run-10`; nothing under any `🗑️generated` folder was swept BY THIS
LANE. (Noted 22:47 the same day: a later lane wiped the whole `🗑️generated` tree and started its own
`wgpu-settle/` in it, so those ten run directories — consoles, `samples.json`, screenshots — no longer
exist and are unrecoverable, since `🗑️generated` is gitignored. Every measurement quoted below was read
off them while they were live; the quoted lines are verbatim.) The react
serve on 6018 was not touched, the procedural guest was NOT rebuilt or restaged, and no git-state
modifying command was run.

---

## 1. TL;DR

| question | answer |
|---|---|
| **A — is the `raster commit candidate witness was stale` quarantine gone?** | **Yes.** Root-caused (§2), fixed structurally (§3), pinned by a fixture-driven Rust law (8/8) and a TS twin (11/11), and proven on 6118: **no quarantine, no fault banner, on every boot since**. |
| **A — does the document paint?** | **Yes.** `dumpFrameStats {windowId: "procedural-main", drawCalls: 2, quadCount: 620, glyphCount: 575}` against the corrected paint census, 34 arena nodes, viewport `1433.6×836`, and a screenshot showing the Flow outline tree + the node-graph canvas with wires and minimap (§5, `run-10/shot-030s.png`). |
| **B — do guest effects reach the host now?** | **Partly, and much further than before.** `renderSurface … effects=0` → **`effects=1 tags=dispatchAction`**; the re-armed `flowEvalTick` is decoded with its real name, routed as an app COMMAND, and reaches the guest's command door. It still fails there, on a wire-decode defect this lane did NOT fix: `handleCommand promise failed: decodeAppFrame: unknown tag 115` (§7). |
| **B — `invokeExtension` / meshes?** | **No.** `invokeExtension 0×`, `respond 0×`, meshes 0. The chain dies one hop earlier, at §7. **This lane does not claim the hexagonal column reached the wgpu World3d surface.** |
| Defects fixed on the way | **Six**, all named and each verified on 6118 by the probe that exposed it (§3, §6). Four of them are host divergences from React's `ShellHost`; one is a peer's trunk regression that had made the whole 6118 bundle unbootable since 11:17; one is a shared wire-projection bug affecting BOTH renderer targets. |

---

## 2. Deliverable A — the witness lifecycle, root-caused

`commit_presented_step(witness)` is a bounded STEP: `AppPresentedRetirement::step` calls it once per
present opportunity until it answers `true`. Its first call admitted the retirement and started the
cursor; **every later call re-validated the candidate and presentation witnesses against the raw
slots** — the very slots that cursor was consuming.

`RasterTextureWitnessSlot` retires ONE field per step (`scene_revision`, then `preview_generation`,
then `operation`), and `get()` answers `Some` only when all three are present. So on the step right
after the cursor took the candidate's first field:

* `self.candidate.get()` → `None` (half-consumed)
* `self.candidate.is_empty()` → `false` (two fields still there)
* → `Err("raster commit candidate witness was stale")`

**The commit refused its own progress.** Deterministic, on the first frame that ever staged an engine
raster — which is why it only became reachable when the document reconcile lane made a document
composite an `engine:<surface>` raster for the first time.

Two more of the same shape were sitting behind it and would have fired next:

* `retirement_step`'s tail re-read `self.presenting.get() != Some(witness)` on every step **after**
  `presenting.retire_one()` had begun — `Err("raster retirement presentation witness was stale")`.
* `abort_presented_step` did the same, and additionally FABRICATED a presentation
  (`if self.presenting.is_empty() { self.presenting.set(witness) }`) for an operation that had never
  presented one.

---

## 3. Deliverable A — the fix

### 3.1 One device-free authority owns the witness pair

`🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs` gains `RasterOperationWitnessLedger`: the candidate slot, the
presentation slot, the operation whose retirement owns them (`retiring`), and the two retired-field
flags. `RasterTextureTable` keeps the staged-table scan (`RasterTextureRetirementCursor` is now
`{mode, scan, owner}` — the retired-field bookkeeping left it) and delegates every witness question.

The ledger owns no `wgpu::Device`, which is the point: **the law drives the production authority
itself**, not a replica.

### 3.2 Admission is a one-time fact, and it is TOTAL

```rust
pub fn retirement_admission(&self, witness) -> Result<RasterWitnessAdmission, &'static str> {
    if let Some(retiring) = self.retiring {
        return if retiring == witness { Ok(Retire) } else { Err("raster retirement authority was occupied") };
    }
    if self.candidate.is_empty() && self.presenting.is_empty() { return Ok(Nothing); }
    …
}
```

* **An operation never reads as stale against its own retirement**, however far that retirement has
  consumed the slots. That is §2's defect, closed at the authority rather than at each caller.
* **A missing raster is `Nothing`, never "stale"** — an engine surface whose first raster has not been
  reserved yet owes nothing at all. That is the brief's "not yet, never stale", made a verdict of the
  type system rather than a comment.
* An operation that reserved a candidate but never armed a presentation retires its candidate ALONE;
  an empty presentation slot is admitted, not refused.

`begin_retirement(witness)` is idempotent for its owner, so a bounded step may re-enter as often as
its budget requires; `retire_step()` clears `retiring` at the terminal so the ledger is reusable.
`commit_presented_step` / `abort_presented_step` are now three lines each over that one door, and the
per-step re-read in `retirement_step` is gone.

### 3.3 A pending metrics invalidation is "not yet" too

With §3.2 in, 6118 quarantined one stage earlier instead:

```
worker-present-failed: engine canvas present: engine primary metrics invalidation is pending
Surface: quarantined · Boot stage: ready            (run-2, t ≈ 9 s, every boot)
```

`EngineCanvasPresenter::realize_step` returned a terminal `Err` whenever
`metrics_invalidation_scan.is_some()`. But that scan is **this authority's own unfinished work**: the
surface-resize cursor (`AppSurfaceResizePhase::{Apply,InvalidateEngine}`) arms it and drains it on a
different cadence from the present cursor, so a frame that reaches the engine mid-scan quarantined the
whole surface for a condition that resolves in at most `ENGINE_SURFACE_CAPACITY + 1` steps. It now
drives one scan unit and answers `Ok(false)`.

---

## 4. Deliverable A — the laws (both run, verbatim)

### 4.1 The Rust law, driving the production ledger

`🖱️ui/🧪️tests/🖼️raster-witness-lifecycle/🦀️.rs` (new, mounted from `🖍️draw/🦀️.rs`), reading the
neutral fixture `🖱️ui/🧫️fixtures/🖼️raster-witness-lifecycle/🔣️.json` (new).

```
$ cargo test -p semio-framework-ui --features wgpu-engine --lib -- raster_witness_lifecycle
running 8 tests
test wgpu::draw::raster_witness_lifecycle_tests::a_commit_stays_admissible_at_every_step_of_the_retirement_it_owns ... ok
test wgpu::draw::raster_witness_lifecycle_tests::a_surface_with_no_raster_yet_owes_nothing_and_never_reads_as_stale ... ok
test wgpu::draw::raster_witness_lifecycle_tests::an_operation_that_reserved_but_never_presented_retires_its_candidate_alone ... ok
test wgpu::draw::raster_witness_lifecycle_tests::the_fixtures_step_arithmetic_is_the_authoritys_own ... ok
test wgpu::draw::raster_witness_lifecycle_tests::a_foreign_operation_is_refused_by_name_and_the_ledger_is_left_untouched ... ok
test wgpu::draw::raster_witness_lifecycle_tests::a_presentation_arms_only_its_own_candidate_and_only_once ... ok
test wgpu::draw::raster_witness_lifecycle_tests::one_frames_rasters_share_one_candidate_and_a_foreign_one_is_named_occupied ... ok
test wgpu::draw::raster_witness_lifecycle_tests::a_presentation_from_another_operation_is_refused_by_its_own_name ... ok
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 390 filtered out
```

That run is against the exact current bytes of `🖍️draw/🦀️.rs`, the law and the fixture (all three
unchanged since, 11:59; every later edit in this lane is in the renderer, the shell, the bridge or the
TypeScript). A re-run at 14:40 could not compile: a peer added `instances_delta_json` to
`semio_framework_ui_scene::World3dScene` ahead of
`🖱️ui/🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs`, which blocks the whole crate's
`--lib test` target. That is §9's churn, not this lane's code.

The headline lane is the defect itself: admit the retirement, then at **every** step assert the
operation is still admissible while a FOREIGN operation is refused by name, and assert the exact
bounded step count the fixture pins (9 for reserved-and-presented, 6 for reserved-only — one step per
witness field plus the step that observes each slot terminal, plus one terminal step). The first
version of this law FAILED against the first version of the fix, which is what forced the `retiring`
ownership into the ledger instead of a caller-side guard.

### 4.2 The TypeScript twin, on the same fixture

`📺️renderer/🧑‍🎨engine/🧪️tests/🖼️wgpu-raster-witness/{🟦️.ts,laws.json}` (new, registered in the wgpu
`vitest.config.ts`).

```
$ bunx vitest run --config vitest.config.ts 🧪️tests/🖼️wgpu-raster-witness/🟦️.ts 🧪️tests/🌳️wgpu-document-reconcile/🟦️.ts
 Test Files  2 passed (2)
      Tests  20 passed (20)
```

It proves the half a unit cannot: the fixture's own step arithmetic closes; every rule names a real,
distinct operation; the ledger is the ONE owner of the pair (the table declares neither slot); the
retired-field flags are NOT on the staged-scan cursor; the admission consults `retiring` before the
raw slots; both the commit and the abort go through that one door and re-read no slot themselves;
`retirement_step` no longer carries the per-step presentation refusal; the renderer's present cursor
mints one witness at `BeginGpu` and commits that same one at `Acknowledge`; the refusal can still name
the three facts it compared; and the engine surface feeds its raster through the frame's own candidate.

---

## 5. Deliverable A — proven on 6118

`run-10`, `http://127.0.0.1:6118/?plugin=generation3d`, 100 s:

```
alert            null                      ← no quarantine, no fault banner
viewport         1433.6 × 836
nodeCount        34
  stack[0]#procedural-play-main.body                                   [0,     0, 1433.6, 836]
  …/tree[0]                                                            [0,     0,  726.4, 836]
  …/tree[0]/stack[0]#procedural-play-graph.nodes                       [0,     0,  726.4, 192]
  …/stack[1]#procedural-play-main.canvas/componentScene[0]#procedural-main [726.4, 0, 707.2, 836]
dumpFrameStats   { windowId: "procedural-main", drawCalls: 2, quadCount: 620, glyphCount: 575 }
```

`run-10/shot-030s.png` shows it: the Flow outline tree on the left (`Vector/Computing`,
`ExtrudeCurve`, `Preview/column-preview`, the `Wires` section with `height@number`,
`profile@radius`, `extrude@solid`, `column-preview@`), the node-graph canvas on the right with its
nodes, wires and value chips, and the minimap. The **Preview grid is NOT in this window** — the
`procedural-preview` surface is a separate window instance and its World3d payload depends on §7.

Baseline for contrast (`📓️wgpu-document-reconcile` §4.4, same probe, previous build):
`worker-present-failed: raster commit candidate witness was stale · Surface: quarantined` at
t ≈ 7.8 s on every boot, `drawCalls` unreadable.

---

## 6. Deliverable B — four host divergences from React, found and fixed

The starting state was `renderSurface … effects=0`, `contributions installed … effects: 0`,
`invokeExtension 0×`. Each fix below moved the failure one hop further and is measured.

### 6.1 The brief's named suspect was NOT the cause

`settleInstanceLifecycle` (`🐚️plugin-bridge.ts`) does discard its turns' effects, and that is fixed
here — the lifecycle and its `route.accept` acknowledgement turns are now collected and stashed into
`pendingTurnEffects` through one shared `stashLeftoverEffects` (which `runQueuedTurn` now also uses, so
leftovers APPEND rather than replace). But the new `[DEBUG] wgpu-bridge lifecycle deferred effects`
line **never printed**: the instance-open turns carry no effects at all. The dropped-open-tick theory
(`📓️wgpu-blank-paint` §6, `📓️wgpu-document-reconcile` §7) is **disproved on the running target**.

### 6.2 Contributions crossed AFTER the first render — the real gate

`ShellState::settle_boot` ran `refresh_ui().await` and only THEN `push_contributions().await`. React's
`refreshUi` starts its publisher **before its own first await** and finishes it before
`pendingRefreshEffects` (`📓️contributions-push-starvation-2026-09-12.md` §2.1).

That ordering matters now because a peer's `may_rearm` gate landed in the guest: a graph whose operator
kinds the flow-extension registry cannot serve **arms nothing at all**. Every wgpu surface therefore
rendered against an EMPTY registry and armed nothing, and the later `setContributions` could not
recover it either. Pushing contributions first:

```
run-3 (after):  renderSurface surface=procedural-main turn=1 effects=0
run-4 (before → after the reorder):
                renderSurface surface=procedural-main turn=1 effects=1 tags=dispatchAction
```

**That is the one change that made the guest arm anything on this target.**

### 6.3 `setContributions`' own answer was thrown away

`push_contributions` called `plugin.push_scoped_contributions(…)` inside `if let Err(error) = …` — the
returned `InvocationResult`, `requested_effects` and all, went nowhere. It now logs
`[DEBUG] setContributions deferred effects {…}` and hands them to `queue_host_effects`, React's
`applyHostEffects` twin. (On this target it answers `effects: 0`, because §6.2 already re-armed the
chain from the render turn — but a dropped answer is a hole either way.)

### 6.4 An `Effect::DispatchAction` whose id is an APP COMMAND must cross as a command

`ShellState::dispatch_action` always built an `ActionInvocation` and called `handle_action`. React's
`makeEffectDispatchOne` picks `handleCommand` whenever
`(session.app.commands ?? []).some(c => c.id === action)` — and `flowEvalTick` is exactly that: an app
command with **no action route at all**. The wgpu shell now mirrors the predicate
(`Self::app_owns_command`) and routes through its existing `dispatch_command`.

Same fix, second half: `dispatch_command` passed `&session.view_state`, the session's stored view,
where `dispatch_action` passes `self.live_view_state(&session)`. The stored view carries no
window-instance roster, so a window-addressed command was refused —

```
run-9: deferred action flowEvalTick failed: handleCommand promise failed:
       target window is absent from the exact ViewModel window instance roster
```

— and it now reads the live view, as React's `resolveViewState(baseSession)` does.

### 6.5 A failing deferred action must not kill the boot

`flush_deferred_actions` propagated the first failure, and it runs inside `settle_boot`, so one
program action's fault terminated the worker:

```
run-4: worker-boot-failed: shell-boot: handle_action promise failed
       Boot stage: shell-boot · Long boot phase: "shell-boot:flush-deferred"
```

It now logs `[DEBUG] wgpu-shell deferred action <id> failed: <cause>` and carries on, which is what
React's `applyHostEffects` does with a dropped dispatch. `handle_command_js` also stopped swallowing
its rejection (`describe_js_rejection`, as its three neighbours already did) — that one line is what
turned `handleCommand promise failed` into §6.4's exact refusal in one boot.

### 6.6 The shared wire projection read three effects at the wrong depth

**This one is not wgpu-specific.** W0 extracted every `req`-bearing effect's payload into its own
`…-params` record (`🔌️plugin/🧬️schema/📜️.wit`), but `wireEffectToFriendly`
(`🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts`) kept reading `dispatch-action`, `open-window` and
`spawn-plugin-instance` FLAT. So the host received:

```
run-5: [DEBUG] wgpu-shell deferred action  failed: handle_action promise failed
                                          ↑ the action id decoded as the empty string
```

Fixed with `params`-scoped readers, plus a `some()` unwrapper — a WIT `option<T>` crosses as
`{tag: "some"|"none", val?}` and the first cut handed that wrapper straight to `coerceWireBytes`
(`run-6`/`run-7`: `coerceWireBytes: unsupported payload {"tag":"some",…}`, which faulted the surface).

Pinned by a new law on the same boundary as the existing `📨️effect-wire-routes` one:

```
$ bunx vitest run --config vitest.config.ts -t "params"     # @semio-tech/framework-actor
 Test Files  1 passed | 10 skipped (11)
      Tests  8 passed | 236 skipped (244)
```

Its neutral oracle is `🎭️actor/🧫️fixtures/🎁️effect-params-nesting/🔣️.json`, and its first assertion
re-reads **the WIT itself** to check which records nest and which stay flat — so a record that gains
or loses the extraction cannot drift past both the schema and the projection.

---

## 7. Deliverable B — what is still broken, exactly

```
run-10: [DEBUG] wgpu-bridge renderSurface surface=procedural-main turn=1 effects=1 tags=dispatchAction
        …
        [DEBUG] wgpu-shell deferred action flowEvalTick failed:
                handleCommand promise failed: decodeAppFrame: unknown tag 115
```

The guest arms the tick, the host decodes it with its real name and arguments, routes it as an app
command with the live window roster, and the guest's command door answers — and the **reply frames do
not decode**. `APP_FRAME_TAGS` (`💻️os/🟦️.ts`) runs `0…26`; `115` is not a variant index at all, so
this is a FRAMING error: `decodeAppFrame` is being handed bytes that are not at a frame boundary, i.e.
some field of a preceding frame is read with the wrong width or the reply stream is split wrongly.
The two places to look:

| piece | file |
|---|---|
| `decodeAppFrame` / `APP_FRAME_TAGS` / `encodeAppFrame` (the TS twin of `protocol_channel::decode_app_frame`) | `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts` (`🟦️.ts:2674`, `:3151`) |
| the reply-frame collector this command's answer crosses | `AppChannelClient.command` → `performInvocation`, `🐚️plugin-bridge/🟦️.ts` |

Worth knowing before digging: a peer is **actively adding** `AppCommand::{LoadDocumentArchive,
ReadDocumentArchive}` and a `DocumentArchivePack` type to `📡️spr/🧵️channel` (their in-flight edit made
the workspace red twice during this lane, §9). A frame variant added on the Rust side ahead of the TS
table is the first hypothesis to rule out, and `flowEvalTick` is a `Migrated` interactive job whose
reply carries more frames than a plain invocation.

`invokeExtension`, `respond`, `flowEvalResolve`/`flowTessellateResolve` and the hexagonal column's
meshes are all downstream of this single hop. **None of them was reached, and none is claimed.**

---

## 8. A peer regression that had made 6118 unbootable, fixed forward

`run-1` did not boot at all:

```
Failed to load module script: Expected a JavaScript-or-Wasm module script but the server responded
with a MIME type of "text/html".
```

At 11:17 today a peer replaced the two checked-in trunk copies

```html
-<link data-trunk rel="copy-file" href="🟦️typescript/🚀️boot.js" />
-<link data-trunk rel="copy-file" href="🟦️typescript/🎞️frame-worker.js" />
+<link data-trunk rel="copy-file" href="../../🚀️browser-boot/🤖️generated/🟨️.js" data-target-path="🚀️boot.js" />
+<link data-trunk rel="copy-file" href="../../🎞️frame-worker/🤖️generated/🟨️.js" data-target-path="🎞️frame-worker.js" />
```

— the right direction (no checked-in duplicate of a generated artifact), but trunk's
`data-target-path` names a **directory**, so the artifacts are served at `/🚀️boot.js/🟨️.js` and
`/🎞️frame-worker.js/🟨️.js` while `<script src="🚀️boot.js">` still pointed at the directory. Every
trunk dist built after 11:17 was unbootable; `📓️wgpu-document-reconcile`'s runs predate it.

**Fixed forward, never reverted**: the script tag now names the served path, and `🚀️browser-boot/🟦️.ts`
resolves its three siblings one level up (`../semio-framework-os-renderer-wgpu.js`,
`../semio-framework-os-renderer-wgpu_bg.wasm`, `../🎞️frame-worker.js/🟨️.js`) — relative, not absolute,
so a non-root `public_url` still works. These are the only `new URL(…, import.meta.url)` sites in that
bundle; everything else it fetches is an absolute `MODULE_ROUTES` path.

---

## 9. Checks, rebuilds and what was observed but not touched

```
$ cargo check -p semio-framework-ui --features wgpu-engine --lib                       # 0 errors
$ cargo check -p semio-framework-os-renderer-wgpu --target wasm32-unknown-unknown
warning: `semio-framework-os-renderer-wgpu` (lib) generated 24 warnings                # unchanged baseline
$ trunk build --config Trunk.toml                                                      # ✅ success
```

The renderer wasm was rebuilt and the serve restarted **six** times
(`kill <trunk pid>` → `screen -dmS g3dwgpu <ticket>/📜️serve-generation3d-wgpu.sh`), each time verifying
the new build actually reached `⚡️cache/📺️renderer-modules/🧊️wgpu/` by grepping the served `_bg.wasm`
for a string only that build carries. Shared cargo build dir throughout,
`CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false`, every build in the foreground.

`🎞️frame-worker.js` WAS regenerated this lane (three times — `🐚️plugin-bridge.ts` and the shared
`🖼️wire-turn.ts` are bundled into it). Two things to know:

* `generate-frame-worker` **writes the artifact and then throws** on
  `checkFrameWorkerCarrierCensus` — the census forbids the literal `localstorage`, and a live peer
  lane's `shardRuntimeDiagnosticsArmed()` in `🎭️actor/🧵️shard-runtime/🟦️.ts` still puts it in the
  bundle. The file is correct; the gate is not this lane's. Same reason the serve script's `dev` path
  fails and falls back to `trunk serve` directly.
* Trunk's `copy-file` did not always re-copy the regenerated worker into the dist (measured: a 13:43
  dist copy against a 13:53 source). Every probe run here verified the SERVED worker carried the build
  it was testing, and copied it into the dist by hand when it did not.

**Observed, not mine** — the workspace was red four separate times for 3–20 minutes on peer refactors
in flight, and went green on its own each time; nothing was reverted and nothing was fought:
`ui-runtime`/`ui-scene`/`ui-render` module files moved before their `#[path]` attributes landed; the
generated registries under `🕸️graph/🤖️generated`, `🖼️assets/🔣️icons` and `🖱️ui/🎨️styling/🔤️tokens`
were mid-regeneration (the graph one needed `bun 📜️script.ts generate` re-run once the taxonomy
validated again); `AppCommand::{LoadDocumentArchive,ReadDocumentArchive}` + `DocumentArchivePack`
landed in `📡️spr/🧵️channel` ahead of `semio-framework-plugin`; and `World3dScene` gained
`instances_delta_json` ahead of a call site. The `@semio-tech/framework-actor` suite also has 17
pre-existing failures in unrelated ajv schema suites (`…/activation/instance/output/schema.json`
cannot resolve `value/schema.json#/$defs/NonZeroU64`); every `🖼️wire-turn.ts` suite passes.

---

## 10. Files

**New**
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🖼️raster-witness-lifecycle/🔣️.json` — the neutral oracle:
  the witness triple, the admission verdicts and their faults, the per-slot/terminal step arithmetic,
  and the production entry names.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️raster-witness-lifecycle/🦀️.rs` — the Rust law (§4.1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖼️wgpu-raster-witness/{🟦️.ts,laws.json}` — the TS twin (§4.2).
- `🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/🎁️effect-params-nesting/🔣️.json` and
  `🧰️framework/🔨️modules/🎭️actor/🧪️tests/🎁️effect-params-nesting/🟦️.ts` — the §6.6 law, with the WIT
  schema as its own oracle.

**Changed**
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs` — `RasterOperationWitnessLedger` +
  `RasterWitnessAdmission`; `RasterTextureTable` delegates; `commit_presented_step`,
  `abort_presented_step`, `begin_retirement`, `retirement_step`, `begin_presenting`,
  `reserve_engine_texture`, `presentation_witnesses`, `close_step`, `terminal_is_empty`; the test mount.
- `📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — a pending primary-metrics
  invalidation drives itself and answers "not yet" (§3.3).
- `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — contributions pushed before the
  first refresh (§6.2); `setContributions`' effects queued (§6.3); app-command effects routed through
  `dispatch_command`, which now reads the live view state (§6.4); `flush_deferred_actions` is
  non-fatal (§6.5).
- `📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` — `handle_command_js`
  reports its rejection cause (§6.5).
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts` — `stashLeftoverEffects`;
  `settleInstanceLifecycle` collects and stashes its turns' effects (§6.1).
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts` — `params`-scoped readers and
  the WIT `option` unwrapper for `dispatch-action`, `open-window`, `spawn-plugin-instance` (§6.6);
  the new law's registration.
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` and
  `…/📦️packages/🦀️rust/🌐️.html` — the served-path fix of §8 (plus the regenerated
  `🚀️browser-boot/🤖️generated/🟨️.js` and `🎞️frame-worker/🤖️generated/🟨️.js`).
- `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/vitest.config.ts` — registers the twin.

No `launch.json` entry was added: `procedural3d-wgpu` already exists and is the row this lane ran, and
both new TS suites run inside existing targets (`@semio-tech/framework-renderer-wgpu:test`,
`@semio-tech/framework-actor:test`).

**Temporary logs left in the tree** — `[DEBUG] setContributions deferred effects`,
`[DEBUG] wgpu-bridge lifecycle deferred effects` and `[DEBUG] wgpu-shell deferred action … failed`.
The last one is not noise: it is the only trace a dropped host effect leaves now that the flush is
non-fatal, and §7 is read straight off it.
