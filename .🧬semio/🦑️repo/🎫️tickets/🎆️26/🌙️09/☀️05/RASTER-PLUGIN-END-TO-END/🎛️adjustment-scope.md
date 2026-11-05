# Non-Destructive Adjustment Expansion Scope

Read-only source audit. No implementation or test result is claimed.

## Contract First

Use a closed shared adjustment descriptor, discriminated by kind, rather than another unrestricted string/parameter map. Preserve brightnessContrast and add invert, grayscale, saturation, gamma, threshold and posterize. Invert/grayscale take no parameters. Reuse the existing editing ranges: saturation -1..1 (identity 0), gamma 0.01..10 (identity 1), threshold 0..255 (proposed creation value 128), posterize integer 2..256 (proposed creation value 8). Brightness/contrast remain -1..1 with identity 0. Define explicit absent/reset defaults, invalid fields and integer requirements in neutral schema fixtures before code. Existing destructive-edit validation allows fractional threshold; retain that shared law unless deliberately changing both contracts.

Keep the persisted Raster layer kind/params model synchronized across JSON Schema, GraphQL, Proto and Rust/TypeScript parsers. Author adjustment-kind and per-kind parameter vocabulary centrally, then have mutation admission, patch admission, stack projection and Inspector derive decisions from it. Reject unknown kinds and parameters before publication. Changing kind must have a specified atomic parameter policy: recommend an exact kind-and-params replacement that clears inapplicable old parameters and records both prior values for undo; the current change-kind mutation only replaces its string and otherwise strands old params.

## Exact Implementation Map

Paths below `P` are relative to `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any`.

| Boundary | Current source and required work |
| --- | --- |
| Shared compositor contract | `🧰️framework/🔨️modules/🔲️pixels/🧩️compositing/🧬️schema/🔣️.json`, `🟦️.ts`, `🦀️.rs`: CompositeContent/CompositeLayer adjustment contains brightness/contrast only. Replace its implicit affine-color program with closed adjustment operation data. Preserve bounded compile/render/retirement and unchanged alpha. |
| Shared stack contract | Same compositing root, `🗂️layers/🧬️schema/🔣️.json`, `🗂️layers/🟦️.ts`, `🗂️layers/🦀️.rs`, `🗂️layers/📋️plan/🦀️.rs`: TS RasterStackLayer const brightnessContrast and Rust RasterStackContent::BrightnessContrast require expansion. Rust keyless pixel currently compiles to identity brightness/contrast; retain explicit identity behavior when refactoring. |
| Raster persisted model | `P/🧬️schema/🔣️.json`, `🔗️.graphql`, `🛰️.proto`, `🟦️.ts`, `📐️layer-model/🦀️.rs`: TS currently parses adjustmentKind as any string and params as IntrinsicValue map; Rust model similarly owns dynamic params. Add per-kind validation while preserving owned-map retirement. Audit editable binary/snapshot codecs for typed contract changes. |
| Semantic mutation | `P/🧬️schema/🧬️mutations/🎛️change-layer-adjustment-parameter/{🦀️.rs,🧬️schema/🔣️.json}` and parent `🟦️.ts`: only brightness/contrast, range -1..1, and brightnessContrast target kind are admitted. Extend expected-value guards by kind and exact ranges. `🎚️change-layer-adjustment-kind/{🦀️.rs,🔺️diff/🦀️.rs,↩️inverse/🦀️.rs,🧬️schema/🔣️.json}` needs atomic kind/params behavior and invalid-kind refusal. |
| Sparse patches | `P/🧬️schema/🔺️diff/{🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts,🌲️layer-tree-apply/🦀️.rs}`: adjustmentParameters currently caps at two entries and names brightness/contrast. Match schema/parser/native validation to per-kind rules and exact inverse. |
| Retained command/UI | `P/✏️editor/🎮️commands/🩹️patch-layer/🦀️.rs`: known field and parameter mutation dispatch hardcode brightness/contrast; adjustmentKind falls back to brightnessContrast for non-string conversion. `P/✏️editor/📌️panels/🔍️inspection/🦀️.rs`: value line 37, range line 96, rows 135–137 only handle brightnessContrast; no adjustment-kind selector is currently rendered. Add EN/DE kind selector, per-kind parameter labels/ranges, mixed-selection intersection and current layer protection. `P/✏️editor/🔌️app-assembly/🦀️.rs` owns current labels and must expand. |
| Export/merge/flatten | `P/🚪️io/🧩️composite-and-media/🦀️.rs` lines 44–47 reject any other kind and project only brightness/contrast. Extend the one retained stack projection; merge and flatten must use it so the displayed result matches saved flattened export. |
| Native displayed canvas | `🧰️framework/🔨️modules/🗺️surface/🎨️paint/📄️document-json/🦀️.rs`: AdjustmentParamsJson at 114–117 only retains two fields. `🧩️compositing/🦀️.rs` rejects other kinds and projects BrightnessContrast. Update both, including exhaustive content traversal. Otherwise export may succeed while native canvas refuses the same document. |

## Pixel Semantics and Work Bounds

Existing destructive color laws in `🧰️framework/🔨️modules/🔲️pixels/✍️editing/🟦️.ts` lines 144–152 already define invert, luma grayscale (0.2126/0.7152/0.0722), saturation, gamma, threshold and posterize. Reuse a codebase-owned scalar color evaluator in both languages; do not run whole PixelEditJob per backdrop/adjustment or add an image copy per layer. Compositor RGB is normalized float, alpha is separate. Current adjust step computes front from backdrop, blends front with backdrop using the layer blend mode, interpolates by layer opacity/coverage, and leaves alpha unchanged (TS compositor lines 167–169). Preserve this composition order. Keep RGB float through nested adjustment layers and round only final output; tests must pin this against accidental per-layer byte rounding. Preserve existing brightnessContrast slope/intercept semantics explicitly.

These six additional operations are constant work per output pixel; compile validates once and retains a closed operation. Existing per-pixel grants and retirement remain sufficient. No extra whole-image preparation or asynchronous authority is required. Cancellation must discard unpublished candidate and preserve prior committed output.

## Required Neutral and Independent Verification

Extend shared compositor/stack fixtures for every kind, defaults, invalid parameter/kind, fractional posterize refusal, transparent pixels, fractional opacity, blend modes, group isolation, stacked adjustments and unchanged input bytes. Run each fixture through Rust and TypeScript, compare exact decoded output to independent Sharp/libvips operations or independently authored channel lookup where its operation order differs; document rounding tolerance instead of claiming exact equality incorrectly. Existing `P/🧬️schema/🧪️tests/🎛️adjustment/🟦️.ts` already validates neutral parameter fixtures through AJV and parser agreement; expand its fixture and Rust companion first.

Retained native tests must cover kind/parameter expected-revision refusal, reset, invalid publication, undo/redo, rename/lock guards, editable pack reload and flatten/export equivalence. Inspector tests must inspect both language controls and emitted semantic payloads, plus mixed different adjustment kinds. Mounted/native canvas projections must render the same new kind; passing shared pixel tests alone cannot prove this wiring.
