# Shared Original Receiving Interface Audit

Read-only current-source review, 2026-10-14. No edits, builds, or tests. Paths are relative to `/Users/ueli/Documents/semio`.

## Actual repeated owner code

LAS `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:57–75` and SemioText `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs:50–68` repeat the same Settlement fields/new/settle/finish_body/observe/refused and receiver macro. Differences are domain Frame/input/output/operation/publication. EPW should use a shared owner rather than create a third copy.

## Safe lower-layer extraction

Place `ReceivingSettlement<'scope,'caller>` beside existing `settle` at `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🫴️receiving/🦀️.rs:56`. The crate manifest `.../📦️packages/🦀️rust/Cargo.toml` has only semio-framework-value as a runtime dependency; preserve that boundary. No OS owner import or external API type is necessary.

Retain the exact current storage: RefCell<SqliteSnapshotReceivingControl<'scope,'caller>>, original native baseline, original SQL remaining bytes, phase, Cell post_body and RefCell<Option<ValueError>>. Expose new, with_control (closure receiving a temporary &mut SqliteSnapshotControl), finish_body, observe(owned,completed,total), refused. Returning a RefMut is possible but a closure limits borrow escape and hides RefCell representation. Shared settlement owns only a borrowed SQL scope; its Drop must restore required_allocation through the existing scope144–147/161, never replace the ledger or grant. All fields are inline; construction must birth no heap.

Preserve current settlement arithmetic exactly. During body work the observer returns true because Port owns SQL checkpoints/admissions; after finish_body, observer settles newly born retirement backing and checkpoints the same SQL phase. finish_body must release the RefCell borrow before native callbacks/retirement. On refusal, actual settlement failure has priority, then captured SQL callback cause, then original native/domain error. No clone/new error String or conversion to generic cancellation. Preserve cumulative baseline accounting across repeated settle calls; do not reset before/remaining after each observer event.

## Generic Frame orchestration at the existing native owner boundary

The actual public types are `NativeSnapshotDecodeOwner` and `NativeSnapshotEncodeOwner`, not a separate OSNativeSnapshotOwner trait. Their implementation sources already live under domain-neutral framework paths: `🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs:47–78` and `.../⏱️control/🛫️snapshot/🦀️.rs:5–27,74–76`. They are exposed through the kernel facade used by domains. Lower SQLite must not depend back on that facade.

Add matched inherent helpers (for example receive_sqlite_frame) to these existing owners, implemented close together through one internal macro. This avoids a new generic owner trait with difficult generic callback/scoped-lifetime methods and avoids object erasure. A generic helper takes SQL control, a zero-backing Frame constructor, an operation closure, a retained status accessor, and a publication closure. Bounds are F:RetireOwned and O:RetireOwned; Decode/Encode use their existing native control types and the actual NativeSnapshotBodyWallet. Closures borrow F/Port temporarily and may capture borrowed domain inputs without storing borrowed pointers in F.

Retained status accessor has shape `fn(&mut F)->&mut Option<Result<(),ValueError>>` (or an equivalent first-party Frame trait), so the error remains in F until successful finish_body. Do not simplify into `operation(frame,port)?`: if finish_body itself refuses, the domain cause must remain under the same paid frame custody. Constructor runs only after Port.work(1,size_of::<F>()), installs Some(F), then output-header work precedes operation. Publication runs only after settlement body receipt succeeds and retained status is extracted. Domain publication moves existing retained output with an empty replacement; it must not clone or create allocating defaults.

Keep the existing sequence: original native checkpoint; checked before+SQL remaining maximum; borrowed ReceivingSettlement; original owner.scoped_native; existing owner.receive<F,O>; install retained Frame; paid operation/status; drop Port/control borrow; finish_body; domain status; publication; refused-cause resolution; append exact owner.progress. The existing receive primitive owns both intermediate and accepted output until paid retirement/publication finishes; use it unchanged. It checks retirement support, initial item/depth/capacity, pending owner and actual native frame charge at snapshot92–126. A standalone helper returning a temporary Frame/output outside receive would break failure custody.

Do not merge native Decode/Encode controls, manufacture a fresh owner/grant, use Box<dyn callback>, or move Frame into a local on error. Existing scoped_native accounts cumulative original progress and parent pending custody; an additional helper must delegate to it, not duplicate ledger settlement. SQL settlement is independent allocation accounting, not a second physical receipt spend.

## Required proof for extraction

Root-owned schema-first fixture should assert original receiver construction/operation/publication order, body refusal retained before finish failure, post-body observer SQL cancellation preserved, and nonzero preexisting native/SQL cumulative baselines. Existing LAS/SemioText complete laws then verify shared helper semantics; EPW uses the same helper. Physical source/runtime qualification remains pending for this proposed extraction. No new authority is justified by reuse.
