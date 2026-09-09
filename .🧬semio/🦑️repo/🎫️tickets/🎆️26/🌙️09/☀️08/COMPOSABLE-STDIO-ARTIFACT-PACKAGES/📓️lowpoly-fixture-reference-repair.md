# Lowpoly Fixture Reference Repair

The bounded census found 17 Lowpoly Any-subset mutation test files with 85 stale compile-time fixture references. Every reference used the same obsolete prefix:

`../../../../../📄️documents/🧬️mutations/`

The canonical fixtures already exist under:

`../../../../../🧫️fixtures/🧬️mutations/`

The mutation kind, case name, snapshot direction, mutation, diff, and outcome tails match exactly. The repair changes only that literal prefix in the existing test files; it adds no aliases, migrations, fixtures, or assertion changes.

## Verification

- 17 touched Rust test files.
- 85 canonical `include_str!` references.
- 0 remaining references to the obsolete document prefix.
- 0 missing canonical targets after resolving each literal relative to its source file.
- Exact-file `rustfmt --edition 2021` completed for all 17 files.
- The scoped read-only Git diff contains only the 85 old/new `include_str!` literal lines.

The repaired mutation/case pairs are:

- `↗️move-object/📍️translates-obj-hull-along-x-and-z`
- `➕️insert-paint-layer/🪜️stacks-a-detail-layer-above-the-base-layer`
- `➖️remove-paint-layer/➖️drops-the-detail-layer-at-index-1`
- `🌫️change-paint-layer-opacity/🌫️fades-the-base-layer-to-half`
- `🌱️create-object/⛵️inserts-obj-mast-between-hull-and-fin`
- `🎛️change-paint-layer-blend-mode/✖️switches-the-base-layer-to-multiply`
- `🎨️edit-paint-layer/🖌️paints-red-over-the-second-half-of-the-base-layer`
- `🏷️rename-object/🏷️retitles-obj-hull`
- `👁️change-paint-layer-visible/🙈️hides-the-base-layer`
- `💀️delete-object/🚫️removes-obj-fin-without-touching-the-order`
- `📐️scale-object/📐️halves-obj-hull-uniformly`
- `🔀️reorder-objects/🔀️moves-obj-fin-in-front-of-obj-hull`
- `🔄️rotate-object/🔄️yaws-obj-hull-about-the-y-axis`
- `🔖️rename-paint-layer/🏷️retitles-the-base-layer-to-undercoat`
- `🔘️change-object-smooth-shading/🟢️turns-on-smooth-shading-for-obj-hull`
- `🕸️create-mesh/🕸️attaches-a-mesh-child-handle-to-obj-fin`
- `🧨delete-mesh/✂️detaches-the-mesh-child-handle-from-obj-hull`
