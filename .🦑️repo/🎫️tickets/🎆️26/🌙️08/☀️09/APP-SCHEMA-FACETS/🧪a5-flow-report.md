# Wave A5 — 🌊️flow

## Summary

Shipped app schema facets for owner `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎚️config`: ten schema leaves (`FlowConfig` + `FlowPresence`, five formats each), runtime `👥️presence` with `FlowPresence` / `FlowPresenceMutation` (snapshot-only), nested `config` / `presence` modules in `📦️glue.rs`, and `DocumentApp` presence types wired on `FlowPlayApp`. Config facet mirrors all `FlowConfig` fields as `local-ui`; presence carries shareable graph selection, preview-off ids, and node-graph `camera` as `shared-ui`.

## Files touched

- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/👥️presence/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs`

## Gate tails

### Scoped `policyAppSchemaBreaches` (flow)

```
0
```

### `cargo check -p semio-s-plugin-flow`

```
(exit 0)
```

### `cargo test -p semio-s-plugin-flow --lib`

```
test result: ok. 84 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s
```

## Unverified

- Runtime presence sync from live config (no dedicated presence dispatch path exercised in tests).
- TypeScript plugin glue export surface (this wave only wired Rust `📦️glue.rs` per assignment).
