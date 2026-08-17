# A5 FEM Report

## Summary

Wave A5 for `🏗️fem` is complete. Both owners (`◻2d`, `🧊️3d`) now have config schema twins (five leaves each), empty presence runtime + schema (five leaves each), glue nesting like lowpoly, and `DocumentApp::Presence` / `PresenceMutation` wired to `Fem2dPresence` / `Fem3dPresence` (and 3d equivalents).

Config fields match real `Fem2dConfig` / `Fem3dConfig` (including nested `FemCamera`) with `local-ui`. Presence is intentionally empty: selection is command-transient, and camera/results already live on config as local-ui — documented in presence docstrings. Mutation uses the NoPresence-style `Noop` pattern.

## Files touched

### Created
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🎚️config/🧬️schema/` — five leaves (`Fem2dConfig`)
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/👥️presence/🦀️component.rs` — `Fem2dPresence` + `Fem2dPresenceMutation`
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/👥️presence/🧬️schema/` — five leaves
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🎚️config/🧬️schema/` — five leaves (`Fem3dConfig`)
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/👥️presence/🦀️component.rs` — `Fem3dPresence` + `Fem3dPresenceMutation`
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/👥️presence/🧬️schema/` — five leaves
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️09/APP-SCHEMA-FACETS/🧪a5-fem-report.md` (this file)

### Updated
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` — nest `config { component; schema }` and `presence { component; schema }` for both apps
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🦀️component.rs` — `Presence` / `PresenceMutation` types
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/🧊️3d/🦀️component.rs` — `Presence` / `PresenceMutation` types
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/📦️index.ts` — export config/presence schema facades

## Gate tails

### 1. Scoped policy (`policyAppSchemaBreaches` filtered to fem)

```
0
```

### 2. `cargo check -p semio-s-plugin-fem`

```
warning: `semio-s-plugin-fem` (lib) generated 65 warnings (run `cargo fix --lib -p semio-s-plugin-fem` to apply 47 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2m 28s
```

(Pre-existing warnings only; exit 0.)

### 3. `cargo test -p semio-s-plugin-fem --lib`

```
test result: ok. 332 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
```

## Unverified

- No host/runtime UI collaboration smoke for presence (empty presence; no shareable fields to exercise).
- Did not run full-repo `bun ./📜️script.ts policy` (forbidden by fan-out brief; scoped scanner only).
