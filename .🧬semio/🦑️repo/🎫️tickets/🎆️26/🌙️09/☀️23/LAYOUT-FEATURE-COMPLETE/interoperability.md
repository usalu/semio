# Layout Interoperability

## Implemented Boundaries

Layout owns semantic mapping in its `🚪️io/🔀️interchange` interface. Canonical firstparty stdio codecs own PDF1.7, generic ISO10303-21/AP214 output, PNG, ZIP and XML syntax. Public interchange results carry native/editable-subset/projection fidelity plus explicit diagnostics. Import/export refuse byte, item, page, geometry and package-safety violations and accept progress/cancellation callbacks. No external library was added as a runtime dependency.

- SVG/DXF/DWG use Layout's neutral embedded-child interface, now bound by the artifact-owned Drawing composition implemented by the parent agent. SVG import creates one canvas page and retains actual background text, fills and transforms; rectangles are not mistaken for pages. Printable native export scene now includes actual story text, paint, proxy image bytes and center-based rotations in native radians, plus parent overrides/layer visibility. Neutral scene text currently has no font-family/size schema: this path is not a typography-conformance claim.
- PDF import creates editable pages, positioned Unicode stories and supported rectangle/raster frames. Supported text matrices, font names/sizes, paint order, RGB/gray paint alpha, transforms and PNG alpha masks are mapped. PDF export uses canonical embedded TrueType font/CMap generation and canonical raster XObjects. Nonrectangular paths, clipping, effects, annotations, complex fonts and unsupported image encodings remain explicit limitations. JPEG soft masks are refused. This importer accepts general PDF; the veraPDF ISO-rule-negative witness is not a PDF syntax-negative fixture.
- STEP import uses canonical indexed Part21 projections for LINE/CIRCLE trims, geometric curve sets and bounded polylines, AXIS2 placements and SI units. Orthogonal closed XY paths become editable centre-rotated rectangles with RGB fill/stroke; positioned Unicode annotations become editable styled text fragments with UTF8 ranges. Remaining curves retain per-curve appearance in a Drawing trace. Named Layout page solids define the canvas without duplicate face frames. NIST FTC11 AP203 remains six circular traces/390points. Direct and resumable exports now emit the selected page solid in millimetres, resolved visible parent/body rectangle geometry, RGB fill/stroke styles, Unicode text annotations and image placement/source annotations. It does not establish complete AP214 conformance, full paragraph flow/font fidelity, embedded pixels/alpha, topology/PMI/assembly transforms or general spline support. Background Drawing export now uses the borrowed scene cursor; compound/over-envelope filled contours still refuse explicitly.
- IDML is admitted through its own firstparty `s.stdio.idml@8.0/*` package identity, never generic ZIP presented as InDesign. It maps rectangular pages/frames, layers, Unicode text and UTF8 style runs, line breaks, text threads, paragraph/character typography, RGBA rectangle fill/stroke, embedded PNG/JPEG bytes, external image links, one-page parent templates, actual spread membership/page coordinates, rectangular guide bounds and center rotation. Strict package admission checks mimetype-first/stored, safe member paths, duplicate members, bounded XML and DOMVersion. Fill/stroke opacity uses independent transparency settings. Styles support font family, size, weight/italic face inference, tracking, leading and alignment; named BasedOn chains and inline overrides resolve into editable native styles. Pages retain margins/columns and text frames retain columns/insets/wrap modes. MasterSpread/AppliedMaster preserve native one-page parent templates. PNG proxy bytes (including alpha) and JPEG bytes are embedded as real graphic Contents, with external links retained when no proxy is available. Native spread IDs/names/member order and per-page origins are retained. Rectangular native guides become four standard IDML ruler guides grouped by standard Label properties; external ruler guides import as zero-width/height editable guide bounds. Direct and resumable exporters preserve source layer-stack order rather than sorting identity strings. Unthreaded rectangular/text/image master-item overrides now map via actual Page.OverrideList suppression plus editable spread copies, retaining sparse bounds/visible/locked presence through standard relationship labels. External unlabeled OverrideList entries import as hidden inherited-item overrides plus ordinary body items. MasterPageTransform translations become editable bounds overrides, and body spreads that hide master items become visibility overrides; unsupported affine/nonfinite transforms refuse explicitly. Shared raster assets retain one native asset identity while each IDML graphic/link placement has a distinct Self. Multi-page/nested masters, threaded master-item overrides, groups/effects, nonrectangular paths, print/data semantics and color profiles remain incomplete. No official Adobe fixture or Adobe desktop interoperability run was available; old official SDK links returned404 during parent research. The parent/validation agents completed canonical IDML snapshot/diff/mutation, package/DSL/storage surfaces and the declaration; see `idml-artifact.md` for their separately verified boundary.

