# Registered Full Snapshot File IO Contract Plan

Read-only source plan, 2026-10-13. No build or runtime qualification.

## Use The Existing Owner Boundary

Export already has a repository-owned full snapshot boundary: `🏪️store/🔗️read/🦀️.rs::ErasedSnapshotRead::get<T>()` returns a borrowed exact root. A registered `P` thunk should downcast this read and call the full typed file producer. It must not call `try_duplicate`, clone the root, or export a raw `IoPayload` first. Original read lifetime extends through operation completion; caller/issuer keeps the original return witness contract.

Import is different: a newly reconstructed `P` is not an issued Store read. `ErasedSnapshotRead::new` is crate-private and requires an actual original registry lease. No inspected constructor turns arbitrary imported `P` directly into a public erased Store read under a caller grant. `RetainedConfigStoreHydration::from_snapshots` (`🏪️store/🎚️config/📥️retained/🦀️.rs:109`) requires admitted genesis, validation/current roots, history, owner catalog and metadata; it constructs an Arc-backed history and takes no receiving grant. It is not a general paid import result constructor. `create_document_envelope` also creates genesis through native encoding, so it is not the full snapshot persistence receiving boundary.

The reusable actual retirement API is `snapshot_read::artifact_retirement_admit_owned<T>` (`🔗️read/🦀️.rs:1400`): observes `Option<T>`, requires empty original active slot, checks item/depth/capacity before taking the original, admits typed retirement with child depth, records exact admitted receipt, and restores the original pending owner on refusal. This erases retirement, not a readable snapshot. A retirement cursor must not be presented as a readable imported snapshot.

Minimum wrapper-free import contract: registered specialized thunk receives a borrowed caller-owned typed receiving slot/owner through its existing Any loan, validates its concrete `P` destination before physical import, reconstructs directly into that owner's retained `Option<P>`, and invokes the caller's original acceptance/publication transition inside original native receiving custody. The destination owner remains responsible for readability and its existing typed retirement. An external callback that merely takes `P` after typed API return is too late to account for additional owner birth/publication. If there is no existing readable caller destination, that interface requires an explicitly schema-defined paid result owner; do not pretend an invented Any box is free.

## Avoid An Unnecessary Async Allocation

Current `🚪️io/🦀️.rs::io_export_sqlite_snapshot` and `io_import_sqlite_snapshot` are async declarations whose bodies execute synchronous receiving closures and contain no await. Extract their genuine synchronous typed implementation and have public async functions call it. Registered function pointers can return `IoResult` directly, or `Ready<IoResult<...>>` for an async-facing facade. This does not require a boxed future, a cloned Codec, or a new receiving authority. If real async work is introduced later, its concrete future/boxed erasure must become a separately quoted retained frame, with cancellation and close custody; boxed async is not currently necessary.

## Registration Identity Changes

`🏪️store/🚪️io/🦀️.rs::ArtifactSqliteSnapshotCodec::identical_to` currently compares schema, optional TypeId and both raw bridge function pointers. New full snapshot thunks must be included in identical-provider comparison, including optional presence; otherwise duplicate registrations with different full snapshot ownership behavior are accepted as equal.

`ArtifactNativeSnapshotIdentity::validates_provider` checks typed TypeId equality and a nonempty owner label; guest identity requires provider TypeId None and matching schema. Full host-native thunks must require typed identity and the same TypeId. Do not assign a host full snapshot thunk to a guest provider by accepting None. Actual registration checks are `🚪️io/🦀️.rs:2259` (identity validation) and `:2267` (duplicate provider comparison); typed direct admission additionally checks provider TypeId and exact `P::SQLITE_SCHEMA` at `:2602`. Keep these checks borrowed and before output/frame birth.

Raw `IoPayload` bridge fidelity describes raw-domain input/output equality. For TXT raw text, custom snapshot schema is not part of that input authority. Therefore loss of arbitrary snapshot schema proves that raw IO is unsuitable for complete snapshot IO; it does not alone prove that the raw-domain descriptor's equality claim is false. Qualify and expose the two scopes independently. The new complete snapshot capability should have a distinct descriptor/contract, rather than silently treating raw input as every legal `P`.

## Neutral Contract And Real Owner Laws

Recommended closed language-neutral fixture fields: dialect; legal complete snapshot (all required TXT schema/lines/lineEnding/trailingNewline fields); expected exact relational rows and metadata; physical file encoding; source owner identity; preexisting cumulative receipts; original body grant/native ceiling/SQL limits; original close grant; expected refusal axis; expected terminal retained state. Keep authored caller authority independent of observed demand.

Use a real registered test owner implementing full SQL snapshot and RetireOwned, with a field deliberately not representable by its raw codec. Export law obtains an actual Store `ErasedSnapshotRead`, records `get::<P>() as *const P`, invokes the registered thunk, and checks the same pointer/source fields before original read return. Pointer identity belongs to the same captured original root, not to a separately reconstructed import root.

Import law lends the actual typed receiving owner/slot. Assert exact fields after publication; record its original allocation before birth; deny each grant axis independently and show no detached P/diagnostics; deny close at the original canceled observer, rearm the same Cell and fund cleanup through the same recipient. Include wrong TypeId/destination/dialect/schema refusals before owned birth. Independent SQLite inspection verifies the physical file and rows, while exact typed fixture equality verifies snapshot fidelity; raw byte roundtrip is not that witness.

Source guards should verify the registered thunk uses `get::<P>()`, current original receiving hooks and pre-admission, without pinning incidental physical field spelling or requiring boxed futures. No selected Native laws are claimed by this plan.
