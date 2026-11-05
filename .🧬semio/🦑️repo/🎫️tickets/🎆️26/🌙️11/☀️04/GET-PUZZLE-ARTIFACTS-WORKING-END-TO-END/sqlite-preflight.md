# Physical SQLite Native Owner Preflight

Read-only source review; no compiler, tests, installs, grants or source changes. Physical SQLite/Bun oracles remain required; this review does not establish execution success.

## Concrete compile blockers

1. **2d malformed UFCS at four call sites.** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`, lines 84, 90, 130 and 134, places `original_owners::with_decoding` beneath `<Puzzle2dPlaySnapshot as ArtifactSqliteSnapshotHost>`. The helper is a module function, not an associated trait item. Put the actual helper outside the closure and the trait-qualified `decode_sqlite_snapshot_native` inside. Native decode still takes payload, SQLite control and native owner; preserve all three.

2. **2d independent serde oracle asks a dependency for a test-only impl.** Same file line 27 calls `serde_json::from_value::<Puzzle2dSnapshot>`. Actual snapshot dependency derives serde only under `cfg(test)` (`2d/.../any/🧬️schema/📸️snapshot/🦀️.rs:31`), has no testing feature, and has no manual Deserialize implementation found. An artifact root lib-test does not enable cfg(test) in its snapshot dependency. Preserve the independent oracle through a proper test-owned representation/owner; do not add runtime serde or remove this law.

3. **5d retirement outcome no longer supports Result::unwrap.** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`, lines 29 and 51, invoke `.unwrap()` directly on `native.close_retirement_recipient(policy)`. Both current native controls return `RetainedWorkOutcome`, whose public fields are result and work (framework value/control/contract line 155), with no unwrap method. Canonical original-snapshot-owners test helper accesses `.result.expect(...)` and retains progress. Preserve the close loop, receipts and owner cleanup on errors.

## Current calls that are not compile drift

The 3d controlled Scale constructors still accept `(4096, &mut callback)`; the three controlled DslField methods still use their current native controls. SQLite projection/reconstruction signatures still take `&mut SqliteSnapshotControl`; native host encode/decode still require the third native-owner argument. Existing original_owners wrappers and erased export/import calls match those signatures. The unused 5d decode_native macro has no invocations and is not counted as an expansion compile blocker.

Existing artifact manifests already declare original-snapshot-owner test helpers and relevant artifact-app-testing/protocol-laws/mutation-testing dev features. No concrete artifact/viewer factory mismatch was found in this bounded review. Fixture contents and physical Bun SQLite query/edit/integrity/malformed oracles were untouched. Positive direct 3d projection uses a newly created SQLite control; runtime recipient admission remains to be proven by the actual owner execution, not inferred from successful type checking.

## Repair review

Read-only review confirms all four 2d decode closures now place with_decoding outside the current trait-qualified call and retain payload, exact SQLite controls/cancellation and actual native owner. The unavailable dependency serde derive was replaced solely in the test: independently serde-parsed full fixture is recursively compared against the owned typed snapshot encoding, with exact object key/cardinality, array order/length, scalar kind/value and f64 bit comparisons. Integer versus float JSON spelling can agree only through the same exact f64 bits. The separate 43-word native laws remain unchanged; no tolerance or runtime Serde derive/dependency introduced by this patch. This remains a complete structural third-party JSON oracle, not an independent typed serde implementation.

Both 5d close loops now inspect success/error retained progress against the unchanged grant before extracting outcome.result; work is retained in diagnostics. Native owner is scoped before close, repeated close and final no-retirement-owner assertion remain. An error still fails the test immediately rather than claiming cleanup succeeded; no success-path custody bypass added. No fixture, physical SQLite oracle, grant or law removed. No further concrete mismatch found in the reviewed changes; actual owned native execution remains necessary.

## Physical Test Binary Allocator Preflight

Actual2d/5dartifactroot manifests lib paths resolve directly artifactroot🦀️.rs; neither declares HeapWitness/global_allocator. Actual3d dedicated snapshot-sqlite integrationroot likewise has no HeapWitness. However the reused original-snapshot-owners helper contains NO observe_heap_allocations or heap/requested-vs-receipt assertions and requires no globalallocator witness; it installs actual native recipients, closes them and checks fundedprogress where applicable. Scopedphysicaltest Rust source also contains no heapobserver/witness assertions. Therefore absence of globalallocator in these three binaries is NOT the concrete missingwitness defect of the separate2daggregate source-custody helper. No witness/bound mismatch demonstrated here; do not add allocator or weaken/helperchange based on analogy. Existing generous original grants unchanged; actualnativephysicalresults remainpending.

