# Native Owned Receiver Contract Audit — 2026-10-28

Read-only support for actual native proctor compiler diagnostics reported by backend17816. No source edits, builds, tests or probes. backend_live owns receiver generic/Sized/lifetime repairs; frontend_live owns key/map/record/dynamic-child iteration. The reported228errors are compiler observations of those owners, not228independent root causes established here.

All paths below are beneath 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/ unless stated otherwise.

## Intended Canonical End State

🌳️value/🦀️.rs:6–21 defines object-safe NativeValueSource:Sync+static. as_source returns the source facet, and variant payload, record entry/child and sequence child return borrowed dyn NativeValueSource. Their values may have different concrete Rust types from the enclosing original source. The outer receiver can keep a concrete or unsized I, while child receiving operates against that dynamic source facet. This is an explicit original-child projection design, not a request to materialize an intermediate DslValue or clone a wire tree.

The common contracts 🦀️.rs:31–56 admit NativeReceive<I:?Sized> and NativeReceiver<I:?Sized,T>; receivers remain Send+static and retain the original input until funded transfer/closure. The static receiver type does not make a borrowed projected input arbitrarily static: original RetainedCloneRef/binding ownership must continue to carry the actual lease. Blanket Sized constraints, owned child conversions or lifetime erasure would change the design rather than finish its migration.

🧩️record/🦀️.rs:6 defines NativeRecordKey as Name(borrowedstr) or Ordinal(u16). NativeRecordField at8–9 declares both canonical and text identities and matches either. The record neutral schema explicitly gives the same expected object from canonical string keys and text ordinal keys. Therefore the enum is intentional. Dynamic string-key maps can use NativeValueSource.record_name at tree17, which accepts Name and refuses Ordinal; fixed authored records should compare their declared field identities. Do not reinterpret ordinal as a string or globally restore the old str-only key shape.

## Root Categories Seen In Current Source

At the initial read, tree135 declared NativeSequenceReceiver with3generic parameters, Sized I and T:NativeReceive<I>, while Drop148 supplied4parameters including S:NativeSequenceInput<I>, and Vec/array149–150 already required outer I:?Sized and T:NativeReceive<dynNativeValueSource>. This directly establishes an incomplete generic migration in source; one erroneous declaration can cascade into many compiler errors. Backend owns aligning it consistently with the child projection design.

NativeSequenceInput at130 accepts I:?Sized but its child method returns dynNativeValueSource; all remaining callers that assume &I for child are similarly inconsistent. NativeRecordKey call sites that invoke string methods on the enum represent another category, to be corrected according to map versus declared-record semantics above. The review does not claim every228diagnostic is accounted for without a corrected compiler run.

## Existing Laws And Independent Oracles

- Tree 🧬️schema/🔣️.json requires originalChildProjection=true, coldConstructionRequired=false and five funding denial axes: items/copy/capacity/release/depth. Its fixture retains malformed-value refusal cases, including duplicate/unknown keys and wrong scalar types.
- Tree 📖️source/🧪️tests/🦀️.rs:4–5 defines a genuine heterogeneous Original with String, Number and Vec fields. It returns borrowed dynamic children and verifies exact pointer identity for text and nested sequence child. This is direct evidence against proxy construction. Its oracle uses Ajv2020 plus Lodash get/isEqual and JS identity over the same neutral fixture.
- Record 🧪️tests/🦀️.rs:4–6 resolves declared field identities, checks original text pointers and zero heap activity against the neutral record fixture. Record schema explicitly pairs canonical names with declared text ordinals.
- Tuple 🌳️value/👥️tuple/🧪️tests/🦀️.rs:14–24 drives real NativeReceive through funded advance/take/close across prefixes, denies each currency without heap/progress, rejects foreign originals and matches allocation/release receipts. Its TS oracle validates heterogeneous tuple shape with Ajv and independent UTF8 Buffer JSON roundtrip.
- Original 🧳️original/🧬️schema/🔣️.json requires receiver→output→original-source→frame close order, original-exact-typed-source authority, funded-once output and4096copy quantum. Its TS tests use Ajv plus SQLite to verify that declared custody ordering.

