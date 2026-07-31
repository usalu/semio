---
goal: ANALYZING-IDS/ANALYZING-VSCODE-IDS
---

# Ticket

## Summary

Audited VS Code extension for old ID format references. Extension is a passive consumer of CLI-generated IDs. Found old-format test data at extension.test.ts lines 839,848,920-927. No ID construction/parsing in the extension itself. All changes needed are in Go CLI GetID() methods.

## Findings

### 1. Architecture: Extension uses Go CLI, not GraphQL for IDs

The extension invokes the Go CLI binary (`repo/cli/cli`) via `execAsync`/`execShell` and parses stdout as JSON events. IDs are constructed exclusively in the Go CLI (`main.go`) via `GetID()` methods on each entity type. The extension receives `TreeNodeData` with pre-built `ID`, `Label`, and `URI` fields.

Key functions:

- `extension.ts:381` — `getRepoBinaryPath()` resolves the CLI binary
- `extension.ts:462` — `execAsync(fullCommand, ...)` executes CLI commands
- `extension.ts:477` — `parseRepoEvents()` parses CLI JSON output

**Implication**: Any ID format change is server-side (CLI) only. The extension is a passive consumer.

### 2. Code that references ID format in `extension.ts`

#### `treeNodeDisplayLabel()` (line 580-596)

- Extracts emoji prefix from `node.ID` via `extractLeadingEmoji()` (generic Unicode emoji detection)
- Uses `fallbackEmojis` map for nodes without emoji prefix: `contributor: "👤️", commit: "🔀️", policy: "🛡️", statute: "⚠️"`
- **No old-format-specific parsing** — works with any emoji prefix generically

#### `parseUri()` (line 771-775)

- Parses `composerepo://` URIs into `{type, path}` tuples
- **Does NOT parse IDs** — only parses URIs which are separate from IDs
- URI format has no old/new distinction; it uses `composerepo://type/path`

#### `navigateToUri()` (line 777-960)

- Navigates to filesystem paths based on parsed URIs
- For tickets: uses `parsed.path` which is `YYYY/MM/DD/SLUG` (from URI, not ID)
- **No ID parsing** — purely URI-based

#### Filter labels (line 1333-1343)

- Hardcoded category labels: `"🏗️Projects"`, `"🎫️Tickets"`, `"🎫️Dates"`, `"🛡️Policies"`, `"👤️Contributors"`
- These are display labels, NOT IDs. They don't follow old or new ID format.

#### `copyId` command (line 1691)

- Copies `item.nodeId` (the raw ID from CLI) to clipboard
- **No ID manipulation** — passes through whatever CLI provides

### 3. Old ID format references in `extension.test.ts`

#### Policy ID with leading `/` (OLD FORMAT: `🛡️/code`)

- **Line 920**: `ID: "🛡️/code"` — test data uses old policy ID format
- **Line 921**: `Label: "🛡️/code"`
- **Line 927**: `assert.strictEqual(policyItem.label, "🛡️/code")`
- Source: `Policy.GetID()` in Go CLI (line 8198-8203) adds `/` prefix: `slug = "/" + slug`
- This is the **old format** where policy IDs have `🛡️/slug`

#### Ticket ID (line 839, 844): `ID: "🎫️test"`

- These test values don't follow either the old (`🎫️2025/02/04/slug`) or new (`🎫️YYYYMMDDslug`) format exactly
- They're just testing emoji extraction and display label logic, not ID parsing

#### File ID (line 848): `ID: "💻️compose/go/compose.go"`

- Uses old format: kind emoji directly without entity prefix
- New format would be: `📄️💻️compose/go/compose.go` (entity emoji + kind emoji + value)

#### Goal ID (line 853): `ID: "🎯️my-goal"`

- No entity prefix — but goals use `🎯️` as both entity and kind emoji

#### Breach Kind ID (line 907): `ID: "🚫️Code#Header#Missing Region"`

- Uses `#` separator (appears to be current format from CLI)

### 4. Current Go CLI ID formats (for reference)

