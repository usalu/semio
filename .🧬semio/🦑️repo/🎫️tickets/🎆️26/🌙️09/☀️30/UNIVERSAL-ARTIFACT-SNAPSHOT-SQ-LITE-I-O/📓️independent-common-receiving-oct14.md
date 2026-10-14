# Independent Common Receiving Audit — October 14

Read-only source audit; no Rust compilation or native runtime performed by this auditor. Existing linker ENOSPC receipt cannot establish EPW runtime acceptance.

## Current publication gap

Common `🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs` line 17 currently calls `settlement.finish_body(owned)?;` followed immediately by frame status take and `publish(frame)`. `finish_body` settles allocation and enables post-body observation; it does not invoke cancellation callbacks. Thus modes 3 and 4 can first refuse at generic receiving retirement, after domain publication has already occurred. The intended original `native.checkpoint()?` between finish_body and status take closes this specific gap in both macro directions.

The `native` parameter is available in the receive closure. The mutable borrow temporarily installed in Port ends when `settlement.with_control` returns. Calling native.checkpoint afterwards should require no new lifetime bounds or control replacement. Native scoped observer forwards the initiating callback first and invokes the settlement observer only if it succeeds. SQL cancellation becomes the original SQL cause stored by settlement; native cancellation remains the original native cause and does not invoke SQL callback. Refused settlement catches cumulative native deltas in both cases.

## Custody and source guard scope

The complete status remains `Option<Result<(),ValueError>>` in the retained frame until explicit take. A pre-status checkpoint error leaves status and intermediate frame intact for the generic receive wrapper to retain in its original recipient. Generic receiving stores successful output before intermediate retirement, so its later checkpoint alone cannot prove domain publication was prevented.

EPW modes 3/4 separately arm body_completed only after project/read succeeds, probe the original callback cause before the observed operation, then assert publication AtomicBool stays false for all four body/direction combinations. Mode 3 reopens sql_live after obtaining the expected original cause; mode 4 reopens native_live. Both flags are reopened before funded retirement. No obvious closure-lifetime obstacle was found in the authored FnOnce three-argument helper. This is source reasoning, not compiled acceptance.

The TypeScript guard requires the exact string `settlement.finish_body(owned)?;native.checkpoint()?;` and checks later status/publish ordering. It proves presence of this source pattern in the common macro, not execution, callback identity, custody, or heap receipts. Its first substring lookup is safe for the current one-macro definition but would not independently inspect multiple distinct implementations.

The cumulative law creates a real admitted Vec seed, independently charges SQL baseline, then checks subsequent SQL/native deltas relative to the retained baseline and unchanged seed pointer. It tests two validation hooks. It does not establish all four hook baselines merely through source presence.

## Copy-start-zero cancellation requirements

NativeDecodeControl copy_text and copy_text_into charge the full source extent before copy_checkpoint(0,len), then reserve backing. Cancellation at that zero checkpoint therefore legitimately reports full native admission with zero born String backing, zero copied prefix, and zero wallet text capacity. SQL settlement must preserve the native charge without inventing physical retained capacity or release.

A valid low-level witness must distinguish borrowed validation zero checkpoints from actual owned-copy zero checkpoints, target the paid charge increase (or a direct isolated copy call), assert source pointer and empty destination unchanged, and compare before/after cumulative native and SQL admission separately from allocator/wallet receipts. Interior-prefix cancellation is a different law: it must reach a later positive copy checkpoint with retained destination capacity and copied length. The existing EPW mode1/2 target>=512/completed>=256 body cancellation does not itself provide copy-start-zero proof.

## Saved checkpoint recheck

Common source now contains `settlement.finish_body(owned)?;native.checkpoint()?;` before status take and publish. EPW original decode/encode helper invocations inspected all pass three closure parameters (`sql,owner,_` or `sql,owner,body_completed`). Registered with_snapshot_file async closures correctly retain their separate two-parameter API (`sql,owner`). No additional concrete defect found in this recheck. Native run 10941 is pending externally; no native pass claimed.

## Next owner census: OS Collection

Authority root: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot`.

Full SQL schema has five tables with widths 3/6/8/3/5: document(schema,name); ordered folders(id,parent optional,name); ordered entries(id,folder optional,name,kind_id,body_kind); keyed document bodies(schema,addressed_document_id); keyed blob bodies(hash,size_high,size_low,media_type). Native u64 blob size is split into checked u32 words. Native bodies are boxed variants, unlike EPW text-only records.

Collection does not author either `to_sqlite_database_receiving` or `from_sqlite_database_receiving`; both inherit ArtifactSqliteSnapshotHost defaults from `🏪️store/🚪️io/🦀️.rs`, which route through sqlite_snapshot_workspace. Its two explicit validation hooks only call the original native checkpoint and then uncontrolled `validate_sqlite_snapshot_subset`. That validator reconstructs a local candidate through `from_sqlite_database`, compares full equality, then lets the candidate drop directly. It does not run the original receiving frame/wallet helper and does not settle a complete partial candidate into original physical custody. This is a concrete source gap for both validation directions under the all-four-hooks receiving goal.

Existing projection uses RowWriter; reconstruction uses fields::reconstruct, locally owned Strings/Vecs, a locally built ArtifactBody then Box allocation, and local diagnostic ValueError::new strings. A refusal midway through local construction can release prior local typed owners before returning. Existing TypeScript tests invoke authored SQL/preflight/semantic oracle contracts only. No collection-local receiving module or six-law original heap/cancellation fixture was found in the root census. These source findings do not claim default workspace runtime failure or runtime acceptance.
