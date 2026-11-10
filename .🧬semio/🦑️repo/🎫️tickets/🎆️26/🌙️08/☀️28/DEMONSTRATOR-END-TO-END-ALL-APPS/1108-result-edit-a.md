# edit-a result
Lib: named/📇️index/🦀️.rs and ♻️transfer/🦀️.rs ported (reply-shaped close with close_route recovery; NamedOwner/NamedRetirement relay; named_close! macro; settle_reply). Transfer source admission now drives the owned projection (project_owned at phase projectionStart, quote owned_projection_demand): this fixes "original projection must be driven or returned through its resident frame".
Tests (new layout): named_index (index.rs + actual.rs) and named_transfer (transfer.rs) with shared helpers.rs; old in-file test mods removed. Advance/close run inside actual original invocations (turn_ref, outcome_turn) because preparation/transfer advance close nested sealed children.
Counts: named_index 3/3 passed (W-edit-a-21, 176 s); named_transfer 2/2 passed (W-edit-a-24, rc 0). Combined reruns W-edit-a-25/26 were blocked by a peer's mid-edit native-owned compile error (RetainedCursorRelease), not by value-edit.
Exact-law notes: close bodies for NamedKeyIndexPreparation and NamedEditPlan are 1<<16 (733 quotes ~8.8 KB at body 4096); transfer test no longer continues into NamedPersistentRoot admission (edit-b root tests cover it).
Rerun: tst.sh W-edit-a-<n> 🧰️framework/Cargo.toml native -p semio-framework-value-edit --test named_index --test named_transfer
STATUS: FIXED-verified W-edit-a-24 (transfer) and W-edit-a-21 (index)
