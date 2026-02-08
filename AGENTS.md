<!-- IMPORTANT -->

ALWAYS work inside a ticket. ALWAYS use semio-repo mcp (or the cli `./semio-repo/cli/cli`) for repo-specific infrastructure. ALWAYS start by listing the current goal tree with `goal_tree` (or `./semio-repo/cli/cli goal tree`). Create a new ticket with mcp tool `ticket_open` (or `./semio-repo/cli/cli ticket open <goal-id> <title> <prompt> <client> <llm> --draft <draft-id>? --parent <parent-ticket-id>?`). This creates a `.semio-repo/tickets/YYYY/MM/DD/TICKETSLUG` folder along with `ticket.md` in it. NEVER answer directly in the chat and ALWAYS document everything (todos, changes, summary, etc) in `ticket.md`. ALWAYS use the mcp tool `ticket_close` (or `./semio-repo/cli/cli ticket close <ticket-id> <summary> <files...>`) to finish the ticket along with the summary and at all the files you worked on (created, updated or removed). When a dev sends a new message to the chat ALWAYS reopen the same ticket with mcp tool `ticket_reopen` (or `./semio-repo/cli/cli ticket reopen <ticket-id> <prompt> <client> <llm> --draft <draft-id>? --title <new-title?> --goal <new-goal-id>? --parent <new-parent-ticket-id>?`).
Create a goal with mcp tool `goal_open`(or `./semio-repo/cli/cli goal open <title> <description> <prompt> <client> <llm> --due <due-date>? --parent <parent-goal>?`). NEVER create a goal when not excplicly asked to do so. Close a goal with mcp tool `goal_close`(or`./semio-repo/cli/cli goal close <GOALSLUG/SUBGOALSLUG> <summary>`). The due date is a date in the format `YYYY-MM-DD`. Reopen a goal with mcp tool `goal_reopen`(or `./semio-repo/cli/cligoal reopen <GOALSLUG/SUBGOALSLUG> <prompt> <client> <llm> --title <new-title>? --description <new-description>? --due <new-due-date>? --parent <new-parent-goal>?`).
A ticket id is `YYYY/MM/DD/TICKETSLUG`. A goal id is `GOALSLUG/SUBGOALSLUG/...`. A title MUST be titleized (e.g. "Some Title on Something") and NEVER be a slug or all caps. Available LLMs are: `opus-4-6`, `opus-4-5`, `sonnet-5`, `sonnet-4-5`, `haiku-4-5`, `gemini-3-pro`, `gemini-3-flash`, `gpt-5-2-codex`, `gpt-5-mini`, `swe-1-5`, `gpt-5-3-codex`. Available Clients are: `copilot-chat`, `windsurf-chat`, `claude-code`, `codex`, `cursor-chat`, `antigravity-chat`, `droid`.

- Multiple agents and a developer ALWAYS work on the same codebase at the same time. NEVER use `git stash`, `git stash pop`, `git checkout`, … because it will mess up others work and worst-case delete their work.
- The codebase in under design and development and not used in production yet. There are many inconsistencies that need to be refactored. ALWAYS use clean mechanisms that might require large refactorings and NEVER care about backwards compatibility.
- For every task you are working on, you MUST update the dev docs (`README.md` and `AGENTS.md`). Every key decision and mechanism ALWAYS needs to be documemented. Every feature, decision MUST be undocumented/uncommented in the code and MUST be documented in the dev docs (AGENTS.md and README.md). The documentation ALWAYS happens three times:

1. Under `# 📦 Bundles` in README.md where it is described from junior-developer perspective (mechanism explanation and reasoning behind the decision, how theory links to implementation, etc).
2. Under `# Software Requirements Specification` in AGENTS.md where it is described from human-interface-designer perspective (concise technical terms without explanation, framework-agnostic, no implementation references). There are two sections: `# Business Logic` and `# UI/UX`.
3. Under `# Codebase` in AGENTS.md where it is described from senior-developer perspective (framework-mechanisms, consice technical terms without explanation, implementation details, etc). The section has the same header structure as the files and folders. All files and folders are flat with `## PATH` e.g. `## semio/js/semio/sketchpad/` or `## semio/net/Semio.cs`
   The purpose of the dev docs is to understand the codebase. NEVER add reasoning or process related (such as what changed, why, how, … - this is part of the log) to the dev docs.

This document MUST ALWAYS BE followed unless explicitly asked to do otherwise.

<!-- IMPORTANT -->

# Software Requirements Specification

## Business Logic

### Code Hygiene

Source files MUST include an SPDX license header.

File headers MUST contain the correct file artifact ID (emoji-prefixed path) instead of plain file paths.

File header artifact ID violations MUST be autofixable by replacing the identified line with the correct artifact ID.

Source files MUST NOT include inline comments except for license headers, region markers, TODO markers (including contiguous comment blocks following them), and comments in configuration files.

Block and JSDoc comments are treated as inline comments.

Comment scanning MUST be language-agnostic via `BaseLanguage.ScanComments` using configurable primitives per language.

Each language MUST declare its string literal flavors (templates, raw backticks, triple quotes, verbatim strings), JSDoc support, and skip directives in its constructor.

Comment detection MUST ignore comment markers inside string literals, template literal text, raw backtick strings, triple-quoted strings, and verbatim strings.

