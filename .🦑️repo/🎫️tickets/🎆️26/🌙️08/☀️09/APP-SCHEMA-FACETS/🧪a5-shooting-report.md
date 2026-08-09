# A5 — Shooting (`semio-s-plugin-shooting`)

## Summary

Wave A5 app-schema facets for the shooting play app owner `🎚️config` + sibling `👥️presence`, following the lowpoly pilot (§13). `ShootingConfig` schema leaves mirror all thirteen fields on the runtime `ShootingConfig` struct (`local-ui`). `ShootingPresence` carries shareable live state (selection, hover, viewport camera, active utility) with Snapshot-only mutations and matching `shared-ui` schema leaves. `📦️glue.rs` nests `config { component; schema }` and `presence { component; schema }`. `ShootingPlayApp` wires `ShootingPresence` / `ShootingPresenceMutation` instead of `NoPresence`.

## Files touched

- `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}` (new)
- `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/👥️presence/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/👥️presence/🧬️schema/{five leaves}` (new)
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/📦️glue.rs` (config/presence mount)
- `✏️s/🔌️plugins/🎥️shooting/🎛️apps/🎥️shooting/🦀️component.rs` (DocumentApp presence types)

## Gate tails

### Scoped `policyAppSchemaBreaches` (shooting)

```
0
```

### `cargo check -p semio-s-plugin-shooting`

```
Finished `dev` profile [unoptimized] target(s) in 1m 03s
```

(7 pre-existing lib warnings in crate; exit 0.)

### `cargo test -p semio-s-plugin-shooting --lib`

```
test result: ok. 95 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

## Unverified

- Runtime presence sync from `ShootingConfig` into `ShootingPresence` on the host/session path was not exercised in this wave (schema + wiring only).
- Collaborative presence DSL/pack round-trip tests were not added (lowpoly pilot pattern; existing config tests unchanged).
