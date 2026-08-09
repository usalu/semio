# 🧪 A5 Process — App Schema Facets Report

Ticket `26/08/09/APP-SCHEMA-FACETS`. Owner: `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎚️config` (`Process3dConfig` / `Process3dPresence`).

## Summary

Implemented wave A5 for the process plugin following the lowpoly pilot: five schema leaves under `🎚️config/🧬️schema` mirroring all `Process3dConfig` fields as `local-ui`; new `👥️presence` runtime (`Process3dPresence` + `Process3dPresenceMutation` with Snapshot pattern) and five presence schema leaves with shareable live state (`shared-ui`) — selection, hover, face pick, selection method, engagement input, camera triple, active utility. Locale and contributions JSON stay config-only. Wired `📦️glue.rs` with nested `config` / `presence` modules and schema submodules; `Process3dPlayApp` now uses typed presence instead of `NoPresence`.

## Files touched

| Path | Action |
| --- | --- |
| `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🎚️config/🧬️schema/*` (5 leaves) | Created |
| `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/👥️presence/🦀️component.rs` | Created |
| `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/👥️presence/🧬️schema/*` (5 leaves) | Created |
| `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs` | Updated |
| `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🦀️component.rs` | Updated (`Presence` types) |

## Gate tails

### Scoped app-schema policy

```
0
```

### `cargo check -p semio-s-plugin-process`

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in ...
```

### `cargo test -p semio-s-plugin-process --lib`

```
test result: ok. 128 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s
```

## Unverified

- Runtime presence sync to peers (`presence_pack` on the wire) — types and schemas only; no end-to-end multiplayer exercise.
- Protobuf package uses `semio.app.process.x3d` (valid identifier) while JSON `$id` uses `process/3d`; policy does not enforce package/$id alignment today.
