# A5 Puzzle Report — APP-SCHEMA-FACETS

## Summary

Wave A5 complete for `semio-s-plugin-puzzle` across three owners (2d / 5d / 3d).

- Added config/schema (five leaves) and sibling presence (+ five schema leaves) per owner, mirroring lowpoly section 13.
- Config schema fields match real Puzzle2dConfig / Puzzle5dConfig / Puzzle3dConfig (config-fidelity); config fields are local-ui, presence fields are shared-ui.
- Renamed runtime structs to the *Config names (Puzzle2dPlayRuntime / Puzzle5dRuntime / Puzzle3dRuntime kept as type aliases) so the fidelity scanner binds the real struct.
- Presence runtime + Snapshot mutations shipped; each DocumentApp uses Puzzle2dPresence / Puzzle5dPresence / Puzzle3dPresence and matching Mutations (replacing NoPresence).
- glue.rs nests `config { component; schema }` and `presence { component; schema }` for all three apps.

## Files touched

### Plugin

- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🎚️config/🧬️schema/`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/👥️presence/🧬️schema/`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🎚️config/🧬️schema/`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/👥️presence/🧬️schema/`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🎚️config/🧬️schema/`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/👥️presence/🧬️schema/`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs`

### Ticket

- `a5_puzzle_gen.py`
- report file (this file)

## Gate tails

### 1. Scoped policyAppSchemaBreaches (puzzle)

```
0
```

### 2. cargo check -p semio-s-plugin-puzzle

```
Finished `dev` profile [unoptimized] target(s) in 0.53s
```

### 3. cargo test -p semio-s-plugin-puzzle --lib

```
test result: ok. 390 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.42s
```

## Unverified

- No end-to-end multiplayer presence sync exercised at runtime (compile/test + policy only).
- Nested helper types in config schema rust leaves are documentation mirrors (plain Serialize/Deserialize), not framework SelectionSet/WorldSunConfig imports.
- Repo MCP unavailable in this agent session; ticket folder discovered via Path.rglob('APP-SCHEMA-FACETS').
