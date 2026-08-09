# A5 Trinity Report — APP-SCHEMA-FACETS

## Summary

Wave A5 for `🔱️trinity` is complete for both owners (`♻️rewrite`, `🔌️jack`).

- Added five-leaf `🎚️config/🧬️schema` facets matching real `RewriteConfig` / `JackConfig` fields (`local-ui`).
- Added sibling `👥️presence` runtime modules (`RewritePresence` / `JackPresence` + Snapshot mutations) with five-leaf presence schemas (`shared-ui`).
- Presence fields: selection, hover/select or fixture/query, viewport camera, and per-window LOD map.
- Wired `📦️glue.rs` `config { component; schema }` + `presence { component; schema }` for both apps.
- Replaced `NoPresence` / `NoPresenceMutation` on both DocumentApps with the real presence types.

## Files touched

### Rewrite owner
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}` (created)
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/👥️presence/🦀️component.rs` (created)
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/👥️presence/🧬️schema/{five leaves}` (created)
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🦀️component.rs` (Presence bindings)

### Jack owner
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/🔌️jack/🎚️config/🧬️schema/{five leaves}` (created)
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/🔌️jack/👥️presence/🦀️component.rs` (created)
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/🔌️jack/👥️presence/🧬️schema/{five leaves}` (created)
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/🔌️jack/🦀️component.rs` (Presence bindings)

### Glue
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/📦️glue.rs` (config.schema + presence modules)

## Gate tails

### 1. Scoped `policyAppSchemaBreaches` (trinity)

```
0
```

### 2. `cargo check -p semio-s-plugin-trinity`

```
warning: `semio-s-plugin-trinity` (lib) generated 49 warnings (run `cargo fix --lib -p semio-s-plugin-trinity` to apply 43 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1m 58s
```

(Pre-existing unused-import / shadow warnings; check succeeded.)

### 3. `cargo test -p semio-s-plugin-trinity --lib`

```
test result: ok. 174 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s
```

## Unverified

- No end-to-end multi-peer presence sync exercised in a running host UI.
- TypeScript package consumers of the new schema leaves were not separately typechecked (Rust + policy gates only).
- Presence field set is a design choice from §6.2 / artifact shared-ui cues; collaborative UX of those fields is not runtime-validated beyond compile/tests.
