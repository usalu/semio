# 🧪 A5 Imperative Report — App Schema Facets

Ticket `26/08/09/APP-SCHEMA-FACETS`, wave A5, plugin `📜️imperative`.

## Summary

Delivered app schema facets for owner `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🎚️config`: five config schema leaves mirroring `ImperativeConfig` (`selectedStepIds`, `runOutputJson`, `locale`, `contributionsJson`, all `local-ui`); sibling `👥️presence` runtime (`ImperativePresence` / `ImperativePresenceMutation`) plus five presence schema leaves with peer `selectedStepIds` as `shared-ui` (live step selection on the script surface). Wired `📦️glue.rs` with nested `config { component; schema }` and `presence { component; schema }` per lowpoly pilot. `ImperativePlayApp` now binds `type Presence` / `type PresenceMutation` to the real presence types.

## Files touched

- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🎚️config/🧬️schema/🦀️component.rs` (new)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🎚️config/🧬️schema/🟦️component.ts` (new)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🎚️config/🧬️schema/🔗️component.graphql` (new)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🎚️config/🧬️schema/🔣️component.json` (new)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🎚️config/🧬️schema/🛰️component.proto` (new)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/👥️presence/🦀️component.rs` (new)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/👥️presence/🧬️schema/🦀️component.rs` (new)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/👥️presence/🧬️schema/🟦️component.ts` (new)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/👥️presence/🧬️schema/🔗️component.graphql` (new)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/👥️presence/🧬️schema/🔣️component.json` (new)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/👥️presence/🧬️schema/🛰️component.proto` (new)
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/📦️glue.rs` (updated)
- `✏️s/🔌️plugins/📜️imperative/🎛️apps/📜️imperative/🦀️component.rs` (updated)

## Gate results

### Scoped `policyAppSchemaBreaches` (imperative)

```
0
```

### `cargo check -p semio-s-plugin-imperative`

```
    Finished `dev` profile [unoptimized] target(s) in 54.18s
```

(final run: exit 0, `Finished dev profile`)

### `cargo test -p semio-s-plugin-imperative --lib`

```
test result: ok. 75 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
```

## Unverified

- Hub/runtime wiring of `presence_pack` with `ImperativePresence` in live multiplayer (A6+ catalog registration).
- No dedicated presence DSL/pack round-trip tests (lowpoly pilot pattern not duplicated in imperative tests yet).
- Config↔presence sync on `setSelection` / config mutations not hooked to SPR presence peers in this wave.
