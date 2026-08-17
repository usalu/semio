# A5 — Raster (`semio-s-plugin-raster`)

## Summary

Wave A5 for owner `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🎚️config`: shipped full `RasterConfig` config facet (five schema leaves, `local-ui`, fields match runtime `RasterConfig` including nested `RasterCamera` and `RasterConfigViewportSize`). Added sibling `👥️presence` with `RasterPresence` / `RasterPresenceMutation` (shareable selection, hover, brush, camera, active utility) plus five presence schema leaves (`shared-ui`). Wired `📦️glue.rs` config/presence nests and set `RasterPlayApp` `Presence` / `PresenceMutation` to the real types.

## Files touched

### Created

- `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/👥️presence/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`

### Updated

- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🦀️component.rs`

## Gate tails

### Scoped `policyAppSchemaBreaches` (raster)

```
0
```

### `cargo check -p semio-s-plugin-raster`

```
warning: `semio-s-plugin-raster` (lib) generated 5 warnings (run `cargo fix --lib -p semio-s-plugin-raster` to apply 4 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 3m 09s
```

### `cargo test -p semio-s-plugin-raster --lib`

```
test result: ok. 55 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s
```

## Unverified

- Runtime hub `presence_pack` encode/decode with live peers (kernel A3 path; no multi-peer raster session exercised here).
- A6 catalog registration for this owner (deferred to wave A6).
