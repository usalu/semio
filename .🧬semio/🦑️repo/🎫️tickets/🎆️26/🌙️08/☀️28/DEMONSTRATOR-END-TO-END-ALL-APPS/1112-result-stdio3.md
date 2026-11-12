# 1112 stdio3
Scope: stdio txt-schema, deflate-examples, semio v1 snapshots brep/drawing/mesh/model/value (hub wasm).
## State
- deflate-examples: W-stdio3-1 rc 0. Publication::close_step is now effect-shaped (`&mut RetainedAllocationEffect`, returns RetainedCloneStep, work u128); zlib callers (tuned-encode, publication-and-job-steps) use close_original_effect_step. Debt: 🚪️io/📤️publication/🧪️tests close() helper still passes a grant.
- semio v1 crates: all 5 errors came from `semio_snapshot_retirement!` in semio-base (admit_shared_retirement removed) -> macro now delegates to framework `SharedValueRetirementFactory<T>` (same impl as flow's FlowSnapshotRetirementFactory). value also: projection tree `reserve_one(remaining)` -> `reserve_one_cold(remaining)` (caller already charged page bytes via control.admit_allocation_bytes).
- txt-schema: sqlite snapshot ported to stdiosnap pattern (decode/encode_sqlite_snapshot_native with output slot + TypedFileSettlement; receive_pack(bytes,output,owner)); hand-written receiving code (🫴️receiving/🦀️.rs + its tests) removed; `bind_pack_rows/bind_text` (TextReceiving tuple) replace the owner-taking receive_*.
## Rerun
`chk.sh W-stdio3-<n> 🌎️hub/Cargo.toml wasm -p semio-s-artifact-stdio-txt-schema -p ...semio-{brep,drawing,mesh,model}-snapshot -p semio-s-artifact-stdio-semio-value-model`
## Debts
- 🫴️receiving tests of txt deleted (tested removed API); semio/🧪️tests/🔬️unit and deflate publication tests target old APIs.
## Verified
- W-stdio3-1 (deflate-examples) rc 0; W-stdio3-4 (txt-schema + semio brep/drawing/mesh/model/value snapshots) rc 0 (hub wasm).
- W-stdio3-5 `-p semio-hub-puzzle` rc 101: none of my crates remain; remaining reds: stdio deflate-snapshot (34), xml-snapshot (6), json-schema (120), dxf-snapshot (4), stl-inferences (28), obj-inferences (24), semio-mesh-editor (1), semio-brep-io (1), playbook-playbook (7).
- semio-mesh-editor / semio-brep-io: only error is removed macro retained_command::terminal_frame_bound! (factorysweep/puzzlefill pattern). Not mine.
STATUS: FIXED-verified W-stdio3-4
