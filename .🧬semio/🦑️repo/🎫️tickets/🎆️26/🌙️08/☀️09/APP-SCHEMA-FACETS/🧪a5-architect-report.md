# A5 architect — app schema facets

## Summary

Wave A5 for `semio-s-plugin-architect`: one owner (`ArchitectConfig` / `ArchitectPresence`) with full config and presence schema twins (five leaves each), runtime `👥️presence` document + `ArchitectPresenceMutation` (Snapshot pattern), `📦️glue.rs` nesting `config { component; schema }` and `presence { component; schema }`, and `DocumentApp` wired to typed presence instead of `NoPresence`.

`ArchitectPresence` holds shareable live UI: selection, active register, adjacency kind filter, and node-graph camera (search/report/analysis caches stay config-only `local-ui`).

## Files touched

- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}` (new)
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/👥️presence/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/👥️presence/🧬️schema/{five leaves}` (new)
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🏛️architect/🎛️apps/🏛️architect/🦀️component.rs`

## Gate tails

### Scoped policy

```
0
```

### `cargo check -p semio-s-plugin-architect`

```
Finished `dev` profile [unoptimized + debuginfo] target(s) in …
```

(exit 0)

### `cargo test -p semio-s-plugin-architect --lib`

```
test result: ok. 248 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
```

## Unverified

- Runtime presence sync from `ArchitectConfig` into `ArchitectPresence` on hub/view-model paths (schema + types only; no end-to-end multiplayer exercise).
- TS plugin glue mirror (Rust `📦️glue.rs` only per A5 scope).
