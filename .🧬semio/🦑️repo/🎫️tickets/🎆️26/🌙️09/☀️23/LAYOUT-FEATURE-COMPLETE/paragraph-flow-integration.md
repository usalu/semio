# Shared Paragraph Flow Integration

## Result

The shared Layout paragraph owner now uses one original whole-story FontParagraphCursor with actual region feedback. TextFlowCursor supplies relative content rectangles, columns, page gutters and cross-page thread continuation. Each framework pause returns the original UTF-8 source end, including hard separators, so the next region neither repeats text nor loses CRLF or multibyte characters. Region height limits placement; exhausted geometry finishes only the planned prefix and retains an explicit overset result.

The prepared FlowProjection preserves source-qualified page, parent-instance and original frame indices. The shared owner retains that projection through its controlled retirement implementation. It prepares one scalar glyph-range slot per projected frame using the original retained grant, quotes that slot's backing/copy/depth before mutation, and reports the exact receipt. Projection capacity demand is the original owner's next-step quote, never new authority.

An addressed request resolves its already-computed head directly. The first requested frame, including a tail painted before its head, creates one story plan for that head. As glyphs publish, the owner records each frame's contiguous glyph interval. Later requests use the cached interval and visit only that frame's glyphs. Empty frames replay an empty interval. This avoids both repeated tail analysis and full-story replay scans.

Live LayoutRenderWork prepares the flow owner through its actual StepContext before shaping deferred text. ExportTextState exposes the same preparation/demand seam and addressed begin API; the PDF text-frame call now supplies page, parent and original frame address. The root agent owns the enclosing context-aware export Plan wiring.

## Source Addresses

ResolvedFrame retains parent membership separately from inherited styling: an overridden parent frame remains a parent source even when inherited is false. Both local and parent resolved records retain their original frame index through layer sorting. Deferred DisplayParagraph carries that source address. The three neutral layer-order cases now assert the address as well as paint order, visibility and override geometry; independent lodash sorting agrees.

## Executed Evidence

The permanent test-only oracle interface is scene/paragraph/tests/flow/oracle.mjs, invoked by ticket input paragraph-flow-check.mjs. It loads existing compiled-font source assets and first-party Unicode/UAX14 data with no runtime filesystem dependency in the feature itself. External libraries remain test-only behind the owned interface.

- Six neutral flow cases across grants 1, 7 and 4096: 18 runs agree with independent OpenType glyph selection and metrics. Cases cover three linked frames across physical pages, different frame widths, original UTF-8 hard-break consumption, a too-short region consuming nothing, an explicit overset prefix, and three columns within one rotated frame.
- Six cancellation paths and six mismatched original UTF-8 sources pass.
- Six neutral addressed cases across those grants: 60 requests create exactly 18 analyses, one per independent thread/run. Tail-before-head paint order, repeats, empty regions and per-frame range replay pass. Cached requests take at most their own glyph count plus two scalar turns. Eighteen invalid-address refusals pass; cached range records pass independent Ajv2020 schema admission.
- FontOutlineRunJob previously drained the caller's borrowed font array during retirement. The test-first frozen-array case reproduced the defect; the child now detaches its borrow. Twelve completed/cancelled cases across grants 1, 7 and 4096 preserve the same source array, face and bytes. Independent OpenType glyph IDs agree.
- Five borrowed link-label cases across three grants pass 15 OpenType glyph/metric runs, five cancellation paths and 15 changed-source refusals. Labels preserve the original link source and rotated finite region without cloning the whole artifact-kind String.
- Existing interactive publication remains 24 exact OpenType/Sharp pixel comparisons with maximum pixel error 0, four authored-style cases, five Three matrix transforms, twelve cancellation/stale-generation paths and eight refusals.
- Existing original render-demand laws remain eight portable cases plus 1000 deterministic properties against Ajv2020.
- Headless catalog remains 16 source admissions/hash checks, seven Grüße 🌍 glyph observations and four strict resource/missing-glyph refusals.
- Fresh strict TypeScript graph: 455 total sources, including 117 implementation/test sources, zero diagnostics.
- Latest focused Rust grammar: 13 current source/test files, zero syntax errors.

Native tests are permanently mounted for the same flow corpus and addressed requests, including independent ttf-parser glyph metrics, exact source consumption, original document refusal, per-thread plan reuse and bounded replay. They have not executed. Current repo frontend still cannot provide native compilation/test execution; no raw build-tool bypass was used.

## Files

Shared paragraph Rust/TypeScript owner; flow fixture/address fixture, schema, native tests, portable tests and oracle interface; export paragraph wrapper; live render-work owner/tests; computed resolved-frame address schema and native model; deferred display record/assembly; export frame-plan address retention and text begin call; layer-order fixture/native/portable tests; font-run borrowed-source retirement and durable native/portable ownership tests; engine test-only ttf-parser declaration. Ticket inputs/reports are retained.

## Remaining Physical Boundaries

The reusable framework paragraph cursor is semantically bounded and analyzes the original story once; it still retains admitted whole-story analysis, not just the painted prefix. Its internal font/Unicode/paragraph reserve_slot allocations and fixed 64-substep wrapper are not yet fully quoted/debited through original retained authority. Small fragment formatting, initial deferred display graph/background projection, transform-preview document cloning and the final synchronous scene_surface carrier/tree construction remain explicit expensive boundaries. This integration does not claim those operations are physically controlled or that native compilation passed.

The export address planner and shared borrowed metadata owner now replace synchronous parent/layer/override/image-link lookup and cloned ResolvedFrame in next_frame_plan. Original NumericIndex structural reservation quotes and grants are forwarded through metadata, frame preparation and flow projection. Borrowed image fallback labels also use the finite paragraph owner and preserve source clusters. See [frame-preparation.md](frame-preparation.md) for exact APIs, independent observations, grants, native limitation and remaining physical boundaries.