These laws are the available intended contract, not newly observed passing results. Repairs should preserve grant checks, child pointer identity, genuine original authority, typed/dynamic facet separation and exact funded cleanup. No compatibility branch, new runtime dependency or guard suppression is called for by this audit.

## Targeted Name / Ordinal / Tag / Flatten Semantics

The actual text source is 🗣️dsl/🧬️schema/🛬️receive/🦀️.rs:15–19. record_key(index) obtains the stored field id from get_index and returns Ordinal(id); index is enumeration position, not identity. Field child remains the value at that enumeration position. DSL derive record-field-codegen/🦀️.rs:39–60 sets id to Rust declaration index as u16, with a separate positional counter; authored key/rename does not change that id. Sparse, reordered or skipped field entries must therefore not be matched by current scan position.

NativeRecordField.matches is the existing exact law: Name compares the canonical declared wire name; Ordinal compares the actual declared field id. The neutral fixed-record fixture deliberately reorders canonical names while text fields retain ids0/1/2. No mechanical Ordinal-to-decimal-string conversion is supported. The current value-derive parser supports rename/rename_all, but an arbitrary serde alias is not automatically a value-native alias; do not silently invent another accepted name based on a third-party cold-codec test.

Tagged text enum behavior has direct executable-law source: DSL receive/🧪️tests/🦀️.rs:46–53 derives OriginalStatement with tag kind and Nested/Mark payloads. The actual original is (String,RecordValue), with variant_tag and variant_payload and payload fields Ordinal0(label),Ordinal1(count). There is no synthetic kind member inside that text payload. Fixed payload ordinals belong to the carrier record itself; canonical JSON tagged-record key scanning and direct text variant facets are distinct source representations. Scalar enum ordinal likewise has its own enum_ordinal port and must not be confused with record field ordinals.

NativeValueSource.record_name at tree17 is the existing strict projection for genuinely named dynamic records; it refuses Ordinal. Record entries at tree37 and DSL receive33 represent borrowed original map entry owners, not synthetic keyed record fields or copied pairs. Unnamed tuple input uses sequence_child/sequence length and ordered tuple semantics, independently from record keys (tuple funded laws21–24).

No existing portable/native law found establishes an ordinal offset/renumbering algorithm across flattened records. Native FlattenedRecord in sealed-shared-receiver tests98–114 exercises original DslValue Name keys, including duplicate/unknown/missing refusal; the cold flatten-with-skip integration tests compare Serde JSON Name semantics only. Two flattened children each declaring Ordinal0 would be ambiguous without further authored identity. Existing native membership-count semantics refuse membership>1; inventing offsets or strings is not justified by those laws. A new explicit cross-format flatten rule would need its own schema/law, rather than being inferred from compiler errors.

These exact references and limits were sent to frontend/backend while they own implementation. This audit changes no source and executes no tests.

## Pre-Edit Source Fixture Recovery Evidence

Backend reported accidental overwrite of 🌳️value/📖️source/🧪️tests/🔣️.json, subsequent HEAD restoration and moving its new schema to a separate filename. This audit checked only its own prior read context and known retained artifacts. No source or Git restoration was performed.

My pre-edit tools read the source projection Rust tests and full oracle.mjs, which names and loads this fixture, but did not read that specific fixture JSON or compute its hash. The separately read tree/🧪️tests/🧫️fixtures JSON and tree/🧬️schema are different files and cannot establish its bytes. Therefore I possess no pre-edit full content/hash evidence.

Known captured diagnostic runner retains snapshot only as in-memory text supplied through owned auxiliaryInput; it does not write snapshot.json. The known generated capture logs contain timing/count metadata, not a source image. The previously named old farm snapshot /tmp/semio-graph-farm-image-194452-cXr00b/snapshot.json is absent. No independent byte image was found in these exact known artifacts. No broad cache/directory search was attempted.

HEAD equality and a passing Lodash oracle can establish the restored fixture is valid and behaviorally accepted, but cannot prove no pre-existing uncommitted fixture content was lost. Pre-edit equality remains unproven by this agent evidence. Backend and root received this explicit limitation.

## Sequence Funding Patch Source Review

Read current tree135–166 and NativeReceiveSlot49–62 after backend edits. No concrete defect found in the reviewed ownership/funding boundaries. No build/test/probe performed; compiler and funded-prefix laws remain pending, so this is source consistency only.

