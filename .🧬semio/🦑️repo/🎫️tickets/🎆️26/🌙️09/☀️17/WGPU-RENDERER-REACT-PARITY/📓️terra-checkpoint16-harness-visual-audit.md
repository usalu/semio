# Checkpoint 16 Physical-Harness and World Visual Audit

Read-only audit on 2026-09-20. This reviews the current physical probe and the sealed `checkpoint-16-palette`, `checkpoint-16-palette-b`, and `checkpoint-16-full` receipts. It does not treat a source or native census as physical acceptance.

## Current receipts are correctly failing where the product fails

The new General receipt reads state owned by each renderer, rather than trusting a dispatched action. React reads the mounted `.semio-scope[data-shell-id]` `data-ui-appearance` and scoped `lang` (`🧱️elements/🏛️ShellHost/🟦️.tsx:7794-7808`; `🎯️targets/⚛️react/🟦️.tsx:1872-1880`). WGPU reads the existing accessibility mirror's two settings combobox `aria-valuetext` values (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7822-7858,10059-10120`; `🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:92-98`). That is a sound, non-debug state receipt.

`checkpoint-16-full/steps.json` records WGPU `setAppearance(dark)` and `setLocale(de)` as dispatched, followed by a 15-second failure waiting for those values to reach the mounted shell. React records the intended applied states `{appearance:"dark",locale:"en"}` then `{appearance:"dark",locale:"de"}`. This is a product failure in the current WGPU run, not a harness false positive. The same run also reports the Drivers collapse, Actions Abort, and subsequent pane failures; they prevent later journey steps from providing paired acceptance evidence.

The corrected palette replay must also remain a failure: `checkpoint-16-palette-b/parity.md` reports the WGPU `worker-input-failed: text edit step failed: Protocol` fault after the dismiss/reopen sequence. React completed the staged Appearance consequence. A final red renderer fault is now fatal-classified, which is the required behavior for a physical gate.

## Geometry is measured but not part of the gate

`🐍️parity-interact-probe.mjs:79-101` emits geometry only to `geometry.md`. `stepVerdict` at `:167-184` compares action-id sets, surface additions/removals, and fullscreen state, but never rects or a screenshot/pixel result. Therefore the `boot` and `dismiss-tour` behavioral matches do not establish chrome geometry.

The paired 1600 × 1000 receipt has material, stable differences:

| Control | React rect | WGPU rect | Maximum difference |
| --- | --- | --- | --- |
| `playground.navbar.fixture` | `706,3,192,22` | `750.135,3.2,110.179,22.4` | 81.8 px |
| `playground.navbar.roles.editor` | `909,4,90,22` | `866.713,3.2,86.839,22.4` | 42.3 px |
| `framework.hub.signIn` | `1197,975,67,22` | `1132.787,974.4,80.246,22.4` | 64.2 px |
| `os.task-manager` | `1529,975,68,22` | `1405.195,974.4,113.627,22.4` | 123.8 px |
| `s-presence-peers` | `1018,978,101,16` | absent | missing |

Source boundaries are distinct: React composes the navbar in `🧱️elements/🏛️ShellHost/🟦️.tsx:9508-9534`, while WGPU calculates item widths from its font atlas in `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16197-16208`, aggregates the center width at `:23012-23045`, and paints the retained controls later in the same file. This audit does not attribute the offsets to a particular metric, but the receipt disproves geometry parity. The absent presence pill is separately material: WGPU intends to paint it even with an empty browser roster (`:9371-9384`, `:24570-24581`), yet it was not published in the paired receipt.

Required gate change: compare the existing named control rectangles for the same viewport with an explicit per-control tolerance, and fail a missing required control. No universal tolerance is justified by this audit; it must be selected from an agreed chrome contract rather than silently leaving differences of 42–124 px observational.

## Window, pane, and action target identity remain unasserted

