# PORT PATTERN (green: binary-snapshot, W-stdiosnap-1 rc 0) - reference: 
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/{🪶️sqlite/📸️snapshot/🦀️.rs, 🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs, 💾️binary/📸️snapshot/🦀️.rs}`
1. `impl ArtifactSqliteSnapshotHost`: DELETE `to_sqlite_database_receiving`, `from_sqlite_database_receiving`, `validate_sqlite_snapshot_subset_decoding/_encoding` (trait defaults now run the authored `to_/from_sqlite_database` + `validate_sqlite_snapshot_subset` under the original owner). Delete the crate's hand-written 🫴️receiving project/reconstruct/validate code (keep only carrier helpers still used).
2. Native signatures gain an out-slot and return the stage outcome:
   before: `fn decode_sqlite_snapshot_native(payload,control,owner)->RetainedWorkOutcome<Self,NativeSnapshotFailure>`
   after:  `fn decode_sqlite_snapshot_native(payload,control,owner,output:&mut Option<store::os_store::serialization::SnapshotReceivingOutput<Self>>)->store::os_store::serialization::SnapshotReceivingOutcome`
   same for `encode_sqlite_snapshot_native(&self,encoding,control,owner,output:&mut Option<SnapshotReceivingOutput<store::io_schema::IoPayload>>)`.
   Record-based owners (DSL record): just call `store::decode_sqlite_snapshot_record_native(payload,Self::__DSL_ENVELOPE_ID,spec,&mut |record,snapshot_output,native|{..;*snapshot_output=Some(constructed?);Ok(())},control,owner,output)` / `store::encode_sqlite_snapshot_record_native(encoding,id,spec,&mut |native|..,control,owner,output)` (closures passed by `&mut`; pre-checks that fail return `RetainedWorkOutcome{result:Err(store::sqlite_snapshot::SqliteStageFailure::NotStarted(e)),work:0}`).
   Hand-written bodies: see `🚦️native/🦀️.rs` decode/encode: preflight -> `TypedFileSettlement::new(control,before,Phase)` (`store::space_history::io::sqlite::snapshot::settlement`) -> `owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<T,SnapshotReceivingOutput<O>,NativeSnapshotFailure>(output,&mut |slot,published,_m,_u,native,body|{..; settlement.finish_body(native.owned_bytes()); *published=Some(..)}))`.
3. `ArtifactPackReceiving::receive_pack(bytes,output:&mut Option<Self>,owner)->RetainedWorkOutcome<(),NativeSnapshotFailure>`: `owner.receive::<Self,Self,F>(output,&mut |slot,published,_m,_u,native,body|{..bind into slot..;*published=slot.take();..})`.
4. `NativeSnapshotFailure::into_value_error` is gone; `allocation_stage_native` no longer used by ported crates.
Test debt: 🫴️receiving/🧪️tests (cfg(test)) still target the removed receiving API.

# 1112 stdiosnap
Scope (coordinator): binary-snapshot, semio-base, step-snapshot, dwg; mp4-inferences/avi-snapshot skipped (debt) unless stdio core depends. Final target: semio-hub-stdio-component (hub manifest 🌎️hub/🧩️compositions/🗄️stdio/🧩️component/📦️packages/🦀️rust/Cargo.toml).
Recipe: drop to/from_sqlite_database_receiving + validate_*_decoding/encoding overrides (trait defaults run authored backing); decode/encode native take output slot (TypedFileSettlement + owner.receive, see binary 🚦️native); receive_pack(bytes,output,owner).
## Done
- binary-snapshot: W-stdiosnap-1 rc 0 (wasm hub). Debt: 🫴️receiving/🧪️tests still test old removed receiving API (cfg(test) only).
## Verified
- W-stdiosnap-7 (hub wasm, `-p` semio-base, step-snapshot, dwg, binary-mutations, binary-snapshot): rc 0.
- Edits: binary (snapshot host/native/receiving/receive_pack, mutations `dsl::original_pack_value_operation_receiving!`), dwg mutations (same), step (host, native encode/decode/print via record helpers, `close`=drop, reconstruction forwarded allocation), semio-base (`decode_receiving`, out-slot `encode`, Workspace::close_step), 18 semio subsets (native decode/encode + host wrappers; text receiving module removed), `plugin_app_close_prelude` += `semio_plugin_document_media`.
## Not verified / open
- Component target `semio-hub-stdio-component` (W-stdiosnap-5, rc 101): remaining red is outside my crates: txt/tsv/html/las/jpg/deflate/bmp/gif/wav/mp4/avi/mp3/epw/ifc (hubrest + unassigned), `TxtMutation: OpBinaryReceiving` (147 errors; same `dsl::original_pack_value_operation_receiving!` recipe), semio value projection tree `reserve_one_cold` (peer edited).
- The 18 semio subset crates were compiled in W-stdiosnap-5 only with the (since peer-replaced) retirement macro error; not re-verified green on their own.
- Debt: `Owned<T>` Drop and step `close` now plain drop (no cold retirement API exists); cfg(test) tests of removed receiving code (binary 🫴️receiving/🧪️tests, others) not ported; mp4-inferences/avi-snapshot left to hubrest.
STATUS: FIXED-verified W-stdiosnap-7
