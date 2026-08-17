# 🧪 A5 Playbook Report — App Schema Facets

Ticket `26/08/09/APP-SCHEMA-FACETS`. Owner: `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🎚️config` (`PlaybookConfig` / `PlaybookPresence`).

## Summary

Implemented both app schema facets for the playbook play app following the lowpoly pilot (§13): five config schema leaves mirroring `PlaybookConfig` (`selected_ids`, `locale`, `contributions_json`, all `local-ui`), runtime `PlaybookPresence` with `selected_ids` (`shared-ui`) for peer block-list selection, five presence schema leaves, `Snapshot` presence mutation, nested `config` / `presence` modules in `📦️glue.rs`, and `DocumentApp` presence type bindings on `PlaybookPlayApp`.

## Files touched

| Path | Action |
| --- | --- |
| `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🎚️config/🧬️schema/{🦀️,🟦️/🔗️,🔣️,🛰️}component.*` | Created (5 leaves) |
| `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/👥️presence/🦀️component.rs` | Created |
| `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/👥️presence/🧬️schema/{🦀️,🟦️/🔗️,🔣️,🛰️}component.*` | Created (5 leaves) |
| `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/📦️glue.rs` | Updated (`config` + `presence` nesting) |
| `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🦀️component.rs` | Updated `Presence` / `PresenceMutation` |

## Gate 1 — scoped `policyAppSchemaBreaches`

```bash
bun -e 'const m=await import("./📜️script.ts"); const b=m.policyAppSchemaBreaches(process.cwd()).filter(x=>JSON.stringify(x).includes("playbook")||JSON.stringify(x).includes("📖️playbook")|| (x.scope||"").includes("playbook")); console.log(b.length); for (const x of b) console.log(x.kind, x.summary||x);'
```

```
0
```

## Gate 2 — `cargo check`

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-s-plugin-playbook
```

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in …
```

## Gate 3 — `cargo test --lib`

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p semio-s-plugin-playbook --lib
```

```
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## Unverified

- Runtime wiring of `presence_pack` / peer broadcast (kernel/framework A7+) — presence types compile and satisfy policy only.
- `🧩️extensions/🌀️procedural` `DocumentApp` still uses `NoPresence` (separate app, not in owner table).
