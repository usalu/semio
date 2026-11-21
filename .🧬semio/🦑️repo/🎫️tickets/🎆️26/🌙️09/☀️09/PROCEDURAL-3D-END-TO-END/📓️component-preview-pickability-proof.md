# Component Preview Pickability Proof

## Edit preview authority

Component editing keeps all evaluated input and output instances that have drawable geometry. It carries the graph-origin hover targets and source window into World3d while component operations use the existing geometry domain. Exact tessellated outputs with a source revision and direct authored mesh outputs can offer editable components. Inputs, markers, nested authored meshes and older fallback meshes carry `componentPickable: false` in component mode. The existing output-only component target parser continues to enforce edit authority.

React applies the flag to component raycasts, projected component overlays and marquee candidates. Exclusive component selection also skips the separate projected whole-instance pointer-up fallback. Chromium reproduced that fallback publishing an input port as a face-domain ID despite the geometry raycast filter. Native World3d reads the same field in the scene bridge and uses one instance predicate in the component ray and marquee cursors.

## Verification

- [Mounted Chromium](world3d-browser-probe.ts) passed with `--component`. Flow-origin hover of a render-only input changed 471 pixels in its image region and retained the geometry revision. An independent Three ray ordered the retained foreground before the editable output. The mounted host emitted only the editable output’s face from that ray. The marquee emitted no component for the retained instance and all 12 faces of the editable mesh.
- [Paired Edit and View native producer](preview-direction-native.ts) passed five tests using cached Pack JSON `5262b28961bbf9e7`. Component cases covered input aliases, hover source ownership, a pending output drawn with an older mesh, a point marker, a direct authored mesh output and a nested authored mesh. Regular Edit and View payload parity also passed.
- [Compact native candidate test](component-pickability-native.ts) compiled and tested the production scene admission and eligibility predicate against the same [neutral fixture](component-pickability.json). Both native cursor call sites use that predicate. The native host’s analytic ray and marquee regression now also contains a closer render-only instance as a counterexample. Its full Cargo host lane was outside the cached proof and was left unrun while the machine held a separate Cargo grant.
- The preview direction, retained path, inline geometry, graph hover ownership and world hover ordering suites passed 20 tests. The worktree whitespace check passed for the affected files.

Run the compact scripts through `repo tools exec --light bun --` from the checkout. The paired producer also takes the absolute cache directory `.cache/b/r/m/u/l/p29a0c8` and the Pack JSON hash above.