## Editor Integration

The composed artifact input emits the native ReplaceDocument mutation, giving the core mutation owner responsibility for complete reversible diffs. Format decode occurs before that reducer. STEP/IDML now have typed export commands, exact registered tools, factory wire checks, host publication contracts, checkpoint discriminants4/5 and correct media/extension metadata. STEP selects the active page by default; IDML exports the document package. The tools use resumable format sessions with at most4096output bytes per chunk, per-row admission, UTF8 story fragmentation, canonical stored-ZIP descriptors and a separate quoted retirement stage before the existing fidelity stage. No partial download is published before a terminal commit. Localized English/German shell actions, argument folding, retained-job classification and ArtifactKindSpec format capability rows are mounted. STEP pageId reaches its typed command; IDML admits an empty document-export payload. The shell export descriptions state the supported scope.

Session limitations still requiring broader native verification: reverse-thread lookup scans source frames per text frame; IDML frame IDs/resource names must fit admission bounds; canonical decode/import currently performs bounded whole-file syntax admission before semantic progress. Generic native format jobs and closure tests are authored but not executed in this environment.

## Language-Neutral Fixtures And Oracles

The interchange fixture JSON covers PDF text/dimensions, STEP sheet geometry, IDML Unicode/package safety, cancellation, single SVG canvas interpretation, content/rotation/image scenes, PDF image alpha, long streamed stories, official NIST geometry, IDML RGBA paint, UTF8 typography with margins/insets/columns, embedded PNG/parent-page linkage, facing-spread coordinates and guide grouping, layer stack order, and shell-control format contracts. Canonical ZIP streaming has its own Unicode/empty-member fixture and Rust/TypeScript implementations. Canonical STEP projection has its own schema and Rust/TypeScript implementations. Thirdparty code is isolated in test-only oracle modules.

| Verification | Result |
| --- | --- |
| Persisted ZIP streaming TypeScript law, transpiled through the compiler API and executed with Node assertions; independent fflate0.8.3 decode | PASS,3entries,14chunks,432bytes,maxchunk81bytes; exact Unicode and empty-member content |
| Persisted official NIST TypeScript projection law; canonical firstparty Part21 parser; independent three r182 EllipseCurve/Matrix4 oracle | PASS,6curves,390points, all sampled coordinates identical (maximum error0); unit factor72/25.4 |
| Strict TypeScript compiler API closure for new stream/projection implementations, independent oracle wrappers and durable Bun test declarations | PASS,0diagnostics |
| Rust syntax via tree-sitter,43interchange/export/editor integration files in the final local audit; maximum path196characters | PASS,0syntax errors; this is not a typecheck |
| Six-format shell source contract audit: shell declarations, Migrated rows, argument bridge, text keyword expectations and every-command fixtures | PASS for PNG/SVG/PDF/package/STEP/IDML; static source audit, not native execution |
| Native interchange laws with lopdf/ruststep/independent ZIP/XML, official PDF and NIST fixtures, IDML RGBA/style/session/asset/master laws; typed editor format job law | Authored; native compile/execution unverified |
| Owned TypeScript IDML typography targeted strict check against actual241-file source graph; independent fast-xml-parser5.11.2 XMLBuilder/XMLParser | PASS,14semantic/refusal cases; targeted implementation/test/barrel diagnostics0, not whole-project/native/conformance verification |
| Owned TypeScript IDML override targeted strict check against actual241-file source graph; same independent XML oracle | PASS,12semantic/refusal cases; includes sparse changes, external deletion, translated/hidden masters, duplicate/missing refs, invalid affine/nonfinite transforms and unrepresentable copy paint edits; no native execution claim |
| Old repository CLI test invocation | Returned0 with no test/compiler output; explicitly not counted as a passing check |

A focused consuming-owner source review checked XML/ZIP/STEP public APIs, module privacy, all six export dispatch paths, exact next-release demands and close owner coverage, with no concrete new defect established; see `interoperability-protocol-review.md`. Standalone sessions require an unchanged source document between advances; the editor retains an immutable Arc. The4096byte cap bounds emitted chunks, not every temporary semantic operation.

Repository build frontend repair is owned outside this task. The framework aggregate mutation derive also remains a documented compile blocker (`protocol-alignment.md`). No cargo/bun/system-shell workaround was run. Generated Node test modules were removed after execution; durable test sources and this report remain.