NativeSequenceReceiver now declares outer I:?Sized, child NativeReceiveSlot<dynNativeValueSource,T>, C collection and S input strategy with SequenceRows default. Its impl/Drop and Vec/array/paged adapters use the same four-parameter shape. source at136 validates then projects S::child through original RetainedCloneRef; it does not allocate a proxy or replace original authority. RecordEntries uses record_entry, while SequenceRows uses sequence_child.

Collection interface118–126 separates initial backing demand, incremental reserve demand/step, push depth and final transfer. Paged adapter156–164 uses existing has_reserved_slot, next_reserve_copy_byte_demand, next_allocation_bytes, next_reserve_depth_demand, reserve_one_funded, next_push_depth_demand and push_reserved. Its count≤N bound was separately verified against the canonical list ceiling by root; this audit does not infer another page capacity meaning.

Phase3 at138–139 quotes reserve work before calling reserve_step and charges declared capacity. Without reserve work, it obtains canonical push depth and quotes transfer metadata. On push rejection the returned T is restored into child.output before returning an error with paid progress; index, phase and child-close initiation stay unchanged. Slot.take at56 is only Option.take, so the restoration does retain the exact owner and does not lose an independent receiver state. This branch is not proven executed until the pending regression runs.

All advance paths first call retained_clone_grant_admits on their quoted demand before mutation; denied item/copy/capacity/release/depth grants return default progress. Cancellation closes the child receiver/output before collection output/storage and final source binding (143–146 and Slot57–60). The actual slot output/receiver remain independently retained through cancellation; no destructor-based shortcut or unmetered source release was introduced by the inspected patch. Pending tests should remain authoritative for grant receipts, allocator equality and every cancelled prefix.

## Failed Native Build17816 Fixture Evidence Check

Bounded follow-up inspected only known proctor-native-2026-10-28.md, backend-2026-10-28.md and retained generated/proctor-native-build.log for an actual17816 Source capture/receipt/inventory path. No explicit artifact path identifying a persisted old-byte capture of 🌳️value/📖️source/🧪️tests/🔣️.json was present. The log mentions source.capture progress but not a sealed fixture content/hash record. Therefore no additional artifact was opened and the search stopped, without a broad native-cache crawl or restoration.

The fixture belongs to a cfg(test) source module, while17816 was a normal proctor release build; that makes its inclusion less likely, but is not independent proof of the exact admitted file set. No claim is made that the compiler consumed or excluded the fixture without its actual Source inventory. This check supplies neither old bytes nor a pre-edit digest; original equality remains unproven.

## Actual17816 Diagnostic Primary-Root Triage

Parsed only retained proctor-native-build.log from its first E0107 at log12349 onward, excluding earlier mode conflict and derive E0609 attempts. Normalized primary source paths and grouped code+path without counting secondary notes as independent diagnostics. Total228:

| Primary path under owned | Code | Count | First location |
|---|---|---:|---|
| tree value |E0107|1|148:137|
| list |E0277|54|23:62|
| tree value |E0053|2|128:513|
| tree value |E0277|18|149:115|
| tuple |E0277|68|5:143|
| tree value |E0599|8|62:143|
| tree value |E0308|1|136:677|
| tuple |E0599|70|17:57|
| map |E0308|1|20:157|
| map |E0599|4|21:131|
| map |E0608|1|43:124|

Tree value is 🌳️value/🦀️.rs, tuple is its 👥️tuple/🦀️.rs, list 📋️list/🦀️.rs, map 🗂️map/🦀️.rs. No primary path outside those four current owned clusters appears in this actual228diagnostic batch. The138tuple diagnostics begin with dyn NativeValueSource implicit Sized and downstream NativeReceiveSlot method bounds, consistent with the common slot migration cascade; source review does not establish138independent tuple defects. Tree E0053 is the collection push return-contract mismatch, owned by the sequence patch. Map errors match dynamic child projection and enum-as-string key use, owned by frontend.

No new unowned independent root was identified. This does not guarantee the corrected compile reveals no further errors; the failed compiler batch may stop before downstream crates. Both owners received the exact counts/locations. No test/build/source edit occurred, and fixture pre-edit uncertainty remains unchanged.

## Stable Fixed-Record / Key Dispatch Patch Review

