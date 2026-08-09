# 🧪 A5 Draw Report — App Schema Facets

Ticket `26/08/09/APP-SCHEMA-FACETS`, wave A5, plugin `🖍️draw` / `semio-s-plugin-draw`.

## Summary

Shipped app schema facets for owner `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🎚️config`: ten handcrafted leaves (`DrawConfig` + `DrawPresence`, five formats each), runtime `DrawPresence` / `DrawPresenceMutation` with DSL/pack (selection, hover, engagement input, `DrawCamera`, active utility), nested `config` / `presence` modules in `📦️glue.rs`, and `DrawPlayApp` wired to typed presence instead of `NoPresence`.

`DrawPresence` mirrors shareable canvas view state (not locale). Config facet matches runtime `DrawConfig` field-for-field including nested `DrawCamera`.

## Files touched

- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🎚️config/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🎚️config/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🎚️config/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🎚️config/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/👥️presence/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/👥️presence/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/👥️presence/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/👥️presence/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/👥️presence/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🦀️component.rs`

## Gate results

### Scoped `policyAppSchemaBreaches` (draw)

```
0
```

### `cargo check -p semio-s-plugin-draw`

```
warning: `semio-framework-os` (lib) generated 10 warnings (run `cargo fix --lib -p semio-framework-os` to apply 8 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2m 18s
```

### `cargo test -p semio-s-plugin-draw --lib`

```
test artifacts::draw::snapshot::pack::tests::pack_round_trips_and_agrees_with_dsl ... ok

test result: ok. 83 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

## Unverified

- Live multi-peer presence sync over SPR/hub (A3 pack path exists; draw does not yet publish `DrawPresence` from command handlers).
- A6 central `AppSchemaRegistry` registration for this owner (out of A5 scope).
- Runtime gesture scratch (`DrawSession`) is intentionally not in `DrawPresence` until a typed preview field is designed.
