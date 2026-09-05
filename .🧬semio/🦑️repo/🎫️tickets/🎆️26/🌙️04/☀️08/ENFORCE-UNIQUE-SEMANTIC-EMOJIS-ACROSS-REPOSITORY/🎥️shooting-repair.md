# Shooting Plugin Emoji Repair

## Scope

Owned tree: `✏️s/🔌️plugins/🎥️shooting`, including the Shooting artifact, editor surfaces, mutation manifests, fixtures, package mounts, and exact central oracle registration.

The initial strict audit covered 583 files, 516 directories, and 1,091 governed entries. It found 36 sibling-emoji collisions. Every changed identity was selected and moved explicitly; no automatic emoji chooser, automatic rename planner, migration script, or modifying Git operation was used.

The Scene and Icon editor windows now use `☑️options`, distinct from their `🎚️config` siblings. The retained-command-limits fixture schema is `🧬️.schema.json`, distinct from its `🔣️.json` data file. The subset-local contribution is `🔮️oracle`, distinct from `🧪️tests`. The Change Asset URL mutation owner is `🌐️change-asset-url`, distinct from the sibling `🔗️.graphql` authority. All 31 mutation payload sidecars are `🧬️.schema.json`.

The 31 oracle, Python-adapter, and Gherkin fixture identities were reconciled individually with their existing physical scenarios. Asset scenarios use their existing `🖼️…` identities except the cross-asset drag scenario `🤖️…`; shot create/delete/rename/reorder/activation use `🔴️…`, `🚫️…`, `🔤️…`, `🟢️…`, and `🛟️…`; shot width/height/format/shape/camera use `🦋️…`, `🧭️…`, `🦀️…`, `⛵️…`, and `🦅️…`; saved-camera create/delete/rename/replace/reorder use `🍋️…`, `🚫️…`, `🔤️…`, `🍐️…`, and `🛰️…`; and all six scene-light/material scenarios use their existing `🎞️…` identities.

## Verification

- Final strict scoped audit: 583 files, 516 directories, 1,091 governed entries; missing, generic, presentation, spacing, duplicate, multiple, reserved-emoji, and oracle findings are all zero.
- `validateTaxonomy(loadCatalogTaxonomy())` returns `[]` after the exact Shooting oracle override. No command registrations were needed because no Shooting command identity changed.
- All 334 Rust `#[path]` package mounts resolve.
- All 31 mutation descriptors exactly match their physical owners and declare `🧬️.schema.json`.
- The oracle manifest's 31 mutation owners, 31 payload schemas, and 31 scenario directories resolve.
- All 31 Python adapter fixture roots and both 31-row Gherkin tables resolve, including the shared committed before-snapshot.
- A stale-reference check finds none of the former local option, payload-sidecar, subset-oracle, Change Asset URL, or scenario paths. The remaining `🧪️oracle` references point only to the intentionally unchanged external Stdio oracle authority.
- `bun nx run @semio-tech/shooting-plugin:test-quick`: exited 1 before reaching Shooting because the shared Stdio crate references a missing `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🧬️mutations/🔗create-edge/🔺️diff/🦀️.rs`; Cargo could not compile `semio-s-plugin-stdio`. This is outside the owned Shooting scope.

## Exact Central Override

```json
"✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any": "🔮️oracle"
```
