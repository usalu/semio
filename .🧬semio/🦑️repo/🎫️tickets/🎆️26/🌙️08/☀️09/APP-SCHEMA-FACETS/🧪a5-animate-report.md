# 🧪 A5 Animate Report — App Schema Facets

Ticket `26/08/09/APP-SCHEMA-FACETS`, wave A5, plugin `🎞️animate` / `semio-s-plugin-animate`.

## Summary

Delivered app schema facets for owner `🎬️present` (`PresentConfig` / `PresentPresence`): ten schema leaves under `🎚️config/🧬️schema` and `👥️presence/🧬️schema`, runtime `👥️presence/🦀️component.rs` with `PresentPresence` + `PresentPresenceMutation` (snapshot pattern per lowpoly pilot), nested `config` / `presence` modules in `📦️glue.rs`, and `DocumentApp` presence types wired on `AnimatePresentPlayApp`.

**Config fidelity:** `PresentConfig` schema mirrors runtime fields `selected_ids`, `engagement_input`, `locale` (all `local-ui`).

**Presence design:** `PresentPresence` carries peer-visible `selected_ids` (`shared-ui`) — the only shareable live surface state for tile selection; engagement draft and locale stay config-only.

## Files touched

- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}` (new)
- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/👥️presence/🦀️component.rs` (new)
- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/👥️presence/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}` (new)
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📦️glue.rs` (config/presence nesting + schema mounts)
- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🦀️component.rs` (`Presence` / `PresenceMutation` types)

## Gate results

### Scoped `policyAppSchemaBreaches` (animate filter)

```
0
```

### `cargo check -p semio-s-plugin-animate`

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in ...
```

### `cargo test -p semio-s-plugin-animate --lib`

```
test result: ok. 206 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.32s
```

## Unverified

- Runtime sync between `PresentConfig.selected_ids` and `PresentPresence.selected_ids` on presence peer encode/decode (kernel/framework wiring is A3; this wave only adds types, schema leaves, and `DocumentApp` bindings).
- End-to-end collaborative presence in a live host session (not exercised here).
