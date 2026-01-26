<!-- IMPORTANT -->

ALWAYS open a ticket with semio-repo mcp tool `ticket_open` (or use `./go/repo/repo ticket open "<title>" "<prompt>" --<llm> --<ui>` when no mcp tool is available) for every task you are working on. The title MUST be titleized (e.g. "Some Title on Something") and NOT a slug or all caps. Available LLMs are: `opus-4-5`, `sonnet-4-5`, `haiku-4-5`, `gemini-3-pro`, `gemini-3-flash`, `gpt-5-2-codex`, `gpt-5-mini`. Available UIs are: `claude-code`, `cursor`, `copilot-chat`, `antigravity`, `codex`, `droid`. This creates a `tickets/YYYY/MM/DD/TICKETSLUG` folder along with `plan.md` and `ticket.md`. NEVER answer directly in the chat and ALWAYS just add the message to `ticket.md`. ALWAYS start by writing the plan into `plan.md` and NEVER send it as message. ALWAYS end by writing the summary into the `## Summary` section of `ticket.md` and NEVER send it as message. ALWAYS use the semio-repo mcp tool `ticket_close` (or use `./go/repo/repo ticket close YYYY/MM/DD/TICKETSLUG "<summary>" <files...>` when no mcp tool is available) to finish the ticket along with the summary and at all the files you worked on (created, updated or removed). When a dev sends a new message to the chat ALWAYS reopen the same ticket with semio-repo mcp tool `ticket_reopen` (or use `./go/repo/repo ticket reopen YYYY/MM/DD/TICKETSLUG "<prompt>" --<llm> --<ui>` when no mcp tool is available).

- Multiple agents and a developer ALWAYS work on the same codebase at the same time. NEVER use `git stash`, `git stash pop`, `git checkout`, … because it will mess up others work and worst-case delete their work.
- The codebase in under design and development and not used in production yet. There are many inconsistencies that need to be refactored. ALWAYS use clean mechanisms that might require large refactorings and NEVER care about backwards compatibility.
- For every task you are working on, you MUST update the dev docs (`README.md` and `AGENTS.md`). Every key decision and mechanism ALWAYS needs to be documemented. Every feature, decision MUST be undocumented/uncommented in the code and MUST be documented in the dev docs (AGENTS.md and README.md). The documentation ALWAYS happens three times:

1. Under `# 📦 Bundles` in README.md where it is described from junior-developer perspective (mechanism explanation and reasoning behind the decision, how theory links to implementation, etc).
2. Under `# Software Requirements Specification` in AGENTS.md where it is described from human-interface-designer perspective (concise technical terms without explanation, framework-agnostic, no implementation references). There are two sections: `# Business Logic` and `# UI/UX`.
3. Under `# Codebase` in AGENTS.md where it is described from senior-developer perspective (framework-mechanisms, consice technical terms without explanation, implementation details, etc). The section has the same header structure as the files and folders. All files and folders are flat with `## PATH` e.g. `## js/semio/sketchpad/` or `## net/Semio.cs`
   The purpose of the dev docs is to understand the codebase. NEVER add reasoning or process related (such as what changed, why, how, … - this is part of the log) to the dev docs.

This document MUST ALWAYS BE followed unless explicitly asked to do otherwise.

<!-- IMPORTANT -->

# Software Requirements Specification

## Business Logic

### Code Hygiene

Source files MUST include an SPDX license header.

Source files MUST NOT include inline comments except for license headers and region markers.

Block and JSDoc comments are treated as inline comments.

Comment detection MUST ignore comment markers inside string literals and template literal text.

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

### Devcontainer

Devcontainer provisioning MUST install the workspace VS Code extension automatically after editor attach without manual installation steps.
Playwright browser caches MUST use the workspace `node_modules` volume path so `npx playwright install` stays cached across reloads.
Claude Code and Codex auth plus chat history MUST persist across devcontainer rebuilds via named volumes for CLI config and editor server state.
Claude Code auth files MUST live in the persisted Claude volume and be linked into `$HOME`.

### Sections

File section trees MUST be derived from language-aware section parsing per file.
Section data MUST expose file path, section path, range, and parent-child relationships.
Shell scripts (`.sh`) MUST use hash-based region markers for section parsing.

### Engine

Engine startup MUST support a dev/debug mode flag that waits for debugger attachment before runtime begins.
Engine startup MUST support a pure stdio MCP server mode.

### State Management

App hover and selection state MUST be managed by the Sketchpad state machine.

### Ticket

A `ticket` is a development artifact that tracks a task.

A `ticket` has a `status` of **open** or **finished**.

A ticket MUST store a `prompt` which is the prompt used to create the ticket.

A ticket MUST store a `commit` which is the git commit at ticket creation for line stats calculation.

A ticket MUST store a summary when finished.

A ticket MUST store semantic diffs for bundles, folders, files, sections, and definitions with line stats when finished.

Ticket workspaces MUST store a single ticket.md that captures todos, changes, log entries, and the summary alongside plan.md.

Tickets can be reopened to return to **open** status.

Ticket close and reopen actions invoked from the ticket list MUST apply to the selected ticket without additional selection.

Ticket creation MUST require a prompt and a titleized title (e.g. "Some Title on Something"). Slugs or all-caps titles are forbidden.

Ticket title updates MUST rename the ticket folder and slug path.
Ticket open MUST interpret a `CONTINUE` keyword to continue the latest ticket and a `NOTICKET` keyword to skip ticket creation.

Ticket finish MUST require a summary and a list of files.

Temporary task artifacts MUST be stored inside the active ticket workspace.

Ticket finish MUST derive semantic diffs across bundles, folders, files, sections, and definitions via git diff between the ticket base commit and the current commit, scoped to the files declared on the ticket.
Ticket line metrics MUST map added lines to current scopes and removed lines to base-commit scopes for semantic diffs.

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

Ticket open inputs MUST allow optional `noIssue` and `planPath` fields.
The repo CLI binary MUST be consolidated into a single `go/repo/main.go` source file that owns engine, CLI, MCP, and rendering behavior.
Legacy repo CLI adapter packages MUST NOT exist outside `go/repo/main.go`.
Repo operational commands (benchmark, preflight, update) MUST live in the single-file repo entrypoint.
Ticket close and reopen MUST address tickets via `YYYY/MM/DD/SLUG` path identifiers.
Ticket reopen MUST require `prompt` and `llm` values.
GraphQL TicketUI inputs MUST accept normalized enum tokens (copilot_chat, claude_code, codex, etc.) for UI selection.
GraphQL section/definition ranges MUST expose line and column positions for start and end.
GraphQL range selections MUST request Position subfields (line, column) for start and end.
Ticket listing MUST read from `.semio-repo/tickets` and fall back to legacy `tickets/` directories when present.
Ticket open MUST require a ticket UI enum value.
Repo CLI MUST expose an export command that emits a SQLite snapshot of bundles, folders, files, sections, contributors, tickets, policies, and violations.
Repo section tooling MUST expose an integrate command that merges source files into target sections.
Ticket close MUST apply all affected bundle labels and the `@semio-repo` label for out-of-bundle paths.
Ticket close MUST post a metrics comment listing semantic changes for bundles, folders, files, sections, and definitions with status icons and `+added`/`-removed` counts.
Ticket issue bodies MUST prepend a `# 🤖 Prompt` heading.
Ticket reopen MUST add a `# 🤖 Prompt` comment with the latest prompt.
Ticket close MUST prepend a `# 🔍 Summary` heading to the summary comment.
Ticket GitHub heading formatting MUST be consistent across create, reopen, and close flows.
Ticket line metrics MUST use full line counts for added and deleted scopes, and diff-based counts for modified scopes.
Ticket close MUST ignore files inside the active ticket workspace (plan.md, ticket.md).
Repo analyze without a scope MUST emit a codebase snapshot to `.semio-repo/reports/codebase.json` for semantic diffing.
Ticket GitHub issues MUST be linked to the usalu project 2 on create and reopen.
VS Code extension manifests MUST use an unscoped `name` value for vsce packaging.
Repo CLI commands MUST emit a JSONL event stream with a terminal done payload for machine consumption.
VS Code tooling MUST parse JSONL event streams, surface fatal errors, and use the final result payload as the GraphQL response body.
Repo tooling MUST execute CLI, MCP, and VS Code commands through the streaming registry with emitter events for progress, items, errors, logs, and done payloads.
MCP list tools MUST support cursor and limit paging over streamed item events.

