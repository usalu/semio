# Example dropdown

## What is wrong

On play the navbar example control opens a listbox, but the list is painted off the screen. Measured on the live note pane (`https://play.semio-tech.com/#note`): the trigger sits at viewport `(846, 3)` while the listbox border box is at `(-14513, -4286)`. The play grid uses `transform: matrix(1, 0, 0, 1, -15360, -4320)` to bring the active cell into view. `SelectContent` portaled into the shell layer with `position: fixed` and viewport `left`/`top`. A transformed ancestor is the fixed containing block, so those viewport numbers land one grid cell away from the trigger. The control looks inert.

`#lowpoly` has no control at all. The shell only builds example options when the app declares `setActiveExample` (`appSwitchesExamples`). The lowpoly editor registers `hexagonal-cut-concrete-forest-left` through `ArtifactEditor::examples` and does not declare that action, so the picker is omitted. The same gate hides layout, gis3d, mathematical, sequence, home, and space. Architect, dag, imperative, and trinity-rewriting declare the action in Rust while their committed descriptors predate it.

## Placement fix

`resolveSelectFrame` anchors the listbox to a positioned portal host when that host sits inside a transform, filter, perspective, or paint container. Coordinates are host-local and `position` is `absolute`, so the menu travels with the pane. A dialog isolation root stays the anchor it already was. A page with no transform keeps viewport-fixed coordinates.

Select component tests: 15 passed, including `anchors an open listbox to a translated shell portal instead of the viewport`.

## Still open

Every subset must register examples, and every editor must get this dropdown without a hand-written `setActiveExample` opt-in. Selecting a row must load that example. The placement fix makes an existing picker visible. It does not yet register missing examples or load them for editors that never declared the action.
