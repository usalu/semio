# A5 Space Report — App Schema Facets

## Summary

Wave A5 for `🪐️space` / `semio-s-plugin-space` is complete for both owners:

| Owner | Config schema | Presence runtime | Presence schema | DocumentApp |
| --- | --- | --- | --- | --- |
| `🎛️apps/🏠️home/🎚️config` | `HomeConfig` (5 leaves) | empty `HomePresence` + `Noop` mutation | 5 leaves | `HomePresence` / `HomePresenceMutation` |
| `🎛️apps/🪐️space/🎚️config` | `SpaceConfig` (5 leaves, incl. `SpaceWindowCamera` map) | `SpacePresence` + `Snapshot` mutation | 5 leaves | `SpacePresence` / `SpacePresenceMutation` |

- **HomePresence** is intentionally empty: launcher chrome (`active_panel_tab`, `locale`) stays in `HomeConfig`; no multi-user live surface.
- **SpacePresence** shares live studio surface state peers should see: selection, hover, per-window camera, active/focused node, collapsed + preview-off node ids. Local-only config (clipboard, engagement drafts, pending import, client identity, panel tab, locale, `space_id`) stays out of presence.
- Glue nests `config { component; schema }` and `presence { component; schema }` for both apps (sibling of existing `commands::presence` heartbeat module).

## Files touched

### Created — home
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/👥️presence/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`

### Created — space
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎚️config/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/👥️presence/🧬️schema/{🦀️component.rs,🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`

### Updated
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` — nested config/presence modules for home + space
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🦀️component.rs` — `type Presence` / `PresenceMutation`
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🦀️component.rs` — `type Presence` / `PresenceMutation`

### Ticket
- `🧪a5-space-report.md` (this file)

## Gate tails

### 1. Scoped policy (`policyAppSchemaBreaches` filtered to space)

```
0
```

(After fixing proto3 `optional string` on all `Option<String>` fields in space config + presence protos.)

### 2. `cargo check -p semio-s-plugin-space`

```
warning: `semio-s-plugin-space` (lib) generated 11 warnings
    Finished `dev` profile [unoptimized] target(s) in 2m 18s
```

Exit 0. Pre-existing unused-variable warnings only; no errors.

### 3. `cargo test -p semio-s-plugin-space --lib`

```
test result: ok. 87 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

## Unverified

- End-to-end presence pack broadcast through DocumentHost / SPR (runtime still uses local `publish_presence` selection mirror; typed `SpacePresence` is wired on `DocumentApp` but not yet plumbed into that heartbeat path).
- Repo MCP ticket open/close unavailable in this session (`repo` server not present); work landed under existing ticket `26/08/09/APP-SCHEMA-FACETS`.
