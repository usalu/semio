# Stack Drop Destination Identity

The production Shell resolves dock-drop geometry from `DockState::render_view`: a whole-stack drag removes its source and collapses the remaining tree without changing the committed layout. The resulting paths name the remaining destination stacks. `DockState::apply_drop` incorrectly compares that destination path to the source's old committed path. In a three-stack row, dragging the first stack onto the second stack produces destination path `[0]`, equal to the removed source's old path, and silently refuses the drop.

React Canvas has the same positional comparison. It also reads the target anchor from the committed layout and then resolves that anchor after extraction, which can land a drop on the sibling before the visible target. Both implementations must apply paths directly to the same derived layout used for drop targeting. Invalid destination paths must leave the committed tree intact.

Scope: the Dock Rust implementation, its fixture-driven tests, the Shell's actual pointer-release path, React Canvas and its fixture-driven oracle tests. Existing historical W2d tests cover only two-sibling contraction and do not exercise surviving sibling path aliases. The unrelated maximization question is deferred because both current implementations use the same positional render behavior; it is not part of this proven defect.

## Implementation

The WGPU drop transform now validates insertion in its extracted candidate tree without comparing the destination to the removed source's former path. All existing candidate-tree insertion refusals remain atomic. React resolves destinations in that same extracted tree and returns its original committed layout when a tab or split destination is invalid; this also fixes source loss for the visible tab grip when destination geometry is stale.

The canonical schema and thirteen vectors live under `🖱️ui/🧱️elements/🎨️Canvas/{🧬️schema,🧫️fixtures}/🎯️stack-drop-destination/🔣️.json`. They cover first/middle/last-source movement, tab insertion, split insertion, row/column contraction, and invalid tab/split destinations for both tab and whole-stack payloads. The React test uses Ajv schema validation and React DOM's static projection as the third-party fixture witness. The WGPU test executes the identical vectors and prints destination-path and window-order witnesses.

The Shell regression executes normalized PointerDown, PointerMove and PointerUp for the actual tab grip, checking that promotion preserves the committed tree, a shifted sibling receives the tab, successful release persists and focuses that window, and an invalid target leaves the original tree unpersisted. It controls the accepted destination geometry explicitly rather than asserting an unobserved GPU presentation.

## Executed Validation

- Initial React fail-first gate ran through `bun nx run @semio-tech/ui-react:test --skip-nx-cache -- <co-located test>`: eight failed and two passed. The initial vectors used noncanonical kebab-case corners; these were corrected to the existing schema's camelCase values before the green gate. Positional source/destination refusal and invalid-destination source loss were present independently of that corner typo.
- A subsequent fundamental-level retry exceeded its fifteen-second budget during import and was killed. The green gate was therefore requested at `long`.
- Two green Nx attempts stopped during project-graph construction, before running tests. Reported blockers outside this packet were the new media-module import/declaration boundary, an unowned generated frame-worker JavaScript path, and a writer fixture depending on the absent `semio-repo-test-host` project.
- The scoped whitespace check ran successfully. The Rust fixture include path was resolved and confirmed to exist.

- Fresh React green gate passed after source graph reconciliation: `NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/ui-react:test --skip-nx-cache -- long '../../../../🧱️elements/🎨️Canvas/🧪️tests/🎯️stack-drop-destination/🟦️.tsx'` executed fourteen passing tests (thirteen neutral vectors and the independent Ajv schema check), with a 13.70-second Vitest duration and successful Nx exit. The generated execution log is `🗑️generated/sol-stack-drop-react-green-neutral-split.txt` until ticket cleanup.

Native execution is still pending in the coordinator's gate. No physical browser success is claimed here.

The default Vitest console interceptor muted the vector witnesses even with `--silent=false`. The focused gate was therefore repeated with `--disableConsoleIntercept`, which passed fourteen tests and emitted all thirteen `[DEBUG] React stack drop` projections. Concrete examples include shifted first-source landing at `[0]` with `["a","a-copy","b"]`, later landing at `[1]` with `["a","a-copy","c"]`, and all invalid targets preserving `["a","a-copy"]`, `["b"]`, `["c"]`. Generated console witness log: `🗑️generated/sol-stack-drop-react-witness.txt` until ticket cleanup.

## Reachability and Scope

Whole-stack transforms already exist in both renderers, but the current React Canvas only exposes `startTabDrag`, and WGPU Dock paints tab grips without a stack grip. This packet repairs that existing transform without introducing a new control. The normalized pointer witness covers the reachable tab grip. Maximization is unchanged because the investigated positional rendering behavior is shared by both implementations and had no separate intended-behavior contract.

## Changed Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧬️schema/🎯️stack-drop-destination/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧫️fixtures/🎯️stack-drop-destination/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🧪️tests/🎯️stack-drop-destination/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛰️Dock/🧪️tests/🎯️stack-drop-destination/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs`
- This ticket report.
