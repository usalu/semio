# SVG and PNG Fidelity

The rectangle-only SVG/PNG routes have been removed. Both formats plan the actual page display list used by PDF: parent resolution, overrides, layer order, background paths, shaped glyphs, frame rotations and actual proxy images. Glyph font identity survives shaping; immutable source access returns only first-party FontHandle and byte-slice types. SVG streams real quadratic/cubic contour commands and source image data URLs, with alpha and transforms. PNG uses nonzero winding, adaptive curve flattening at 0.025 page units, 4×4 pixel sampling, round strokes, source-over compositing and inverse-rotated bilinear RGBA proxy sampling.

The existing first-party TrueType admission/outline cursors supply genuine glyphs. Existing first-party ImageDecodeJob prepares PNG/JPEG proxies using the original StepContext retained grant; decoding, source byte copies, font admission, glyph points, curve subdivision, path commands, pixel edge tests and output slices all yield between bounded units. Aggregate font bytes, image source bytes, decoded image pixels, outline points, original segments and flattened edges have explicit resource limits. Unsupported color-font sources, font collections, absent image proxies and unsupported image media fail explicitly instead of substituting a frame rectangle.

A Fidelity close stage retires every original font, glyph cursor, path and image decoder through the repository controlled-retirement interface under the original grant. Existing private PDF item cleanup was tightened to retire path points, RGBA/RGB pixels, proxy strings and marker strings in bounded byte slices. Interchange format additions made by the other agent are retained before this stage.

## Verification

The language-neutral export-fidelity JSON corpus was authored before implementation. Its two path cases, two glyph witnesses, rotated transparent RGBA image case and complete layout document are shared with native laws. Three JavaScript oracle laws actually passed through the TypeScript compiler API and a small bun:test-compatible assertion harness: sharp/libvips SVG rasterization, pngjs independent PNG encoding/decoding, and opentype.js genuine font contours. All new oracle records validate against OracleRegistryEntry. The existing registry as a whole still contains mutationCatalogs, which the current OracleRegistry definition rejects; this pre-existing metadata mismatch was not rewritten.

Native laws are mounted for fill/stroke/alpha/rotation probes against tiny-skia, actual source-font glyph contours against ttf-parser, complete SVG/PNG artifact pixels, and cancellation/checkpoint replay while glyph preparation is active. Existing large-sheet laws now inspect actual shared display-list items instead of the removed rectangle buffer. Strict tree-sitter Rust syntax checks passed for 40 export/scene/font source files. TypeScript syntax checks passed. All include paths and package paths were checked directly.

Native compilation, native law execution and a GUI visual run remain unverified: the available repo runner does not execute this Rust suite. Tree-sitter and JavaScript oracle success do not establish Rust type correctness.

## Remaining limits

The page view still shapes a whole frame/background text batch synchronously in the pre-existing scene builder; the new glyph/paint/image work after that point is resumable. The existing PDF engine teardown remains a whole TextSystem disposal. Color-font painting and collection-font extraction are explicit refusals. The PNG route retains its existing 2048-dimension/4,194,304-pixel output limits. Raster antialiasing is a deterministic approximation (4×4 coverage and 0.025 contour tolerance), while SVG retains exact source curves. No complete feature or native runtime claim is justified.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖋️text/🅰️text-styles-and-text-system/🦀️.rs`
- `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️outline/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🎨️display-list-and-hit-testing/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/✍️drawing-labels-and-rasters/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔠️style-spans-and-preview-marks/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧩️assemble-display-list/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/⚙️layout-engine-and-views/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📜️export-contract-and-state/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🏗️job-admission-and-close/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/✅️validate-plan-and-svg/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📕️pdf-png-and-package-encoding/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/⏭️advance-commit-and-codec/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🎨️faithful-primitives/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/📄️large-sheet/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/🎟️harness-pdf-and-dispatch-laws/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🎨️fidelity/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🎨️fidelity/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🎨️export-fidelity/🔣️.json`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json`

## Audit Repairs

The later fidelity audit findings are repaired in the current source: Fidelity close dispatch reaches the original retained close route; text runs now carry frame rotation and pivot independently of blueprint chrome. The exact regression laws, four executed independent oracle results and native verification limitation are recorded in protocol-alignment.md.
