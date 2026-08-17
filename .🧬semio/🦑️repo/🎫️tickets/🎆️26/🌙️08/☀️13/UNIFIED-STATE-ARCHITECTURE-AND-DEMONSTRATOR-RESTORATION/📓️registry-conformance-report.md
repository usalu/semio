# Plugin Registry Conformance

## Outcome

`bun nx run @semio-tech/plugin-registry:check --skip-nx-cache` exits `0` and reports the generated
catalog, playground catalog, framework catalog, and `.vscode/launch.json` as fresh:

- 59 plugin crates
- 58 playgrounds
- 23 framework packages

The successful canonical run is captured in `🧾️registry-check-final.log`.

## Root Causes and Repairs

### Cumulative Rust module-path parser

The registry models Rust's cumulative `#[path]` resolution with a base-directory stack. It pushed a
new base for every inline `mod` declaration, including one-line facade modules such as
`pub mod op { pub use …; }`. Those scopes close on their declaration line and must not remain on the
stack. Retaining them caused every later leaf path to resolve beneath accumulated facade names,
creating paired false findings: the real component appeared undeclared and the declared target
appeared missing.

`moduleScopeContinues` now pushes only declarations whose opening-brace count exceeds their
closing-brace count. The repair reduced taxonomy findings from 8,352 in the captured baseline to 594
and removed every dangling declared path.

### Independent migration dimensions

`areas["✏️s/🔌️plugins"] = "clean"` describes package-layout migration: plugin owners use the
`📦️packages/<language>` shape without residual owner-root implementation directories. The registry
also used that package-layout state to hard-fail a separate, unfinished taxonomy-tree migration.
Consequently, adding future tree requirements immediately broke an otherwise fresh catalog even
though the area had only graduated its package shape.

The taxonomy schema now declares `pluginAreas` and an independent `pluginTaxonomyStates` map. Shared
`validateTaxonomy` requires one valid tree state for every plugin area and rejects states for non-plugin
areas. The plugin package area remains truthfully `clean`; its tree contract is truthfully `mixed`.
The registry therefore preserves and prints all real tree findings while they remain migration debt,
and will hard-fail them when the dedicated state graduates to `clean`.

## Exact Residual Diagnostics

The canonical gate has zero failing diagnostics and 645 warning diagnostics:

| Class | Count | Status |
| --- | ---: | --- |
| Declared `#[path]` target missing on disk | 0 | repaired |
| Artifact missing required facets/leaves | 365 | taxonomy-tree debt |
| On-disk component not wired by `📦️glue.rs` | 194 | taxonomy-tree debt |
| Plugin root missing `🎮️commands/🦀️component.rs` | 33 | taxonomy-tree debt |
| App missing `⚙️engine/` | 2 | taxonomy-tree debt |
| Manifest without role marker | 44 | discovery warning |
| Unknown language directory below `📦️packages` | 5 | discovery warning |
| Ambiguous direct-manifest plus `🎯️targets` language shape | 2 | discovery warning |

The taxonomy-tree subtotal is 594. The independent discovery subtotal is 51. No plugin business or
component file was changed in this lane.

Largest taxonomy-tree contributors are `🗄️stdio` (234), `📕️norm` (62), `🌀️procedural` (45),
`🌊️flow` (18), `🧩️puzzle` (16), and `🧱️block` (14). The complete path-level inventory is retained in
`🧾️registry-check-final.log`.

## Validation

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/plugin-registry:check --skip-nx-cache` | pass, exit 0 |
| direct `validateTaxonomy(loadTaxonomy())` | 0 problems |
| targeted taxonomy-state validation test | 1 pass, 0 fail |
| `git diff --check` on the four edited files | pass |
| `bun test …/🧪️index.test.ts -t validateTaxonomy` | 21 pass, 1 pre-existing stale `📡️spr` assertion fail |
| `bun nx run @semio-tech/repo-lib:lint --skip-nx-cache` | pre-existing generated-manifest/rootDir/type failures; none reference this change |

Logs retained in this ticket:

- `🧾️registry-check-initial.log`
- `🧾️registry-check-after-module-parser.log`
- `🧾️registry-check-final.log`
- `🧾️registry-repo-lib-lint.log`

## Files Changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
