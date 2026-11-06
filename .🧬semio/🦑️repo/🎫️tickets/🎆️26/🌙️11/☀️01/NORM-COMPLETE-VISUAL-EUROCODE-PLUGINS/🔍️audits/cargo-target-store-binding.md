# Cargo Target Store Binding

Gate 70313 is an actual store-layout mismatch, not a test result. Its log records CARGO_BUILD_BUILD_DIR=/home/deck/semio/.cache/b/r/m/u/l/p42dbba and CARGO_TARGET_DIR=/home/deck/semio/.cache/b/r/m/u/l/p42dbba/t. The lane metadata identifies p42dbba as semio-framework-job. Its existing target receipt `.cargo-store.json`, modified 2026-11-05 02:12:10 UTC, contains version 1 with buildDirectory=/home/deck/semio/.cache/b/r/m/u/l/p42dbba/b and targetDirectory=/home/deck/semio/.cache/b/r/m/u/l/p42dbba/t. It is a regular physical file with one link, and its content matches its historical /b binding exactly.

Current unit-cache lane/🟦️.ts::lanePathsV1 returns build=base and target=base/t. The shared launcher obtains those paths from the acquired lane and passes both to acquireCargoBuildLeaseV1. The latter canonicalizes both and bindCargoTargetStoreV1 compares the exact serialized pair to its immutable physical target receipt. Thus the selected warm target was created under a different build-directory layout, and the current guard correctly refuses it. The log does not independently prove any particular source change history; the existing receipt and current lane derivation prove the mismatch.

The supplied `repo cargo --store=shared --configuration=prototype ...` is the correct explicit command; changing flags or overriding target directories cannot make this target binding valid. The production lane pool should distinguish its current build-layout identity when selecting a warm lane, retaining the old receipt and target custody; select/admit a fresh appropriately bound lane when incompatible, under existing capacity and leases. A regression should construct a historically bound lane, request the current layout, assert no mutation/deletion/rebinding and that the selected new lane has the exact current receipt. Do not delete or rewrite receipts or bypass the guard. This audit made no store changes and did not run another build.

## Current Layout Namespace Repair

An initial protected-incompatible-lane approach passed its direct laws but audit identified a genuine idle-lane starvation at the maximum live lane bound. It was removed, including its pool filtering and product identity changes; it is not the final implementation.

The current approach derives a fresh target directory namespace from a schema-owned version and the exact authored build/target layout inside the same exclusively claimed lane. The existing target receipt and directory are not rewritten, the bounded pool and product selection are unchanged, and current-layout callers reuse the same namespace. Compiler-origin checks now recognize exactly the current derived target namespace and its descendants. The independent schema-neutral layout corpus, existing concurrency laws and physical binding tests are running under normal capacity; no passing result is yet claimed.

Archive successor43068 and forms+B26406 returned 1 before Rust on distinct concurrent workspace identity inconsistencies; exact diagnostics are recorded in archive-native-work.md.


The physical dead-holder custody suite completed successfully (execution 8216): 6 tests and 62 assertions cover authenticated multiple target bindings and ledger publication, recovery after a killed holder, and refusal to trim conflicts or unresolved custody. The independent retention I/O suite and production capture remain pending. Published binding and destination records preserve historical evidence; they do not assert that a removed physical target still exists.

The independent retention I/O suite passed on the current source (execution 39592: 10 tests, 142 assertions), including six real concurrent publishers, refusal on destination-recording failure, conflict preservation and lane release. A further marker-association review tied parsed destination to the same physical marker identity and digest; the repeated full physical custody suite is queued as execution 17097.

Independent source review found and rechecked a marker association race: the current capture derives the physical shared destination and marker hash from the same parsed marker, and revalidates its inode and digest around publication and ledger settlement. No further concrete issue was found in that focused review. Retry 17097 was refused before testing because its operand lacked ./; correctly prefixed retry 59711 is queued.

The corrected full physical custody successor 59711 passed all 6 tests and 62 assertions after the final marker-association change. This complements retention I/O 39592 (10/10, 142 assertions) and the independent source re-review. Production capture remains pending.

Actual Cargo execution 35604 captured and admitted the current production source graph (451 units, 2,952 files); Rust compilation remains in progress. This verifies production capture, not archive or overall norm acceptance.
