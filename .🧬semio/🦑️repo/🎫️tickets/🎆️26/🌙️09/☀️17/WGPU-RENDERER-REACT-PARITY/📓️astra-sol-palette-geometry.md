# React Command Palette Geometry

## Scope

This packet diagnoses and repairs the React command palette geometry regression captured by the strengthened browser journey. It also reconciles the two WGPU palette laws that Native 34 exposed while preserving the already established interaction contract. No WGPU production code changed in this packet.

## Runtime evidence

The checkpoint artifact `🗑️generated/astra-runtime/palette-dismiss-react-15b/react/03-command-palette-activate.png` showed a filtered `Set Theme` row at the viewport top while the dialog input and Close control were above the viewport. The corresponding browser geometry was:

```json
{
  "dialog": [544, -23.992, 512, 47.984],
  "style": {
    "position": "fixed",
    "top": "0px",
    "left": "800px",
    "translate": "-50% -50%"
  },
  "portal": [0, 0, 1600, 1000],
  "activeElement": "ui.search.input"
}
```

The filtered journey reported the content at `[544,-48,512,96]`, the input at `y=-48`, the result row at `y=6`, and Close at `[1024,-48,16,24]`. The portal and document had zero scroll offsets, so focus scrolling was not moving the centered dialog. The computed `top: 0px` proved that the authored vertical centering utility had been removed before the browser applied the classes.

## Cause

`DialogContent` already authored both `top-[50%]` and `left-[50%]`. The repository-owned `cn()` implementation classified every physical inset edge under one `inset-start` group. Consequently, the later `left-[50%]` removed `top-[50%]`, leaving `translateY(-50%)` anchored at zero and pulling half the dialog above the viewport.

This was a shared class-composition defect. A Dialog-specific class-order workaround would have left the same defect on every component that combines independent inset edges.

## Repair

`🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🟦️.ts` now gives top, right, bottom, left, logical start, and logical end independent groups. The shorthand conflict graph is explicit:

- `inset` supersedes every axis and edge.
- `inset-x` supersedes horizontal physical and logical edges.
- `inset-y` supersedes top and bottom.
- Repeated utilities on one edge remain last-wins.

The language-neutral BDD scenario in `🧪️tests/🔀️merge-conflicting-utilities/🥒️.feature` pins independent edges, same-edge replacement, and shorthand-plus-narrow-edge composition. Its TypeScript adapter registers the new scenario. The mounted ShellSearch React test independently requires both centering utilities on the actual rendered dialog.

## Native law reconciliation

Native 34 exposed two stale test-harness assumptions rather than a new production defect:

1. `the_palette_chord_toggles_and_never_types_itself_into_the_query` expected an unactivated chord dismissal to clear the query. React and the existing neutral palette contract preserve the query. The law now requires `De` to survive dismissal and proves that ordinary closed-state typing and the chord's `p` do not leak into it.
2. `publish_palette_chrome` began directly at the overlay phase and omitted production's input-frame retirement step. The next synthetic frame therefore combined closed chrome with stale staged palette hit targets. The helper now drains the bounded hit-retirement step before publishing the next frame, matching the production double-buffer lifecycle.

These changes are confined to the two Rust law files. Root owns their native execution.

## Verification

Focused React ShellSearch:

- `@semio-tech/ui-react:test-quick` on the ShellSearch component test: **6/6 passed**.
- Receipt: `🗑️generated/astra-palette-geometry/shell-search-focused.log`.

Class composition:

- Both generated `test-subject` projects passed: `merge-conflicting-utilities` and `flatten-class-name-inputs`.
- Receipt: `🗑️generated/astra-palette-geometry/class-name-subjects.log`.
- Their broader `test-quick` contract wrappers did not reach the subjects. Both fail in shared contract infrastructure because `nativeSecondImplementationBreaches` passes `undefined` to `isSemioNativeArtifact`, which reads `artifact.length` at repository test line 3335. Receipt: `🗑️generated/astra-palette-geometry/class-name-contract-runner.log`.

Broader React census:

- **787/804 tests passed**, 26/28 files passed.
- The 17 failures are outside this packet: repeated missing `🧰️framework/🎨️styling/🖌️ui/🎨️.css`, a missing tutorial fixture, stale source-shape assertions, and one UIDialog test whose fixture submits `terrain` while its schema allows only `map`.
- Receipt: `🗑️generated/astra-palette-geometry/ui-react-census.log`.

Requested native filters for the root-owned rerun:

```text
shell_input_tests::the_palette_chord_toggles_and_never_types_itself_into_the_query
shell_shortcuts_palette_tests::the_command_palette_publishes_filters_and_activates_through_normal_chrome
```

## Acceptance boundary

The source and focused tests prove that React now retains both independent centering utilities and that the WGPU laws represent the established query and hit-registry lifecycle. A fresh activated browser artifact remains necessary to prove that the dialog, input, result rows, and Close control are physically inside the viewport after bundling. No runtime post-fix claim is made here.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧪️tests/🔀️merge-conflicting-utilities/🥒️.feature`
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🏷️class-name-composition/🧪️tests/🔀️merge-conflicting-utilities/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔎️ShellSearch/🧪️tests/🧩️component/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs`
