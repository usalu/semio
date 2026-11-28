# FEM standalone viewers: defect fixes (2D 6088, 3D 6089)

Ticket `26/11/22/GET-FEM-PLUGIN-WORKING-END-TO-END`, 2026-11-27. Browser checks: `node matrix-v3/run.mjs viewer2d|viewer3d`. Rust laws ran natively in private `C:\t\vw` (deleted).

## Summary

| # | Defect | Verdict | Fix lands | Native law | Browser now |
|---|---|---|---|---|---|
| 1 | 2D viewer pans only with the middle button | real | wasm republish | PASS | still FAIL (old wasm) |
| 2 | `Actions` enabled in both viewers | by design, no editing action present | n/a (check was wrong) | PASS | PASS (check redefined) |
| 3 | 3D viewer has no results caption (2D has none either) | real in both | wasm republish | PASS | still FAIL (old wasm) |
| 4 | `#viewer` app id boots as Editor unless the role var is set | real, framework | live (TS, vite) | PASS | PASS (verified live, role var removed) |

## 1. Primary-button drag pans in the 2D viewer

- Root cause: the Canvas2d host pans on the primary button only when the scene carries a `role: "meta"` layer with `leftDragPans: true` (`📐️Canvas2dHost/🗂️layers-session` `pointerDown`/`pointerMove`, `rightDragPans` likewise). The 2D editor emits that layer from `fem2d_canvas_meta_utility_layer` (`leftDragPans` for Direct Select, `rightDragPans` always); there is no separate Navigate mode in the 2D editor (utilities are selectDirect, marquee, lasso, transform). The viewer emitted no meta layer, so only the host's built-in middle-button pan worked. The 3D viewer needs nothing (its controller orbits on the primary button, pans on middle/right/Shift).
- Fix: shared `canvas_2d_viewer_meta_layer()` in `⚙️engine/🖥️app-surface/🦀️.rs` (same layer as the editor's Direct Select: primary and secondary drag pan), inserted as first layer by the 2D viewer Model window. Viewer never imports the editor.
- Files: `✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs`, `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs`.
- Laws: `viewer_meta_layer_pans_with_primary_and_secondary_drag` (app-surface), `viewer_scene_pans_with_primary_drag_and_carries_the_case_caption` (2D viewer, reads the assembled scene).
- Matrix: `viewer.drag-camera` now requires the LEFT drag to move the view (2D pan, 3D orbit); before it accepted any button. 3D passes now (`middle:moved left:moved right:moved`); 2D reads `left:no change middle:moved right:no change` until the wasm is republished.

## 2. `Actions` button in the viewers

- Findings: the viewer declares no action of its own, but the framework injects read-only shell verbs into every window: Copy, Export Document, Import Document (opens a picked archive as a NEW document, never overwrites the focused one). The palette opened live in both viewers lists exactly `Copy`, `Export Document`, `Import Document…`. The role gate is the framework's, by action KIND: `command-bar/🔶️scope` filters `mutation`-kind commands from the palette when `session.app.role === "viewer"`, and `input-funnel` refuses a `mutation` dispatch with `viewer-read-only`. Utilities is already disabled (the viewer declares none).
- Decision: keep the palette. Removing or disabling it would remove legitimate read-only verbs (export, copy) and diverge from the framework rule; the requirement is that editing actions are absent, and they are. No product change.
- Law: `viewer_declares_no_mutation_action_anywhere` in both viewer crates (no `ActionKind::Mutation` in the app roster or any window kind roster). The framework gate itself already has its own laws.
- Matrix: `viewer.no-editor-chrome` now opens the palette and asserts the entries are within the read-only framework verbs (Copy, Export/Import Document, DE spellings), with no enabled Utilities/Inspection/Results. PASS in both.
- Inventory V2D-03 / V3D-03 ("declares no actions") stays true for the plugin; the shell verbs are framework-owned.

## 3. Viewer caption

- Root cause: the editor caption (`Case: dead`, `Converged · …`) belongs to the editor's separate Results window. The viewer has only the Model window and no results at all, so neither viewer showed a caption (the matrix check had passed 2D vacuously, `d === 2 ? true`). The brief's premise that the 2D viewer shows one was wrong; both are fixed.
- Fix: the viewer is read-only and does not solve, so its caption names what it draws, the load case: `Case: <first load case id>` / `Lastfall: <id>` (`∅` when none), localized EN and DE, as a data text node above the scene (same positional-key column the editor Results window uses, so it is DOM text and accessible, also for the 2D canvas). Shared `viewer_case_caption` and `with_viewer_caption` in app-surface; used by both viewer Model windows.
- Files: app-surface `🦀️.rs`; 2D and 3D viewer `🪟️windows/🧱️model/🦀️.rs`.
- Laws: `viewer_case_caption_names_the_load_case_in_both_locales` (app-surface), `viewer_names_the_load_case_above_the_scene_in_both_locales` (3D viewer, projects the tree, EN and DE, scene still `world-3d`), the 2D caption assert in `viewer_scene_pans_with_primary_drag_and_carries_the_case_caption`.
- Matrix: `viewer.caption` now requires `^(Case|Lastfall): ` for both viewers (2D no longer vacuous). Both FAIL (`caption=""`) until the wasm is republished.

## 4. App id implies the role; single source of truth

- Root cause: `🧑‍💻dev/🟦️.ts` defaulted the role env to `"editor"` and `Shell/🔒️boot-and-locks` `resolveBootAppRole` returned `"editor"` when nothing was stated, so `appRole` was always defined. `resolveBootPrimaryAppV1` is role-last-wins and projects the pinned anchor onto `appRole`, so a pinned `…#viewer` was always projected back to Editor.
- Decision: yes, the app id implies the role (a surface app id is `<dialect>#<role>` by construction); an explicit `?role=` / `VITE_SEMIO_APP_ROLE` still wins (the existing `?role=viewer` over an editor pin behavior and its laws are unchanged). An UNSTATED role must not override the pin.
- Fix: the boot role is `AppRole | undefined`: `resolveBootQueryAppRole(search, fallback: AppRole | undefined)`, `envAppRole` and `resolveBootAppRole` return `undefined` when nothing was stated; `resolveBootPrimaryAppV1` with no anchor and no role prefers the editor surface (`appRole ?? "editor"`). The wgpu boot descriptor (Rust-fed, `appRole` always `viewer|editor`) was not changed; it is a separate shell and not served by these ports.
- Files (all under `🧰️framework/🛍️products/💻️os/🔨️modules/`): `🧑‍💻dev/🔗️boot-query/🟦️.ts`, `🧑‍💻dev/🟦️.ts`, `📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🔒️boot-and-locks/🟦️.ts`, `…/🐚️Shell/🟦️.tsx` (doc), `…/🏛️ShellHost/🔀️surface-switch/🟦️.ts`, `…/🏛️ShellHost/🔐️identity-binding/🟦️.ts` (doc).
- Law (language-agnostic fixture, run through the shipped module, the in-file oracle and the bare-node twin): `…/🏛️ShellHost/🧪️tests/🧫️fixtures/🔀️surface-switch/🔣️.json` gains six boot rows (pinned viewer boots viewer, viewer default boots viewer, pinned editor boots editor, explicit `?role=editor` projects a viewer pin, explicit env viewer keeps a viewer pin, no anchor and no role prefers editor) and `envRole`/`expectedRole` may be `null`; `🧪️tests/🔀️surface-switch/🟦️.ts` updated (`oracleBootApp`, twin, null handling). `bash viewer-vitest.sh` equivalent: renderer-react Vitest of that file, 8 of 8 pass.
- Harness: `matrix-v3/serve-viewers.sh` no longer sets `VITE_SEMIO_APP_ROLE` (both viewers restarted without it; the `#viewer` app id alone boots the viewer, `viewer.role` PASS on 6088 and 6089).

## Test infrastructure repaired on the way

- The viewer crates' unit tests did not compile before this work (2D used `semio_plugin::artifact_app_laws` without the `artifact-app-testing` feature; 3D used a path that does not export `built_surface_scene`/`project_and_retire_fixture_tree`). Both viewer `Cargo.toml` now carry `semio-plugin = { workspace = true, features = ["artifact-app-testing"] }` as dev-dependency (the unused `semio-plugin-testing-laws` dev-dependency of the 3D viewer was dropped; the 2D and 3D `Cargo.lock` viewer entries were adjusted to match). Existing scene tests read the surface through the caption column (`node.children[1].children[0]`).
- Native results: `semio-s-fem-app-surface` viewer laws 2 of 2, `semio-s-artifact-fem-2d-viewer` 7 of 7, `semio-s-artifact-fem-3d-viewer` 6 of 6.

## Browser status of the whole viewer matrix (served wasm, current)

2D viewer 7 of 9 PASS: boot, role, model-drawn, no-editor-chrome, mesh-overlay, wheel-zoom, read-only. FAIL: `drag-camera` (left drag), `caption`. 3D viewer 9 of 10 PASS; FAIL: `caption`. No other viewer check fails.

## Needs the next wasm publish (coordinator)

Defects 1 and 3 (Rust): rebuild and publish the `semio:fem` component for BOTH variants after these crates change: `semio-s-fem-app-surface`, `semio-s-artifact-fem-2d-viewer`, `semio-s-artifact-fem-3d-viewer`. Then `bash matrix-v3/serve-viewers.sh` need not change; re-run `node matrix-v3/run.mjs viewer2d` and `viewer3d`; expected: `viewer.drag-camera` 2D left moved, `viewer.caption` `Case: dead` (3D) and the 2D demo's first load case.
Defect 4 and the check changes are live now (vite). The editors also load the changed TS: an editor served with `VITE_SEMIO_APP_ID=…#editor` and no role var still boots the editor, `?role=viewer` still projects.

## Cleanup

`C:\t\vw`, `C:\t\vwtmp` deleted; vitest cache used `C:\t\vwtmp\vitest` (deleted); ticket-local helper scripts `viewer-vitest.sh`, `viewer-fixes-law.sh`, `viewer-fixes-law3d.sh` kept; generated logs deleted. The viewers on 6088 and 6089 are running (restarted without the role var).