Repo operational artifacts (tickets, contributors, reports) MUST be stored under `.semio-repo/`.
Repo analyze MUST exclude gitignored files, `.semio-repo/`, and `assets/repo/` from analysis.
Repo file/folder listing and VS Code diagnostics MUST exclude gitignored files and `.semio-repo/` paths.
Ticket close MUST derive bundle labels from semantic bundle diffs and MUST NOT infer `@semio-repo` from README.md or AGENTS.md.
Ticket iterations MUST store their own semantic diff payloads; tickets MUST NOT store diff payloads at the top level.

### MCP Tools

MCP tool calls MUST validate argument presence and types.
File and folder parameters MUST reference correct path kinds (file vs folder).
Invalid MCP tool arguments MUST return errors at the tool boundary.

### Contributor

Contributor contributions MUST be derived from ticket frontmatter and source file headers.

Contributor ordering MUST be based on ticket contribution count.

Contributor contributions MUST expose tickets, commits, bundles, files, and line totals.

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

- Ticket tree items expose inline close and reopen actions that apply to the selected ticket based on status.
- Ticket tree hovers show only the ticket description.
- Ticket creation prompts for LLM and ticket UI selections.
- Ticket tree items list commit entries as child nodes.
- Ticket commands collect title/prompt/LLM for open, prompt/LLM for reopen, and operate on `YYYY/MM/DD/SLUG` ticket identifiers.
- Ticket detail views consume git-derived per-file and total line stats stored on iterations and ticket close.
- Command trees mirror the CLI command and subcommand hierarchy; matching a command group keeps its subtree visible.
- Problem list diagnostics open in pinned editor tabs for immediate saves.
- Repo diagnostics and trees exclude gitignored files and `.semio-repo/` content.
- Contributor tree items list emails with mailto actions, links with external navigation, and contribution nodes with line summary descriptions.
- Contributor contributions are grouped into commits, bundles, tickets (year/month/day), and files (folder/file) with navigation actions and inline ticket close/reopen actions.
- The built-in Explorer hosts the Sections view; selecting a section navigates to it, F2 renames, drag-and-drop moves sections, JSON keys surface as sections, and inline actions create child sections, rename sections, and delete sections via repo commands.
- The Sections view resolves the active file's section tree with line ranges so navigation and section actions match the current editor content.
- Ticket tooling treats temporary artifacts as part of the active ticket workspace.
- Devcontainer setup installs the workspace VS Code extension automatically on attach without manual installation actions.

# Monorepo

## Devcontainer

The monorepo uses a devcontainer for consistent cross-platform development. The devcontainer includes:

**Languages & Runtimes:**

- Node.js 22.x with npm 11.x
- Go 1.24
- Python 3.14 with uv package manager
- .NET SDK 7.0 and 8.0
- Rust with wasm32-unknown-unknown target

**Tools:**

- Git with LFS and GitHub CLI
- SQLite3
- Playwright (Chromium)
- All VS Code extensions from `.vscode/extensions.json`

**Forwarded Ports:**

- 2507: Engine API
- 3000: Sketchpad Dev
- 4000: Play Dev
- 4321: Docs Dev
- 5678: Python Debugger
- 6006: Storybook

**Setup Scripts:**

- `post-create.sh`: Runs after container creation (npm install, uv sync at root, Go build, dotnet restore, Playwright install)
- `post-start.sh`: Runs on each container start (activates root Python venv)
- `post-attach.sh`: Runs after editor attach to build and install the local semio VS Code extension

## Git