Coordinated stable files with frontend_live, then read owned NativeRecordFields/key helpers, derive receiving core and enum generator. No concrete new defect found in the reviewed dispatch paths. No test/build/probe/source edit; quote compilation and runtime assertions remain pending.

Fixed-record matching now delegates NativeRecordField{name:wire,ordinal:declarationindex}. Scan stores actual source enumeration position in positions; phase3 uses that runtime index to borrow record_child/sequence_child. The apparent index in source-generation quote is a runtime binding from positions (derive core86/104), not the compile-time field index accidentally substituted. Name and Ordinal identities therefore remain independent from input ordering.

NativeRecordKey.comparison_bytes uses UTF8 name.len or u16 width; generated schema comparison budget uses actual declared UTF8 wire lengths for Name and fixed u16 widths for Ordinal, delegating unchanged key through flattened children. No ordinal stringification or inferred offset was introduced. Membership>1 refusal remains, including flattened collisions, and borrowed original child projection keeps the parent lease.

Enum generator29–31 constructs all_indices/types/slots/constructors in consistent canonicalN then directN order. Direct variant selection uses N+variantindex, and child_source40 projects actual variant_payload. Generated direct named payload type excludes the canonical tag field; canonical carrier separately declares its own tag first with Ordinal0. Thus the text direct facet can retain payloadordinal0/1 without consuming a synthetic tag slot, while ordinary canonical key scanning remains supported. External canonical wrappers remain Name-only and fixed direct typed payload borrowing does not convert a wire owner.

The final guarded grant/close paths and the new tests must still be compiled and executed by the owning suites. This source consistency review does not establish backend build, browser integration or fixture pre-edit equality. frontend_live received the exact checked quote ordering and runtime-index reasoning before suite readiness.

## Patched Contract Test-Mount And Coverage Check

Read actual manifest/lib mounting while18416 queues; no build/test/probe. Native-owned manifest lib points to package 🦀️.rs, which mounts owned root at11–12 and reexports actual derive macros for cfg(test) at22–25. Root mounts native laws58, persistent-list laws59 and enum laws60. Tree154 mounts source_tests, tuple49 its tuple laws, shared80 its receiver tests, record44 its tests and map63 tests.rs. Referenced paths exist; no orphaned new Rust module or obvious path-refactor mismatch was found.

New erased-source test tree/source/tests8–12 statically instantiates NativeReceive<dynNativeValueSource> bounds for SealedShared, Vec, array, PagedList and PersistentList. Its execution then checks source strategies and JSON equality; it does not drive each of those five real receivers through funded advance/take/close. New storage test16–18 directly checks Vec/array push returns the exact rejected value and unchanged array; it does not exercise sequence receiver phase3 restoration or paged reserve across cancelled prefixes. Existing PersistentList laws33–75 and tuple funded laws are mounted, but compile-only erased receiver checks must not be reported as full erased-runtime funding coverage.

New record key test11–14 executes Name/Ordinal matching and comparison-byte fixture values. Map dynamic key test35–38 constructs Name/Ordinal original keys, directly checks fragment acceptance and closes the fresh receiver. New flattened ordinal test shared120 asserts membership count2 for colliding Ordinal0 and0 for undeclaredOrdinal1; it does not itself drive the receiver refusal branch. The existing Name-key flattened receiver cancellation/refusal laws104–114 remain mounted.

Native enum suite58 drives canonical DslValue variants plus Serde comparison and funded closure. The real text direct-variant plus ordinal payload law in DSL schema/receive/tests46–53 belongs to another crate and is not included by native-owned --lib. Its behavior must be verified separately or accurately named as pending. No zero-test or runtime-pass claim follows merely from compiling native-owned.

Third-party scope is likewise separate: native enum Rust laws actually use Serde; tree source portable oracle uses Ajv/Lodash and tuple oracle uses Ajv/Buffer but are JS invocations, not automatically executed by Rust --lib. Current record tests folder contains Rust and fixtures only; map contains Rust and fixture/schema only. Any newly prepared ticket portable oracle must be reported by exact path and its own execution result rather than attributed to Cargo unit-test execution. Owners received these precise scope boundaries; no new test was written by this audit.

## New Funded Erased-Source Driver Review

