# Interactive Paragraph Scheduling

The live Blueprint and Preview now opt into ArtifactEditor retained RenderWork. Their paragraph work retains the original document identity, uses the same LayoutParagraphState as PDF/SVG/PNG, and publishes actual Unicode/OpenType glyph outlines instead of unshaped host text. Each work advance emits at most one glyph header or outline segment. Glyph positions, style-run color, italic shear, frame rotation, and ordered frame/layer publication are retained. Drop preview and gumball chrome remain after content.

## Shared Owner

The shared owner is ✏️editor/⚙️engine/🎬️scene/📄️paragraph/🦀️.rs. Export paragraph-planning is a wrapper preserving glyph_cluster, glyph, glyph_style, mapped_segment and segment accessors used by the PDF encoder. Glyph style lends the authored family directly by retained paragraph/character index and preserves character-over-paragraph weight and italic precedence. Label metadata defaults to Layout Sans 400. External Drawing labels use begin_external_label/advance_external/external_cluster, retaining scalar source identity and borrowing the original text each advance rather than cloning it.

Text inset is interpreted as a relative content rectangle intersected with frame bounds; the default 80×40 inset therefore provides 80×40 content, rather than width 1 from subtracting inset size as a margin. Zero-area deferred interactive regions are skipped. Both deferred and prior display projection width/origin now follow this rectangle contract.

## Render Demand Contract

ArtifactEditor and ArtifactApp expose render_work_demand. EditorApp forwards the hook. PluginApp exposes render_demand; VcsArtifactApp forwards original untouched query demands and retains original retirement quotation for closing work. The existing original render call checks all independent currencies against the current and original total grants. It never increases source authority or changes cancellation identity. A known scene String capacity exceeding original total authority yields an explicit OwnershipLimit rather than looping forever. A spent current-turn allowance yields Pending. Existing original source/work/tree custody and controlled retirement remain in place.

The portable/native demand policy and schema are at framework plugin instance jobs render/🎟️demand. Layout quotes paged key/chunk backing, exact contiguous output reservation, and UTF-8-safe copies of at most 512 bytes; reservations and final copies debit the same StepContext receipt. The neutral sort owner was present but unmounted; its first-party value façade now exposes sort for the bounded scalar HeapSortCursor used by the live work. Native instance-jobs gains only a test-only serde_json dependency to decode its neutral fixtures.

## Zero-Touch Headless Resources

The stable export_font_count/source port now always exposes Layout Sans plus the 15 existing first-party built-ins. Their existing outline.ttf files are compiled directly into the Layout scene resource owner, independent of TextSystem/UI installation and runtime filesystem access. The roster stays stationary even if a UI text catalog is installed during a job. Canonical catalog metadata and a TypeScript reader interface live beside that owner. There are 16 sources totaling 1,110,320 bytes; all sizes and SHA256 values match the existing declarations. The strict missing-glyph policy remains.

## Executed Verification

All checks used Node/TypeScript/first-party source APIs, with durable test-only oracle interfaces. No raw build command bypass was used.

- Interactive publication: 4 neutral paragraph cases × 3 grants × 2 paint configurations = 24 exact pixel comparisons against independent OpenType.js paths rendered through Sharp SVG transforms. Maximum observed pixel error 0.
- 4 authored style cases, including label defaults, agree with independent lodash defaults.
- 5 portable path-segment transformations agree with independent Three.js matrix projection.
- 12 cancellation/stale-generation cases publish no tree; 8 changed-source/invalid-grant refusals pass.
- Render demand: 8 neutral cases and 1000 deterministic fast-check properties agree with independent Ajv2020 admission.
- Headless fonts: 16 real font admissions and SHA256 checks, 7 glyphs from Grüße 🌍 agree with independent OpenType mapping and metrics; globe selects catalog face 11; 4 strict resource/missing-glyph refusals pass.
- Strict TypeScript program: 405 actual source files, 0 diagnostics, including the durable test mounts and owned interfaces.
- Native tests are mounted for authored metadata borrowing, external source identity, zero-area text, transformed segments, actual outline publication/overlay order, changed document identity, demand cases, and compiled resource availability. These native tests have not run.

Durable test interfaces are under canvas/🧵️paragraph-work/🧪️tests, scene/🔤️text-fonts/🧪️tests, and framework render/🎟️demand/🧪️tests. Ticket wrappers paragraph-interactive-check.mjs and render-demand-check.mjs invoke the checked-in test interfaces; the first-party runtime does not depend on oracle libraries. Existing root test dependencies declare OpenType.js, Sharp, Three, lodash, Ajv, fast-check and TypeScript.

## Remaining Boundaries

Native compilation/test execution is unverified because the only cached repo frontend lacks the current runner and no raw cargo workaround was authorized. Fresh Rust grammar checks accept changed Layout work and new policy/tests; existing ArtifactEditor/ArtifactApp associated-type defaults produce the same 3 unsupported old tree-sitter grammar nodes. These are existing trait declarations, not a native compilation result.

This change does not yet prove the entire interactive render path physically bounded. Initial deferred display projection still resolves/clones frame graphs and background sources synchronously; transform preview still materializes its existing document clone. Font/Unicode/paragraph semantic advances are bounded, but their internal allocations are not yet quoted through the render demand hook. Small JSON fragment formatting likewise needs construction receipts. Final plugin scene_surface still synchronously split_lanes, encodes the spine, materializes all text carriers and builds the tree; final carrier publication therefore remains a concrete expensive boundary. The contiguous layer String itself has exact admission/copy receipts, but the subsequent scene/tree construction needs a genuine domain-neutral resumable owner.

The shared owner now integrates one reusable whole-story cursor with bounded fitting-region feedback, columns, cross-page threads and overset prefixes. Addressed frame requests cache one plan per thread and direct per-frame glyph intervals; see paragraph-flow-integration.md for the current evidence and remaining allocation/publication boundaries. Native execution remains unverified.

## Focused Files

Layout engine Cargo dependencies; scene module; shared paragraph Rust/TS/schema/fixtures/tests; export paragraph-planning wrapper; deferred DisplayParagraph/DisplayTextRun records and their constructor sites; display assembly and layout-engine views; canvas piece helper and paragraph-work Rust/TS/tests; LayoutPlayApp RenderWork hooks; command crate focused first-party dependencies; text-font catalog Rust/TS/schema/data/tests.

Framework: value package sort mount; ArtifactEditor, ArtifactApp, EditorApp render-demand declarations/forwarding; PluginApp render quote; VcsArtifactApp render quote/original admission; OriginalRenderWork query quotation; demand schema/Rust/TS/fixtures/oracle/native/TS tests; instance-jobs test dependency.
