# 🧪 A5 — Sourcing (`semio-s-plugin-sourcing`)

## Summary

Wave A5 app schema facets for owner `✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🎚️config` (`SourcingCurateConfig` / `SourcingCuratePresence`, app `🪵️sourcing/🗂️curate`).

- **Config facet** — five schema leaves documenting the existing `SourcingCurateConfig` fields (`filters`, `selectedObjectId`, `locale`, `contributionsJson`) with `local-ui` on every field; nested `Filters` / `TableSort` / `SortDirection` in JSON `$defs` and matching GraphQL/proto/TS shapes (flow/writer pattern).
- **Presence facet** — runtime `SourcingCuratePresence` + `SourcingCuratePresenceMutation` (Snapshot) and five schema leaves for shareable live state: peer `selectedObjectId`, grid `worldCameraPosition` / `worldCameraTarget` / `worldCameraFov` (`shared-ui`).
- **Wiring** — `📦️glue.rs` nests `config { component; schema }` and `presence { component; schema }`; `SourcingCurateApp` uses typed `Presence` / `PresenceMutation` instead of `NoPresence`.

## Files touched

| Path |
| --- |
| `✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🎚️config/🧬️schema/{🦀️,🟦️,🔗,🔣️,🛰️}component.*` |
| `✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/👥️presence/🦀️component.rs` |
| `✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/👥️presence/🧬️schema/{🦀️,🟦️,🔗,🔣️,🛰️}component.*` |
| `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs` |
| `✏️s/🔌️plugins/🪵️sourcing/🎛️apps/🗂️curate/🦀️component.rs` |

## Gate tails

### Scoped `policyAppSchemaBreaches` (sourcing)

```
0
```

### `cargo check -p semio-s-plugin-sourcing`

```
warning: `semio-s-plugin-sourcing` (lib) generated 3 warnings (run `cargo fix --lib -p semio-s-plugin-sourcing` to apply 3 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 42.26s
```

### `cargo test -p semio-s-plugin-sourcing --lib`

```
test result: ok. 64 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

## Unverified

- Runtime presence sync (encoding peers’ `SourcingCuratePresence` into SPR / renderer) is not wired in this wave — only types, schema facets, and `DocumentApp` bindings.
- End-to-end multi-user curate session with live peer selection/camera overlays not exercised manually.
