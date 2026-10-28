# Shared Native Population Owner Checkpoint

This initial population10 checkpoint is historical and has independent source-review closure. The current actual Direct consumer migration is recorded in native-alignment-direct-clause-implementation.md; it adds request-only unavailable access and a real local clause, while Terminal extraction remains pending. No native execution or full-view/publication capability follows from either checkpoint.

At this initial checkpoint, the adjacent private MVD population owner prepared one original source container and one stable ID index, then lent both through a one-turn borrowed callback. Direct and Terminal still prepared their own standalone populations at that point.

## Source and ownership

`concepts/population.rs::IfcMvdPopulationWork<R>` receives the exact already-admitted sealed request donor after its enclosing endpoint validates the request. This internal precondition is explicit: the population owner does not validate arbitrary endpoint request semantics or claim to authenticate how an unrelated caller originally constructed a donor. Initial receiving retains one original-paid snapshot Arc in its born-owner slot before posting and moves the admitted request once. Snapshot sealing moves that same Arc; neither request nor source is cloned for each clause. The retained source prevents pointer reuse or exclusive source mutation between original turns.

Original source pointer, request pointer/type, epoch and actual OriginalSourceBinding remain authoritative. Repeated receive, changed source/epoch/caller and post-close access are refused before effects. Source epoch remains continuation metadata, separate from canonical document revision. The one source index lives in an original-paid stable Box adopted before receipt reconciliation; moving the enclosing Work preserves its identity.

The private `with_view<T: Copy>` callback is higher-ranked over the borrowed view lifetime. It lends the actual source, original request and prepared index plus the same original StepContext for one turn. Clause code retains only copyable source/request/type/index/row-buffer/original-caller pins. The view constructor checks actual `ShapeCursor.complete()/valid()` and completed index identity for the retained source/epoch; a phase flag alone cannot grant access. A view cannot be returned or retained as an async owner. The callback's clause still funds its own bounded metadata/query work against that original context.

Preparation is container/index preparation, not IFC schema admission. Unknown declarations and duplicate IDs remain observable; missing, exact original ordinal and ambiguity stay distinct. An invalid ADD2 header, zero source ID or zero reference makes the container unavailable. The source scan and index occur once inside this owner. No second catalog, whole graph copy, request alias or complete standalone clause Work is constructed.

## Funding and close

Receive, source construction, each container step, stable index birth and index advancement preflight independent original item/copy/capacity/release/depth/fuel demands. Born sources and index candidates are retained before a fallible receipt post. Accepted child physical receipts are posted once; failures retain their original primary and any posting refusal. Index work uses the existing original-funded source index implementation.

Explicit close first resets borrowed shape/view metadata under its own original quote, then retires index, sealed source, any born source alias and original request. Empty-owner phase changes are separate paid metadata turns. A terminal retirement cursor remains retained until its real slot is empty; child Complete alone does not finish the owner. Cancellation and expired ordinary deadlines use existing original-retirement scope and `retirement_should_yield`, preserving paid close. The small local `PopulationClose` implements actual framework `RetireOwned`/`RetirementCursor` around an active RetainedCloneClose so derived fallback retirement can drain an interrupted close before later source/request fields. RetainedCloneClose itself is not presumed to implement RetireOwned.

## Schema-first and execution evidence

Eight closed neutral cases live beside the multi-implementation concepts source: population-cases.json, population.schema.json and population.feature. The contract covers empty/unsorted/exact-max-u64/duplicate/unknown populations and unavailable header/zero-ID/zero-reference cases. The TypeScript suite validates the schema and consumes the actual existing shared `ifcConceptIndexSteps` implementation for every prepared case, preserving source order and source data.

- Focused actual TypeScript consumer/schema: **5 passed,25 assertions,0 failures**; `🗑️generated/native-population-contract.txt`.
- Narrow strict TypeScript including the actual shared index graph: **exit0**; `🗑️generated/native-population-types.txt`.
- Independent repository-owned PopulationLookupOracle uses **SQLite3.50.4** through Python's SQLite interface: **5/5 prepared-case lookup outputs match**, with exact decimal TEXT identity storage. The three unavailable-container cases are explicitly unobserved by this SQL oracle. `native-population-oracle.py`; `🗑️generated/native-population/oracle.json`. This is index reference evidence, not native output or IFC schema validity.
- Six native law functions are mounted by population.rs in the actual native MVD module. They consume all eight neutral cases; prove two clause probes share one stable index after moving their parent; refuse another prepared index despite identical source/request; check unchanged receipts/heap/state on denied turns and substitutions; assert accepted actual heap allocation/release versus original receipts; cancel every prefix; close under expired deadlines; and reject forged ready phases without actual completed shape/index. These laws are authored, not executed.
- Rustfmt syntax parsing of six changed native modules: **exit0**, emitted source suppressed; `🗑️generated/native-population-syntax.json`.
- Authentic `cargo test --lib native_population` selector: **exit101 at enforcing compiler -vV before rustc**; `🗑️generated/native-population-cargo.txt`. No native typecheck, execution, heap, physical receipt or output proof follows.

The new TypeScript suite awaits root's single registration change after this actual-consumer checkpoint; no shared package-script edit was made here. Root's preceding full86 run was1820 passed/80073 assertions/12existing gated skips/0failures and predates registration of this new suite. It does not execute these native laws.

## Post-audit source-only correction

API inspection after the historical Terminal11 audit found that Direct and Terminal's constructor-demand branches treated framework RetainedCloneBirthDemand as a full RetirementDemand. The actual framework birth type exposes only capacity_bytes/depth; those branches assigned nonexistent copy_bytes and mixed it with full demands. Four branches now explicitly construct RetirementDemand from actual birth capacity/depth and their existing alias-copy quote. No numeric/domain/report behavior changes. The new population source uses the same explicit conversion.

This is a source-only correction discovered after the Terminal11 source review; the old eleven-hash audit remains historical and cannot establish type correctness. Current source hashes include those affected demand sites, the narrow ShapeCursor.complete accessor, native root mount and adjacent owner/tests/contracts. No compiler/enforcement/build files changed. Native type/runtime/heap/receipt/mount/capability and semantic publication flags remain false.

## Next bounded integration

Extract standalone Direct/Terminal local clause continuations that borrow this same private prepared view and retain its pin across original turns. Reuse existing numeric/query/ordered-list semantics, request validation and partial report contracts. A joint scheduler must own one population plus independent clause state, without dual whole Works or a duplicated shape scan. Its output remains separate partial clauses until a released correspondence policy justifies a closed full-view request. This checkpoint does not claim that migration or full view conformance.