Read the newly added source/tests driver21–38 while12430 owns compilation/execution. It now instantiates and actually drives all five dynamic receivers: Vec, fixedarray, PagedList, PersistentList and SealedShared. receive_original projects original RetainedCloneSource into its dynamic source facet, calls real demands/advance until output_ready, takes/observes the output and closes receiver, output and source through funded custody. This closes the earlier compile-only coverage distinction for those successful paths, subject to pending execution. No concrete production defect found from this driver review.

At27 it rejects an items=0 grant before each real advance and asserts zero progress/allocator activity. It then checks actual allocation/release against each admitted advance receipt. At29 close steps are likewise allocator/receipt checked, and22 funds retirement of output/source with allocator equality. The expected sequence values come from neutral fixture1/2, independently reproduced by its TS Ajv2020/Lodash map oracle; record entry values use toPairs. That oracle is a separate managed JS execution, not implicitly run by Cargo.

Precise remaining evidence limits: this driver denies only the item axis, not copy/capacity/release/depth; it does not immediately compare post-denial demands/cursor state, and original equality is checked only after successful completion/close. begin_receive receipt/allocator activity is ignored at26; take progress is checked at28 but take allocation is not observed. It follows completion, not every cancelled prefix. Existing other funded laws may cover those boundaries, but this new driver alone must not be described as exhaustive five-axis/prefix allocator validation. These specific scope limits were sent to backend_live; no new test or source change was made by this audit.

12430 compilation/run claim remains pending. Fixture pre-edit equality uncertainty remains unchanged.

## Distinct DSL Direct-Variant Law: Exact Owning Test

Root now reports native-owned13230 release suite31passed/0failed, including the new funded erased-source/key laws; proctor12123 remains a separate queued build. This source-only preparation does not launch another test or upgrade the app runtime claim.

The actual owner of the direct text variant/ordinal payload law is semio-framework-dsl-record-model, not the outward dsl-record crate. Manifest 🗣️dsl/🧬️schema/📐️model/📦️packages/🦀️rust/Cargo.toml points lib ../../🦀️.rs to model source. That source10–11 mounts ../🛬️receive/🦀️.rs as native_receive; receive13 mounts cfg(test) tests. Framework Cargo.toml member314 explicitly includes the model package; its package.workspace resolves to framework.

After app build/capacity permits, the exact supported command from cwd /home/deck/semio/🧰️framework is:

~~~sh
repo --lang=en cargo --store=shared --configuration=release --parallelism=1 -- test -p semio-framework-dsl-record-model --lib native_receive::tests::genuine_tagged_records_receive_original_ordinal_payloads_without_wire_conversion -- --exact
~~~

Source test52 defines that precise mounted name; if compilation and test-binary execution succeed, the filter matches one test. Require the actual result to say1passed/0failed, not a zero-test exit0. This command is prepared, not executed.

The law at46–53 derives OriginalNested(label:String,count:u32) and OriginalStatement with #[value(tag=kind)] for both newtype Nested and named Mark variants. For each tag it constructs actual RecordValue fields id0text/id1integer under (String,RecordValue), asserts variant_tag and payloadOrdinal0, receives through dynNativeValueSource with real grants, denies zero-items, checks allocation receipts, validates resulting label/count, funds receiver closure and source/output retirement. Thus this is the distinct real text variant facet path that native-owned canonical DslValue/Serde enum tests do not substitute for. Its limited denial/close scope should still be described exactly; no exhaustive new claim is added.

## Exact Fixture Git Index Check

At root explicit request, read only git show :<exactfixture> and git show HEAD:<exactfixture> through repo Node, alongside the current physical bytes. No modifying Git command, stash, checkout, restoration, source copy, reflog or object crawl. Exact fixture is 🌳️value/📖️source/🧪️tests/🔣️.json.

Index, HEAD and current each contain895bytes and have identical SHA25647b09194c975ae9ad0fa1de6b24b2f69e748ec2492b19bccb15f263ac15cb746; direct byte equality checks also returnedtrue. Therefore no distinct staged uncommitted fixture bytes were available in the index at this check. This is negative recovery evidence only: it cannot prove no unstaged pre-overwrite content existed or establish pre-edit equality. No restoration was performed. Backend and root received the actual hashes/equality result.
