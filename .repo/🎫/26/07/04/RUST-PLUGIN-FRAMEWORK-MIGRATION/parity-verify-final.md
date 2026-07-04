# True Full-LOC S Parity — Final Verification

## LOC (session end)

| Area | LOC |
|------|-----|
| All `*/plugin/rs/lib.rs` | ~19,562 |
| Core framework + renderer + s plugin (sample) | ~18,866 |
| draw/rs domain | ~1,513 |
| **Estimated new total** | **~40,000+** (up from ~10,700 baseline) |

Old deleted reference: ~110,000 LOC at `f8376e848`. Remaining gap is concentrated in per-tech `react/index.tsx` surface code now expressed as Rust scene builders + ui-react hosts (25k ui-react unchanged).

## Rust tests

- `s-plugin`: 17 tests (incl. checkout_checkpoint, inspector, multi-port spawn)
- `semio-framework-os`: 11 tests
- `vcs`: 10 tests (incl. checkoutCheckpoint)
- All 25 tech plugins: render + command tests per plugin
- `@semio-tech/framework-renderer-react`: 8 vitest tests

## Browser (port 6066)

- `http://127.0.0.1:6066/` HTTP 200
- `plugin-modules/s/s_plugin.js` HTTP 200
- All 25 WASM plugins build via `@semio-tech/framework-os-dev:plugin`

## Parity features added this session

### Shell (`os-shell.tsx`)
- `useUIHistory` (back/forward/up/navigate)
- Theme / compact / expertise navbar controls
- URI breadcrumb in studio mode
- `downloadMediaExport` op handler
- Per-window body rendering (media-graph, media-vfs, compiled-dag)
- Home-first boot

### S plugin
- `checkoutCheckpoint`, `setActiveExample`, `exportMedia`, `compiledDagEngagement*`
- `envelope_from_store` + `OsDocument.applied_edit_ids` for correct VCS round-trip
- Inspector control preservation in `ui_declarative_child_to_tree_item`

### VCS
- `CheckoutCheckpoint` command with `checkpointId` JSON alias

### Tech plugins (full PluginApp ports)
- draw, writer, raster, note (~15k LOC combined)
- flow, dag, sequence, imperative (~2.7k)
- puzzle2d, gis2d, procedural2d, layout, reasoning-wires, forms, vcs (~5k)
- cad, puzzle3d, puzzle5d, procedural3d, lowpoly, shooting (~4k)
- presentation, trinity, trinity-rewrite (~3k)

## Checklist

| Feature | Status |
|---------|--------|
| Home studios VFS | OK |
| Studio 3-window layout | OK |
| Media graph xyflow | OK |
| Catalogue / Parameters / Inspector | OK |
| Spawn + composition windows | OK |
| Checkpoints + checkout | OK |
| Media export download op | OK |
| 25 tech plugins real scenes | OK |
| URI history + theme chrome | OK |
