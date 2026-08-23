# P6g Mounted Fem2d Operation Session Census — 2026-08-23

## Scope

This is the required pre-edit production caller and owner census for the first mounted Phase 6
packet. The packet is limited to the 2D FEM application, the shared render authority needed to bind
that application to an actual instance/revision/generation, and permanent source verification. It
does not claim or implement the P6h numerical micro-cursors or P6i cursorized visual encoder.

## Production reachability before P6g

| Authority | Live production construction | Existing non-live construction |
| --- | ---: | --- |
| `FemJobGraph::new` | 0 | tests only |
| `MeshJob::new` | 0 | tests only |
| `AssemblyJob::new` | 1 batch adapter | tests plus synchronous `assemble_system` |
| `PcgJob::new` | 1 batch adapter | tests plus synchronous `pcg` |
| `Fem2dLiveVisual` consumed by `render_with_progress` | 0 | tests only |

The sole production 2D renderer call is
`model_window::render -> render_with_progress(doc, camera, None)`. The static editor receives only a
borrowed snapshot and therefore has no app-instance, base-revision, generation, cancellation, or
owned snapshot authority with which to mount a retained session.

## Existing owners and missing handoffs

- `VcsArtifactApp` owns the canonical snapshot `Arc`, content revision, app instance identity, and
  store generation, but its live `ArtifactView` previously exposed none of that authority.
- `FemJobGraph`, `MeshJob`, `AssemblyJob`, and `PcgJob` retain their own progress state, but no live
  2D app session owns or schedules them.
- `Fem2dLiveVisual` exists and the model renderer accepts it, but production always supplies `None`.
- Phase 2's fixed progress/preview overlay is fed by the live WGPU `ShardOutcome::Job` path. P6g must
  reuse that authority rather than create a second UI or worker transport.

## Required post-cut census

P6g must establish exactly one live 2D mount for each existing job family, one process-worker
opportunity per pump, revision/generation restart and commit validation, retained terminal cleanup,
and a live `Some(&Fem2dLiveVisual)` renderer consumer. All underlying model-sized constructor and
numerical loops remain explicit Phase 6h/6i runtime gates.

## Baseline verdict

**RED.** The useful FEM job graph is not reachable from the live application before this packet.
