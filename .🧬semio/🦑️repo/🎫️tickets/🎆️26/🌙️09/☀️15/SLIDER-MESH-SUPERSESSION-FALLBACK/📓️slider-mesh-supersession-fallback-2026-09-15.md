# Slider Move Must Not Drop Tessellated Solids — lane `slider-mesh-supersession-fallback` (2026-09-15)

Symptom: after moving a slider (e.g. hex column height to 4 m), the **extruded solid mesh disappears** while inline **vector** and **wire/polygon** markers stay visible.

## Root cause

When a dirty branch re-evaluates, the brep **handle for `extrude@solid` changes immediately** in the live/painted evaluation. That is correct — the painted merge only fills **unanswered** nodes, not nodes the walk already answered with a new handle.

Two paths then dropped the solid:

1. **`retain_preview_meshes`** kept only handles named in the painted eval. The new handle had no pack yet; the **old handle’s pack was retired**, so nothing could be painted for the solid channel.
2. **`preview_payload` / viewer mesh table** looked up mesh data only by the **new** handle, so the payload omitted the solid even though the session still held (briefly) the previous tessellation.

Inline vector/wire geometry is built without kernel handles, so it kept showing — exactly the reported split.

Related prior work: `📓️slider-latency-incremental-eval-2026-09-15.md` §3.2–3.3 (unanswered-node merge + retain against painted eval). This gap is the **superseded-handle** case: answered with a **new** handle before tessellation lands.

## Fix

- `FlowEvalSession::converged_eval_json()` — read the last converged walk.
- `preview_mesh_retention_handles` — union converged handles for channel leaves whose painted handle has no pack yet.
- `mesh_data_for_session_preview_channel` — paint the converged pack while the new handle is still tessellating.
- `preview_mesh_pack_fingerprint` — viewer retained mesh table invalidates when fallback packs change.
- Editor `preview_payload` and viewer `build_preview_mesh_table` use the session channel lookup.

## Laws

- `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test incremental-eval` — 6/6 green (existing painted-eval + tessellation cache laws).

## Verify in UI

Restage procedural guest, open hexagonal mushroom column, move **Column Height** to 4 m: solid should **stay visible** (possibly stale for a moment) then update when tessellation completes.
