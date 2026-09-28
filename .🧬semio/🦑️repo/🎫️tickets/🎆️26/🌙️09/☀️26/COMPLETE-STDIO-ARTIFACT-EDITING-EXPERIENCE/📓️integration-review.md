# Integration Review

## Shared Editing Boundary

The first shared Rust reducer now defines typed set, insert, remove, move, rename, and source-replacement events. It works on a detached value and decodes back to the artifact's own snapshot before producing a mutation. Invalid edits therefore need not modify the live document. Typed re-encoding comparison is included to prevent silently discarded edits.

The following checks were sent to the implementation owner before integration:

- Action argument controls and parsing must agree on types. A text argument copied as a string cannot change a boolean, number, or collection. Use an explicit JSON argument or actual typed controls, without guessing whether an ordinary string should be parsed.
- Typed re-encoding must compare object contents independently of map iteration order. Appending a lexically earlier map key is valid even when the snapshot stores a sorted map. Duplicate keys must still fail.
- Required field removal, unknown fields, fractional integer edits, pointer escaping, move-into-descendant, large numbers, and source decode failures need rejection and preservation fixtures.
- Expensive conversion and large collection rendering need bounded work and paging. Hidden collection entries must remain reachable.

## Native Mutation Publication

All new actions must join the editor's command id, retained tool roster, factory keys, owner proof, publication lane, and app declaration. A new native `SetSnapshot` variant also needs schema, text and binary codecs, diff, inverse, and persisted event replay coverage. Preview windows must not advertise write commands that return a no-op.

## Current Scope

These are implementation review findings, not passing acceptance results. The family agents are building complete JSON and PNG pilots before applying the integration to the remaining subset surfaces.

## Component Reachability

The committed default component exposes only nine text/data editors. The full fleet comments cite an unverified function-ceiling refusal, while the earlier play audit measured only the nine-editor component (387787 unoptimized functions, 50514 optimized); it did not reproduce the full release fleet failure. Added an Nx command to build the full optimized component and validate its extracted module independently with the native WebAssembly parser before changing shipped exposure. Library catalog tests alone cannot establish end-user availability.

## Direct Manipulation and Draft Conflicts

The framework table host only understood text, number, stepper and button cells, so declaring `set-cell` did not make CSV grid cells editable. Assigned a typed text-input cell protocol and both renderer implementations to the shared execution owner; the text/office owner will emit it from table windows. Draft Apply also needs a conflict rule because retaining a local draft across a changed persisted buffer must not silently overwrite a collaborator or a newly opened document. Assigned conflict handling to the same owner.

## Current Integration Milestone

All 88 artifact editor roots now declare the shared Details window. This is a source inventory result, not runtime acceptance: generated native mutation leaves are still being compiled, all-root non-no-op mutation cases are being authored, and the full browser component has not yet linked successfully. Advanced Source is being changed to complete typed JSON across adapters to avoid native-encoding normalization. Nested Details tree window paths and control budgets still require verification.

## Native Retained Acceptance Fixtures

The catalog fixture now covers all format families with exact PDF, GIF, IFC, and Semio subset overrides. Root resolved the pending spatial cases from their authored snapshot schema: SVG uses `/doc/root/name` (internally tagged XML, no `content` wrapper); STEP has a named `/header/fileName/name`; IFC4 uses a `kind:string,value` enum; IFC2x3 uses its complete optional EDM preamble. Semio collection-only subsets receive complete typed records instead of fabricated scalar fields. These source-checked fixtures still require the native catalog run to confirm exact application coordinates, codec round trips, retained publication, undo, and redo.

## Empty Values and Canonical Arguments

Host required-argument admission previously classified every null and empty string as missing. That prevented clearing a text field or assigning null even when the declared schema was `Any`. Added neutral fixture cases for null, empty strings, empty object keys, root JSON pointers, and JSON-encoded full-width integer controls. The checks now parse the **host-effective** argument object, so missing argument metadata cannot hide a value-encoding transport failure. `Any` now requires presence; string schemas explicitly admit an empty value with `min_length(0)`. Shared edit actions use the canonical `value` argument throughout and no longer retain unused `valueJson`, `source`, or `key` aliases. Strict duplicate-key checks also cover JSON-encoded field inputs. These later edits await the native rerun.

