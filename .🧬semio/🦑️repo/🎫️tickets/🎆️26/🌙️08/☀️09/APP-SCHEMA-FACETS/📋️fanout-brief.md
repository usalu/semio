# App Schema Facets — Fan-out Brief

Read first: the normative spec in this ticket folder (`📜️normative-spec.md`).
Owner table: `🧪owner-table.json`.

## Absolute rules

1. Touch **only** your assigned plugin's files (and that plugin's `📦️glue.rs` / TS glue). Never edit root `📜️script.ts`, taxonomy, kernel, or framework schema module — those belong to A1/A2/A3.
2. Diff your leaves against the lowpoly pilot leaves quoted in the normative spec §13 (filled by A4). Do not invent a different shape.
3. Gate **only** with the scoped scanner below — never `bun ./📜️script.ts policy` (it reports 1173 unrelated breaches).
4. On macOS: `DEVELOPER_DIR=/Library/Developer/CommandLineTools`.
5. Rename any `🧮️config` → `🎚️config` and `🕸️wasm` → `🌉️wasm` in your plugin before writing schemas. (Correct emoji: abacus+VS16 `🧮️config` becomes `🎚️config`.)
6. Presence fields: only shareable live state. Empty `XPresence {}` is valid when the app has nothing to share — document that in the docstring.
7. Config schema fields must match the existing `XConfig` Rust struct exactly (`app-schema/config-fidelity`).

## Scoped gate

```bash
bun -e 'const m = await import("./📜️script.ts");
const b = m.policyAppSchemaBreaches(process.cwd()).filter(x => x.scope.includes("PLUGIN_DIR_FRAGMENT"));
console.log(b.length); for (const x of b) console.log(x.kind, "|", x.summary);'
```

Also: `cargo check -p CRATE` then `cargo test -p CRATE --lib`.

## Owner assignments

See `🧪owner-table.json`. One agent per plugin crate. Norm's 15 apps share one owner at `✏️s/🔌️plugins/📕️norm/🎚️config` + sibling `👥️presence`.

## Per-owner deliverable

For each assigned owner:

```
🎚️config/🧬️schema/{🦀️component.rs,🟢️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}
👥️presence/🦀️component.rs          # XPresence + XPresenceMutation
👥️presence/🧬️schema/{five leaves}
```

Wire in `📦️glue.rs`: mount `config::schema`, `presence`, `presence::schema`. Update `type Presence = …` on the app's `DocumentApp` impl.

State classes: every config field `local-ui`; every presence field `shared-ui`.
