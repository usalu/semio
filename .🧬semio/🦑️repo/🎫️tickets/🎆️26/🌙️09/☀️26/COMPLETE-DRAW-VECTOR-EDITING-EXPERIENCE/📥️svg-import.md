# Editable SVG Import

SVG import now has a source-level registration in Draw's IO registry, backed by an incremental owned document builder. Native execution and the end-user import flow remain unverified. The implementation retains editable paths, groups and text instead of flattening incoming artwork to a bitmap.

## Path Semantics

Relative coordinates, repeated commands, implicit lines after moves, horizontal/vertical lines, smooth quadratic/cubic controls, arc flags, and closed contour origins normalize to Draw's owned absolute path segments. Nonfinite coordinates, missing operands, invalid flags, unknown commands, negative arc radii, and paths without an initial move fail before a result is returned.

The [SVG 1.1 path specification](https://www.w3.org/TR/SVG11/paths.html) requires smooth cubic controls to reflect only a preceding cubic command, and smooth quadratic controls to reflect only a preceding quadratic command. A command following close starts a new contour at the closed contour's start. These rules inform the neutral fixtures.

## Third-Party Oracle

The installed Three.js SVGLoader incorrectly shares a previous control between quadratic and cubic families. The mixed `Q L T S` fixture exposes that discrepancy: its final cubic first control must be `(40,0)`, whereas SVGLoader reflects the previous quadratic control to `(50,0)`. Local SVGLoader source confirms it unconditionally reflects the shared `control` value in its `S` case.

That fixture explicitly records an independently authored SVG with full controls and an `oracleReason`. Other valid fixtures compare their original SVG directly against normalized geometry through SVGLoader, with xmldom providing XML parsing. The Draw implementation was not changed to reproduce the reference parser's error.

## Verification

- `tests-svg-path-import-red.txt`: missing implementation reproduced.
- `tests-svg-path-import-current.txt`: 135 passed / 1 failed; the mixed-curve third-party discrepancy above.
- `tests-svg-path-oracle.txt`: registered `@semio-tech/draw-js:test` passed: 136 tests, 18,688 assertions, 23 files. Existing field-patch and publication-authority checks also passed.
- Rust normalization is authored but not yet verified by a completed native run. Run 9208 stopped before Draw compilation on twelve shared OS-kernel errors; see `tests-native-selection-settled.txt`.

## Remaining Work

Implement the document importer with preserved hierarchy, transforms, styles, gradients, text and embedded images; register its deserializer; connect the end-user import flow; verify editable round trips and progress/cancellation. Permissive separator handling inherited from the current first-party SVG parser is not strict grammar validation.

## Fill-Rule Prerequisite

The import investigation found that path scene projection hardcoded even-odd while pointer picking used nonzero. [Authored Fill Rules](🌀️fill-rules.md) now records the persisted vocabulary, projection/picking changes, fixture updates and verification. SVG document import must explicitly apply the source winding rule. Its semantic mutation and bilingual inspector are now authored; native/browser validation remains pending.


## Editable Transforms

Added owned Rust and TypeScript SVG transform normalization beside the path importer. Matrix, translate, scale, centered rotation, skew X/Y, reflection and collapsed-axis transforms retain the complete six-coefficient affine matrix through Draw's decomposition. The Rust implementation uses the existing first-party SVG transform parser after grammar validation; TypeScript uses owned token parsing and the shared affine primitives. No runtime library dependency was added.

The [SVG 1.1 coordinate-system specification, §7.6](https://www.w3.org/TR/SVG11/coords.html#TransformAttribute) defines transform order, rotation centers, numeric grammar and separators. The neutral cases retain eleven valid matrices and reject seventeen malformed/nonfinite inputs. Both implementations check argument/function separators and the SVG whitespace vocabulary; reject undefined skew angles and overflowing composition; and preserve shear rather than reducing a matrix to rotation and scale alone.

The independent Three.js SVGLoader oracle transforms three noncollinear path points for every valid case, then compares them with Draw's reconstructed affine. This exercises the authored transform strings, not an oracle matrix constructed by the implementation.

Evidence:

- `tests-svg-transform-red.txt`: registered target failed on missing transform implementation (exit 1).
- `tests-svg-transform-current.txt`: 162 tests / 18,890 assertions passed (exit 0).
- `tests-svg-transform-grammar-red.txt`: malformed separator cases reproduced acceptance bugs (exit 1).
- `tests-svg-transform-grammar-current.txt`: **170 tests / 18,906 assertions / 25 files passed** (exit 0), plus 48 field-patch cases and the 36-command publication audit.
- Rust fixture test is registered and authored, but a completed current native run has not yet verified it. Native run 79513 and component run 93519 remain live. They were started before these transform sources, so their eventual source coverage must be checked.

This finishes the transform-normalization prerequisite. It does not yet provide document import, viewBox mapping, inherited presentation/gradients, hierarchy construction or the end-user import command. Those remain required, including progress and cancellation for large documents. The goal stays active.


## Document Construction and IO Registration

Added `SvgImportJob` twins with explicit `step`, `cancel`, progress and terminal `take` methods. A cancelled, failed or unfinished job cannot publish a partial document. Supported document construction includes nested groups, normalized paths, rectangle/rounded rectangle/circle/ellipse/line/polygon/polyline geometry, basic text, local transforms, root viewBox fitting, inherited solid paint, fill rules, stroke caps/joins/dashes and per-layer opacity. IDs derive from structural positions and names retain source IDs/labels. Text coordinates account for Draw's top-origin text positioning. Zero-width/height rectangles and zero-radius ellipses produce no painted geometry.

The Rust `SvgIntoDraw` deserializer is registered in `io()` for SVG 1.1. It accepts text or UTF-8 bytes and reports semantic fidelity. The shared IO registry's `resolve_ready` **requires completion on its first poll**; an actually suspending importer there would panic for multi-batch documents. The deserializer therefore drains the job synchronously to obey that currently authoritative boundary. The editor import workflow must drive the job directly through scheduled steps to provide UI cancellation and responsiveness. Native tests explicitly cover a >64-node call through the actual registered entry, but remain unverified.

Seven neutral documents cover hierarchy/inherited paint, root viewport/text placement, explicit unsupported-feature errors and nonpainted zero-width geometry. The TypeScript test compares transformed paths against Three.js SVGLoader and separately checks cancellation without partial publication. Rust tests read the same expected snapshots and exercise both payload types.

Evidence:

- `tests-svg-document-red.txt`: missing document implementation reproduced (exit 1).
- `tests-svg-document-current.txt`: **177 tests / 18,918 assertions** passed (exit 0), plus 48 field-patch cases and 36-command publication audit.
- `tests-svg-document-zero-red.txt`: zero-width rectangle incorrectly emitted a painted line; fixture failed before the fix (exit 1).
- `tests-svg-document-registered.txt`: **178 tests / 18,919 assertions / 26 files passed** (exit 0), plus the existing audits. This is TypeScript evidence; it does not prove the native IO registration runs.
- Native run **79513** and component build **93519** remain live. Both predate this source. Confirm coverage on completion and schedule fresh native/component validation as needed; do not restart a live job.

Remaining import work: gradients and paint servers, embedded images, use/symbol references, CSS cascade and the wider color/length vocabulary, text spans/font fidelity, clipping, nested viewports, group compositing isolation, strict document-level syntax checks, and app workflow/publishing tests. XML parsing, path tokenization, group child expansion and cancellation cleanup are not yet fully budgeted; current progress counts hierarchy work, not every byte/allocation. The overall editing objective remains incomplete and active.

- Final schema-aware follow-up `tests-svg-document-schema-current.txt` passed **178 tests / 18,926 assertions / 26 files**, 48 field-patch cases and 36-command publication audit (run 54281, exit 0). Native 79513 and component 93519 were polled and confirmed live again.


## Linear Gradient Paint Servers

Linear SVG paint servers now import into editable `FillStyle::LinearGradient` in both language implementations. Local `href`/`xlink:href` chains resolve forward references; cycles/missing definitions fail explicitly. Definitions support object-bounding-box and user-space coordinates, gradient transforms, inherited stops, inline stop colors/opacity, clamped monotonic offsets, fill opacity and degenerate/no-stop/single-stop behavior. Unsupported spread modes and radial/text gradient coordinates remain explicit errors.

The [SVG 1.1 gradient specification](https://www.w3.org/TR/SVG11/pservers.html#LinearGradients) explains why non-square bounding boxes change the gradient normal, not only its endpoints. Normalization transforms the color-rate vector with the inverse transpose, then reconstructs equivalent endpoints in local layer coordinates. The [stop rules](https://www.w3.org/TR/SVG11/pservers.html#GradientStops) specify clamping and nondecreasing offsets; offsets are not sorted, preserving abrupt color transitions.

Two independent Sharp/libvips SVG raster comparisons cover a non-square diagonal gradient and a transformed forward-reference gradient with inherited stops and opacity. The tests compare every RGBA byte within a two-level tolerance. Sharp is test-only, already installed in the workspace; the importer uses owned code and system/first-party parsers.

- `tests-svg-gradient-red.txt`: missing paint-server support reproduced (exit 1).
- `tests-svg-gradient-current.txt`: 180 tests / 18,930 assertions passed (exit 0).
- `tests-svg-gradient-references.txt`: **185 tests / 18,940 assertions** passed (exit 0), including both independent raster comparisons and the 48 field-patch / 36-command audits.

Native tests consume the expanded document fixtures but remain unverified. Runtime app import, radial/focal/elliptical gradients, gradient strokes, repeat/reflect spread and the full editing acceptance list remain incomplete. The existing radial fill model lacks a full gradient coordinate system and focal point, so those cases need a schema/renderer extension rather than an approximate import. Definition indexing is still synchronous and must join the fully budgeted import work before the UI workflow is complete.

- Final gradient run `tests-svg-gradient-final.txt` passed **186 tests / 18,942 assertions / 26 files** (exit 0), plus the 48 field-patch cases and 36-command audit. A preceding red case proved that unsupported linear-light color interpolation was accepted; referenced gradients now reject it explicitly instead of changing appearance silently.
- Native 79513 completed **407 tests: 404 passed, 3 failed** (`tests-native-fill-rule-mutation.txt`). It started before the importer; coverage of later sources is not confirmed. Failures were assertion representations: an omitted false UI flag, JSON integer-vs-float identity, and an omitted empty point-selection domain after blank click. Corrections preserve the behavioral assertions. The earlier hypothesis that the point test failed on drag start was wrong: the actual failure was its final empty-selection check at line 1765, after preview/cancel/history assertions. Fresh native run **88159** is live in `tests-native-svg-gradients.txt`.
