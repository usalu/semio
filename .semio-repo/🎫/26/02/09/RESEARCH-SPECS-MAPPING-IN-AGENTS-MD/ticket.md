---
goal: R26-02/UPDATED-DOCS/UPDATED-DEV-DOCS/UPDATED-AGENTS-MD
---

# Ticket

## Summary

Mapped all 33 Business Logic + 8 UI/UX spec subsections in AGENTS.md to their owning bundles/files with implementation-specificity analysis. Domain model requirements (Design through Stat, 16 sections) are clean. Repo CLI requirements (Code Hygiene, Tree, Tooling, Repo Tooling, Artifact Kind Derivation) are heavily implementation-specific. Detailed line ranges, primary files, and cleaning needs documented in ticket.

## Changes

## Log

- Researched full Business Logic section (lines 22–530) and UI/UX section (lines 532–612)
- Mapped every subsection to primary bundle/file/folder
- Analyzed each for implementation-specific language

## Todos

## Plan

# 💯Requirements Mapping

## Business Logic (lines 22–530)

### Code Hygiene (lines 24–94)

- **Primary bundle/file:** `semio-repo/cli/` — specifically `semio-repo/cli/main.go` (policies, `headerPolicy`, `sectionPolicy`, `ScanComments`, `BaseLanguage`, language plugins, autofix logic)
- **Secondary:** All source files across monorepo (consumers of the policy)
- **Implementation-specific language:** YES — heavily references Go implementation identifiers: `FormatHeader`, `BaseLanguage.ScanComments`, `headerPolicy`, `sectionPolicy`, `applyAutofixes`, language-specific skip directives by name (`nolint`, `noqa`, `eslint-`, `@ts-`, `pragma`). Also references `supportsHeaders=true` and constructor patterns.

### Devcontainer (lines 96–110)

- **Primary bundle/folder:** `.devcontainer/` — `post-attach.sh`, `post-create.sh`, `post-start.sh`, `devcontainer.json`
- **Secondary:** `semio-repo/vscode/` (extension install), `~/.codeium/windsurf/mcp_config.json`
- **Implementation-specific language:** MODERATE — references specific tools (VS Code CLI, Cursor, Windsurf, Antigravity IPC hook CLIs), `$mid` location keys, `list-extensions`, WSL detection, but these are infrastructure-level naming.

### Sections (lines 112–116)

- **Primary bundle/file:** `semio-repo/cli/main.go` (section parsing, `LanguagePlugin`)
- **Implementation-specific language:** MINIMAL — "language-aware section parsing" is fairly generic. "Hash-based region markers" for shell is implementation-describing but not Go-specific.

### Move, Integrate, Extract (lines 118–130)

- **Primary bundle/file:** `semio-repo/cli/main.go` (`moveCommand`, `integrateCommand`, `extractCommand`, `ToolFileMove`, `ToolFolderMove`, `ToolSectionMove`, `ToolIntegrate`, `ToolExtract`)
- **Implementation-specific language:** YES — references Go function names: `ParseArtifactRef`, `UnSlugify`, `ToolFileMove`, `ToolFolderMove`, `ToolIntegrate`, `ToolExtract`, `UpdateAgentsDocsPath`, `RemoveAgentsDocsEntry`, `#`-delimited slugs.

### Tree (lines 132–154)

- **Primary bundle/file:** `semio-repo/cli/main.go` (`BuildMonorepoTree`, `FilterMonorepoTree`, `SearchMonorepoTree`, `RenderMonorepoTree`, `treeCommand`)
- **Implementation-specific language:** MODERATE — references bleve (library name), `--only-<kind>`/`--no-<kind>` CLI flags. Describes behavior generically but some are CLI-flag level.

### Engine (lines 156–159)

- **Primary bundle/file:** `semio/engine/engine.py`
- **Implementation-specific language:** MINIMAL — "dev/debug mode flag" and "stdio MCP server mode" are generic. No code identifiers.

### State Management (lines 161–163)