The driver correctly makes each local pane action physical and records its containing `windowId` (`🐍️parity-interact-probe.mjs:1091-1126`). It also records pre/post body maps for divider and window-cap operations. But `stepVerdict` discards `detail`, including that identity and those maps. Both renderers could open a same-suffix pane in different windows, or resize different sibling bodies, and still pass if their action sets and surface sets coincide.

The same applies to action payload targets: `actionIds` deliberately collapses events to a unique bare action string (`:151-167`). Controller normalization is reasonable, but a physical action that carries a known `windowId` needs an identity assertion. Use the existing local window identity on the interaction result and compare the canonical seeded window identity, while retaining the current controller normalization. This affects Actions, pane chips, focus/unfocus, divider drag, and window close/reopen; it is not a concern about ordinary repeated frame actions.

## World visual evidence

The boot/dismiss screenshot difference is not a window allocation or initial camera mismatch. `checkpoint-16-full/cameras.md` pairs both seeded world windows by identity, with position/target deviations at or below `0.00005`, matching projection/FOV, and viewport-size difference `0.06562` CSS px. The visibly different mesh/grid/backdrop therefore belongs in world rendering.

### Missing GLB mesh outlines — high confidence product gap

React derives an `EdgesGeometry` outline for every GLB mesh in `applyGlbMeshEdgeBorders` (`🧱️elements/🌐️World3dHost/🟦️.tsx:2282-2303`) and calls it for the cloned loaded scene at `:2357-2362`. Its ordinary shaded-mesh fallback also emits an outline when source edge data is absent (`:2990-3003`).

WGPU `append_component_overlays` (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:9713-9735`) does the opposite: it omits every mesh whose schema has zero source edges. The checkpoint mesh diagnostics report `edgePositions:0` for the loaded Concrete Forest meshes. That directly accounts for the absent/weak WGPU outline visible in `02-dismiss-tour.png`. WGPU needs the same derived boundary/edge representation for such GLBs; merely changing line color or camera values cannot expose lines that are not emitted.

### Grid and backdrop — visible mismatch, cause not yet isolated

React uses Drei's camera-following infinite shader grid with `cellThickness={0.6}` (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:935-984`). WGPU emits a finite, faded set of line vertices (`♾️infinite/🌍️world/🦀️.rs:7747-7788`). The grid is visibly different in the same-camera screenshot, but the evidence does not establish whether line coverage, blending, rasterization, or a remaining grid parameter is responsible. A fixed-camera pixel/line coverage law is required before selecting a rendering change.

React leaves a transparent world canvas when the environment background is omitted or `transparent` (`🧱️elements/🌐️World3dHost/🟦️.tsx:932-934,7396-7401`; `♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3689-3770`). WGPU always fills the world bounds with `environment_clear_color`, which maps the absent/transparent case to `theme.canvas_clear` (`♾️infinite/🌍️world/🦀️.rs:12658-12680,8157-8168`). This is a verified rendering-path difference and a plausible cause of the lighter WGPU backdrop; a sampled backdrop pixel away from grid/reference content is needed to prove the color/compositing defect.

Reference-plane offset is not established. CAD-to-Three conversion is identity (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:258-260`), and both paths currently place the default-origin plane directly (`r3f:3825-3985`; WGPU `world/🦀️.rs:12910-12922`). The screenshots alone cannot distinguish a reference-plane discrepancy from grid/backdrop styling. Add a projected-corner receipt for a non-default reference only if this becomes a target; do not change reference placement on this evidence.

## Acceptance decision

Checkpoint 16 has useful physical coverage, especially its corrected state receipt and fatal renderer-fault classification, but cannot certify parity yet. It currently has concrete WGPU settings, Drivers, Actions/pane, and palette protocol failures, a direct GLB-outline visual gap, and measured chrome geometry differences excluded from the verdict. A rerun should require successful state receipts, paired window identities for window-owned gestures, required-control geometry comparison, and a fixed-camera visual law for the grid/backdrop after the known functional failures are repaired.
