# Native Owned Source Boundaries — 2026-11-06

Native qualification remains open. The current changes preserve the real mutable Original receivers and all existing private assertions; none of the boundary checks establish a compiled native or end-to-end pass.

## Canonical immutable owners

The actual NativeValueSource/NativeTreeKind/NativeRecordKey/NativeRecordField contracts and their implementations now belong to the firstparty semio-framework-value-native-source crate at value/owned/tree/source. The actual record schema matcher belongs to semio-framework-value-native-schema at value/owned/record/schema. NativeOwned explicitly reexports the same canonical types rather than introducing mirrors. Both use explicit normal firstparty dependencies, actual Framework workspace membership and matching outer exclusions. They do not depend on NativeOwned; the normal dependency graph is acyclic.

The authored input native-source-owner-checks.ts validates Cargo with the independent TOML parser, all normal path dependencies, workspace ownership, bounded paths and the real dependency graph. Its preserved result reports 23 normal owners, 87 edges and zero cycles.

## Preserved public native laws

Cargo's existing default integration discovery now includes these public-API law executables:

- native-sequence-storage
- native-map-public
- native-original-projection
- native-recursive-receiver
- native-persistent-list-public
- native-shared-slot-public

The public assertions and case corpora were moved to genuine standalone executables. Tests inspecting private cursor fields remain inline beside those fields. Existing public test facades use the canonical testing-admission crate instead of a deleted helper file. No source-limit increase, test removal, source exclusion or private-to-public implementation exposure was used.

## Fresh source closure evidence

The authored native-owned-boundary-census.ts uses the actual build source-closure reader with native cfg(test) flags. At this observation:

| Owner | Files | Bytes | Active lines |
| --- | ---: | ---: | ---: |
| NativeOwned library | 30 | 261164 | 1332 |
| NativeSource | 1 | 8045 | 55 |
| NativeSchema | 1 | 13730 | 50 |
| SnapshotProjection | 13 | 102336 | 692 |

All retain the fixed 32 / 262144 / 4000 limits. NativeOwned has 980 bytes of headroom. The six standalone public owners range from 4520 to 18643 bytes, each with at most four files. Later concurrent edits require a fresh observation.

## Projection work tests

The existing projection tests now pass explicit finite CPU limits and create their true Source through staged admission. The output-handoff test retains the real boxed donor throughout admission_borrowed, validates allocation observations and separately pays the allocation-marker clear.

The new derived String/Vec<String> record law consumes the schema-first bounded corpus. It covers maximum work 0, 1, 2, 4 and 32 plus every declared cancellation prefix. It checks quote <= requested work, actual <= quote, zero-work no effects, exact physical heap receipts, unchanged original pointer, independent serde_json output comparison, and paid retirement of each partial cursor/output/Source. All resources remain in the finite fixture wallet. Root owns the neutral AJV/SQLite oracle and production projection implementation.

The existing custom refusal test retains independent primary and receipt causes. The test Source factories use the canonical five-argument capture helper, whose CPU view is independent of retained-item currency.

## Qualification state

The real actor-receipt request 3395, captured in generated/mcp-58692.json, terminated after approximately 6244 seconds with an owned-process capture failure. Cargo child 66836 ended, its streams remained open, and the process owner reported 'Owned child terminal events lost: cargo'. This is an infrastructure failure, not a Rust type/law result. The capture also predates the new source-owner workspace memberships and later source edits. No duplicate or automatic restart was issued.

The native-owner-rustfmt input parsed the projection entry, both canonical owner entries and all six public executables. Exit 1 contained formatting diffs only, with no syntax errors. This is only a parse check. The actual compiled law results remain mandatory.

The final portable projection batch passed all three tests with 68 assertions: derive work quotes against independent SQLite, bounded multilingual projection fixture with AJV/SQLite, and scalar root-plus-parent capture fixture. The tool operand guard initially rejected unprefixed file filters before running anything; the retained input now uses explicit ./ paths. This does not qualify native execution.

Fresh no-duplicate evidence: doctor census at 1793959169231 found no matching ActorReceipt/NativeOwned/Projection target process. The actual repo leases_status --all result mcp-38256.json had no corresponding held/waited native targets. New native request inputs are prepared but wait for the shared storage/publication source edits to settle.

## Mandatory native receiver test migration

A later source audit found that the canonical NativeReceiver contract requires maximum_work_units in demands/advance while existing tests still used the old arity. Real receiver callers across 21 files were migrated. Inside an actually issued native work loan, the test borrows NativeDecodeControl.available_work_units before lending the mutable control to the child. Outside the callback, demands uses the existing finite fixture wallet fuel. Stale one-unit helper loans were replaced with the actual finite wallet view; no maximum integer, byte-derived fuel, dummy one-unit quote, or authority-free work stock was added. Source/header/default/variant selector APIs remain distinct and are changed only when their owning production port changes.

