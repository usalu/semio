# wgpu Accessibility / Status / i18n at RUNTIME — lane `wgpu-a11y-status-i18n-runtime` (2026-09-14)

Closes the three reds of `📓️wgpu-end-to-end-verification-2026-09-14.md` §5.5 (`status-a11y-i18n`,
**3/8**). The probe now scores **8/9** on the coordinator's serve, with the one remaining red
root-caused to a layer this lane deliberately did not touch.

- Probe (rewritten): `🐍️wgpu-status-a11y-i18n-probe.mjs`
- New recon probes: `🐍️wgpu-a11y-runtime-recon.mjs`, `🐍️wgpu-palette-locale-recon.mjs`, `🐍️wgpu-status-pill-recon.mjs`
- Evidence: `🗑️generated/wgpu-a11y-runtime/{recon,recon-after,palette,palette-2,status-*,probe-1,probe-2}/`
  and the battery's own `🗑️generated/wgpu-verify/status-a11y-i18n/`

---

## 1. TL;DR

| item | before | root cause | after |
|---|---|---|---|
| ♿️ ARIA mirror | `nodeCount 0` | the mirror was refreshed from `onUiTurn`, which the transport raises **only for a turn that breached its budget** — a healthy shell overruns once, at boot, so the mirror was painted once from a race and never again. And the dump it pulled answered for **one** window (the largest), so 26 of the app's 64 announced nodes were unreachable even when the race was won. | ✅ **64 ARIA nodes across all 4 live windows**, and still correct after the document changes under it (§2) |
| ⏳️ status pill | never observed | **not the shell.** Across a 23 s `Sphere Cut With Torus` evaluation driven from the live example picker, the generation3d preview window body is published **exactly twice** — once before, once after — both carrying `phase:"idle" inFlight:0 cancellable:false`. No renderer, wgpu or React, can show progress for work whose producer never publishes a non-idle status (§3). | ❌ **still not observed, and explicitly not claimed** — characterised, owning layer named, NOT fixed here |
| 🗣️ German via the palette | "the palette did not dispatch" | **it always dispatched.** Nothing in the shell logged a palette activation, an os command or a resolved locale, and every translated string on this target is painted into pixels — so "no `setLocale` in the console" was never evidence. Two REAL palette defects were found on the way and fixed. | ✅ **`os.setLocale:de` dispatched from the palette; chrome, generation3d window labels and the whole os command registry resolve German** (§4) |

Two product defects fixed at the owning layer with laws (§2, §4), one peer compile break fixed
forward (§6), one stale oracle in three ticket probes fixed (§5) — that last one is what had the
battery reporting `status:trigger` as `blocked` with `offered: []` against a shell that was in fact
answering every navbar control.

```
bun 🐍️wgpu-battery.mjs --only=status-a11y-i18n
  → status-a11y-i18n 8/9, 314 s, 0 page errors
    ✓ boot  ✓ accessibility:mirror  ✓ accessibility:per-window  ✓ status:trigger
    ✗ status:pill-while-computing   ✓ status:settled  ✓ accessibility:live
    ✓ locale:palette-dispatch       ✓ locale:german-via-palette
```

⚠️ **Screenshots are not evidence on this target.** The canvas is an `OffscreenCanvas` owned by the
frame Worker and headless Chromium captures it blank (every `*.png` under the run directories is the
same 5 854-byte empty frame). Every claim below rests on the DOM, on the renderer's own introspection
exports, or on its `[DEBUG]` console traces — which is exactly why §4 had to add a trace before
German could be proven at all.

---

## 2. ♿️ The ARIA mirror — root cause, fix, laws, runtime

### 2.1 Root cause: the mirror hung off the BUDGET channel, not the FRAME channel

`🚚️browser-frame-transport/🟦️.ts:518-521`:

```ts
observeUiTurn(site: string, executingMs: number | undefined): boolean {
  const outcome = this.uiTurns.admit(site, executingMs);
  if (outcome.verdict !== "admitted" && outcome.verdict !== "clock-fault") this.onUiTurn?.(outcome);
  …
```

