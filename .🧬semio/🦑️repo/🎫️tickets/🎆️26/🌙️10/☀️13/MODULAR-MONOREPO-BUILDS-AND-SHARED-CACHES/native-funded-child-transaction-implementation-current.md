# Native Funded Child Transaction Implementation

## Current Consumed Source

Existing acceptNativeStorageCleanupV1 now consumes storageCustodyV1.transaction rather than acquiring a second journal lease through a duplicate path. The transaction privately supplies captureJournalCleanup, bound to its own actual journal EX, real issuance control, lease directory and original backing identity. Only private transaction code can receive this closure; it is not exported or a public registrar. Actual lease issuance/private liveness refuses a delayed call after lease release. The public allocation cleanup API and its real receiver/physical capability remain unchanged.

Both normal transaction replay and cleanup now use the same bounded descriptor observation of storage-events.jsonl. Initial regular/no-link/single-link extent is checked before allocation; the physical read ceiling is that captured extent under the finite schema maximum; same descriptor/path identity is checked before and after the read; UTF-8, complete line framing and full schema replay remain mandatory. The real incoming control or actual cleanup token supplies every check and remaining value. No reader issues a new Original, control, Native provider or cleanup capability. This avoids routing the cleanup seam through the old readFileSync path.

The sole birth schema/model is finite DATA with two closed paths. Durable success retains the child and parent charge; failure or explicit retirement requires child-retired before parent-retired. The eight neutral corpus rows include early child birth, late loan, duplicate, unknown stage, early parent release, retained success and forbidden parent release after retention. Independent SQLite JSON path matching and Ajv validate the same expected output. The model is not a Native issuer or a completed constructor.

## Necessary Next Actual Owner API

The actual canonical factory must own one closed request before effects: actual retained parent, required incoming Original/control, exact parent policy, compiler-state owner/bytes, closed child directory/filesystem/backing envelope and producer policy digest. It accepts the same prebirth cleanup enrollment once. Private allocation birth state exists before append; captureJournalCleanup is invoked before the durable reserved event, not afterward from the returned reservation.

The next private backend attempt must be accepted before creating the child directory/backing, and retain its exact original creator EX, parent frame, request and captured identities. A durable producer preclaim must bind its exact directory/root dev/ino/mode and parent allocation association before the first backing-create command. Later durable checkpoints bind the actual backing, kernel mapping and producer receipt before successful transfer. A crash cannot turn an unknown directory or copied receipt into a new actual creator.

The actual parent journal must acknowledge this preclaim under its own writer before child creation proceeds. Its private source records the exact immutable association and actual issuer. Existing child-internal disposable allocation binding is preserved and remains a separate resource. Success transfers the actual retained cache binding and leaves the charge held. Failure/explicit retirement may remove the parent charge only after private backend completion proves actual accepted work drained, real child EX/descriptor recensus, exact allocation/payload settlement, kernel inactive state, exact known-file removal and metadata durability. A public Boolean, callback result, path, structural model port or copied token cannot authorize that transition.

The existing compiler volume is deliberately unchanged at this checkpoint: its reserve/intent/open sequence has not been claimed corrected by these prerequisites. No complete parent association, failed-birth teardown, whole-cache retirement or restart-recovery feature is claimed. Further code must integrate this genuine transaction and backend attempt, rather than expose the DATA model as authority.

## Limits

All tests use Source, ordinary ticket filesystem data, actual issued Original/cleanup lifetimes and actual SQLite exclusion. No Native/image/mount/compiler/store/Hub/SDK/browser/network operation ran. Actual backend constructor lifetime, whole-volume retirement, durable crash recovery, Windows directory durability and production capacity sharing remain unqualified. Existing caps, floors, cleanup reserve and budgets are unchanged. Historical failure logs remain retained.

## Checkpoint

Final Source93235/prototype and57448/release: BOTH17 tests/463 assertions, terminal0. Expanded strict17331/prototype and41150/release: BOTH0. All own handles terminal. Exact30-file hash roster and earlier genuine REDs are in whole-cache-retirement-execution-current.md. No actual funded constructor or runtime feature is claimed.


## Successor Preclaim Checkpoint

See [Native Funded Child Preclaim Checkpoint](native-funded-child-preclaim-current.md) for the exact successor Source snapshot. Actual filesystem preclaim and canonical parent preappend allocation custody prerequisites are implemented; final Source93231/prototype and8644/release BOTH32/837, expanded strict82755/prototype and27116/release BOTH0, all own handles terminal. The canonical backend attempt/factory/compiler-volume integration remains intentionally open for independent Source review. No actual Native operation or complete funded constructor/retirement qualification is claimed.


## Actual Provider Attempt Successor

[Native Constructor Preparation Attempt Checkpoint](native-constructor-attempt-current.md) records the actual Mac/Fixed ordinary-success versus deferred-failure split and real pre-effect EX cleanup loans. BOTH Source37/892 and expanded strict BOTH exit0; six changed paths and full observed closure are frozen for review. Parent-token funding orchestration, backend preclaim adoption, handoff-gap retirement and Volume wiring remain open; no Native runtime or complete funded-constructor claim follows.
