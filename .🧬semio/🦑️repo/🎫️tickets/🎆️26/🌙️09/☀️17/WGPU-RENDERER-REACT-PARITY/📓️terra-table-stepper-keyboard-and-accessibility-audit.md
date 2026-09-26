# Table Stepper Keyboard and Accessibility Audit

**Scope:** read-only source audit on 2026-09-26. No source changed and no test was run by this audit.

## Finding

`TableCell::Stepper` has working WGPU pointer segments, but its React keyboard and assistive-technology contract is absent from the WGPU scene path.

React makes the centre readout the one focus stop: it is a read-only `Input` with `role="spinbutton"`, a column-derived label, and live minimum, maximum, numeric value, and value text. It maps ArrowUp/ArrowRight to `+step`, ArrowDown/ArrowLeft to `-step`, PageUp/PageDown to ten steps, and Home/End to the bounds. It clamps the resulting delta and dispatches the cell's authored action with `{ delta }` merged into the authored args. See [Table React host](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🟦️.tsx:63) and its rendered control [at line 97](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🟦️.tsx:97).

The existing React interaction oracle already asserts the semantic triple, dispatches, bounds, and key behavior at [engine contract test line 6668](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:6668). It calls the pointer half a WGPU parity contract, but it does not execute a WGPU key route.

WGPU paints minus, plain centre text, and plus. The centre is not a focusable node and has no stable control id. Its hit resolver only recognizes pointer left/right thirds; the centre returns `Some(None)` to suppress row selection. See [render path](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3686) and [hit path](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:3804). The WGPU table test covers those pointer segments and centre swallowing at [line 181](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-table/🦀️.rs:181), with no stepper-key test.

## Missing execution path

The platform key vocabulary cannot express the whole React behavior: `KeyAction` has arrows but no Home, End, PageUp, or PageDown at [input line 109](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:109), and native winit conversion recognizes no such values at [line 509](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:509). The shell mapper must likewise gain the four variants; its existing map is at [line 13598](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13598).

`UiEvent` itself can carry a key, but the retained event router only generates a `UiCommand::Scene` on pointer and scroll routes. Its `KeyDown` path handles retained generic controls and editing, but does not emit a scene command ([event router](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:2216)). The interpreter's scene-intent converter rejects every event other than pointer and scroll at [line 1484](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1484). This is why adding only key enum variants would leave a table stepper inert.

## Accessibility finding

The accepted retained-document projection walks authored `UiNodeRecord`s only ([projection](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs:95)); a `Component::Surface` announces one `application` node ([contract role](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:45)), and a surface consumes its document subtree ([reconciler](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:252)). Therefore table rows and cells do not appear in the accepted document tree.

The shell's separate chrome projection cannot supply them: it deliberately excludes every retained-body hit at [line 31460](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:31460), and its range fields are unset at [line 31514](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:31514). The shared projection wire already has the required role, focus, rectangle, and range fields ([schema](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:158)); no browser-specific schema is needed.

## Coherent slice boundary

Implement a scene-owned, accepted-frame `TableStepperFocus` address. It should contain the window id and generation, mounted scene node, scene host id, stable row id, and column id. On every key it must re-resolve the current `TableScene` payload, verify that the same current component is a table and that the identified cell remains a stepper, compute React's raw and clamped delta, and only then merge the current action args. It must return no action for an unrelated key, a bound, a missing/replaced row or cell, a closing document, a retired scene, or a stale generation.

This mirrors the existing focused text-editor law at [line 1537](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1537): it retains window generation, node, and host id, then revalidates all three plus component kind and retirement on every key. The reconciler preserves a component identity only for the same key, kind, and surface; otherwise it retires and remounts it ([lines 1126–1233](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:1126)).

Publish one scene-produced virtual accessibility node for the accepted current stepper readout. Its stable key may be `{host_id}.row.{row_id}.{column_id}.stepper`; it needs role `spinbutton`, the React column/value label, its centre-third rectangle, `value_min`, `value_max`, `value_now`, `value_text`, `focusable`, `actionable`, and accepted focus state. Retire it with the same focus address. This is scene behavior, rather than a generic `NumberStepper` replacement: the table value is read-only and drives the table cell's own action descriptor.

## Required oracle and journey

Use a language-neutral fixture family describing `value`, `min`, `max`, `step`, row identity, column id/label, authored args, a key, and expected merged delta or no action. Minimum independent cases:

- ArrowRight at `3/0..10`, step `2` yields `+2`.
- PageUp at `3/0..10`, step `2` yields clamped `+7`.
- PageDown at `3/0..10`, step `2` yields `-3`.
- Home and End yield exact deltas to the lower and upper bounds.
- ArrowLeft at the lower bound and PageUp at the upper bound yield no action.
- A non-step key, removed row, changed column cell kind, replaced scene generation, and retired host yield no action.

The React test should consume this fixture with its existing `@testing-library` interaction oracle. The WGPU scene test must use the production centre-focus then key route, rather than call a helper alone, and assert the current descriptor's retained args plus delta. It must also show that centre focus does not dispatch the row-selection action. An accepted-frame test must assert that candidate-only spinbutton focus or projection cannot become live before acknowledgement, then after acknowledgement verifies its role/value fields; replacing or removing the table scene must remove the node and make a late key inert.
