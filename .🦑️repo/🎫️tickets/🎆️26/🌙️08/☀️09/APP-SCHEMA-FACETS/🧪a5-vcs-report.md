# 🧪 A5 VCS Report — App Schema Facets

Ticket `26/08/09/APP-SCHEMA-FACETS`, wave A5 (`🌿️vcs`).

## Summary

Shipped config + presence app-schema facets for owner `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🎚️config` (`VcsDemoConfig` / `VcsDemoPresence`), following the lowpoly pilot shape. Config documents `selected_checkpoint_ids` and `locale` as `local-ui`. Presence is intentionally empty: all view state lives in config; docstring documents that. Wired `presence` + `config::schema` / `presence::schema` in `📦️glue.rs` and set `DocumentApp::Presence` / `PresenceMutation` on `VcsPlayApp`.

## Files touched

- `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}` (new)
- `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/👥️presence/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/👥️presence/🧬️schema/{five leaves}` (new)
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🦀️component.rs`

## Gate: scoped app-schema policy

```
0
```

## Gate: `cargo check -p semio-s-plugin-vcs`

```
    Checking semio-s-plugin-vcs v0.1.0 (...)
    Finished `dev` profile [unoptimized] target(s) in 1m 15s
```

(exit 0; upstream framework warnings only)

## Gate: `cargo test -p semio-s-plugin-vcs --lib`

```
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## Unverified

- Runtime presence pack round-trip not covered by dedicated tests (empty pack path only; mirrors `NoPresence` behavior).
- TS glue / `launch.json` not updated (no TS app-schema export surface required for this wave).
