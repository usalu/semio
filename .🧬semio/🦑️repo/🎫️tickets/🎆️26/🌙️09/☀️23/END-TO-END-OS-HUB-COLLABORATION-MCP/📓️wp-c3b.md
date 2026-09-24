# WP-C3b — Exact Self-Exclusion + Real 3D Presence

**Status:** complete (unit/typecheck green; collab-e2e harness extended, live run blocked by taxonomy)
**Ticket:** session 9 `.tmp-ticket` / C3 gaps + collab audit G4

## Design

1. Hosts subscribe to hub-admitted actor via `useLocalPresenceActorIdV1` and pass it as `myActor`; overlay returns null until actor is known (no `__local__` placeholder).
2. Presence view gains optional `rayOrigin` triple (schema-first). World space: `pointer` = ground/hit XYZ, `rayOrigin` = camera origin. Overlay projects hit through local orbit camera (`orbitPointToScreen`); HUD normalized fallback removed. Peer selection marks project onto `objectWorldPositions` when available.
3. Collab-e2e STEP 14 asserts `[data-peer-cursor]` / `[data-testid=peer-cursor]` markers and position change after peer pointer move (writer + draw + puzzle3d when kinds exist).

## Landed

- [x] actor id threading; `__local__` removed
- [x] `rayOrigin` schema + TS/Rust codecs + fixtures + wire bins
- [x] `orbitPointToScreen` + peer-overlay-v1 orbit cursorScreen
- [x] CanvasPresenceOverlayV1 3D markers + selection highlights
- [x] World3dHost publish hit+rayOrigin; localOrbit + objectWorldPositions
- [x] language-agnostic fixture + Rust/TS runners + renderer unit tests
- [x] collab-e2e STEP 14 assertions + selectors
- [ ] live two-browser collab-e2e evidence (blocked — see Gaps)

## Verified

| Check | Result | Capture |
|-------|--------|---------|
| `cargo test -p semio-framework-replication peer_overlay_v1_fixture` | PASS | `generated/c3b-cargo-peer-overlay.txt` |
| `cargo test -p semio-framework-replication presence_peer` | PASS 12 | `generated/c3b-cargo-presence-peer.txt` |
| `@semio-tech/framework-replication` test quick | PASS 12 | `generated/c3b-replication-ts-test.txt` |
| `@semio-tech/framework-renderer-react` test quick | PASS 5 | `generated/c3b-canvas-presence-test.txt` |
| `@semio-tech/framework-renderer-react` typecheck | PASS | `generated/c3b-renderer-typecheck.txt` |
| `cargo test -p semio-framework-os-kernel --features sync wire_fixtures_stay_byte_identical` | PASS | `generated/c3b-wire-fixtures-regen5.txt` |
| collab-e2e STEP 14 live | BLOCKED | `generated/c3b-collab-e2e.txt` |

## Gaps

- Full `verify collab` live run could not finish: (1) long plugin prebuild let hub bootstrap close (`UnsafeAuthConfiguration`); (2) subsequent starts hit concurrent taxonomy validation errors (`areas["…plugins"] may only be "exempt"…`). Harness STEP 14 + skip envs (`S_COLLAB_SKIP_PREBUILD`, `S_COLLAB_SKIP_ACTIVATE`, `S_COLLAB_OUT`) are in place for a clean re-run.
- Screenshots for STEP 14 not captured (run never reached scenario steps).

## Files

- MOD: replication `schema`, `wire` TS+RS, peer-overlay TS+RS+tests+fixture, presence-peer-codec + wire bin fixtures, artifact-bootstrap-protocol expects
- MOD: canvas-presence ts/tsx/tests; Board2dHost; World3dHost; TextEditor
- MOD: store sync unit `PresenceWindowView` literals; collab-e2e STEP 14 + outdir/prebuild skip envs

## Commands

```bash
cargo test -p semio-framework-replication peer_overlay_v1_fixture
cargo test -p semio-framework-replication presence_peer
cd …/replication/packages/typescript && bun ./script.ts test quick
cd …/renderer-react/packages/typescript && bun ./script.ts test quick && bun ./script.ts typecheck
S_COLLAB_HUB_PORT=7750 S_COLLAB_USER1_PORT=6250 S_COLLAB_USER2_PORT=6251 \
  S_COLLAB_OUT=.tmp-ticket/wp-c3b/generated S_COLLAB_SKIP_PREBUILD=1 S_COLLAB_SKIP_ACTIVATE=1 \
  S_COLLAB_HUB_BINARY=…/target-hc1-hub/debug/os-hub \
  bun ./script.ts verify collab   # from os-dev typescript package
```
