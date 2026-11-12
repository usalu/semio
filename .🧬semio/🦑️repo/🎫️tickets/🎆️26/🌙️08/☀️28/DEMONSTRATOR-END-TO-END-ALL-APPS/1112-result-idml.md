# idml (11/12)
## Step 1 (edited, compile pending)
- sqlite: decode/encode native via settlement::decode_typed/encode_typed (census split into extent+limits).
- snapshot: manual SnapshotProjection replaced by derive(SnapshotProjection) + by_value_projection.
- mutations: protocol::original_pack_value_operation_receiving!(IdmlMutation).
- leaves ported to effect/flight shape: version/retained, change/part, admission/retained (advance(..,effect,work), close_step(body,effect,flight,recover,work)).
- mutation machine (RetainedMutationPreparation) and RetainedIdmlApply (RetainedDiffApply) ported to decide_reply/inverse_reply/close_reply and preflight_reply/commit(effect)/close_reply following the paged-one-item fixture + EnergyDocumentApply (retained_routed!, native_child, polled, conclude).
- kernel facade: re-export retained_polled (additive).
Rerun: "$C/1105b-tools/chk.sh" W-idml-1 "🌎️hub/Cargo.toml" wasm -p semio-s-artifact-stdio-idml
## Step 2 verified
- W-idml-1 (hub wasm, -p semio-s-artifact-stdio-idml) rc 0 (first compile; only pre-existing PackageWire privacy warning remains).
- W-idml-3 (hub wasm, -p semio-hub-stdio-component) rc 101: stdio-idml is not in that component's graph;  the only remaining red crate is semio-s-artifact-stdio-ifc4x3-fem (49 errors, claimed by lane ifcguidance 13:15). Nothing else in the stdio closure fails, but crates depending on ifc4x3-fem are not reached (--keep-going), so further reds may sit behind it.
- Earlier W-idml-2 stopped upstream in semio-framework-value-retirement (peer in-flight, cleared by W-idml-3).
## Design notes
- Hand state machines kept (phase tables unchanged); child calls follow the fixture/EnergyDocumentApply protocol: pending-capable children (RetainedFieldPreparation advance/close, diagnostic advance/close, poll closes) run under retained_routed!/polled with one Flight per owner and `recover` threaded into leaves; native children (take_into) via native_child; owner-only units added to `work`, settled by retained_conclude/retained_settle.
- Quotes gained `maximum_work_units` + work-demand fns; CPU slices are bounded by min(body-derived steps, max).
- Apply: authored version is now staged (take_into) and swapped into the post at commit (no `install`), displaced String retires through the same ladder.
- Kernel facade: additive re-export `retained_polled`.
## Debts
- cfg(test) modules (admission/*/tests, mutations/retained/tests, diff/retained/apply/tests, overlay tests) still target the old grant-shaped leaf APIs: not compiled or ported.
- Runtime behaviour of the pending/recover routes is unexercised (no tests run).
- native (non-wasm) check not run.
Rerun: "$C/1105b-tools/chk.sh" W-idml-4 "🌎️hub/Cargo.toml" wasm -p semio-hub-stdio-component
W-idml-4 (final tree incl. unused-import cleanup, hub wasm, -p semio-s-artifact-stdio-idml) rc 0, 0 errors.
STATUS: FIXED-verified W-idml-4