## Official Fixture Provenance

- W3C SVG accepted corpus: [shapes-rect-01-t.svg](https://www.w3.org/Graphics/SVG/Test/20110816/svg/shapes-rect-01-t.svg), plus coords-trans-02-t.svg, painting-fill-01-t.svg and text-text-01-b.svg under the parent's conformance fixture folder. Original copyright notices remain. Exact URLs/hashes/status are in adjacent provenance files. The rectangle witness also exercises canonical SVG/independent XML rewriting; the parent added actual composed layout import laws for the three other witnesses.
- [NIST STEP File Analyzer And Viewer](https://www.nist.gov/services-resources/software/step-file-analyzer-and-viewer), [official PMI STEP archive](https://www.nist.gov/system/files/documents/noindex/2024/06/19/NIST-PMI-STEP-Files.zip), original AP203 geometry-only `nist_ftc_11_asme1_rb.stp`; retained fixture SHA256 `a5abfea460499dc31bc812aca68917948a117565fea4f5a080e36eb32f39650d`. This is a geometry projection witness, not AP214/complete CAD compliance.
- Official pinned [veraPDF corpus](https://github.com/veraPDF/veraPDF-corpus) witnesses implementation-limit.pdf, file-trailer.pdf and text-state-negative.pdf. Parent provenance records exact commit, original names, license, hashes and rule scope; actual independent pdfjs-dist5.7.284 oracle records one500x500page each and Hello World for the negative text-state witness. Native importer comparisons are authored.

## Files Owned By This Work

- Layout IO root, interchange schema/fixtures/tests and PDF/STEP/IDML/session mapping modules; IO Cargo dependencies; artifact composer capability rows.
- Editor composed-artifact import reducer; engine layout format facade; STEP/IDML command schemas/leaves; commands2/command aggregates; app command mounts; localized shell actions and stdio capability rows; argument bridge and command laws; reserved export request dispatch; export wire matching/publication proofs; export kind/checkpoint/session/retirement dispatch and format job tests.
- Canonical stdio IDML package codec/schema (parent extends declaration/event/storage); canonical ZIP stream schema/Rust/TypeScript/tests; canonical STEP orthographic projection schema/Rust/TypeScript/test-oracle/tests and mounts.

This work remains a supported-subset implementation. Full Layout feature completion and native verification are not established by this report.

## IDML Primary Specification Follow-Up

The Adobe-authored IDML File Format Specification mirror was retrieved successfully from [the retained PDF](https://raw.githubusercontent.com/jorisros/IDMLlib/master/docs/idml-specification.pdf),6044406bytes,SHA256 `c45a0d21257a83c0c7f66a13e78f38b07a38d96bc94eacac8b870f741da570bc`,520pages. The official Adobe Community attachment still returned404. PDF.js5.7.284 extracted the normative schemas/examples: pages88(masterspread shape),121-122(graphic Contents/GraphicBounds),127-130(text frame preferences/transparency),158-161(page master references/margins and standard ruler Guide/Label fields). This is a primary-document mirror used for implementation research, not an Adobe-produced test fixture or an Adobe desktop verification. No source PDF was written to the ticket.

Adobe primary API references also confirm [opacity and its separate scopes](https://developer.adobe.com/indesign/uxp/dom/api/b/blending-setting/), [paragraph typography](https://developer.adobe.com/indesign/uxp/dom/api/p/paragraph-style/), [character overrides](https://developer.adobe.com/indesign/uxp/dom/api/c/character-style/), [embedded link state](https://developer.adobe.com/indesign/uxp/dom/api/l/link-status/), and [image links](https://developer.adobe.com/indesign/uxp/dom/api/l/link/). XML field spellings were checked in the mirrored primary specification rather than inferred from UI labels.

The expanded native IDML laws remain authored/unrun. The earlier TypeScript ZIP and STEP oracle passes do not establish the new native typography/asset/master implementation's runtime correctness.

## IDML Page Override Follow-Up

The neutral override schema and twelve-case fixture were written before the implementation. Native direct and resumable laws independently rewrite ZIP/XML and cover rectangle, unthreaded text, raster-image and empty sparse overrides, rotation, unchanged source masters and a second unaffected page. Native refusal/deletion laws cover foreign OverrideList references and unsupported copy edits. The typed editor job law now keeps a parent and a real override instead of removing them from its input. These native laws remain authored/unrun.

Adobe primary specification p157 defines Page.OverrideList as space-separated Self references and p121 permits standard Graphic Properties/Label entries. The implementation writes real IDML body copies and opacity/paint/image content; labels retain only source identity and sparse property presence, never an opaque Layout document. Copies that change content beyond native bounds/visible/locked are explicitly refused rather than silently dropping those edits. Threaded master text overrides remain an explicit limitation.

Actual TypeScript verification is reproducible with retained input `override-ts-check.mjs`: `{strictTargetDiagnostics:0,actualSourceFiles:241,cases:12,generatedModules:5}`. Generated modules are removed by the harness. Typography verification from the validation agent is separately recorded in `interoperability-text-ts.md`: fourteen actual XML semantic/refusal cases with targeted diagnostics0.

## PDF UTF8 And STEP Content Follow-Up

Fixed a definite PDF importer bug: native TextStyleRun.end used Unicode scalar count although native offsets are UTF8 bytes. It now uses content.len(). A neutral Grüße witness and native PDF/lopdf to IDML/independent-XML law assert seven-byte range ends through the full chain; native execution remains unavailable.

The STEP frame-content mapping has now landed in both the direct exporter and resumable editor session. The canonical writer emits real entities one at a time, and the outer job chunks at4096bytes with exact queued-entity retirement. Native mixed-content laws assert direct/session byte equality, resolved parent bounds, hidden-layer omission, Unicode/image references and editable import after the independent Ruststep reader. These native laws remain authored/unrun. The background source cursor is now consumed. Complex filled contours, paragraph flow, pixel/alpha and general CAD semantics remain open.

## STEP Editable Content Verification

The owned canonical draughting reader uses an indexed Part21 graph and extracts RGB curve/fill appearance, character height, Unicode text, explicit extent and normalized planar placement axes. Layout import recovers true rectangle centre rotation instead of axis bounding boxes; text fragments get native styles and UTF8 byte ranges. Unsupported nonplanar/mirrored text placement, unsupported literal extent/alignment and multi-boundary fills produce issues. Generic face projection remains limited to supported planar rectangular faces. Original parent linkage, story threads and image pixels cannot be reconstructed from these draughting entities.

Actual targeted TypeScript checks, using real source graphs and independent Three physical geometry: `drafting-check.mjs` returns0diagnostics/197sources/6cases (5primitives including pen width and1invalid-RGB issue)/6generatedModules; `layout-step-geometry-check.mjs` returns0diagnostics/239sources/9cases/6generatedModules (3frame inverses,2annotation baselines,4rectangle refusals); `nist-projection-check.mjs` returns0diagnostics/444sources/6curves/390points/maximumError0/4generatedModules. Generated compiler modules self-clean. Twenty current interchange/draughting/projection Rust files parse without grammar errors; this is not compilation or native execution. Source review corrected concrete slice coercion, vector RGB scalar and import re-export visibility boundaries. Full goal completion remains unestablished.

## Canonical PDF CMap Follow-Up

The canonical importer already handles embedded Encoding streams and1–4byte physical codes. A concrete eager ToUnicode range-expansion cost was removed: range lookup/reverse encoding now stay compact and lazy while preserving Encoding widths. Actual PDF.js5.7.284 and owned TS checks pass2portable Unicode/mixed-width cases with targeted strict0diagnostics against216real sources; native FontCodec laws remain authored/unrun. Details and reproducible evidence: interoperability-pdf-cmap.md / unicode-cmap-check.mjs.

## Authored PDF Marked-Text Recovery

Layout recognizes BMC/BDC SemioLayoutText, promoting its invisible semantic text into positioned editable frames and recovering optional Unicode font family/positive-u32 weight/italic metadata. Ordinary invisible text retains prior handling. Native integration laws remain authored/unrun. Actual15TS policy/style/string cases plus2PDF.js documents pass targeted strict0diagnostics across197real sources. See interoperability-pdf-marker.md and semantic-pdf-check.mjs.

## STEP Borrowed Background Consumer

The earlier background-cursor-required limitation is now replaced by a real borrowed Drawing consumer in direct/editor STEP sessions. It preserves nested geometry, RGB/pen width, bounded stroke batches, curved samples, text and image placement/MIME references without copying a whole scene or child image bytes. Compound/over65point filled contours still refuse; pixels/alpha/font fidelity and general CAD semantics remain incomplete. Native lifecycle/refusal laws are authored/unrun; actual10Three physical checks pass targeted strict0diagnostics/198sources. See interoperability-step-background.md.

## Bounded XML Admission Prerequisite

Canonical XML now owns a borrowed no-DTD UTF8/XML1.0 syntax cursor, native+TS, with fixed numeric spans and per-turn work grants. Actual strict target check0/194real sources; Saxes agrees on32neutralcases+1000deterministicmutations. Grant/cancel/depth/attribute/long-name laws pass in TS; native grammar clean/tests authored, native execution unavailable. This is admission groundwork, not retained IDML publication. See interoperability-xml-validation.md and interoperability-retained-idml.md.

## Canonical IDML Borrowed Admission

Native and TypeScript IDML admission now use borrowed parts, bounded XML scanning, paged numeric name metadata and a bounded DOMVersion decoder. The TS event algebra has a production admission default; native cold package admission no longer constructs every XML DOM. Actual strict check: 0 target diagnostics over 201 real sources; fflate/Saxes: 12 package cases, 2,051 members, typed default mutation/inverse, grants and cancellation. Native grammar clean, execution unverified. Retained batch semantics/issuer remain unfinished; see interoperability-idml-admission.md.

## IDML Sparse Source Planning

Borrowed native+TS change planning now reuses admitted indexes and streams numeric sparse field coordinates. Actual eight-case typed diff application agrees with independent RFC6902/ZIP/XML at four work grants and four cancellation cuts; four raw-version replacement cases also pass. Native source/typed application and genuine source-lease laws are authored and grammar-clean, execution unverified. Full retained semantic owners/central commit/issuer are still unfinished. See interoperability-idml-changes.md.

## PDF Marked Outline Ownership And General Paths

Native PDF path construction now preserves Move/Line/Cubic(v/y)/Close/rect geometry through the real editable Drawing composition. Marked outlines are suppressed only after supported editable text promotion, preventing duplicate alpha paint; unsupported text transforms retain the outline fallback. Actual targeted TypeScript checks: eight path cases/five PDF.js physical pages have zero differing raster channels (strict0/194 real sources);25 marker/style/matrix/ownership cases and2 physical PDF.js extraction documents pass (strict0/197 sources). Native integration laws are grammar-clean, unrun. Per-page Drawing placement, mixed path/frame paint order, even-odd/stroke variants/clipping and retained import remain open; see interoperability-pdf-paths.md.

## PDF Per-Page Drawing Binding

Canonical PDF path import now uses genuine Page.backgroundDrawing children with Canvas placement. The global one-page seam/multi-page general-path refusal is removed. Native two-page distinct-geometry/margin law is authored and syntax-clean, unrun. Mixed native-frame/path paint order and unsupported fill-rule/stroke/clip semantics remain open.

## Genuine IDML Retained Diff Owner

IdmlDiff now has a real native retained central apply owner, with read-only live-post numeric preflight, selected genuine authored-field cloning, capability commit that moves untouched fields, explicit displaced retirement and semantic diagnostics. Actual overlay laws pass12cases×4grants/4cuts+2051members against independent ZIP/XML/RFC6902 (targeted strict0/203sources); native central ownership laws are authored/unrun. RetainedMutation/issuer still incomplete. See interoperability-idml-retained-diff.md.

## Genuine IDML Mutation And Issuer Source

Real native RetainedMutation now complements RetainedDiff, and the canonical IDML module exposes the concrete shared bounded publication/replay issuer and granted catalog construction. Generic runtime owner declarations are repaired; native central/generic issuer source custody laws are authored, syntax-clean and unrun. Actual eight mutation cases/four inverses and12batch cases/4grants/4cuts/2051members agree with independent ZIP/XML/RFC6902. The actual retained document creation caller must consume the new source constructor; native execution and full goal completion remain unverified. See interoperability-idml-retained-mutation.md.

## Native IDML Owner Registration


The explicit native codec/declaration caller now selects IDML’s actual shared retained semantic source with original full constructor grants and failure cleanup. Independent Rust-AST/Ajv checks passed 9 actual source files, 2 registration routes, 4 grant axes and 6 grant verdicts. Native laws are authored/unrun. Existing codec cold dispatch generic Clone/ColdMutation drift remains a distinct unresolved native compilation boundary. See interoperability-idml-registration.md.


## Layout Retained Sparse Edit Work


Active implementation now has a mounted borrowed numeric keyed-list planner native+TS. Actual strict0 diagnostics/194 sources,11 portable vectors/44 cancellation cuts and independent RFC6902 witnesses pass; native grammar5files clean, laws unrun. Typed retained LayoutDiff/Mutation and actual Layout issuer registration remain incomplete. See interoperability-layout-retained.md.


## Production Layout RetainedDiff Integration

The Layout diff crate now mounts retained::apply and its concrete RetainedDiff<LayoutSnapshot> owner. Sparse ordinary root fields/collections, repeated page/frame/layer/override/guide and story-run edits, optional data dictionaries, source-preserving staging, typed membership moves and displaced-field cleanup are wired. The page cache preparation module is provided by the graph owner through a borrowed final row view. Drawing text edits still refuse with an explicit funded diagnostic until their retained child edit port exists. Retained LayoutMutation decision/inverse owners and Layout issuer registration are still outstanding; no native execution or completion claim is made.

Actual ticket/layout-retained-central-check.mjs passed strict0/401 actual sources,3 ordered nested cases/12 cuts plus real canonical37 portable cases,45 goldens,500 properties and10 refusals through fast-json-patch. Native15 full-document witness laws and75 preflight cancellation cuts are authored/unrun. See interoperability-layout-retained.md for source and verification boundaries.

## Ordered Full-Document Retained Diff Witnesses

The shared Layout diff algebra corpus now includes3 additional full-document ordered nested witnesses, each checked by fast-json-patch and the actual canonical TypeScript applier/inverse/replay/compose. Current central portable evidence is40 portable cases,45 goldens,500 properties,10 refusals plus3 numeric provenance cases/12cuts, strict0 over401 actual sources. Native central laws consume18 full-document cases and90 preflight cancellation cuts, authored/unrun. A source audit correction normalizes child close Complete to parent Progress until all original owners settle and adds the direct first-party diagnostic package dependency.

## Genuine Ordinary Retained Mutation Path

LayoutMutation now implements real RetainedMutation<LayoutSnapshot> for23 ordinary command kinds, with original-source target resolution, bounded numeric/string admission, genuine authored payload preparation, typed sparse field moves and affected-field inverse preparation. All original mutation vectors retain the original issuer factory for retirement. Other36 variants currently issue a funded explicit retained semantic-kind refusal. The existing interactive LayoutPlayApp publication factory activates this partial owner; these are live capability gaps, while the separate document codec remains bare. Native generic issuer integration laws31cases×3bodygrants+7cuts are authored/unrun.

Actual layout-simple-check.mjs passes strict0/254 actual sources,35 cases covering all23 ordinary kinds,804 turns,2412 undergrant denials and124 cuts. Outputs and inverses are checked through actual canonical TypeScript semantic/applier code against fast-json-patch full-document expectations. Target lookup separately passes lodash7cases/1264turns/28cuts. All36 current retained diff/mutation Rust sources parse with0 grammar errors; native type/borrow/runtime remain unverified. Details and exact remaining variants are in interoperability-layout-retained.md.

### Retained Reorder Continuation

The retained owner now handles 26 of 59 command kinds, including genuine page/frame/layer reorders with bounded local and inherited lock reads. Actual paired suite passes 45 cases,857turns,2571undergrant probes and164cancellation cuts with strict0diagnostics across 254 source files and an independent RFC6902 oracle. Native40-case issuer laws are authored/unrun. Existing live editor selection is explicit; remaining33kinds remain incomplete. Details: interoperability-layout-retained.md.

### Retained Creation Continuation

The live retained mutation owner now handles 32 of 59 command kinds. New genuine creation owners cover links, character styles, paragraph styles, spreads and Unicode styled stories; inverses retain only the created identifier. The actual portable suite passes 63 cases with strict 0 diagnostics, 2919 undergrant probes and 236 cancellation cuts through an independent RFC6902 oracle. Native 59-case issuer laws are authored/unrun. The remaining 27 kinds are still active work; interoperability-layout-retained.md records exact scope.

### Retained Positional Output Continuation

The live retained owner handles 34 of 59 command kinds. Guide and story-run replacement move actual authored rows incrementally and keep skipped strings under original issuer custody. The actual portable output test validates 70 original row identities, one skipped owner and preservation of an unrelated live-post run through an independent RFC6902 oracle. The paired suite passes 76 cases with strict 0 diagnostics across 256 real sources. Native 72-case issuer laws are authored/unrun. The remaining 25 kinds remain active work. See interoperability-layout-retained.md for exact scope.

### Retained Deletion Continuation

The live retained owner handles 39 of 59 command kinds. Five deletion families now scan incoming references incrementally and prepare only the original removed row for restoration, including stored-order character typography recovery. Actual paired tests pass 86 cases with strict 0 diagnostics and an independent RFC6902 oracle; native 82-case issuer laws are authored/unrun. Remaining 20 kinds and full diagnostic fields remain required. See interoperability-layout-retained.md.

### Retained Layer Continuation

The live retained owner handles 40 of 59 command kinds, including local layer creation/removal and exact nondefault flag restoration using only original String preparations. Actual paired tests pass 95 cases with strict 0 diagnostics across 257 real sources and the independent RFC6902 oracle. Native 91-case issuer laws are authored/unrun. Remaining 19 kinds remain active work. See interoperability-layout-retained.md.

### Imported Layer And Guide Capacity

Removed command limits unsupported by the canonical model so deleting/restoring imported long-name layers, seventeenth layers and 129-guide pages remains admissible. Actual paired coverage remains 40 kinds, now 98 cases with strict 0 diagnostics. Seven positional producer cases preserve 135 original row identities through an independent RFC6902 oracle. Native 94-case laws remain authored/unrun. Retained nullable Drawing fields now follow Core's reference-only LayoutDrawingMember schema. Details: interoperability-layout-retained.md.

## Genuine Drawing Import Conversion — 2026-10-21

`interchange/🪆️drawing` now contains a direct first-party VectorScene→genuine Draw DrawingSnapshot conversion in Rust and TypeScript. It retains all six path command families, cubic/quadratic controls, SVG arc flags/rotation, nested planar quaternion transforms/reflections, nonzero winding, RGB alpha independently from object opacity, explicit canvas paint, and PNG assets through canonical PNG decoding. This cold syntax preparation is explicitly separate from retained publication and its original-funded member-reference duplication; the actual PDF/STEP import bundle callers still need that integration. The old inline Semio content route is being removed by the genuine child migration.

Language-neutral witnesses preceded implementation. Actual portable execution against canonical Draw types and affine projection passes three scenes, ten physical control/endpoints compared with Three0.182.0 quaternion matrices, eight source segments, twelve cancellation cuts and three explicit refusals. A two-pixel alpha PNG is independently encoded/decoded by Sharp0.34.5 and retains both intrinsic RGBA samples. Strict target diagnostics are zero against245 real source files. Native paired laws consume the same durable Three coordinates and Sharp samples; native source grammar passes, compilation/runtime remains unverified. Reproducible ticket input: `drawing-lift-check.mjs` (compiler output self-cleans).

Typography is an explicit open boundary: the neutral VectorScene Text carries neither font family nor font size, and genuine Draw permits only its four owned authored font families. Inventing a family/size would lose semantics, so the direct lift refuses neutral text rather than silently substituting it. PDF/STEP recovered authored text remains in native editable Layout stories; general SVG text needs its proper typed import seam. This is not full Layout import completion.

## Original Drawing Birth Preparation — 2026-10-21

`DrawingBirthPreparation<S>` now mounts a genuine native original-source route: source admission projects only LayoutDrawingBirth<S>.member; RetainedFieldPreparation duplicates that parent reference under its original clone grants; the field’s projection aliases close before take_authority extracts the exact original birth and typed S. Output transfers the prepared parent member together with that unchanged original birth. Cancellation closes original source authority, partial field, parent reference and typed payload through their actual owners. No member.clone, S clone, raw content hash identity, or manufactured source authority is used. Native admission/demand/advance/ready/take/close APIs forward original grants; undergrants preserve input custody.

Portable twin laws use declared logical metadata units and a required typed payload retirement port, not JavaScript physical heap claims. Actual execution passes three shared birth inputs (including long Unicode child/artifact identities),833 turns,1790 denied undergrant axes,18 cancellation cuts,3 original source substitutions and18 original birth retirements. Lodash independently compares complete prepared member records; Ajv admits the genuine reference shape. Strict target diagnostics are zero against233 real source files. The reproducible ticket input is `drawing-birth-check.mjs`; compiler output self-cleans. Native laws assert original payload/member String pointers, separate prepared parent allocations, and exact actual heap requested/released bytes using the first-party trace observer. Native implementation/laws parse cleanly; native compilation/runtime remains unverified.

This helper does not make the cold format composer a live funded importer. The cold Layout composer now explicitly refuses snapshot-only publication for external document/drawing dialects until original retained preparation is selected. Root owns the eventual retained importer host; the format issuer/mapping, typed birth bundle and same-job close must be connected before live import publication is claimed.

## Explicit Cold Source Plan — 2026-10-21

The public cold import_document now returns LayoutImportSourcePlan::Document{document,drawing_sources} or ::External{format,source,name}. External formats are exactly Svg/Dxf/Dwg; the original encoded source remains pending before target identity/parser/mapping admission. PDF/STEP keep generated neutral Drawing scenes beside an unpublished parent draft, associated with explicit Page source IDs. Their old raw DslValue/inline child constructors are removed. IDML/JSON carry mapped parent drafts. No parent/member reference placeholder is created, and dropping pending content is not a successful persistent import. Root owns removal of old cold root facade helpers and the composer’s explicit preparation-required boundary.

Portable paired plan execution passes seven cases, twelve cancellation cuts and one original pending scene identity; independent Lodash compares original encoded bytes. UTF16 BOM and opaque non-UTF8 bytes remain pending unchanged; actual parser encoding admission belongs to retained decoding. Strict target diagnostics are zero against258 actual source files. Native laws consume the same external-source fixture; PDF/STEP parser assertions now explicitly inspect the Document variant and separate pending scenes, including two-page PDF isolation. Native grammar passes; compilation/runtime remains unverified. Reproducible retained input: import-source-plan-check.mjs.

Three prior native STEP background export laws still require migration to genuine typed child read/vector providers. Validator confirms that the required concrete qualified provider/helper is not yet complete; Core owns that custody/planner integration. Those laws were not deleted, skipped or marked passing. Their remaining old inline setup is an explicit native source migration gap, not production compatibility support.

### Retained Text-Frame Continuation

The live retained owner now covers45 of59 command kinds, adding editable story/thread/inset changes with bounded original graph reads and exact affected-field inverse preparation. Actual portable execution passes156 cases,7554 undergrant probes and608 cancellation cuts with strict0 diagnostics across257 real sources. Six direct sparse-output cases verify24 original inset-word transfers,18 partial cuts and preservation of unrelated concurrent geometry through the independent RFC6902 oracle. Native152-case issuer laws are authored and affected Rust grammar passes; compilation/runtime remain unverified. Fourteen command kinds, relevant live-post graph preflight and actual Draw member/import job integration remain active work. See interoperability-layout-retained.md.

### Parent Removal And Source Admission Continuation

The selected owner now covers46 of59 command kinds, adding bounded parent-page deletion with sparse original identifier/ordinal transfer and genuine preparation of the original deleted ParentPage inverse payload. Actual portable execution passes161 cases,7815 undergrant probes and628 cancellation cuts with strict0 diagnostics across257 actual sources. Three direct parent-removal cases and9 partial cuts pass the independent RFC6902 oracle. Native157-case issuer and132 direct-decision laws are authored/unrun. The inverse command is CreateParentPage, whose retained execution remains pending among13 families; live parent deletion undo is therefore not yet complete.

Cold source-plan tests additionally compare exact0/4097/8193-byte progress checkpoints and all11 cut boundaries against shared fixtures; portable execution passes7 semantic cases plus3 chunk cases. Native paired tests call the actual root four-argument helper with its real control callback. Birth preparation now also quotes the owning source constructor copy measure explicitly and has native original-admission heap/undergrant/pointer laws authored. Nine touched Rust implementation/test sources parse cleanly; native execution remains unverified.

### Prepared Parent Ownership And Genuine Child Vocabulary

Parent cache preparation and an original-bound ParentCreationOutput helper are now implemented in native/portable code, with actual independent Lodash/RFC6902 output witnesses. The new creation output is unselected until the authoritative actual-post graph gate lands; see retained-parent-cache.md and retained-parent-creation.md. Core removed obsolete Layout-owned Drawing text edits in favor of genuine qualified Draw events. Current command vocabulary is58, with46 selected and12 still incomplete. The retained inline Drawing text refusal/literals were removed, without a compatibility lane.

Current source-change checkpoint resolved: Core removed the canonical drawingTexts type field. Fresh execution now passes the retained161-case/46-kind suite with strict257sources0diagnostics, plus the parent-creation3case/79turn/248denial/15cut original-row laws with strict248sources0diagnostics. The old inline field was not restored. Current total vocabulary is58.

The owned portable interchange barrel now directly exports externalDrawingSourcePlan/documentSourcePlan and LayoutImportSourcePlan/LayoutExternalDrawingFormat/PendingDrawingSource, alongside its namespace. These first-party cold source-plan values preserve original bytes or mapped drafts/scenes; the exports do not claim a retained import job or successful persistent publication. The artifact IO root facade remains Root-owned.

Fresh public source-plan verification includes the actual interchange barrel and passes strict264sources0diagnostics with7 semantic cases,12 cancellation cuts,3 chunk cases,11 exact checkpoint cuts and1 original pending-scene identity. Native tests remain unrun.
