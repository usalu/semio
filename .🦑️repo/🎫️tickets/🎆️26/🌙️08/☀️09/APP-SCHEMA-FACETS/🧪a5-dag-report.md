# A5 — DAG (`semio-s-plugin-dag`)

## Summary

Wave A5 app schema facets for the single owner `✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/🎚️config` (`DagConfig` / `DagPresence`, app `🕸️dag/🕸️dag`).

- **Config facet**: five schema leaves mirroring runtime `DagConfig` (`selected_node_ids`, `camera_x`/`camera_y`/`camera_zoom`, `locale`); all fields `local-ui`.
- **Presence facet**: runtime `DagPresence` + `DagPresenceMutation` (Snapshot pattern, DocumentDsl/Pack per lowpoly pilot) and five schema leaves. Shareable live fields: peer selection, node-graph viewport camera, optional hover targets (`hovered_node_id`, `hovered_edge_id`); all `shared-ui`. Locale stays config-only (local).
- **Wiring**: `📦️glue.rs` nests `config { component; schema }` and `presence { component; schema }`. `DagPlayApp` uses `DagPresence` / `DagPresenceMutation` instead of `NoPresence`.

## Files touched

| Path | Action |
| --- | --- |
| `✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/🎚️config/🧬️schema/*` (5 leaves) | Added |
| `✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/👥️presence/🦀️component.rs` | Added |
| `✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/👥️presence/🧬️schema/*` (5 leaves) | Added |
| `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/📦️glue.rs` | Updated |
| `✏️s/🔌️plugins/🕸️dag/🎛️apps/🕸️dag/🦀️component.rs` | Updated (`Presence` types) |

## Gate tails

### Scoped `policyAppSchemaBreaches` (dag)

```
0
```

### `cargo check -p semio-s-plugin-dag`

```
    Finished `dev` profile [unoptimized] target(s) in 32.89s
```

### `cargo test -p semio-s-plugin-dag --lib`

```
test result: ok. 69 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

## Unverified

- End-to-end `presence_pack` relay / renderer JSON for DAG peers (kernel wiring assumed done in A3; no runtime session test in this wave).
- `nodeGraphHover` still has an empty command payload; hover fields in `DagPresence` are schema-ready but not yet driven from commands.