- **Primary bundle/file:** `semio/js/sketchpad/Sketchpad.tsx` (XState state machine)
- **Implementation-specific language:** MINIMAL — "Sketchpad state machine" is a conceptual name used consistently.

### Tooling (lines 165–201)

- **Primary bundles:** `semio-repo/cli/main.go` (CLI artifact IDs, `IdToUri`, `UriToId`, `sync github`, `--json` output, cobra), `semio-repo/vscode/extension.ts` (sidebar views, `semio.navigate` command), `semio-repo/cli/cli.go` (streaming registry)
- **Implementation-specific language:** YES — heavy references to Go/CLI specifics: `IdToUri`, `UriToId`, cobra SilenceUsage/SilenceErrors, `semiorepo://` URI scheme, GraphQL TicketClient enum tokens, `YYYY/MM/DD/SLUG` identifiers, JSONL event stream. Also VS Code-specific: `semio.navigate` command name.

### Ticket (lines 203–226)

- **Primary bundle/file:** `semio-repo/cli/main.go` (ticket lifecycle), `.semio-repo/tickets/` (storage)
- **Secondary:** `semio-repo/vscode/extension.ts` (ticket UI), `semio-repo/server/main.go` (server persistence)
- **Implementation-specific language:** MODERATE — references storage format (`.semio-repo/tickets`), `ticket.md`, `important.md`, `CONTINUE`/`NOTICKET` keywords. These are spec-level naming conventions, not code identifiers.

### Goal (lines 228–242)

- **Primary bundle/file:** `semio-repo/cli/main.go` (goal lifecycle), `.semio-repo/goals/` (storage)
- **Implementation-specific language:** MODERATE — references `.semio-repo/goals/SLUG/goal.json` storage path, GitHub milestones/issues sync hierarchy.

### Repo Dev Server (lines 244–253)

- **Primary bundle/file:** `semio-repo/server/main.go`
- **Implementation-specific language:** MINIMAL — describes HTTP endpoints, bearer tokens, webhooks generically. No code identifiers.

### Repo Tooling (lines 255–301)

- **Primary bundle/file:** `semio-repo/cli/main.go` and `semio-repo/cli/cli.go` (CLI binary consolidation), `semio-repo/vscode/extension.ts` (VS Code tooling)
- **Secondary:** `graphql/repo/schema.graphql` (GraphQL types), `sql/sqlite/repo/schema.sql` (export)
- **Implementation-specific language:** YES — heavily references: `YYYY/MM/DD/SLUG` path identifiers, `--all` flag, GraphQL field names (`TicketClient`, `TicketDate`, `Interaction`, `Position`), `semio-repo/cli/cli.go` file path, `.semio-repo/reports/codebase.json`, `.semio-repo/` storage, `--md` flag, `--text` flag, JSONL event stream, usalu project 2. Mix of behavioral requirements and implementation details.

### Artifact Kind Derivation (lines 303–316)

- **Primary bundle/file:** `semio-repo/cli/main.go` (`DeriveBundleKind`, `DeriveFolderKind`, `DeriveFileKind`, `DeriveDefinitionKind`, `FileHeaderId`)
- **Implementation-specific language:** YES — references Go function names: `extractDefinitionKeyword`, `DeriveDefinitionKind`, `refineDefinitionKind`, `FileHeaderId`. Also references `package.json`/`project.json` `bundleKind` field.

### MCP Tools (lines 318–320)

- **Primary bundle/file:** `semio-repo/cli/main.go` (MCP handler functions)
- **Implementation-specific language:** MINIMAL — generic validation requirements. No code identifiers.

### Contributor (lines 322–328)

- **Primary bundle/file:** `semio-repo/cli/main.go` (contributor derivation), `.semio-repo/contributors/` (storage)
- **Implementation-specific language:** MINIMAL — describes behavioral requirements without code references.

### Kit (lines 330–338)

