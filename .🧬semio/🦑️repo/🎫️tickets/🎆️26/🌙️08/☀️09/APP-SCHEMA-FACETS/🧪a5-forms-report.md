# Wave A5 — forms (`semio-s-plugin-forms`)

## Summary

Implemented app schema facets for owner `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🎚️config`: five config schema leaves matching runtime `FormsConfig` (`selected_ids`, `current_step_index`, `try_values_json`, `locale`, `contributions_json`, all `local-ui`). Added sibling `👥️presence` with empty `FormsPresence` / `FormsPresenceMutation` (`Noop` pattern, no shareable live state yet) plus five presence schema leaves. Wired `📦️glue.rs` (`config { component; schema }`, `presence { component; schema }`) and set `FormsPlayApp` `Presence` / `PresenceMutation` to the real types.

## Files touched

### Created

- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/👥️presence/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`

### Updated

- `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🦀️component.rs`

## Gate tails

### Scoped `policyAppSchemaBreaches` (forms)

```
0
```

### `cargo check -p semio-s-plugin-forms`

```
(exit 0)
```

### `cargo test -p semio-s-plugin-forms --lib`

```
test result: ok. 87 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

## Unverified

- TS package glue mirror (forms has no app-level TS glue module beyond examples; Rust wiring only).
- Runtime multi-user presence sync (empty presence by design until shareable fields are defined).
