# Astra Sol Layout Contract

## Contract Finding

`LayoutSpec::Overlay` is an in-flow positioning context. Its children stack inside a relatively positioned box and `inset` supplies the box's inner spacing. `LayoutSpec::Absolute` is the out-of-flow node variant.

Three independent sources agree:

- The renderer-neutral contract describes Overlay as “a positioning context whose children stack on top of one another anchored to the box.”
- The same contract describes Absolute as placement “outside normal flow.”
- The independent generic UI renderer maps Overlay to Taffy `Position::Relative` with `inset` mapped to padding, while Absolute maps to `Position::Absolute`.

React previously projected Overlay as `position: absolute; inset: …`, then `ContainerView` overwrote it to `position: relative`. That same overwrite set every non-Overlay layout's `position` to `undefined`, erasing Absolute. WGPU directly treated Overlay as absolute and its old laws asserted that contradiction.

## Repair

- React now projects Overlay directly as `position: relative` plus padding and preserves the complete style returned for every layout, including Absolute.
- WGPU now maps Overlay to an in-flow flow style whose per-side padding is the contract inset.
- Absolute remains explicit out-of-flow layout in both targets.
- The obsolete WGPU Overlay out-of-flow laws were replaced with shared-fixture geometry laws.

## Shared Fixture and Oracles

`🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📐️overlay-flow/🔣️.json` defines one 200×100 vertical layout with:

- an asymmetric Overlay padding context;
- one fixed child inside it;
- one Absolute sibling that consumes no flow space;
- one normal following sibling whose y-coordinate proves the Overlay consumes flow space.

The React law mounts real `ContainerView` output and reads the browser-style `CSSStyleDeclaration`. The Rust flex law reads the same JSON and compares production geometry. A second Rust law builds the same fixture with the third-party Taffy solver. The mounted-layout law verifies that production publication retains the same rectangles.

The focused React gate passed both Overlay and Absolute laws. Native Rust laws await the parent's integrated Cargo run because the available Cargo slot remained locked throughout this pass.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/📐️overlay-flow/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📐️flex/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/📐️overlay-flow/🟦️.tsx`

The fixture preserves `anchor` and `dismissible` on the wire. Per-child anchor placement remains a separately documented gap in the generic renderer and is outside this flow-semantics repair.

## Parent Integration Review

Astra corrected the following sibling's expected width from 200 to 38.4. The fixture explicitly authors `Fixed(Xxl)`; both the current production `FlowStyle::flow_size` and CSS/Taffy stretch only an automatic cross size. Its y=35.2 assertion remains the flow-semantics oracle. This was a fixture expectation error found during source review, not an observed native test failure; the native run remains pending.
