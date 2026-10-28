# Large-Sheet Export

## Implementation

Removed the independent eight-frames-per-page and 1,024-total-frame semantic ceilings. Frame, override, style-run and layer-object validation now uses the existing decoded-item and output byte budgets; nested records advance one row per job work unit. Page and text-frame column counts must remain positive.

Vector sheet dimensions are finite positive native u32 dimensions; PNG retains its raster dimension/pixel budgets. Oversized unrelated pages do not block a selected smaller PNG page. Image link dimensions remain metadata rather than imposing raster output limits on vector exports.

Shared scene builders can shape only the document background or one resolved frame. PDF planning now advances background once, then at most one frame per fuel unit. The export cursor visits bounded layer rows with scalar state and keeps no additional retained clone buffers. It preserves parent-before-local layer stacking, overrides and layer visibility for PDF, SVG and PNG planning. Checkpoint replay still regenerates the exact completed work prefix; close ownership is unchanged.

## Verification

A portable 32-frame 8192 by 4096 vector-sheet fixture fixes expected paint order and x coordinates using installed lodash.sortBy 4.18.1. The independent oracle projection and TypeScript syntax check passed. Native test sources assert per-row validation, per-frame planning, vector admission, PNG raster refusal, cancellation, a checkpoint during frame planning, cancellation from that frontier and byte-identical resumed SVG output. Existing resource-cap laws now reject the decoded-item budget rather than a semantic eight-frame ceiling. All include_str paths in 28 inspected native source files resolve.

Native Rust compilation, export execution, cancellation and replay tests are NOT executed because the current repo runner exposes no working native test route. These remain test definitions and source inspection evidence only.

## Remaining Fidelity Work

SVG and PNG job encoders still use rectangle-based plan primitives inherited from the original implementation, representing text/images as rectangles. The next phase must reuse shaped text, actual image pixels and vector paths. PDF now shapes per frame, but each individual bounded story/font or background batch still runs synchronously within its work unit.

## Files

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📜️export-contract-and-state/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🏗️job-admission-and-close/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/✅️validate-plan-and-svg/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📕️pdf-png-and-package-encoding/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/🚧️caps-json-checkpoint-pdf-style/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/📄️large-sheet/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/📄️large-sheet/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🧫️fixtures/📄️large-sheet/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧩️assemble-display-list/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/⚙️layout-engine-and-views/🦀️.rs`