## Actual Root2d7406 Outcome

Independently observed retained sqlite2d-1.log through ordinaryrepo reads, withoutpolling/restartingprocesshandle. Actual command ends OwnedCommandFailure status101 after nativecompilation; no selected2dtests executed or count verdict. First concrete errors are shared Flow host-source E0404 expectedtrait/foundderive macro semio_framework_value::RetireOwned at ordering source. Exact diagnostics:

```text
error[E0404]: expected trait, found derive macro `semio_framework_value::RetireOwned`
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/📦️packages/🦀️rust/../../🔤️ordering/🦀️.rs:55:38
   |
55 | macro_rules! owned{($type:ty)=>{impl semio_framework_value::RetireOwned for $type{fn retirement(self)->Box<dyn semio_framework_value...
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a trait
56 | owned!(OriginalTextOrdering);
   | ---------------------------- in this macro invocation
   |
   = note: this error originates in the macro `owned` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these traits instead
   |
 2 + use crate::RetireOwned;
   |
 2 + use protocol::retirement::RetireOwned;
   |
 2 + use semio_framework_value::retirement::RetireOwned;
   |

error[E0404]: expected trait, found derive macro `semio_framework_value::RetireOwned`
  --> /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/📦️packages/🦀️rust/../../🔤️ordering/🦀️.rs:55:38
   |
55 | macro_rules! owned{($type:ty)=>{impl semio_framework_value::RetireOwned for $type{fn retirement(self)->Box<dyn semio_framework_value...
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not a trait
56 | owned!(OriginalTextOrdering);
57 | owned!(FlowTextOrderingWork);
   | ---------------------------- in this macro invocation
   |
   = note: this error originates in the macro `owned` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider importing one of these traits instead
   |
 2 + use crate::RetireOwned;
   |
 2 + use protocol::retirement::RetireOwned;
   |
 2 + use semio_framework_value::retirement::RetireOwned;
```

Couldnotcompile semio-framework-os-flow-host-source(lib) due2errors. Warnings/intermediate silence were not verdicts. This does not establish declaredJSONred or physicalIOsuccess; neither new law reached execution.

## Independent sqlite2d-3 Log Frontier

Read-only log observation now shows compiler failure before selected2d tests: procedural-3d-mut-widgets21errors, first E0425 `OriginalReceivingPrimary` not found in semio_framework_pack_json generated by NativeReceive at create-widget/update-widget/delete-widget/move-widget and related payloads. This is the previously reviewed receiving-owner class; immutable Source timing/current owner handle require parent confirmation. No intended2d test count or JSON runtime result observed, and no handle poll/restart performed.

##5d Native Decode Macro Host Trait Follow-Through

Independentcurrentdefinitionread confirmsArtifactSqliteSnapshotHost extendsbaseSnapshotandowns3argdecode_sqlite_snapshot_native(payload,SqliteSnapshotControl,NativeSnapshotDecodeOwner), liftedstoreIOline18. Actualdecode_native! nowusesHostUFCS andunchangedwith_original_decodeclosure/sql/nativeowner. Existingmacrocallsretainpayload/control/casevalues; baseSnapshottraitcannotownthismethod. NarrowcurrentAPIrepairsource-reviewed, physical5dexecutionstillpending.

Reachabilityprecision: scopedcurrent5dSQLiteRustsearch findsdecode_nativeonlydefinition69/pubuse74, no macroinvocations. ThereforeHostUFCSfixisvaliddormanthelpercorrection, notprovedcurrentexecutedfrontier; previousmacrocallsretainedphrasingdoesnotassertnonzeroactualcalls. Actualphysicalprefixmuststillqualifydirectnativepaths.


## Sixth physical 2d observation

Independent bounded read of sqlite2d-6.log now shows Cargo failure101 before any selected physical tests: E0308 at plugin reactor/surfaces/read.rs159. The retire_metadata closure expects unit, but its tail cursor.begin_close() returns bool; the compiler suggests terminating that call with a semicolon. semio-plugin-plugin-program failed with one previous error. Warnings are not classified as failures. Root76254 owns authoritative handle settlement; no physical 2d/JSON result is inferred.