Language-specific skip directives (e.g., `nolint` for Go, `noqa`/`type: ignore` for Python, `pragma` for C#, `eslint-`/`@ts-` for TypeScript) MUST be excluded from inline comment violations alongside built-in directives (`TODO`, `semio-ignore-`).

Inline comment violations MUST be grouped per contiguous inline-comment block.

Temporary diagnostic logs MUST include the `[DEBUG]` prefix and are considered removable.

Region blocks MUST be properly nested and MUST be closed with a matching named end marker.

Region blocks MUST NOT be empty.

All code MUST be within sections.

Developer documentation MUST be centralized in the root `README.md` and `AGENTS.md`; non-root `AGENTS.md` files and non-package `README.md` files are forbidden.

Shared UI element libraries MUST remain domain-neutral and MUST NOT use domain-specific terminology (kit, design, type, connector, connection, docs, feedback).

Shared UI element libraries MUST NOT import Sketchpad shells or app modules.

Only shared UI element libraries may import third-party dependencies; other JavaScript workspace sources MUST import within the workspace.

Sketchpad shell and app modules MUST only import shared elements, shared utilities, and core domain modules.

Code analysis problems MUST include reason and solution text.

Autofix MUST detect violations, filter to autofixable kinds, group by file, and apply fixes bottom-up per file.

Autofixable violation kinds: file header artifact ID replacement, empty section removal, missing section end name, section name mismatch, inline comment removal, block comment removal, JSDoc comment removal.

Empty section autofix MUST remove the section start, end, and content lines plus one surrounding blank line (prefer preceding).

Missing section end name autofix MUST walk backward from the end marker through nested sections to find the matching start name.

Section name mismatch autofix MUST replace the end marker name with the matching start name.

Inline comment autofix MUST remove the contiguous block of inline comment lines including intervening blank lines. If an inline comment is on the same line as code, only the comment portion MUST be removed.

Block and JSDoc comment autofix MUST remove the comment markers and content. If the comment markers are on the same line as code, the surrounding code MUST be preserved.

Post-removal blank line collapse MUST prevent consecutive blank lines in the output.

### Devcontainer

Devcontainer provisioning MUST install the workspace VS Code extension automatically after editor attach without manual installation steps.
Devcontainer post-attach MUST uninstall any existing semio-repo extension via IDE IPC hook CLIs and extensions directory cleanup, clear stale VS Code and Cursor extension caches, install the workspace extension for VS Code, Cursor, Windsurf, and Antigravity, validate installs with list-extensions, and fall back to direct extensions directory installs with extensions.json updates that include `$mid` location keys when CLIs report WSL-only usage.
Devcontainer post-attach MUST write the Windsurf MCP config at `~/.codeium/windsurf/mcp_config.json` to register the semio-repo MCP server.
Semio VS Code extension engine compatibility MUST include Cursor's supported VS Code version range.
Playwright browser caches MUST use the workspace `node_modules` volume path so `npx playwright install` stays cached across reloads.
Claude Code and Codex auth plus chat history MUST persist across devcontainer rebuilds via named volumes for CLI config and editor server state.
Claude Code auth files MUST live in the persisted Claude volume and be linked into `$HOME`.

### Sections

File section trees MUST be derived from language-aware section parsing per file.
Section data MUST expose file path, section path, range, and parent-child relationships.
Shell scripts (`.sh`) MUST use hash-based region markers for section parsing.

### Move, Integrate, Extract

The `move` command MUST accept two artifact ID arguments (source and target) and dispatch based on artifact kind pairs: file→file (`ToolFileMove`), folder→folder (`ToolFolderMove`), section→section within the same file (`ToolSectionMove`), file→section (`ToolIntegrate` then delete source), section→file (`ToolExtract`).

Artifact IDs MUST be parsed via `ParseArtifactRef` which detects kind from emoji prefix (📁 folder, 💻/📄 file, 🔖 section) and extracts path and section parts from `#`-delimited slugs.

Section slug resolution MUST attempt to match existing section names case-insensitively before falling back to `UnSlugify` conversion.

The `integrate` command MUST accept either two artifact ID positional arguments (source file, target section) or `--file`, `--target-file`, `--target-section`, `--parent-section` flags.

The `extract` command MUST accept either two artifact ID positional arguments (source section, target file) or `--file`, `--section`, `--target-file` flags.

File and folder move operations MUST automatically update `AGENTS.md` `# Codebase` section headers by replacing old path prefixes with new ones via `UpdateAgentsDocsPath`.

Cross-kind file→section move MUST remove the source file and its `AGENTS.md` entry after successful integration via `RemoveAgentsDocsEntry`.

MCP tools `move`, `extract`, and `integrate` MUST expose the same functionality as their CLI counterparts.

### Tree

The `tree` command MUST render the complete monorepo as a hierarchical tree of categories (projects, goals, drafts, policies, contributors, commits) with nested entity nodes.

The `tree` command MUST accept an optional positional query for fuzzy full-text search via bleve across all node attributes (id, label, description, status, contributor, kind, URI).

Search results MUST preserve the parent chain of matched items and prune unmatched branches.

Kind-level filtering MUST use `--only-<kind>` and `--no-<kind>` flags for projects, bundles, folders, files, sections, definitions, goals, tickets, drafts, policies, contributors, and commits.

Excluded kinds MUST collapse, promoting their children to the parent level.

Sub-kind filtering MUST narrow within a kind via `--only-<subkind>` and `--no-<subkind>` flags (library, schema, binary, client, site, assets for bundles; organization, required for folders; code, script, config, test, docs, resource, license for files; implementation, interface, constant for definitions).

Date filtering MUST support `--only-year`, `--no-year`, `--only-month`, `--no-month`, `--only-day`, `--no-day` for tickets and commits.

Status filtering MUST support `--only-open`, `--only-closed`, `--open`, `--closed` for goals and tickets.

Contributor filtering MUST support `--only-contributor-name` and `--no-contributor-name`.

Section and definition parsing MUST be opt-in, activated only when `--only-section`, `--only-definition`, or a search query is present.

Tree building MUST use a single concurrent filesystem walk for all folders and files with parallel streaming of all other data sources.

### Engine

Engine startup MUST support a dev/debug mode flag that waits for debugger attachment before runtime begins.
Engine startup MUST support a pure stdio MCP server mode.

### State Management

App hover and selection state MUST be managed by the Sketchpad state machine.

### Tooling

Sidebar view providers MUST be registered once per view with a single shared filter state source.

VS Code extension test runners MUST support headless Linux execution by provisioning a virtual display when `DISPLAY` is missing.

CLI artifact IDs MUST enforce emoji text presentation using U+FE0E to avoid terminal glyph overlap with adjacent characters.
Artifact IDs are the primary identification system and MUST be used in GraphQL, logs, messages, and UI labels.
Artifact URIs (`semiorepo://` scheme) are the secondary identification system and MUST be used where IDs are not supported (MCP resources, clickable links).
Section URIs MUST encode section names as UPPERCASE-SLUG path segments replacing `#` separators with `/`.
Definition URIs MUST encode section and definition names as UPPERCASE-SLUG path segments replacing `#` and `§` separators with `/`.
Project URIs MUST include the `@` prefix in the project code path segment.
Collection artifact types (projects, bundles, folders, files, sections, definitions, tickets, goals, drafts, todos, policies, violationKinds, contributors, commits) MUST have dedicated ID and URI formats.
`IdToUri` MUST convert any emoji-prefixed artifact ID to the corresponding `semiorepo://` URI.
`UriToId` MUST convert any `semiorepo://` URI to the corresponding emoji-prefixed artifact ID.
The `navigate` MCP tool MUST accept either an artifact ID or URI and return both the resolved URI and ID.
The `semio.navigate` VS Code command MUST accept either an artifact ID or URI and navigate to the corresponding resource.
Repo tool definitions MUST be top-level only (anchored at the start of the line).
The `sync github` command MUST reconcile local tickets and goals with GitHub by: ensuring root goals (depth 0) have milestones, ensuring first-generation child goals (depth 1) have issues with the `goal` label linked to the root goal's milestone, ensuring deeper goals (depth 2+) have issues with the `goal` label linked as sub-issues to their parent goal's issue without milestone, repairing existing goal issues so depth 1 issues always carry the root milestone, depth 2+ issues always have a parent sub-issue link and no milestone, and missing `goal` labels are restored, migrating child goals from legacy milestones to issues, processing goals in depth-first order so parents exist before children, ensuring issue titles and descriptions match local goal and ticket data, reopening GitHub issues if local tickets or goals are open, linking parent tickets as sub-issues to their parent ticket's issue, closing issues for closed tickets, resolving goal milestones by title via the GitHub API before assigning them to ticket issues, synchronizing repository label definitions for all valid project and bundle `@` labels, updating stored milestone URLs, and removing invalid `@` labels that do not map to current projects or bundles from both ticket-linked issues and repository-wide issue listings.
Go repo-tooling tests MUST support fast and slow execution lanes, and slow-lane suites MUST be shardable across parallel jobs while preserving full command-surface coverage.
CLI `--json` output MUST emit pure data per line without event wrappers or `{"data": ...}` GraphQL envelopes; errors MUST go to stderr; stdout MUST be empty on error.
CLI cobra root MUST set `SilenceUsage` and `SilenceErrors` to prevent stdout pollution on errors.

### Ticket

A `ticket` is a development artifact that tracks a task.

A `ticket` has a `status` of **open** or **finished**.

A ticket MUST store a `prompt` which is the prompt used to create the ticket.

A ticket MUST store a `commit` which is the git commit at ticket creation for line stats calculation.

A ticket interaction MUST store `started` and optional `finished` timestamps.
A ticket interaction `author` payload MUST be accepted as either a string or an object when reading persisted ticket and goal histories.

A ticket MUST store a summary when finished.

A ticket MUST store semantic diffs for projects, bundles, packages, folders, files, sections, and definitions with line stats when finished.

Ticket workspaces MUST store a single ticket.md that captures todos, changes, log entries, and the summary.
Ticket workspaces MUST store the content of the draft if provided.
The draft content MUST NOT be duplicated in ticket.md.

Ticket workspaces MUST store an `important.md` file for remaining compulsory actions. Ticket finish MUST throw an error if `important.md` is not empty.

Tickets can be reopened to return to **open** status.

Ticket close and reopen actions invoked from the ticket list MUST apply to the selected ticket without additional selection.

Ticket creation MUST require a prompt and a titleized title (e.g. "Some Title on Something"). Slugs or all-caps titles are forbidden.
Ticket LLM and Client inputs MUST be resolved forgivingly by matching allowed values as substrings within the slugified input.

Ticket title updates MUST rename the ticket folder and slug path.
Ticket open MUST interpret a `CONTINUE` keyword to continue the latest ticket and a `NOTICKET` keyword to skip ticket creation.

Ticket finish MUST require a summary and a list of files.

Temporary task artifacts MUST be stored inside the active ticket workspace.

Ticket finish MUST derive semantic diffs across projects, bundles, packages, folders, files, sections, and definitions via git diff between the ticket base commit and the current commit, scoped to the files declared on the ticket.
Ticket line metrics MUST map added lines to current scopes and removed lines to base-commit scopes for semantic diffs.

### Goal

A `goal` is a high-level grouping for `tickets`.

A `goal` has a `status` of **open** or **closed**.

A `goal` is stored in `.semio-repo/goals/SLUG/goal.json`.

Goals reflect the hierarchy of goals.

Tickets can optionally be assigned to a `goal`.

Tickets can optionally be assigned to a `parent-ticket` for hierarchy.

Root goals (depth 0) are synced as GitHub milestones. First-generation child goals (depth 1) are synced as GitHub issues with the `goal` label linked to the root goal's milestone. Deeper goals (depth 2+) are synced as GitHub issues with the `goal` label and linked as sub-issues to their parent goal's issue without milestone. Ticket issues are linked to the root ancestor goal's milestone.

### Repo Dev Server

The repo dev server MUST persist ticket state, scopes, claims, warnings, violations, and event history in a local database.
The repo dev server MUST accept diff ingestion payloads that include unified patches or file snapshots.
The repo dev server MUST recompute scope indexes and claims for files referenced by ingested diffs.
The repo dev server MUST emit conflict warnings when the same scope is claimed by multiple open tickets.
The repo dev server MUST expose HTTP endpoints for ticket lifecycle commands, diff ingestion, precommit checks, indexing, and read-only queries.
The repo dev server MUST support bearer token authentication for non-health endpoints.
The repo dev server MUST verify GitHub webhook signatures when configured.
The repo dev server MUST send outbound notifications formatted with prompt and summary headings.

### Repo Tooling

Ticket open inputs MUST allow optional `noIssue` and `draft` fields.
The repo CLI binary MUST be consolidated into a single `semio-repo/cli/cli.go` source file that owns engine, CLI, MCP, and rendering behavior.
Legacy repo CLI adapter packages MUST NOT exist outside `semio-repo/cli/cli.go`.
Repo operational commands (benchmark, preflight, update) MUST live in the single-file repo entrypoint.
Ticket close and reopen MUST address tickets via `YYYY/MM/DD/SLUG` path identifiers.
Ticket close MUST support an `--all` flag to bulk close all open tickets without summary requirements or GitHub interaction.
Ticket reopen MUST require `prompt` and `client` values. `llm` is optional.
GraphQL TicketClient inputs MUST accept normalized enum tokens (copilot_chat, claude_code, codex, etc.) for Client selection.
GraphQL `TicketDate` fields MUST include `started` and `finished` timestamps.
GraphQL interaction queries MUST return a list of `Interaction` objects with prompt, author, and time bounds.
GraphQL section/definition ranges MUST expose line and column positions for start and end.
GraphQL range selections MUST request Position subfields (line, column) for start and end.
Section list queries MUST include nested children ranges for full tree hydration.
Ticket listing MUST read from `.semio-repo/tickets` and fall back to legacy `tickets/` directories when present.
Ticket open MUST require a Goal ID.
Ticket open MUST require a ticket Client enum value.
Repo CLI MUST expose an export command that emits a SQLite snapshot of projects, bundles, packages, folders, files, sections, contributors, tickets, policies, and violations.
Repo section tooling MUST expose an integrate command that merges source files into target sections.
Ticket close MUST apply all affected bundle labels and the `semio-repo` label for out-of-bundle paths.
Ticket close MUST post a metrics comment listing semantic changes for projects, bundles, packages, folders, files, sections, and definitions with status icons and `+added`/`-removed` counts.
Ticket issue bodies MUST prepend a `# 🤖 Prompt` heading.
Ticket reopen MUST add a `# 🤖 Prompt` comment with the latest prompt.
Ticket close MUST prepend a `# 🔍 Summary` heading to the summary comment.
Ticket GitHub heading formatting MUST be consistent across create, reopen, and close flows.
Ticket line metrics MUST use full line counts for added and deleted scopes, and diff-based counts for modified scopes.
Ticket close MUST ignore files inside the active ticket workspace (ticket.md).
Repo analyze without a scope MUST emit a codebase snapshot to `.semio-repo/reports/codebase.json` for semantic diffing.
Ticket GitHub issues MUST be linked to the usalu project 2 on create and reopen.
VS Code extension manifests MUST use an unscoped `name` value for vsce packaging.
Repo CLI commands MUST emit a JSONL event stream with a terminal done payload for machine consumption.
VS Code tooling MUST parse JSONL event streams, surface fatal errors, and use the final result payload as the GraphQL response body.
Repo tooling MUST execute CLI, MCP, and VS Code commands through the streaming registry with emitter events for progress, items, errors, logs, and done payloads.
MCP list tools MUST support cursor and limit paging over streamed item events.

Repo operational artifacts (tickets, contributors, reports) MUST be stored under `.semio-repo/`.
Repo analyze MUST exclude gitignored files, `.semio-repo/`, and `assets/repo/` from analysis.
Repo file/folder listing and diagnostics MUST apply `.gitignore` patterns directly (including tracked matches) and exclude `.semio-repo/` paths in the repo CLI.
Repo `tree` and `list` commands MUST support a `--md` flag that outputs a nested Markdown bullet list using `semiorepo://` URI schemes.
Repo `tree` command MUST display nested Markdown bullet output by default and MUST support ASCII tree output via `--text`.
Repo CLI analyze and fix commands MUST accept scope arguments through flags or positional inputs.
GraphQL `node(id:)` MUST accept the canonical node IDs emitted by the schema (`semio/...`, `semio-repo/...`).
Ticket close MUST derive bundle labels from semantic bundle diffs and MUST NOT infer `semio-repo` from README.md or AGENTS.md.
Ticket interactions MUST store their own semantic diff payloads; tickets MUST NOT store diff payloads at the top level.

Ticket close MUST require at least one considered file after applying repo exclusions and `.gitignore` filtering.

### Artifact Kind Derivation

Bundle kind MUST be derived from the `bundleKind` field in `package.json` or `project.json` at the bundle root, falling back to `library`.
Valid bundle kinds: `library`, `schema`, `binary`, `ui`, `site`, `assets`.
Folder kind MUST be derived from the folder name: `.`-prefixed folders and folders containing package manifests (`package.json`, `pyproject.toml`, `go.mod`, `Cargo.toml`, `*.csproj`, `*.sln`) are `required`; all others are `organization`.
File kind MUST be derived from the file name and extension using pattern matching: test files (`*.test.*`, `_test.*`, `*.spec.*`, `*.stories.*`, `*.benchmark.*`), config files (`.json`, `.yaml`, `.toml`, `.xml`, etc. plus named files like `Dockerfile`, `Makefile`), docs (`.md`, `.txt`, `.rst`), resources (images, fonts, media, archives, binaries), code (comprehensive language extension list), scripts (`.sh`, `.bash`, `.bat`, `.ps1`, etc.), and license files (names containing `license` or `licence`).
`FileHeaderId` MUST override the filename-derived file kind to `script` when the file exists on disk and its first line starts with a shebang (`#!`).
Definition kind MUST be derived from the language processor keyword via `extractDefinitionKeyword` and `DeriveDefinitionKind`: interface-like keywords (`interface`, `type`, `trait`, `abstract`, `delegate`, `record`, `union`, `scalar`, `extend *`) map to `interface`; constant-like keywords (`const`, `enum`, `var`, `let`, `static`) map to `constant`; all others (including `function`, `class`, `struct`, `def`, `func`, `fn`) map to `implementation`.
`extractDefinitionKeyword` MUST prioritize the word directly preceding the definition name over fallback keyword scanning, and MUST skip access modifiers (`public`, `private`, `protected`, `internal`, `abstract`, `sealed`, `virtual`, `override`, `async`, `partial`, `pub`, `export`).
`refineDefinitionKind` MUST reclassify `const`/`let`/`var` definitions as `implementation` when the initializer is an arrow function (`=>`), function expression, or class expression.

### MCP Tools

MCP tool calls MUST validate argument presence and types.
File and folder parameters MUST reference correct path kinds (file vs folder).
Invalid MCP tool arguments MUST return errors at the tool boundary.

### Contributor

Contributor contributions MUST be derived from ticket frontmatter and source file headers.

Contributor ordering MUST be based on ticket contribution count.

Contributor contributions MUST expose tickets, commits, projects, bundles, packages, files, and line totals.

### Kit

A `kit` is a collection of `types`, `designs`, `authors`, `qualities`, `attributes`, and `concepts`.

A `kit` is either _static_ (a special `.zip` file) or _dynamic_ (bound to a runtime).

A _static_ `kit` contains a reserved `.semio` folder that contains a `kit.db` sqlite file.

The SQL-schema of `kit.db` is found in `./sql/sqlite/schema.sql`.

For Inter-Process-Communication (IPC) the JSON-schema in `./jsonschema/kit.json` is used.

### Design

A `design` is an undirected graph of `pieces` (nodes) and `connections` (edges) with organizational `layers`, `groups`, `stats`, `attributes`, and `concepts`.

A `design` is _proto_ (a _protodesign_) when it has no _parent_ `design`.

The _children_ of a _parent_ `design` are _subdesigns_.

A _flat_ `design` has no `connections` and all `pieces` are _fixed_.

The `pieces` are _placed_ _hierarchically_ (breadth-first) for every _component_.

Additional `connections` which where not used in the _placement_ can be used to validate the computed `planes`.

### Type

A `type` is a reusable component with different `models`, `connectors`, `attributes`, `concepts`, and `authors`.

The `type` is _proto_ (a _prototype_) when it has no _parent_.

The _childen_ of a _parent_ `type` are _subtypes_.

A `type` can be **virtual** (intermediate type requiring other virtual types to form a physical type), **scalable**, and **mirrorable** with **stock** quantity, **unit**, and optional **location**.

### Connection

A `connection` is a 3D-Link between two `pieces` with the _translation_ parameters **gap** (offset in y-direction), **shift** (offset in x-direction) and **rise** (offset in z-direction), and the _rotation_ parameters **rotation** (rotation around y-axis), **turn** (rotation around z-axis) and **tilt** (rotation around x-axis).

The _translation_ is applied first, then the _rotation_.

The two `pieces` are called **_connected_** and **_connecting_** but there is no difference between them.

The _direction_ of a `connection` goes from the lower _hierarchy_ to the higher _hierarchy_ of the `pieces`.

A `connection` can have `attributes` and diagram positioning with **u** and **v** offsets.

### Piece

A `piece` is an instance of either a `type` or a `design` with **id**, optional **name**, optional **description**, optional **plane**, **center** position, **scale**, optional **mirror plane**, **hidden** and **locked** states, **color**, and `attributes`.

A `piece` is either _fixed_ (with a `plane`) or _linked_ (with a `connection`).

A group of _connected_ `pieces` is called a _component_.

The _hierarchy_ of a `piece` is the length of the shortest path to the next _fixed_ `piece`.

### Connector

A `connector` is a conceptual connection **point** with an outwards **direction**, **id**, optional **name**, optional **description**, and **t** value for diagram ring positioning.

A `connector` can be marked as **mandatory** in which case it is required to be connected to a `piece`.

A `connector` can reference an **port** (PortId) for explicit compatibility control. The port defines which other ports it is compatible with.

No **port** means the _default_ port which is compatible with all other connectors.

Connector compatibility is determined by the `port` definitions at the kit level.

A `connector` can have `props` that define measurable characteristics and `attributes` for additional metadata.

### Model

A `model` is a **guid**, optional **name**, **file** reference (FileId), optional **tags** (TagId references), optional **description**, and `attributes`.

The **file** is a required reference to a kit-level `file` entity via `FileId` (guid).

The **tags** are optional references to kit-level `tag` entities via `TagId` (guid). No **tags** means the _default_ model.

The similarity of `models` is determined by the jaccard index of their **tag** guids.

##### Supported 3D File Extensions

Model files should use supported 3D formats including: `gltf`, `glb`, `fbx`, `obj`, `dae`, `3ds`, `stl`, `ply`, `usdz`, `vrm`, `ifc`, `3mf`, and more.

##### Model Tag Selection

The footer displays all tag names from the type's/design's models. Clicking a tag toggles its selection. The model with the highest Jaccard index matching the selected tags is displayed in the scene.

### Attribute

A `attribute` is metadata with a unique **name**, an optional **value**, an optional **unit** and an optional **definition** (`url` or text).

The **name** is kebab-cased and with `.`-separated string similar to toml keys.

No **value** is equivalent to the boolean _true_ where the **name** is the category of the attribute.

The **unit** is a unit identifier.

- `mm` for millimeter, `cm` for centimeter, `dm` for decimeter, `m` for meter, `km` for kilometer
- `m²` for square meter, `m³` for cubic meter, `m⁴` for quartic meter
- `°` for degree, `rad` for radian
- `N` for newton, `kN` for kilonewton, `MN` for meganewton
- `°C` for degree Celsius, `°F` for degree Fahrenheit
- `W` for watt, `kW` for kilowatt, `MW` for megawatt, `GW` for gigawatt
- `Wh` for watt-hour, `kWh` for kilowatt-hour, `MWh` for megawatt-hour, `GWh` for gigawatt-hour
- `J` for joule, `kJ` for kilojoule, `kcal` for kilocalorie
- `kWh/m²a` for kilowatt-hour per square meter per year
- `m/s` for meter per second, `m²/s` for square meter per second, `m³/s` for cubic meter per second
- `Pa` for pascal, `kPa` for kilopascal, `MPa` for megapascal
- ...

A list of attributes is semantically equivalent to nested dictionaries where the key is the **name** and the value is the **value**.

### Tag

A `tag` is a kit-level entity with a unique **guid**, **name**, optional **description**, optional **icon**, and `attributes`.

Tags are used to categorize and filter `models` within a `type`. A `model` references tags via `TagId` (guid reference).

### Concept

A `concept` is a kit-level entity with a unique **guid**, **name**, optional **description**, optional **icon**, and `attributes`.

Concepts provide semantic grouping for `types` and `designs`. Types and designs reference concepts via `ConceptId` (guid reference).

### Plane

A `plane` is a location (**origin**) and orientation (**x-axis**, **y-axis** and derived z-axis) in 3D space.

The coordinate system is left-handed where the thumb points up into the direction of the z-axis, the index-finger forwards into the direction of the y-axis and the middle-finger points to the right into the direction of the x-axis.

### Url

A `url` is either _relative_ (to the root of the `.zip` file) or _remote_ (http, https, ftp, ...) string.

A _relative_ `url` is a `/`-normalized path to a file in the `.zip` file and is not prefixed with with `.`, `./`, `/`, ....

### Quality

A `quality` is a measurement definition with a **key**, **name**, **description**, **kind** (General, Design, Type, Piece, Connection, Connector), **unit information** (SI and Imperial), **range constraints** (min/max with exclusion flags), **default value**, and optional **formula**.

A `quality` can be **scalable** (adjusts with piece scaling) and have multiple **benchmarks** for performance evaluation.

The **kind** determines which entities the quality can be applied to using a bitwise enum system.

### Benchmark

A `benchmark` is a performance standard within a `quality` with a **name**, optional **icon**, and **range** (min/max with exclusion flags).

Benchmarks provide reference points for evaluating quality measurements against industry or design standards.

### Port

An `port` is a connector compatibility definition with **name**, optional **description**, optional **icon**, optional list of **compatible ports** (PortId references), and `attributes`.

The `port` is defined at the kit level and referenced by `connectors` via PortId.

An empty **compatible ports** list means the port is compatible with all other ports.

Two connectors are compatible if:

- Both have no port specified (default compatibility)
- They reference the same port
- One port's compatible list includes the other port's guid
- Either port has an empty compatible list and the other explicitly allows it

### Concept

A `concept` is a **name** and **order** pair that provides semantic grouping for `kits`, `types`, or `designs`.

Concepts enable hierarchical organization and categorization of design elements beyond simple naming.

### Author

An `author` has a **name** and **email** and can be associated with `kits`, `types`, or `designs` with a **rank** indicating contribution level.

Authors provide attribution and contact information for design ownership and collaboration.

### Layer

A `layer` is an organizational grouping within a `design` with a **name**, optional **description**, and **color** for visual organization.

Layers provide a way to group and manage pieces logically within complex designs.

### Group

A `group` is a collection of `pieces` within a `design` with optional **name**, **description**, **color**, and **attributes**.

Groups enable semantic clustering of pieces that belong together functionally or conceptually.

### Prop

A `prop` is a **key-value** pair on a `connector` that references a `quality` with a specific **value** and optional **unit**.

Props define measurable characteristics of connectors using the quality system for standardized measurement.

### Stat

A `stat` is a statistical measurement on a `design` that references a `quality` with **range** (min/max) and optional **unit**.

Stats provide computed or measured performance data for entire designs using the quality framework.

## UI/UX

### Sketchpad

### Ticket UX

Ticket close output MUST present semantic change lists for bundles, folders, files, sections, and definitions with status icons and line metrics.

### CLI

Terminal output markers MUST render emoji in text presentation (U+FE0E) to keep glyph spacing stable next to adjacent text.
The CLI MUST render relative dates for tickets and goals in both text and markdown outputs.
The `sync github` command MUST report issue closures, milestone reconciliation, repository `@` label create/delete operations, and `@` label removals with warnings on failures.

#### Toolbar

The toolbar is a floating panel positioned at the bottom center of the canvas. Each app registers toolbar sections via `addSection("toolbar", { id, specificity, order, content })`.

- **Home app**: Filter toggles for kit kinds (temporary, local, remote) with action buttons to create new kits
- **Kit app**: Filter toggles for artifact kinds (designs, types, qualities, ports, tags, concepts, files, folders, authors) with action buttons to create new artifacts
- **Design app**: Selection tools (normal, additive, subtractive) and lasso tools (rectangular, freeform)
- **Type app**: Selection tools and connector creation tool
- **Feedback app**: Send button to submit feedback form

Toolbar panel visibility defaults to `true` for all apps via `panelVisibility: { toolbar: true, ... }` in default state creation.

#### Interaction State

- Hover and selection feedback across Home, Kit, Design, Type, Quality, Docs, and Feedback is driven by the app state machine.
- Hover and selection highlights MUST be consistent across tables, lists, and diagrams.

#### Borders

- Element border kind (hover color)
- Window border kind (normal border color)
- Window spacing: 1-unit gap between windows and 1-unit margin to canvas edge
- Base canvas uses the base background surface; windows, panels, and temporary UI surfaces use their respective background levels
- Exactly one window is active in a multi-window layout; the active window surface uses an active background tint
- Table views use the active window surface background
- Global Sketchpad shell is wrapped in base level so Navbar/Footer resolve base background
- Panels are rendered under panel level so panel surfaces resolve panel background
- Window chrome controls MUST be rendered as Action UI elements
- Window frames use inset overlay strokes so all four edges remain visible with clipped layouts

#### Windows

- Sketchpad apps MUST render inside a multi-window workspace.
- Each app MUST define a set of window kinds and a default window layout.
- Window layouts MUST be persisted per app as JSON strings (`windowLayout`).
- The active window MUST be tracked for focus-sensitive UI.
- Window chrome MUST expose action controls for open-in-new-window, maximize/minimize, and close.

### VS Code Extension

- The semio-repo sidebar MUST expose exactly two views: Monorepo and Filter.
- The Filter view MUST represent each filter kind as a single item and expose filter options as view item menu actions.
- Filter view items MUST render emoji plus name labels with tooltip descriptions; filter option menu actions MUST use emoji-only labels and MUST NOT use codeicons.
- Filter state MUST apply globally to all Monorepo tree branches.
- Monorepo root nodes MUST expand to show children for Projects, Goals, Tickets, Policies, Contributors, and Commits.
- Ticket tree items expose inline close and reopen actions that apply to the selected ticket based on status.
- Ticket tree hovers show only the ticket description.
- Ticket creation prompts for LLM and ticket UI selections.
- Ticket tree items list commit entries as child nodes.
- Ticket commands collect title/prompt/LLM for open, prompt/LLM for reopen, and operate on `YYYY/MM/DD/SLUG` ticket identifiers.
- Ticket detail views consume git-derived per-file and total line stats stored on interactions and ticket close.
- Command trees mirror the CLI command and subcommand hierarchy; matching a command group keeps its subtree visible.
- Problem list diagnostics open in pinned editor tabs for immediate saves.
- Repo diagnostics and trees are driven by repo CLI ignore rules for gitignored files and `.semio-repo/` content.
- Contributor tree items list emails with mailto actions, links with external navigation, and contribution nodes with line summary descriptions.
- Contributor contributions are grouped into commits, bundles, tickets (year/month/day), and files (folder/file) with navigation actions and inline ticket close/reopen actions.
- The built-in Explorer hosts the Sections view; selecting a section navigates to it, F2 renames, drag-and-drop moves sections, JSON keys surface as sections, and inline actions create child sections, rename sections, and delete sections via repo commands.
- The Sections view resolves the active file's section tree with line ranges so navigation and section actions match the current editor content.
- Monorepo section tree rendering MUST include only section-typed section children and MUST exclude definition-typed children from section rows.
- Ticket tooling treats temporary artifacts as part of the active ticket workspace.
- Devcontainer setup uninstalls any existing semio-repo extension, clears stale VS Code and Cursor caches, then installs the workspace extension for VS Code, Cursor, Windsurf, and Antigravity on attach without manual installation actions, validating installs per detected editor IPC hook CLI and falling back to extensions directories with extensions.json registration (including `$mid` location keys) on WSL-only CLI responses.
- Extension engine compatibility targets the lowest supported editor version so Cursor accepts the packaged VSIX.
- Sidebar view registration keeps a single filter view and monorepo view instance wired to the shared filter state.

# Monorepo

## Devcontainer

The monorepo uses a devcontainer for consistent cross-platform development. The devcontainer includes:

**Policies:**

- ALWAYS document mechanisms technicallly in `AGENTS.md` and in `README.md`. Those documents NEVER keep a log and ALWAYS show the current state of the codebase.
- ALWAYS finish everything without asking in between.
- NEVER interrupt between TODOs or tickets.
- NEVER remove functionality. Not even to get the code to work quickly.
- ALWAYS be thorough.
- NEVER create scripts to automate manual tasks.
- NEVER leave a placeholder.
- NEVER stop halfways and ask if you should continue.
- If a task is too big, ALWAYS start with one small part and ALWAYS finish it and keep on as much as you can.
- ALWAYS finish the task.
- ALWAYS make the choice directly! If you have several options, don't ask in between, be opionionated and just go for it. Try to do as much as you can.
- ALWAYS toolfriendly over intuitive.
- ALWAYS expose the canonical CI/CD scripts `dev`, `build`, `test`, `update`, `prepublish`, and `publish` only at the root (which forwards them through `npx nx run-many -t <target>`). Do not add missing commands to workspace packages; keep only the scripts they already define, treat `dev` as the only long-running watch mode, and make sure the remaining commands exit so CI runners and agents can finish reliably.
- When multiple long-running dev processes exist for a single workspace, use hierarchical naming for VS Code tasks/launch configs (e.g. `dev js js storybook`, `dev js js sketchpad`) and use `dev:<...>` for root `package.json` scripts when spaces are not possible.
- NEVER create new files when not explicitly asked. ALWAYS add code to existing files using regions and subregions for structuring. Regions organize code into collapsible sections (e.g., `#region 🔖RegionName` / `#endregion` in C#, or `//#region 🔖RegionName` / `//#endregion` in JavaScript/TypeScript). Use subregions within regions for hierarchical organization. This keeps related code together and maintains a single source of truth per logical unit.
- NEVER create new `README.md` files. Documentation is centralized in the dev-docs (`README.md` and `AGENTS.md`).
- NEVER create new folders unless required by the ticket workflow; temporary data belongs in the active ticket folder.
- NEVER create additional example files and implement it directly in the dependent parts.
- NEVER remove code that is commented out.
- NEVER add comments to the code. Especially not to communicate to the user.
- NEVER ask to run a command where you are not using the output. All dev servers, debugging and testing processes are running.
- NEVER run modifying `git` commands such as (`git checkout`, `git branch`, `git stash`, …) because there are other are ALWAYS agents/processes/devs working on the same set of files at the same time. Only read-only `git` commands are allowed. If you messed up, ALWAYS fix the file.
- NEVER create tests unless you are explicitly asked to.
- ALWAYS use inline syntax if possible.
- NEVER add two statements into the same line.
- ALWAYS inline code.
- NEVER create a variable, function, … class, that is only used once and inline it.
- NEVER add extra new blank lines/newlines inside of code.
- NEVER add raw text to client elements. ALWAYS use i18n setups and provide translations for the existing languages.
- ALWAYS add `[DEBUG] ` prefix to temporary logs so that they can be easily removed later.
- Keep Sketchpad runtime console output clean: avoid persistent `console.log` usage and rely on warnings/errors plus removable `[DEBUG]` diagnostics only when investigating.
- NEVER care about backwards compatibility unless explicitly asked to. Even on schema changes ALWAYS refactor to clean code and introduce breaking changes.
- NEVER use `type` for naming enums, ports, or types. ALWAYS use `kind` instead to avoid confusion with the native `type` concept in Semio. Examples: `ArtifactType` → `ArtifactKind`, `WindowType` → `WindowKind`, etc.
- When fixing problems, ALWAYS update the existing file and NEVER create new fixed, updated, migrated, etc. files next to the old one.
- NEVER change (e.g. simplify/remove functionality) or skip any test to pass. ALWAYS adjust implementation to pass the tests.
- NEVER create additional scripts, tests, fixtures, assets, …
- NEVER create scripts outside the folder of the current ticket. Not even when debugging or diagnosing a library problem.
- ALWAYS create temporary scripts, tests, fixtures, assets, … inside the active ticket folder.
- ALWAYS run specific tests and NEVER use default interactive test mode that creates a never ending process.
- NEVER say that a test is passing when you didn't run it. ALWAYS run the test and check the report.

# Codebase

The folders and files are listed like this: [PATH] [DISKNAME]? # [NAME | SHORTNAME | …]? [SUMMARY]?

├── .claude
│ ├── agents
│ │ ├── reformatter.md # Exclusively to reformat text (code, lists, …)
│ │ ├── reorderer.md # Exclusively to reorder text (code, lists, …)
│ │ └── schema-changer.md # Exclusively to change the schema (code, api, database, …)
│ └── settings.json
├── .cursor
│ ├── constraints
│ │ └── repo.mdc # \*_/_.\*
├── .vscode
│ ├── launch.json # Lifecycle-ordered per-package launch configs with dev/test/build/publish variants
│ ├── tasks.json # Per-package task catalog for dev, test variants, build, publish flows
│ └── extensions.json
├── .github
│ ├── chatmodes
│ │ ├── Reformatter.chatmode.md # Exclusively to reformat text (code, lists, …)
│ │ ├── Reorderer.chatmode.md # Exclusively to reorder text (code, lists, …)
│ │ └── Schema-Changer.chatmode.md # Exclusively to change the schema (code, api, database, …)
│ ├── workflows
│ │ └── gh-pages.yml # Deploy user docs togh-pages
│ └── dependabot.yml
├── .semio-repo
│ ├── contributors # Repo contributor registry
│ ├── reports
│ │ └── codebase.json # Codebase snapshot for semantic ticket diffs
│ └── tickets # Repo ticket workspaces
├── .venv # Centralized Python virtual environment
├── coda
├── semio
├── semio-repo
├── nx.json # Nx targets and plugin configs
├── package-lock.json # All javascript dependencies
├── package.json # Monorepo and workspace setup
├── pyproject.toml # Python workspace setup
├── uv.lock # Python lock file
├── README.md # GFM dev docs

In general, if the user talks about an old file, then probably there is the same file with the suffix `*.old` that is the original state.

## 📁.devcontainer/

Devcontainer configuration and lifecycle scripts.

## 📄.devcontainer/devcontainer.json

Devcontainer configuration with VS Code customizations, container/remote env, post-create/start/attach commands, and persisted volumes for AI auth, editor server state, and Playwright cache under `node_modules`.

## 📄.devcontainer/post-create.sh

Devcontainer provisioning steps for dependency installs, including Playwright browser install into the shared cache path.

## 📄.devcontainer/post-start.sh

Devcontainer start script that fixes ownership for persisted volumes, normalizes Claude Code auth storage, sets git safe directories, and activates the Python virtual environment.

## 📄.devcontainer/post-attach.sh

Devcontainer post-attach script that uninstalls any existing semio-repo extension via IDE IPC hook CLIs and extensions directory cleanup, clears stale VS Code and Cursor caches, builds and installs the local semio extension via VS Code, Cursor, Windsurf, or Antigravity IPC hook CLIs with list-extensions validation and extensions directory fallback plus extensions.json registration (using `$mid` location keys) on WSL-only CLI responses, then writes the Windsurf MCP config for the semio-repo server.

## 📁semio-repo/

Repo tooling, CLI, and editor integration sources.

## 📁semio-repo/vscode/

VS Code extension source for semio-repo tooling workflows.

## 📄semio-repo/vscode/.vscode-test.mjs

VS Code test-cli configuration entrypoint that defines the compiled test glob and Electron launch arguments for extension tests.

## 📄semio-repo/vscode/extension.test.ts

VS Code extension integration tests covering command registration, diagnostics, sidebar view contributions, filter state behavior, and monorepo tree provider roots.

## 📄semio-repo/vscode/extension.ts

Extension activation entrypoint that registers the two sidebar views (Monorepo and Filter) backed by tree data providers wired to a shared filter state source.
The Filter view exposes one item per filter kind with emoji + name labels, tooltip descriptions, and emoji-only menu actions for option toggles.
The Monorepo view applies the shared filter state across all branches and uses GraphQL-backed data retrieval via the repo CLI executor.
Section child rendering filters GraphQL section-interface children to section-typed nodes before building section rows so definitions are rendered only in definition rows.
URI resolution uses the `semiorepo://` scheme. The `semio.navigate` command accepts either a `semiorepo://` URI or a plain artifact ID and resolves it to the appropriate resource. Ticket and goal URIs resolve directly to filesystem paths. File, folder, bundle, project, section, and definition URIs resolve via a tree node cache built from the CLI `tree --json` output. The `semio.navigateTo` command shows a quick pick of all cached tree nodes. A `vscode.UriHandler` is registered for the `semiorepo` scheme to handle external URI navigation. All tree items (including goals) have click-to-navigate commands.

## 📁js/

Javascript code with shared core (semio/js) that uses storybook and exports a handful of React components (Sketchpad, Diagram, Model) for both web-based and desktop-based environments, a documentation (semio/docs) that uses astro with starlight and mdx, and desktop (semio/desktop) that runs in electron.

### Policies

- NEVER use inline styling. Use tailwindcss (v4). v4 uses a `theme.css` (`semio/js/theme.css`) for theming and not `{theme:{…}}` in `tailwindconfig`.
- ALWAYS use colors defined in `@theme inline {…}` from `js/semio/globals.css`. NEVER use direct colors such as light, gray, …, dark, primary, secondary, tertiary outside of `js/semio/globals.css` and ALWAYS use semantic colors instead such as active, disabled, hover, …
- Borders use semantic kinds via Tailwind color tokens: `border-element` (hover color) and `border-window` (normal border color).
- GoldenLayout window chrome uses the window background token to match window content surfaces.
- GoldenLayout stack frames use inset strokes so window borders remain continuous on all four sides.
- ALWAYS add tooltips (normal and extensive) to all ui elements.
- ALWAYS load icons via the semantic icon layer in `semio/assets` and NEVER import icons directly from external libraries (lucide, heroicons, .). Only reexport placeholder assets from those libraries inside `semio/assets` and consume them through its semantic exports.

### Styling

- The ui consists of a three horizontal strips: navbar, canvas and footer. A canvas consists of windows. On top of the canvas are panels which can toggled on and off.
- Navbar panel toggles always order panels as Details, Chat, then Settings for every app.

## 📁js/semio/

Shared react components. The main component is Sketchpad. Sketchpad is used in three different szenarios:

1. As guest mode (readonly) in a statically generated pages.
2. As user mode in the browser (nextjs).
3. As user mode in a desktop app (electron).
   Sketchpad has a local store in yjs which syncs with indexeddb and the backend provider.

**Policies:**

- Domain logic is ALWAYS in semio.ts and whenever an operation is not ui bound, it should be implemented there.
- **State Management Architecture**: XState is the SINGLE SOURCE OF TRUTH for all UI state. Yjs is ONLY used for collaborative Kit data (types, designs, etc.) via `KitStore`. All other app stores (Design, Type, Quality, Docs, Home, Feedback) use `PlainAppStore` or `PlainKitDiffAppStore` base classes which do NOT use Yjs. React components read state via `useSelector(actor, ...)` and send events via `actor.send({type: ...})`. NO Yjs in React components.
  - `machines.ts` - Unified XState machine with all app state
  - `xstate-hooks.ts` - Clean React hooks using XState selectors
  - State is ALWAYS accessed over hooks. Mutation ALWAYS is via actor events. NEVER use useState for app state.
- **Keyed Initialization Pattern**: App initialization hooks (e.g., `useDesignAppInitialize`, `useTypeAppInitialize`, `useKitAppYjsToXStateSync`) use keyed refs to track initialization scope. Instead of boolean `hasInitialized`, use `initializedKeyRef = useRef<string | null>(null)` with composite keys like `${kitGuid}:${designGuid}` to properly reinitialize when route scope changes.
- **Event Handler Registration**: ONLY use `registerEventHandler` for XState event handling. The legacy `registerRuntimeAction` mechanism exists but MUST NOT be duplicated with `registerEventHandler`. Each event should have exactly ONE registration.
- **Granular Hook Architecture**: All app state hooks follow the `[value, setter, canSet]` tuple pattern:
  - **Pattern**: `const [value, setValue, canSetValue] = useAppValue();`
  - **Types**: `HookResult<T>` for read-write hooks, `HookNoSetResult<T>` for read-only hooks
  - **Field<T> Type**: Alternative object-based pattern with always-defined `set` (no-op when disabled):
    ```typescript
    interface Field<T> {
      value: T;
      canSet: boolean;
      set: (next: T) => void;
    }
    const field = useDesignAppSelectionField();
    field.set(newSelection); // Safe - no-op if canSet is false
    ```
  - **ActionField Type**: For action-only hooks without value:
    ```typescript
    port ActionField {
      canExecute: boolean;
      execute: () => void;
    }
    const action = useXStateAction(canEvent, event);
    action.execute(); // Safe - no-op if canExecute is false
    ```
  - **Adapters**: Use `fieldToHookResult(field)` and `hookResultToField(result)` for interop
  - **No Parameters**: Hooks use scope providers (`useKitScope()`, `useDesignScope()`, `useTypeScope()`, `usePieceScope()`, `useConnectionScope()`, `useQualityScope()`) to get context
  - **canSet**: Boolean indicating if the action is available (scope exists and controller is valid). Use this to disable UI elements when action is unavailable.
  - **Examples**:
    - `const [selection, setSelection, canSetSelection] = useDesignAppSelection();`
    - `const field = useDesignAppSelectionField();` // Field<T> pattern
    - `const [camera, setCamera, canSetCamera] = useTypeAppCamera();`
    - `const [isHovered, _, canReadHover] = useKitAppIsTypeHovered();` (inside TypeScopeProvider)
    - `const [loadingKits, _, canReadLoadingKits] = useHomeLoadingKits();` (read-only)
    - `const [theme, setTheme, canSetTheme] = useTheme();` (global settings)
    - `const [language, setLanguage, canSetLanguage] = useLanguage();` (global settings)
    - `const [expertise, setExpertise, canSetExpertise] = useExpertise();` (global settings)
    - `const [mode, setMode, canSetMode] = useMode();` (global settings)
    - `const [device, setDevice, canSetDevice] = useDevice();` (global settings)
  - **Scope Providers**: Wrap components in appropriate scope providers to enable hooks:
    - `<KitScopeProvider guid={kitGuid}>` - For kit context
    - `<DesignScopeProvider guid={designGuid}>` - For design context
    - `<TypeScopeProvider guid={typeGuid}>` - For type context
    - `<PieceScopeProvider guid={pieceGuid}>` - For piece context
    - `<ConnectionScopeProvider guid={connectionGuid}>` - For connection context
    - `<QualityScopeProvider guid={qualityGuid}>` - For quality context
- **Targeted Hooks**: Components MUST use targeted hooks for kit data access. Use the following hooks from `Sketchpad.tsx`:
  - `useKitTypes(guid?)` - returns types array
  - `useKitFiles(guid?)` - returns files array
  - `useKitDesigns(guid?)` - returns designs array
  - `useKitQualities(guid?)` - returns qualities array
  - `useKitAuthors(guid?)` - returns authors array
  - `useKitFolders(guid?)` - returns folders array
  - `useKitPorts(guid?)` - returns ports array
  - `useKitTags(guid?)` - returns tags array
  - `useKitConcepts(guid?)` - returns concepts array
  - `useKitName(guid?)` - returns kit name
  - `useKitDescription(guid?)` - returns kit description
  - `useTypeFromKit(typeGuid, kitGuid?)` - returns specific type
  - `useDesignFromKit(designGuid, kitGuid?)` - returns specific design
- **Stable Selectors**: When using `useSyncExternalStore` (via `useKit`, `useSyncField`, etc.), selectors MUST be stable references. Inline functions like `(k) => k.types ?? []` are recreated each render, causing the `getSnapshot` callback to be recreated and triggering infinite re-render loops. Use one of:
  - Module-level constant functions: `const selectTypes = (k) => k.types ?? EMPTY_TYPES;`
  - `useCallback` with proper dependencies for dynamic selectors
  - Stable fallback constants: `const EMPTY_TYPES: Type[] = [];` instead of inline `[]`
- **Deep vs Shallow Subscriptions**: AVOID `deep=true` unless you need to react to nested property changes within array items. Use `deep=false` (default) for add/remove/replace operations.
- **Stabilizing useMemo Dependencies**: When hooks return object/array references that change on each render, extract primitive values before passing to `useMemo`. Use refs to track previous values and `useEffect` for side effects that should only run when data actually changes:

  ```typescript
  const type = useType();
  const typeGuid = type?.guid;  // Extract primitive
  const typeModels = type?.models;  // Reference will change but content is stable
  const prevModelGuidRef = useRef<string | null>(null);

  const { modelGuid } = useMemo(() => { /* compute */ }, [typeModels, ...]);

  useEffect(() => {
    if (modelGuid !== prevModelGuidRef.current) {
      prevModelGuidRef.current = modelGuid;
      console.log("Model changed:", modelGuid);
    }
  }, [modelGuid]);
  ```

- **Performance Logging**: Use `enablePerformanceLogging(true)` to enable performance logging that tracks overfetching. Check console for `[PERF] Rapid re-render` warnings indicating components re-rendering too frequently.
- **Granular Piece Metadata System**: The piece metadata system uses DerivedStore for efficient caching of computed piece data:
  - **`usePiecesMetadataMap()`**: Returns a cached `Map<string, PieceMetadata>` for all pieces in the current design. Uses DerivedStore to cache the full piecesMetadata computation. Only recomputes when pieces or connections change.
  - **`usePieceMetadata(pieceId?)`**: Returns metadata for a specific piece, extracting from the cached Map.
  - **`useFlatPiecePlane(id?)`**: Returns the flattened plane for a piece.
  - **`useFlatPieceCenter(id?)`**: Returns the flattened center for a piece.
  - **`useIsConnectedPiece(id?)`**: Returns whether a piece has a parent connection.
  - **`usePieceDepth(id?)`**: Returns the depth of a piece in the connection hierarchy.
  - **`useFixedPieceId(id?)`**: Returns the fixed piece ID (root of the connected component).
  - **`useParentPieceId(id?)`**: Returns the parent piece ID if connected.
- **YPath and DerivedStore**: For fine-grained subscriptions beyond field-level:
  - **YPath**: Navigate Y.js structures with `[yPathMapKey("pieces"), yPathArrayItemById(pieceGuid, "guid")]`
  - **usePath(store, path, selector)**: Subscribe to a specific path in a Y.js store
  - **useDerived(derivedStore, key, deps, compute, selector)**: Subscribe to a computed value that depends on base paths
  - **DerivedStore**: Each `KitStore` and `DesignStore` has a `derived` property for caching computed values
- Kit concepts live in `KitStore` as `ConceptStore` entries backed by the `yConcepts` Y.Array; snapshots return full `Concept` objects (name, description, icon, attributes) and persistence rebuilds them from `yDoc.getArray("concepts")` with legacy guid fallback.
- Commands ALWAYS have an origin. ALWAYS add the id of the ui element as origin when calling commands.
- There is a transaction mechanism for kits. Every app transaction is an extended kit transaction. The undo redo manager is on app level and stores the diff of the transaction along with the app state. This way undo redo works even when the kit changes because only the diff is stored. The inverted diff is stored along with the diff to enable relative undo redo.
- NEVER use direct strings or `useTranslation` for displaying text. ALWAYS assign an `id` the ui element and use i18n keys which match the id.
- The code runs in different environments (different browsers, electron, mobile/desktop/tablet). Platform-specific functionality MUST be generalized and provided as props to Sketchpad. NEVER hardcode platform-specific behavior or APIs directly in components.
- Model tag selection is implemented via `TypeAppFooter` and `DesignAppFooter` components showing clickable tag names, the `selectBestModel(models, selectedTagGuids)` function to find the best matching model, and `selectedModelTags` state tracked per type (in Design app: `Record<Guid, string[]>` mapping type guids to selected tag guids).
- `SUPPORTED_3D_EXTENSIONS` constant in `semio.ts` lists all supported 3D formats. Use `validateModelFile(filename)` to check if a file extension is supported.

The former `Canvas`, `Navbar`, `Footer`, `Panel`, and `store` modules now live inside `js/semio/sketchpad/Sketchpad.tsx`. Keep the region order intact when modifying this file so downstream imports continue to work.

### Architecture - Open-Closed Principle

The codebase follows the Open-Closed Principle (OCP): closed for modification, open for extension. Adding new features ONLY requires adding new files/folders, NEVER modifying existing ones.

### Sketchpad App Plugin Architecture

The sketchpad uses a plugin-based architecture for apps. Each app (Home, Kit, Type, Design, Quality, Docs) registers itself via the `AppPlugin` system, enabling open/closed extensibility.

#### Plugin Structure

Each app plugin provides:

- **id**: Unique identifier (e.g., "home", "kit", "type", "design")
- **namespace**: Event prefix (e.g., "HOME", "KIT", "TYPE", "DESIGN")
- **machine**: XState machine contributions (actions, guards, eventHandlers, selectors)
- **createDefaultState**: Factory for initial app state
- **registerStores**: Optional store factory registration

##### File Layout

```
js/semio/sketchpad/
  shared.ts          # AppPlugin port, registry functions
  apps/
    index.ts         # Single import point for all app plugins
  Home.tsx           # Home app + homeAppPlugin
  Kit.tsx            # Kit app + kitAppPlugin
  Type.tsx           # Type app + typeAppPlugin
  Design.tsx         # Design app + designAppPlugin
  Quality.tsx        # Quality app + qualityAppPlugin
  Docs.tsx           # Docs app + docsAppPlugin
  Feedback.tsx       # Feedback app + feedbackAppPlugin
  Sketchpad.tsx      # Main orchestrator, XState machine
```

##### Plugin Registration

Apps register plugins as a side-effect on module import:

```typescript
const myAppPlugin: AppPlugin = {
  id: "myapp",
  namespace: "MYAPP",
  machine: {
    actions: {},
    guards: {},
    eventHandlers: {},
    selectors: {},
    createDefaultState: () => ({ ... }),
  },
};

if (typeof window !== "undefined") {
  registerAppPlugin(myAppPlugin);
}
```

##### Dynamic Event Dispatch

The sketchpad machine uses **dynamic event dispatch** via `dispatchAppEvent` action with **wildcard event handling**. Navigation states use `"*"` wildcard to accept ANY event, which is then dispatched to registered handlers.

**Architecture:**

```
┌─────────────────────────────────────────────────────────────┐
│ Sketchpad.tsx (App-Agnostic)                                │
│                                                             │
│  sketchpadMachine:                                          │
│    on: {                                                    │
│      // Explicit handlers for global events                 │
│      SET_THEME, SET_LANGUAGE, NAVIGATE, ...                │
│      // Wildcard at ROOT level catches all app events       │
│      "*": { actions: "dispatchAppEvent" }                  │
│    }                                                        │
│    states:                                                  │
│      navigation: { home: {}, kit: {}, design: {}, ... }    │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ shared.ts (Event Registry)                                  │
│                                                             │
│  registerEventHandler("HOME.TOGGLE_PANEL", handler)        │
│  registerEventHandler("KIT.SET_FILTER", handler)           │
│  executeEventHandler(context, event) → context updates     │
└─────────────────────────────────────────────────────────────┘
                           ▲
                           │
┌──────────────┬──────────────┬──────────────┬───────────────┐
│  Home.tsx    │   Kit.tsx    │  Design.tsx  │   Type.tsx    │
│              │              │              │               │
│ registerEvent│ registerEvent│ registerEvent│ registerEvent │
│ Handler(...) │ Handler(...) │ Handler(...) │ Handler(...) │
└──────────────┴──────────────┴──────────────┴───────────────┘
```

**Event Handler Registration:**

```typescript
import { registerEventHandler } from "./shared";

// Register handler for a specific event type
registerEventHandler("MYAPP.TOGGLE_PANEL", {
  guard: (context, event) => context.myApp !== undefined, // optional
  action: (context, event) => ({
    myApp: {
      ...context.myApp,
      panelVisibility: { ...context.myApp.panelVisibility, [event.panel]: !context.myApp.panelVisibility[event.panel] },
    },
  }),
});
```

**Key Functions:**

- **`registerEventHandler(eventType, config)`**: Registers a handler for a specific event type (e.g., "HOME.TOGGLE_PANEL")
- **`executeEventHandler(context, event)`**: Looks up and executes the handler for the event type
- **`dispatchAppEvent` action**: The sketchpad machine action that dispatches events dynamically
- **Fallback**: If no handler is registered via `registerEventHandler`, falls back to legacy `registerRuntimeAction` handlers

##### App Hooks Registry

Apps register hooks via the registry in `shared.ts` to enable cross-app communication without direct imports:

- **`registerDesignAppHooks(hooks)`**: Design.tsx registers its hooks (selection, hover, commands, etc.)
- **`registerKitAppHooks(hooks)`**: Kit.tsx registers its hooks (commands)
- **`registerDocsRegistry(registry)`**: Docs.tsx registers the docsRegistry
- **`getDesignAppHooks()`**: Returns registered design hooks (fallback defaults if not registered)
- **`getKitAppHooks()`**: Returns registered kit hooks (fallback defaults if not registered)
- **`getDocsRegistry()`**: Returns registered docs registry (null if not registered)

This pattern ensures:

- Sketchpad.tsx has no app-specific caches or hook getters
- elements.tsx has no imports from app modules
- Apps are self-contained and register their hooks on module load

**Benefits:**

- **Open/Closed Principle**: Adding a new app requires NO changes to `Sketchpad.tsx`
- **Self-contained apps**: Each app file registers its own event handlers
- **Wildcard handling**: Navigation states accept any event via `"*"` pattern
- **Guards in handlers**: Guards can be defined in the handler config, not in the machine
- **Gradual migration**: Existing `registerRuntimeAction` handlers continue to work
- **Single machine**: Only one `createMachine` call - `uiMachine` has been removed

##### Hook Pattern (Triadic)

All hooks follow the triadic pattern: `[value, setValue, canSetValue]`

- **UI components**: Only use triadic hooks, never access stores directly
- **Hooks**: Read from stores via subscriptions, write via `actor.send()` XState events
- **State machine**: Only writer API, accepts contributions from plugins
- **Stores/commands**: Implementation details behind machine actions

Example:

```typescript
export function useMyAppSelection(): HookResult<MySelection> {
  const actor = useSketchpadActor();
  const canSetEvent = useMemo(() => ({ type: "MYAPP.SET_SELECTION" as const, ... }), [...]);
  const canSet = useSelector(actor, (snapshot) => snapshot.can(canSetEvent));
  const setSelection = useMemo(() => {
    if (!canSet) return undefined;
    return (value: MySelection) => actor.send({ type: "MYAPP.SET_SELECTION", ... });
  }, [actor, canSet, ...]);
  return conditionalHookResult(canSet, selection, setSelection);
}
```

##### Adding a New App

1. Create app file with types, state, hooks, and UI components
2. Define `AppPlugin` with namespace and machine contributions
3. Register plugin: `registerAppPlugin(myAppPlugin)`
4. Import app module in `apps/index.ts`
5. No edits to `Sketchpad.tsx` required (open/closed principle)

####### App Structure Standards

All apps in `js/semio/sketchpad/*App.tsx` (Design.tsx, Home.tsx, Kit.tsx, Quality.tsx, Type.tsx, Docs.tsx) MUST follow this structure:

1. **Region Order:** Header → Imports → Types → Store → Commands → Components → App → Config
2. **Store Base Class:** MUST extend either `AppStore` or `KitDiffAppStore` (no custom base classes)
3. **Store Registration:** MUST use inline registration pattern (no wrapper functions)
4. **Component Regions:** MUST nest under Components region (Navbar, Canvas, Panels, Tools, Footer)
5. **Tools:** MUST have Tools region if app has multiple interaction modes
6. **Scope Providers:** MUST be defined in app file (not App.tsx)
7. **Commands:** MUST define all commands in Commands region

See `REFACTOR.md` for detailed rationale and migration guide.

####### Adding a New App

To add a new app:

1. Create a file in `js/semio/sketchpad/{AppName}.tsx`.
2. Add a single file that:
   - exports the default React component,
   - declares and exports `config: AppConfig`,
   - wires any local state, commands, or helpers needed by the app.
3. Keep optional helpers (pages, panels, tools) alongside the file and import them from the same module.

The app registry auto-discovers app files via `import.meta.glob('./*.tsx')`.

Example section inside the app file:

```typescript
import { FC } from "react";
import { AppConfig } from "../registry";

const App: FC = () => {
  // ...
};

export const config: AppConfig = {
  id: "myapp",
  component: App,
  routeSegments: [{ path: "my/:id", paramName: "id" }],
  getPanels: (t) => [{ key: "details", icon: Info, tooltip: t("panels.details"), hotkey: "⌘L" }],
  matchesPath: (pathParts) => pathParts[0] === "my",
  order: 50,
};

export default App;
```

##### Sketchpad Apps

###### Home App (Home.tsx)

Landing page for kit management. Extends `AppStore` (no kit modifications).

**State (`HomeState`):**

- `panelVisibility` - Panel toggle states
- `selection` - Selected kit GUIDs
- `sortColumn` / `sortDirection` - Sorting preferences
- `loadingKits` - Kits currently being loaded

**Events:**

- `HOME.TOGGLE_PANEL` - Toggle panel visibility
- `HOME.SET_PANEL_VISIBILITY` - Set all panel states
- `HOME.SELECT_KIT` / `HOME.DESELECT_KIT` - Kit selection
- `HOME.SET_SORT` - Change sorting

**Hooks:**

- `useHomeApp()` - Full home app state
- `useHomeSelection()` - Selected kits
- `useHomeLoadingKits()` - Loading state
- `useHomePanelVisibility()` - Panel visibility

###### Kit App (Kit.tsx)

Kit artifact management with multi-window layout. Extends `KitDiffAppStore` (modifies kit data).

**Window Kinds (`KitAppWindowKind`):**

- `Table` - Tabular view of kit artifacts (types, designs, qualities, etc.)
- `Diagram` - Force-directed graph visualization of artifacts and relationships

**Diagram Relationships:**

- **Part-of**: Parent-child relationships (type/design parent, folder containment)
- **Reference**: Usage relationships (e.g., type referenced by design via pieces)

**State (`KitAppState`):**

- `panelVisibility` - Panel toggle states
- `selection` - Selected artifacts (types, designs, qualities, ports, tags, concepts, files, folders, authors)
- `hover` - Hovered artifact
- `filterSearch` - Search filter string
- `expandedRows` - Expanded table rows
- `sortColumn` / `sortDirection` - Sorting preferences
- `windowLayout` - Multi-window layout configuration

**Selection Types:** Types, designs, qualities, ports, tags, concepts, files, folders, authors

**Events:**

- `KIT.TOGGLE_PANEL` - Toggle panel visibility
- `KIT.SELECT_TYPE` / `KIT.DESELECT_TYPE` - Type selection
- `KIT.SELECT_DESIGN` / `KIT.DESELECT_DESIGN` - Design selection
- `KIT.SET_HOVER` - Set hover state
- `KIT.SET_FILTER_SEARCH` - Update search filter
- `KIT.SET_EXPANDED_ROWS` - Expand/collapse rows
- `KIT.CREATE_TYPE` / `KIT.CREATE_DESIGN` / `KIT.CREATE_QUALITY` - Create artifacts

**Hooks:**

- `useKitApp()` - Full kit app state
- `useKitAppSelection()` - Current selection
- `useKitAppHover()` - Hover state
- `useKitAppFilterSearch()` - Filter string
- `useKitAppWindowLayout()` - Window layout configuration

###### Type App (Type.tsx)

Type editing (connectors, models). Extends `KitDiffAppStore`.

**State (`TypeAppState`):**

- `panelVisibility` - Panel toggle states
- `activeTool` - Current tool (selection, etc.)
- `selection` - Selected connectors/models
- `hover` - Hovered connector/model
- `camera` - 3D camera state
- `focusedConnectorGuid` - Connector being edited
- `selectedModelGuid` - Active model
- `selectedModelTags` - Tags for model selection
- `fullscreenWindow` - Fullscreen mode
- `windowLayout` - Window arrangement

**Events:**

- `TYPE.TOGGLE_PANEL` - Toggle panel visibility
- `TYPE.SET_TOOL` - Change active tool
- `TYPE.SELECT_CONNECTOR` / `TYPE.DESELECT_CONNECTOR` - Connector selection
- `TYPE.SELECT_MODEL` / `TYPE.DESELECT_MODEL` - Model selection
- `TYPE.SET_HOVER` - Set hover state
- `TYPE.SET_CAMERA` - Update camera
- `TYPE.SET_SELECTED_MODEL_TAGS` - Model tag selection

**Hooks:**

- `useTypeApp()` - Full type app state
- `useTypeAppSelection()` - Current selection
- `useTypeAppHover()` - Hover state
- `useTypeAppCamera()` - Camera state
- `useTypeAppActiveTool()` - Active tool

###### Design App (Design.tsx)

Design editing (pieces, connections). Extends `KitDiffAppStore`.

**State (`DesignAppState`):**

- `panelVisibility` - Panel toggle states
- `activeTool` - Current tool (selection, connection, etc.)
- `selection` - Selected pieces/connections/connector
- `hover` - Hovered pieces/connections/connectors/types/designs
- `camera` - 3D camera state
- `diagramCenter` / `diagramScale` - 2D diagram view
- `focusedPieceGuid` - Piece being edited
- `selectedModelTags` - Model tags per type (`Record<Guid, string[]>`)
- `fullscreenWindow` - Fullscreen mode
- `windowLayout` - Window arrangement

**Selection Types:** Pieces, connections, connector (single connector selection for connection)

**Events:**

- `DESIGN.TOGGLE_PANEL` - Toggle panel visibility
- `DESIGN.SET_TOOL` - Change active tool
- `DESIGN.SELECT_PIECE` / `DESIGN.DESELECT_PIECE` - Piece selection
- `DESIGN.SELECT_CONNECTION` / `DESIGN.DESELECT_CONNECTION` - Connection selection
- `DESIGN.SET_HOVER` - Set hover state
- `DESIGN.SET_CAMERA` - Update 3D camera
- `DESIGN.SET_DIAGRAM_CENTER` / `DESIGN.SET_DIAGRAM_SCALE` - 2D diagram view
- `DESIGN.DELETE_SELECTED` - Delete selected elements
- `DESIGN.SET_SELECTED_MODEL_TAGS` - Model tag selection per type

**Commands:**

- `semio.designApp.selectAll` - Select all pieces and connections
- `semio.designApp.deselectAll` - Clear selection
- `semio.designApp.deleteSelected` - Delete selected elements

**Hooks:**

- `useDesignApp()` - Full design app state
- `useDesignAppSelection()` - Current selection
- `useDesignAppHover()` - Hover state
- `useDesignAppCamera()` - 3D camera
- `useDesignAppActiveTool()` - Active tool
- `useDesignAppDiagramCenter()` / `useDesignAppDiagramScale()` - Diagram view

**HoverIntentContext:**

Design app uses `HoverIntentContext` to manage hover/pan/drag state via refs instead of module-level variables:

- `hoverClearTimeoutRef` - Timeout for clearing hover state
- `currentHoveredPieceGuidRef` - Currently hovered piece GUID
- `isPanningRef` - Whether user is panning the canvas
- `isDraggingNodeRef` - Whether user is dragging a node

Access via `useHoverIntent()` hook within `HoverIntentProvider` scope.

**Derived State Providers:**

- `TransactionPiecesProvider` - Provides `changedPieces` Set and `statusMap` Map for pieces affected by current transaction
- `HoverPiecesProvider` - Provides `transitivelyHoveredPieces` and `transitivelyHoveredTypes` for hover highlighting

Both use `useSyncExternalStore` with structural equality helpers (`areSetsEqual`, `areMapsEqual`) instead of JSON.stringify diffing.

###### Quality App (Quality.tsx)

Quality/benchmark editing with formula visualization. Extends `KitDiffAppStore`.

**State (`QualityAppState`):**

- `panelVisibility` - Panel toggle states
- `activeTool` - Current tool
- `selection` - Selected formula nodes
- `hover` - Hovered formula node
- `formulaNodes` - Parsed formula tree
- `fullscreenWindow` - Fullscreen mode
- `windowLayout` - Window arrangement

**Formula Functions:** Numeric (Add, Subtract, Multiply, Divide, ...), Branching (If, Switch, ...), Data (Min, Max, Avg, ...), Text, Comparison

**Events:**

- `QUALITY.TOGGLE_PANEL` - Toggle panel visibility
- `QUALITY.SET_TOOL` - Change active tool
- `QUALITY.SELECT_FORMULA_NODE` / `QUALITY.DESELECT_FORMULA_NODE` - Node selection
- `QUALITY.SET_HOVER` - Set hover state

**Hooks:**

- `useQualityApp()` - Full quality app state
- `useQualityAppSelection()` - Current selection
- `useQualityAppHover()` - Hover state

###### Docs App (Docs.tsx)

In-app documentation viewer with MDX support.

**Features:**

- MDX file loading from `./pages/**/*.mdx`
- Section-based navigation
- Heading extraction for table of contents
- Tab components for content organization

**MDX Loading:**

- `loadMDXFile(path)` - Load single MDX file
- `getAllMDXFiles()` - List all MDX files
- `getMDXFilesBySection(section)` - Files in a section
- `getAllSections()` - All available sections

**Heading State:**

- `useHeadings()` - Subscribe to heading updates
- `headingsState.registerHeading(id, level, text)` - Register heading
- `headingsState.setActiveHeading(id)` - Set active heading

###### Feedback App (Feedback.tsx)

Bug report and feature idea submission form. State managed via XState triadic hooks.

**Route:** `/feedback`

**State (`FeedbackAppState` in Sketchpad.tsx):**

- `panelVisibility` - Panel toggle states
- `formData` - Form data (kind, title, description, app, name, email)
- `isSubmitting` - Form submission in progress
- `isSubmitted` - Form successfully submitted
- `error` - Error message if submission failed

**Form Kinds (`FeedbackKind`):**

- `bug` - Bug report (requires app selection)
- `idea` - Feature idea

**Triadic Hooks:**

- `useFeedbackFormData()` - `[formData, setFormData, canSet]`
- `useFeedbackIsSubmitting()` - `[isSubmitting, setIsSubmitting, canSet]`
- `useFeedbackIsSubmitted()` - `[isSubmitted, setIsSubmitted, canSet]`
- `useFeedbackError()` - `[error, setError, canSet]`
- `useFeedbackReset()` - `[reset, canReset]`

**Events:**

- `FEEDBACK.TOGGLE_PANEL` - Toggle panel visibility
- `FEEDBACK.SET_FORM_DATA` - Update form fields
- `FEEDBACK.RESET_FORM` - Reset form to initial state
- `FEEDBACK.SET_SUBMITTING` - Set submitting state
- `FEEDBACK.SET_SUBMITTED` - Set submitted state
- `FEEDBACK.SET_ERROR` - Set error message

**Global Footer Action:**

The feedback icon appears in every app's footer via `GlobalFooterItems` component in Sketchpad.tsx, providing universal access to the feedback form.

####### Adding a New Tool

To add a new tool to an app:

1. Create a `*Tool.tsx` file directly inside `js/semio/sketchpad/`.
2. Export a `Tool<AppState>` object with a unique `id` and `render` implementation.

Each app loads sibling `*Tool.tsx` modules via `import.meta.glob('./*Tool.tsx', { eager: true })`, so simply dropping the file in place registers it.

Example:

```typescript
export const MyTool: Tool<MyAppState> = {
  id: ToolKind.MY_TOOL,
  label: "My Tool",
  icon: <Icon />,
  render: (context) => ({ scene: <></>, diagram: null, table: null }),
};
```

####### Adding Panel Sections

Panel sections are dynamically added in the app's `useEffect`:

```typescript
useEffect(() => {
  removeSection("details", "my-section");
  addSection("details", {
    id: "my-section",
    label: t("mySection"),
    content: () => <MyComponent />,
    order: 1,
  });
  return () => removeSection("details", "my-section");
}, [appType, addSection, removeSection]);
```

Policies:

1. When a section id is conditional (for example `"properties"` vs `"multipleTitle"`), always `removeSection` for all possible ids before adding the currently active one.
2. Always `removeSection` for every id you `addSection` (including conditional variants) in the effect cleanup.
3. If the section content uses scope-bound hooks (`useKit()`, `useDesign()`, `useType()`), wrap `content` with the corresponding `*ScopeProvider` when registering the section.

####### Tutorials

The tutorial system is consolidated in `js/semio/sketchpad/Tutorials.tsx` and is split into regions for types, store, commands, built-in tutorials, and UI components. `TutorialStore` wraps a Y.js map and keeps playback, milestone ordering, and recording state (`TutorialPlaybackState`, `TutorialRecordingState`). Always create the store with the app transaction handler so tutorial mutations participate in undo/redo.

Wrap consumers in `TutorialProvider` and use the helper hooks (`useTutorialStore`, `useActiveTutorial`, `useTutorialProgress`, `useTutorialCommandInterceptor`, etc.) instead of accessing the store directly. `TutorialControls`, `RecordingControls`, `RecordButton`, and `TutorialOverlay` are the canonical UI integrations for playback, recording, highlighting, and capture.

Tutorial commands are consolidated in `Tutorials.tsx` under the `tutorialCommands` and `devCommands` objects for the `semio.tutorial.*` and `semio.recording.*` namespaces. Bundle reusable walkthroughs or recordings as data objects (for example `helloTutorial`, `sketchpadTour`) and register them with `addTutorial`.

All tutorial-related code (types, store, commands, UI components, and built-in tutorials) is now in a single file using regions for organization instead of being spread across multiple files in a separate folder.

####### Footer

`FooterItemProvider` wraps `Sketchpad` so apps can register footer entries with `useAddFooterItem` and remove them via `useRemoveFooterItem`; the provider keeps items ordered by the optional `order` field.

Register items inside effects and always call the remove helper in the cleanup; default contributions now live inside each app's `App.tsx`, next to the `config` export.

Providing an `id` shows the translated `DescriptionTooltipContent`, and the base footer auto-hides in fullscreen until the cursor nears the bottom edge, so interactive elements must tolerate that visibility change.

The shared `Footer` component has a fixed `h-medium` height.

##### Styling

- NEVER use colors and spacing directly. ALWAYS use semantic variables from `global.css`. Only `global.css` uses colors and pixels directly.
- NEVER add semantic values and ALWAYS use hardcoded values in `theme.css`. NEVER use `theme.css` outside of `global.css`.
- ALWAYS use the standardized unit-based sizing system defined in globals.css (derived from `--spacing`):
  - Single: 1 unit - spacing between elements and between icon and element (e.g. `gap-1`)
  - Tiny: 3 units - icon size in actions, action text size (e.g. `h-tiny`, `w-tiny`, `text-tiny`)
  - Small: 5 units - actions, avatars, Strip items (e.g. `h-small`, `w-small`)
  - Medium: 7 units - buttons, toggles, inputs, sliders, steppers, Footer, table rows, Strip (e.g. `h-medium`, `w-medium`)
  - Large: 9 units - Band, Navbar (e.g. `h-large`, `w-large`)
  - Huge: 11 units - height of navigation buttons at bottom of docs pages (e.g. `h-11`)
  - Mega: 13 units - width of toggles with actions (toggles with dropdown or action buttons) (e.g. `w-mega`)
  - Giga: 15 units - reserved for future use (e.g. `w-giga`)
- Table body cells MUST NOT add vertical padding; `Table` centers cell content and uses `px-single py-0` so `h-medium` rows stay fixed even when rendering `h-medium` controls.

##### Store Architecture

This document describes the generalized store hierarchy for the semio application.

#### Overview

The store architecture consists of three levels of abstraction:

1. **Store** - Base class for any component with data
2. **AppStore** - Base class for apps with transaction support and undo/redo
3. **KitDiffAppStore** - Base class for apps that modify kits and track both app-specific and kit diffs

#### Store Hierarchy

```
Store<TState>
  ↓ extends
AppStore<TState, TDiff, TSelectionDiff, TEdit, TCommandContext, TCommandResult>
  ↓ extends
KitDiffAppStore<TState, TDiff, TSelectionDiff, TEdit, TCommandContext, TCommandResult>
```

#### 1. Store (Base Class)

The `Store` class is the foundation for all components that hold data.

##### Responsibilities

- State management with snapshot caching
- Observable pattern (onChanged, onChangedDeep)
- Access to parent SketchpadStore
- Y.js integration via yMap

##### Abstract Methods

- `hash(state: TState): string` - Generate a hash for cache invalidation
- `buildSnapshot(): TState` - Build the current state snapshot

##### Usage

Use this for simple components that only need state management without editing capabilities (e.g., HomeStore).

#### 2. AppStore (extends Store)

The `AppStore` adds transaction support with undo/redo functionality for any app.

##### Responsibilities

- Transaction management (start, finalize, abort)
- Undo/redo with two stacks:
  - **Current transaction stack**: Edits in the active transaction (merged on finalize)
  - **Past transactions stack**: Finalized transactions
- Selection management with diff-based updates
- Panel visibility and fullscreen management

##### Transaction Model

Every app supports transactions:

1. **Start Transaction**: `startTransaction()`
   - Activates transaction mode
   - New edits go to current transaction stack

2. **During Transaction**: `executeCommand(...)`
   - Each command creates an edit with `do` and `undo` steps
   - Edits accumulate in current transaction stack
   - Undo/redo work within the current transaction

3. **Finalize Transaction**: `finalizeTransaction()`
   - Merges all edits in current transaction into one edit
   - Moves merged edit to past transactions stack
   - Clears redo stack

4. **Abort Transaction**: `abortTransaction()`
   - Undoes all edits in current transaction
   - Clears current transaction stack

##### UI Transaction Context (Sketchpad elements)

Sketchpad UI elements resolve transactions via React context (not props):

- `js/semio/sketchpad/elements.tsx` defines `TransactionProvider` and `useTransaction()`.
- `js/semio/sketchpad/elements.tsx` `Geometry` treats `color` as the base (non-interactive) color and uses selection/hover theme colors for the rendered material/edges when `selected`/`hovered` are true.
- `js/semio/sketchpad/Design.tsx` diagram piece nodes use non-inset rings (`ring-*`, not `ring-inset`) so rings remain visible on `Avatar` nodes with full-size `AvatarFallback` backgrounds.
- Elements such as `Input`, `Textarea`, `Select`, `Slider`, `Stepper`, `Combobox`, and `ActionDropdown` call `useTransaction()` internally and do not accept a `transaction` prop.
- Apps are responsible for scoping transactions by wrapping their UI subtree with `TransactionProvider` using the appropriate transaction hook (per-app or kit-level), so all descendant elements participate consistently.

##### Hooks and Helpers

- **`useSync` / `useSyncDeep`** (from `js/semio/sketchpad/Sketchpad.tsx`) wrap `useSyncExternalStore` against a store's `onChanged` / `onChangedDeep` events. Pass a selector (defaults to `identitySelector`) to scope renders to the slice you need.
- **`useSyncField` / `useSyncFields`** subscribe to Y.js-backed store fields with optional `comparator?: (a: TSelected, b: TSelected) => boolean` parameter for custom equality checks instead of JSON.stringify. Use for Set/Map values or other complex types.
- **`createObserver`** bridges a Y.js map or array into the store by registering either shallow or deep observers; always dispose the returned cleanup in `useEffect` finalizers.
- **`RemoteProviders`** bundles the `yProvider` and `fileProvider` factories needed when constructing `SketchpadStore` so persistence and external file access stay aligned.

##### Edit Structure

```typescript
interface AppEdit<TSelectionDiff> {
  do: AppStep<TSelectionDiff>;
  undo: AppStep<TSelectionDiff>;
}

interface AppStep<TSelectionDiff> {
  selectionDiff?: TSelectionDiff;
}
```

Each edit stores:

- **do**: Forward diff to apply the change
- **undo**: Inverse diff to revert the change

#### Abstract Methods (in addition to Store)

- `applySelectionDiff(selectionDiff: TSelectionDiff): void` - Apply selection changes to Y.js
- `inverseSelectionDiff(selection, diff): TSelectionDiff` - Calculate inverse diff for undo
- `getSelection()` - Get current selection state

##### Undo/Redo Behavior

**Within Transaction:**

- Undo: Pops from current transaction stack, stores in temp variable
- Redo: Pushes temp variable back to current transaction stack

**Outside Transaction:**

- Undo: Moves edit from past transactions stack to redo stack
- Redo: Moves edit from redo stack back to past transactions stack

##### Usage

Use this for apps that don't modify kits (e.g., HomeStore for managing the home screen).

#### 3. KitDiffAppStore (extends AppStore)

The `KitDiffAppStore` extends AppStore for apps that modify kits (designs, types).

##### Additional Responsibilities

- Tracks kit diffs alongside app-specific diffs
- Applies kit changes through KitStore
- Records both app and kit changes in edits

##### Edit Structure

```typescript
interface KitDiffAppEdit<TSelectionDiff> {
  do: KitDiffAppStep<TSelectionDiff>;
  undo: KitDiffAppStep<TSelectionDiff>;
}

interface KitDiffAppStep<TSelectionDiff> {
  kitDiff?: KitDiff;
  selectionDiff?: TSelectionDiff;
}
```

Each edit stores:

- **do.kitDiff**: Forward kit diff to apply changes
- **do.selectionDiff**: Forward selection diff
- **undo.kitDiff**: Inverse kit diff to revert changes
- **undo.selectionDiff**: Inverse selection diff

##### Undo/Redo Behavior

Extends AppStore undo/redo to also:

- Apply/revert kit diffs through `kit().change(kitDiff)`
- Handle both kit and selection changes atomically

##### Abstract Methods

- `kit(): KitStore` - Get the associated kit store

##### Usage

Use this for apps that modify kits:

- **DesignAppStore** - Edit designs (pieces, connections)
- **TypeAppStore** - Edit types (connectors, models)
- **KitAppStore** - Edit kits (types, designs, qualities, files, authors)

#### Concrete Implementations

##### DesignAppStore

Edits design content:

- Selection: pieces, connections, connectors
- Kit diffs: piece changes, connection changes
- Transaction support for complex multi-step operations

##### TypeAppStore

Edits type definitions:

- Selection: connectors, models
- Kit diffs: connector changes, model changes
- Transaction support for type modifications

##### KitAppStore

Edits kit metadata:

- Selection: types, designs, qualities, files, authors
- Kit diffs: add/remove artifacts
- Transaction support for kit-level operations

##### HomeStore

Manages home screen (extends AppStore, not KitDiffAppStore):

- Selection: kits
- No kit diffs (doesn't modify kit content)
- Sorting and filtering state

#### Command Pattern

All apps use a command pattern:

```typescript
interface CommandContext {
  // Current state
}

interface CommandResult {
  diff?: TDiff;      // App-specific diff
  kitDiff?: KitDiff; // Kit diff (only for KitDiffAppStore)
}

executeCommand<T>(command: string, ...args): Promise<T>
```

##### Command Execution Flow

1. Look up command in registry
2. Build context with current state
3. Execute command function
4. Apply diffs (app diff + kit diff)
5. Record edit for undo/redo
6. Return result

#### Best Practices

1. **Always use transactions** for multi-step operations
2. **Keep edits atomic** - each edit should be independently undoable
3. **Calculate inverse diffs correctly** - critical for undo
4. **Don't nest transactions** - finish one before starting another
5. **Clear redo stack on new edits** - standard undo/redo behavior
6. **Use selection diffs** for all selection changes

#### Files

- `js/semio/sketchpad/Sketchpad.tsx` - Base Store, AppStore, KitDiffAppStore, SketchpadStore, KitStore
- `js/semio/sketchpad/Design.tsx` - DesignAppStore and design app state
- `js/semio/sketchpad/Type.tsx` - TypeAppStore and type toolchain
- `js/semio/sketchpad/Quality.tsx` - QualityAppStore and quality workflows
- `js/semio/sketchpad/Kit.tsx` - KitAppStore and kit command wiring
- `js/semio/sketchpad/Home.tsx` - HomeStore and home experience
- `js/semio/sketchpad/Docs.tsx` - DocsAppStore and documentation app
- `js/semio/sketchpad/Tutorials.tsx` - Tutorial system (consolidated)
- `js/semio/sketchpad/shared.ts` - Shared types and utilities

#### Kit app artifact creation

- `js/semio/sketchpad/Kit.tsx` create actions for `ports`, `tags`, `concepts`, and `folders` set the active `kind` filter and selection to the newly created entity.
- Default names are resolved via i18n labels: `semio.sketchpad.app.port.defaultName`, `semio.sketchpad.app.tag.defaultName`, `semio.sketchpad.app.concept.defaultName`.

#### XState State Machines

The application uses XState v5 for all Sketchpad UI state. Y.js is reserved for collaborative Kit data.

#### Architecture

- **XState actor** is the source of truth for Sketchpad UI state (`SketchpadState` + app slices).
- **Local persistence**: Sketchpad UI state is written to `localStorage` at `semio.sketchpad.state.<id>`.
- **Y.js** is used only for Kit data (per-kit `KitStore` documents, optionally connected via `RemoteProviders.yProvider`).
- **React hooks** read via `@xstate/react` `useSelector` and write via `actor.send({ type: ... })`.

#### Machine Files

**`Sketchpad.tsx`** contains the main machines:

##### sketchpadMachine

Unified state machine combining data management and hierarchical navigation:

**Root Structure (parallel):**

- Sketchpad UI state lives in the machine context (`SketchpadState` + app slices)
- `navigation` parallel state with hierarchical sub-states

**Navigation States:**

- `home` → `kit` → `design`/`type`/`quality`/`docs`
- State transitions via `KIT.INIT`, `DESIGN.INIT`, `TYPE.INIT` events

**State-Scoped Events:**

App-specific events are only available in their respective navigation states:

- **home**: `HOME.TOGGLE_PANEL`, `HOME.SET_HOVER`, `HOME.SELECT_KIT`, etc.
- **kit**: `KIT.SYNC`, `KIT.TOGGLE_PANEL`, `KIT.SET_FILTER`, `KIT.SELECT_TYPE`, etc.
- **design**: `DESIGN.SYNC`, `DESIGN.SET_HOVER`, `DESIGN.SELECT_PIECE`, `DESIGN.DELETE_SELECTED`, etc.
- **type**: `TYPE.SYNC`, `TYPE.SET_HOVER`, `TYPE.SELECT_CONNECTOR`, `TYPE.HOVER_MODEL`, etc.
- **quality**: `QUALITY.TOGGLE_PANEL`, `QUALITY.TOGGLE_BENCHMARK`

**Global Events (always available):**

- Navigation: `NAVIGATE`, `NAVIGATE_BACK`, `NAVIGATE_FORWARD`
- Settings: `SET_THEME`, `SET_LANGUAGE`, `SET_EXPERTISE`, `SET_MODE`, `SET_DEVICE`
- Background operations: `BACKGROUND.START`, `BACKGROUND.COMPLETE`, `BACKGROUND.FAIL`
- Tutorial: `TUTORIAL.START`, `TUTORIAL.END`, `TUTORIAL.NEXT_STEP`, etc.
- Sketchpad state updates: `CHANGE`

**Per-App Transaction Events (scoped to navigation state):**

Transaction management is per-app, not global. Each app (Design, Type, Kit) has its own transaction state embedded in its app state port.

- **design**: `DESIGN.TRANSACTION.START`, `DESIGN.TRANSACTION.COMMIT`, `DESIGN.TRANSACTION.ABORT`, `DESIGN.TRANSACTION.UNDO`, `DESIGN.TRANSACTION.REDO`, `DESIGN.TRANSACTION.RECORD_EDIT`
- **type**: `TYPE.TRANSACTION.START`, `TYPE.TRANSACTION.COMMIT`, `TYPE.TRANSACTION.ABORT`, `TYPE.TRANSACTION.UNDO`, `TYPE.TRANSACTION.REDO`, `TYPE.TRANSACTION.RECORD_EDIT`
- **kit**: `KIT.TRANSACTION.START`, `KIT.TRANSACTION.COMMIT`, `KIT.TRANSACTION.ABORT`, `KIT.TRANSACTION.UNDO`, `KIT.TRANSACTION.REDO`, `KIT.TRANSACTION.RECORD_EDIT`

**Navigation State Selectors:**

```typescript
import { selectNavigationState, selectIsInDesign, selectIsInType } from "./Sketchpad";

// Check current navigation state
const navState = useSelector(actor, selectNavigationState); // "home" | "kit" | "design" | "type" | "quality" | "docs"
const isInDesign = useSelector(actor, selectIsInDesign); // boolean
```

**Constraint Enforcement:**

- `DESIGN.DELETE_SELECTED` requires `hasDesignSelection` guard AND being in design state
- App-specific events are silently ignored when not in the correct navigation state
- This prevents invalid state transitions (e.g., selecting a piece when not in design view)

##### uiMachine (legacy)

Separate hierarchical UI state machine (kept for reference, functionality merged into sketchpadMachine):

- `interaction` region: Idle → Hovered → Selected → ContextMenu substates
- `tool` region: Active tool state (Design/Type apps)
- `drag` region: Drag-and-drop state (Design app)
- `modal` region: Command palette and search overlays

#### XState Hooks

**`Sketchpad.tsx`** provides XState-based hooks:

- `useSketchpadActor()` - Get the XState actor ref
- `useSketchpadSelector()` - Generic selector using @xstate/react
- `useSketchpadSnapshot()` - Full state snapshot
- `useSketchpadActions()` - Event dispatching functions
- App-specific hooks: `useThemeXState()`, `useModeXState()`, etc.

#### Y.js-XState Bridge

**`shared.ts`** contains bridge utilities:

- `createYjsSyncActor()` - Creates callback actor for Y.js observation
- `createYjsFieldSyncActor()` - Single field observation
- `yTransact()` - Transaction wrapper
- `createYjsUpdateAssign()` - Assign action for Y_UPDATE events
- `createYjsSelector()` - Cached selector with dirty checking

#### State ownership

- Sketchpad UI state (navigation/settings/panel sizes and per-app UI slices) is owned by `sketchpadMachine` context and exposed through XState selectors.
- Kit data is owned by per-kit Y.js documents (`KitStore`) and accessed via kit-level stores/hooks.

#### Transaction State Management

Transaction state is embedded in each app's state port via `AppTransactionState`:

```typescript
interface AppTransactionState<TEdit = any> {
  isTransactionActive: boolean;
  currentTransactionStack: TEdit[]; // Edits in current active transaction
  pastTransactionStack: TEdit[]; // Finalized transactions (for undo)
  redoStack: TEdit[]; // Undone transactions (for redo)
}
```

**Transaction Flow:**

1. **Start**: `APP.TRANSACTION.START` activates transaction mode, clears redo stack
2. **Record Edit**: `APP.TRANSACTION.RECORD_EDIT` pushes edit to current stack
3. **Commit**: `APP.TRANSACTION.COMMIT` merges current stack into one edit, moves to past stack
4. **Abort**: `APP.TRANSACTION.ABORT` discards current stack, deactivates transaction mode
5. **Undo**: `APP.TRANSACTION.UNDO` pops from current (if active) or past stack
6. **Redo**: `APP.TRANSACTION.REDO` moves edit from redo back to past stack

**Background Operations:**

Long-running async operations (kit import, file upload) are tracked via `backgroundOperations`:

```typescript
backgroundOperations: Record<
  string,
  {
    type: string;
    status: "pending" | "running" | "completed" | "failed";
    error?: string;
  }
>;
```

These continue even when navigating away from the originating app.

#### Command System

All state mutations are executed through commands. Commands provide a consistent port for operations and enable undo/redo, logging, and origin tracking.

#### Command Registry

Each store maintains a `commandRegistry` that maps command strings to handler functions. Commands are registered using `registerCommand` and unregistered using `unregisterCommand`.

#### Command Execution

Commands are executed via `executeCommand(command: string, ...args: any[])`:

1. **Origin Extraction**: If the first argument is a string starting with `semio.sketchpad.`, it's treated as the origin (UI element ID). Otherwise, origin is undefined.
2. **Command Lookup**: The command registry is searched for the handler.
3. **Context Building**: A command context is built with current state snapshot.
4. **Handler Execution**: The handler receives context and remaining arguments.
5. **Diff Application**: Result diffs are applied to the store.
6. **Edit Recording**: For AppStore/KitDiffAppStore, edits are recorded for undo/redo.

#### Command Naming Convention

Commands follow the pattern `semio.{scope}.{action}`:

- `semio.sketchpad.*` - Sketchpad-level commands
- `semio.kitApp.*` - Kit app commands
- `semio.designApp.*` - Design app commands
- `semio.typeApp.*` - Type app commands
- `semio.home.*` - Home app commands

Special commands:

- `semio.{app}.startTransaction` - Start a transaction
- `semio.{app}.finalizeTransaction` - Finalize current transaction
- `semio.{app}.abortTransaction` - Abort current transaction
- `semio.{app}.undo` - Undo last edit
- `semio.{app}.redo` - Redo last undone edit

#### Command Origin

Every command execution should include an origin string identifying the UI element that triggered it. Origins follow the pattern `semio.sketchpad.{path}` matching the element's `id` prop. This enables:

- Debugging and logging
- Tutorial recording
- Analytics tracking

### Diff System

The diff system tracks changes to models for undo/redo, synchronization, and persistence.

#### Diff Types

Every model has an associated `Diff` type that represents partial changes:

- **ModelDiff**: Partial update to a single model instance
- **ModelsDiff**: Collection diffs with `removed`, `updated`, and `added` arrays

#### Diff Operations

Each model type supports four diff operations:

1. **`getDiff(before, after): Diff`** - Calculate diff between two states
2. **`inverseDiff(original, appliedDiff): Diff`** - Calculate inverse diff for undo
3. **`mergeDiff(diff1, diff2): Diff`** - Merge two diffs (later takes precedence)
4. **`applyDiff(base, diff): Model`** - Apply diff to base state

#### Diff Status

Diffs track status using `DiffStatus` enum:

- `Unchanged` - No change
- `Added` - Newly added item
- `Removed` - Deleted item
- `Modified` - Updated item

#### Collection Diffs

Collection diffs (`*sDiff`) track changes to arrays/lists:

```typescript
interface CollectionDiff<T> {
  removed?: TId[]; // IDs of removed items
  updated?: { id: TId; diff: TDiff }[]; // Updated items with their diffs
  added?: T[]; // Newly added items
}
```

#### Inverse Diffs

Inverse diffs enable undo by reversing operations:

- `removed` → `added` (restore removed items)
- `added` → `removed` (remove added items)
- `updated` → inverse of the update diff

### Routing & App Registration

Apps are registered via the `AppRegistry` which auto-discovers apps using `import.meta.glob('./*/App.tsx')`.

#### AppConfig

Each app exports a `config: AppConfig`:

```typescript
interface AppConfig {
  id: string; // Unique app identifier
  component: ComponentType; // React component
  routeSegments: RouteSegment[]; // Route path segments
  getPanels: (t: TFunction) => PanelDefinition[]; // Panel definitions
  matchesPath: (pathParts: string[]) => boolean; // Path matcher
  order?: number; // Display order
}
```

#### Route Segments

Route segments define the app's URL structure:

```typescript
interface RouteSegment {
  path: string; // React Router path pattern
  paramName?: string; // Parameter name (e.g., "id")
  scopeProvider?: ComponentType<{ guid: string; children: ReactNode }>; // Scope wrapper
}
```

#### Path Matching

Apps can match paths using `matchesPath(pathParts: string[])`. The registry searches apps in order and returns the first match.

#### Scope Providers

Scope providers wrap app components to provide context (e.g., kit/design/type GUIDs) via React Router params.

### Hotkeys

Hotkeys are configurable keyboard shortcuts stored in the SketchpadStore with user overrides.

#### Hotkey Paths

Hotkey paths follow the pattern `semio.sketchpad.{element.path}` matching UI element IDs. This enables:

- Automatic tooltip display
- Settings UI integration
- Tutorial highlighting

#### Hotkey Values

Hotkeys use the format from `react-hotkeys-hook`:

- `mod+k` - Meta/Ctrl + K
- `shift+alt+d` - Shift + Alt + D
- `escape` - Escape key

#### Hotkey Overrides

Users can override default hotkeys via `hotkeyOverrides` in SketchpadStore. Overrides take precedence over defaults.

#### Hotkey Hooks

- `useHotkey(path, callback, deps)` - Register hotkey handler (from `js/semio/sketchpad/Sketchpad.tsx`)
- `useSetHotkey()` - Set hotkey override
- `useResetHotkey()` - Reset hotkey to default
- `useResetAllHotkeys()` - Reset all overrides

### Core Types (shared.ts)

The `shared.ts` module exports all core types, enums, and ports used across the Sketchpad.

#### Hook Result Types

All hooks follow the triadic pattern returning `[value, setter, canSet]`:

```typescript
type HookResult<T> = readonly [T, ((value: T) => void) | undefined, boolean];
type HookNoSetResult<T> = readonly [T, undefined, boolean];
```

**Helper Functions:**

- `readonlyHookResult(value)` - Create read-only result
- `writableHookResult(value, setter, canSet?)` - Create writable result
- `conditionalHookResult(canSet, value, setter)` - Create conditional result

#### Field<T> Type

Alternative object-based pattern with always-defined `set` function (no-op when disabled):

```typescript
interface Field<T> {
  value: T;
  canSet: boolean;
  set: (next: T) => void;
}

interface ActionField {
  canExecute: boolean;
  execute: () => void;
}
```

**Helper Functions:**

- `createField(value, setter, canSet)` - Create writable field
- `createReadonlyField(value)` - Create read-only field
- `createAction(execute, canExecute)` - Create action field
- `fieldToHookResult(field)` - Convert Field to HookResult
- `hookResultToField(result)` - Convert HookResult to Field

**XState Helpers (Sketchpad.tsx):**

- `useXStateField(value, canEvent, createEvent)` - Create Field from XState selector
- `useXStateFieldWithScope(value, canEvent, createEvent, hasScope)` - With wildcard fallback
- `useXStateAction(canEvent, event)` - Create ActionField from XState event

**App-Level Helper Pattern (Design.tsx):**

```typescript
interface UseDesignAppFieldOptions<T, TEvent> {
  selector: (s: DesignAppState) => T;
  fallback: T;
  canEventType: TEvent["type"];
  createCanEvent: (kitGuid: Guid, designGuid: Guid) => TEvent;
  createSendEvent: (kitGuid: Guid, designGuid: Guid, value: T) => TEvent;
  useWildcardFallback?: boolean;
}

function useDesignAppField<T, TEvent>(options: UseDesignAppFieldOptions<T, TEvent>): Field<T>;
```

#### Core Enums

```typescript
enum Theme {
  SYSTEM = "system",
  LIGHT = "light",
  DARK = "dark",
}
enum Expertise {
  BEGINNER = "beginner",
  NORMAL = "normal",
  EXPERT = "expert",
}
enum Mode {
  USER = "user",
  DEV = "dev",
}
enum StoreStatus {
  IDLE = "idle",
  LOADING = "loading",
  ERROR = "error",
  READY = "ready",
}
enum ToolKind {
  SELECTION_NORMAL,
  SELECTION_ADDITIVE,
  SELECTION_SUBTRACTIVE,
  LASSO_RECTANGULAR,
  LASSO_FREEFORM,
  CONNECTOR,
}
enum WindowKind {
  TABLE = "table",
  SCENE = "scene",
  DIAGRAM = "diagram",
  CUSTOM = "custom",
}
enum PanelPosition {
  LEFT = "left",
  RIGHT = "right",
  MIDDLE = "middle",
  BOTTOM = "bottom",
}
enum PanelKind {
  WORKBENCH,
  TOOLS,
  TOOLBAR,
  HUD,
  STATS,
  DETAILS,
  CHAT,
  SETTINGS,
  PARAMS,
}
```

#### Panel System

Panels are configured via `PanelKind` with predefined positions and behaviors:

```typescript
interface PanelKindConfig {
  icon: ComponentType<{ size?: number }>;
  position: PanelPosition;
  group?: string;
  isTransparent?: boolean;
  isGroupable?: boolean;
  hotkey?: string;
}

interface PanelVisibility {
  toolbar?: boolean;
  workbench?: boolean;
  tools?: boolean;
  hud?: boolean;
  stats?: boolean;
  details?: boolean;
  chat?: boolean;
  settings?: boolean;
  params?: boolean;
}

interface PanelSection {
  id: string;
  content: ReactNode | (() => ReactNode);
  specificity?: number;
  defaultOpen?: boolean;
  order?: number;
  actions?: Array<{ id: string; icon: ReactNode; onClick: () => void }>;
}
```

**Panel Positioning:**

- **LEFT**: Workbench, Tools (grouped)
- **RIGHT**: Details, Chat, Settings (grouped)
- **MIDDLE**: HUD, Stats (grouped, transparent)
- **BOTTOM**: Toolbar

#### Tool System

Tools define interaction modes within apps:

```typescript
interface Tool<TState = any> {
  id: ToolKind | string;
  icon?: ReactNode;
  render: (context: ToolRenderContext<TState>) => { scene?: ReactNode; diagram?: ReactNode | null; table?: ReactNode | null };
}

interface ToolMode {
  id: string;
  icon?: ReactNode;
  label?: string;
  tooltipId?: string;
}

interface ToolDefinition {
  id: string;
  defaultMode: ToolKind | string;
  modes: ToolMode[];
}
```

#### App IDs

Each app has a typed ID structure:

```typescript
interface KitAppId {
  kit: Guid;
}
interface TypeAppId {
  kit: Guid;
  type: Guid;
}
interface DesignAppId {
  kit: Guid;
  design: Guid;
}
interface QualityAppId {
  kit: Guid;
  quality: Guid;
}
```

### YPath and DerivedStore

YPath provides granular subscriptions to nested Y.js structures. DerivedStore caches computed values.

#### YPath

Navigate Y.js structures with path segments:

```typescript
type YPathSegment = { kind: "mapKey"; key: string } | { kind: "arrayIndex"; index: number } | { kind: "arrayItemById"; id: string; idKey: string };

type YPath = YPathSegment[];
```

**Path Helpers:**

- `yPathMapKey(key)` - Access a Y.Map key
- `yPathArrayIndex(index)` - Access a Y.Array index
- `yPathArrayItemById(id, idKey?)` - Find array item by ID

**Usage:**

```typescript
const path = [yPathMapKey("pieces"), yPathArrayItemById(pieceGuid, "guid")];
const value = getValueAtPath(yMap, path);
```

#### DerivedStore

Caches computed values that depend on Y.js paths:

```typescript
class DerivedNode<T> {
  snapshot(): T;
  subscribe(cb: () => void): Disposable;
  dispose(): void;
}

class DerivedStore {
  getOrCreate<T>(key: string, deps: BaseDependency[], compute: () => T): DerivedNode<T>;
  get<T>(key: string): DerivedNode<T> | undefined;
  delete(key: string): boolean;
  clear(): void;
}
```

**Usage:**

```typescript
const piecesMetadataNode = derivedStore.getOrCreate("piecesMetadata", [{ store: designStore, path: [yPathMapKey("pieces")] }], () => computePiecesMetadata(designStore.snapshot()));
```

### App Plugin Registry

Apps register plugins that contribute event handlers, guards, and state factories.

#### AppPlugin Port

```typescript
interface AppPlugin {
  id: string; // e.g., "home", "kit", "design"
  namespace: string; // e.g., "HOME", "KIT", "DESIGN"
  machine: AppMachineContribution;
  registerStores?: () => void;
  onRegister?: () => void;
}

interface AppMachineContribution {
  actions?: Record<string, (context: any, event: any) => any>;
  guards?: Record<string, (context: any, event: any) => boolean>;
  eventHandlers?: Record<string, { guard?: string; actions?: string | string[] }>;
  selectors?: Record<string, (context: any, ...args: any[]) => any>;
  createDefaultState?: () => any;
}
```

#### Registration Functions

- `registerAppPlugin(plugin)` - Register an app plugin
- `getAppPlugins()` - Get all registered plugins
- `getAppPlugin(id)` - Get plugin by ID
- `hasAppPlugin(id)` - Check if plugin exists
- `composePluginContributions()` - Merge all plugin contributions

#### Event Handler Registry

Dynamic event dispatch for app-specific events:

```typescript
interface EventHandlerConfig<TContext = any, TEvent = any> {
  guard?: (context: TContext, event: TEvent) => boolean;
  action: (context: TContext, event: TEvent) => Partial<TContext>;
}
```

**Registration:**

```typescript
registerEventHandler("HOME.TOGGLE_PANEL", {
  action: (context, event) => ({
    homeApp: {
      ...context.homeApp,
      panelVisibility: { ...context.homeApp.panelVisibility, [event.panel]: !context.homeApp.panelVisibility[event.panel] },
    },
  }),
});
```

**Functions:**

- `registerEventHandler(eventType, config)` - Register handler
- `unregisterEventHandler(eventType)` - Remove handler
- `executeEventHandler(context, event)` - Execute handler
- `getEventTypesForNamespace(namespace)` - List events for namespace
- `getRegisteredNamespaces()` - List all namespaces

#### Guard Registry

Named guards for conditional event handling:

- `registerGuard(name, guard)` - Register guard
- `unregisterGuard(name)` - Remove guard
- `getGuard(name)` - Get guard function
- `executeGuard(name, context, event)` - Execute guard

### Store Factory Registry

Apps register store factories to avoid circular dependencies:

```typescript
registerDesignAppStoreFactory(factory);
registerKitAppStoreFactory(factory);
registerTypeAppStoreFactory(factory);
registerQualityAppStoreFactory(factory);

getDesignAppStoreFactory();
getKitAppStoreFactory();
getTypeAppStoreFactory();
getQualityAppStoreFactory();
```

### File Providers

File providers abstract file storage for kits, supporting multiple backends.

#### FileProvider Port

```typescript
interface FileProvider {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
}
```

#### Provider Types

1. **MemoryFileProvider**: In-memory storage using Map (temporary kits)
2. **LocalFileProvider**: IndexedDB storage (browser persistence)
3. **RemoteFileProvider**: HTTP-based storage (server backend)
4. **CompositeFileProvider**: Combines multiple providers with fallback order

#### File Operations

File operations are handled automatically when kit diffs include file changes:

- **Added files**: Uploaded via provider, `remoteUrl` updated in kit
- **Removed files**: Deleted via provider
- **Updated files**: Re-uploaded if blob changed

### Y.js Integration

Y.js provides CRDT-based state synchronization and persistence.

#### Y.js Types

Stores use Y.js types for reactive state:

- `Y.Map` - Key-value maps (state objects)
- `Y.Array` - Arrays (lists, selections)
- `Y.Text` - Text (rarely used)

#### Persistence

- **IndexeddbPersistence**: Local browser persistence for kits
- **YProvider**: Remote synchronization (WebSocket, HTTP)

#### Observers

Y.js observers bridge Y.js changes to store updates:

- **Shallow observers**: Watch top-level map keys
- **Deep observers**: Watch nested changes

Use `createObserver` helper and dispose in `useEffect` cleanup.

#### Transactions

Y.js transactions batch operations:

- All Y.js mutations happen within transactions
- Store `transact` function wraps Y.js transactions
- Origin strings propagate to Y.js for debugging

### Coordinate System

semio uses a left-handed coordinate system that differs from Three.js.

#### semio Coordinate System

- **X-axis**: Right (thumb points right)
- **Y-axis**: Forward (index finger forward)
- **Z-axis**: Up (middle finger up)

#### Three.js Coordinate System

- **X-axis**: Right
- **Y-axis**: Up
- **Z-axis**: Backward (negative)

#### Conversion Functions

- `toThreeRotation()` - Matrix4 for semio → Three.js rotation
- `toSemioRotation()` - Matrix4 for Three.js → semio rotation
- `toThreeQuaternion()` - Quaternion for semio → Three.js
- `toSemioQuaternion()` - Quaternion for Three.js → Semio
- `vectorToThree(v)` - Convert Point/Vector to THREE.Vector3

### Expertise & Tooltips

The UI adapts to user expertise level, showing different tooltip content.

#### Expertise Levels

```typescript
enum Expertise {
  BEGINNER = "beginner", // Full tooltips with tutorials
  NORMAL = "normal", // Standard tooltips
  EXPERT = "expert", // No tooltips
}
```

#### Tooltip Content

Tooltips automatically adapt based on expertise:

- **BEGINNER**: Shows `.beginner` i18n key, tutorials, manuals, hotkeys
- **NORMAL**: Shows standard `.label` i18n key, manuals, hotkeys
- **EXPERT**: No tooltips shown

#### i18n Keys for Tooltips

Each UI element with an `id` prop automatically gets tooltip content from i18n:

- `{id}.label` - Standard label
- `{id}.beginner` - Beginner-friendly description
- `{id}.manual` - Manual page path
- `{id}.tutorial` - Tutorial path
- `{id}.hotkey` - Hotkey display string

#### Tooltip Components

- `<Tooltip>` - Base tooltip wrapper
- `<DescriptionTooltipContent>` - Automatic content from element ID
- `<EnhancedTooltipContent>` - Manual configuration

### Windows

Windows are the primary content areas within the canvas.

#### Window Kind

A window kind is an app-defined content surface identified by a stable id.

#### Window Layout

Window layouts are persisted per app as a JSON string (`windowLayout`).

#### Active Window

The canvas tracks the active window id for focus-sensitive UI.

#### Window Chrome

Window chrome includes action controls for open-in-new-window, maximize/minimize, and close.

### Validation

#### Overview

semio includes a **domain-pure validation system** built entirely in `semio.ts` with **zero JSON dependencies**. All validation logic works with `Kit` objects and produces `KitDiff`-based fixes.

#### Architecture

##### Layer 1: Domain Logic (`semio.ts`)

- **100% JSON-agnostic** - No JSON paths, parsing, or serialization logic
- **Pure functions** - All validation is deterministic and side-effect free
- **Diff-based fixes** - Every fix is a `KitDiff` that can be applied, inverted, and merged
- **Reusable everywhere** - Works in Sketchpad UI, CLI, backend, VS Code, and any other platform

##### Layer 2: Platform Integrations

Each platform provides its own thin wrapper:

- **VS Code Extension** (`js/vscode`) - JSON linter with Quick Fixes
- **Sketchpad UI** - In-app validation panel
- **CLI** - Command-line validation tool
- **Backend** - API validation endpoint

#### Validation Types

##### Core Types

```typescript
type SemioEntityKind = "Kit" | "Type" | "Design" | "Piece" | "Connection" | "Connector" | "Attribute" | "File" | "Folder" | "Quality" | "Port" | "Prop" | "Model" | "Layer" | "Group" | "Stat";
type Severity = "error" | "warning";

interface SemioDomainLocation {
  entityKind: SemioEntityKind;
  entityGuid?: Guid;
  field?: string;
}

interface Fix {
  title: string;
  diff: KitDiff;
}

interface Problem {
  constraintId: string;
  severity: Severity;
  message: string;
  location: SemioDomainLocation;
  relatedGuids?: Guid[];
  fixes: Fix[];
}

interface ValidationResult {
  problems: Problem[];
}
```

##### Validation Context

```typescript
interface ValidationContext {
  kit: Kit;
  typesByGuid: Map<Guid, Type>;
  designsByGuid: Map<Guid, Design>;
  piecesByGuid: Map<Guid, { designGuid: Guid; piece: Piece }>;
  connectorsByTypeGuid: Map<Guid, Connector[]>;
  modelsByTypeGuid: Map<Guid, Model[]>;
}
```

#### Validation Constraints

All validation constraints follow the pattern:

```typescript
type Constraint = (ctx: ValidationContext) => Problem[];
```

##### Default Constraints

#### 1. GUID Uniqueness (`guid-unique`)

**Severity:** Error

All GUIDs must be unique across the entire kit, including:

- Kit
- Types
- Designs
- Pieces
- Connections
- Stats
- Qualities
- Ports
- Files
- Folders

**Fix:** Regenerates a new GUID and updates all references throughout the kit.

#### 2. Type Name Uniqueness (`type-name-unique`)

**Severity:** Error

Types with the same parent must have unique names.

**Fix:** Renames the type with a unique suffix (e.g., "Wall 2", "Wall 3").

#### 3. Design Name Uniqueness (`design-name-unique`)

**Severity:** Error

Designs with the same parent must have unique names.

**Fix:** Renames the design with a unique suffix.

#### 4. Piece Name Uniqueness (`piece-name-unique`)

**Severity:** Error

Pieces within a design must have unique names.

**Fix:** Renames the piece with a unique suffix.

#### 5. Quality Name Uniqueness (`quality-name-unique`)

**Severity:** Error

All qualities within a kit must have unique names.

**Fix:** Renames the quality with a unique suffix.

#### 6. Port Name Uniqueness (`port-name-unique`)

**Severity:** Error

All ports within a kit must have unique names.

**Fix:** Renames the port with a unique suffix.

#### 7. File Name Uniqueness (`file-name-unique`)

**Severity:** Error

All files within a kit must have unique names.

**Fix:** Renames the file with a unique suffix.

#### 8. Folder Name Uniqueness (`folder-name-unique`)

**Severity:** Error

Folders with the same parent must have unique names.

**Fix:** Renames the folder with a unique suffix.

#### 9. Connector Name Uniqueness (`connector-name-unique`)

**Severity:** Error

Connectors within a type must have unique names.

**Fix:** Renames the connector with a unique suffix.

#### 10. Model Name Uniqueness (`model-name-unique`)

**Severity:** Error

Models within a type must have unique names.

**Fix:** Renames the model with a unique suffix.

#### 11. Layer Path Uniqueness (`layer-path-unique`)

**Severity:** Error

Layer paths within a design must be unique.

**Fix:** Renames the layer path with a unique suffix.

#### Uniqueness Requirements Summary

| Entity     | Scope                  | Field | Constraint ID         |
| ---------- | ---------------------- | ----- | --------------------- |
| Kit        | Global                 | guid  | guid-unique           |
| Type       | Siblings (same parent) | name  | type-name-unique      |
| Type       | Global                 | guid  | guid-unique           |
| Design     | Siblings (same parent) | name  | design-name-unique    |
| Design     | Global                 | guid  | guid-unique           |
| Piece      | Within design          | name  | piece-name-unique     |
| Piece      | Global                 | guid  | guid-unique           |
| Connection | Global                 | guid  | guid-unique           |
| Connector  | Within type            | name  | connector-name-unique |
| Model      | Within type            | name  | model-name-unique     |
| Quality    | Global                 | name  | quality-name-unique   |
| Quality    | Global                 | guid  | guid-unique           |
| Port       | Global                 | name  | port-name-unique      |
| Port       | Global                 | guid  | guid-unique           |
| File       | Global                 | name  | file-name-unique      |
| File       | Global                 | guid  | guid-unique           |
| Folder     | Siblings (same parent) | name  | folder-name-unique    |
| Folder     | Global                 | guid  | guid-unique           |
| Layer      | Within design          | path  | layer-path-unique     |
| Stat       | Global                 | guid  | guid-unique           |

#### Usage

##### In Domain Code

```typescript
const result = validateSemioKit(kit);
if (hasSemioErrors(result)) {
  console.error("Validation errors found:", result.problems);
}
```

##### Applying Fixes

```typescript
const problem = result.problems[0];
const fix = problem.fixes[0];
const fixedKit = applyKitDiff(kit, fix.diff);
```

##### Custom Validation

```typescript
const customConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  // Custom validation logic
  return problems;
};

const result = validateSemioKit(kit, {
  constraints: [...defaultConstraints, customConstraint],
});
```

###### Creating New Constraints

1. Define the constraint function following `Constraint` signature
2. Use `semioMakeFix` helper to generate `KitDiff`-based fixes
3. Add to `defaultConstraints` array
4. Document in this section

Example:

```typescript
export const semioCustomConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  // Validation logic
  // Use semioMakeFix to create fixes
  return problems;
};
```

#### Cross-Platform Connectorable Validation

All implementations (TypeScript, Python, C#) produce **identical** validation output for cross-platform compatibility. Problems include fixes with `KitDiff` structures.

##### Format

```json
{
  "problems": [
    {
      "constraintId": "type-name-unique",
      "severity": "error",
      "message": "Duplicate type name \"...\" among siblings.",
      "entityKind": "Type",
      "entityGuid": "...",
      "fixes": [
        {
          "title": "Rename \"...\"",
          "diff": { "types": { "updated": [...] } }
        }
      ]
    }
  ]
}
```

##### Implementation

- **TypeScript**: `toValidationResult()`, `serializeValidationResult()`, `areValidationResultsEqual()`
- **Python**: `ValidationResult.toDict()`, `ValidationResult.serialize()`, `areValidationResultsEqual()`
- **C#**: `SemioValidator.ValidateKit()`, `ValidationResult.Serialize()`, `ValidationResult.AreEqual()` (fix comparison pending)

##### Test Data

- `assets/semio/kit_invalid.json` - Invalid kit with all validation constraint violations
- `assets/semio/validation.json` - Expected output (sorted by constraintId, then entityGuid)

##### Updating Metabolism Assets

```bash
npx tsx scripts/update-metabolism.tsx
```

This script consolidates all Metabolism asset generation:

- Regenerates `metabolism.zip` with updated SQL schema and copies to all public folders
- Generates diff files (`diff_kit_metabolism.json`, `diff_kit_metabolism_inverted.json`, `kit_metabolism_diffed.json`)
- Generates `validation.json` from `kit_invalid.json`

##### Validation Constraints

| Constraint ID           | Description                                  |
| ----------------------- | -------------------------------------------- |
| `guid-unique`           | All GUIDs must be unique across the kit      |
| `type-name-unique`      | Type names must be unique among siblings     |
| `design-name-unique`    | Design names must be unique among siblings   |
| `piece-name-unique`     | Piece names must be unique within a design   |
| `connector-name-unique` | Connector names must be unique within a type |
| `model-name-unique`     | Model names must be unique within a type     |
| `quality-name-unique`   | Quality names must be unique                 |
| `port-name-unique`      | Port names must be unique                    |
| `file-name-unique`      | File names must be unique                    |
| `folder-name-unique`    | Folder names must be unique among siblings   |
| `layer-path-unique`     | Layer paths must be unique within a design   |

##### Fix Comparison Notes

- New GUIDs in `guid-unique` fixes can differ between implementations
- Fix diffs are normalized (GUIDs replaced with `<GUID>`) before comparison
- C# fix generation is pending; comparison skips fix diff for now

## 📁js/semio/sketchpad/

Sketchpad app modules, state machine wiring, and shared app surfaces for Home, Kit, Design, Type, Quality, Docs, and Feedback.

## 📄js/semio/sketchpad/elements.tsx

`Table` supports row-level hover callbacks for app hover state dispatch.

## 📄js/semio/sketchpad/Home.tsx

Home app hover state is stored in the Sketchpad state machine and updated via hover commands for table rows.

## 📄js/semio/sketchpad/Kit.tsx

Kit app hover state covers all artifact kinds and is updated via table and diagram hover dispatch.

## 📄js/semio/sketchpad/Sketchpad.tsx

Home command hooks forward hover events, including clear, into the Sketchpad state machine.

## 📁js/vscode/

VSCode extension providing violation diagnostics for open files and kit validation. Compatible with VS Code and Windsurf (engine: `^1.106.0`).

### Violation Diagnostics

The extension shows violation diagnostics for every open file using the repo analyze command:

- **On file open**: Loads cached violations from `.semio-repo/cache/analyze/<hash>.json` for immediate display
- **On file save**: Re-runs `repo analyze <relativePath>` and updates diagnostics from the refreshed cache
- **On file close**: Clears diagnostics and aborts any running analysis process
- Active preview tabs with semio diagnostics are pinned to regular editor tabs

Supported file types: TypeScript, JavaScript, JSON, Python, C#, Go.

### Kit Validation

For kit documents (JSON files with `kit_` prefix, `_kit` suffix, or named `kit.json`):

- Real-time validation using `validateKit()` from `semio/js`
- Problem-to-diagnostic mapping with entity location highlighting
- Quick Fix code actions applying `KitDiff` fixes

### Sidebar Views

Tree data providers for tickets, policies, contributors, and commands with search/filter capabilities.
Contributor tree nodes group contributions into commits, bundles, tickets by date, and files by folder with navigation commands for tickets, commits, bundles, and files.
Contributor commit items expose inline copy SHA and open in GitHub actions.
Sections view provider resolves the active editor path, calls `repo section list`, normalizes GraphQL section ranges into a nested SectionInfo tree, opens section locations on selection, binds F2 rename, supports drag-and-drop section moves, maps JSON object keys into the section tree, and routes rename/create-child/delete actions through repo section commands with refresh on active editor and document changes.

### Filters

FilterProvider manages cross-view filter state with multiple filter dimensions:

- **File Kind Filters**: code, script, config, test, docs, resource, license
- **Definition Category Filters**: implementation, interface, constant
- **Time Filters**: year, month, day (for ticket filtering)
- **Contributor Filters**: filter by contributor ID
- **Policy Filters**: filter by policy ID
- **Violation Filters**: filter by violation kind ID

Filter state uses `no<Kind>` and `only<Kind>` arrays to support both exclusion and inclusion patterns.
All toggle methods call `refreshAllViews()` to apply filters across all tree providers (codebase, tickets, contributors, policies, commands).
Filter values (years, months, days, contributors, policies, violations) are loaded on extension startup via `loadAvailableFilterValues()`.

### Commands

Command tree nodes are derived from command ids by segmenting the action name into group paths, with leaf commands attached to the last group node.
Command search includes command ids, titles, and segment names while group matches expand full subtrees.

### Tickets

Ticket tree items use `ticketOpen` and `ticketClosed` context values for inline close or reopen actions that apply to the clicked ticket, surface commit nodes from ticket commits, and limit hover tooltips to the summary or prompt text.

### Code Actions

- `RepoCodeActionProvider`: Quick fixes for violation diagnostics that run `repo fix <path>`
- `KitCodeActionProvider`: Quick fixes for kit validation problems that apply diff-based fixes

## 📄js/vscode/extension.ts

Extension activation, repo CLI command execution helpers, GraphQL client piping through the repo CLI, ticket command prompts aligned to repo ticket inputs including LLM and UI selection on ticket open, and section tree normalization for the Sections view based on repo section list output with line-only range start/end values.

## 📄js/vscode/package.json

VS Code extension manifest with unscoped name for vsce packaging, command contributions, scripts, and engine compatibility for Cursor support.

## 📁js/vscode/generated/

GraphQL codegen outputs for the VS Code extension.

## 📄js/vscode/generated/graphql.ts

Generated GraphQL types and typed documents for the VS Code extension.

## 📁graphql/repo/

Repo CLI GraphQL schema mirror for tooling.

## 📄graphql/repo/schema.graphql

GraphQL schema mirror of the repo CLI schema with TicketUI enum and ticket UI fields for VS Code codegen and typed documents.

## 📁sql/sqlite/repo/

SQLite schema definitions for repo exports.

## 📄sql/sqlite/repo/schema.sql

SQLite schema with ticket UI storage alongside LLM and commit metadata.

## 📁semio-repo/cli/

Repo CLI implementation and tests for the Go-based `semio-repo` tooling entrypoint.

## 📄semio-repo/cli/cli.go

Repo CLI single-file entrypoint with command registry, GraphQL execution, and three-tier hierarchical GitHub sync where root goals (depth 0) map to milestones, first-generation child goals (depth 1) map to issues with the `goal` label linked to the root milestone, and deeper goals (depth 2+) map to sub-issues of their parent goal's issue without milestone, with enforcement that existing goal issues always have the root milestone at depth 1, a parent sub-issue link and no milestone at depth 2+, and a restored `goal` label when missing. Ticket issues link to root goal milestones. Repository label catalog synchronization plus repository-wide `@` label cleanup for project/bundle tags, and emoji-normalized artifact IDs. Artifact kind derivation: `DeriveBundleKind` reads `bundleKind` from `package.json`/`project.json` at bundle root (fallback `library`); `DeriveFolderKind` classifies `.`-prefixed and manifest-containing folders as `required`; `DeriveFileKind` pattern-matches file names/extensions into `code`/`test`/`config`/`docs`/`resource`/`script`/`license`; `DeriveDefinitionKind` maps language keywords to `implementation`/`interface`/`constant` via `extractDefinitionKeyword` (word-before-name priority, modifier skipping) and `refineDefinitionKind` (arrow function detection). Dual identification system: `GetArtifactID` generates emoji-prefixed primary IDs (with kind-specific emoji helpers `projectKindEmoji`, `bundleKindEmoji`, `fileKindEmoji`, `folderKindEmoji`, `definitionKindEmoji`), `GetArtifactURI` generates `semiorepo://` secondary URIs with `SectionIdValueToUriPath`/`DefinitionIdValueToUriPath` for `#`/`§`→`/SLUG` encoding, `ParseSectionUriPath` for reverse URI→file+slugs parsing. `IdToUri`/`UriToId` bidirectional conversion with emoji normalization (strip `\uFE0E`/`\uFE0F` for matching). Collection types (projects, bundles, folders, files, sections, definitions, tickets, goals, drafts, todos, policies, violationKinds, contributors, commits) have dedicated ID and URI formats. `navigate` MCP tool resolves ID or URI input to both forms. MCP resource templates use slash-based paths for sections/definitions. Relative date rendering for tickets and goals supports RFC3339 and YYYY-MM-DD input formats via a flexible parser that uses the `dustin/go-humanize` package for human-friendly scheduling and history display. `--json` outputs pure data per line (no event wrappers, no `{"data": ...}` GraphQL envelope). Errors go to stderr. Cobra root has `SilenceUsage`/`SilenceErrors` to prevent stdout pollution on errors. Three rendering modes via `--format` flag: `NDJSONRenderer` (json), `HumanRenderer` (text), `MarkdownRenderer` (md, default). `inferEntityKind(key)` maps GraphQL operation names to entity kinds via prefix matching for generic mutation/query result dispatch. `renderEntityMarkdownLink(kind, data)` produces `[<id>](<uri>) - <prop1> - <prop2>` markdown for any entity kind. `renderEntityMarkdown(kind, data)` adds `- ` list prefix. `formatMarkdownResult` and `formatResult` use generic entity-kind dispatch with `inferEntityKind` fallback — no JSON code-block or dump fallback paths. Streaming list items detected via command suffix (`" list"`, `" tree"`) to apply correct `- ` markdown prefix. `repoContext.Fix()` detects violations via `CheckPoliciesWithContext`, filters autofixable ones, groups by file, and delegates to `applyAutofixes()` which applies per-violation-kind handlers bottom-up to avoid line number shifts: empty section removal with surrounding blank cleanup, missing/mismatched section end name resolution via `findMatchingSectionStartName()` stack-based walk, inline comment contiguous block removal with blank line tracking, block/JSDoc comment removal. Post-removal blank line collapse prevents double blank lines. `extractFileFromScope()` strips `#section` and `::definition` suffixes from scope strings. `FileHeaderId` generates emoji-prefixed file artifact IDs for headers using `DeriveFileKind` and `GetArtifactID`; for code files it reads the file content and overrides the kind to `script` when the first line starts with a shebang (`#!`). `ViolationCodeHeaderWrongFileId` (autofixable) replaces `ViolationCodeHeaderMissingFilename`; `headerPolicy` detects wrong/missing file IDs by scanning header lines for file extensions and emoji markers; `applyAutofixes` replaces the identified line with the correct artifact ID comment. `LanguagePlugin` interface with `BaseLanguage` base struct provides language-agnostic comment scanning via `ScanComments` using configurable primitives: `commentPrefix`, `blockCommentStart`/`blockCommentEnd`, `hasJSDoc`, `hasTemplates`, `hasRawBackticks`, `hasTripleQuotes`, `hasVerbatimStrings`, `skipDirectives`. `SkipDirectives()` merges built-in directives (`TODO`, `semio-ignore-`) with language-specific ones. `CommentScanState` tracks scanner state across lines (block comments, string literals, template expressions, triple quotes, raw backticks, verbatim strings, TODO blocks, escape sequences). Language constructors: TypeScript (`hasJSDoc`, `hasTemplates`, skip `eslint-`/`@ts-`/`noinspection`), Go (`hasRawBackticks`, skip `nolint`), Python (`hasTripleQuotes`, skip `noqa`/`type: ignore`/`pylint:`/`pragma:`), C# (`hasVerbatimStrings`, skip `pragma`). `ParseIgnoreDirectives` accepts language-aware `commentPrefix` parameter. `applyAutofixes` inline comment fix uses `PolicySectionStartMatch`/`PolicySectionEndMatch` and `SkipDirectives()` for language-aware region/directive detection.

## 📄semio-repo/cli/main.go

Go repo CLI/runtime implementation with custom `Interaction` JSON decoding that normalizes legacy object-form and current string-form `author` payloads into a single author string for ticket and goal interaction reads. MCP tool handlers call `Tool*` functions directly instead of GraphQL mutations to ensure consistent output between MCP and CLI. MCP resource handlers use validated GraphQL queries with correct field names (`range { start end }` for sections/definitions, `interactions` for tickets, `emails` for contributors, `id sha title date` for commits). `goalType` GraphQL object includes `uri` field resolved via `Goal.GetURI()`. `emojiText` enforces U+FE0E text presentation selector on all artifact ID emojis. `loadProjectsInternal` detects both `@`-prefixed and non-prefixed project directories, excluding hidden dirs and `node_modules`. `TreeNodeKind` enum and `TreeNode` struct model the complete monorepo hierarchy (project, bundle, folder, file, section, definition, goal, ticket, draft, policy, violationKind, contributor, commit, category). `TreeFilter` supports kind-level (`OnlyKinds`/`ExcludeKinds`), sub-kind-level (`OnlySubKinds`/`ExcludeSubKinds`), date (`OnlyYears`/`ExcludeYears`/months/days), status, and contributor filtering with case-insensitive matching. `BuildMonorepoTree` streams all data sources concurrently (projects, goals, tickets, drafts, policies, contributors, commits, folders, files) with a single filesystem walk, assigns folders/files to bundles by path prefix, and optionally parses sections via `TreeBuildOptions.IncludeSections`. `FilterMonorepoTree` recursively filters nodes and `collapseFilteredKinds` promotes children of excluded kinds to parent level. `SearchMonorepoTree` indexes all node attributes into a bleve in-memory index, performs fuzzy match queries, and `pruneUnmatched` preserves parent chains. `RenderMonorepoTree` outputs connector-based text trees and `RenderMonorepoTreeMarkdown` outputs nested markdown bullets; `treeCommand` selects JSON, markdown (default), or text rendering from `Config.Format`. `bindTreeFlags` registers `--only-<kind>`/`--no-<kind>` for 12 kinds, `--only-<subkind>`/`--no-<subkind>` for bundle/folder/file/definition sub-kinds, `--only-open`/`--only-closed`/`--open`/`--closed`, `--only-year`/`--no-year`/month/day int slices, and `--only-contributor-name`/`--no-contributor-name` string slices. `ArtifactRef` struct with `Kind` (file/folder/section), `Path`, and `SectionParts` fields parsed by `ParseArtifactRef` from emoji-prefixed artifact IDs (📁→folder, 💻/📄→file, 🔖→section with `#`-delimited slugs). `UnSlugify` converts `UPPER-KEBAB` slugs to title case. `FindSectionBySlug` case-insensitively matches section names against slugified forms. `ResolveSectionName` attempts slug-to-section resolution via file parsing, falling back to `UnSlugify`. `moveCommand` dispatches on source/target kind pairs: file→file, folder→folder, section→section (rename), file→section (integrate+delete+`RemoveAgentsDocsEntry`), section→file (extract). `integrateCommand` accepts artifact ID positional args or `--file`/`--target-file`/`--target-section`/`--parent-section` flags. `extractCommand` accepts artifact ID positional args or `--file`/`--section`/`--target-file` flags. `ToolProjectList` emits `Project` entities sourced from `LoadProjects` (sorted by name) for type-consistent project tool payloads. `ToolProjectTree` emits `Project` entities via `StreamProjects` through the `toolResultFromEvents` event pipeline, sorting events by `formatMarkdownResult` output for stable rendered-line ordering. `ToolBundleList` emits `Bundle` entities sourced from `LoadBundles` (sorted by name) for bundle-specific listing. `runProjectTree` sorts project events by `formatMarkdownResult` output to match `ToolProjectTree` ordering. `UpdateAgentsDocsPath` replaces old path prefixes with new ones in `AGENTS.md` `## ` headers under `# Codebase`. `RemoveAgentsDocsEntry` removes matching `## ` header and its content paragraph from `AGENTS.md`. `ToolFileMove`/`ToolFolderMove` call `UpdateAgentsDocsPath` after successful rename. MCP handlers `sectionExtract` and `artifactMove` expose extract and move functionality.

## 📄semio-repo/cli/cli_test.go

Consolidated Go test suite for the repo CLI and tooling behavior, structured for fast/slow lane execution with explicit slow-test sharding for parallel CI distribution. `executeCommand` separates stdout/stderr with 3-return signature. Wrong-argument tests cover all command categories (ticket, goal, policy, folder, file, section, definition, contributor, graphql). `TestCliJsonPureData` validates no event wrappers or `{"data": ...}` envelopes. `TestCliJsonErrorsToStderr` validates empty stdout on errors. Fix tests cover all autofixable violation kinds (empty section, missing end name, name mismatch, inline/block/JSDoc comments), nested sections, idempotency, multiple violations per file, non-autofixable detection, GraphQL mutation, RepoContext integration, scope extraction, violation kind metadata, and fixture-based end-to-end validation via `file_fixable.tsx`/`file_fixable_expected.tsx`. GraphQL fix mutation tests are scoped to avoid fixture side effects. `TestArtifactIDAndURI` covers all artifact and collection types (repo, projects, project, bundles, bundle, folders, folder, files, file, sections, section, definitions, definition, tickets, ticket, goals, goal, drafts, draft, todos, todo, policies, policy, violationKinds, violationKind, contributors, contributor, commits, commit). `TestIdToUri`/`TestUriToId` validate bidirectional ID↔URI conversion for all types including edge cases (ticket status stripping, empty input, invalid URIs). `TestSectionIdValueToUriPath`/`TestDefinitionIdValueToUriPath`/`TestParseSectionUriPath` validate URI path encoding and parsing helpers. Markdown output tests: `TestFormatMarkdownResult_MutationKeys` (21 mutation operations), `TestFormatMarkdownResult_SingleEntities` (12 entity kinds), `TestFormatMarkdownResult_Lists` (12 list types), `TestFormatMarkdownResult_Analyze`, `TestFormatMarkdownResult_Fix`, `TestFormatMarkdownResult_FileWithSections`, `TestFormatMarkdownResult_NoJSONFallback`, `TestFormatResult_MutationKeys` (12 HumanRenderer mutations), `TestRenderEntityMarkdownLink_AllKinds` (15 entity kinds), `TestInferEntityKind` (27 operation name mappings). `TestFileHeaderId` validates artifact ID generation for code, test, config, docs, script, resource, and license file kinds plus shebang-based script override for code files. `TestDeriveFileKind` covers 32 file name/extension patterns across all file kinds. `TestFileKindEmoji` covers all 7 file kind emojis plus unknown/empty fallback. `TestFixHeaderWrongFileId` tests autofix line replacement. `TestFixHeaderWrongFileIdIdempotent` verifies correct IDs are not flagged. `TestFixHeaderWrongFileIdDetection` validates violation detection and autofixable status. `TestFixHeaderWrongFileIdEndToEnd` tests detect-fix-verify cycle. Per-language comment scanning tests: `TestScanCommentsGo` (inline, block, TODO, nolint, raw backtick strings, header exclusion, region markers, debug markers, URL schemes, grouped comments), `TestScanCommentsPython` (inline, TODO, noqa, type: ignore, triple-quoted strings, header exclusion, region markers, regular strings, trailing comments), `TestScanCommentsCSharp` (inline, block, TODO, pragma, verbatim strings, region markers, header exclusion, no-JSDoc), `TestScanCommentsTypeScript` (JSDoc, template literals, template expressions, eslint/ts directives, string literals, config files), `TestScanCommentsShell` (inline, strings, region markers), `TestScanCommentsRust` (inline, block, TODO). `TestScanCommentsAutofix` validates cross-language autofix for Python inline, Python trailing, Go block, and C# inline comment removal.

## 📄semio-repo/cli/main_test.go

Go test suite includes interaction-author shape coverage to validate JSON decoding for both legacy object payloads and current string payloads. MCP tool tests cover all `Tool*` functions used by MCP handlers: `TestToolProjectList`, `TestToolProjectTree`, `TestToolBundleList`, `TestToolContributorList`, `TestToolGoalList`, `TestToolTicketList`, `TestToolDraftList`, `TestToolFolderList`, `TestToolFolderTree`, `TestToolFileList`, `TestToolFileTree`, `TestToolSectionList`, `TestToolSectionTree`, `TestToolDefinitionList`, `TestToolPolicyList`, `TestToolPolicyCheck`, `TestToolAnalyzeScope`, `TestToolFixScope`, `TestToolFolderCRUD`, `TestToolFileCRUD`, `TestToolTicketLifecycle`, `TestToolDraftLifecycle`, `TestToolGoalUri`. `TestUriToId` expects U+FE0E text presentation selector in all artifact IDs. Monorepo tree tests: `TestTreeNodeKindConstants` (distinctness, non-empty), `TestTreeFilterIsKindVisible` (default/only/exclude/category), `TestTreeFilterMatchesSubKind` (default/only/exclude/empty/case-insensitive), `TestTreeFilterMatchesDate` (default/only-year/exclude-year/month/combined), `TestTreeFilterMatchesStatus` (default/open/closed/case-insensitive), `TestTreeFilterMatchesContributor` (default/only/exclude/case-insensitive), `TestFilterMonorepoTree` (no-filter/exclude-bundle/no-folder-collapse/only-library/status/year/contributor/nil), `TestSearchMonorepoTree` (empty-query/match/no-match/parent-chain), `TestRenderMonorepoTree` (text connectors plus markdown bullet renderer coverage), `TestBuildMonorepoTree` (categories/projects/with-sections/without-sections), `TestCollapseFilteredKinds` (folder-collapse/nested-collapse), `TestSortTreeChildren` (alphabetical/folders-first), `TestTreeCommandFlags` (flag-binding/empty-flags), `TestTreeCommands` (default markdown plus `--text` and `--json` modes), and `TestMarkdownOutput` (default markdown tree/list outputs).

## 📁net/

## 📄net/Semio.cs

Core library containing all model definitions, validation, serialization, and the Meta class for reflection-based metadata.

## 📄net/Semio.Grasshopper.cs

Grasshopper plugin providing components for constructing, deconstructing, and modifying semio models.

#### Architecture

The plugin uses a component hierarchy with base classes that provide default behavior:

- **`ModelComponent<TParam, TGoo, TModel>`**: Base class for model components with virtual methods for customization
- **`IdComponent`**, **`DiffComponent`**: Specialized base classes for Id and Diff model types
- **`SerializeComponent`**, **`DeserializeComponent`**: Base classes for serialization components

#### Component Structure

Each model type has a set of classes:

- **`*Goo`**: Grasshopper wrapper for the model type with cast methods
- **`*Param`**: Grasshopper parameter definition
- **`*Component`**: Main model component for construct/deconstruct/modify
- **`Serialize*Component`**: JSON serialization component
- **`Deserialize*Component`**: JSON deserialization component

#### Hardcoded Parameters

Components use virtual methods to define their inputs/outputs:

- `RegisterModelInputParams(pManager)`: Define input parameters
- `RegisterModelOutputParams(pManager)`: Define output parameters
- `GetModelData(DA, model)`: Read input data into model
- `SetModelData(DA, model)`: Write model data to outputs

Components can override these to hardcode their parameter structure, ensuring stable input/output definitions across schema changes.

## 📁py/

Python code with the engine (semio/engine) for schema generation and validation.

## 📁py/engine/

Python engine providing schema generation, validation, and backend functionality.

- `engine.py` - Main engine module with Kit parsing, validation, transformation, dev-mode startup flag, and stdio MCP startup flag
- `engine.test.py` - Unit tests for engine functionality
- `generate-schemas.ts` - Generates GraphQL, JSON, and SQL schemas from TypeScript definitions
- `sqliteschema.ts` - SQLite schema generation utilities

## 📁net/

C# code with the core library (`Semio.cs`) and Grasshopper plugin (`Semio.Grasshopper.cs`).

## 📄net/Semio/Semio.cs

Core library containing all model definitions, validation, serialization, and the Meta class for reflection-based metadata.

## 📄net/Semio.Grasshopper/Semio.Grasshopper.cs

Grasshopper plugin providing components for constructing, deconstructing, and modifying semio models.

### Architecture

The plugin uses a component hierarchy with base classes that provide default behavior:

- **`ModelComponent<TParam, TGoo, TModel>`**: Base class for model components with virtual methods for customization
- **`IdComponent`**, **`DiffComponent`**: Specialized base classes for Id and Diff model types
- **`SerializeComponent`**, **`DeserializeComponent`**: Base classes for serialization components

### Component Structure

Each model type has a set of classes:

- **`*Goo`**: Grasshopper wrapper for the model type with cast methods
- **`*Param`**: Grasshopper parameter definition
- **`*Component`**: Main model component for construct/deconstruct/modify
- **`Serialize*Component`**: JSON serialization component
- **`Deserialize*Component`**: JSON deserialization component

### Hardcoded Parameters

Components use virtual methods to define their inputs/outputs:

- `RegisterModelInputParams(pManager)`: Define input parameters
- `RegisterModelOutputParams(pManager)`: Define output parameters
- `GetModelData(DA, model)`: Read input data into model
- `SetModelData(DA, model)`: Write model data to outputs

Components can override these to hardcode their parameter structure, ensuring stable input/output definitions across schema changes.

# Hierarchies

Use this hierarchy for code organization (order of appearance of regions, classes, properties, functions, methods, types, statements, constants, …).

## 1. Models

1. Attribute
2. Coord
3. Vec
4. Point
5. Vector
6. Plane
7. Camera
8. Location
9. Author
10. File
11. Benchmark
12. QualityKind
13. Quality
14. Port
15. Prop
16. Model
17. Connector
18. Type
19. Layer
20. Piece
21. Group
22. Side
23. Connection
24. Stat
25. Design
26. Kit

## 2. Classes | Types

1. Model
2. Id
3. Shallow
4. Diff
5. Diffs
6. Input
7. Output
8. Context
9. Prediction

## 3. Properties

### Attribute

1. Key
2. Value
3. Definition

### Coord

1. U
2. V

### Vec

1. U
2. V

### Point

1. X
2. Y
3. Z

### Vector

1. X
2. Y
3. Z

### Plane

1. Origin
2. XAxis
3. YAxis

### Camera

1. Position
2. Forward
3. Up

### Location

1. Longitude
2. Latitude
3. Altitude
4. Attributes

### Author

1. Name
2. Email
3. Attributes

### File

1. Path
2. RemoteUrl
3. Description
4. Attributes

### Benchmark

1. Name
2. Icon
3. Min
4. MinExcluded
5. Max
6. MaxExcluded
7. Definition
8. Attributes

### QualityKind

1. General
2. Type
3. Design
4. Piece
5. Connection
6. Connector

### Quality

1. Key
2. Name
3. Kind
4. Default
5. Formula
6. DefaultSiUnit
7. DefaultImperialUnit
8. Min
9. MinExcluded
10. Max
11. MaxExcluded
12. CanScale
13. Benchmarks
14. Definition
15. Attributes

### Port

1. Name
2. Description
3. Icon
4. CompatiblePorts
5. Attributes

### Prop

1. Key
2. Value
3. Unit
4. Attributes

### Model

1. Name
2. Tags
3. Url
4. Description
5. Attributes

### Connector

1. Id
2. Name
3. Point
4. Direction
5. T
6. Mandatory
7. Port
8. Description
9. Attributes

### Type

1. Name
2. Variant
3. Models
4. Connectors
5. Props
6. IsVirtual
7. CanScale
8. CanMirror
9. Unit
10. AvailableCount
11. Location
12. Authors
13. Concepts
14. Icon
15. Image
16. Description
17. Attributes

### Layer

1. Path
2. IsHidden
3. IsLocked
4. Color
5. Description
6. Attributes

### Group

1. Pieces
2. Color
3. Name
4. Description
5. Attributes

### Piece

1. Id
2. Name
3. Type
4. Design
5. Plane
6. Center
7. Scale
8. MirrorPlane
9. Props
10. IsHidden
11. IsLocked
12. Color
13. Description
14. Attributes

### Side

1. Piece
2. DesignPiece
3. Connector

### Connection

1. Connected
2. Connecting
3. Gap
4. Shift
5. Rise
6. Rotation
7. Turn
8. Tilt
9. U
10. V
11. Description
12. Attributes

### Design

1. Name
2. Variant
3. View
4. Pieces
5. Connections
6. Stats
7. Props
8. Layers
9. ActiveLayer
10. Groups
11. CanScale
12. CanMirror
13. Unit
14. Location
15. Authors
16. Concepts
17. Icon
18. Image
19. Description
20. Attributes

### Kit

1. Name
2. Version
3. Types
4. Designs
5. Qualities
6. Files
7. Authors
8. RemoteUrl
9. HomepageUrl
10. License
11. Concepts
12. Icon
13. Image
14. Description
15. Attributes