## Shipping Catalog Closure

A new neutral-fixture-backed gate failed on the actual default feature closure: only 7 of 36 format editor packages were included (nine subset editors). That expansion was subsequently reverted by the concurrent hub integration after its measured unoptimized component exceeded the browser function limit. The working tree currently ships the bounded nine-editor default and retains `full-app-catalog` for 88-editor native acceptance and an isolated optimized component diagnostic. The separate shipping gate is deliberately red. Complete shipping exposure remains required; library coverage does not substitute for it.

## 2026-09-28 — DOCX Retained Formatting Properties

DOCX snapshots retained XML formatting fragments at run, paragraph, table, row, and cell levels, while the corresponding sparse diffs omitted every fragment field. Detail edits could therefore disappear at publication. Each diff now carries the XML artifact’s existing recursive XmlChildrenDiff; between/apply/inverse/absorb and both text/binary codecs include all five fields. The XML implementation exposes its existing fragment algebra rather than duplicating it in DOCX.

A language-neutral fixture changes nested attributes and text, retains comments, adds processing instructions, and clears each fragment collection. Native laws compare direct/text/binary replay, inversion and composition against a serde_json pointer-edit oracle. Rust formatting succeeds; full native DOCX session30840 is pending. JSON/TypeScript mirrors are owned by the Text/Office execution lane. The native fragment API still uses the XML artifact’s existing whole-fragment algebra; migrating preparation to immutable retained cursors remains a separate unfinished capacity gate.

The retained-copy foundation audit identified enforceable immutable-source ownership, cancellation admission, repeated map growth and progress-accounting blockers. Sol execution is repairing them before artifact migration, as detailed in the foundation audit report.

## XML Attribute Identity Order and Explicit Clears

The reused XML algebra ignored attribute reordering and described attribute-addition index composition as approximate. XML attribute diffs now retain a complete final identity order for every nonempty edit, validate that it is a unique permutation, and preserve it through between/apply/inverse/absorb and text/binary codecs. The SetAttribute mutation supplies the final order. A neutral ordered-array fixture and native law cover every contiguous composition of reorder, add, remove, value change and clear states, including invalid order refusal and a serde_json ordered-array oracle. JSON/TypeScript parsers preserve optional sparse lists and reject repeated order identities. DOCX embedded grammar uses the same new order section.

The separate null-clear repair adds explicit double-option decoding to XML declaration and doctype fields, with a four-case neutral fixture and replay/inverse law. The transient unresolved `value::` helper references were replaced by the XML crate’s existing `dsl::` aliases before the current checkpoint.

Scoped diff checks and Rust formatting pass. Native verification is still pending because current DOCX/XML builds failed in an in-flight retained-clone dependency before reaching these artifact tests.

The published DOCX and XLSX operation/diff grammars now include the archive-comment slot already required by the native codecs. XML diff grammar includes prolog edits and the complete attribute-order section. XML child addition validation also rejects the off-by-one final-length position instead of allowing the apply helper to clamp it silently. These grammar/validation changes await the native family suites.

The independent audit confirmed that the new complete XML attribute order made XmlAttrAdded.index redundant. It has been removed from Rust, TypeScript, JSON Schema and both codecs; `order` is now the only attribute-placement authority. Child additions still carry indices because their separate collection algebra is positional. Tests include clearing and re-adding the same attribute names.

## DOCX Save Targets and Last-Style Deletion

Export now follows relationship-selected main/styles paths and writes an existing styles part after all styles are cleared. The new neutral/native save-reopen law includes an independent ZIP reader; it is authored but unrun. The accompanying `🔍️research/🧬docx-save-part-identity-and-body-fidelity-2026-09-28.md` records remaining nested/interleaved WordprocessingML content loss that requires canonical XML ownership.

## Generic Preparation Capacity Review