- The `main` branch is compressed (squashed history) and acts as the canonical integration branch.
- If a release receives updates after `main` already progressed, create a parallel `release/rYY.MM-V` branch for that release and keep it compressed as well.
- Commit messages follow `MAIN-TASK-SYMBOL SUMMARY WORK-SYMBOL` where `WORK-SYMBOL` is one of `🪛` < `🔨` < `🛠️` < `🏗️`.

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
- NEVER create new files when not explicitly asked. ALWAYS add code to existing files using regions and subregions for structuring. Regions organize code into collapsible sections (e.g., `#region RegionName` / `#endregion` in C#, or `//#region RegionName` / `//#endregion` in JavaScript/TypeScript). Use subregions within regions for hierarchical organization. This keeps related code together and maintains a single source of truth per logical unit.
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
- NEVER add raw text to ui elements. ALWAYS use i18n setups and provide translations for the existing languages.
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

## Keywords

Whenever a keyword is used, ALWAYS directly proceed with the task and NEVER ask for approval.

- `DIAGNOSE`: Think about the problem and possible causes. ALWAYS add console logs to the codebase to help understand the problem. NEVER assume to know the solution and ALWAYS use logs to verify your hypothesis. ALWAYS add a `[SLUG] ` (replace SLUG with a unique slug for this diagnosis) after `[DEBUG] ` to the console log in order to identify the logs related to the diagnosis. E.g. `[DEBUG] [PIECE-DRAG-AND-DROP-ISSUE] Mounting Dropzone: …`. Then you will receive the logs from the user and if the logs are enough to verify your hypothesis, ALWAYs directly implement the solution. When the `DIAGNOSE` is not enough, update the document with the new information, add new logs and continue the process.

- `FIX`: Anaylze and fix the problem imediatley in one step (without any approval). When you are not sure about the root cause, pick the most likely one and try to implement the solution directly. ALWAYS extend, change and refactor everything (even if it is an intermediate step) to fix the problem.

- `CLEAN`: Clean up everything intermediate such as diagnostic console logs, comments, and temporary code.

- `I18N`: Run `tsx scripts/i18n.ts` to regenerate `reports/i18n.md`; fix all reported translation problems, add missing keys, update incomplete entries, remove unused keys, and rerun the report and loop until all errors/warnings are gone and the report is clean.

- `AUTOMATE`: Create a `*.ts` script to automate a task (use `scripts/utils.ts` for reusable code). Create a run configuration in `package.json`, create a task in `.vscode/tasks.json` and create a `.vscode/launch.json` along with the script. Call the script from the `preflight.ts` script.

- `FINISH`: Finish a task that was started but not completed. ALWAYS first search for recent tickets using `repo ticket list` and analyze with git staged and unstaged changes that are related to the task.

- `SCHEMA`: Extend the schema for `semio.ts` then run `tsx scripts/schema.ts` to regenerate `reports/schema.json`; Fix all reported schema problems, add missing fields, update incomplete entries, remove unused fields, and rerun until the report is clean. Rerun the script until the report is clean.

## CI/CD

### Commands

### Reports

Linters generate JSON reports in `reports/`:

- `reports/i18n.json` - i18n translation validation
- `reports/eslint.json` - ESLint linting problems
- `reports/code.json` - Codebase code-quality problems
- `reports/typescript.json` - TypeScript compiler errors
- `reports/ruff.json` - Python Ruff linting problems

Reports are gitignored (except README) and regenerated on each commit.

#### Reports Directory

The `reports/` folder stores all autogenerated JSON outputs (gitignored) so hook outputs are centralized and disposable.

### Repo CLI

The `repo` CLI provides unified access to all monorepo operations. It is implemented in Go (`go/repo`). All commands output JSON for programmatic consumption. Interactive wizards are provided via the VSCode extension.

**Architecture:**

```
go/
├── repo/     # Go CLI binary - non-interactive, JSON output
│   └── main.go
└── mcp/      # MCP server - calls repo binary, exposes tools to LLMs
```

**Usage:**

```bash
# Using Go binary (preferred)
go/repo/repo <command> [subcommand] [options]

# Using TypeScript fallback
npx tsx repo.tsx <command> [subcommand] [options]
```

**Commands:**

| Command                                   | Description                         |
| ----------------------------------------- | ----------------------------------- |
| `help`                                    | Show help message                   |
| `analyze [scope]`                         | Analyze codebase for problems       |
| `fix [scope]`                             | Apply autofixes for problems        |
| `policy list`                             | List all registered policies        |
| `policy check <id>`                       | Check a specific policy             |
| `ticket open <slug>`                      | Create a new ticket                 |
| `ticket list [year] [month] [day]`        | List tickets                        |
| `ticket read <year> <month> <day> <slug>` | Read a ticket                       |
| `ticket progress`                         | Progress iteration on a ticket      |
| `ticket close`                            | Finish a ticket                     |
| `ticket reopen`                           | Reopen a ticket                     |
| `bundle list`                             | List Nx bundles                     |
| `bundle tree`                             | Show bundle dependency tree         |
| `folder tree [path]`                      | Show folder structure               |
| `folder create <path>`                    | Create a folder                     |
| `folder move <src> <dst>`                 | Move/rename a folder                |
| `folder delete <path>`                    | Delete a folder                     |
| `folder list [path]`                      | List folders                        |
| `file tree [path]`                        | Show file tree                      |
| `file create <path>`                      | Create a file                       |
| `file move <src> <dst>`                   | Move/rename a file                  |
| `file delete <path>`                      | Delete a file                       |
| `file list [scope]`                       | List files in scope                 |
| `section tree <file>`                     | Show section structure of a file    |
| `section create <file>`                   | Create a section                    |
| `section move <file>`                     | Move/rename a section               |
| `section delete <file>`                   | Delete a section                    |
| `section integrate <src> <sec> <dst> [p]` | Integrate code into target section  |
| `section list <file>`                     | List sections                       |
| `definition list <file>`                  | List definitions in a file          |
| `tool <name> [args...]`                   | Run an Nx target (e.g., lint, test) |

**Scope Syntax:**

- `@semio` - Repo scope (all files)
- `@semio/js` - Bundle scope (Nx bundle)
- `js/semio/sketchpad/` - Folder scope
- `js/semio/sketchpad/App.tsx` - File scope
- `file.tsx#Region` - Section scope
- `file.tsx§Function` - Definition scope

**Output:**

All commands output JSON with structure:

```json
{
  "output": { "lines": [...], "exitCode": 0 },
  "data": { ... },
  "error": "..."
}
```

**Built-in Policies:**

- `code` - Validates source file headers, sections, and comments
  - `code:header:*` - Header region validation
    - `code:header:missing-region` - Missing header section
    - `code:header:missing-filename` - Missing filename in header
    - `code:header:missing-contributors` - Missing contributors in header
    - `code:header:missing-license` - Missing SPDX license
    - `code:header:wrong-license` - Incorrect license type
  - `code:section:*` - Section block validation
    - `code:section:empty` - Empty section
    - `code:section:missing-start-name` - Missing section start name
    - `code:section:missing-end-name` - Missing section end name
    - `code:section:name-mismatch` - Section name mismatch
  - `code:comment:*` - Forbidden comments detection
    - `code:comment:inline` - Inline comment detected
    - `code:comment:block` - Block comment detected
    - `code:comment:jsdoc` - JSDoc comment detected
- `dev-docs` - Validates README.md and AGENTS.md documentation structure

**Examples:**

```bash
repo analyze js/semio/semio.ts                              # Analyze single file
repo section tree repo.tsx                                  # Show section structure
repo definition list js/semio/semio.ts                      # List all definitions
repo bundle list                                            # List all Nx bundles
repo folder tree js/semio                                   # Show folder tree
repo ticket open <title> <prompt> <llm> <ui> [--no-issue]    # Create ticket (positional syntax)
repo ticket open --title <t> --prompt <p> --llm <l> --ui <u>  # Create ticket (explicit syntax) - llm: opus-4-5, claude-sonnet-4, haiku-4-5, gemini-3-pro, gpt-5-2, gpt-5-2-codex; ui: claude-code, cursor, copilot-chat, antigravity, codex, droid
repo ticket list [year] [month] [day]                       # List tickets (optionally filtered by date)
repo ticket close <YYYY/MM/DD/SLUG> <summary> <files...> [--title <new-title>]  # Close ticket (--title updates GitHub issue)
repo ticket reopen <YYYY/MM/DD/SLUG> <prompt> <llm> [--title <new-title>]      # Reopen ticket (--title updates GitHub issue)
repo tool build                                             # Run Nx build target
```

### MCP Server

The MCP server (`go/mcp`) exposes the repo CLI tools via the Model Context Protocol for LLM integration.
MCP tool calls validate argument types and path kinds before invoking the CLI and return errors on invalid input.

**Building:**

```bash
cd go/mcp && go build
```

**Usage:**

The MCP server communicates via stdio and exposes all repo tools as MCP tools. Configure it in your MCP client (e.g., Claude Desktop, Cursor) by adding to `mcp_settings.json`:

```json
{
  "mcpServers": {
    "semio-repo": {
        "type": "stdio",
        "command": "go/repo/repo",
        "args": [
          "--mcp-stdio"
        ]
  }
}
```

**Available MCP Tools:**

- `analyze` - Analyze codebase for policy violations
- `fix` - Apply autofixes for violations
- `policy_list` - List all registered policies
- `policy_check` - Check a specific policy
- `ticket_open` - Create a new ticket
- `ticket_list` - List tickets
- `ticket_read` - Read a ticket
- `ticket_progress` - Progress iteration on a ticket
- `ticket_close` - Finish a ticket
- `project_list` - List Nx bundles
- `project_tree` - Show bundle tree
- `folder_create`, `folder_move`, `folder_delete`, `folder_list`, `folder_tree` - Folder operations
- `file_create`, `file_move`, `file_delete`, `file_list`, `file_tree` - File operations
- `section_create`, `section_move`, `section_delete`, `section_list`, `section_tree`, `integrate` - Section operations
- `definition_list` - List definitions
- `tool_run` - Run Nx tool
- `graphql` - Execute GraphQL queries against the repo schema

### GraphQL API

The repo CLI exposes a GraphQL API for querying and mutating repository data. The GraphQL schema is defined in `go/repo/main.go` and mirrored in `graphql/repo/schema.graphql` for tooling.

**CLI Usage:**

```bash
repo graphql "{ repo { id name } }"
repo graphql "{ repo { bundles { name root } } }"
repo graphql "query Tickets { repo { tickets { id slug status } } }"
repo graphql "{ analyze(scope: \"js/semio/\") { violations { id summary } } }" -v "{}"
```

**Schema Location:** `go/repo/main.go` (source), `graphql/repo/schema.graphql` (mirror)

**Executor:** `go/repo/main.go` builds the schema with graphql-go and executes queries in-process.

**Available Queries:**

- `repo` - Repository root with bundles, tickets, policies, contributors
- `bundle(name: String!)` - Single bundle by name
- `folder(path: String!)` - Single folder by path
- `file(path: String!)` - Single file by path
- `section(path: String!, sectionPath: [String!]!)` - Single section
- `definition(path: String!, name: String!)` - Single definition
- `contributor(id: String!)` - Single contributor
- `ticket(year: Int!, month: Int!, day: Int!, slug: String!)` - Single ticket
- `policy(id: String!)` - Single policy
- `violationKind(id: String!)` - Single violation kind
- `analyze(scope: String)` - Analyze codebase for violations

**Available Mutations:**

- `fix(scope: String)` - Apply autofixes for violations
- `ticketOpen(input: TicketOpenInput!)` - Create a new ticket
- `ticketProgress(input: TicketProgressInput!)` - Progress ticket iteration
- `ticketClose(input: TicketCloseInput!)` - Finish a ticket
- `ticketReopen(input: TicketReopenInput!)` - Reopen a ticket

**VS Code Extension Integration:**

The VS Code extension (`js/vscode/extension.ts`) includes a GraphQL client that pipes queries through the CLI:

```typescript
const result = await executeGraphQL<{ repo: { bundles: GqlBundle[] } }>(BUNDLES_QUERY);
const bundles = result.data?.repo?.bundles ?? [];
```

Helper functions provided:

- `executeGraphQL<TData, TVariables>(query, variables?)` - Execute any GraphQL query
- `fetchRepoViaGraphQL()` - Fetch complete repo data
- `fetchBundlesViaGraphQL()` - Fetch all bundles
- `fetchTicketsViaGraphQL(year?, month?, day?, status?)` - Fetch tickets with filters
- `fetchPoliciesViaGraphQL()` - Fetch all policies
- `fetchContributorsViaGraphQL()` - Fetch all contributors
- `analyzeViaGraphQL(scope?)` - Analyze codebase
- `fixViaGraphQL(scope?)` - Apply fixes

## Testing

### Overview

semio uses a multi-layered testing approach:

1. **Unit Tests** (`.test.ts`) - Domain logic testing next to modules
2. **E2E Tests** (Playwright) - Hierarchical integration testing
3. **Platform Tests** - VS Code extension, CLI, etc.

### Unit Tests

**Location:** Next to the module with `.test.ts` extension

**Example:** `semio.test.ts` tests `semio.ts` domain logic

**Policies:**

- Test domain logic in isolation
- Use vitest framework
- Mock external dependencies
- Focus on pure functions and diffs

### E2E Tests (Playwright)

**Location:** `js/semio/playwright/`

**Structure:** Hierarchical seeding matching app structure

```
playwright/
  seed.spec.ts              # Sketchpad root seed (empty stub)
  kit/
    seed.spec.ts            # Kit seed (creates kit, type, design)
    design/
      seed.spec.ts          # Design seed (requires kit)
      drag-and-drop.spec.ts # Feature test (requires design seed)
    type/
      seed.spec.ts          # Type seed (requires kit)
    quality/
      seed.spec.ts          # Quality seed (requires kit)
  docs/
    seed.spec.ts            # Docs seed
```

**Nested Seeding Pattern:**

- Tests organized hierarchically: `sketchpad → kit → {design, type, quality, ...}`
- Each `seed.spec.ts` creates minimum state for child tests
- Seeds run sequentially to build required state
- Feature tests depend on their app's seed completing first

**Policies:**

1. **ID Locators Only**: `page.locator('[id="semio.sketchpad.navbar.back"]')`
   - NEVER use text selectors, CSS classes, or other brittle selectors
   - IDs follow pattern `semio.sketchpad.{path}`
   - Ensures stable selectors across UI changes

2. **No Direct Browser API**: Only interact through Sketchpad UI elements
   - NEVER use `window.`, `document.`, or DOM manipulation
   - Ensures tests work in browser, Electron, and future platforms
   - All interactions via locators and page actions

3. **Minimal Seeding**: Each seed creates bare minimum for subtests
   - Kit seed: Creates kit, one type, one design
   - Design seed: Creates design in existing kit
   - Feature tests: Use seeded state, add only what's needed

4. **Comment Dependencies**: Start feature tests with `// Requires {app} seed to run first`

**Example Seed Pattern:**

```typescript
test.describe("design", () => {
  test("seed", async ({ page }) => {
    // Requires kit seed to run first
    await page.goto("http://localhost:5173");
    await page.locator("#semio\\.sketchpad\\.app\\.home\\.createKit").click();
    await page.locator("#semio\\.sketchpad\\.app\\.kit\\.kitApp\\.createDesign").click();
    await expect(page.getByText("New Design")).toBeVisible();
  });
});
```

**Example Feature Test:**

```typescript
test("drag type from workbench to canvas", async ({ page }) => {
  // Requires design seed to run first
  await page.goto("http://localhost:5173");
  await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
  // ... test implementation
});
```

**Known Limitation: dnd-kit Drag-and-Drop Testing**

dnd-kit's `PointerSensor` requires native browser `PointerEvent` objects. Playwright's synthetic events (`page.mouse`, `dragTo`, `dispatchEvent`) fail the `instanceof PointerEvent` check. To test drag-and-drop:

- **Validate infrastructure**: Test that draggable elements exist with `aria-roledescription="draggable"`
- **Validate post-drop state**: Use exposed commands to create pieces directly, then validate plane properties
- **Future**: Consider adding `KeyboardSensor` to enable keyboard-based drag testing

### VS Code Extension Tests

**Location:** `js/vscode/extension.test.ts`

**Framework:** VS Code Test Runner

**Policies:**

- Test validation against real VS Code API
- Use fixture files from `assets/semio/`
- Test Quick Fixes apply correct diffs
- Verify diagnostics appear at correct locations

- `I18N`: Run `tsx scripts/i18n.ts` to regenerate `reports/i18n.md`; fix all reported translation problems, add missing keys, update incomplete entries, remove unused keys, and rerun until the report is clean.

- `AUTOMATE`: Create a script to automate a task. `*.ts` for all automation tasks (use `scripts/utils.ts` for reusable code). `*.py` for python related tasks (use `@semio/engine` for reusable code).

### UI Component ID System

Every interactive UI component MUST have a unique `id` prop following the pattern `semio.sketchpad.*`. This ID serves as the central integration point for **7 major subsystems**.

#### ID Convention

**Pattern:**

```
semio.sketchpad.<context>.<feature>.<component>
```

**Policies:**

1. All IDs MUST start with `semio.sketchpad.`
2. Use kebab-case for multi-word segments
3. Follow hierarchical structure reflecting UI containment
4. Only the final DOM element receives the `id` attribute

**Examples:**

```tsx
// Navigation
id = "semio.sketchpad.navbar.back";
id = "semio.sketchpad.navbar.panelToggle.workbench";

// App-specific
id = "semio.sketchpad.app.kit.createType";
id = "semio.sketchpad.app.design.panel.workbench.typeList";
id = "semio.sketchpad.app.quality.panel.details.name";
```

#### Integration Points

##### 1. Internationalization (i18n)

**Location:** `js/semio/sketchpad/locales/{lang}.json`

Every ID automatically maps to translation keys with standard suffixes:

- `.label.normal` - Standard label text
- `.label.beginner` - Beginner-friendly description
- `.manual` - Path to manual page (e.g., `"navigation"` → `/docs/manual/navigation`)
- `.tutorial` - Path to tutorial (e.g., `"getting-started/intro"`)
- `.hotkey` - Keyboard shortcut display (e.g., `"Ctrl+J"`)

**Example translation:**

```json
{
  "semio.sketchpad.navbar.back": {
    "label": {
      "normal": "Go back",
      "beginner": "Click to go back, hold to see history"
    },
    "manual": "navigation",
    "tutorial": "getting-started/intro",
    "hotkey": "Alt+Left"
  }
}
```

**Usage in components:**

```tsx
// Automatic label via hook
const label = useLabel("semio.sketchpad.navbar.back");

// Auto-label prop
<Input id="semio.sketchpad.app.quality.name" showLabel />;

// Manual translation
const { t } = useTranslation();
const text = t("semio.sketchpad.navbar.back.label.normal");
```

**Validation:**

- Script: `tsx scripts/i18n.ts`
- Report: `reports/i18n.md`
- Checks: missing keys, unused keys, incomplete translations

##### 2. Tooltips

**Components:**

- `DescriptionTooltipContent` - Auto-resolves content from ID
- `IdSemioTooltip` - Wrapper providing ID-based tooltip
- `EnhancedTooltipContent` - Manual tooltip configuration

**Mechanism:**

```tsx
function DescriptionTooltipContent({ id }) {
  // Resolves based on expertise level:
  // EXPERT: no tooltip
  // NORMAL: .label.normal + .manual + .hotkey
  // BEGINNER: .label.beginner + .manual + .tutorial + .hotkey
}
```

**Usage:**

```tsx
// Automatic via wrapper
<Input id="semio.sketchpad.app.quality.name" showLabel />

// Manual tooltip
<Tooltip>
  <TooltipTrigger asChild>
    <Button id="semio.sketchpad.navbar.back">...</Button>
  </TooltipTrigger>
  <TooltipContent>
    <DescriptionTooltipContent id="semio.sketchpad.navbar.back" />
  </TooltipContent>
</Tooltip>

// Wrapper shorthand
<IdSemioTooltip id="semio.sketchpad.navbar.back">
  <Button>...</Button>
</IdSemioTooltip>
```

##### 3. Hotkeys

**Location:** `js/semio/sketchpad/App.tsx` (SketchpadStore)

IDs serve as paths for hotkey configuration:

- Path = UI element ID
- Value = `react-hotkeys-hook` format (`ctrl+k`, `mod+j`, etc.)
- Stored in `hotkeyOverrides: Map<HotkeyPath, HotkeyValue>`
- Persisted via Y.js

**Usage:**

```tsx
// Register hotkey
useHotkeys("ctrl+j", () => togglePanel("workbench"));

// Display in tooltip (automatic from i18n)
// Shows hotkey from `${id}.hotkey`

// Click hotkey to navigate to settings
// Handled automatically by DescriptionTooltipContent
```

##### 4. Command Origins

**Purpose:** Track which UI element triggered a command for logging, debugging, and undo/redo context.

**Pattern:**

```tsx
// ALWAYS pass origin as first parameter
executeCommand(
  "semio.kitApp.addType", // command
  "semio.sketchpad.app.kit.createType", // origin (matches button id)
  typeData, // ...args
);
```

**Origin extraction:**

```tsx
async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
  let origin: string | undefined;

  // First arg is origin if it's a semio.sketchpad.* string
  if (rest.length > 0 &&
      typeof rest[0] === "string" &&
      rest[0].startsWith("semio.sketchpad.")) {
    origin = rest[0];
    rest = rest.slice(1);
  }

  // Execute command with context
  const result = callback(context, ...rest);

  // Log with origin for debugging
  console.log(`[${origin || "unknown"}] ${command}`, result);

  return result;
}
```

**Usage in components:**

```tsx
<Button
  id="semio.sketchpad.app.kit.createType"
  onClick={() =>
    executeCommand(
      "semio.kitApp.addType",
      "semio.sketchpad.app.kit.createType", // origin = id
      newTypeData,
    )
  }
/>
```

##### 5. Tutorial Recording

**Location:** `js/semio/sketchpad/Tutorials.tsx`

Command origins enable tutorial recording and playback:

- Records sequence of commands with origins
- Highlights UI elements during playback
- Validates user actions match expected origins

**Recording structure:**

```typescript
interface TutorialRecordingEvent {
  timestamp: number;
  command: string;
  origin: string; // UI element ID
  parameters: any[];
}
```

**Playback:**

```tsx
<TutorialOverlay highlightedElementId={currentEvent.origin} description={t(`${currentEvent.origin}.label.beginner`)} />
```

##### 6. E2E Testing

**Location:** `js/semio/e2e/**/*.spec.ts`

IDs provide stable selectors for Playwright tests:

```typescript
test("create type and design", async ({ page }) => {
  // Create temporary kit
  await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();

  // Create type
  await page.locator('[id="semio.sketchpad.app.kit.kitApp.createType"]').click();

  // Navigate back
  await page.locator('[id="semio.sketchpad.navbar.back"]').click();

  // Create design
  await page.locator('[id="semio.sketchpad.app.kit.kitApp.createDesign"]').click();
});
```

**Benefits:**

- Stable selectors (don't break with CSS changes)
- Semantic (reads like documentation)
- Debuggable (clear which element failed)

##### 7. Analytics & Logging

Command origins provide analytics data:

```typescript
// All commands logged with origin
console.log(`[DEBUG] [${origin}] Command: ${command}`, parameters);

// Analytics event
analytics.track("command_executed", {
  command,
  origin,
  timestamp: Date.now(),
});
```

#### Component Authoring

**Required:**

1. Every interactive component MUST have an `id` prop
2. All `executeCommand` calls MUST include origin as first parameter

**Optional:** 3. Use `showLabel` prop for auto-label from i18n 4. Use `DescriptionTooltipContent` for auto-tooltip

**Component template:**

```tsx
interface MyComponentProps {
  id: string; // Required
  // ... other props
}

export function MyComponent({ id, ...props }: MyComponentProps) {
  const label = useLabel(id);

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button
          id={id}
          onClick={() =>
            executeCommand(
              "my.command",
              id, // origin
              data,
            )
          }
        >
          {label}
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <DescriptionTooltipContent id={id} />
      </TooltipContent>
    </Tooltip>
  );
}
```

**Helper hook pattern:**

```tsx
const useCommandExecutor = (id: string) => {
  return (command: string, ...args: any[]) => {
    executeCommand(command, id, ...args);
  };
};

// Usage
const execute = useCommandExecutor("semio.sketchpad.app.kit.createType");
<Button onClick={() => execute("semio.kitApp.addType", typeData)} />;
```

#### i18n Management

**File structure:**

```
js/semio/sketchpad/locales/
  en.json
  de.json
```

**Key structure (nested):**

```json
{
  "semio": {
    "sketchpad": {
      "navbar": {
        "back": {
          "label": {
            "normal": "Go back",
            "beginner": "Click to go back, hold to see history"
          },
          "manual": "navigation",
          "tutorial": "getting-started/intro",
          "hotkey": "Alt+Left"
        }
      }
    }
  }
}
```

**Adding new UI element:**

1. Add component with `id` prop
2. Add translation entry in `locales/en.json`
3. Add translation entry in `locales/de.json`
4. Run `tsx scripts/i18n.ts` to validate

**Validation workflow:**

```bash
# 1. Run validation
tsx scripts/i18n.ts

# 2. Check report
cat reports/i18n.md

# 3. Fix problems in locale files

# 4. Re-run validation
tsx scripts/i18n.ts
```

### Internationalization (i18n)

All user-facing text must be internationalized using i18next. The system supports English (`en`) and German (`de`) by default.

#### i18n Key Convention

Every UI element with an `id` prop automatically gets i18n keys based on that ID:

- `{id}.label` - Standard label text
- `{id}.beginner` - Beginner-friendly description (optional)
- `{id}.manual` - Manual page path (optional)
- `{id}.tutorial` - Tutorial path (optional)
- `{id}.hotkey` - Hotkey display string (optional)

#### Using i18n in Components

NEVER use `useTranslation` directly or hardcode strings. Instead:

1. Assign an `id` prop to the UI element matching the i18n key path
2. Use `<DescriptionTooltipContent>` or let tooltips automatically resolve content
3. For custom text, use `t(id)` where `id` matches the element's `id` prop

#### Translation Files

Translations live in `js/semio/locales/{lang}.json`. Keys follow dot-notation paths matching UI element IDs.

#### Tooltip Integration

The tooltip system automatically resolves i18n content from element IDs, adapting to expertise level (beginner/normal/expert).

### Styling

- NEVER use hardcoded (hex, rgb, …) or standard colors. All theme colors are explicitly defined.
- ALWAYS use colors for light mode. Dark mode is automatically derived. There are scales for the following number of colors: 2 (dark, light), 3 (dark, gray, light), 4 (dark, dark-gray-gray, light-gray-gray, light), 5 (dark, dark-gray, gray, light-gray, light), 6 (dark, dark-gray-gray, gray, light-gray-gray, light), 7 (dark, dark-6-7, dark-5-7, gray, light-5-7, light-6-7, light), 8 (dark, d-d-d-g, dark-gray, d-g-g-g, light-gray, l-g-g-g, l-l-l-g, light), 9 (dark, dark-8-9, dark-7-9, dark-gray, gray, light-gray, light-7-9, light-8-9, light), 10 (gray-100, gray-200, gray-300, gray-400, gray, gray-600, gray-700, gray-800, gray-900, light), 11 (dark, gray-100, dark-gray-gray, dark-gray, gray, light-gray, light-gray-gray, light-light-gray, l-l-l-g, gray-900, light). ALWAYS pick the one with the highest contrast.
- All closed ui elements ALWAYS have a border.
- NEVER use hardcoded pixels. ALWAYS use the standardized unit-based sizing system defined in globals.css (derived from `--spacing`):
  - Single: 1 unit - spacing between elements and between icon and element (e.g. `gap-1`)
  - Tiny: 3 units - icon size in actions, action text size (e.g. `h-tiny`, `w-tiny`, `text-tiny`)
  - Small: 5 units - actions, avatars, Strip items (e.g. `h-small`, `w-small`)
  - Medium: 7 units - buttons, toggles, inputs, sliders, steppers, Footer, table rows, Strip (e.g. `h-medium`, `w-medium`)
  - Large: 9 units - Band, Navbar (e.g. `h-large`, `w-large`)
  - Huge: 11 units - height of navigation buttons at bottom of docs pages (e.g. `h-11`)
  - Mega: 13 units - width of toggles with actions (toggles with dropdown or action buttons) (e.g. `w-mega`)
  - Giga: 15 units - reserved for future use (e.g. `w-giga`)
- NEVER use rounded corners unless a circle.
- NEVER use shadows.
- Whenever a ui element can be interacted (left/right clicked with/without hold or modifier keys, dragged, …) with, ALWAYS make it visible (different hover color, different cursor, tooltip, …).
- The ui ALWAYS consists of five levels: 1. base, 2. window, 3. panel, 4. overlay, 5. temporary. Every level is on top of the previous one. Levels base, window, panel, and temporary have progressively darker background colors (in light mode) or lighter (in dark mode). Overlay is transparent and only affects z-index. Every ui element ALWAYS uses the level context (`useLevel()` hook, `LevelProvider`) and helper functions (`getLevelBgClass()`, `getLevelHoverClass()`, `getLevelZClass()`, `getLevelActiveHoverClass()`).
- ALWAYS indicate on the element and the cursor when it is interactive. Clickable elements have a pointer cursor and a hover effect. Dragable elements have a grab cursor. While dragging, the cursor changes to a grabbing cursor.

### Horizontal Containers

- **Band**: Horizontal-only container with `h-large` height and optional horizontal scrolling; accepts `items: BandItem[]` (each item renders in a `h-medium` slot and can set wrapper `className` like `flex-1 min-w-0`).
- **Strip**: Smaller horizontal container with `h-medium` height and optional horizontal scrolling; accepts `items: StripItem[]` (each item renders in a `h-small` slot).
- **Navbar**: Non-scrollable Band with `h-large` height and fixed DOM id `navbar`; accepts `items: NavbarItem[]` and `level` for background theming.
- **Footer**: A horizontal container with `h-medium` height. Footer items are action-like and render via `ActionGroupItem` (supports `icon`, `text`, or `content`) with ordering and optional click handlers.

### Action Components

- **Action**: A clickable action with optional `icon` and/or `text`. When `text` is provided, it renders with tiny text size (`text-tiny`).
- **ActionGroup**: A group of related actions displayed together.
- **ActionGroupItem**: Items within an ActionGroup that support `icon` and/or `text` props.

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
├── reports # Generated validation reports
│ ├── i18n.json # i18n validation report
│ ├── eslint.json # ESLint linting report
│ ├── code.json # Codebase code-quality report with reason/solution metadata
│ ├── codebase.json # Full codebase snapshot for semantic ticket diffs
│ ├── typescript.json # TypeScript compiler report
│ └── ruff.json # Python Ruff linter report
├── antlr
├── assets # @semio/gh: assets for the complete repo
│ ├── badges
│ ├── contributors
│ ├── cursors
│ ├── fonts
│ ├── grasshopper
│ ├── icons
│ ├── images
│ ├── lists
│ ├── logo
│ ├── models
│ ├── repo # Repo CLI fixtures
│ └── semio
├── engineering
│ ├── dataarchitecture.pu # blueprint for sql schemas
│ ├── interfacearchitecture.txt # blueprint for json-based (rest api, graphql api, copy&paste) schemas
│ └── softwarearchitecture.txt # blueprint for object-oriented code
├── examples
│ ├── geometry
│ ├── hello-semio
│ ├── metabolism # main example with all features
│ ├── starters
│ ├── urban-patterns
│ └── voxels
├── graphql
│ └── schema.graphql # autogenerated from `py/engine/generate-schemas.ts`
├── js
│ ├── ai
│ ├── desktop
│ │ └── package.json # @semio/desktop
│ ├── docs
│ │ └── package.json # @semio/docs
│ ├── js # @semio/js: all shared js code (ui, domain logic, configs, …)
│ │ ├── .storybook
│ │ ├── elements
│ │ │ ├── aggregation
│ │ │ │ ├── Accordion.stories.tsx
│ │ │ │ ├── Accordion.tsx
│ │ │ │ ├── Collapsible.stories.tsx
│ │ │ │ ├── Collapsible.tsx
│ │ │ │ ├── Dialog.stories.tsx
│ │ │ │ ├── Dialog.tsx
│ │ │ │ ├── Resizable.stories.tsx
│ │ │ │ ├── Resizable.tsx
│ │ │ │ ├── Scrollable.stories.tsx
│ │ │ │ ├── Scrollable.tsx
│ │ │ │ ├── Tabs.stories.tsx
│ │ │ │ ├── Tabs.tsx
│ │ │ │ ├── Tree.stories.tsx
│ │ │ │ ├── Tree.tsx
│ │ │ │ └── TreeStateProvider.tsx
│ │ │ ├── display
│ │ │ │ ├── Avatar.stories.tsx
│ │ │ │ ├── Avatar.tsx
│ │ │ │ ├── HoverCard.stories.tsx
│ │ │ │ ├── HoverCard.tsx
│ │ │ │ ├── Icons.stories.tsx
│ │ │ │ ├── Icons.tsx
│ │ │ │ ├── Tooltip.stories.tsx
│ │ │ │ └── Tooltip.tsx
│ │ │ ├── docs
│ │ │ │ ├── Aside.tsx
│ │ │ │ ├── Card.tsx
│ │ │ │ ├── FileTree.tsx
│ │ │ │ ├── Page.tsx
│ │ │ │ ├── Section.tsx
│ │ │ │ ├── Steps.tsx
│ │ │ │ ├── Tabs.tsx
│ │ │ │ └── index.ts
│ │ │ ├── input
│ │ │ │ ├── Action.stories.tsx
│ │ │ │ ├── Action.tsx
│ │ │ │ ├── Button.stories.tsx
│ │ │ │ ├── Button.tsx
│ │ │ │ ├── ButtonGroup.stories.tsx
│ │ │ │ ├── ButtonGroup.tsx
│ │ │ │ ├── Combobox.stories.tsx
│ │ │ │ ├── Combobox.tsx
│ │ │ │ ├── Input.stories.tsx
│ │ │ │ ├── Input.tsx
│ │ │ │ ├── Select.stories.tsx
│ │ │ │ ├── Select.tsx
│ │ │ │ ├── Slider.stories.tsx
│ │ │ │ ├── Slider.tsx
│ │ │ │ ├── Stepper.stories.tsx
│ │ │ │ ├── Stepper.tsx
│ │ │ │ ├── Textarea.stories.tsx
│ │ │ │ ├── Textarea.tsx
│ │ │ │ ├── Toggle.stories.tsx
│ │ │ │ ├── Toggle.tsx
│ │ │ │ ├── ToggleGroup.stories.tsx
│ │ │ │ └── ToggleGroup.tsx
│ │ │ ├── navigation
│ │ │ │ ├── Breadcrumb.stories.tsx
│ │ │ │ └── Breadcrumb.tsx
│ │ │ ├── panels
│ │ │ │ ├── BottomPanel.tsx
│ │ │ │ ├── LeftPanel.tsx
│ │ │ │ ├── MiddlePanel.tsx
│ │ │ │ ├── Panel.tsx
│ │ │ │ ├── PanelGroup.tsx
│ │ │ │ └── RightPanel.tsx
│ │ │ ├── windows
│ │ │ │ ├── Diagram.tsx
│ │ │ │ ├── Scene.tsx
│ │ │ │ ├── Table.tsx
│ │ │ │ └── Window.tsx
│ │ │ ├── Canvas.stories.tsx
│ │ │ ├── Canvas.tsx
│ │ │ ├── Command.stories.tsx
│ │ │ ├── Command.tsx
│ │ │ ├── Footer.stories.tsx
│ │ │ ├── Footer.tsx
│ │ │ ├── Layout.stories.tsx
│ │ │ ├── Layout.tsx
│ │ │ ├── Navbar.stories.tsx
│ │ │ ├── Navbar.tsx
│ │ │ ├── Popover.stories.tsx
│ │ │ ├── Popover.tsx
│ │ │ └── index.ts
│ │ ├── locales
│ │ │ ├── de.json
│ │ │ └── en.json
│ │ ├── sketchpad
│ │ │ ├── Sketchpad.tsx # central barrel (Canvas padding, window containers, LayoutCanvas GoldenLayout integration, Navbar, Footer, store, kits, panels)
│ │ │ ├── Design.tsx # design app
│ │ │ ├── Docs.tsx # documentation app
│ │ │ ├── Home.tsx # home app
│ │ │ ├── Kit.tsx # kit app
│ │ │ ├── Quality.tsx # quality app
│ │ │ ├── Type.tsx # type app
│ │ │ ├── Tutorials.tsx # consolidated tutorial system
│ │ │ ├── elements.tsx # UI elements (Window kind, LevelProvider/useLevel, TransactionProvider, primitives, Diagram with d3-force layout, DIAGRAM_UNIT=48px coordinate system)
│ │ │ ├── locales
│ │ │ │ ├── de.json
│ │ │ │ └── en.json
│ │ │ └── pages # documentation pages
│ │ │ ├── index.mdx
│ │ │ ├── getting-started
│ │ │ │ ├── index.mdx
│ │ │ │ ├── installation.mdx
│ │ │ │ ├── intro
│ │ │ │ │ ├── index.mdx
│ │ │ │ │ ├── think-in-semio.mdx
│ │ │ │ │ └── why-semio.mdx
│ │ │ │ └── starter.mdx
│ │ │ ├── integrations
│ │ │ │ ├── index.mdx
│ │ │ │ ├── cloud.mdx
│ │ │ │ ├── ladybug.mdx
│ │ │ │ ├── rhino.mdx
│ │ │ │ ├── speckle.mdx
│ │ │ │ └── wasp.mdx
│ │ │ ├── manuals
│ │ │ │ ├── index.mdx
│ │ │ │ ├── grasshopper.mdx
│ │ │ │ ├── semio
│ │ │ │ │ ├── index.mdx
│ │ │ │ │ └── kit.mdx
│ │ │ │ └── sketchpad.mdx
│ │ │ ├── showcases
│ │ │ │ ├── index.mdx
│ │ │ │ └── metabolism.mdx
│ │ │ ├── theory
│ │ │ │ ├── index.mdx
│ │ │ │ ├── design-information-modeling.mdx
│ │ │ │ ├── graphs.mdx
│ │ │ │ └── kit-of-parts-architecture.mdx
│ │ │ └── tutorials
│ │ │ ├── index.mdx
│ │ │ ├── hello-semio
│ │ │ │ ├── index.mdx
│ │ │ │ ├── model-brick-set.mdx
│ │ │ │ ├── model-design.mdx
│ │ │ │ ├── save-kit.mdx
│ │ │ │ ├── show-design.mdx
│ │ │ │ └── sketch-setup.mdx
│ │ │ ├── metabolism
│ │ │ │ └── index.mdx
│ │ │ └── serial-conversion
│ │ │ ├── index.mdx
│ │ │ └── sketchpad
│ │ │ └── index.mdx
│ │ ├── components.json
│ │ ├── constants.json
│ │ ├── eslint.config.ts
│ │ ├── globals.css # Tailwind utilities, sizing tokens, GoldenLayout theme overrides (window surfaces use window background level, 1-unit splitter gaps, inset stack frames, Action-based window chrome controls)
│ │ ├── i18n.ts
│ │ ├── index.ts
│ │ ├── package.json
│ │ ├── postcss.config.ts
│ │ ├── semio.ts # all domain logic
│ │ ├── tailwind.config.ts
│ │ ├── theme.css
│ │ ├── tsconfig.json
│ │ ├── vite.config.ts
│ │ └── vitest.workspace.ts
│ ├── play
│ └── vscode
│ └── extension.ts # @semio/vscode: VSCode extension with violation diagnostics
├── jsonschema # autogenerated from `py/engine/generate-schemas.ts`
├── liveblocks
├── meta
├── net
│ ├── Semio
│ │ ├── Semio.cs # @semio/net: all .NET code
│ │ └── UserObjects
│ │ ├── github
│ │ ├── gitlab
│ │ ├── monoceros
│ │ ├── semio
│ │ └── wasp
│ ├── Semio.Grasshopper
│ │ └── Semio.Grasshopper.cs # @semio/gh: all grasshopper code
│ ├── Semio.Grasshopper.Tests
│ └── Semio.Tests
├── py
│ ├── engine
│ │ ├── build.ts # wrapper
│ │ ├── dev.ts # wrapper
│ │ ├── engine.py # @semio/engine
│ │ ├── test_engine.py # pytest
│ │ ├── package.json # monorepo integration
│ │ ├── pyproject.toml # uv member file
│ │ ├── test.ts # wrapper
│ ├── semio
│ │ ├── pyproject.toml # uv member file
│ │ ├── semio.py # @semio/semio
│ │ ├── semio.test.py # pytest
├── rb
├── rdf
├── scripts
│ ├── i18n.ts # Checks that all i18n keys are up to date and produces report under `reports/i18n.json`
│ ├── log.ts # Ticket CLI and frontmatter utilities for `tickets/{year}/{month}/{day}/{slug}.md` (ticket open, ticket progress, ticket close)
│ ├── utils.ts # General TypeScript utilities for scripts
│ └── schema.ts # Checks that all schemas are up to date and produces report under `reports/schema.json`
├── sql
│ ├── sqlite
│ │ └── schema.sql # autogenerated from `py/engine/generate-schemas.ts`
├── tickets # Development task tickets organized by date
│ ├── YEAR
│ │ ├── MONTH
│ │ │ ├── DAY
│ │ │ │ └── SLUG
│ │ │ │ │ │─ ticket.md # Ticket file with YAML frontmatter
│ │ │ │ │ └─ FILES... # Additional files for the ticket
├── yak
├── .gitignore
├── .gitmodules
├── .prettierignore
├── .prettierrc.json
├── AGENTS.md # All general ai information
├── CITATION.cff
├── CLAUDE.md # Claude specific
├── go
│ ├── go.work # Go workspace file
│ ├── mcp # MCP server exposing repo tools to LLMs
│ │ ├── go.mod
│ │ └── main.go # MCP tool schema, argument validation, path validation, repo command handlers
│ ├── server # Repo dev server
│ │ ├── go.mod
│ │ ├── go.sum
│ │ └── main.go # Repo dev server (config, SQLite schema, event bus, diff ingestion, indexing, claims, warnings, HTTP API)
│ └── repo # Go CLI binary
│ ├── go.mod
│ ├── cmd
│ │ └── repo
│ │ │ └── main.go # Repo CLI entrypoint
│ ├── internal
│ │ ├── adapters
│ │ │ ├── cli # Cobra CLI adapter + renderers
│ │ │ └── mcp # MCP adapter
│ │ ├── core # Streaming engine + requests
│ │ └── events # Streaming event schema
│ ├── cmd_benchmark.go
│ ├── cmd_preflight.go
│ ├── cmd_update.go
│ └── main.go # Repo domain and GraphQL schema
├── .venv # Centralized Python virtual environment
├── nx.json # Nx targets and plugin configs
├── package-lock.json # All javascript dependencies
├── package.json # Monorepo and workspace setup
├── pyproject.toml # Python workspace setup
├── repo.tsx # TypeScript fallback CLI (Ink + TypeScript)
├── update.ts # All-in-one dependency update script (reads .github/dependabot.yml)
├── uv.lock # Python lock file
├── README.md # GFM dev docs

In general, if the user talks about an old file, then probably there is the same file with the suffix `*.old` that is the original state.

## 📁 .devcontainer/

Devcontainer configuration and lifecycle scripts.

## 📄 .devcontainer/devcontainer.json

Devcontainer configuration with VS Code customizations, container/remote env, post-create/start/attach commands, and persisted volumes for AI auth, editor server state, and Playwright cache under `node_modules`.

## 📄 .devcontainer/post-create.sh

Devcontainer provisioning steps for dependency installs, including Playwright browser install into the shared cache path.

## 📄 .devcontainer/post-start.sh

Devcontainer start script that fixes ownership for persisted volumes, normalizes Claude Code auth storage, sets git safe directories, and activates the Python virtual environment.

## 📄 .devcontainer/post-attach.sh

Devcontainer post-attach script that builds and installs the local semio VS Code extension.

## 📁 js/

Javascript code with shared core (@semio/js) that uses storybook and exports a handful of React components (Sketchpad, Diagram, Model) for both web-based and desktop-based environments, a documentation (@semio/docs) that uses astro with starlight and mdx, and desktop (@semio/desktop) that runs in electron.

### Policies

- NEVER use inline styling. Use tailwindcss (v4). v4 uses a `theme.css` (`@semio/js/theme.css`) for theming and not `{theme:{…}}` in `tailwindconfig`.
- ALWAYS use colors defined in `@theme inline {…}` from `js/semio/globals.css`. NEVER use direct colors such as light, gray, …, dark, primary, secondary, tertiary outside of `js/semio/globals.css` and ALWAYS use semantic colors instead such as active, disabled, hover, …
- Borders use semantic kinds via Tailwind color tokens: `border-element` (hover color) and `border-window` (normal border color).
- GoldenLayout window chrome uses the window background token to match window content surfaces.
- GoldenLayout stack frames use inset strokes so window borders remain continuous on all four sides.
- ALWAYS add tooltips (normal and extensive) to all ui elements.
- ALWAYS load icons via the semantic icon layer in `@semio/assets` and NEVER import icons directly from external libraries (lucide, heroicons, .). Only reexport placeholder assets from those libraries inside `@semio/assets` and consume them through its semantic exports.

### Styling

- The ui consists of a three horizontal strips: navbar, canvas and footer. A canvas consists of windows. On top of the canvas are panels which can toggled on and off.
- Navbar panel toggles always order panels as Details, Chat, then Settings for every app.

## 📁 js/semio/

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

## 📁 js/semio/sketchpad/

Sketchpad app modules, state machine wiring, and shared app surfaces for Home, Kit, Design, Type, Quality, Docs, and Feedback.

## 📄 js/semio/sketchpad/elements.tsx

`Table` supports row-level hover callbacks for app hover state dispatch.

## 📄 js/semio/sketchpad/Home.tsx

Home app hover state is stored in the Sketchpad state machine and updated via hover commands for table rows.

## 📄 js/semio/sketchpad/Kit.tsx

Kit app hover state covers all artifact kinds and is updated via table and diagram hover dispatch.

## 📄 js/semio/sketchpad/Sketchpad.tsx

Home command hooks forward hover events, including clear, into the Sketchpad state machine.

## 📁 js/vscode/

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

- Real-time validation using `validateKit()` from `@semio/js`
- Problem-to-diagnostic mapping with entity location highlighting
- Quick Fix code actions applying `KitDiff` fixes

### Sidebar Views

Tree data providers for tickets, policies, contributors, and commands with search/filter capabilities.
Contributor tree nodes group contributions into commits, bundles, tickets by date, and files by folder with navigation commands for tickets, commits, bundles, and files.
Contributor commit items expose inline copy SHA and open in GitHub actions.
Sections view provider resolves the active editor path, calls `repo section list`, normalizes GraphQL section ranges into a nested SectionInfo tree, opens section locations on selection, binds F2 rename, supports drag-and-drop section moves, maps JSON object keys into the section tree, and routes rename/create-child/delete actions through repo section commands with refresh on active editor and document changes.

### Commands

Command tree nodes are derived from command ids by segmenting the action name into group paths, with leaf commands attached to the last group node.
Command search includes command ids, titles, and segment names while group matches expand full subtrees.

### Tickets

Ticket tree items use `ticketOpen` and `ticketClosed` context values for inline close or reopen actions that apply to the clicked ticket, surface commit nodes from ticket commits, and limit hover tooltips to the summary or prompt text.

### Code Actions

- `RepoCodeActionProvider`: Quick fixes for violation diagnostics that run `repo fix <path>`
- `KitCodeActionProvider`: Quick fixes for kit validation problems that apply diff-based fixes

## 📄 js/vscode/extension.ts

Extension activation, repo CLI command execution helpers, GraphQL client piping through the repo CLI, ticket command prompts aligned to repo ticket inputs including LLM and UI selection on ticket open, gitignore-aware analysis gating with `.semio-repo/` and `assets/repo/` exclusions for diagnostics, and section tree normalization for the Sections view based on repo section list output with line-only range start/end values.

## 📄 js/vscode/package.json

VS Code extension manifest with unscoped name for vsce packaging, command contributions, scripts, and engine compatibility.

## 📁 js/vscode/generated/

GraphQL codegen outputs for the VS Code extension.

## 📄 js/vscode/generated/graphql.ts

Generated GraphQL types and typed documents for the VS Code extension.

## 📁 graphql/repo/

Repo CLI GraphQL schema mirror for tooling.

## 📄 graphql/repo/schema.graphql

GraphQL schema mirror of the repo CLI schema with TicketUI enum and ticket UI fields for VS Code codegen and typed documents.

## 📁 sql/sqlite/repo/

SQLite schema definitions for repo exports.

## 📄 sql/sqlite/repo/schema.sql

SQLite schema with ticket UI storage alongside LLM and commit metadata.

## 📁 go/repo/

Repo CLI source with GraphQL schema build/execution, ticket workflows, and MCP server mode.

## 📄 go/repo/main.go

Single-file repo CLI implementation with embedded engine, streaming command registry and emitter event model for CLI/MCP/VS Code adapters, CLI command definitions with GraphQL query strings for repo operations (including export and section integrate commands), ticket workflows with optional noIssue/planPath inputs, `CONTINUE`/`NOTICKET` keyword handling, and `YYYY/MM/DD/SLUG` identifiers, TicketUI enum normalization for GraphQL inputs, range resolution returning line/column positions with Position subfield selections in GraphQL queries, ticket UI validation, MCP tool handlers with cursor/limit paging over streamed item events, ticket title updates that rename folder paths, ticket.md creation and summary injection with plan.md seeding, per-iteration semantic ticket diffs for bundles/folders/files/sections/definitions with full-line metrics for added/deleted scopes and diff metrics for modified scopes, ticket workspace file exclusion on close, ticket GitHub project linking on create/reopen, GitHub issue/comment prompt and `# 🔍 Summary` heading formatters, bundle label derivation from bundle diffs with README/AGENTS exclusions, semantic metrics comments with status icons and `+added`/`-removed` counts, benchmark/preflight/update operational command wiring, codebase snapshot export to `.semio-repo/reports/codebase.json` from `repo analyze`, file/folder streaming and folder file listing filtered by gitignored paths plus `.semio-repo/` and `assets/repo/` exclusions, comment policy scanning with string/template literal awareness and grouped inline comment violations, JSON section parsing, shell script language registration, GraphQL file section resolution, legacy ticket directory fallback for reads, and contributor aggregation from tickets and source headers.

## 📁net/

C# code with the core library (`Semio.cs`) and Grasshopper plugin (`Semio.Grasshopper.cs`).

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

Python code with the engine (@semio/engine) for schema generation and validation.

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