- **Primary bundle/file:** `semio/js/semio.ts` (TypeScript Kit model), `semio/go/semio.go` (Go Kit model), `semio/net/Semio/Semio.cs` (C# Kit model), `semio/py/engine/engine.py` (Python Kit model)
- **Secondary:** `sql/sqlite/schema.sql`, `semio/jsonschema/kit.json`
- **Implementation-specific language:** MODERATE — references `.zip` file format, `.semio` folder, `kit.db` sqlite file, specific file paths for schemas.

### Design (lines 340–352)

- **Primary bundle/file:** Cross-platform domain model — `semio/js/semio.ts`, `semio/go/semio.go`, `semio/net/Semio/Semio.cs`, `semio/py/`
- **Implementation-specific language:** NONE — pure domain language (proto, subdesigns, flat, hierarchical placement, components).

### Type (lines 354–362)

- **Primary bundle/file:** Cross-platform domain model (same as Design)
- **Implementation-specific language:** NONE — pure domain language (prototype, subtypes, virtual, scalable, mirrorable).

### Connection (lines 364–376)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE — pure domain language (gap, shift, rise, rotation, turn, tilt, connected, connecting, hierarchy, direction).

### Piece (lines 378–386)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE — pure domain language (fixed, linked, component, hierarchy).

### Connector (lines 388–400)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE — pure domain language (point, direction, mandatory, port, compatibility).

### Model (lines 402–416)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE — pure domain language (guid, file reference, tags, Jaccard index, 3D file extensions, tag selection).

### Attribute (lines 418–442)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE — pure domain language (kebab-cased name, unit identifiers, nested dictionaries).

### Tag (lines 444–448)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE

### Concept (lines 450–454)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE

### Plane (lines 456–460)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE — pure geometry (left-handed coordinate system).

### Url (lines 462–466)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE

### Quality (lines 468–474)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE — pure domain language (measurement, kind bitwise enum, range constraints, formula, benchmarks).

### Benchmark (lines 476–480)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE

### Port (lines 482–496)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE — pure domain language (compatibility rules).

### Concept (lines 498–502) — duplicate header

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE

### Author (lines 504–508)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE

### Layer (lines 510–514)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE

### Group (lines 516–520)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE

### Prop (lines 522–526)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE

### Stat (lines 528–530)

- **Primary bundle/file:** Cross-platform domain model
- **Implementation-specific language:** NONE

---

## UI/UX (lines 532–612)

### Sketchpad (lines 534–534)

- **Primary bundle/file:** `semio/js/sketchpad/` (all app files)
- **Implementation-specific language:** N/A — empty section header

### Ticket UX (lines 536–538)

- **Primary bundle/file:** `semio-repo/vscode/extension.ts` (ticket close output), `semio-repo/cli/main.go` (CLI rendering)
- **Implementation-specific language:** MINIMAL

### CLI (lines 540–546)

- **Primary bundle/file:** `semio-repo/cli/main.go` (`sanitizeProp`, `sanitizeSingleLine`, `renderEntityMarkdownLink`, `renderEntityHuman`)
- **Implementation-specific language:** YES — references Go function names: `sanitizeProp`, `sanitizeSingleLine`, `renderEntityMarkdownLink`, `renderEntityHuman`. Also references `sync github` as command name.

### Toolbar (lines 548–559)

- **Primary bundle/file:** `semio/js/sketchpad/Sketchpad.tsx` (panel system), individual app files (`Home.tsx`, `Kit.tsx`, `Design.tsx`, `Type.tsx`, `Feedback.tsx`)
- **Implementation-specific language:** MODERATE — references `addSection("toolbar", { id, specificity, order, content })` function signature, `panelVisibility: { toolbar: true, ... }` state structure.

### Interaction State (lines 561–563)

- **Primary bundle/file:** `semio/js/sketchpad/` (all app state machines)
- **Implementation-specific language:** MINIMAL

### Borders (lines 565–576)

- **Primary bundle/file:** `semio/js/globals.css`, `semio/js/sketchpad/elements.tsx`
- **Implementation-specific language:** MODERATE — references CSS-level concepts (border kinds, background levels, inset overlay strokes, Action UI elements).

### Windows (lines 578–584)

- **Primary bundle/file:** `semio/js/sketchpad/Sketchpad.tsx` (window system)
- **Implementation-specific language:** MODERATE — references `windowLayout` state field, JSON string persistence.

### VS Code Extension (lines 586–612)

- **Primary bundle/file:** `semio-repo/vscode/extension.ts`, `semio-repo/vscode/extension.test.ts`
- **Implementation-specific language:** YES — references VS Code API concepts (sidebar views, tree items, code actions, F2 rename, drag-and-drop), `YYYY/MM/DD/SLUG` identifiers, VSIX packaging, `$mid` location keys, WSL-only CLI responses. Deep coupling to VS Code extension framework.

---

## Category Summary: Spec Ownership by Bundle

| Bundle / Folder                              | Spec Sections Owned                                                                                                                                                                    | Impl-Specific? |
| -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| **semio-repo/cli/** (Go binary)              | Code Hygiene, Sections, Move/Integrate/Extract, Tree, Tooling (partial), Ticket (partial), Goal (partial), Repo Tooling, Artifact Kind Derivation, MCP Tools, Contributor, CLI (UI/UX) | HIGH           |
| **semio-repo/server/**                       | Repo Dev Server                                                                                                                                                                        | LOW            |
| **semio-repo/vscode/**                       | Tooling (partial), Ticket UX (UI/UX), VS Code Extension (UI/UX)                                                                                                                        | HIGH           |
| **.devcontainer/**                           | Devcontainer                                                                                                                                                                           | MODERATE       |
| **semio/engine/**                            | Engine                                                                                                                                                                                 | LOW            |
| **semio/js/sketchpad/**                      | State Management, Toolbar (UI/UX), Interaction State (UI/UX), Borders (UI/UX), Windows (UI/UX)                                                                                         | MODERATE       |
| **semio/js/semio.ts** + cross-platform       | Kit, Design, Type, Connection, Piece, Connector, Model, Attribute, Tag, Concept, Plane, Url, Quality, Benchmark, Port, Author, Layer, Group, Prop, Stat                                | NONE           |
| **semio/go/**, **semio/net/**, **semio/py/** | (consumers of domain model requirements)                                                                                                                                               | NONE           |

## Implementation-Specificity Classification

### CLEAN (implementation-agnostic, pure domain requirements)

Lines 340–530: Design, Type, Connection, Piece, Connector, Model, Attribute, Tag, Concept, Plane, Url, Quality, Benchmark, Port, Author, Layer, Group, Prop, Stat

### NEEDS CLEANING (contains implementation identifiers)

- **Code Hygiene** (lines 24–94): Remove Go function/type names (`FormatHeader`, `BaseLanguage`, `ScanComments`, `headerPolicy`, `sectionPolicy`)
- **Move/Integrate/Extract** (lines 118–130): Remove `ToolFileMove`, `ParseArtifactRef`, `UnSlugify`, etc.
- **Tooling** (lines 165–201): Remove `IdToUri`/`UriToId`, cobra references, JSONL stream details
- **Repo Tooling** (lines 255–301): Remove GraphQL field names, file path references, flag names
- **Artifact Kind Derivation** (lines 303–316): Remove `extractDefinitionKeyword`, `DeriveDefinitionKind`, etc.
- **CLI** (UI/UX lines 540–546): Remove `sanitizeProp`, `sanitizeSingleLine`, `renderEntityMarkdownLink`, etc.
- **VS Code Extension** (UI/UX lines 586–612): Remove VS Code API specifics

### BORDERLINE (moderate implementation references)

- **Kit** (lines 330–338): References `.zip`, `.semio` folder, `kit.db` — arguably spec-level format decisions
- **Devcontainer** (lines 96–110): References specific editors/tools by name — infrastructure naming
- **Tree** (lines 132–154): References bleve, CLI flags — library name + UI choice
- **Ticket** (lines 203–226): References `ticket.md`, `important.md` — storage format decisions
- **Goal** (lines 228–242): References `.semio-repo/goals/SLUG/goal.json` — storage format
- **Toolbar** (UI/UX lines 548–559): References function signatures, state structure
