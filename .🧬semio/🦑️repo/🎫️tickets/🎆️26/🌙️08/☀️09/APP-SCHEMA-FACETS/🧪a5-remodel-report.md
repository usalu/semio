# 🧪 A5 — remodel (`semio-s-plugin-remodel`)

## Summary

Wave A5 app schema facets for **RemodelConfig** / **RemodelPresence** (owner `📸️remodel/🎚️config`). Added config `🧬️schema` five-leaf facet mirroring nested runtime config (`camera`, `selection`, `layers`, `frameCursor`, scalars). Added `👥️presence` runtime (`RemodelPresence` + `RemodelPresenceMutation` Snapshot pattern) and presence schema with shared-ui fields: selection, orbit camera, frame cursor, active utility, report table. Wired `📦️glue.rs` (`config` / `presence` + `schema` submodules) and `RemodelPlayApp` `Presence` / `PresenceMutation` types.

## Files touched

**Plugin (`✏️s/🔌️plugins/📸️remodel/`)**

- `🎛️apps/📸️remodel/🎚️config/🧬️schema/` — `🦀️component.rs`, `🟦️component.ts`, `🔗️component.graphql`, `🔣️component.json`, `🛰️component.proto`
- `🎛️apps/📸️remodel/👥️presence/🦀️component.rs` (new)
- `🎛️apps/📸️remodel/👥️presence/🧬️schema/` — five leaves (new)
- `🎛️apps/📸️remodel/🦀️component.rs` — `Presence` / `PresenceMutation` bindings
- `📦️packages/🦀️rust/📦️glue.rs` — nest config/presence + schema modules

**Ticket**

- `gen_remodel_a5.py` — generator used for leaves (kept in ticket folder)
- `🧪a5-remodel-report.md` — this report

## Gate tails

### Scoped `policyAppSchemaBreaches` (remodel)

```
0
```

### `cargo check -p semio-s-plugin-remodel`

```
    Finished `dev` profile [unoptimized] target(s) in 3.17s
```

### `cargo test -p semio-s-plugin-remodel --lib`

```
test result: ok. 376 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 54.09s
```

## Unverified

- End-to-end hub/renderer encoding of `RemodelPresence` into `presence_pack` (kernel/framework integration is outside this plugin-only wave).
- Live sync from local `RemodelConfig` edits into typed presence broadcasts (presence types exist; command wiring not added in A5).
