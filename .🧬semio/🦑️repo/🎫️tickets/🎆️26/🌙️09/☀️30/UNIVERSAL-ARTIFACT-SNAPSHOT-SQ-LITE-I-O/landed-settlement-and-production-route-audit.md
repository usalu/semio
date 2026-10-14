# Landed Settlement And Production Route Audit

Read-only source review, 2026-10-13. Root Source30690/Native29513 live; no new builds or runtime claims.

## Actual Scope Binding

`🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs:2612` now stores `RefCell<SqliteSnapshotReceivingControl<'a,'b>>`; constructor captures remaining then enters actual `control.receiving_scope()`. Existing double dereference binders correctly reborrow actual Control. Guard survives all owner receive, close/publication callbacks and residual settlement until settlement drop. Settle function argument coercion follows RefMut -> Guard -> Control. No new concrete lifetime/ledger/cause defect found by inspection.

Nested History guard records previous=true and restores true before outer guard restores its original mode. Body observer remains no-borrow while post_body=false. Binder drops its body borrow before enabling post-body; close observer takes temporary settle borrow then a separate checkpoint borrow. New Kernel negative macros still keep settlement through original zero/funded cleanup and final residual reconciliation, with actual inner cause pointer captured and compared afterward. Source guards check genuine receiving guard and actual mounted law names; execution outcomes remain separate.

## Actual Full Typed File Caller Boundary

Repository Rust search excluding test/testing/generated/ticket paths returned only the IO declaration owner plus three files with inline cfg(test) modules: DXF R12 header, PDF1.4 and PDF1.7 SQLite snapshot owners. Their typed file calls are inside those test modules. Earlier 408 migrated expressions remain the test corpus; no non-test production invocation of these direct typed full snapshot file APIs was found in this search. This is a source caller inventory, not proof of every dynamic linkage or runtime route.

The current production graph uses actual `snapshot_export_hop`/`snapshot_import_hop` with `provider.export` at IO line2759 and `provider.import` at2797. Registered `ArtifactSqliteSnapshotCodec` still takes/returns IoPayload; that payload's declared schema is only Text(String)/Binary(Vec<u8>) (`🧰️framework/🔨️modules/🚪️io/📦️payload/🧬️schema/🦀️.rs:10`). Its doc explicitly defines raw native encoding as route currency. It cannot borrow a complete arbitrary typed Snapshot from Store by itself.

Therefore current paid guard improves genuine raw route accounting and direct typed API custody but does not mount a global complete-snapshot production route. Existing `ErasedSnapshotRead::get<P>()` still provides the exact borrowed Store root needed for registered full export; registered import should receive directly into actual caller typed owner/slot. Keep raw payload equality separate from full snapshot equality, preserving current raw format wire.

Minimal next integration remains `full-snapshot-registered-thunk-contract.md`: synchronous full snapshot thunk specialized by actual P, borrowed original registry identity/schema and existing read, no export clone/raw normalization/BoxFuture. Imported P enters original paid destination before publication; no fabricated Any box or synthetic Store lease. Current typed async implementation has no await and can share a synchronous receiving producer with the registered thunk. Preserve exact provider identity comparisons when registering this added capability.