| Entity      | Current `GetID()` format             | Example                                   |
| ----------- | ------------------------------------ | ----------------------------------------- |
| Project     | `{kindEmoji}@{name}`                 | `👤️@compose`                              |
| Bundle      | `{kindEmoji}{name}`                  | `📚️compose/js`                            |
| Folder      | `{kindEmoji}{path}`                  | `🗃️compose/js`                            |
| File        | `{kindEmoji}{path}`                  | `💻️compose/go/compose.go`                 |
| Section     | `🔖️{file}#{name}`                    | `🔖️compose/js/compose.ts#Entity IDs`      |
| Definition  | `{kindEmoji}{file}#{section}§{name}` | `🛠️compose/js/compose.ts#Kit§validateKit` |
| Ticket      | `🎫️{YYYY}/{MM}/{DD}/{slug}`          | `🎫️2026/02/14/MY-TICKET`                  |
| Goal        | `🎯️{id}`                             | `🎯️R26-02/RUNNING-SKETCHPAD`              |
| Draft       | `✍️{id}`                              | `✍️MY-DRAFT`                               |
| Todo        | `📝️{id}`                             | `📝️FIX-BUG`                               |
| Policy      | `🛡️/{id}`                            | `🛡️/code`                                 |
| Statute     | `🚫️{path}`                           | `🚫️Code#Header#Missing Region`            |
| Contributor | `👤️{github}`                         | `👤️usalu`                                 |
| Commit      | `🔀️{sha}`                            | `🔀️abc123`                                |

### 5. What needs updating for new ID format

**In the Go CLI (`main.go`)** — these are where actual ID construction happens:

- `Project.GetID()` (line 7181): `{kindEmoji}@{name}` → needs `{entityEmoji}{kindEmoji}{name}` (i.e. `🏗️🏘️compose`)
- `Bundle.GetID()` (line 7236): `{kindEmoji}{name}` → needs `📦️{kindEmoji}{name}`
- `Folder.GetID()` (line 7386): `{kindEmoji}{path}` → needs `📁️{kindEmoji}{path}`
- `File.GetID()` (line 7618): `{kindEmoji}{path}` → needs `📄️{kindEmoji}{path}`
- `Ticket.GetID()` (line 7990): `🎫️{YYYY}/{MM}/{DD}/{slug}` → `🎫️{YYYYMMDD}{slug}` (no date separators)
- `Policy.GetID()` (line 8198): `🛡️/{id}` → `🛡️{id}` (no leading `/`)
- `IdToUri()` (line 34691) and `UriToId()` (line 34848): need updating for entity prefix parsing

**In `extension.test.ts`** — test data that mirrors old format:

- Line 839: `ID: "🎫️test"` — minor, emoji-only test
- Line 848: `ID: "💻️compose/go/compose.go"` — should become `📄️💻️compose/go/compose.go`
- Line 920-927: `ID: "🛡️/code"` — should become `🛡️code` (no leading `/`)

**In `extension.ts`** — NO changes needed because:

- `extractLeadingEmoji()` and `treeNodeDisplayLabel()` use generic emoji detection
- `parseUri()` parses URIs (separate from IDs)
- The extension is a passive consumer of CLI-generated IDs

## Changes

No code changes made (audit-only ticket).

## Log

- Searched all files in `repo/vscode/` for emoji patterns, ID parsing, URI handling
- Read `extension.ts` parseUri, navigateToUri, treeNodeDisplayLabel, extractLeadingEmoji
- Read `extension.test.ts` for all ID-related test data
- Cross-referenced Go CLI `GetID()` methods for all entity types
- Cross-referenced `IdToUri()` and `UriToId()` conversion functions

## Todos

- [x] Search extension.ts for old ID format patterns
- [x] Search extension.test.ts for old ID format patterns
- [x] Determine if extension uses CLI or GraphQL for IDs
- [x] Document which lines reference old format
- [x] Write report

## Plan

Audit-only — no code changes. Report findings for follow-up migration work.
