# WP-C3 In-Canvas Remote Presence

**Status:** complete (tests green)  
**Ticket:** session 9 `.tmp-ticket` / canonical `26/09/18` + collab audit G4  
**Contract:** PR1 peer wire + S12 generic presence; **ephemeral shared only** (never WAL).

## Design

1. **Wire:** optional `activeTool` as PresencePeer **bit 12** (distinct from `toolRun`). Pointer + viewport on `views[]`; selections on `interaction.domains`. Throttle publish at 50ms.
2. **Reduce:** language-agnostic `peer-overlay-v1` fixture maps peers to overlay rows (hub color, label, pointer in artifact space, selection ids, activeTool) filtered by `windowId`+`space`, excluding self.
3. **Publish:** ShellHost heartbeat stamps `views` from host registry + `activeTool` from local registry; `socket-actor` publishes hub-admitted actor for self-exclusion; presence events fan full wire peers into canvas-presence roster.
4. **Render:** domain-neutral `CanvasPresenceOverlayV1` (CSS `--presence-N`, en+de labels) in Board2dHost, World3dHost, TextEditor.
5. **Plugins:** writer (TextEditor), draw + puzzle 2d (Board2dHost), puzzle 3d/5d (World3dHost) — no parallel presence packs.

## Landed

- [x] activeTool bit 12 TS+Rust+schema JSON+codec fixture (`presence-peer-codec-v1`, unknown-flag moved to bit 13)
- [x] peer-overlay reduce (TS+Rust) + `peer-overlay-v1` fixture runners
- [x] canvas-presence registry + React overlay + unit tests (incl. self-actor exclusion)
- [x] ShellHost publish views/activeTool/roster/actor
- [x] Board2dHost / World3dHost / TextEditor overlays + throttled publish
- [x] Checks green (see Commands)

## Verified

| Check | Result | Capture |
|-------|--------|---------|
| `cargo test -p semio-framework-replication peer_overlay_v1_fixture` | 1 passed | `wp-c3/generated/c3-cargo-peer-overlay.txt` |
| `cargo test -p semio-framework-replication presence_peer` | 12 passed | `wp-c3/generated/c3-cargo-presence-peer.txt` |
| `cargo check -p semio-framework-replication` | ok | `wp-c3/generated/c3-cargo-check.txt` |
| `@semio-tech/framework-replication` vitest quick | 12 passed (2 files) | `wp-c3/generated/c3-replication-ts-test.txt` |
| canvas-presence unit (direct vitest) | 4 passed | `wp-c3/generated/c3-canvas-presence-test.txt` |
| `@semio-tech/framework-renderer-react` typecheck | exit 0 | `wp-c3/generated/c3-renderer-typecheck.txt` |

Runtime browser two-user cursor paint: not probed this slice (needs hub+two serves; G4 E2E remains session outcome).

## Files

- NEW: `framework/.../replication/peer-overlay/` (ts, rs, unit)
- NEW: `framework/.../replication/fixtures/peer-overlay-v1/json`
- NEW: `framework/.../renderer/.../canvas-presence/` (ts, tsx, unit)
- MOD: replication wire + presence-codec test; replication.ts exports; schema json; presence-peer-codec-v1; vitest config include
- MOD: ShellHost; Board2dHost; World3dHost; TextEditor
- MOD: packages/rust `peer_overlay` module path

## Gaps

- Hosts still pass `myActor="__local__"`; overlay resolves hub actor via `localPresenceActorV1` after `socket-actor`.
- World3d peer cursor uses normalized pointer HUD fallback (not full frustum paint).
- Draw presence pack unchanged (selection already on framework interaction broadcast).
- No wasm plugin rebuild required for this slice (hosts are TS React).
