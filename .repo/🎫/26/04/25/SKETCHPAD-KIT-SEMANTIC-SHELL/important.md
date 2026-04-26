# Semio Layer Parity (follow-up)

## Session notes

- `semio/js/index.ts`: `Matrix4` / `Vector3` from `three` at **top** of module (ESM). `importKit` JSDoc avoided `**/` (comment terminator). `applyKitDiff` type-updated id match uses explicit `tid` so `===` is not combined with `??` incorrectly. `executeSemioKitCommand`: `updateFolder` seeds `r`, `import` is no-op success after fetch (no invalid `addChild` Kit), returned shape guards `r`. Embed denylist: removed re-exported bridge helper names that are intentionally present (`selectBestRepresentation`, `findRepresentation`, `arePortsCompatible`).

- Validation: `npm test -w @semio/js` (139), `npm test -w @semio/react` (10), `cargo test --lib` in `semio/rs` (122), `npx tsc` js+react, `npm run build -w @semio/sketchpad` (0).

- Repo MCP `ticket_close` was not available in this environment; `ticket.json` updated manually.