`onUiTurn` is documented as "Reported for every UI turn that breached its ceiling" (`:281`). The
accessibility mirror was refreshed from it (`🚀️browser-boot/🟦️.ts`, `onUiTurn` hook). Measured on
6118 before the fix: `canvas.dataset.uiTurn` froze at `recorded-overrun:ready-hook:2.100` for a whole
60 s session (`🗑️generated/wgpu-a11y-runtime/recon/console.txt`) — ONE overrun, at boot, and none
after. So the mirror was painted exactly once, from a pull issued inside `onReady` plus its 400 ms
coalescing follow-up, and whether that pull landed before or after the guest published its first
document was a coin flip:

* the lane that built the mirror won the race → 34 nodes;
* the end-to-end lane lost it → `nodeCount 0`;
* this lane's first recon won it again → 38 nodes, with `uiTurn` still frozen at the boot overrun.

No source changed in between. It is a race whose loser is permanent, and — far worse than the flake —
**every later document was invisible to a reader even when the race was won**: a locale switch, an
example switch, a selection, a new window. A later `nodeCount 0` on a slower machine is the same
defect, not a different one.

**Second defect, found while fixing the first.** The mirror pulls `dumpAccessibility()` with no
window id, and `build_accessibility_dump` reused `build_structure_dump`'s selection rule — *the
largest last-known viewport*. That rule is right for a diagnostic and wrong for a production
accessibility path: on generation3d it announced `procedural-main` (38 nodes) and silently dropped
`procedural-preview` (11), `procedural-main/framework.section.measures` (3) and
`procedural-preview/framework.section.measures` (12). A reader could not reach the 3D preview at all.

### 2.2 Fix

* `🚀️browser-boot/🟦️.ts` — the mirror is refreshed from `onDirectives`, the transport's FRAME channel
  (raised for every accepted frame at the current generation, `🚚️browser-frame-transport/🟦️.ts:724`),
  and `onUiTurn` is left as the pure budget/diagnostics channel it is documented to be. `refresh()`
  now ALWAYS schedules its pull on a timer instead of pulling inline: the driving hook runs inside the
  UI turn's executing clock and `introspect` flushes a batch, so pulling inline would re-enter frame
  message handling. The 400 ms coalescing floor is unchanged, so this is still one round trip per
  400 ms at most and nothing ticks while nothing happens.
* `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` — `DumpAccessibility` now carries `windows: [{ windowId, nodes }]`
  instead of one `nodes` list. A caller that NAMES a window gets exactly that window (and an empty
  answer when it is not live, so "not mounted" stays distinguishable from "mounted and empty"); a
  caller that names none gets EVERY live window in the engine's own window order. No compatibility
  shape was kept — the single consumer (the mirror) moved with it.
* `🚀️browser-boot/🟦️.ts` — the mirror paints the concatenation, each element carrying
  `data-window`, and the root carries `data-windows` beside `data-node-count`. Description span ids
  are now window-qualified, which they had to become once two windows can contribute the same node id.

### 2.3 Laws

`🧪️tests/📨️browser-frame-transport/🟦️.ts` (`bun ./📜️script.ts test quick`, **33/33**):

1. `raises the frame channel for every accepted frame while the budget channel stays silent` — five
   accepted frames produce five `onDirectives` and **zero** `onUiTurn`. This is the behavioural law
   the defect needed: anything hung on `onUiTurn` runs exactly once in a healthy session.
2. `refreshes the accessibility mirror from the frame channel, never from the overrun channel` — the
   directive hook contains `accessibility?.refresh()`, the turn hook contains no `accessibility` at
   all, `refresh` has no inline `void pull()`, and the paint consumes `dump.windows` with a
   `data-window` per element.

`🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs` (`cargo test -p semio-framework-os-renderer-wgpu --lib -- introspection`, **5/5**):

3. `the_accessibility_dump_announces_every_live_window_unless_one_is_named` — three live windows, an
   unnamed dump announces all three, a named dump announces exactly one and still names the others, a
   window that is not live announces nothing.

`build_accessibility_dump`/`dump_window_ids` were widened from `#[cfg(target_arch = "wasm32")]` to
`#[cfg(any(target_arch = "wasm32", test))]` so the law drives the REAL body natively rather than a
copy — the wasm-only gate is why this selection had no law at all.

### 2.4 Runtime on `http://127.0.0.1:6118/?plugin=generation3d`

