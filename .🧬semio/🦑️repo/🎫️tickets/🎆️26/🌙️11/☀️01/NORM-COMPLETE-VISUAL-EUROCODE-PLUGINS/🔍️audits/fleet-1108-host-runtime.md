# Fleet 1108 Lane host-runtime

## Findings (source inspection, no compiler verdict yet)

- The norm editor crate (`semio-s-artifact-norm-en1990-editor`) depends on framework `semio-plugin`, `semio-os-kernel`, `semio-framework`; it is a member of the `s` workspace and compiles as part of `@semio-tech/norm-en1990-rs:build` (queued by lane en1990).
- Norm has no own wasm or react target directory (compare puzzle: `🎯️targets`, `🌉️wasm`); `✏️s/🧑‍💻dev/📕️norm` only has TS contract tests. There is no norm-specific host load target; mounting depends on the generic s host.
- `@semio-tech/framework-plugin` has `check` and `test` but no `build`; its `check` is the compile gate for everything the editor imports from semio-plugin (the native Context / Source chain).
- The STATUS chain (paged-text encoder, original slots, retained map) lives in `framework-plugin` / `framework-plugin-host` test targets (`test-original-*-native`). The editor does not call those test targets; only a compile error in the library `check` would block it.

## Gates

- `@semio-tech/framework-plugin:check`: started 03:2x, log `🗑️generated/host-runtime/1.txt`, queued behind ~12 capacity jobs (swap above ceiling, load 28). No verdict yet.

## Verdict so far

Unproven. No evidence that the infrastructure chain blocks the editor until framework-plugin:check and norm-en1990-rs:build report compiler errors.

## Audit items (fleet-1108-audit-requests.md, host-runtime), source-checked only, no build

- #196 / #192 keyboard undo/redo: already wired by design, no code change needed. Norm `handle_key` (app-surface/🔌️shell-hooks/🦀️.rs:237) returns `Ignored` for `HostRoute::History`; the wgpu shell then reaches its universal chord tail (`⌨️keyboard/🦀️.rs` handle_keyboard_async: mod+z, mod+shift+z, mod+y dispatch the framework `undo`/`redo` verbs, the same funnel as History panel and palette). React does the same (`control-keybinding-context/🟦️.tsx:140,145`). Existing tests: norm `shell-hooks/🧪️tests/🦀️.rs:84-85,114` (History is Ignored so the chord falls through) and shell law `⌨️wgpu-shell-shortcuts-palette/🦀️.rs:110-122` (`shell_edit_verb_for`). Not executed this session (build gate saturated). Copy/paste/duplicate are mapped through `HostRoute::Entity`. The audit grep for `ArtifactCommand::Undo` finds 0 hits because the store verb is reached via the framework action id, not from norm.
- #224 hub manifests: no `norm-en1996-inputs/results` left in any source tree (only ticket markdown). Stale, done.
- #52, #98, #191, #219, #220: framework features (world-3d binding args, device_view dock switch on resize, LiveCommand::Edit, plugin selection write API, outliner drop targets). Each needs framework edits plus a wgpu build; not started because the gate is saturated and the framework is owned by other teams. Open.

## Gate status

`@semio-tech/framework-plugin:check` still queued (log 🗑️generated/host-runtime/1.txt, load ~46, swap full, 11 ahead). No exit yet.

## Follow-up: framework items and hub component

Decision per item for the norm end-to-end path (open norm, edit a numeric field, see check update, undo/redo, reload persisted, EN+DE). No framework edits made; each skipped item is a convenience outside that path and a blind framework edit without a gated build would risk shared files:

- #52 world-3d handle binding args: direct-manipulation of 3D handles; the numeric edit goes through the properties inspector. Skipped.
- #98 dock switch on resize via device_view: mobile/tablet layout; e2e is desktop. Skipped.
- #191 LiveCommand::Edit for duplicate/paste: not part of edit/undo/persist. Skipped.
- #219 select-all via interaction-domain write API: not required; selection by click works through interactionSelect. Skipped.
- #220 outliner drag-drop targets: not required. Skipped.

Hub component: `semio_hub_norm_component.wasm` (cargo package `semio-hub-norm-component`, `🌎️hub/🧩️compositions/📕️norm/🧩️component`) is produced by `@semio-tech/os-hub:trusted-catalog-bootstrap --packages norm` (parser in `🌎️hub/🎛️commands/🏷️trusted-catalog/🟦️.ts`). `TICKET/🧰️scripts/after-en1990-hub-norm-component.sh` waits for `🗑️generated/en1990/1.txt` to exit 0 and then queues it via `build-slot.sh` (extended to forward extra args after the log path); log `🗑️generated/host-runtime/hub-component.txt`. The component links all norm artifact crates, so it needs them green. Unverified: whether `repo run … --output=static --packages norm` forwards the flag as expected.

