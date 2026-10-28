# Layout Architecture and Conformance Audit

Inspected 2026-10-21. Read-only source inspection; no build or conformance run is claimed.

## Existing Layout Architecture

The artifact root is `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any`. JSON, GraphQL, Proto, text grammar and binary protocol contracts already exist. `🧬️schema/🧬️mutations` has per-verb Rust operations, schemas, inverse functions and language-agnostic before/mutation/diff/after/outcome fixtures. `↔️change-page-width/🦀️.rs` demonstrates semantic diff construction, missing-target diagnostics, no-op warnings, inverse operations and English/German labels.

The central Rust authority is `🧬️schema/🔺️diff/🧮️diff-apply-and-algebra/🦀️.rs`, implementing `MutationDiff<LayoutSnapshot>::apply` with positional collection deltas and checked error paths. The TypeScript diff file currently exposes interfaces and parsers rather than an apply/algebra implementation. The TypeScript mutation file mirrors the closed externally tagged Rust enum.

The existing Rust editor has substantive canvas, inspection, transform commands and export jobs. `✏️editor/🕹️layout-play-app/🦀️.rs` currently accepts only `fields:in` form.dictionary media (lines 263 onward), storing it as data fields; it does not bind values to story rendering. `artifact:out` exports the native pack. Resumable export job code has checkpoint and cancellation infrastructure.

Rust SVG/DXF/DWG import adapters exist under `🚪️io/📥️import/🧩️deserializers`; SVG calls `layout_document_json_from_format`. It explicitly declares lossy trace import: closed axis-aligned rectangles become pages and original drawing becomes the background; stories, native frames and styles are not recovered. Corresponding TypeScript serializer/deserializer files contain only `export {};`. No STEP, PDF import or IDML adapter path exists in the inspected layout tree. Native PDF/SVG export is implemented under `✏️editor/⚙️engine/📤️export`, separate from the empty TS format adapters.

No .svg/.pdf/.step/.stp/.idml official sample binaries were found inside the layout plugin. Its examples manifest references a sample layout; the tree provides JSON mutation and export fixtures.

## Reusable Canonical Format Engines

Use `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts` as the canonical codec domain; its AGENTS.md explicitly owns real codecs and framework SDK traits. SVG lives in `🎨️svg`, PDF in `📖️pdf`, STEP in `📐️step`, ZIP in `🎒️zip`, XML in `📰️xml`. No IDML artifact directory exists. Do not introduce a parallel parser into layout.

SVG crate root reexports `SvgSnapshot`, `SvgDiff`, `SvgMutation`; standard is 1.1 with base/tiny/basic validators and registered native codecs. Its base generator codec is `🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/🦀️.rs`.

STEP crate root reexports `StepSnapshot`; its owned standard is AP214 with cc1–cc6 validators and native codecs. Base snapshot/I/O is under `📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base`. AP203/AP242 samples must not be represented as AP214 conformance proof merely because all use Part 21 syntax.

PDF crate root exposes versions 1.4 and 1.7. The 1.7 base includes lexical, xref, encryption, font, image, content, lift/lower and writer modules. Reusable I/O path is `📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io`. Root comments explicitly scope the 1.4 PageDoc model and separate PDF/A, PDF/X and other subsets. Inspection did not establish full standard conformance.

## Primary Fixture Sources and Honest Scope

- [W3C SVG 1.1 Second Edition Test Suite](https://www.w3.org/Graphics/SVG/Test/20110816/) supplies accepted tests and an archive. Prefer accepted tests; draft-watermarked tests are explicitly under development. Tests cover SVG rendering behavior, not native layout round-trip semantics. Pair selected tests with own page/frame/story expectations and an independent SVG renderer.
- [PDF Association Isartor suite](https://pdfa.org/resource/isartor-test-suite/) deliberately violates individual PDF/A-1b requirements to test validators. It is a negative PDF/A validation corpus, not proof of arbitrary PDF import fidelity or PDF 1.7 compliance. [Terms and download](https://pdfa.org/isartor-test-suite-terms-of-use-download/) must accompany any vendoring decision; preserve provenance.
- [NIST official STEP models](https://www.nist.gov/ctl/smart-connected-systems-division/smart-connected-manufacturing-systems-group/mbe-pmi-0) supplies AP203/AP242 geometry and PMI models, FTC/STC/CTC definitions and validation results. Scope assertions to actual schema and model: geometry-only AP203 differs from graphical and semantic PMI AP242. NIST permits unrestricted use, requests acknowledgement and disallows promotional logo use. These do not certify the existing AP214 implementation.
- [Adobe supported formats](https://helpx.adobe.com/indesign/desktop/get-started/system-and-product-info/supported-file-formats.html) establishes IDML as the interchange target. Official historical Adobe IDML specification and cookbook URLs under `/content/dam/acom/en/devnet/indesign/sdk/cs6/idml/` returned 404 during this audit; an Adobe community attachment URL for the specification also returned 404. Therefore no retrieved official IDML fixture corpus is claimed. Do not label handcrafted or third-party examples official. Obtain an accessible Adobe SDK/schema and Adobe-produced IDML packages, record exact DOMVersion and package provenance, then test ZIP packaging/XML references/styles/pages/stories and open/export through InDesign as independent oracle.

Full SVG/PDF/STEP/IDML compliance needs format-specific coverage, declared unsupported constructs, independent oracle checks and round-trip semantics; passing a small fixture subset establishes only that subset.
