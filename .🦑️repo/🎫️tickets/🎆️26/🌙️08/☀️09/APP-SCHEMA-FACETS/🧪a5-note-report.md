# A5 wave — 🗒️note

## Summary

Implemented APP-SCHEMA-FACETS for owner `🎚️config` / `NoteConfig` and sibling `👥️presence` / `NotePresence` on app `🗒️note/🗒️note`.

- **Config schema** (five leaves under `🎚️config/🧬️schema/`): mirrors runtime `NoteConfig` with `local-ui` on every field, including nested `NoteCamera` on `camera` (`s.note.note.config`).
- **Presence runtime** (`👥️presence/🦀️component.rs`): `NotePresence` with shareable live fields (selection, flattened camera, hover, active utility), `NotePresenceMutation` Snapshot pattern, DocumentDsl/Pack.
- **Presence schema** (five leaves): `shared-ui` facets for the same presence fields (`s.note.note.presence`).
- **Wiring**: `📦️glue.rs` nests `config { component; schema }` and `presence { component; schema }`; `NotePlayApp` uses `NotePresence` / `NotePresenceMutation` instead of `NoPresence`.

## Files touched

| Path | Action |
|------|--------|
| `✏️s/🔌️plugins/🗒️note/🎛️apps/🗒️note/🎚️config/🧬️schema/*` | Created (5 leaves) |
| `✏️s/🔌️plugins/🗒️note/🎛️apps/🗒️note/👥️presence/🦀️component.rs` | Created |
| `✏️s/🔌️plugins/🗒️note/🎛️apps/🗒️note/👥️presence/🧬️schema/*` | Created (5 leaves) |
| `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/📦️glue.rs` | Updated |
| `✏️s/🔌️plugins/🗒️note/🎛️apps/🗒️note/🦀️component.rs` | Updated |

## Gate tails

### Scoped policy (`policyAppSchemaBreaches`, note filter)

```
0
```

### `cargo check -p semio-s-plugin-note`

```
(exit 0)
```

### `cargo test -p semio-s-plugin-note --lib`

```
test result: ok. 65 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## Unverified

- Runtime presence channel sync with `NoteConfig` (config↔presence projection) not wired in this wave; presence types and facets only.
- Collaborative hosts consuming `NotePresence` over the wire not exercised in this crate’s tests.
