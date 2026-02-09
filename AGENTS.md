<!-- IMPORTANT -->

ALWAYS work inside a ticket. ALWAYS use semio-repo mcp (or the cli `./semio-repo/cli/cli`) for repo-specific infrastructure. ALWAYS start by listing the current goal tree with `goal_tree` (or `./semio-repo/cli/cli goal tree`). Create a new ticket with mcp tool `ticket_open` (or `./semio-repo/cli/cli ticket open <goal-id> <title> <prompt> <client> <llm> --draft <draft-id>? --parent <parent-ticket-id>?`). This creates a `.semio-repo/tickets/YYYY/MM/DD/TICKETSLUG` folder along with `ticket.md` in it. NEVER answer directly in the chat and ALWAYS document everything (todos, changes, summary, etc) in `ticket.md`. ALWAYS use the mcp tool `ticket_close` (or `./semio-repo/cli/cli ticket close <ticket-id> <summary> <files...>`) to finish the ticket along with the summary and at all the files you worked on (created, updated or removed). When a dev sends a new message to the chat ALWAYS reopen the same ticket with mcp tool `ticket_reopen` (or `./semio-repo/cli/cli ticket reopen <ticket-id> <prompt> <client> <llm> --draft <draft-id>? --title <new-title?> --goal <new-goal-id>? --parent <new-parent-ticket-id>?`).
Create a goal with mcp tool `goal_open`(or `./semio-repo/cli/cli goal open <title> <description> <prompt> <client> <llm> --due <due-date>? --parent <parent-goal>?`). NEVER create a goal when not excplicly asked to do so. Close a goal with mcp tool `goal_close`(or`./semio-repo/cli/cli goal close <GOALSLUG/SUBGOALSLUG> <summary>`). The due date is a date in the format `YYYY-MM-DD`. Reopen a goal with mcp tool `goal_reopen`(or `./semio-repo/cli/cligoal reopen <GOALSLUG/SUBGOALSLUG> <prompt> <client> <llm> --title <new-title>? --description <new-description>? --due <new-due-date>? --parent <new-parent-goal>?`).
A ticket id is `YYYY/MM/DD/TICKETSLUG`. A goal id is `GOALSLUG/SUBGOALSLUG/...`. A title MUST be titleized (e.g. "Some Title on Something") and NEVER be a slug or all caps. Available LLMs are: `opus-4-6`, `opus-4-5`, `sonnet-5`, `sonnet-4-5`, `haiku-4-5`, `gemini-3-pro`, `gemini-3-flash`, `gpt-5-2-codex`, `gpt-5-mini`, `swe-1-5`, `gpt-5-3-codex`. Available Clients are: `copilot-chat`, `windsurf-chat`, `claude-code`, `codex`, `cursor-chat`, `antigravity-chat`, `droid`.

- Multiple agents and a developer ALWAYS work on the same codebase at the same time. NEVER use `git stash`, `git stash pop`, `git checkout`, … because it will mess up others work and worst-case delete their work.
- The codebase in under design and development and not used in production yet. There are many inconsistencies that need to be refactored. ALWAYS use clean mechanisms that might require large refactorings and NEVER care about backwards compatibility.
- For every task you are working on, you MUST update the dev docs (`README.md` and `AGENTS.md`). Every key decision and mechanism ALWAYS needs to be documemented. Every feature, decision MUST be undocumented/uncommented in the code and MUST be documented in the dev docs (AGENTS.md and README.md). The documentation ALWAYS happens three times:

1. Under `# Docs` in the bundle or folder `README.md` where it is described from junior-developer perspective (mechanism explanation and reasoning behind the decision, how theory links to implementation, etc).
2. Under `# Software Requirements Specification` in AGENTS.md which indexes distributed spec files. Specs are placed at their most specific location: `README.md` at the bundle root (under `# Specs`), `README.md` in folders (under `# Specs`), Header Specs region in files, comments after section start markers, and language-native docstrings for definitions. Spec text uses RFC 2119 keywords and MUST NOT contain implementation-specific syntax.
3. Under `# Codebase` in AGENTS.md where it is described from senior-developer perspective (framework-mechanisms, consice technical terms without explanation, implementation details, etc). The section has the same header structure as the files and folders. All files and folders are flat with `## PATH` e.g. `## semio/js/semio/sketchpad/` or `## semio/net/Semio.cs`
   The purpose of the dev docs is to understand the codebase. NEVER add reasoning or process related (such as what changed, why, how, … - this is part of the log) to the dev docs.

This document MUST ALWAYS BE followed unless explicitly asked to do otherwise.

<!-- IMPORTANT -->

# Software Requirements Specification

Specs are distributed across source code at their most specific location:

