# Native Fixture Repairs

Run 77357 compiled 373 tests: 325 passed, 48 failed. Painted picking and primitive picking were absent from the failure list. The failures include canonical fixture omissions for shear and text patch fields, the Demo DSL asset missing shear, an affine scene test using an empty path, a floating-point equality in movement preview, and Select All admission settled through the typed-command helper instead of the framework-reserved helper.

The fixture updates are explicit current-schema assets; no compatibility branch or migration script is added. The movement assertions now use absolute 1e-10 tolerance. The scene test supplies a real rectangle. Select All uses the same reserved-admission settle helper as interactionSelect. The fixes still require a fresh native run.

## Files Updated

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/✏️update-path-geometry/✏️reshape-curve/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/✏️update-path-geometry/✏️reshape-curve/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/✏️update-path-geometry/✏️reshape-curve/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/🔄️update-layer-transform/📐️translates-and-scales-shape-a/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/🔄️update-layer-transform/📐️translates-and-scales-shape-a/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/🔄️update-layer-transform/📐️translates-and-scales-shape-a/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/🔄️update-layer-transform/📐️translates-and-scales-shape-a/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/🔍️update-layer-trace-params/🔍️sharpens-the-trace/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/🔍️update-layer-trace-params/🔍️sharpens-the-trace/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/🔍️update-layer-trace-params/🔍️sharpens-the-trace/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/🔍️update-layer-trace-params/🔍️sharpens-the-trace/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/🔀set-layer-boolean-operation/➖️union-to-subtract/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/🔀set-layer-boolean-operation/➖️union-to-subtract/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🔀️transform/🧫️fixtures/🧬️mutations/🔀set-layer-boolean-operation/➖️union-to-subtract/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🔃reorder-layer/⬆️moves-shape-a-above-shape-b/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🔃reorder-layer/⬆️moves-shape-a-above-shape-b/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🔃reorder-layer/⬆️moves-shape-a-above-shape-b/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/📋️duplicate-layer/🚫️rejects-a-missing-source-layer/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/📋️duplicate-layer/🚫️rejects-a-missing-source-layer/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/➕️create-layer/➕️appends-shape-b-at-the-root/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/➕️create-layer/➕️appends-shape-b-at-the-root/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/➕️create-layer/➕️appends-shape-b-at-the-root/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/➕️create-layer/➕️appends-shape-b-at-the-root/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes-group-a-with-its-child/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes-group-a-with-its-child/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧫️fixtures/🧬️mutations/👁️set-layer-visible/🙈️hides-shape-a/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧫️fixtures/🧬️mutations/👁️set-layer-visible/🙈️hides-shape-a/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧫️fixtures/🧬️mutations/👁️set-layer-visible/🙈️hides-shape-a/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧫️fixtures/🧬️mutations/✏️rename-layer/✏️renames-shape-a-without-touching-its-id/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧫️fixtures/🧬️mutations/✏️rename-layer/✏️renames-shape-a-without-touching-its-id/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧫️fixtures/🧬️mutations/✏️rename-layer/✏️renames-shape-a-without-touching-its-id/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧫️fixtures/🧬️mutations/🔒️set-layer-locked/🔒️locks-shape-a/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧫️fixtures/🧬️mutations/🔒️set-layer-locked/🔒️locks-shape-a/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🏷️metadata/🧫️fixtures/🧬️mutations/🔒️set-layer-locked/🔒️locks-shape-a/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🎨️replace-layer-fill/🌈️solid-to-linear-gradient/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🎨️replace-layer-fill/🌈️solid-to-linear-gradient/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🎨️replace-layer-fill/🌈️solid-to-linear-gradient/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🎨️replace-layer-fill/🌈️solid-to-linear-gradient/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🖊️replace-layer-stroke/🖊️adds-a-dashed-stroke/🦠️mutation/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🖊️replace-layer-stroke/🖊️adds-a-dashed-stroke/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🖊️replace-layer-stroke/🖊️adds-a-dashed-stroke/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🖊️replace-layer-stroke/🖊️adds-a-dashed-stroke/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🌓️set-layer-blend-mode/✖️normal-to-multiply/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🌓️set-layer-blend-mode/✖️normal-to-multiply/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🌓️set-layer-blend-mode/✖️normal-to-multiply/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🌫️set-layer-opacity/🌫️dims-shape-a-to-half/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🌫️set-layer-opacity/🌫️dims-shape-a-to-half/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/🎨️style/🧫️fixtures/🧬️mutations/🌫️set-layer-opacity/🌫️dims-shape-a-to-half/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/📖️.grammar.semio`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/📖️.grammar.semio`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/↗️affine/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`


Native run 24892 finished successfully: the full Draw library suite passes after painting both drag-test rectangles explicitly. This confirms the fixture now exercises interior movement under precise painted picking. Node-gesture changes added afterwards require a fresh validation run.
