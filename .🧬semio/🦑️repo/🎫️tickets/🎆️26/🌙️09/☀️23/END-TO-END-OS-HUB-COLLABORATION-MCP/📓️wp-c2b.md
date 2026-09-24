# WP-C2b — Rust Sync Actor Root Fixes

Slice: C2b. Scratch: `.tmp-ticket/wp-c2b/`.

## Verdict

**PASS** — `cargo test -p semio-framework-os-kernel --lib --features sync,ureq sync` → **60 passed; 0 failed**. TS parity (7) + neighbour (9) green.

## Status

| Cluster | Status |
|---------|--------|
| A. actor_tests Session / mock hub protocol | **PASS** |
| B. ops text inverse record | **PASS** |
| C. fixtures path + remote ops | **PASS** |
| D. turn fault ownership leak | **PASS** |
| TS parity / neighbour | **PASS** |

## Fixes

### A — One document-WS protocol (`semio.session.v1`)
- Mock hub in `store/sync/tests/unit` negotiates `Sec-WebSocket-Protocol: semio.session.v1` (was hardcoding deleted `semio.socket.v1`).
- Session `actor` comes from `?actor=` (grant actor_id), not connection order — fixes B-before-A connect races.
- Ungated actor tests (`publish_preview`, `command_outcome`, `detach_drains`, `reconnect_since`) now `configure_mock_hub` + wait for Session (same as gated two-host tests).
- Aligns with C4b framework path `/scopes/{space}%2F{doc}/document/ws` + session credential.

### B — Ops text writer matches strict parser
- `store/rs` `print_edit_lines` now emits the full append unit: edit + inverse + per-forward metadata (what `replay_ops` requires).
- `print_ops_log` delegates to that unit (no duplicate inverse/meta).

### C — Language-agnostic fixtures + external-edit HLC
- Fixtures live at `os/.../store/sync/fixtures` (not missing `packages/rust/fixtures`).
- Unit resolves `CARGO_MANIFEST_DIR/../../modules/store/sync/fixtures`.
- Handcrafted `basic-remote-ops` + `remote-ops-backlog` (`fixture.dsl` + `edits.ops`); expected ids use `edit#0` mutation form.
- Append-only folder remotes without HistoryOpMeta HLT were stamped `(0,0)` and reordered under local edits → snapshot stayed at old head. Actor now advances HLC before deliver.

### D — TurnFault ownership
- On turn panic, retain future in `terminal_turn` and **do not** `enqueue` a drain job (shut-down pool minted a spurious `Pool(Shutdown)` `terminal_job` that stole the first `close_one` grant).

## Evidence

```text
CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-kernel --lib --features sync,ureq sync
→ test result: ok. 60 passed; 0 failed; … finished in 0.55s
  log: .tmp-ticket/wp-c2b/generated/rust-sync-full-3.txt

bun … script.ts test long -t "backbone parity"
→ 7 passed  log: generated/ts-parity.txt

bun … script.ts test long -t "handleHubFrame|rebootstrap|outbox|socket actor"
→ 9 passed  log: generated/ts-neighbour.txt
```

## Files changed

- `os/.../store/sync/rs` — TurnFault enqueue; external-edit HLC stamp; append_ops doc
- `os/.../store/rs` — `print_edit_lines` / `print_ops_log`
- `os/.../store/sync/tests/unit/rs` — mock hub session protocol + actor query; configure_mock_hub on ungated tests; fixtures path; ParsedDocumentText retirement; dsl mirror path
- `os/.../store/sync/fixtures/` — `basic-remote-ops`, `remote-ops-backlog`

## Honest gaps

- Did not re-run hub suite (C4c owns that). Did not touch O3b policy work.
- `snapshot-replaced` actor fixture not ported (JSON→dsl); two external-edit fixtures cover the shared harness.