## #52 handle drag (framework change, documented)

Finding: norm's viewport binds `Delta`/`Commit`/`Abort` on the world surface (`🧊️viewport3d/🧰️toolbar/🦀️.rs` gesture_bindings) and `Gesture::parse` expects `{handle, s, origin, modifiers, at, phase}`, but no renderer merged those arguments, and handles reached the host as plain ring markers without an axis.

Schema first (`🧰️framework/🔨️modules/🖱️ui/🎬️scene`; this schema is hand-pinned, there is no generated twin, so no generator run applies):
- `🧬️schema/📏️world3d-modelling/🔣️.json`: optional `axis` (vec3) on the marker.
- Rust `World3dMarker.axis: Option<World3dVec3>` (serde default, skipped when absent), codec read/write (`push_option`/`fields.option`), validation (non-zero), constructor.
- TS `World3dMarker.axis?`, parser accepts and validates it.
- Fixture `🧪️tests/🧫️fixtures/📏️world3d-modelling/🔣️.json`: valid `annotations-drag-handle-marker`, invalid `marker-zero-axis`. TS suite (`bun test`, light): 11 pass, 0 fail, includes Ajv reference validation. The Rust twin of that fixture runs in `@semio-tech/ui-scene-rs:test` (gated, not yet run).
- Norm producer: `🫧️lane-overlays/🦀️.rs` `grip_marker` gives Translate/ScalarAlong handles their axis; other handles stay plain; `🏷️overlays/🦀️.rs` literal updated.

React host (World3dHost + Interpreter), minimal:
- `ComponentSceneHostProps.onTrigger` (`🧩️components/🟦️.ts`), filled by `Interpreter/🖼️surface-views` only when the surface node binds delta/commit/abort, forwarded by `🔌️surface-bridge`; implemented with the existing `dispatchTrigger`, whose payload already merges an object input over the binding's authored arguments (`🎬️action/🧭️intent-address-and-sequencer`), so the static `phase` of each binding is kept.
- `World3dHost/🧲️handle-drag/🟦️.ts` (pure): `world3dHandleDragParameterV1`, `world3dHandlePickV1`, `world3dHandleDragStepV1` (press arms silently, changed parameter streams delta, release after travel commits once, cancel after travel aborts, a click dispatches nothing). The existing `axisDragParam` now delegates to the parameter function (one implementation, identical algebra).
- `World3dHost/🧲️handle-drag/🪝️session/🟦️.ts`: hook `useWorldHandleDrag` (pick against the projected annotation markers within 12 px, camera ray via `axisDragParam`, latest-wins promise chain to `onTrigger`, Escape aborts, pointercancel aborts). Wired in `🎛️marquee-and-gumball` (returned) and consumed in `👆️pointer-and-drop` (`active-utility` press and move, `handle-pointer-up` release), ahead of the relocate and marquee paths.
- Language-agnostic fixture `World3dHost/🧪️tests/🧫️fixtures/🧲️handle-drag.json` (6 parameter vectors, 3 pick vectors, 5 event sequences); test `🧑‍🎨engine/🧪️tests/🧲️world-handle-drag/🟦️.ts`: 14 pass, 0 fail (`repo --service=off tools exec --light bun -- test`, absolute path). Independent numpy oracle `TICKET/🧰️scripts/handle-drag-oracle.py` matches all parameter vectors.
- All edited TS files transpile (`bun build --no-bundle`); that is syntax only.

NOT done: wgpu. Its world surface runs through the GPU pick and the intent queue (`♾️infinite/🌍️world/🧵️interaction`, `handle_world3d_pointer_*`, `WorldGumballState`) with no access to the retained node's bindings; a handle drag there needs new drag state in the world authority plus a binding dispatch path, which I did not write blind. The pure gesture and the fixture are ready to be ported to Rust against the same JSON.

Unvalidated (all gated): TypeScript types of every edit above (`@semio-tech/framework-os-dev:typecheck`), the Rust marker codec and fixture (`@semio-tech/ui-scene-rs:test`), the norm app-surface compile with the new field (shared lane norm build), and the actual React drag in a browser: whether the orbit camera rig also takes the press on a handle and rotates during the drag is unknown and may need the host to suppress orbit while a session is active.

Gate: `TICKET/🧰️scripts/handle-drag-gate.sh` runs typecheck then ui-scene-rs:test through `build-slot.sh`; logs `🗑️generated/host-runtime/handle-drag-typecheck.txt` and `handle-drag-scene-test.txt`.

## #220 outliner drag-drop

Skipped: `drop_target_action` (`Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📋️receipts-drop-and-clipboard/🦀️.rs:238`) only reads `Stack` nodes, the norm outliner rows carry no drop bindings, and norm defines no reparent or reorder mutation for a drop to dispatch. A blind renderer edit would be speculative; not on the edit/undo/persist path.
