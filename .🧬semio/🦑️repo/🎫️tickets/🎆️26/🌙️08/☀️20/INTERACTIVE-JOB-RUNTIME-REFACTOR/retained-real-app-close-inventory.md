# Real App Close Authority Inventory

## Scope

Read-only source inspection after the synthetic latest-wins four-test native pass. No real-app close pass is claimed.

| App | Draft | Presence | Transient | Missing explicit close hooks |
| --- | --- | --- | --- | --- |
| CAD | NoDraft | CadPresence | NoTransient | Draft owners/disposer, Presence disposer, Transient disposer |
| Flow | NoDraft | FlowPresence | NoTransient | Draft owners/disposer, Presence disposer, Transient disposer |
| Procedural2d | NoDraft | Procedural2dPresence | NoTransient | Draft owners/disposer, Presence disposer, Transient disposer |
| Procedural3d | NoDraft | Procedural3dPresence | NoTransient | Draft owners/disposer, Presence disposer, Transient disposer |

All four editor implementations currently omit these hooks. The new default-None draft owner hook therefore leaves them explicitly unadmitted; it does not implicitly grant a generic cleanup implementation.

CAD's real constructor smoke test calls app.close_step under the production grant. After successful factory activation it will reach the missing draft-disposer fault unless this adoption happens before the next test snapshot. TestApp's zero-sized Presence fixture disposer cannot be used for CAD or the other apps: their real presence types own variable strings, and their peer rosters can be populated.

## Domain Evidence

CadPresence has active_utility_id, engagement_step, and optional engagement_pane strings in addition to fixed camera scalars. Procedural2dPresence has show_mode and optional selected_generation_id strings. Procedural3dPresence has active_utility_id and show_mode strings alongside camera data. Their final local and peer roots require exact domain byte retirement; assuming default or empty peer presence would only hide the missing lifecycle.

## Required Safe Adoption

NoDraft can explicitly reuse the existing bounded document cursor owner catalog because its concrete snapshot and mutation payloads are zero-state. NoTransient needs a concrete empty-type disposer, not a generic default. Real Presence needs a retained Store-owned handoff that empties the local/peer slots and returns each root to the app's typed retirement factory, with a final terminal witness. A captured peer root must retain the old values until its own release; shutdown cannot recursively drop a populated peer array or replace it with a fresh default while claiming empty ownership.

The plugin source is currently held for ten native lifecycle tests. These real-app changes remain a subsequent scoped packet, coordinated with the Flow/Procedural owners.