- **Bundle-level**: README.md at the bundle root (under `# Specs`)
- **Folder-level**: README.md in the folder (under `# Specs`)
- **File-level**: Header Specs region
- **Section-level**: Comments after section start markers
- **Definition-level**: Language-native docstrings

Spec text MUST use RFC 2119 keywords (MUST, SHOULD, SHALL, MAY, REQUIRED, RECOMMENDED, OPTIONAL) and MUST NOT contain implementation-specific syntax (backtick-wrapped code, function call patterns).

Spec locations:

- semio/README.md - Domain model (Kit, Design, Type, Connection, Piece, Connector, Model, Attribute, Tag, Concept, Plane, Url, Quality, Benchmark, Port, Author, Layer, Group, Prop, Stat)
- semio-repo/cli/README.md - Repo CLI (Code Hygiene, Sections, Move/Integrate/Extract, Tree, Tooling, Ticket, Goal, Repo Dev Server, Repo Tooling, Artifact Kind Derivation, MCP Tools, Contributor, CLI, Ticket UX)
- semio-repo/vscode/README.md - VS Code Extension (Sidebar, Tickets, Commands, Diagnostics, Contributors, Sections View)
- semio/engine/README.md - Engine
- .devcontainer/README.md - Devcontainer
- semio/js/sketchpad/README.md - Sketchpad UI (State Management, Toolbar, Interaction State, Borders, Windows)

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

Each bundle and significant folder has a `README.md` with `# Summary`, `# Docs`, and `# Specs` sections. See those files for detailed documentation:

## 📁.devcontainer/

See [.devcontainer/README.md](.devcontainer/README.md) for devcontainer configuration, lifecycle scripts, persistence, and extension install docs.

## 📁semio-repo/

See [semio-repo/README.md](semio-repo/README.md) for repo tooling overview.

## 📁semio-repo/cli/

See [semio-repo/cli/README.md](semio-repo/cli/README.md) for repo CLI implementation, policies, code hygiene, tree, tickets, and MCP docs.

## 📄semio-repo/cli/main.go

Repo CLI single-file entrypoint with command registry, GraphQL execution, policy enforcement, and MCP server.

## 📄semio-repo/cli/main_test.go

Consolidated Go test suite for the repo CLI and tooling behavior.

## 📁semio-repo/vscode/

See [semio-repo/vscode/README.md](semio-repo/vscode/README.md) for VS Code extension, sidebar views, diagnostics, and section explorer docs.

## 📄semio-repo/vscode/extension.ts

Extension activation entrypoint with sidebar views, filter state, and tree data providers.

## 📁semio-repo/graphql/

See [semio-repo/graphql/README.md](semio-repo/graphql/README.md) for GraphQL schema mirror docs.

## 📁semio-repo/sqlite/

See [semio-repo/sqlite/README.md](semio-repo/sqlite/README.md) for SQLite schema docs.

## 📁semio-repo/server/

See [semio-repo/server/README.md](semio-repo/server/README.md) for repo dev server docs.

## 📁semio/

See [semio/README.md](semio/README.md) for domain model specs.

## 📁semio/js/

See [semio/js/README.md](semio/js/README.md) for JavaScript ecosystem, shared React components, Sketchpad architecture, state management, app plugins, store hierarchy, and UI system docs.

## 📁semio/js/sketchpad/

See [semio/js/sketchpad/README.md](semio/js/sketchpad/README.md) for Sketchpad app modules and state machine wiring docs.

## 📁semio/js/vscode/

VS Code extension providing violation diagnostics and kit validation. See [semio-repo/vscode/README.md](semio-repo/vscode/README.md).

## 📁semio/net/

See [semio/net/README.md](semio/net/README.md) for .NET core library and Grasshopper plugin docs.

## 📁semio/py/

See [semio/py/README.md](semio/py/README.md) for Python library docs.

## 📁semio/engine/

See [semio/engine/README.md](semio/engine/README.md) for Python engine docs.

## 📁semio/go/

See [semio/go/README.md](semio/go/README.md) for Go library docs.

## 📁semio/rs/

See [semio/rs/README.md](semio/rs/README.md) for Rust library docs.

## 📁semio/desktop/

See [semio/desktop/README.md](semio/desktop/README.md) for Electron desktop app docs.

## 📁semio/docs/

See [semio/docs/README.md](semio/docs/README.md) for documentation site docs.

## 📁semio/play/

See [semio/play/README.md](semio/play/README.md) for playground app docs.

## 📁semio/sketchpad/

See [semio/sketchpad/README.md](semio/sketchpad/README.md) for standalone Sketchpad web app docs.

## 📁semio/assets/

See [semio/assets/README.md](semio/assets/README.md) for shared assets docs.

## 📁semio/sqlite/

See [semio/sqlite/README.md](semio/sqlite/README.md) for SQLite schema docs.

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
