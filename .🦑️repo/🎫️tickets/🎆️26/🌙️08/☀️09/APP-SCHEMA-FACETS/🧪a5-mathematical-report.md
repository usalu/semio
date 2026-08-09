# Wave A5 — mathematical

## Summary

Implemented app schema facets for owner `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎚️config`: ten schema leaves (config + presence), runtime `MathematicalPresence` / `MathematicalPresenceMutation` (empty shareable surface — graph edits are document ops; viewport/locale stay in config), `📦️glue.rs` nesting `config { component; schema }` and `presence { component; schema }`, and `DocumentApp` presence type bindings on `MathematicalPlayApp`.

Config facet matches runtime `MathematicalConfig`: `camera` (`MathematicalCamera` nested) + `locale`, all `local-ui`. Presence facet is empty `MathematicalPresence` with `Noop` mutation (framework `NoPresence` pattern).

## Files touched

- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}` (new)
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/👥️presence/🦀️component.rs` (new)
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/👥️presence/🧬️schema/{five leaves}` (new)
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🦀️component.rs`

## Gate — scoped policy

```
0
```

## Gate — cargo check

```
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-s-plugin-mathematical
Finished `dev` profile [unoptimized] target(s) in 44.48s
(exit 0; warnings only in dependencies and pre-existing mathematical artifact code)
```

## Gate — cargo test

```
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p semio-s-plugin-mathematical --lib
test result: ok. 52 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Unverified

- Runtime presence pack relay through SPR / multi-user sessions (no shareable fields yet).
- TS package glue mirror (this plugin has no separate TS glue mount for app config/presence schemas).