Test bank helpers now validate actual work against remaining finite CPU stock and retain checked debit. Controlled-retirement helpers check their genuine selected computational quote before effects. The three test NativeValueSource implementations quote their actual constant accessor work, including the two-call default RecordName path. The terminal PoisonHeader test receiver keeps its normal-path panic and quotes zero for an already empty logical child. The manual Row receiver delegates exact selected work to actual field quote ports and preserves complete retirement failures. Production NativeReceiver/field/variant/original-owner coherence is owned by Root and still pending.

The whole public original-receiving cancellation/denial law now executes in the existing native-original-projection integration owner. Its private receiver/original parent-close ordering assertions remain inline and run across all 128 cancellation cuts. Two public accessor/scalar conversion laws also have their own default native-source-accessors executable. No assertion or corpus was dropped.

After Server confirmed the shared-storage/publication-order fix, the real SnapshotProjection --lib replacement was issued with a four-hour client timeout. Its live exec session is 9628, launcher 21276, MCP server 2172. It was still in build/Cargo coordination at approximately 570 seconds, with no native result. NativeOwned remains unsubmitted until the discovered mandatory production/test port mismatches are coherent.

SnapshotProjection replacement 9628 is terminal: mcp-36724.json actual exit 1 after 1919594 ms, Framework manifest source extent expected 131745 but observed 132076. This is a stale-source precompiler refusal. No projection native law ran, and no automatic duplicate was started. Canonical source-owner memberships changed during this capture and are part of the necessary freshness context.

## Native Receiver Continuation — 2026-11-06

Scalar/text ownership moved into the canonical `semio-framework-value-native-leaf` crate. Standard-type `NativeReceive` implementations remain in NativeOwned and select the actual external receiver types; the trait did not move, so there is no orphan implementation or mirrored contract. Collection storage/reservation and immutable input selection contracts now live in the existing native-reservation owner. This owner holds the actual bounded selection and allocation effect; caller extent is immutable DATA and every physical birth remains bounded by the actual Source grant. Already-admitted physical capacity needs no further reservation.

The fixed-array path now retains a funded child projection and both value/retirement effects, narrows work against the actual live NativeDecodeControl, and carries the real marker through nested retirement. Initialized rows, unpublished donors and failed prefixes remain in the actual array owner.

The dynamic-tree owner now has separate persistent body/effect storage, actual pre-effect value/backing recipient frames, mandatory bounded work quotes and marker-last controlled close. Text/key copying and comparison are bounded by the actual work ceiling. Getter execution occurs inside the funded callback; demand selection reads only retained metadata/cached source references. Branch work/copy accounting was audited, including actual comparison prefixes, metadata writes, initialized frame capture, failure slots and empty-frame retirement. These changes are **not yet Rust type-checked or native-qualified**; pending callback/marker tests and the full native law run remain required.

A manifest inspection caught the new leaf dependency initially appended to the final test table. It was corrected into NativeOwned's normal dependencies before qualification. The strengthened independent TOML/path/workspace check now covers all seven canonical owners, finds 30 normal owners and 138 edges, and rejects missing normal paths/cycles. No runtime external dependency was added.

Fresh source census: NativeOwned 27 files / 260809 bytes / 1183 active lines; native source 1 / 8066; matcher 1 / 18892; scalar/text leaf 1 / 15248; dynamic tree approximately 27 KiB / 1 file; reservation 1 / 18965; contract 1 / 3146; SnapshotProjection 13 / 102865. All measured owners fit unchanged 32-file / 262144-byte / 4000-line limits. NativeOwned has only about 1.3 KiB remaining, so further growth needs a genuine owner/public-law factor.

The 22-root rustfmt parser invocation reported no syntax errors; exit 1 reflects formatting differences. It is not a compiler or test pass. The new language-neutral tree work fixture plus strict AJV validation and an independent SQLite quote oracle passed 1 test / 14 assertions. A separate genuine native tree executable was authored with actual source admission, finite work wallets, zero-work refusal, quote ceilings, heap/marker comparison, cancelled prefixes, serde oracle and complete funded close. That executable has not run.

Current native qualification is still blocked on full production/test API coherence, including all remaining `close_step(grant, actual)` and RetireOwned associated-`Retirement` test consumers. No replacement native request was submitted and no warming/source-only evidence is labeled native success. Existing native terminal evidence remains authoritative; the old projection run reached the PagedList compiler, then the Framework manifest extent fence, so it was not exclusively a precompiler failure.
