# A5 GIS Report — App Schema Facets

Ticket: `26/08/09/APP-SCHEMA-FACETS`  
Plugin: `🌍️gis` (`semio-s-plugin-gis`)

## Summary

Shipped config + presence schema facets for both GIS owners (`Gis2dConfig` / `Gis2dPresence`, `Gis3dConfig` / `Gis3dPresence`), wired nested `config { component; schema }` and `presence { component; schema }` in `📦️glue.rs`, and replaced temporary `NoPresence` bindings on both DocumentApps with the real presence types.

Config facets mirror the existing Rust config structs exactly (`local-ui`). Presence facets carry the shareable live subset: 2d = selection / camera / feature selection / hover / selection method+mode; 3d = camera / pin selection (`shared-ui`).

## Files touched

### ◻2d owner
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/👥️presence/🧬️schema/{five leaves}`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🦀️component.rs` — `Presence` / `PresenceMutation` bindings

### 🧊️3d owner
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🎚️config/🧬️schema/{five leaves}`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/👥️presence/🧬️schema/{five leaves}`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🦀️component.rs` — `Presence` / `PresenceMutation` bindings

### Wiring
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📦️glue.rs`

### Ticket logs
- `🧪a5-gis-cargo-check.log`
- `🧪a5-gis-cargo-test.log`
- `🧪a5-gis-policy.log`
- `🧪a5-gis-report.md` (this file)

## Gate tails

### 1. `cargo check -p semio-s-plugin-gis`
```
Finished `dev` profile [unoptimized] target(s) in 2m 20s
```
Exit 0 (pre-existing warnings only; no errors).

### 2. `cargo test -p semio-s-plugin-gis --lib`
```
test result: ok. 144 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```
Exit 0.

### 3. Scoped `policyAppSchemaBreaches` (gis)
```
0
```
Exit 0 — zero scoped breaches.

## Unverified

- Runtime peer presence round-trip over the hub (encode/decode of `Gis2dPresence` / `Gis3dPresence` packs in a live multiplayer session) was not exercised beyond DocumentDsl/Pack compile + lib tests.
- TypeScript package index was not extended with config/presence schema re-exports (lowpoly pilot also left TS index artifact-only; policy does not require it).