```
accessibility:mirror      pass  {"mirrorRole":"region","mirrorLabel":"Semio controls",
                                 "rawNodeCountAttribute":"64","nodeCount":64,
                                 "mirroredWindows":["procedural-main",
                                                    "procedural-main/framework.section.measures",
                                                    "procedural-preview",
                                                    "procedural-preview/framework.section.measures"],
                                 "labelled":47,
                                 "roles":["group","tree","treeitem","application","combobox",
                                          "paragraph","region","switch","slider"],
                                 "liveRegions":1,"shortcuts":1}
accessibility:per-window  pass  {"(all windows)":{"procedural-main":38,
                                                  "…measures":3,"procedural-preview":11,
                                                  "…preview/…measures":12,"total":64},
                                 "procedural-main":{"total":38}, …}
accessibility:live        pass  gesture=shell.example.sphere-cut-with-torus
                                 before 64 nodes / 47 labels ["Column Height","Profile Radius","Side Count", …]
                                 after  60 nodes / 38 labels ["Torus Major Radius","Sphere", …]
```

`accessibility:live` is the step that could not exist before: it switches the example — which rewrites
every node name in `procedural-main` — and demands that the mirror's ARIA labels followed. A mirror
driven by `onUiTurn` cannot pass it in a healthy session, by construction.

Two nodes carry real assistive semantics beyond a label, and both survive the crossing:

```
{window: procedural-main,    role: application, label: "Node graph canvas", shortcut: "Enter"}
{window: procedural-preview, role: application, label: "3D preview canvas", live: "polite"}
```

Full dumps: `🗑️generated/wgpu-a11y-runtime/probe-2/accessibility.json`,
`🗑️generated/wgpu-verify/status-a11y-i18n/accessibility.json`.

---

## 3. ⏳️ The World3d compute-status pill — measured, root-caused, NOT fixed

### 3.1 What the earlier lanes actually measured

