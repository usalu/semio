# A5 Block Report — APP-SCHEMA-FACETS

## Summary

Wave A5 for `semio-s-plugin-block` is complete. Three owners (2d / 5d / 3d apps) each have:

- config schema facet — five leaves documenting the real `Block*Config` fields as `local-ui`
- presence runtime — typed `Block*Presence` + `Block*PresenceMutation` (Snapshot)
- presence schema facet — five leaves with `shared-ui` fields
- `DocumentApp` Presence bindings switched from `NoPresence` to the real types
- glue.rs nested `config { component; schema }` and `presence { component; schema }` like lowpoly

Presence design:

- **2d / 5d:** `selectedIds` (peer selection)
- **3d:** `selectedIds` + `hoveredVortexFullId` (selection + vortex hover)

Config fidelity for 3d includes nested helpers (`Block3dWindowView`, `Block3dBrushPreview`, `BlockCamera3d`) as `$ref` / placeholder types in TS/GraphQL/JSON/proto, matching the artifact-schema pattern.

## Files touched

- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🎚️config/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🎚️config/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🎚️config/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🎚️config/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/👥️presence/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/👥️presence/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/👥️presence/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/👥️presence/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/👥️presence/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🎚️config/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🎚️config/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🎚️config/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🎚️config/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/👥️presence/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/👥️presence/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/👥️presence/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/👥️presence/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🖐️5d/👥️presence/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎚️config/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎚️config/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎚️config/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎚️config/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/👥️presence/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/👥️presence/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/👥️presence/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/👥️presence/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🧱️block/🎛️apps/🧊️3d/👥️presence/🧬️schema/🦀️component.rs`

### Ticket logs

- `🧪a5-block-cargo-check.log`
- `🧪a5-block-cargo-test.log`
- `🧪a5-block-policy.log`

## Gate tails

### 1. cargo check -p semio-s-plugin-block

```
warning: `semio-s-plugin-block` (lib) generated 60 warnings (run `cargo fix --lib -p semio-s-plugin-block` to apply 60 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 3m 01s
```

### 2. cargo test -p semio-s-plugin-block --lib

```
test result: ok. 100 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

### 3. Scoped policyAppSchemaBreaches (filter includes "block")

```
0
```

## Unverified

- No runtime console confirmation of peer presence pack round-trip over SPR (compile + unit tests + policy only).
- Nested 3d helper types in config schema leaves are placeholder / `$ref` shells (same pattern as block artifact schemas), not fully expanded field trees.
