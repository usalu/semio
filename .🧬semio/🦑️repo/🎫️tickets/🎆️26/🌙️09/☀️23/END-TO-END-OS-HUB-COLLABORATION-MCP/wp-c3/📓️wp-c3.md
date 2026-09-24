# WP-C3 In-Canvas Remote Presence

**Status:** PASS (unit/oracle green; live dual-browser not re-probed)
**Gap:** G4 — remote cursors / in-canvas presence
**Contract base:** existing PresenceWindowView / views / pointer / PresenceInteraction (+ optional activeTool bit 12). Ephemeral shared only — never persisted. S12 one framework presence root; no per-editor presence packs.

## Design

1. **Wire (existing):** peer identity hub-admitted; apps own views[] (canvas/orbit/geo + size + pointer), interaction.domains (selected/hovered), optional activeTool.
2. **Reduce:** peersForWindow / peers_for_window filters remote peers to matching windowId+space, folds marks only for matched peers, projects canvas pointer to screen and peer viewport rect; orbit frustum helpers for world space. Language-agnostic fixture peer-overlay-v1.
3. **Publish:** hosts throttle (~50ms) publishLocalPresenceWindowViewV1 (+ publishLocalActiveToolV1); ShellHost heartbeat stamps collected views/activeTool onto outbound presence; publishArtifactPresenceRosterV1 mirrors verified roster to hosts (runtimeKey local).
4. **Render:** domain-neutral CanvasPresenceOverlayV1 (CSS --presence-0..11, en+de labels) mounted in Board2dHost, World3dHost, TextEditor — covers writer / draw / puzzle 3d+5d via shared hosts.
5. **Plugins:** no parallel presence packs.

## Landed

- [x] Overlay derivation TS+Rust + fixture (windowId+space filter; geometry center-based)
- [x] Optional activeTool bit 12 on PresencePeer (schema + codec oracle)
- [x] canvas-presence registry + React overlay + unit tests
- [x] ShellHost heartbeat views/roster fan-in
- [x] Board2dHost / World3dHost / TextEditor publish + overlay
- [x] Vitest peer-overlay (12) + rust peer_overlay_v1_fixture (1) + canvas-presence (3) + presence-peer-codec oracle + scoped-presence (10)

## Files

- NEW framework replication peer-overlay (ts, rs, unit tests)
- NEW replication fixtures peer-overlay-v1 json
- NEW renderer elements canvas-presence (ts, tsx, unit tests)
- MOD replication wire/schema/exports + presence-peer-codec fixture; vitest include for peer-overlay
- MOD ShellHost, Board2dHost, World3dHost, TextEditor

## Commands (pass/fail)

| check | result | evidence |
|---|---|---|
| framework-replication test | PASS 12/12 | wp-c3/generated/peer-overlay-vitest.txt |
| cargo test --lib peer_overlay | PASS 1/1 | wp-c3/generated/c3-cargo-peer-overlay.txt |
| canvas-presence vitest unit | PASS 3/3 | wp-c3/generated/canvas-presence-vitest.txt |
| presence-peer-codec-check --oracle-only | PASS 32 vectors / 25 hostile | wp-c3/generated/presence-peer-codec-check.txt |
| scoped-presence-check | PASS 10 | wp-c3/generated/scoped-presence-check.txt |

## Honest gaps

- No fresh dual-browser in-canvas screenshot this slice.
- TextEditor publishes constant activeTool edit.
- Selections ride interaction via ephemeral snapshot; no separate pushPresence guest path.