Both earlier runs watched a session with **nothing to compute**. `?plugin=generation3d` with no
example boots an empty document; the producer publishes
`{"phase":"idle","progress":{"unitsTotal":0,…},"cancellable":false}` on every frame, and no pill is
the CORRECT answer there (`world3d_status_is_visible` is byte-for-byte React's own gate). "The pill
never painted" said nothing about the pill.

### 3.2 The two real orderings, both measured

`🐍️wgpu-status-pill-recon.mjs` measures both.

**Boot axis (`?example=sphere-cut-with-torus`)** — `🗑️generated/wgpu-a11y-runtime/status-sphere-cut-with-torus/`:

```
wgpu-worker boot_shell leave 27494 ms        ← the example is evaluated INSIDE boot
flowEvalTick settled ×7                       10.9s → 29.0s
wgpu-shell dock plan …                        29.6s   ← the first window exists
world3d surface=procedural-preview …          30.7s   ← the surface exists
```

The whole 27.5 s evaluation runs before **any** window exists. That is a defect in its own right —
an expensive operation with no progress and no cancellation — but it is not one the shell can fix: at
`t < 29.6 s` there is no surface to annotate.

**Live re-evaluation (the shell's own example picker)** —
`🗑️generated/wgpu-a11y-runtime/status-picked-2/`. Boot with no example, wait until the World3d surface
is live and painting (7.7 s), then drive `playground.navbar.fixture` → `shell.example.sphere-cut-with-torus`
from the shell's own hit trace. This is the ordering the coordinator asked for and it is now reachable
(§5). Result:

```
picked  shell.example.sphere-cut-with-torus  at 87.3s
                                              …23 s of evaluation…
distinct producer statuses carried by the preview surface, whole 150 s session:
  7 693 ms  {"phase":"idle","phaseLabel":{"en":"Idle","de":"Bereit"},
             "progress":{"unitsDone":0,"unitsTotal":0,"inFlight":0,"ratio":1.0},
             "cancellable":false,"cancelAction":"toolRunAbort","debug":{"meshesLen":2,"instancesLen":2}}
110 211 ms  … identical, except "debug":{"meshesLen":3644,"instancesLen":707}
```

and the published body itself, by size, moved **exactly twice**:

```
  7 693 ms  meshes=2b    instances=2b
110 211 ms  meshes=3644b instances=707b
138 988 ms  meshes=2b    instances=2b
```

### 3.3 Root cause, and why this lane did not fix it

The wgpu shell's half is correct and unchanged: `world3d_status` is refreshed from the surface
registration on every frame the surface paints (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`,
`mirror_engine_surface_states`'s `World3d { status_json }` arm), and `surface_status_pills_for`
paints the pill the instant a non-idle status arrives — 11 shared fixture rows and 6 laws already pin
that. **The producer never publishes one.** The generation3d preview window's own projection is
complete (`🧵️preview-eval/🦀️.rs` `preview_scene_status_json` → `computing` from
`FlowEvalSession::pending()` or an abortable `ToolRunView`, plus the tessellation and budgeted-eval
ledgers), but at every hop boundary of a `flowEvalTick` chain `tick_scheduled` is false (a window
waiting on an extension answer is marked *owed*, not armed) and both ledgers are empty, so every
publication during the chain is byte-identical `idle`/`0`. Nothing a renderer does can show progress
for that.

This is **not fixed here**, deliberately:

* the owning layer is the generation3d preview-eval projection and the flow host's progress ledgers
  (`✏️s/…/🧊️generation3d/…/🧵️preview-eval/🦀️.rs`, `🧰️framework/…/🌊️flow/🖥️host/🦀️.rs`), not the wgpu
  renderer this lane owns;
* the abortable-run half of that same projection is `ToolRunView`, which a peer is migrating right now
  (`ToolRunDefinition` gained a `settings` field mid-session — §6), so an edit there would collide;
* the shell-side claim the coordinator asked for ("a late-attaching surface must receive the current
  status") is already true and is not what fails: the surface receives the current status on the frame
  it attaches, and the current status is `idle`.

`status:settled` ✅ still proves the whole contract is read end to end (`phase`, bilingual `phaseLabel`
`{en: "Idle", de: "Bereit"}`, `cancellable`, `cancelAction: "toolRunAbort"`).

---

## 4. 🗣️ German at runtime — proven, and two real palette defects on the way

### 4.1 Root cause of "the palette did not dispatch": there was no witness

The palette dispatched all along. Nothing in the shell logged a palette chord, a palette activation,
an os command or a resolved locale, and generation3d's translated strings live in chrome painted on
the canvas — not in the retained document (whose text nodes are the user's own node names, identical
in every tongue) and not in the DOM. Both earlier lanes concluded "no `setLocale` dispatch anywhere in
the console" from a console that could not have carried one.

**Fix — a runtime witness.** `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` gains `trace_resolved_locale(reason)`,
printed once per RESOLUTION (a preferences load and a `setLocale`), never per frame — the durable
half of §6.2 of `📓️wgpu-status-a11y-i18n-2026-09-13.md`. Plus one line each for the palette chord, the
palette activation (query, item count, what was offered, what was chosen) and every os command.

### 4.2 Defect A — `mod+p` was a one-way door

`handle_keyboard` gated every hardcoded shell chord on `editing = input.focused_id.is_some()`, and the
chord that OPENS the quick-search palette sets `input.focused_id = Some("shell.search.input")`. So the
closing half of its own toggle could never run: a second `mod+p` fell through to the open palette's
`Char` arm and typed a literal **`p` into the query**. Measured directly
(`🗑️generated/wgpu-a11y-runtime/palette/console.txt`): the recon's first run pressed `Control+p` then
`Meta+p`, searched for `pDeutsch`, matched nothing, and `activate_search_item(0)` returned on an empty
list — a perfect reproduction of "the palette does not dispatch", caused by the product, in a probe.

### 4.3 Defect B — Escape was claimed by the focused-input commit

`handle_keyboard_async`'s `if input.focused_id.is_some() { Enter | Escape => commit_focused_input }`
arm runs before `handle_keyboard`'s own palette branch. With the palette's query field focused, Escape
was consumed by the commit and the palette's Escape arm never ran. With §4.2 that left the palette with
**no keyboard route out at all** — only a click outside closed it.

### 4.4 Fix

One predicate, `Self`-less so its law drives the real body:

```rust
pub(crate) const SHELL_OVERLAY_INPUT_IDS: [&str; 2] = ["shell.search.input", "shell.find.input"];

pub(crate) fn content_is_editing(focused_id: Option<&str>, sync_card_open: bool) -> bool {
    focused_id.is_some_and(|id| !SHELL_OVERLAY_INPUT_IDS.contains(&id)) || sync_card_open
}
```

Both call sites use it. The chord additionally clears the query and releases focus when it closes, so
a reopened palette starts empty — React's palette behaves the same way.

### 4.5 Laws

`🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs` (`cargo test -p semio-framework-os-renderer-wgpu --lib -- shell_input_tests shell_chrome_parity_tests`, **43/43**):

1. `the_shells_own_overlay_fields_do_not_count_as_the_user_typing` — the predicate, five rows.
2. `the_palette_chord_toggles_and_never_types_itself_into_the_query` — open → type `De` into the query
   → the same chord closes it, clears the query and releases focus. Drives the real `handle_keyboard`
   over a real `ShellState`.
3. `escape_closes_the_palette_rather_than_committing_its_query_field` — drives the real
   `handle_keyboard_async`.

### 4.6 Runtime — German, proven

```
[DEBUG] wgpu-shell palette chord search-open=true overlay=Search focused=Some("shell.search.input")
[DEBUG] wgpu-shell palette activate index=0 query="Deutsch" items=1
        offered=["command.os:os.setLocale.de"]
        chose=Some(("command.os:os.setLocale.de", Some("os-command:os.setLocale:de")))
[DEBUG] wgpu-shell os command id=os.setLocale value=Some("de")
[DEBUG] wgpu-shell locale resolved=de terminology=native reason=setLocale
  chrome=[overlay.search.title=Suchen, overlay.find.title=Auf Seite suchen,
          example.overlay.title=Beispiele, common.cancel=Abbrechen, common.close=Schliessen,
          common.focus=Fokussieren, common.loading=Wird geladen,
          nodeGraph.fitGraph=Graph einpassen, contextMenu.goHome=Zur Startseite]
  windows=[procedural-main=Workflow, procedural-preview=Vorschau,
           generation3d-generations=Generationen, generation3d-generate-form=Formular,
           generation3d-generate-preview=Vorschau]
  commands=[os.toggleFullscreen=Vollbild umschalten, os.setAppearance=Erscheinungsbild festlegen,
            os.setDriver=Treiber festlegen, os.setLocale=Sprache festlegen,
            os.setTerminology=Terminologie festlegen, os.setThemeId=Design festlegen,
            os.resetDock=Dock-Layout zurücksetzen]
```

Every generation3d window label and every chrome string the shell resolves is German, from one real
user gesture (`ctrl+p` → `Deutsch` → `Enter`) on the running serve. `procedural-main` resolves to
`Workflow` in both tongues — that is the word generation3d's own terminology declares, not a missing
translation.

### 4.7 One more input defect, characterised only

`key_action_from_dispatch` maps `" "` to `KeyAction::Space(pressed)`, and the renderer's `handle_key`
returns on `Space` unless a context menu is open (`🧊️renderer/🦀️.rs:13707-13715`). **A space therefore
never reaches the shell's keyboard routing at all** — it cannot be typed into the palette query or
into any focused shell input. Out of this lane's scope (it interacts with `space_pressed`, which the
camera paths read); recorded for the input lane.

---

## 5. 🔎️ The stale oracle that had the battery reporting `blocked`

Three ticket probes parse the shell's hit trace with

```js
/os_host pointer hit x=([-\d.]+) y=([-\d.]+) targets=(\d+) hit=(.*)$/
```

The trace now reads `… targets=69 staged=1 gen=3 hit=Some(…)`. Every match therefore failed, the
navbar sweep answered `[]`, and the battery's `status:trigger` scored `blocked` with `offered: []` —
against a shell that in the very same run resolved `playground.navbar.fixture` **33 times**
(`🗑️generated/wgpu-a11y-runtime/status-picked-sphere-cut-with-torus/console.txt`). The regex is
widened to tolerate any number of `key=value` fields between `targets=` and `hit=` in
`🐍️wgpu-status-a11y-i18n-probe.mjs`, `🐍️wgpu-status-pill-recon.mjs` and `🐍️wgpu-journey-probe.mjs`.
With it the example picker and its ten rows are reachable again, which is what made §3.2's live
re-evaluation measurable at all.

---

## 6. 🩹️ Peer compile break fixed forward

A peer's in-flight `toolRun*` migration added a `settings` field to `ToolRunDefinition`
(`🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:1841`) and had not reached every call site. The renderer links
the plugin registry, so this stopped the renderer's own build. Fixed minimally, one field and one
import, in `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/🛠️tools/🪣️fill/🦀️.rs` — `settings:
ToolRunSettingsReads::default()`, the spelling every other call site in the tree already uses. The
`◻️2d` twin was fixed by its owner while this lane waited; nothing was changed there.

---

## 7. Files

**Changed**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` — mirror on the frame channel, all-windows paint, deferred pull.
- `…/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` — `DumpAccessibility { windows }`, all-window selection, test-visible cfg.
- `…/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs` — the selection law.
- `…/🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts` — the two channel laws (`turns` hook added to the harness).
- `…/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `SHELL_OVERLAY_INPUT_IDS`, `content_is_editing`, palette-chord/activate/os-command/locale traces.
- `…/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs` — three palette laws.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` — peer break fixed forward (§6).
- `…/PROCEDURAL-3D-END-TO-END/🐍️wgpu-status-a11y-i18n-probe.mjs` — rewritten.
- `…/PROCEDURAL-3D-END-TO-END/🐍️wgpu-journey-probe.mjs` — hit-trace regex (§5).

**Added**
- `…/PROCEDURAL-3D-END-TO-END/🐍️wgpu-a11y-runtime-recon.mjs`
- `…/PROCEDURAL-3D-END-TO-END/🐍️wgpu-palette-locale-recon.mjs`
- `…/PROCEDURAL-3D-END-TO-END/🐍️wgpu-status-pill-recon.mjs`
- `…/PROCEDURAL-3D-END-TO-END/📓️wgpu-a11y-status-i18n-runtime-2026-09-14.md` (this file)

**Builds** — renderer wasm `@semio-tech/framework-renderer-wgpu:wasm` (10:04 and again 10:27 on the
current tree), `generate-browser-boot`, `generate-frame-worker`. One renderer wasm build at a time:
two peer builds were waited out before each.

---

## 8. What is NOT claimed

1. **The status pill is still not observed at runtime.** §3 measures why with a full evidence chain
   and names the owning layer; the pill's own contract and layout remain proven only by their 11
   shared fixture rows and 6 laws. This lane did not touch the guest.
2. **Nothing is claimed from a screenshot.** The canvas is an `OffscreenCanvas`; every capture in
   every run directory is blank. The pill's pixels, the palette's pixels and the German chrome's
   pixels are unproven as pixels — the German claim rests on the shell's own resolution trace.
3. **The `?example=` boot axis is not fixed.** A boot-axis example is evaluated inside `boot_shell`
   for 27.5 s with no window, no progress and no cancel. Characterised in §3.2, not addressed.
4. **The space key is still swallowed** before the shell's keyboard routing (§4.7).
5. **One battery run in this lane was lost to a peer's renderer regression**, not to anything here:
   the 10:18 renderer build faulted the surface at ~10 s with
   `worker-present-failed: offscreen prepared frame admission: prepared render revision is stale:
   live=68, packet=59`, which detaches the introspection beacon and disposes the mirror, so every
   step after `boot` read zeros (`🗑️generated/wgpu-verify/status-a11y-i18n/console.txt` of that run,
   since overwritten). The 10:27 rebuild off the same tree is clean and is the 8/9 above. A future
   `nodeCount 0` should check for a surface fault before suspecting the mirror.
6. **`accessibility:live` proves the mirror follows the DOCUMENT**, not that it follows at a
   particular latency. The coalescing floor is 400 ms and nothing here measures the tail.
7. **No suite-wide green is claimed for `semio-framework-os-renderer-wgpu`.** Every law this lane
   owns or touches was run green on a compiling tree
   (`--lib -- introspection` 5/5, `--lib -- shell_input_tests shell_chrome_parity_tests` 43/43,
   `bun ./📜️script.ts test quick`'s vitest half 33/33 for `📨️browser-frame-transport`). A whole-suite
   run afterwards could not complete, for two reasons that are not this lane's:
   `async_boundary_tests` carries four reds against source a peer edited at 09:44 and 10:21 today
   (`native_binary_owns_exactly_one_entrypoint_driver` wants `drive_entrypoint(` twice and finds it
   three times; `renderer_asset_probe_…` SIGABRTs in `WorldAssetFetchOwner::drop`), and by 10:47 the
   crate stopped compiling at all on an unresolved `tool_run_trace` module in
   `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:69` — the same in-flight `toolRun*` migration as §6, mid-edit.
   Re-run the suite once that peer lands.