The new lifecycle fixture covers scalar Store publication correctly in source, but does not exercise ordinary multi-kilobyte artifact fields. Root identified the full String/Vec allocation versus per-turn byte-grant mismatch and assigned a neutral larger-field law to the core owner. See `🔍️research/🧬generic-preparation-capacity-admission-review-2026-09-28.md`; native behavior is not yet established.

## Independent XML Boundary Audit Findings

Terra found stale copied XML public types in PowerPoint snapshot/diff JSON, TypeScript, Protocol Buffers, and GraphQL. Rust already stores the canonical XML document there, so those public copies must retain epilog and doctype position too. The audit also found unchecked Rust snapshot text/binary deserialization and builder publication paths that accept invalid authored boundaries rejected by the public TypeScript decoder; an editor subsequently calls the infallible writer. These are assigned for repair at the XML worker’s next turn. Root’s independent native quick-xml test addition closes a separate executed-oracle wiring gap once its native target runs.

## Canonical Office Public Contract Gate

Root owns the two shared Office schema/type contract test files while the DOCX worker owns artifact fixtures and native tests. The shared Ajv registry now resolves the authoritative XML snapshot schema. Removed semantic DOCX diff assertions are replaced by canonical XML-part diff replay, all four optional declaration/doctype states, and the five WordprocessingML property-node scopes. A negative schema assertion rejects the removed semantic shadow root. The public facet type proof carries a canonical XML part with prolog/doctype position/epilog through all three DOCX subset types.

The existing complete `@semio-tech/stdio-js:test` target is running as session24070/log `office-public-facets-typescript-current-6.log`; no result claimed yet. The artifact fixture migration may still cause an expected integration failure. XML execution has resumed Terra’s scoped findings and owns all PPTX JSON/TS/Proto/GraphQL facets plus XML/SVG ingress validation and grammar repair; it will leave these two global test files to root.

## DOCX Canonical Retained-Route Audit Checkpoint

The canonical production build does not compile the old DOCX `📬️preparation` files: their module declaration and mounted `preparation_route` were removed. The stale `.document` accesses there therefore are dead source, not a current compile failure. The important remaining audit is behavior: retained command text collection is distinct from Store mutation preparation, and emitting `SetRunText` does not prove bounded native snapshot publication.

Root also verified that `addressed_run_text` presently calls `snapshot.project_document()` before selecting a run. It materializes the semantic document rather than borrowing only the target text. Both execution and the independent auditor were asked to track this explicitly, restore direct/bounded XML addressing, and verify the actual mounted native preparation path before accepting broad DOCX editing. The production model/API freeze remains valid; these behavior repairs are still required.

## Sparse XML Diff and Spreadsheet Scope Review

Office schema integration now exercises the new canonical DOCX fixture rather than a semantic shadow snapshot. The shared XML guard needed sparse-preserving optional triples; the regression was reproduced in TypeScript12 and fixed in TypeScript13. The new source review `🔍️research/🧬xlsx-canonical-save-and-grid-gap-2026-09-28.md` records the next canonical XLSX and natural worksheet-grid requirements. The earlier unchanged-draft repair does not resolve regenerated-XML loss or blank-cell editing.

## Current Integration Repairs and Follow-Ups

Full-catalog18 exposed a wasm32-only DOCX safe-integer literal overflow; root widened the constant and both comparisons. Root also repaired the top-level DOCX run projection so tables no longer abort the whole primary window, with five neutral nested-table text/path expectations and three EN/DE subset window laws (native execution pending). The shared DocumentWindowKit TypeScript draft now preserves explicit canonical arguments and owns nested values; a fail-first fixture law then passed.

The next DOCX formatting/projection cut is explicitly recorded in `🔍️research/🧬docx-namespace-and-formatting-follow-up-2026-09-28.md`: prefix-independent semantic reads, correct attribute namespace handling, explicit false/inherited formatting distinctions, and visible natural controls remain required. Core paged primitives are undergoing an independent Terra audit before mounting. XLSX canonical migration and resumable media host integration remain active execution lanes.
