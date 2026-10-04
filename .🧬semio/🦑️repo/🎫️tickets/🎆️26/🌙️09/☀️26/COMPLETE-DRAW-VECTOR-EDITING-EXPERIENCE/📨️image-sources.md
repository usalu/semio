# Encoded Image Source Preparation

Current integration is recorded in [encoded scene assets](🖼️scene-assets.md): the scene contract now references shared encoded asset identities, the TypeScript consumer has executed decode/filter/group/blend and progress/cancellation verification, and the native twin is authored but blocked before Draw test execution by shared dependencies. The production drawing-document producer and end-user export flow remain required. The prerequisite notes below retain their historical context.

The active Draw ticket needs a shared, resumable boundary between document image strings and the verified PNG decoder. Existing tracing, document conversion, and PDF consumers decode image strings synchronously and can omit malformed images without an actionable error.

## Contract and Ownership

The image preparation schema admits declared media type, encoded source, source byte cap, binary byte cap, pixel cap, and chunk cap. Current codec support is PNG; unsupported media types must fail explicitly. Bare canonical standard base64, base64 data URLs, percent-escaped base64 data URLs, and percent-encoded binary data URLs share one private publication boundary.

The job validates source tokens before allocating binary output, then decodes with a fixed quartet scratch buffer and delegates PNG reconstruction one work unit at a time. Native source ownership uses an Arc<String>. Cancellation and failure release private source, bytes, and pixels; incomplete results cannot escape. Async TypeScript preparation checks cancellation around progress observers and yields between grants.

