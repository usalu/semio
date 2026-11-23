# React lane for layout (opus1122) - status 2026-11-22
Host load ~56, lease contention; builds slow.
- Owners (renderer): framework-surface-rs, framework-editor-rs, framework-os-node-graph-rs (`:wasm`).
- Scripts: opus1122-react-run.sh (native runner wrapper), opus1122-react-chain.sh (3 wasm), opus1122-react-chain2.sh (retry surface, fonts, prepare/activate/serve layout react prototype).
- surface wasm attempt 1 exited 124 (dev-layout-native no progress 10 min at boot.loaded, starved by load); chain2 retries.
- Logs: $T/🗑️generated/opus1122/react2-*.log. Smoke test (Playwright) not yet run.
