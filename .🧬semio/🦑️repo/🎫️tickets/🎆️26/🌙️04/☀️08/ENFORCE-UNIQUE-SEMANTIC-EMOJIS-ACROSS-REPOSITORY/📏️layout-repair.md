# Layout Plugin Emoji Repair

## Scope

Owned tree: `✏️s/🔌️plugins/📏️layout`, including the Layout artifact, editor surfaces, serializers, mutation manifests, fixtures, package mounts, and exact central registrations.

The initial strict audit covered 450 files, 386 directories, and 829 governed entries. It found 49 sibling-emoji collisions. Every changed identity was selected and moved explicitly; no automatic emoji chooser, automatic rename planner, migration script, or modifying Git operation was used.

All three editor-window option authorities now use `☑️options`. The complete command map is `🖼️add-frame`, `➕️add-page`, `🩹️patch-frame`, `🧵️patch-page`, `📤️engagement-submit`, `📦️export-package`, `📕️export-pdf`, `🌄️export-png`, `🎨️export-svg`, `🧭️engagement-input`, `🎯️focus-preflight-issue`, `📑️set-active-page`, `🗣️set-locale`, `🚪️canvas-drag-leave`, `🛬️canvas-drag-over`, `📥️canvas-drop`, `👇️canvas-pointer-down`, `↔️canvas-pointer-move`, `👆️canvas-pointer-up`, and `📷️set-camera`.

Both DWG serializer families use `🏗️dwg`; both DXF families use `📐️dxf`. The subset-local contribution is `🔮️oracle`. The Edit Story mutation owner is `✍️edit-story`, distinct from the sibling `📝️text` authority, while Change Link Path is `🛤️change-link-path`, distinct from the sibling `🔗️.graphql` authority. All 25 mutation payload sidecars are `🧬️.schema.json`.

The 25 oracle, Python-adapter, and Gherkin fixture identities were reconciled individually with their physical scenarios: document/page scenarios use their existing `📃️…` identities; create/delete frame use `🚪️…`/`🚫️…`; story create/delete/edit use `🟤️…`/`🚫️…`/`🟦️…`; link scenarios use `🔗️…`; frame move/resize/fill/stroke use `🔵️…`, `📐️…`, `🍀️…`, and `🦅️…`; wrap/column scenarios use `🔤️…`; data fields uses `⛵️…`; and print target uses `🖨️…`.

## Verification

- Final strict scoped audit: 450 files, 386 directories, 829 governed entries; missing, generic, presentation, spacing, duplicate, multiple, reserved-emoji, and oracle findings are all zero.
- `validateTaxonomy(loadCatalogTaxonomy())` returns `[]` after the exact Layout oracle override and 13 previously absent command registrations. Seven command identities already existed exactly and were not duplicated.
- All 223 Rust `#[path]` package mounts resolve.
- The oracle manifest, all 25 mutation owners, all 25 scenario directories, all 25 payload schemas, and all 25 Python adapter fixture roots resolve.
- A stale-reference check finds none of the former local option, command, serializer, oracle, mutation-owner, payload-sidecar, or scenario paths. The only remaining `🧪️oracle` references are the intentionally unchanged external Stdio oracle registry and explanatory prose about that external authority.
- `bun nx run @semio-tech/layout-plugin:test-quick`: exited 1 before reaching Layout because the shared Stdio crate references a missing `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️1/🪆️subsets/✳️brep/🧬️schema/💡️inferences/🏷classification/🦀️.rs`; Cargo could not compile `semio-s-plugin-stdio`. This is outside the owned Layout scope.

## Exact Central Override

```json
"✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any": "🔮️oracle"
```
