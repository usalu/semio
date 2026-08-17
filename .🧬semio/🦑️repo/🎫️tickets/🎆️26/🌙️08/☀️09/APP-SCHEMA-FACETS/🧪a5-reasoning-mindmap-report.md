# A5 — Reasoning Mindmap (`semio-s-plugin-reasoning-mindmap`)

## Summary

Wave A5 app schema facets for owner `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🎚️config` (`WiresConfig` / `WiresPresence`, app `💡️reasoning/🔌️wires`).

- **Config facet**: five schema leaves mirroring runtime `WiresConfig` (`selected_ids`, `drag_node_id`, `drag_last_x`, `drag_last_y`, `locale`); all fields `local-ui`.
- **Presence facet**: runtime `WiresPresence` + `WiresPresenceMutation` (Snapshot pattern, DocumentDsl/Pack per lowpoly pilot) and five schema leaves. Shareable live fields: peer selection and in-flight canvas drag (`drag_node_id`, `drag_last_x`, `drag_last_y`); all `shared-ui`. Locale stays config-only (local). Board camera lives on the document fixture, not presence.
- **Wiring**: `📦️glue.rs` nests `config { component; schema }` and `presence { component; schema }`. `ReasoningWiresPlayApp` uses `WiresPresence` / `WiresPresenceMutation` instead of `NoPresence`.

## Files touched

| Path | Action |
| --- | --- |
| `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🎚️config/🧬️schema/*` (5 leaves) | Added |
| `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/👥️presence/🦀️component.rs` | Added |
| `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/👥️presence/🧬️schema/*` (5 leaves) | Added |
| `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📦️glue.rs` | Updated |
| `✏️s/🔌️plugins/💡️reasoning/🎛️apps/🔌️wires/🦀️component.rs` | Updated (`Presence` types) |

## Gate tails

### Scoped `policyAppSchemaBreaches` (reasoning / reasoning-mindmap)

```
0
```

### `cargo check -p semio-s-plugin-reasoning-mindmap`

```
    Finished `dev` profile [unoptimized] target(s) in 54.95s
```

### `cargo test -p semio-s-plugin-reasoning-mindmap --lib`

```
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

## Unverified

- End-to-end `presence_pack` relay / renderer JSON for wires peers (kernel wiring assumed done in A3; no runtime session test in this wave).
- Commands still write selection/drag only to `WiresConfig`; presence is typed on `DocumentApp` but not yet populated from handlers for live multi-peer sync.