The transport rules follow [RFC 2397](https://www.rfc-editor.org/rfc/rfc2397.html) and [RFC 4648](https://www.rfc-editor.org/rfc/rfc4648.html). Data URLs distinguish a final base64 flag from named parameters. Binary data URLs encode unsafe bytes as percent escapes. Standard base64 rejects whitespace, URL alphabet, misplaced padding, and noncanonical unused bits.

## Test-First Evidence

The schema and shared fixtures were authored before implementation. Forty-seven valid sources cover the PNG corpus and transport variations; twenty-five negative sources cover declared type, header, transport, limits, and corrupt PNG. TypeScript uses system data URL decoding and pngjs; native tests use independent base64, percent-encoding, and png libraries.

The stub TypeScript owner command terminated with 25 passes and 49 expected failures. Native compilation succeeded and the stub command terminated with 0 passes, 1 expected failure, and two remaining focused tests stopped by fail-fast. These are red tests, not implementation verification.

## Remaining Work

Implement and verify both jobs, run the shared base64 corpus and full pixel owner suites, regenerate launch entries, and integrate the preparation job into Draw's scene/asset pipeline. This kernel alone does not complete image editing or validate end-user behavior.
## Implemented Boundary

Both first-party jobs now admit source limits, parse bounded data URL headers, validate canonical base64 or percent-encoded binary transport before binary allocation, and reconstruct PNG pixels through the existing cooperative PNG decoder. Every source step consumes one token, each base64 quartet writes at most three bytes, and the nested PNG job receives one work unit per parent step. Header parsing is capped at 4 KiB. Native binary storage reserves admitted capacity and appends bounded groups; TypeScript allocates the admitted typed-array buffer. These allocator calls are bounded by the supplied cap and are not separately interruptible.

The native error and source/progress/result types belong to the codebase; native source storage is Arc<String> and decoded bytes move into the PNG job without a full copy. The TypeScript async wrapper yields between grants and checks abort before construction and before result publication, including after progress observers. Cancelled/failed jobs release private candidates; previously published pixel storage survives job cancellation.

The existing base64 owner now shares allocation-free quartet validation across image preparation and ordinary decoding. New tests cover the neutral RFC corpus, deterministic independent codec comparisons, base64url separation, malformed groups, and unchanged caller storage on failed writes. No third-party runtime dependency was added. Independent codecs are test-only.

The negative image corpus is now 36 sources, including all declared resource minima/maxima and empty/oversized media declarations. The previous whitespace fixture represented a literal backslash+n; it now contains an actual newline. The uppercase metadata fixture was decoded correctly by both jobs; Bun's data-URL fetch oracle refused its uppercase base64 flag. Only the oracle metadata is normalized to lowercase; all implementation runs retain the original source.

## Verification Recorded So Far

- Stub reds: TypeScript 25 passed / 49 failed; native 0 passed / 1 failed with the other focused groups stopped by fail-fast.
- Initial implemented native preparation: all 3 focused groups passed, including 47 shared sources under grants 1, 7, and 4096.
- Full pixel/base64/DEFLATE owner run before the final added limit cases: strict TypeScript and 373 tests passed, 1,038,673 assertions across 10 files; native 88 tests passed, zero skipped.
- Added exact-limit and every-live-phase lifecycle checks: 75 focused TypeScript tests passed with strict checking; all 4 focused native groups passed.
- Full Draw TypeScript owner: 361 tests passed, 228,871 assertions across 35 files, plus the owner strict/field/publication checks.
- Launch generator terminated successfully, and both image preparation commands are present in the generated launch configuration.
- Draw native baseline terminated unsuccessfully on a shared mesh slice-length callback. Replacing Vec::len with the slice-compatible length closure allowed compilation past that file. The corrected Draw command terminated unsuccessfully on shared graph manifest calls to the old one-argument ValueError::new constructor. Native Draw tests did not execute. The mesh change is compilation repair; no new mesh behavior is claimed.
- Final expanded corpus owner commands both terminated successfully: strict TypeScript plus **385 tests passed, zero failures, 1,045,142 assertions across 10 files**; native pixels/DEFLATE/base64 **89 tests passed, zero skipped**. Runtime diagnostics confirm all 47 valid / 36 negative encoded sources, exact source/binary/pixel limits, cancellation in every live preparation phase, async yielding/abort, and independent codec comparisons. A targeted whitespace check also passed.

## Required Draw Integration

This preparation boundary is not yet called by Draw's resolved scene raster job or the production export producer. Existing consumers still include synchronous/optional image decoding. PNG support alone is not a complete image format/import UI.

The next scene contract should carry an explicit source asset catalog and image nodes referencing asset identities with authored geometry dimensions. It needs schema-first mandatory source/byte/chunk limits, aggregate source-byte admission, aggregate decoded pixel admission before allocation, one decode per shared asset, and cancellation during source and PNG phases. Image transforms must scale intrinsic pixels into the authored image rectangle before existing affine filtering. Hidden, singular, or offscreen nodes can skip decoding after their reference/geometry contracts are validated.

The TypeScript and Rust scene owners must share neutral encoded-source composition fixtures, independent image/composition oracles, exact budget checks, duplicate/missing asset failures, cache reuse, nested opacity/blending, and private cancellation. Existing fixtures and all root input literals must be updated together, without an omitted-field compatibility default. Native Draw's shared dependency compile errors still require a successful full owner run before end-user validation.

The broad Draw goal and ticket remain active. Repo MCP lifecycle tools are unavailable in this session; no ticket close or goal completion was performed.

## Files in This Continuation

- Created `🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🧬️schema/🔣️.json`.
- Created `🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🧫️fixtures/🔣️.json`.
- Created `🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🧫️fixtures/⚠️invalid/🔣️.json`.
- Created `🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🟦️.ts`.
- Created `🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🦀️.rs`.
- Created `🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🧪️tests/🟦️.ts`.
- Created `🧰️framework/🔨️modules/🔲️pixels/🖼️image/📥️decode/🧪️tests/🦀️.rs`.
- Updated `🧰️framework/🔨️modules/🚪️io/🔤️base64/🟦️.ts`.
- Updated `🧰️framework/🔨️modules/🚪️io/🔤️base64/🦀️.rs`.
- Updated `🧰️framework/🔨️modules/🚪️io/🔤️base64/🧪️tests/🔬️unit/🦀️.rs`.
- Created `🧰️framework/🔨️modules/🚪️io/🔤️base64/🧪️tests/🔬️unit/🟦️.ts`.
- Updated `🧰️framework/🔨️modules/🔲️pixels/🦀️.rs`.
- Updated `🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust/Cargo.toml`.
- Updated `🧰️framework/🔨️modules/🔲️pixels/📋️project.json`.
- Updated `🧰️framework/🔨️modules/🔲️pixels/📜️script.ts`.
- Updated `.vscode/🧩️launch.seed.jsonc`.
- Updated `.vscode/launch.json`.
- Updated `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🛠️modeling/🦀️.rs`.
- Cargo owner preparation updated the native dependency inventory in `Cargo.lock`; unrelated concurrent lock changes were preserved.
- Created this report and updated `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-DRAW-VECTOR-EDITING-EXPERIENCE/📥️image-decode-preparation.md` and `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-DRAW-VECTOR-EDITING-EXPERIENCE/📋️acceptance.md`.
- Generated verification logs live only under the active ticket's `🗑️generated` directory. No permanent auxiliary script was created; no input/report file was removed.
