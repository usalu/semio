# Prepared Layout Frame Addresses and Shared Metadata

## Result

Export frame planning now retains scalar FramePlanAddress records instead of cloned ResolvedFrame values. Each address carries physical page, parent template, original frame, override and image-link ordinals. Borrowed frame, scalar bounds and visibility queries preserve the original snapshot. Stable parent-before-local, authored layer and within-layer order agrees with the independent oracle. Normal image references resolve once through the prepared metadata owner.

The shared Layout MetadataIndex reads borrowed original identifiers one Unicode scalar per turn. It handles exact hash collisions through an incremental comparison chain, isolates domains and parent/page scopes, refuses duplicate identifiers and changed source/query identity, and retains no source strings. The prior FlowProjection now uses this same owner; its private duplicate hashing/index implementation was removed.

FlowProjection exposes next_advance_demand with copy, capacity, release and depth currencies. Its caller forwards the original grant, and each turn reports actual copied bytes and original allocated capacity. Addressed paragraph preparation uses this full demand rather than manufacturing a copy/depth grant from capacity. Controlled retirement delegates original paged owners, including manual copy and release quotes.

## Original Structural Reservation

Test-first owner laws reproduced the absent NumericIndex funded reservation API. Caller laws independently reproduced capacity admissions quoted with depth 1. NumericIndex now delegates next_reserve_demand and reserve_one_funded to its original PagedList owner. This preserves the native owner's structural copy quote (currently zero for ownership moves), capacity quote, traversal depth and actual receipt. TypeScript exposes the same owned scalar contract over its accounted fixed-page model.

MetadataIndex, FramePlanState and FlowProjection forward the incoming retained grant directly to that owner and account returned structural copy/capacity receipts. A below-depth, below-capacity or below-copy grant does not admit the page. The new API does not turn a demand into authority. Native NumericIndex, the neutral reservation schema and fixture, portable laws, native laws and permanent test entrypoint were updated together.

## Borrowed Image Fallback Labels

ExportTextState.begin_link_label borrows the original ImageLink.artifact_kind with an original-document/source guard. It uses a finite padded content rectangle, retained paragraph fonts, exact glyph clusters, UTF-8 source ranges and the image frame's rotation/center. Newline and height overflow finish only the fitting prefix. Zero-area input publishes no glyphs. No whole source String clone is required.

## Executed Evidence

Permanent test-only interfaces are mounted beside the numeric reservation, metadata, frame-planning and flow-projection owners. The retained ticket input frame-preparation-check.mjs loads those actual source interfaces; no source files are copied into the ticket. Independent lodash stable ordering/find, fast-json-patch RFC6902 observations and Ajv2020 admission remain behind those interfaces.

- Numeric reservation: three runs, 273 keys each, 189 independently under-funded attempts, 63 backing admissions and 882 close steps. The fixture crosses the 16-row and 256-row boundaries and observes structural depths 3, 4 and 5. The test-first missing-method and caller missing-depth failures were observed before implementation.
- Frame preparation: 45 runs, 1746 returned addresses, 17336 blocked copy/capacity/depth/release attempts, 28 cancellation cuts and 45 changed-source refusals. Includes selected-page/all-page modes, reused parents and per-instance overrides, hidden/reordered/unassigned layers, local/parent image link ordinals, 257 frames, 513-scalar Unicode identifiers and a borrowed 32769-byte wrap-mode string.
- Metadata: three runs, 27 queries, 60 blocked grants, 33 close units and two source/query refusals. The distinct identifiers k1jg4pp9 and k1v560fk have the same pinned scoped hash 3053141472 and resolve independently.
- Migrated projection: five neutral cases across three grants, four cancellation cuts and seven semantic refusals. The wide fixture prepares 3280 frames and 41 heads, then flows each region exactly once. Portable positive-case turns: 112846; wide turns: 229681; close units: 14825. Per-turn actual capacity equals its preceding owner quote and actual copy does not exceed its quote. Explicit insufficient structural depth leaves the owner pending without a receipt.
- Shared paragraph flow regression: six cases/18 runs, six cancellations, six source refusals; addressed replay makes 60 requests from exactly 18 story analyses and refuses 18 invalid addresses. Borrowed font ownership passes twelve completion/cancellation runs.
- Borrowed link labels: five neutral cases across three grants, 15 glyph/metric runs, five cancellations and 15 changed-source refusals, checked against the compiled headless font catalog and independent OpenType.
- Interactive publication: 24 exact OpenType/Sharp pixel comparisons (maximum pixel error 0), four style cases, five Three transform cases, twelve cancellation/stale-generation paths and eight refusals. Original render-demand laws: eight cases and 1000 deterministic Ajv2020 properties. Headless catalog: 16 sources, seven Unicode glyph observations and four resource refusals.
- Latest strict graph: 455 total TypeScript sources, including 117 implementation/test sources, zero diagnostics. Thirteen focused Rust source/test files parse without grammar errors. New source paths stay below 256 characters.

A native API inspection also corrected the shared frame-glyph capacity quote to public PagedList.next_reserve_depth_demand; the private next_page_reserve_depth_demand is no longer called across crates.

Native Rust compilation and native test execution remain unverified. The available cached repo frontend has no current runner; the live repo path also failed while persisting its oversized search index. No raw cargo/build-tool bypass was used. Grammar and TypeScript execution do not prove Rust type correctness.

## Owned Sources

- scene/metadata-index: schema, Rust/TypeScript owner, neutral fixture and independent portable/native laws.
- export/frame-planning: schema, Rust/TypeScript address planner, fixtures and independent portable/native laws.
- scene/flow/projection: shared metadata migration, full grant/demand/receipt, controlled retirement and laws.
- framework/value/ordered/numeric/scratch/reservation: owned reservation contract, neutral fixture and oracle/native laws; numeric index and scalar-page reservation surfaces; permanent numeric test entrypoint.
- Shared paragraph owner/export wrapper: borrowed link source, finite label region, source guards and neutral tests.
- Export contract/validate-plan/job state: address variant and delegated next_frame_plan, preparation state and original-context preparation seam. Root owns production PDF consumption, enclosing demand wiring and paired retirement.

## Remaining Boundaries

This proves the focused structural reservation contract, not the entire physical scheduling chain. NumericIndex lookup, AVL insertion/repair and its ordinary cold APIs still execute a bounded-by-tree-height walk within one turn. The TypeScript page model reports owned scalar capacity independent of JavaScript object headers or garbage-collection timing.

The detailed next-owner source audit is [paragraph-admission-audit.md](paragraph-admission-audit.md). Shared paragraph begin_story_box still eagerly reserves four style vectors from the complete style-run count; font admission, Unicode/paragraph scratch owners and fixed 64-substep wrappers are not all debited through original retained authority. Style ID resolution still scans per run. Initial deferred display graph/background resolution, transform-preview snapshot cloning, small fragment formatting and final synchronous scene_surface carrier/tree assembly remain expensive boundaries. These need their own admitted resumable owners rather than invented demand-derived grants.

Finite paragraph regions constrain line placement but do not yet add a geometric glyph-ink clipping path. Root owns current PDF/image/Unicode and Page-child/Canvas integration; this report does not claim whole Layout feature completeness.
