# 📚 Docs

## [🧰semiorepo🛂graphql](semiorepo://bundle/semio-repo/graphql)

## schema.graphql

GraphQL schema mirror of the repo CLI schema with TicketUI enum and ticket UI fields for VS Code codegen and typed documents.

## [🧰semiorepo⌨️server](semiorepo://bundle/semio-repo/server)

The CLI sends unified diffs or file snapshots; the server parses them, reindexes affected files, updates claims, and emits conflict warnings and precommit blockers.
HTTP endpoints cover ticket lifecycle commands, diff ingestion, precommit checks, indexing, and read-only queries for warnings, breachs, and scopes.
Webhook receivers enrich GitHub issue events, and Discord notifications format prompt/summary headings to match ticket workflow conventions.

## [🧰semiorepo🛂sqlite](semiorepo://bundle/semio-repo/sqlite)

## schema.sql

SQLite schema with ticket UI storage alongside LLM and commit metadata.

## [🧰semiorepo🖱️vscode](semiorepo://bundle/semio-repo/vscode)

## .vscode-test.mjs

VS Code test-cli configuration entrypoint that defines the compiled test glob and Electron launch arguments for extension tests.

## extension.test.ts

VS Code extension integration tests covering command registration, diagnostics, sidebar view contributions, filter state behavior, and monorepo tree provider roots.

## extension.ts

Extension activation entrypoint that registers the two sidebar views (Monorepo and Filter) backed by tree data providers wired to a shared filter state source.
The Filter view exposes one item per filter kind with emoji + name labels, tooltip descriptions, and emoji-only menu actions for option toggles.
The Monorepo view applies the shared filter state across all branches and uses GraphQL-backed data retrieval via the repo CLI executor.
Section child rendering filters GraphQL section-interface children to section-typed nodes before building section rows so definitions are rendered only in definition rows.
URI resolution uses the `semiorepo://` scheme. the `semio.navigate` command accepts either a `semiorepo://` uri or a plain artifact id and resolves it to the appropriate resource. ticket and goal uris resolve directly to filesystem paths. file, folder, bundle, technology, section, and definition uris resolve via a tree node cache built from the cli `tree --json` output. the `semio.navigateto` command shows a quick pick of all cached tree nodes. a `vscode.urihandler` is registered for the `semiorepo` scheme to handle external uri navigation. all tree items (including goals) have click-to-navigate commands.

## package.json

VS Code extension manifest with unscoped name for vsce packaging, command contributions, scripts, and engine compatibility for Cursor support.

## generated/

GraphQL codegen outputs for the VS Code extension.

## generated/graphql.ts

Generated GraphQL types and typed documents for the VS Code extension.

## Command Tree

The command browser groups actions by command families and subcommands, using the same command breakdown as the CLI so users learn one structure across tools.
Search matches either command labels or group names, and matching a group keeps its full subtree visible for quick discovery.

## Sections Explorer

The VS Code extension adds a Sections view to the built-in Explorer that lists nested regions for the active file via the repo section list.
Selecting a section navigates to its start, F2 triggers rename, drag-and-drop moves sections, and inline actions use repo commands to create child sections, rename sections, or delete them.
The view refreshes on editor focus and text changes so the tree stays aligned with the current file structure.
JSON files surface object keys as section entries so structured config files are navigable in the same tree.

## Breach Diagnostics

Shows policy breachs as diagnostics for all supported file types (TypeScript, JavaScript, JSON, Python, C#, Go):

- Loads cached breachs on file open for immediate feedback
- Re-runs `repo analyze` on file save and updates diagnostics
- Quick Fix actions to apply automated fixes via `repo fix`
- Promotes preview editors opened from the Problems list into regular tabs for save-ready edits

## Kit Validation

Real-time validation for kit JSON files with Quick Fix code actions that apply `KitDiff`-based fixes.

## Sidebar

Tree views for tickets, policies, contributors, and commands with search and filter support.
Section tree expansion in the Monorepo view treats GraphQL section children as mixed interface nodes and only renders nodes identified as sections (`__typename: Section` or `section:` IDs), preventing definition entries from appearing twice.

## Tickets

Ticket tree items expose inline close or reopen actions that apply to the clicked ticket, list commit entries derived from ticket and interaction commits, and keep hover tooltips limited to the ticket description.

## [🧰semiorepo⌨️cli💻maingo](semiorepo://file/semio-repo/cli/main.go)

Monorepo CLI tool for repository management, analysis and code generation.

## [🧰semiorepo⌨️cli💻maingo🔖preamble](semiorepo://section/Preamble)

Package declaration and dependency imports for the semio-repo CLI.

## [🧰semiorepo⌨️cli💻maingo🔖engineevents](semiorepo://section/Engine%20Events)

Event types and payload structures for the engine event stream.
Kind represents a kind value.

## [🧰semiorepo⌨️cli💻maingo🔖engineerrors](semiorepo://section/Engine%20Errors)

Error code constants for engine failure classification.
ErrorCode represents a error code value.

## [🧰semiorepo⌨️cli💻maingo🔖enginerequests](semiorepo://section/Engine%20Requests)

Request command types and argument structures for engine invocation.
Command represents a command value.

## [🧰semiorepo⌨️cli💻maingo🔖engine](semiorepo://section/Engine)

Core engine that dispatches requests and emits events over a channel.
GraphQLExecutor defines the interface contract for graph q l executor operations.

## [🧰semiorepo⌨️cli💻maingo🔖cliadapter](semiorepo://section/Cli%20Adapter)

CLI adapter that wires cobra commands to the engine and renders output.
Config holds the data fields for a config record.

## [🧰semiorepo⌨️cli💻maingo🔖utilities](semiorepo://section/Utilities)

General-purpose utility functions for time parsing and formatting.

## [🧰semiorepo⌨️cli💻maingo🔖models](semiorepo://section/Models)

Data model types for tickets, goals, and tree representation.
TicketNode holds the data fields for a ticket node record.

## [🧰semiorepo⌨️cli💻maingo🔖monorepotreetypes](semiorepo://section/Monorepo%20Tree%20Types)

Tree node kinds, filter criteria, and matching logic for monorepo tree queries.
EntityKinds holds the data fields for a EntityKinds record.

## [🧰semiorepo⌨️cli💻maingo🔖treelogic](semiorepo://section/Tree%20Logic)

Tree construction, filtering, searching, and rendering for goals, sections, and monorepo nodes.

## [🧰semiorepo⌨️cli💻maingo🔖monorepotree](semiorepo://section/Monorepo%20Tree)

Monorepo tree builder that assembles all entity nodes into a unified tree.
TreeBuildOptions holds the data fields for a tree build options record.

## [🧰semiorepo⌨️cli💻maingo🔖querycache](semiorepo://section/Query%20Cache)

Local Bleve index under .semio-repo/cache for keyword search. Uses composite git fingerprint (supertechnology HEAD, dirty state, submodule pointers and working state) for invalidation. Supports incremental updates via git diff.

## [🧰semiorepo⌨️cli💻maingo🔖treecache](semiorepo://section/Tree%20Cache)

Gzip-compressed JSON cache of the full TreeNode tree under .semio-repo/cache. Uses same git fingerprint as Query Cache for invalidation. Saves ~95% of tree build time on cache hit.

## [🧰semiorepo⌨️cli💻maingo🔖clirenderers](semiorepo://section/CLI%20Renderers)

Stream renderers that format engine events for NDJSON, human-readable, and markdown output.
StreamRenderer defines the interface contract for stream renderer operations.

## [🧰semiorepo⌨️cli💻maingo🔖ansi](semiorepo://section/ANSI)

ANSI escape code constants for terminal colorization.

## [🧰semiorepo⌨️cli💻maingo🔖mermaid](semiorepo://section/Mermaid)

Mermaid diagram generation for LOC visualizations as treemap-beta strings.

## [🧰semiorepo⌨️cli💻maingo🔖graphqltypes](semiorepo://section/GraphQL%20Types)

GraphQL-facing domain types, enums, constants, and entity node implementations.
Node defines the interface contract for node operations.

## [🧰semiorepo⌨️cli💻maingo🔖drafts](semiorepo://section/Drafts)

Draft management for creating, listing, and deleting draft file sets.
Draft holds the data fields for a draft record.

## [🧰semiorepo⌨️cli💻maingo🔖graphqlinputtypes](semiorepo://section/GraphQL%20Input%20Types)

GraphQL mutation input types for tickets, goals, todos, and contributors.
FileListInput holds the data fields for a file list input record.

## [🧰semiorepo⌨️cli💻maingo🔖providers](semiorepo://section/Providers)

Composable provider interfaces and implementations for source control, management, sandbox, language, and editor integrations.
Provider interfaces define contracts for composable integrations.
SourceControlProvider defines the interface for source control operations (GitHub, GitLab, BitBucket, ...).

## [🧰semiorepo⌨️cli💻maingo🔖providerinterfaces](semiorepo://section/Provider%20Interfaces)

Provider interfaces define contracts for composable integrations.
SourceControlProvider defines the interface for source control operations (GitHub, GitLab, BitBucket, ...).

## [🧰semiorepo⌨️cli💻maingo🔖githubmanagementprovider](semiorepo://section/GitHub%20Management%20Provider)

GitHub implementation of ManagementProvider using the gh CLI.
GitHubManagementProvider holds the data fields for a github management provider record.

## [🧰semiorepo⌨️cli💻maingo🔖githubsourcecontrolprovider](semiorepo://section/GitHub%20Source%20Control%20Provider)

GitHub implementation of SourceControlProvider using the gh CLI.
GitHubSourceControlProvider holds the data fields for a github source control provider record.

## [🧰semiorepo⌨️cli💻maingo🔖devcontainersandboxprovider](semiorepo://section/Devcontainer%20Sandbox%20Provider)

Devcontainer implementation of SandboxProvider.
DevcontainerSandboxProvider holds the data fields for a devcontainer sandbox provider record.

## [🧰semiorepo⌨️cli💻maingo🔖editorproviders](semiorepo://section/Editor%20Providers)

Editor provider implementations for Copilot, Cursor, Windsurf, Claude Code, Droid, Codex, Antigravity.
CopilotEditorProvider holds the data fields for a copilot editor provider record.

## [🧰semiorepo⌨️cli💻maingo🔖providerregistry](semiorepo://section/Provider%20Registry)

Registry functions for accessing all available providers.
AllEditorProviders returns all registered editor providers.

## [🧰semiorepo⌨️cli💻maingo🔖types](semiorepo://section/Types)

Scope, todo, breach, and ticket metric types for the repository model.
ScopeKind represents a scope kind value.

## [🧰semiorepo⌨️cli💻maingo🔖languages](semiorepo://section/Languages)

Language plugin registry with parsers for sections, definitions, comments, imports, and headers.
LanguagePlugin defines the interface contract for language plugin operations.

## [🧰semiorepo⌨️cli💻maingo🔖typescript](semiorepo://section/TypeScript)

TypeScript language plugin with section, definition, comment, and import support.
TypeScriptLanguage holds the data fields for a type script language record.

## [🧰semiorepo⌨️cli💻maingo🔖go](semiorepo://section/Go)

Go language plugin with section, definition, import, and package support.
GoLanguage holds the data fields for a go language record.

## [🧰semiorepo⌨️cli💻maingo🔖c](semiorepo://section/C/)

C# language plugin with section, definition, and import support.
CSharpLanguage holds the data fields for a c sharp language record.

## [🧰semiorepo⌨️cli💻maingo🔖json](semiorepo://section/JSON)

JSON language plugin with section parsing via embedded comment keys.
JSONLanguage holds the data fields for a j s o n language record.

## [🧰semiorepo⌨️cli💻maingo🔖markdown](semiorepo://section/Markdown)

Markdown language plugin with heading-based section parsing.
MarkdownLanguage holds the data fields for a markdown language record.

## [🧰semiorepo⌨️cli💻maingo🔖rust](semiorepo://section/Rust)

Rust language plugin with section, definition, and import support.
RustLanguage holds the data fields for a rust language record.

## [🧰semiorepo⌨️cli💻maingo🔖ruby](semiorepo://section/Ruby)

Ruby language plugin with section, definition, and import support.
RubyLanguage holds the data fields for a ruby language record.

## [🧰semiorepo⌨️cli💻maingo🔖shell](semiorepo://section/Shell)

Shell language plugin with section and comment support.
ShellLanguage holds the data fields for a shell language record.

## [🧰semiorepo⌨️cli💻maingo🔖toml](semiorepo://section/TOML)

TOML language plugin with section heading and comment support.
TomlLanguage holds the data fields for a toml language record.

## [🧰semiorepo⌨️cli💻maingo🔖yaml](semiorepo://section/YAML)

YAML language plugin with section heading and comment support.
YamlLanguage holds the data fields for a yaml language record.

## [🧰semiorepo⌨️cli💻maingo🔖sql](semiorepo://section/SQL)

SQL language plugin with section and comment support.
SqlLanguage holds the data fields for a sql language record.

## [🧰semiorepo⌨️cli💻maingo🔖graphql](semiorepo://section/GraphQL)

GraphQL language plugin with section and comment support.
GraphqlLanguage holds the data fields for a graphql language record.

## [🧰semiorepo⌨️cli💻maingo🔖codebasetypes](semiorepo://section/Codebase%20Types)

Internal metric, contributor, ticket, policy, breach, and tree node types for codebase analysis.
BundleMetricsInternal holds the data fields for a bundle metrics internal record.

## [🧰semiorepo⌨️cli💻maingo🔖utils](semiorepo://section/Utils)

File system, git, path normalization, and formatting utilities.

## [🧰semiorepo⌨️cli💻maingo🔖sections](semiorepo://section/Sections)

Section parsing, JSON section manipulation, and section lookup utilities.
ParseCodeSections parses the input and returns the code sections result.

## [🧰semiorepo⌨️cli💻maingo🔖policies](semiorepo://section/Policies)

Policy definitions, context, checkers, and individual policy implementations.
PolicyFunc is a function type for policy func callbacks.

## [🧰semiorepo⌨️cli💻maingo🔖codebase](semiorepo://section/Codebase)

Codebase builder that assembles bundles, folders, files, sections, definitions, contributors, tickets, policies, and breachs.
CodebaseContext holds the data fields for a codebase context record.

## [🧰semiorepo⌨️cli💻maingo🔖tickets](semiorepo://section/Tickets)

Ticket and goal lifecycle management including creation, closing, reopening, deletion, and diff computation.
GetTicketsDir returns the tickets dir of the value.

## [🧰semiorepo⌨️cli💻maingo🔖ticketfileresolution](semiorepo://section/Ticket%20File%20Resolution)

Ticket file input normalization for close operations.
Specs: Accept repo-relative paths, absolute paths, semiorepo file URIs, and file artifact IDs.
Docs: Used by ticket close to map file identifiers to repo paths.

## [🧰semiorepo⌨️cli💻maingo🔖sqliteexport](semiorepo://section/SQLite%20Export)

SQLite export functions for persisting repository data.
ExportResult holds the data fields for a export result record.

## [🧰semiorepo⌨️cli💻maingo🔖graphqlcontextport](semiorepo://section/GraphQL%20Context%20Port)

GraphQL context port adapter for request context propagation.
RepoContext defines the interface for repo context operations.

## [🧰semiorepo⌨️cli💻maingo🔖graphqlresolver](semiorepo://section/GraphQL%20Resolver)

GraphQL resolver implementation binding queries to data sources.
Resolver holds the data fields for a resolver record.

## [🧰semiorepo⌨️cli💻maingo🔖defaultcontext](semiorepo://section/Default%20Context)

Default context factory providing baseline resolver context.

## [🧰semiorepo⌨️cli💻maingo🔖graphqlexecutor](semiorepo://section/GraphQL%20Executor)

GraphQL executor dispatching queries against the schema.

## [🧰semiorepo⌨️cli💻maingo🔖schemabuilder](semiorepo://section/Schema%20Builder)

Schema builder constructing the GraphQL schema from type definitions.

## [🧰semiorepo⌨️cli💻maingo🔖queryresolvers](semiorepo://section/Query%20Resolvers)

Query resolver methods implementing GraphQL read operations.
Query executes the query query.

## [🧰semiorepo⌨️cli💻maingo🔖mutationresolvers](semiorepo://section/Mutation%20Resolvers)

Mutation resolver methods implementing GraphQL write operations.
Mutation performs the mutation operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🔖entityresolvers](semiorepo://section/Entity%20Resolvers)

Entity resolver methods implementing GraphQL entity lookups.

## [🧰semiorepo⌨️cli💻maingo🔖resolverinterfaces](semiorepo://section/Resolver%20Interfaces)

Resolver interface definitions for the GraphQL server.
QueryResolver defines the interface for query resolver operations.

## [🧰semiorepo⌨️cli💻maingo🔖mcp](semiorepo://section/Mcp)

MCP protocol handlers for the model context protocol server.

## [🧰semiorepo⌨️cli💻maingo🔖args](semiorepo://section/Args)

Argument parsing utilities for CLI and MCP commands.

## [🧰semiorepo⌨️cli💻maingo🔖paths](semiorepo://section/Paths)

Path resolution utilities for file and folder operations.

## [🧰semiorepo⌨️cli💻maingo🔖graphql](semiorepo://section/GraphQL)

GraphQL query and mutation string constants.

## [🧰semiorepo⌨️cli💻maingo🔖handlers](semiorepo://section/Handlers)

Request handler functions for CLI and MCP operations.

## [🧰semiorepo⌨️cli💻maingo🔖mcpresourceshandlers](semiorepo://section/Mcp%20Resources%20Handlers)

MCP resource handler functions for resource listing and reading.

## [🧰semiorepo⌨️cli💻maingo🔖cli](semiorepo://section/Cli)

GraphQL helper functions for query construction and execution.

## [🧰semiorepo⌨️cli💻maingo🔖graphqlhelpers](semiorepo://section/GraphQL%20Helpers)

GraphQL helper functions for query construction and execution.

## [🧰semiorepo⌨️cli💻maingo🔖analyzecommand](semiorepo://section/Analyze%20Command)

Analyze command implementation for policy breach detection.

## [🧰semiorepo⌨️cli💻maingo🔖fixcommand](semiorepo://section/Fix%20Command)

Fix command implementation for automatic policy breach repair.

## [🧰semiorepo⌨️cli💻maingo🔖missingutilities](semiorepo://section/Missing%20Utilities)

Utility functions that are missing from the main codebase.
ScopeToFiles performs the scope to files operation.

## [🧰semiorepo⌨️cli💻maingo🔖resolvermethods](semiorepo://section/Resolver%20Methods)

Resolver method implementations for GraphQL field resolution.
Bundles performs the bundles operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🔖missingtoolfunctions](semiorepo://section/Missing%20Tool%20Functions)

Tool function stubs for unimplemented features.
ToolAnalyze performs the tool analyze operation.

## [🧰semiorepo⌨️cli💻maingo🔖benchmarkcommand](semiorepo://section/Benchmark%20Command)

Benchmark command implementation for performance measurement.

## [🧰semiorepo⌨️cli💻maingo🔖hooks](semiorepo://section/Hooks)

Hook event types, context, handler, and blocked tool patterns for git and agent lifecycle hooks.
HookEvent represents a lifecycle event kind for hooks.

## [🧰semiorepo⌨️cli💻maingo🔖configure](semiorepo://section/Configure)

Configure command auto-generates native hook configs for all supported clients.
ClientHookMapping maps client names to their native event configuration format.

## [🧰semiorepo⌨️cli💻maingo🔖updatecommand](semiorepo://section/Update%20Command)

Update command implementation for dependency updates.

## [🧰semiorepo⌨️cli💻maingo🔖fileutilities](semiorepo://section/File%20Utilities)

File utility functions for reading, writing and path manipulation.
MoveFile performs the move file operation.

## [🧰semiorepo⌨️cli💻maingo🔖goals](semiorepo://section/Goals)

Goal management functions for planning and tracking.
GetRepoGoalsDir retrieves and returns the repo goals dir.

## [🧰semiorepo⌨️cli💻maingo🔖todos](semiorepo://section/Todos)

Todo tracking functions for task management.
GetTodos retrieves and returns the todos.

## [🧰semiorepo⌨️cli💻maingo🔖entityrendering](semiorepo://section/Entity%20Rendering)

Artifact ID parsing and resolution utilities.
SemanticId holds the data fields for a semantic id record.

## [🧰semiorepo⌨️cli💻maingo🔖artifactid](semiorepo://section/Artifact%20ID)

Artifact ID parsing and resolution utilities.
SemanticId holds the data fields for a semantic id record.

## [🧰semiorepo⌨️cli💻maingo🔖entityrendering](semiorepo://section/Entity%20Rendering)

Entity rendering functions for formatted output generation.

## [🧰semiorepo⌨️cli💻maingo✂️kind](semiorepo://definition/semio-repo/cli/main.go/Kind)

Kind represents a kind value.

## [🧰semiorepo⌨️cli💻maingo✂️event](semiorepo://definition/semio-repo/cli/main.go/Event)

Event holds the data fields for a event record.

## [🧰semiorepo⌨️cli💻maingo✂️progress](semiorepo://definition/semio-repo/cli/main.go/Progress)

Progress holds the data fields for a progress record.

## [🧰semiorepo⌨️cli💻maingo✂️artifact](semiorepo://definition/semio-repo/cli/main.go/Artifact)

Artifact holds the data fields for a artifact record.

## [🧰semiorepo⌨️cli💻maingo✂️errpayload](semiorepo://definition/semio-repo/cli/main.go/ErrPayload)

ErrPayload holds the data fields for a err payload record.

## [🧰semiorepo⌨️cli💻maingo✂️donepayload](semiorepo://definition/semio-repo/cli/main.go/DonePayload)

DonePayload holds the data fields for a done payload record.

## [🧰semiorepo⌨️cli💻maingo✂️errorcode](semiorepo://definition/semio-repo/cli/main.go/ErrorCode)

ErrorCode represents a error code value.

## [🧰semiorepo⌨️cli💻maingo✂️command](semiorepo://definition/semio-repo/cli/main.go/Command)

Command represents a command value.

## [🧰semiorepo⌨️cli💻maingo✂️request](semiorepo://definition/semio-repo/cli/main.go/Request)

Request holds the data fields for a request record.

## [🧰semiorepo⌨️cli💻maingo✂️graphqlargs](semiorepo://definition/semio-repo/cli/main.go/GraphQLArgs)

GraphQLArgs holds the data fields for a graph q l args record.

## [🧰semiorepo⌨️cli💻maingo✂️graphqlexecutor](semiorepo://definition/semio-repo/cli/main.go/GraphQLExecutor)

GraphQLExecutor defines the interface contract for graph q l executor operations.

## [🧰semiorepo⌨️cli💻maingo✂️engine](semiorepo://definition/semio-repo/cli/main.go/Engine)

Engine holds the data fields for a engine record.

## [🧰semiorepo⌨️cli💻maingo🛠️newengine](semiorepo://definition/semio-repo/cli/main.go/NewEngine)

NewEngine creates and returns a new Engine instance.

## [🧰semiorepo⌨️cli💻maingo🛠️run](semiorepo://definition/semio-repo/cli/main.go/Run)

Run dispatches the request and returns an event channel.

## [🧰semiorepo⌨️cli💻maingo✂️config](semiorepo://definition/semio-repo/cli/main.go/Config)

Config holds the data fields for a config record.

## [🧰semiorepo⌨️cli💻maingo🛠️isjson](semiorepo://definition/semio-repo/cli/main.go/IsJSON)

IsJSON reports whether the Config is j s o n.

## [🧰semiorepo⌨️cli💻maingo🛠️ismarkdown](semiorepo://definition/semio-repo/cli/main.go/IsMarkdown)

IsMarkdown reports whether the Config is markdown.

## [🧰semiorepo⌨️cli💻maingo🛠️istext](semiorepo://definition/semio-repo/cli/main.go/IsText)

IsText reports whether the Config is text.

## [🧰semiorepo⌨️cli💻maingo✂️enginefactory](semiorepo://definition/semio-repo/cli/main.go/EngineFactory)

EngineFactory is a function type for engine factory callbacks.

## [🧰semiorepo⌨️cli💻maingo✂️exiterror](semiorepo://definition/semio-repo/cli/main.go/ExitError)

ExitError holds the data fields for a exit error record.

## [🧰semiorepo⌨️cli💻maingo🛠️error](semiorepo://definition/semio-repo/cli/main.go/Error)

Error returns the string representation of the error.

## [🧰semiorepo⌨️cli💻maingo🛠️newroot](semiorepo://definition/semio-repo/cli/main.go/NewRoot)

NewRoot creates and returns a new Root instance.

## [🧰semiorepo⌨️cli💻maingo🛠️newrootwithconfig](semiorepo://definition/semio-repo/cli/main.go/NewRootWithConfig)

NewRootWithConfig creates and returns a new RootWithConfig instance.

## [🧰semiorepo⌨️cli💻maingo🛠️execute](semiorepo://definition/semio-repo/cli/main.go/Execute)

Execute runs the root command and returns any error.

## [🧰semiorepo⌨️cli💻maingo✂️ticketnode](semiorepo://definition/semio-repo/cli/main.go/TicketNode)

TicketNode holds the data fields for a ticket node record.

## [🧰semiorepo⌨️cli💻maingo✂️goalnode](semiorepo://definition/semio-repo/cli/main.go/GoalNode)

GoalNode holds the data fields for a goal node record.

## [🧰semiorepo⌨️cli💻maingo🪨entitykinds](semiorepo://definition/semio-repo/cli/main.go/EntityKinds)

EntityKinds holds the data fields for a EntityKinds record.

## [🧰semiorepo⌨️cli💻maingo🪨resourcekinds](semiorepo://definition/semio-repo/cli/main.go/ResourceKinds)

ResourceKinds holds the data fields for a ResourceKinds record.

## [🧰semiorepo⌨️cli💻maingo🪨diffablekinds](semiorepo://definition/semio-repo/cli/main.go/DiffableKinds)

DiffableKinds holds the data fields for a DiffableKinds record.

## [🧰semiorepo⌨️cli💻maingo🪨relatedtofilekinds](semiorepo://definition/semio-repo/cli/main.go/RelatedToFileKinds)

RelatedToFileKinds holds the data fields for a RelatedToFileKinds record.

## [🧰semiorepo⌨️cli💻maingo✂️treenodekind](semiorepo://definition/semio-repo/cli/main.go/TreeNodeKind)

TreeNodeKind represents a tree node kind value.

## [🧰semiorepo⌨️cli💻maingo✂️treenode](semiorepo://definition/semio-repo/cli/main.go/TreeNode)

TreeNode holds the data fields for a tree node record.

## [🧰semiorepo⌨️cli💻maingo✂️treefilter](semiorepo://definition/semio-repo/cli/main.go/TreeFilter)

TreeFilter holds the data fields for a tree filter record.

## [🧰semiorepo⌨️cli💻maingo🛠️hasonlykinds](semiorepo://definition/semio-repo/cli/main.go/HasOnlyKinds)

HasOnlyKinds reports whether the TreeFilter has only kinds.

## [🧰semiorepo⌨️cli💻maingo🛠️iskindvisible](semiorepo://definition/semio-repo/cli/main.go/IsKindVisible)

IsKindVisible reports whether the TreeFilter is kind visible.

## [🧰semiorepo⌨️cli💻maingo🛠️matchessubkind](semiorepo://definition/semio-repo/cli/main.go/MatchesSubKind)

MatchesSubKind performs the matches sub kind operation on the TreeFilter.

## [🧰semiorepo⌨️cli💻maingo🛠️matchesdate](semiorepo://definition/semio-repo/cli/main.go/MatchesDate)

MatchesDate performs the matches date operation on the TreeFilter.

## [🧰semiorepo⌨️cli💻maingo🛠️matchesstatus](semiorepo://definition/semio-repo/cli/main.go/MatchesStatus)

MatchesStatus performs the matches status operation on the TreeFilter.

## [🧰semiorepo⌨️cli💻maingo🛠️matchescontributor](semiorepo://definition/semio-repo/cli/main.go/MatchesContributor)

MatchesContributor performs the matches contributor operation on the TreeFilter.

## [🧰semiorepo⌨️cli💻maingo✂️treebuildoptions](semiorepo://definition/semio-repo/cli/main.go/TreeBuildOptions)

TreeBuildOptions holds the data fields for a tree build options record.

## [🧰semiorepo⌨️cli💻maingo🛠️buildmonorepotree](semiorepo://definition/semio-repo/cli/main.go/BuildMonorepoTree)

BuildMonorepoTree constructs and returns the monorepo tree structure.

## [🧰semiorepo⌨️cli💻maingo🛠️propagateparentids](semiorepo://definition/semio-repo/cli/main.go/PropagateParentIDs)

PropagateParentIDs holds the data fields for a PropagateParentIDs record.

## [🧰semiorepo⌨️cli💻maingo🛠️filtermonorepotree](semiorepo://definition/semio-repo/cli/main.go/FilterMonorepoTree)

FilterMonorepoTree filters the monorepo tree based on the given criteria.

## [🧰semiorepo⌨️cli💻maingo🛠️searchmonorepotree](semiorepo://definition/semio-repo/cli/main.go/SearchMonorepoTree)

SearchMonorepoTree performs a text search across the monorepo tree.

## [🧰semiorepo⌨️cli💻maingo🛠️rendermonorepotree](semiorepo://definition/semio-repo/cli/main.go/RenderMonorepoTree)

RenderMonorepoTree renders the monorepo tree into its output representation.

## [🧰semiorepo⌨️cli💻maingo🛠️rendermonorepotreemarkdown](semiorepo://definition/semio-repo/cli/main.go/RenderMonorepoTreeMarkdown)

RenderMonorepoTreeMarkdown renders the monorepo tree markdown into its output representation.

## [🧰semiorepo⌨️cli💻maingo✂️streamrenderer](semiorepo://definition/semio-repo/cli/main.go/StreamRenderer)

StreamRenderer defines the interface contract for stream renderer operations.

## [🧰semiorepo⌨️cli💻maingo✂️ndjsonrenderer](semiorepo://definition/semio-repo/cli/main.go/NDJSONRenderer)

NDJSONRenderer holds the data fields for a n d j s o n renderer record.

## [🧰semiorepo⌨️cli💻maingo🛠️render](semiorepo://definition/semio-repo/cli/main.go/Render)

Render renders the into its output representation.

## [🧰semiorepo⌨️cli💻maingo✂️humanrenderer](semiorepo://definition/semio-repo/cli/main.go/HumanRenderer)

HumanRenderer holds the data fields for a human renderer record.

## [🧰semiorepo⌨️cli💻maingo🛠️render](semiorepo://definition/semio-repo/cli/main.go/Render)

Render renders the into its output representation.

## [🧰semiorepo⌨️cli💻maingo✂️markdownrenderer](semiorepo://definition/semio-repo/cli/main.go/MarkdownRenderer)

MarkdownRenderer holds the data fields for a markdown renderer record.

## [🧰semiorepo⌨️cli💻maingo🛠️render](semiorepo://definition/semio-repo/cli/main.go/Render)

Render renders the into its output representation.

## [🧰semiorepo⌨️cli💻maingo✂️node](semiorepo://definition/semio-repo/cli/main.go/Node)

Node defines the interface contract for node operations.

## [🧰semiorepo⌨️cli💻maingo✂️definitionkind](semiorepo://definition/semio-repo/cli/main.go/DefinitionKind)

DefinitionKind represents a definition kind value.

## [🧰semiorepo⌨️cli💻maingo🛠️isvalid](semiorepo://definition/semio-repo/cli/main.go/IsValid)

IsValid reports whether the DefinitionKind is valid.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String returns the string representation of the DefinitionKind.

## [🧰semiorepo⌨️cli💻maingo🛠️derivedefinitionkind](semiorepo://definition/semio-repo/cli/main.go/DeriveDefinitionKind)

DeriveDefinitionKind infers and returns the definition kind from the given input.

## [🧰semiorepo⌨️cli💻maingo✂️ticketstatus](semiorepo://definition/semio-repo/cli/main.go/TicketStatus)

TicketStatus represents a ticket status value.

## [🧰semiorepo⌨️cli💻maingo🛠️isvalid](semiorepo://definition/semio-repo/cli/main.go/IsValid)

IsValid reports whether the TicketStatus is valid.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String returns the string representation of the TicketStatus.

## [🧰semiorepo⌨️cli💻maingo✂️breachpriority](semiorepo://definition/semio-repo/cli/main.go/BreachPriority)

BreachPriority represents a breach priority value.

## [🧰semiorepo⌨️cli💻maingo🛠️isvalid](semiorepo://definition/semio-repo/cli/main.go/IsValid)

IsValid reports whether the BreachPriority is valid.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String returns the string representation of the BreachPriority.

## [🧰semiorepo⌨️cli💻maingo🪨allowedllms](semiorepo://definition/semio-repo/cli/main.go/AllowedLLMs)

AllowedLLMs holds the allowed l l ms values.

## [🧰semiorepo⌨️cli💻maingo🪨allowedclients](semiorepo://definition/semio-repo/cli/main.go/AllowedClients)

AllowedClients holds the allowed clients values.

## [🧰semiorepo⌨️cli💻maingo🛠️normalizellmslug](semiorepo://definition/semio-repo/cli/main.go/NormalizeLLMSlug)

NormalizeLLMSlug normalizes the l l m slug to its canonical form.

## [🧰semiorepo⌨️cli💻maingo🛠️normalizeclientslug](semiorepo://definition/semio-repo/cli/main.go/NormalizeClientSlug)

NormalizeClientSlug normalizes the client slug to its canonical form.

## [🧰semiorepo⌨️cli💻maingo🛠️resolveallowedllm](semiorepo://definition/semio-repo/cli/main.go/ResolveAllowedLLM)

ResolveAllowedLLM resolves and validates the allowed l l m against known values.

## [🧰semiorepo⌨️cli💻maingo🛠️resolveallowedclient](semiorepo://definition/semio-repo/cli/main.go/ResolveAllowedClient)

ResolveAllowedClient resolves and validates the allowed client against known values.

## [🧰semiorepo⌨️cli💻maingo✂️range](semiorepo://definition/semio-repo/cli/main.go/Range)

Range holds the data fields for a range record.

## [🧰semiorepo⌨️cli💻maingo✂️linemetrics](semiorepo://definition/semio-repo/cli/main.go/LineMetrics)

LineMetrics holds the data fields for a line metrics record.

## [🧰semiorepo⌨️cli💻maingo✂️difflines](semiorepo://definition/semio-repo/cli/main.go/DiffLines)

DiffLines holds the data fields for a diff lines record.

## [🧰semiorepo⌨️cli💻maingo✂️countmetrics](semiorepo://definition/semio-repo/cli/main.go/CountMetrics)

CountMetrics holds the data fields for a count metrics record.

## [🧰semiorepo⌨️cli💻maingo✂️contributoricons](semiorepo://definition/semio-repo/cli/main.go/ContributorIcons)

ContributorIcons holds the data fields for a contributor icons record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorlink](semiorepo://definition/semio-repo/cli/main.go/ContributorLink)

ContributorLink holds the data fields for a contributor link record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketdate](semiorepo://definition/semio-repo/cli/main.go/TicketDate)

TicketDate holds the data fields for a ticket date record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketsectionmetrics](semiorepo://definition/semio-repo/cli/main.go/TicketSectionMetrics)

TicketSectionMetrics holds the data fields for a ticket section metrics record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketfilemetricsentry](semiorepo://definition/semio-repo/cli/main.go/TicketFileMetricsEntry)

TicketFileMetricsEntry holds the data fields for a ticket file metrics entry record.

## [🧰semiorepo⌨️cli💻maingo✂️analyzemetrics](semiorepo://definition/semio-repo/cli/main.go/AnalyzeMetrics)

AnalyzeMetrics holds the data fields for a analyze metrics record.

## [🧰semiorepo⌨️cli💻maingo✂️prioritycount](semiorepo://definition/semio-repo/cli/main.go/PriorityCount)

PriorityCount holds the data fields for a priority count record.

## [🧰semiorepo⌨️cli💻maingo✂️repo](semiorepo://definition/semio-repo/cli/main.go/Repo)

Repo holds the data fields for a repo record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Repo is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Repo.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Repo.

## [🧰semiorepo⌨️cli💻maingo✂️technologykind](semiorepo://definition/semio-repo/cli/main.go/TechnologyKind)

TechnologyKind represents a technology kind value.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String returns the string representation of the TechnologyKind.

## [🧰semiorepo⌨️cli💻maingo✂️bundlekind](semiorepo://definition/semio-repo/cli/main.go/BundleKind)

BundleKind represents a bundle kind value.

## [🧰semiorepo⌨️cli💻maingo🛠️isvalid](semiorepo://definition/semio-repo/cli/main.go/IsValid)

IsValid reports whether the BundleKind is valid.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String returns the string representation of the BundleKind.

## [🧰semiorepo⌨️cli💻maingo🛠️derivetechnologykind](semiorepo://definition/semio-repo/cli/main.go/DeriveTechnologyKind)

DeriveTechnologyKind infers and returns the technology kind from the given input.

## [🧰semiorepo⌨️cli💻maingo🛠️derivebundlekind](semiorepo://definition/semio-repo/cli/main.go/DeriveBundleKind)

DeriveBundleKind infers and returns the bundle kind from the given input.

## [🧰semiorepo⌨️cli💻maingo✂️technology](semiorepo://definition/semio-repo/cli/main.go/Technology)

Technology holds the data fields for a technology record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Technology is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Technology.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Technology.

## [🧰semiorepo⌨️cli💻maingo✂️bundle](semiorepo://definition/semio-repo/cli/main.go/Bundle)

Bundle holds the data fields for a bundle record.

## [🧰semiorepo⌨️cli💻maingo✂️package](semiorepo://definition/semio-repo/cli/main.go/Package)

Package holds the data fields for a package record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Bundle is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Bundle.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Bundle.

## [🧰semiorepo⌨️cli💻maingo✂️folderkind](semiorepo://definition/semio-repo/cli/main.go/FolderKind)

FolderKind represents a folder kind value.

## [🧰semiorepo⌨️cli💻maingo🛠️isvalid](semiorepo://definition/semio-repo/cli/main.go/IsValid)

IsValid reports whether the FolderKind is valid.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String returns the string representation of the FolderKind.

## [🧰semiorepo⌨️cli💻maingo🛠️derivefolderkind](semiorepo://definition/semio-repo/cli/main.go/DeriveFolderKind)

DeriveFolderKind infers and returns the folder kind from the given input.

## [🧰semiorepo⌨️cli💻maingo🛠️isgeneratedfolder](semiorepo://definition/semio-repo/cli/main.go/IsGeneratedFolder)

IsGeneratedFolder reports whether the value is generated folder.

## [🧰semiorepo⌨️cli💻maingo✂️folder](semiorepo://definition/semio-repo/cli/main.go/Folder)

Folder holds the data fields for a folder record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Folder is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Folder.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Folder.

## [🧰semiorepo⌨️cli💻maingo✂️file](semiorepo://definition/semio-repo/cli/main.go/File)

File holds the data fields for a file record.

## [🧰semiorepo⌨️cli💻maingo🛠️derivefilekind](semiorepo://definition/semio-repo/cli/main.go/DeriveFileKind)

DeriveFileKind infers and returns the file kind from the given input.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the File is node.

## [🧰semiorepo⌨️cli💻maingo🛠️isgenerated](semiorepo://definition/semio-repo/cli/main.go/IsGenerated)

IsGenerated reports whether the value is generated.

## [🧰semiorepo⌨️cli💻maingo🛠️issemanticallyignored](semiorepo://definition/semio-repo/cli/main.go/IsSemanticallyIgnored)

IsSemanticallyIgnored reports whether the value is semantically ignored.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the File.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the File.

## [🧰semiorepo⌨️cli💻maingo✂️section](semiorepo://definition/semio-repo/cli/main.go/Section)

Section holds the data fields for a section record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Section is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Section.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Section.

## [🧰semiorepo⌨️cli💻maingo✂️definition](semiorepo://definition/semio-repo/cli/main.go/Definition)

Definition holds the data fields for a definition record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Definition is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Definition.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Definition.

## [🧰semiorepo⌨️cli💻maingo✂️contributor](semiorepo://definition/semio-repo/cli/main.go/Contributor)

Contributor holds the data fields for a contributor record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorcontributionstree](semiorepo://definition/semio-repo/cli/main.go/ContributorContributionsTree)

ContributorContributionsTree holds the data fields for a contributor contributions tree record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorbundle](semiorepo://definition/semio-repo/cli/main.go/ContributorBundle)

ContributorBundle holds the data fields for a contributor bundle record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorfolder](semiorepo://definition/semio-repo/cli/main.go/ContributorFolder)

ContributorFolder holds the data fields for a contributor folder record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorfile](semiorepo://definition/semio-repo/cli/main.go/ContributorFile)

ContributorFile holds the data fields for a contributor file record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorsection](semiorepo://definition/semio-repo/cli/main.go/ContributorSection)

ContributorSection holds the data fields for a contributor section record.

## [🧰semiorepo⌨️cli💻maingo✂️contributordefinition](semiorepo://definition/semio-repo/cli/main.go/ContributorDefinition)

ContributorDefinition holds the data fields for a contributor definition record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Contributor is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Contributor.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Contributor.

## [🧰semiorepo⌨️cli💻maingo✂️commit](semiorepo://definition/semio-repo/cli/main.go/Commit)

Commit holds the data fields for a commit record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Commit is node.

## [🧰semiorepo⌨️cli💻maingo✂️draft](semiorepo://definition/semio-repo/cli/main.go/Draft)

Draft holds the data fields for a draft record.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Draft.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Draft.

## [🧰semiorepo⌨️cli💻maingo🛠️getdraftspath](semiorepo://definition/semio-repo/cli/main.go/GetDraftsPath)

GetDraftsPath returns the drafts path of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️listdrafts](semiorepo://definition/semio-repo/cli/main.go/ListDrafts)

ListDrafts returns all available drafts entries.

## [🧰semiorepo⌨️cli💻maingo🛠️createdraft](semiorepo://definition/semio-repo/cli/main.go/CreateDraft)

CreateDraft creates a new draft and persists it.

## [🧰semiorepo⌨️cli💻maingo🛠️deletedraft](semiorepo://definition/semio-repo/cli/main.go/DeleteDraft)

DeleteDraft removes the specified draft.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Commit.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Commit.

## [🧰semiorepo⌨️cli💻maingo✂️ticket](semiorepo://definition/semio-repo/cli/main.go/Ticket)

Ticket holds the data fields for a ticket record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Ticket is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️gettitle](semiorepo://definition/semio-repo/cli/main.go/GetTitle)

GetTitle returns the title of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️getprompt](semiorepo://definition/semio-repo/cli/main.go/GetPrompt)

GetPrompt returns the prompt of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️getlatestprompt](semiorepo://definition/semio-repo/cli/main.go/GetLatestPrompt)

GetLatestPrompt returns the latest prompt of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️getllm](semiorepo://definition/semio-repo/cli/main.go/GetLLM)

GetLLM returns the l l m of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️getclient](semiorepo://definition/semio-repo/cli/main.go/GetClient)

GetClient returns the client of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️getstatus](semiorepo://definition/semio-repo/cli/main.go/GetStatus)

GetStatus returns the status of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️getauthor](semiorepo://definition/semio-repo/cli/main.go/GetAuthor)

GetAuthor returns the author of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️getcommit](semiorepo://definition/semio-repo/cli/main.go/GetCommit)

GetCommit returns the commit of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️getsummary](semiorepo://definition/semio-repo/cli/main.go/GetSummary)

GetSummary returns the summary of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️getdatestarted](semiorepo://definition/semio-repo/cli/main.go/GetDateStarted)

GetDateStarted returns the date started of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️getdatefinished](semiorepo://definition/semio-repo/cli/main.go/GetDateFinished)

GetDateFinished returns the date finished of the Ticket.

## [🧰semiorepo⌨️cli💻maingo🛠️getinteractionfiles](semiorepo://definition/semio-repo/cli/main.go/GetInteractionFiles)

GetInteractionFiles returns all unique InteractionFile entries across all interactions.

## [🧰semiorepo⌨️cli💻maingo✂️ticketbundlecontrib](semiorepo://definition/semio-repo/cli/main.go/TicketBundleContrib)

TicketBundleContrib holds the data fields for a ticket bundle contrib record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketfilecontrib](semiorepo://definition/semio-repo/cli/main.go/TicketFileContrib)

TicketFileContrib holds the data fields for a ticket file contrib record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketsectioncontrib](semiorepo://definition/semio-repo/cli/main.go/TicketSectionContrib)

TicketSectionContrib holds the data fields for a ticket section contrib record.

## [🧰semiorepo⌨️cli💻maingo✂️policy](semiorepo://definition/semio-repo/cli/main.go/Policy)

Policy holds the data fields for a policy record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Policy is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Policy.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Policy.

## [🧰semiorepo⌨️cli💻maingo✂️statutemeta](semiorepo://definition/semio-repo/cli/main.go/StatuteMeta)

StatuteMeta holds the data fields for a statute meta record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the StatuteMeta is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the StatuteMeta.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the StatuteMeta.

## [🧰semiorepo⌨️cli💻maingo✂️analyzeresult](semiorepo://definition/semio-repo/cli/main.go/AnalyzeResult)

AnalyzeResult holds the data fields for a analyze result record.

## [🧰semiorepo⌨️cli💻maingo✂️fixresult](semiorepo://definition/semio-repo/cli/main.go/FixResult)

FixResult holds the data fields for a fix result record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorcontributions](semiorepo://definition/semio-repo/cli/main.go/ContributorContributions)

ContributorContributions holds the data fields for a contributor contributions record.

## [🧰semiorepo⌨️cli💻maingo✂️contributionbundle](semiorepo://definition/semio-repo/cli/main.go/ContributionBundle)

ContributionBundle holds the data fields for a contribution bundle record.

## [🧰semiorepo⌨️cli💻maingo✂️contributionfolder](semiorepo://definition/semio-repo/cli/main.go/ContributionFolder)

ContributionFolder holds the data fields for a contribution folder record.

## [🧰semiorepo⌨️cli💻maingo✂️contributionfile](semiorepo://definition/semio-repo/cli/main.go/ContributionFile)

ContributionFile holds the data fields for a contribution file record.

## [🧰semiorepo⌨️cli💻maingo✂️contributionsection](semiorepo://definition/semio-repo/cli/main.go/ContributionSection)

ContributionSection holds the data fields for a contribution section record.

## [🧰semiorepo⌨️cli💻maingo✂️contributiondefinition](semiorepo://definition/semio-repo/cli/main.go/ContributionDefinition)

ContributionDefinition holds the data fields for a contribution definition record.

## [🧰semiorepo⌨️cli💻maingo✂️semanticchangetype](semiorepo://definition/semio-repo/cli/main.go/SemanticChangeType)

SemanticChangeType represents a semantic change type value.

## [🧰semiorepo⌨️cli💻maingo✂️semanticchange](semiorepo://definition/semio-repo/cli/main.go/SemanticChange)

SemanticChange holds the data fields for a semantic change record.

## [🧰semiorepo⌨️cli💻maingo🛠️buildsemanticdiffs](semiorepo://definition/semio-repo/cli/main.go/BuildSemanticDiffs)

BuildSemanticDiffs constructs and returns the semantic diffs structure.

## [🧰semiorepo⌨️cli💻maingo✂️filelistinput](semiorepo://definition/semio-repo/cli/main.go/FileListInput)

FileListInput holds the data fields for a file list input record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketopeninput](semiorepo://definition/semio-repo/cli/main.go/TicketOpenInput)

TicketOpenInput holds the data fields for a ticket open input record.

## [🧰semiorepo⌨️cli💻maingo✂️draftcreateinput](semiorepo://definition/semio-repo/cli/main.go/DraftCreateInput)

DraftCreateInput holds the data fields for a draft create input record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketprogressinput](semiorepo://definition/semio-repo/cli/main.go/TicketProgressInput)

TicketProgressInput holds the data fields for a ticket progress input record.

## [🧰semiorepo⌨️cli💻maingo✂️goalcreateinput](semiorepo://definition/semio-repo/cli/main.go/GoalCreateInput)

GoalCreateInput holds the data fields for a goal create input record.

## [🧰semiorepo⌨️cli💻maingo✂️goalchangeinput](semiorepo://definition/semio-repo/cli/main.go/GoalChangeInput)

GoalChangeInput holds the data fields for a goal change input record.

## [🧰semiorepo⌨️cli💻maingo✂️goalcloseinput](semiorepo://definition/semio-repo/cli/main.go/GoalCloseInput)

GoalCloseInput holds the data fields for a goal close input record.

## [🧰semiorepo⌨️cli💻maingo✂️goalreopeninput](semiorepo://definition/semio-repo/cli/main.go/GoalReopenInput)

GoalReopenInput holds the data fields for a goal reopen input record.

## [🧰semiorepo⌨️cli💻maingo✂️goaldeleteinput](semiorepo://definition/semio-repo/cli/main.go/GoalDeleteInput)

GoalDeleteInput holds the data fields for a goal delete input record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketdeleteinput](semiorepo://definition/semio-repo/cli/main.go/TicketDeleteInput)

TicketDeleteInput holds the data fields for a ticket delete input record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketcloseinput](semiorepo://definition/semio-repo/cli/main.go/TicketCloseInput)

TicketCloseInput holds the data fields for a ticket close input record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketreopeninput](semiorepo://definition/semio-repo/cli/main.go/TicketReopenInput)

TicketReopenInput holds the data fields for a ticket reopen input record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketchangeinput](semiorepo://definition/semio-repo/cli/main.go/TicketChangeInput)

TicketChangeInput holds the data fields for a ticket change input record.

## [🧰semiorepo⌨️cli💻maingo✂️contributoraddinput](semiorepo://definition/semio-repo/cli/main.go/ContributorAddInput)

ContributorAddInput holds the data fields for a contributor add input record.

## [🧰semiorepo⌨️cli💻maingo✂️filterinput](semiorepo://definition/semio-repo/cli/main.go/FilterInput)

FilterInput holds the data fields for a filter input record.

## [🧰semiorepo⌨️cli💻maingo🛠️tostreamoptions](semiorepo://definition/semio-repo/cli/main.go/ToStreamOptions)

ToStreamOptions converts the filter input into stream options.

## [🧰semiorepo⌨️cli💻maingo✂️sourcecontrolprovider](semiorepo://definition/semio-repo/cli/main.go/SourceControlProvider)

SourceControlProvider defines the interface for source control operations (GitHub, GitLab, BitBucket, ...).

## [🧰semiorepo⌨️cli💻maingo✂️managementissue](semiorepo://definition/semio-repo/cli/main.go/ManagementIssue)

ManagementIssue holds the data fields for a management issue record.

## [🧰semiorepo⌨️cli💻maingo✂️managementmilestone](semiorepo://definition/semio-repo/cli/main.go/ManagementMilestone)

ManagementMilestone holds the data fields for a management milestone record.

## [🧰semiorepo⌨️cli💻maingo✂️managementlabel](semiorepo://definition/semio-repo/cli/main.go/ManagementLabel)

ManagementLabel holds the data fields for a management label record.

## [🧰semiorepo⌨️cli💻maingo✂️managementprovider](semiorepo://definition/semio-repo/cli/main.go/ManagementProvider)

ManagementProvider defines the interface for issue/milestone management operations (GitHub, Jira, Trello, Linear, ...).

## [🧰semiorepo⌨️cli💻maingo✂️sandboxprovider](semiorepo://definition/semio-repo/cli/main.go/SandboxProvider)

SandboxProvider defines the interface for sandbox/container operations (Devcontainer, Podman, ...).

## [🧰semiorepo⌨️cli💻maingo✂️editorhookmapping](semiorepo://definition/semio-repo/cli/main.go/EditorHookMapping)

EditorHookMapping holds the data fields for an editor hook mapping record.

## [🧰semiorepo⌨️cli💻maingo✂️editorprovider](semiorepo://definition/semio-repo/cli/main.go/EditorProvider)

EditorProvider defines the interface for editor/agent operations (VSCode/Copilot, Cursor, Windsurf, Claude Code, Codex, Droid, Antigravity, ...).

## [🧰semiorepo⌨️cli💻maingo✂️githubmanagementprovider](semiorepo://definition/semio-repo/cli/main.go/GitHubManagementProvider)

GitHubManagementProvider holds the data fields for a github management provider record.

## [🧰semiorepo⌨️cli💻maingo✂️nullmanagementprovider](semiorepo://definition/semio-repo/cli/main.go/NullManagementProvider)

NullManagementProvider is a no-op implementation of ManagementProvider.

## [🧰semiorepo⌨️cli💻maingo✂️githubsourcecontrolprovider](semiorepo://definition/semio-repo/cli/main.go/GitHubSourceControlProvider)

GitHubSourceControlProvider holds the data fields for a github source control provider record.

## [🧰semiorepo⌨️cli💻maingo✂️devcontainersandboxprovider](semiorepo://definition/semio-repo/cli/main.go/DevcontainerSandboxProvider)

DevcontainerSandboxProvider holds the data fields for a devcontainer sandbox provider record.

## [🧰semiorepo⌨️cli💻maingo✂️copiloteditorprovider](semiorepo://definition/semio-repo/cli/main.go/CopilotEditorProvider)

CopilotEditorProvider holds the data fields for a copilot editor provider record.

## [🧰semiorepo⌨️cli💻maingo✂️cursoreditorprovider](semiorepo://definition/semio-repo/cli/main.go/CursorEditorProvider)

CursorEditorProvider holds the data fields for a cursor editor provider record.

## [🧰semiorepo⌨️cli💻maingo✂️windsurfeditorprovider](semiorepo://definition/semio-repo/cli/main.go/WindsurfEditorProvider)

WindsurfEditorProvider holds the data fields for a windsurf editor provider record.

## [🧰semiorepo⌨️cli💻maingo✂️claudecodeeditorprovider](semiorepo://definition/semio-repo/cli/main.go/ClaudeCodeEditorProvider)

ClaudeCodeEditorProvider holds the data fields for a claude code editor provider record.

## [🧰semiorepo⌨️cli💻maingo✂️droideditorprovider](semiorepo://definition/semio-repo/cli/main.go/DroidEditorProvider)

DroidEditorProvider holds the data fields for a droid editor provider record.

## [🧰semiorepo⌨️cli💻maingo✂️codexeditorprovider](semiorepo://definition/semio-repo/cli/main.go/CodexEditorProvider)

CodexEditorProvider holds the data fields for a codex editor provider record.

## [🧰semiorepo⌨️cli💻maingo✂️antigravityeditorprovider](semiorepo://definition/semio-repo/cli/main.go/AntigravityEditorProvider)

AntigravityEditorProvider holds the data fields for an antigravity editor provider record.

## [🧰semiorepo⌨️cli💻maingo🛠️alleditorproviders](semiorepo://definition/semio-repo/cli/main.go/AllEditorProviders)

AllEditorProviders returns all registered editor providers.

## [🧰semiorepo⌨️cli💻maingo🛠️geteditorprovider](semiorepo://definition/semio-repo/cli/main.go/GetEditorProvider)

GetEditorProvider returns the editor provider for the given client slug.

## [🧰semiorepo⌨️cli💻maingo🛠️defaultmanagementprovider](semiorepo://definition/semio-repo/cli/main.go/DefaultManagementProvider)

DefaultManagementProvider returns the default management provider (GitHub).

## [🧰semiorepo⌨️cli💻maingo🛠️defaultsourcecontrolprovider](semiorepo://definition/semio-repo/cli/main.go/DefaultSourceControlProvider)

DefaultSourceControlProvider returns the default source control provider (GitHub).

## [🧰semiorepo⌨️cli💻maingo🛠️defaultsandboxprovider](semiorepo://definition/semio-repo/cli/main.go/DefaultSandboxProvider)

DefaultSandboxProvider returns the default sandbox provider (Devcontainer).

## [🧰semiorepo⌨️cli💻maingo✂️scopekind](semiorepo://definition/semio-repo/cli/main.go/ScopeKind)

ScopeKind represents a scope kind value.

## [🧰semiorepo⌨️cli💻maingo✂️scope](semiorepo://definition/semio-repo/cli/main.go/Scope)

Scope holds the data fields for a scope record.

## [🧰semiorepo⌨️cli💻maingo✂️todocreateinput](semiorepo://definition/semio-repo/cli/main.go/TodoCreateInput)

TodoCreateInput holds the data fields for a todo create input record.

## [🧰semiorepo⌨️cli💻maingo✂️todochangeinput](semiorepo://definition/semio-repo/cli/main.go/TodoChangeInput)

TodoChangeInput holds the data fields for a todo change input record.

## [🧰semiorepo⌨️cli💻maingo✂️todo](semiorepo://definition/semio-repo/cli/main.go/Todo)

Todo holds the data fields for a todo record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Todo is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Todo.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Todo.

## [🧰semiorepo⌨️cli💻maingo✂️location](semiorepo://definition/semio-repo/cli/main.go/Location)

Location holds the data fields for a location record.

## [🧰semiorepo⌨️cli💻maingo✂️breach](semiorepo://definition/semio-repo/cli/main.go/Breach)

Breach holds the data fields for a breach record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Breach is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Breach.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Breach.

## [🧰semiorepo⌨️cli💻maingo🛠️priority](semiorepo://definition/semio-repo/cli/main.go/Priority)

Priority returns the priority of the breach from its kind metadata.

## [🧰semiorepo⌨️cli💻maingo🛠️autofixable](semiorepo://definition/semio-repo/cli/main.go/Autofixable)

Autofixable reports whether the statute supports automatic fixing.

## [🧰semiorepo⌨️cli💻maingo✂️ticketfilemetrics](semiorepo://definition/semio-repo/cli/main.go/TicketFileMetrics)

TicketFileMetrics holds the data fields for a ticket file metrics record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketbundlemetrics](semiorepo://definition/semio-repo/cli/main.go/TicketBundleMetrics)

TicketBundleMetrics holds the data fields for a ticket bundle metrics record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketbundles](semiorepo://definition/semio-repo/cli/main.go/TicketBundles)

TicketBundles represents a ticket bundles value.

## [🧰semiorepo⌨️cli💻maingo✂️languageplugin](semiorepo://definition/semio-repo/cli/main.go/LanguagePlugin)

LanguagePlugin defines the interface contract for language plugin operations.

## [🧰semiorepo⌨️cli💻maingo✂️definitionrange](semiorepo://definition/semio-repo/cli/main.go/DefinitionRange)

DefinitionRange holds the data fields for a definition range record.

## [🧰semiorepo⌨️cli💻maingo✂️baselanguage](semiorepo://definition/semio-repo/cli/main.go/BaseLanguage)

BaseLanguage holds the data fields for a base language record.

## [🧰semiorepo⌨️cli💻maingo🛠️name](semiorepo://definition/semio-repo/cli/main.go/Name)

Name performs the name operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️extensions](semiorepo://definition/semio-repo/cli/main.go/Extensions)

Extensions performs the extensions operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️commentprefix](semiorepo://definition/semio-repo/cli/main.go/CommentPrefix)

CommentPrefix performs the comment prefix operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️blockcommentstart](semiorepo://definition/semio-repo/cli/main.go/BlockCommentStart)

BlockCommentStart performs the block comment start operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️blockcommentend](semiorepo://definition/semio-repo/cli/main.go/BlockCommentEnd)

BlockCommentEnd performs the block comment end operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️usesindentscoping](semiorepo://definition/semio-repo/cli/main.go/UsesIndentScoping)

UsesIndentScoping performs the uses indent scoping operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportssections](semiorepo://definition/semio-repo/cli/main.go/SupportsSections)

SupportsSections performs the supports sections operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsdefinitions](semiorepo://definition/semio-repo/cli/main.go/SupportsDefinitions)

SupportsDefinitions performs the supports definitions operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportscomments](semiorepo://definition/semio-repo/cli/main.go/SupportsComments)

SupportsComments performs the supports comments operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsheaders](semiorepo://definition/semio-repo/cli/main.go/SupportsHeaders)

SupportsHeaders performs the supports headers operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️matchesextension](semiorepo://definition/semio-repo/cli/main.go/MatchesExtension)

MatchesExtension performs the matches extension operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️formatsectionstart](semiorepo://definition/semio-repo/cli/main.go/FormatSectionStart)

FormatSectionStart formats the section start into its string representation.

## [🧰semiorepo⌨️cli💻maingo🛠️formatsectionend](semiorepo://definition/semio-repo/cli/main.go/FormatSectionEnd)

FormatSectionEnd formats the section end into its string representation.

## [🧰semiorepo⌨️cli💻maingo🛠️formatsectionboth](semiorepo://definition/semio-repo/cli/main.go/FormatSectionBoth)

FormatSectionBoth formats the section both into its string representation.

## [🧰semiorepo⌨️cli💻maingo🛠️formatheader](semiorepo://definition/semio-repo/cli/main.go/FormatHeader)

FormatHeader formats the header into its string representation.

## [🧰semiorepo⌨️cli💻maingo🛠️policysectionstartmatch](semiorepo://definition/semio-repo/cli/main.go/PolicySectionStartMatch)

PolicySectionStartMatch performs the policy section start match operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️policysectionendmatch](semiorepo://definition/semio-repo/cli/main.go/PolicySectionEndMatch)

PolicySectionEndMatch performs the policy section end match operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️parsesections](semiorepo://definition/semio-repo/cli/main.go/ParseSections)

ParseSections parses the input and returns the sections result.

## [🧰semiorepo⌨️cli💻maingo🛠️parsedefinitions](semiorepo://definition/semio-repo/cli/main.go/ParseDefinitions)

ParseDefinitions parses the input and returns the definitions result.

## [🧰semiorepo⌨️cli💻maingo🛠️extraorphandefinitions](semiorepo://definition/semio-repo/cli/main.go/ExtraOrphanDefinitions)

ExtraOrphanDefinitions performs the extra orphan definitions operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️skipdirectives](semiorepo://definition/semio-repo/cli/main.go/SkipDirectives)

SkipDirectives performs the skip directives operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️scancomments](semiorepo://definition/semio-repo/cli/main.go/ScanComments)

ScanComments performs the scan comments operation on the BaseLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️extractimports](semiorepo://definition/semio-repo/cli/main.go/ExtractImports)

ExtractImports extracts the imports from the given input.

## [🧰semiorepo⌨️cli💻maingo🛠️formatimports](semiorepo://definition/semio-repo/cli/main.go/FormatImports)

FormatImports formats the imports into its string representation.

## [🧰semiorepo⌨️cli💻maingo🛠️extractpackage](semiorepo://definition/semio-repo/cli/main.go/ExtractPackage)

ExtractPackage extracts the package from the given input.

## [🧰semiorepo⌨️cli💻maingo✂️typescriptlanguage](semiorepo://definition/semio-repo/cli/main.go/TypeScriptLanguage)

TypeScriptLanguage holds the data fields for a type script language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newtypescriptlanguage](semiorepo://definition/semio-repo/cli/main.go/NewTypeScriptLanguage)

NewTypeScriptLanguage creates and returns a new TypeScriptLanguage instance.

## [🧰semiorepo⌨️cli💻maingo🛠️scancomments](semiorepo://definition/semio-repo/cli/main.go/ScanComments)

ScanComments performs the scan comments operation on the TypeScriptLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️extractimports](semiorepo://definition/semio-repo/cli/main.go/ExtractImports)

ExtractImports extracts the imports from the given input.

## [🧰semiorepo⌨️cli💻maingo🛠️formatimports](semiorepo://definition/semio-repo/cli/main.go/FormatImports)

FormatImports formats the imports into its string representation.

## [🧰semiorepo⌨️cli💻maingo✂️golanguage](semiorepo://definition/semio-repo/cli/main.go/GoLanguage)

GoLanguage holds the data fields for a go language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newgolanguage](semiorepo://definition/semio-repo/cli/main.go/NewGoLanguage)

NewGoLanguage creates and returns a new GoLanguage instance.

## [🧰semiorepo⌨️cli💻maingo🛠️extraorphandefinitions](semiorepo://definition/semio-repo/cli/main.go/ExtraOrphanDefinitions)

ExtraOrphanDefinitions performs the extra orphan definitions operation on the GoLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️extractimports](semiorepo://definition/semio-repo/cli/main.go/ExtractImports)

ExtractImports extracts the imports from the given input.

## [🧰semiorepo⌨️cli💻maingo🛠️formatimports](semiorepo://definition/semio-repo/cli/main.go/FormatImports)

FormatImports formats the imports into its string representation.

## [🧰semiorepo⌨️cli💻maingo🛠️extractpackage](semiorepo://definition/semio-repo/cli/main.go/ExtractPackage)

ExtractPackage extracts the package from the given input.

## [🧰semiorepo⌨️cli💻maingo✂️pythonlanguage](semiorepo://definition/semio-repo/cli/main.go/PythonLanguage)

PythonLanguage holds the data fields for a python language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newpythonlanguage](semiorepo://definition/semio-repo/cli/main.go/NewPythonLanguage)

NewPythonLanguage creates and returns a new PythonLanguage instance.

## [🧰semiorepo⌨️cli💻maingo🛠️extractimports](semiorepo://definition/semio-repo/cli/main.go/ExtractImports)

ExtractImports extracts the imports from the given input.

## [🧰semiorepo⌨️cli💻maingo🛠️formatimports](semiorepo://definition/semio-repo/cli/main.go/FormatImports)

FormatImports formats the imports into its string representation.

## [🧰semiorepo⌨️cli💻maingo✂️csharplanguage](semiorepo://definition/semio-repo/cli/main.go/CSharpLanguage)

CSharpLanguage holds the data fields for a c sharp language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newcsharplanguage](semiorepo://definition/semio-repo/cli/main.go/NewCSharpLanguage)

NewCSharpLanguage creates and returns a new CSharpLanguage instance.

## [🧰semiorepo⌨️cli💻maingo🛠️extractimports](semiorepo://definition/semio-repo/cli/main.go/ExtractImports)

ExtractImports extracts the imports from the given input.

## [🧰semiorepo⌨️cli💻maingo🛠️formatimports](semiorepo://definition/semio-repo/cli/main.go/FormatImports)

FormatImports formats the imports into its string representation.

## [🧰semiorepo⌨️cli💻maingo✂️jsonlanguage](semiorepo://definition/semio-repo/cli/main.go/JSONLanguage)

JSONLanguage holds the data fields for a j s o n language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newjsonlanguage](semiorepo://definition/semio-repo/cli/main.go/NewJSONLanguage)

NewJSONLanguage creates and returns a new JSONLanguage instance.

## [🧰semiorepo⌨️cli💻maingo🛠️supportssections](semiorepo://definition/semio-repo/cli/main.go/SupportsSections)

SupportsSections performs the supports sections operation on the JSONLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsdefinitions](semiorepo://definition/semio-repo/cli/main.go/SupportsDefinitions)

SupportsDefinitions performs the supports definitions operation on the JSONLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportscomments](semiorepo://definition/semio-repo/cli/main.go/SupportsComments)

SupportsComments performs the supports comments operation on the JSONLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsheaders](semiorepo://definition/semio-repo/cli/main.go/SupportsHeaders)

SupportsHeaders performs the supports headers operation on the JSONLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️parsesections](semiorepo://definition/semio-repo/cli/main.go/ParseSections)

ParseSections parses the input and returns the sections result.

## [🧰semiorepo⌨️cli💻maingo✂️markdownlanguage](semiorepo://definition/semio-repo/cli/main.go/MarkdownLanguage)

MarkdownLanguage holds the data fields for a markdown language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newmarkdownlanguage](semiorepo://definition/semio-repo/cli/main.go/NewMarkdownLanguage)

NewMarkdownLanguage creates and returns a new MarkdownLanguage instance.

## [🧰semiorepo⌨️cli💻maingo🛠️supportssections](semiorepo://definition/semio-repo/cli/main.go/SupportsSections)

SupportsSections performs the supports sections operation on the MarkdownLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsdefinitions](semiorepo://definition/semio-repo/cli/main.go/SupportsDefinitions)

SupportsDefinitions performs the supports definitions operation on the MarkdownLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportscomments](semiorepo://definition/semio-repo/cli/main.go/SupportsComments)

SupportsComments performs the supports comments operation on the MarkdownLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️parsesections](semiorepo://definition/semio-repo/cli/main.go/ParseSections)

ParseSections parses the input and returns the sections result.

## [🧰semiorepo⌨️cli💻maingo✂️rustlanguage](semiorepo://definition/semio-repo/cli/main.go/RustLanguage)

RustLanguage holds the data fields for a rust language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newrustlanguage](semiorepo://definition/semio-repo/cli/main.go/NewRustLanguage)

NewRustLanguage creates and returns a new RustLanguage instance.

## [🧰semiorepo⌨️cli💻maingo🛠️extraorphandefinitions](semiorepo://definition/semio-repo/cli/main.go/ExtraOrphanDefinitions)

ExtraOrphanDefinitions performs the extra orphan definitions operation on the RustLanguage.

## [🧰semiorepo⌨️cli💻maingo✂️rubylanguage](semiorepo://definition/semio-repo/cli/main.go/RubyLanguage)

RubyLanguage holds the data fields for a ruby language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newrubylanguage](semiorepo://definition/semio-repo/cli/main.go/NewRubyLanguage)

NewRubyLanguage creates and returns a new RubyLanguage instance.

## [🧰semiorepo⌨️cli💻maingo🛠️parsedefinitions](semiorepo://definition/semio-repo/cli/main.go/ParseDefinitions)

ParseDefinitions parses the input and returns the definitions result.

## [🧰semiorepo⌨️cli💻maingo🛠️extraorphandefinitions](semiorepo://definition/semio-repo/cli/main.go/ExtraOrphanDefinitions)

ExtraOrphanDefinitions performs the extra orphan definitions operation on the RubyLanguage.

## [🧰semiorepo⌨️cli💻maingo✂️shelllanguage](semiorepo://definition/semio-repo/cli/main.go/ShellLanguage)

ShellLanguage holds the data fields for a shell language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newshelllanguage](semiorepo://definition/semio-repo/cli/main.go/NewShellLanguage)

NewShellLanguage creates and returns a new ShellLanguage instance.

## [🧰semiorepo⌨️cli💻maingo✂️tomllanguage](semiorepo://definition/semio-repo/cli/main.go/TomlLanguage)

TomlLanguage holds the data fields for a toml language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newtomllanguage](semiorepo://definition/semio-repo/cli/main.go/NewTomlLanguage)

NewTomlLanguage creates and returns a new TomlLanguage instance.

## [🧰semiorepo⌨️cli💻maingo🛠️supportssections](semiorepo://definition/semio-repo/cli/main.go/SupportsSections)

SupportsSections performs the supports sections operation on the TomlLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsdefinitions](semiorepo://definition/semio-repo/cli/main.go/SupportsDefinitions)

SupportsDefinitions performs the supports definitions operation on the TomlLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportscomments](semiorepo://definition/semio-repo/cli/main.go/SupportsComments)

SupportsComments performs the supports comments operation on the TomlLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsheaders](semiorepo://definition/semio-repo/cli/main.go/SupportsHeaders)

SupportsHeaders performs the supports headers operation on the TomlLanguage.

## [🧰semiorepo⌨️cli💻maingo✂️yamllanguage](semiorepo://definition/semio-repo/cli/main.go/YamlLanguage)

YamlLanguage holds the data fields for a yaml language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newyamllanguage](semiorepo://definition/semio-repo/cli/main.go/NewYamlLanguage)

NewYamlLanguage creates and returns a new YamlLanguage instance.

## [🧰semiorepo⌨️cli💻maingo🛠️supportssections](semiorepo://definition/semio-repo/cli/main.go/SupportsSections)

SupportsSections performs the supports sections operation on the YamlLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsdefinitions](semiorepo://definition/semio-repo/cli/main.go/SupportsDefinitions)

SupportsDefinitions performs the supports definitions operation on the YamlLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportscomments](semiorepo://definition/semio-repo/cli/main.go/SupportsComments)

SupportsComments performs the supports comments operation on the YamlLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsheaders](semiorepo://definition/semio-repo/cli/main.go/SupportsHeaders)

SupportsHeaders performs the supports headers operation on the YamlLanguage.

## [🧰semiorepo⌨️cli💻maingo✂️sqllanguage](semiorepo://definition/semio-repo/cli/main.go/SqlLanguage)

SqlLanguage holds the data fields for a sql language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newsqllanguage](semiorepo://definition/semio-repo/cli/main.go/NewSqlLanguage)

NewSqlLanguage creates and returns a new SqlLanguage instance.

## [🧰semiorepo⌨️cli💻maingo✂️graphqllanguage](semiorepo://definition/semio-repo/cli/main.go/GraphqlLanguage)

GraphqlLanguage holds the data fields for a graphql language record.

## [🧰semiorepo⌨️cli💻maingo🛠️newgraphqllanguage](semiorepo://definition/semio-repo/cli/main.go/NewGraphqlLanguage)

NewGraphqlLanguage creates and returns a new GraphqlLanguage instance.

## [🧰semiorepo⌨️cli💻maingo🛠️getlanguage](semiorepo://definition/semio-repo/cli/main.go/GetLanguage)

GetLanguage returns the language of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️getlanguagebyname](semiorepo://definition/semio-repo/cli/main.go/GetLanguageByName)

GetLanguageByName returns the language by name of the value.

## [🧰semiorepo⌨️cli💻maingo✂️gitauthor](semiorepo://definition/semio-repo/cli/main.go/GitAuthor)

GitAuthor holds the data fields for a git author record.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String returns the string representation of the GitAuthor.

## [🧰semiorepo⌨️cli💻maingo🛠️findandupdatecontributor](semiorepo://definition/semio-repo/cli/main.go/FindAndUpdateContributor)

FindAndUpdateContributor searches for and returns the matching and update contributor.

## [🧰semiorepo⌨️cli💻maingo🛠️getsystem](semiorepo://definition/semio-repo/cli/main.go/GetSystem)

GetSystem returns the system of the value.

## [🧰semiorepo⌨️cli💻maingo✂️interactionfile](semiorepo://definition/semio-repo/cli/main.go/InteractionFile)

InteractionFile holds a file reference with path, id and uri.

## [🧰semiorepo⌨️cli💻maingo✂️interaction](semiorepo://definition/semio-repo/cli/main.go/Interaction)

Interaction holds the data fields for a interaction record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketsessionrename](semiorepo://definition/semio-repo/cli/main.go/TicketSessionRename)

TicketSessionRename holds the data fields for a ticket session rename record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketsessiondiffstats](semiorepo://definition/semio-repo/cli/main.go/TicketSessionDiffStats)

TicketSessionDiffStats holds the data fields for a ticket session diff stats record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketsessiondiff](semiorepo://definition/semio-repo/cli/main.go/TicketSessionDiff)

TicketSessionDiff holds the data fields for a ticket session diff record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketsessionreads](semiorepo://definition/semio-repo/cli/main.go/TicketSessionReads)

TicketSessionReads holds the data fields for a ticket session reads record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketsessioninteraction](semiorepo://definition/semio-repo/cli/main.go/TicketSessionInteraction)

TicketSessionInteraction holds the data fields for a ticket session interaction record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketsession](semiorepo://definition/semio-repo/cli/main.go/TicketSession)

TicketSession holds the data fields for a ticket session record.

## [🧰semiorepo⌨️cli💻maingo🛠️unmarshaljson](semiorepo://definition/semio-repo/cli/main.go/UnmarshalJSON)

UnmarshalJSON performs the unmarshal j s o n operation on the Interaction.

## [🧰semiorepo⌨️cli💻maingo✂️interactionresource](semiorepo://definition/semio-repo/cli/main.go/InteractionResource)

InteractionResource holds a flat interaction enriched with its source context.

## [🧰semiorepo⌨️cli💻maingo🛠️listinteractions](semiorepo://definition/semio-repo/cli/main.go/ListInteractions)

ListInteractions aggregates interactions from all tickets and goals.

## [🧰semiorepo⌨️cli💻maingo🛠️streaminteractions](semiorepo://definition/semio-repo/cli/main.go/StreamInteractions)

StreamInteractions streams interactions from all tickets and goals.

## [🧰semiorepo⌨️cli💻maingo✂️ticketsection](semiorepo://definition/semio-repo/cli/main.go/TicketSection)

TicketSection holds the data fields for a ticket section record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketfile](semiorepo://definition/semio-repo/cli/main.go/TicketFile)

TicketFile holds the data fields for a ticket file record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketmanagementdata](semiorepo://definition/semio-repo/cli/main.go/TicketManagementData)

TicketManagementData holds the data fields for a ticket github data record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketfilerenamed](semiorepo://definition/semio-repo/cli/main.go/TicketFileRenamed)

TicketFileRenamed holds the data fields for a ticket file renamed record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketdiffset](semiorepo://definition/semio-repo/cli/main.go/TicketDiffSet)

TicketDiffSet holds the data fields for a ticket diff set record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketdiffs](semiorepo://definition/semio-repo/cli/main.go/TicketDiffs)

TicketDiffs holds the data fields for a ticket diffs record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketdata](semiorepo://definition/semio-repo/cli/main.go/TicketData)

TicketData holds the data fields for a ticket data record.

## [🧰semiorepo⌨️cli💻maingo✂️goal](semiorepo://definition/semio-repo/cli/main.go/Goal)

Goal holds the data fields for a goal record.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode reports whether the Goal is node.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Goal.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Goal.

## [🧰semiorepo⌨️cli💻maingo✂️goaldates](semiorepo://definition/semio-repo/cli/main.go/GoalDates)

GoalDates holds the data fields for a goal dates record.

## [🧰semiorepo⌨️cli💻maingo✂️goalmanagementdata](semiorepo://definition/semio-repo/cli/main.go/GoalManagementData)

GoalManagementData holds the data fields for a goal github data record.

## [🧰semiorepo⌨️cli💻maingo✂️statute](semiorepo://definition/semio-repo/cli/main.go/Statute)

Statute represents a statute value.

## [🧰semiorepo⌨️cli💻maingo🛠️info](semiorepo://definition/semio-repo/cli/main.go/Info)

Info returns the metadata for the statute.

## [🧰semiorepo⌨️cli💻maingo✂️territory](semiorepo://definition/semio-repo/cli/main.go/Territory)

Territory holds the data fields for a statute group record.

## [🧰semiorepo⌨️cli💻maingo🛠️allkinds](semiorepo://definition/semio-repo/cli/main.go/AllKinds)

AllKinds returns all statutes associated with the group.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID returns the i d of the Territory.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI returns the u r i of the Territory.

## [🧰semiorepo⌨️cli💻maingo✂️policydef](semiorepo://definition/semio-repo/cli/main.go/PolicyDef)

PolicyDef holds the data fields for a policy def record.

## [🧰semiorepo⌨️cli💻maingo🛠️allkinds](semiorepo://definition/semio-repo/cli/main.go/AllKinds)

AllKinds returns all statutes associated with the group.

## [🧰semiorepo⌨️cli💻maingo✂️analyzereport](semiorepo://definition/semio-repo/cli/main.go/AnalyzeReport)

AnalyzeReport holds the data fields for a analyze report record.

## [🧰semiorepo⌨️cli💻maingo✂️summary](semiorepo://definition/semio-repo/cli/main.go/Summary)

Summary holds the data fields for a summary record.

## [🧰semiorepo⌨️cli💻maingo✂️filecache](semiorepo://definition/semio-repo/cli/main.go/FileCache)

FileCache holds the data fields for a file cache record.

## [🧰semiorepo⌨️cli💻maingo✂️outputtype](semiorepo://definition/semio-repo/cli/main.go/OutputType)

OutputType represents a output type value.

## [🧰semiorepo⌨️cli💻maingo✂️outputline](semiorepo://definition/semio-repo/cli/main.go/OutputLine)

OutputLine holds the data fields for a output line record.

## [🧰semiorepo⌨️cli💻maingo✂️commandoutput](semiorepo://definition/semio-repo/cli/main.go/CommandOutput)

CommandOutput holds the data fields for a command output record.

## [🧰semiorepo⌨️cli💻maingo✂️toolresult](semiorepo://definition/semio-repo/cli/main.go/ToolResult)

ToolResult holds the data fields for a tool result record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorticket](semiorepo://definition/semio-repo/cli/main.go/ContributorTicket)

ContributorTicket holds the data fields for a contributor ticket record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorcommit](semiorepo://definition/semio-repo/cli/main.go/ContributorCommit)

ContributorCommit holds the data fields for a contributor commit record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorcontributionsstorage](semiorepo://definition/semio-repo/cli/main.go/ContributorContributionsStorage)

ContributorContributionsStorage holds the data fields for a contributor contributions storage record.

## [🧰semiorepo⌨️cli💻maingo✂️bundlemetricsinternal](semiorepo://definition/semio-repo/cli/main.go/BundleMetricsInternal)

BundleMetricsInternal holds the data fields for a bundle metrics internal record.

## [🧰semiorepo⌨️cli💻maingo✂️foldermetricsinternal](semiorepo://definition/semio-repo/cli/main.go/FolderMetricsInternal)

FolderMetricsInternal holds the data fields for a folder metrics internal record.

## [🧰semiorepo⌨️cli💻maingo✂️filemetricsinternal](semiorepo://definition/semio-repo/cli/main.go/FileMetricsInternal)

FileMetricsInternal holds the data fields for a file metrics internal record.

## [🧰semiorepo⌨️cli💻maingo✂️sectionmetricsinternal](semiorepo://definition/semio-repo/cli/main.go/SectionMetricsInternal)

SectionMetricsInternal holds the data fields for a section metrics internal record.

## [🧰semiorepo⌨️cli💻maingo✂️definitionmetricsinternal](semiorepo://definition/semio-repo/cli/main.go/DefinitionMetricsInternal)

DefinitionMetricsInternal holds the data fields for a definition metrics internal record.

## [🧰semiorepo⌨️cli💻maingo✂️rangeposition](semiorepo://definition/semio-repo/cli/main.go/RangePosition)

RangePosition holds the data fields for a range position record.

## [🧰semiorepo⌨️cli💻maingo✂️filerange](semiorepo://definition/semio-repo/cli/main.go/FileRange)

FileRange holds the data fields for a file range record.

## [🧰semiorepo⌨️cli💻maingo✂️breachfile](semiorepo://definition/semio-repo/cli/main.go/BreachFile)

BreachFile holds the data fields for a breach file record.

## [🧰semiorepo⌨️cli💻maingo✂️breachfolder](semiorepo://definition/semio-repo/cli/main.go/BreachFolder)

BreachFolder holds the data fields for a breach folder record.

## [🧰semiorepo⌨️cli💻maingo✂️codebasebreach](semiorepo://definition/semio-repo/cli/main.go/CodebaseBreach)

CodebaseBreach holds the data fields for a codebase breach record.

## [🧰semiorepo⌨️cli💻maingo✂️codebasebundle](semiorepo://definition/semio-repo/cli/main.go/CodebaseBundle)

CodebaseBundle holds the data fields for a codebase bundle record.

## [🧰semiorepo⌨️cli💻maingo✂️codebasefolder](semiorepo://definition/semio-repo/cli/main.go/CodebaseFolder)

CodebaseFolder holds the data fields for a codebase folder record.

## [🧰semiorepo⌨️cli💻maingo✂️filebreachref](semiorepo://definition/semio-repo/cli/main.go/FileBreachRef)

FileBreachRef holds the data fields for a file breach ref record.

## [🧰semiorepo⌨️cli💻maingo✂️codebasefile](semiorepo://definition/semio-repo/cli/main.go/CodebaseFile)

CodebaseFile holds the data fields for a codebase file record.

## [🧰semiorepo⌨️cli💻maingo✂️codebasesection](semiorepo://definition/semio-repo/cli/main.go/CodebaseSection)

CodebaseSection holds the data fields for a codebase section record.

## [🧰semiorepo⌨️cli💻maingo✂️codebasedefinition](semiorepo://definition/semio-repo/cli/main.go/CodebaseDefinition)

CodebaseDefinition holds the data fields for a codebase definition record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorbundlecontrib](semiorepo://definition/semio-repo/cli/main.go/ContributorBundleContrib)

ContributorBundleContrib holds the data fields for a contributor bundle contrib record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorfoldercontrib](semiorepo://definition/semio-repo/cli/main.go/ContributorFolderContrib)

ContributorFolderContrib holds the data fields for a contributor folder contrib record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorfilecontrib](semiorepo://definition/semio-repo/cli/main.go/ContributorFileContrib)

ContributorFileContrib holds the data fields for a contributor file contrib record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorsectioncontrib](semiorepo://definition/semio-repo/cli/main.go/ContributorSectionContrib)

ContributorSectionContrib holds the data fields for a contributor section contrib record.

## [🧰semiorepo⌨️cli💻maingo✂️contributordefinitioncontrib](semiorepo://definition/semio-repo/cli/main.go/ContributorDefinitionContrib)

ContributorDefinitionContrib holds the data fields for a contributor definition contrib record.

## [🧰semiorepo⌨️cli💻maingo✂️contributorcontributionsinternal](semiorepo://definition/semio-repo/cli/main.go/ContributorContributionsInternal)

ContributorContributionsInternal holds the data fields for a contributor contributions internal record.

## [🧰semiorepo⌨️cli💻maingo✂️contributormetricsinternal](semiorepo://definition/semio-repo/cli/main.go/ContributorMetricsInternal)

ContributorMetricsInternal holds the data fields for a contributor metrics internal record.

## [🧰semiorepo⌨️cli💻maingo✂️codebasecontributor](semiorepo://definition/semio-repo/cli/main.go/CodebaseContributor)

CodebaseContributor holds the data fields for a codebase contributor record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketdateinfo](semiorepo://definition/semio-repo/cli/main.go/TicketDateInfo)

TicketDateInfo holds the data fields for a ticket date info record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketbundlecontribinfo](semiorepo://definition/semio-repo/cli/main.go/TicketBundleContribInfo)

TicketBundleContribInfo holds the data fields for a ticket bundle contrib info record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketfoldercontribinfo](semiorepo://definition/semio-repo/cli/main.go/TicketFolderContribInfo)

TicketFolderContribInfo holds the data fields for a ticket folder contrib info record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketfilecontribinfo](semiorepo://definition/semio-repo/cli/main.go/TicketFileContribInfo)

TicketFileContribInfo holds the data fields for a ticket file contrib info record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketsectioncontribinfo](semiorepo://definition/semio-repo/cli/main.go/TicketSectionContribInfo)

TicketSectionContribInfo holds the data fields for a ticket section contrib info record.

## [🧰semiorepo⌨️cli💻maingo✂️ticketdefinitioncontrib](semiorepo://definition/semio-repo/cli/main.go/TicketDefinitionContrib)

TicketDefinitionContrib holds the data fields for a ticket definition contrib record.

## [🧰semiorepo⌨️cli💻maingo✂️codebaseticket](semiorepo://definition/semio-repo/cli/main.go/CodebaseTicket)

CodebaseTicket holds the data fields for a codebase ticket record.

## [🧰semiorepo⌨️cli💻maingo✂️policybreachref](semiorepo://definition/semio-repo/cli/main.go/PolicyBreachRef)

PolicyBreachRef holds the data fields for a policy breach ref record.

## [🧰semiorepo⌨️cli💻maingo✂️codebasepolicy](semiorepo://definition/semio-repo/cli/main.go/CodebasePolicy)

CodebasePolicy holds the data fields for a codebase policy record.

## [🧰semiorepo⌨️cli💻maingo✂️cbtreenodekind](semiorepo://definition/semio-repo/cli/main.go/CbTreeNodeKind)

CbTreeNodeKind represents a cb tree node kind value.

## [🧰semiorepo⌨️cli💻maingo✂️cbtreenode](semiorepo://definition/semio-repo/cli/main.go/CbTreeNode)

CbTreeNode holds the data fields for a cb tree node record.

## [🧰semiorepo⌨️cli💻maingo✂️codebase](semiorepo://definition/semio-repo/cli/main.go/Codebase)

Codebase holds the data fields for a codebase record.

## [🧰semiorepo⌨️cli💻maingo🛠️getrootdir](semiorepo://definition/semio-repo/cli/main.go/GetRootDir)

GetRootDir returns the root dir of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️setrootdir](semiorepo://definition/semio-repo/cli/main.go/SetRootDir)

SetRootDir sets the root dir on the value.

## [🧰semiorepo⌨️cli💻maingo🛠️getrepometadir](semiorepo://definition/semio-repo/cli/main.go/GetRepoMetaDir)

GetRepoMetaDir returns the repo meta dir of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️getrepometapath](semiorepo://definition/semio-repo/cli/main.go/GetRepoMetaPath)

GetRepoMetaPath returns the repo meta path of the value.

## [🧰semiorepo⌨️cli💻maingo✂️gitignorepattern](semiorepo://definition/semio-repo/cli/main.go/GitignorePattern)

GitignorePattern holds the data fields for a gitignore pattern record.

## [🧰semiorepo⌨️cli💻maingo🛠️normalizepath](semiorepo://definition/semio-repo/cli/main.go/NormalizePath)

NormalizePath normalizes the path to its canonical form.

## [🧰semiorepo⌨️cli💻maingo🛠️ensuredir](semiorepo://definition/semio-repo/cli/main.go/EnsureDir)

EnsureDir ensures the dir exists, creating it if necessary.

## [🧰semiorepo⌨️cli💻maingo🛠️getrelativepath](semiorepo://definition/semio-repo/cli/main.go/GetRelativePath)

GetRelativePath returns the relative path of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️readtextfile](semiorepo://definition/semio-repo/cli/main.go/ReadTextFile)

ReadTextFile reads and returns the text file content.

## [🧰semiorepo⌨️cli💻maingo🛠️writetextfile](semiorepo://definition/semio-repo/cli/main.go/WriteTextFile)

WriteTextFile writes the text file content to storage.

## [🧰semiorepo⌨️cli💻maingo🛠️writejsonfile](semiorepo://definition/semio-repo/cli/main.go/WriteJSONFile)

WriteJSONFile writes the j s o n file content to storage.

## [🧰semiorepo⌨️cli💻maingo🛠️readjsonfile](semiorepo://definition/semio-repo/cli/main.go/ReadJSONFile)

ReadJSONFile reads and returns the j s o n file content.

## [🧰semiorepo⌨️cli💻maingo🛠️fileexists](semiorepo://definition/semio-repo/cli/main.go/FileExists)

FileExists performs the file exists operation.

## [🧰semiorepo⌨️cli💻maingo🛠️isdir](semiorepo://definition/semio-repo/cli/main.go/IsDir)

IsDir reports whether the value is dir.

## [🧰semiorepo⌨️cli💻maingo🛠️loadgitignore](semiorepo://definition/semio-repo/cli/main.go/LoadGitignore)

LoadGitignore loads the gitignore from storage.

## [🧰semiorepo⌨️cli💻maingo🛠️simpleglob](semiorepo://definition/semio-repo/cli/main.go/SimpleGlob)

SimpleGlob performs the simple glob operation.

## [🧰semiorepo⌨️cli💻maingo🛠️isotimestamp](semiorepo://definition/semio-repo/cli/main.go/ISOTimestamp)

ISOTimestamp performs the i s o timestamp operation.

## [🧰semiorepo⌨️cli💻maingo🛠️formatdate](semiorepo://definition/semio-repo/cli/main.go/FormatDate)

FormatDate formats the date into its string representation.

## [🧰semiorepo⌨️cli💻maingo🛠️padnumber](semiorepo://definition/semio-repo/cli/main.go/PadNumber)

PadNumber performs the pad number operation.

## [🧰semiorepo⌨️cli💻maingo🛠️pathtouripath](semiorepo://definition/semio-repo/cli/main.go/PathToUriPath)

PathToUriPath performs the path to uri path operation (no whitespace, reversible).

## [🧰semiorepo⌨️cli💻maingo🛠️pathfromuripath](semiorepo://definition/semio-repo/cli/main.go/PathFromUriPath)

PathFromUriPath performs the uri path to path operation (reverse of PathToUriPath).

## [🧰semiorepo⌨️cli💻maingo🛠️flat](semiorepo://definition/semio-repo/cli/main.go/Flat)

Flat performs the Flat operation.

## [🧰semiorepo⌨️cli💻maingo🛠️slugify](semiorepo://definition/semio-repo/cli/main.go/Slugify)

Slugify performs the slugify operation.

## [🧰semiorepo⌨️cli💻maingo🛠️titleizeslug](semiorepo://definition/semio-repo/cli/main.go/TitleizeSlug)

TitleizeSlug performs the titleize slug operation.

## [🧰semiorepo⌨️cli💻maingo🛠️statutepathtoidvalue](semiorepo://definition/semio-repo/cli/main.go/StatutePathToIdValue)

StatutePathToIdValue performs the statute path to id value operation.

## [🧰semiorepo⌨️cli💻maingo🛠️statuteidvaluetopath](semiorepo://definition/semio-repo/cli/main.go/StatuteIdValueToPath)

StatuteIdValueToPath performs the statute id value to path operation.

## [🧰semiorepo⌨️cli💻maingo🛠️execcommand](semiorepo://definition/semio-repo/cli/main.go/ExecCommand)

ExecCommand performs the exec command operation.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitauthor](semiorepo://definition/semio-repo/cli/main.go/GetGitAuthor)

GetGitAuthor returns the git author of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitauthorgithub](semiorepo://definition/semio-repo/cli/main.go/GetGitAuthorGithub)

GetGitAuthorGithub returns the git author github of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitcommit](semiorepo://definition/semio-repo/cli/main.go/GetGitCommit)

GetGitCommit returns the git commit of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitignoredset](semiorepo://definition/semio-repo/cli/main.go/GetGitIgnoredSet)

GetGitIgnoredSet returns the git ignored set of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️newoutput](semiorepo://definition/semio-repo/cli/main.go/NewOutput)

NewOutput creates and returns a new Output instance.

## [🧰semiorepo⌨️cli💻maingo🛠️info](semiorepo://definition/semio-repo/cli/main.go/Info)

Info returns the metadata for the statute.

## [🧰semiorepo⌨️cli💻maingo🛠️success](semiorepo://definition/semio-repo/cli/main.go/Success)

Success performs the success operation on the CommandOutput.

## [🧰semiorepo⌨️cli💻maingo🛠️error](semiorepo://definition/semio-repo/cli/main.go/Error)

Error returns the string representation of the error.

## [🧰semiorepo⌨️cli💻maingo🛠️warn](semiorepo://definition/semio-repo/cli/main.go/Warn)

Warn performs the warn operation on the CommandOutput.

## [🧰semiorepo⌨️cli💻maingo🛠️plain](semiorepo://definition/semio-repo/cli/main.go/Plain)

Plain performs the plain operation on the CommandOutput.

## [🧰semiorepo⌨️cli💻maingo🛠️print](semiorepo://definition/semio-repo/cli/main.go/Print)

Print performs the print operation on the CommandOutput.

## [🧰semiorepo⌨️cli💻maingo🛠️json](semiorepo://definition/semio-repo/cli/main.go/Json)

Json performs the json operation on the CommandOutput.

## [🧰semiorepo⌨️cli💻maingo🛠️listdirentries](semiorepo://definition/semio-repo/cli/main.go/ListDirEntries)

ListDirEntries returns all available dir entries entries.

## [🧰semiorepo⌨️cli💻maingo🛠️walkdir](semiorepo://definition/semio-repo/cli/main.go/WalkDir)

WalkDir recursively walks the dir and invokes the callback.

## [🧰semiorepo⌨️cli💻maingo🛠️parsescope](semiorepo://definition/semio-repo/cli/main.go/ParseScope)

ParseScope parses the input and returns the scope result.

## [🧰semiorepo⌨️cli💻maingo🛠️readlines](semiorepo://definition/semio-repo/cli/main.go/ReadLines)

ReadLines reads and returns the lines content.

## [🧰semiorepo⌨️cli💻maingo🛠️parsecodesections](semiorepo://definition/semio-repo/cli/main.go/ParseCodeSections)

ParseCodeSections parses the input and returns the code sections result.

## [🧰semiorepo⌨️cli💻maingo🛠️parsemarkdownsectionsinternal](semiorepo://definition/semio-repo/cli/main.go/ParseMarkdownSectionsInternal)

ParseMarkdownSectionsInternal parses the input and returns the markdown sections internal result.

## [🧰semiorepo⌨️cli💻maingo✂️jsonsectionlocation](semiorepo://definition/semio-repo/cli/main.go/JsonSectionLocation)

JsonSectionLocation holds the data fields for a json section location record.

## [🧰semiorepo⌨️cli💻maingo🛠️parsejsonsectionsdetailed](semiorepo://definition/semio-repo/cli/main.go/ParseJSONSectionsDetailed)

ParseJSONSectionsDetailed parses the input and returns the j s o n sections detailed result.

## [🧰semiorepo⌨️cli💻maingo🛠️parsejsonsections](semiorepo://definition/semio-repo/cli/main.go/ParseJSONSections)

ParseJSONSections parses the input and returns the j s o n sections result.

## [🧰semiorepo⌨️cli💻maingo🛠️parsesections](semiorepo://definition/semio-repo/cli/main.go/ParseSections)

ParseSections parses the input and returns the sections result.

## [🧰semiorepo⌨️cli💻maingo🛠️parsedefinitions](semiorepo://definition/semio-repo/cli/main.go/ParseDefinitions)

ParseDefinitions parses the input and returns the definitions result.

## [🧰semiorepo⌨️cli💻maingo🛠️hydratesectionswithdefinitions](semiorepo://definition/semio-repo/cli/main.go/HydrateSectionsWithDefinitions)

HydrateSectionsWithDefinitions populates the sections with definitions with associated child data.

## [🧰semiorepo⌨️cli💻maingo🛠️normalizesectionpath](semiorepo://definition/semio-repo/cli/main.go/NormalizeSectionPath)

NormalizeSectionPath normalizes the section path to its canonical form.

## [🧰semiorepo⌨️cli💻maingo🛠️findsection](semiorepo://definition/semio-repo/cli/main.go/FindSection)

FindSection searches for and returns the matching section.

## [🧰semiorepo⌨️cli💻maingo✂️policyfunc](semiorepo://definition/semio-repo/cli/main.go/PolicyFunc)

PolicyFunc is a function type for policy func callbacks.

## [🧰semiorepo⌨️cli💻maingo🛠️findpolicy](semiorepo://definition/semio-repo/cli/main.go/FindPolicy)

FindPolicy searches for and returns the matching policy.

## [🧰semiorepo⌨️cli💻maingo🛠️getpolicies](semiorepo://definition/semio-repo/cli/main.go/GetPolicies)

GetPolicies returns the policies of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️streampolicies](semiorepo://definition/semio-repo/cli/main.go/StreamPolicies)

StreamPolicies streams the policies over a channel with optional filtering.

## [🧰semiorepo⌨️cli💻maingo✂️policycontext](semiorepo://definition/semio-repo/cli/main.go/PolicyContext)

PolicyContext holds the data fields for a policy context record.

## [🧰semiorepo⌨️cli💻maingo🛠️newpolicycontext](semiorepo://definition/semio-repo/cli/main.go/NewPolicyContext)

NewPolicyContext creates and returns a new PolicyContext instance.

## [🧰semiorepo⌨️cli💻maingo🛠️newpolicycontextwithfiles](semiorepo://definition/semio-repo/cli/main.go/NewPolicyContextWithFiles)

NewPolicyContextWithFiles creates and returns a new PolicyContextWithFiles instance.

## [🧰semiorepo⌨️cli💻maingo🛠️files](semiorepo://definition/semio-repo/cli/main.go/Files)

Files performs the files operation on the PolicyContext.

## [🧰semiorepo⌨️cli💻maingo🛠️readtext](semiorepo://definition/semio-repo/cli/main.go/ReadText)

ReadText reads and returns the text content.

## [🧰semiorepo⌨️cli💻maingo🛠️sections](semiorepo://definition/semio-repo/cli/main.go/Sections)

Sections performs the sections operation on the PolicyContext.

## [🧰semiorepo⌨️cli💻maingo🛠️parseignoredirectives](semiorepo://definition/semio-repo/cli/main.go/ParseIgnoreDirectives)

ParseIgnoreDirectives parses the input and returns the ignore directives result.

## [🧰semiorepo⌨️cli💻maingo🛠️ignoredirectives](semiorepo://definition/semio-repo/cli/main.go/IgnoreDirectives)

IgnoreDirectives performs the ignore directives operation on the PolicyContext.

## [🧰semiorepo⌨️cli💻maingo🛠️isignored](semiorepo://definition/semio-repo/cli/main.go/IsIgnored)

IsIgnored reports whether the PolicyContext is ignored.

## [🧰semiorepo⌨️cli💻maingo🛠️createbreach](semiorepo://definition/semio-repo/cli/main.go/CreateBreach)

CreateBreach creates a new breach and persists it.

## [🧰semiorepo⌨️cli💻maingo🛠️filterignored](semiorepo://definition/semio-repo/cli/main.go/FilterIgnored)

FilterIgnored filters the ignored based on the given criteria.

## [🧰semiorepo⌨️cli💻maingo🛠️speclines](semiorepo://definition/semio-repo/cli/main.go/SpecLines)

SpecLines performs the spec lines operation on the PolicyContext.

## [🧰semiorepo⌨️cli💻maingo🛠️isspecline](semiorepo://definition/semio-repo/cli/main.go/IsSpecLine)

IsSpecLine reports whether the PolicyContext is spec line.

## [🧰semiorepo⌨️cli💻maingo🛠️isspecblock](semiorepo://definition/semio-repo/cli/main.go/IsSpecBlock)

IsSpecBlock reports whether the PolicyContext is spec block.

## [🧰semiorepo⌨️cli💻maingo🛠️sectiondoclines](semiorepo://definition/semio-repo/cli/main.go/SectionDocLines)

SectionDocLines performs the section doc lines operation on the PolicyContext.

## [🧰semiorepo⌨️cli💻maingo🛠️issectiondocline](semiorepo://definition/semio-repo/cli/main.go/IsSectionDocLine)

IsSectionDocLine reports whether the PolicyContext is section doc line.

## [🧰semiorepo⌨️cli💻maingo🛠️definitiondoclines](semiorepo://definition/semio-repo/cli/main.go/DefinitionDocLines)

DefinitionDocLines performs the definition doc lines operation on the PolicyContext.

## [🧰semiorepo⌨️cli💻maingo🛠️isdefinitiondocline](semiorepo://definition/semio-repo/cli/main.go/IsDefinitionDocLine)

IsDefinitionDocLine reports whether the PolicyContext is definition doc line.

## [🧰semiorepo⌨️cli💻maingo🛠️checkpolicies](semiorepo://definition/semio-repo/cli/main.go/CheckPolicies)

CheckPolicies validates the policies and returns any breachs.

## [🧰semiorepo⌨️cli💻maingo🛠️checkpolicieswithcontext](semiorepo://definition/semio-repo/cli/main.go/CheckPoliciesWithContext)

CheckPoliciesWithContext validates the policies with context and returns any breachs.

## [🧰semiorepo⌨️cli💻maingo✂️commenttemplatestate](semiorepo://definition/semio-repo/cli/main.go/CommentTemplateState)

CommentTemplateState holds the data fields for a comment template state record.

## [🧰semiorepo⌨️cli💻maingo✂️commentscanstate](semiorepo://definition/semio-repo/cli/main.go/CommentScanState)

CommentScanState holds the data fields for a comment scan state record.

## [🧰semiorepo⌨️cli💻maingo🛠️intemplateraw](semiorepo://definition/semio-repo/cli/main.go/InTemplateRaw)

InTemplateRaw performs the in template raw operation on the CommentScanState.

## [🧰semiorepo⌨️cli💻maingo✂️godfile](semiorepo://definition/semio-repo/cli/main.go/Godfile)

Godfile holds the data fields for a Godfile record.

## [🧰semiorepo⌨️cli💻maingo✂️codebasecontext](semiorepo://definition/semio-repo/cli/main.go/CodebaseContext)

CodebaseContext holds the data fields for a codebase context record.

## [🧰semiorepo⌨️cli💻maingo🛠️newcodebasecontext](semiorepo://definition/semio-repo/cli/main.go/NewCodebaseContext)

NewCodebaseContext creates and returns a new CodebaseContext instance.

## [🧰semiorepo⌨️cli💻maingo🛠️loadbundles](semiorepo://definition/semio-repo/cli/main.go/LoadBundles)

LoadBundles loads the bundles from storage.

## [🧰semiorepo⌨️cli💻maingo🛠️loadfiles](semiorepo://definition/semio-repo/cli/main.go/LoadFiles)

LoadFiles loads the files from storage.

## [🧰semiorepo⌨️cli💻maingo🛠️loadbreachs](semiorepo://definition/semio-repo/cli/main.go/LoadBreachs)

LoadBreachs loads the breachs from storage.

## [🧰semiorepo⌨️cli💻maingo🛠️loadtickets](semiorepo://definition/semio-repo/cli/main.go/LoadTickets)

LoadTickets loads the tickets from storage.

## [🧰semiorepo⌨️cli💻maingo🛠️loadpolicies](semiorepo://definition/semio-repo/cli/main.go/LoadPolicies)

LoadPolicies loads the policies from storage.

## [🧰semiorepo⌨️cli💻maingo🛠️getbundleforfile](semiorepo://definition/semio-repo/cli/main.go/GetBundleForFile)

GetBundleForFile returns the bundle for file of the CodebaseContext.

## [🧰semiorepo⌨️cli💻maingo🛠️getbundleinfo](semiorepo://definition/semio-repo/cli/main.go/GetBundleInfo)

GetBundleInfo returns the bundle info of the CodebaseContext.

## [🧰semiorepo⌨️cli💻maingo🛠️getfileid](semiorepo://definition/semio-repo/cli/main.go/GetFileID)

GetFileID returns the file i d of the CodebaseContext.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolderid](semiorepo://definition/semio-repo/cli/main.go/GetFolderID)

GetFolderID returns the folder i d of the CodebaseContext.

## [🧰semiorepo⌨️cli💻maingo🛠️fileuri](semiorepo://definition/semio-repo/cli/main.go/FileURI)

FileURI performs the file u r i operation on the CodebaseContext.

## [🧰semiorepo⌨️cli💻maingo🛠️folderuri](semiorepo://definition/semio-repo/cli/main.go/FolderURI)

FolderURI performs the folder u r i operation on the CodebaseContext.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasebundles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseBundles)

BuildCodebaseBundles constructs and returns the codebase bundles structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasefolders](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseFolders)

BuildCodebaseFolders constructs and returns the codebase folders structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasefiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseFiles)

BuildCodebaseFiles constructs and returns the codebase files structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasesections](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseSections)

BuildCodebaseSections constructs and returns the codebase sections structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasedefinitions](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseDefinitions)

BuildCodebaseDefinitions constructs and returns the codebase definitions structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasecontributors](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseContributors)

BuildCodebaseContributors constructs and returns the codebase contributors structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasetickets](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseTickets)

BuildCodebaseTickets constructs and returns the codebase tickets structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasepolicies](semiorepo://definition/semio-repo/cli/main.go/BuildCodebasePolicies)

BuildCodebasePolicies constructs and returns the codebase policies structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasebreachs](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseBreachs)

BuildCodebaseBreachs constructs and returns the codebase breachs structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasetree](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseTree)

BuildCodebaseTree constructs and returns the codebase tree structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebase](semiorepo://definition/semio-repo/cli/main.go/BuildCodebase)

BuildCodebase constructs and returns the codebase structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasesnapshot](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseSnapshot)

BuildCodebaseSnapshot constructs and returns the codebase snapshot structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasebundlesforfiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseBundlesForFiles)

BuildCodebaseBundlesForFiles constructs and returns the codebase bundles for files structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasefoldersforfiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseFoldersForFiles)

BuildCodebaseFoldersForFiles constructs and returns the codebase folders for files structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasefilesforfiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseFilesForFiles)

BuildCodebaseFilesForFiles constructs and returns the codebase files for files structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasesectionsforfiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseSectionsForFiles)

BuildCodebaseSectionsForFiles constructs and returns the codebase sections for files structure.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasedefinitionsforfiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseDefinitionsForFiles)

BuildCodebaseDefinitionsForFiles constructs and returns the codebase definitions for files structure.

## [🧰semiorepo⌨️cli💻maingo🛠️toolcodebase](semiorepo://definition/semio-repo/cli/main.go/ToolCodebase)

ToolCodebase performs the tool codebase operation.

## [🧰semiorepo⌨️cli💻maingo🛠️getticketsdir](semiorepo://definition/semio-repo/cli/main.go/GetTicketsDir)

GetTicketsDir returns the tickets dir of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️getticketpath](semiorepo://definition/semio-repo/cli/main.go/GetTicketPath)

GetTicketPath returns the ticket path of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️getticketfilepath](semiorepo://definition/semio-repo/cli/main.go/GetTicketFilePath)

GetTicketFilePath returns the ticket file path of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️getimportantfilepath](semiorepo://definition/semio-repo/cli/main.go/GetImportantFilePath)

GetImportantFilePath returns the important file path of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️getticketjsonpath](semiorepo://definition/semio-repo/cli/main.go/GetTicketJsonPath)

GetTicketJsonPath returns the ticket json path of the value.

## [🧰semiorepo⌨️cli💻maingo🛠️findticketbyslug](semiorepo://definition/semio-repo/cli/main.go/FindTicketBySlug)

FindTicketBySlug searches for and returns the matching ticket by slug.

## [🧰semiorepo⌨️cli💻maingo🛠️latestticket](semiorepo://definition/semio-repo/cli/main.go/LatestTicket)

LatestTicket performs the latest ticket operation.

## [🧰semiorepo⌨️cli💻maingo🛠️openticket](semiorepo://definition/semio-repo/cli/main.go/OpenTicket)

OpenTicket performs the open ticket operation.

## [🧰semiorepo⌨️cli💻maingo🛠️opengoal](semiorepo://definition/semio-repo/cli/main.go/OpenGoal)

OpenGoal performs the open goal operation.

## [🧰semiorepo⌨️cli💻maingo🛠️updatetickettitle](semiorepo://definition/semio-repo/cli/main.go/UpdateTicketTitle)

UpdateTicketTitle performs the update ticket title operation.

## [🧰semiorepo⌨️cli💻maingo🛠️createticket](semiorepo://definition/semio-repo/cli/main.go/CreateTicket)

CreateTicket creates a new ticket and persists it.

## [🧰semiorepo⌨️cli💻maingo🛠️countlines](semiorepo://definition/semio-repo/cli/main.go/CountLines)

CountLines performs the count lines operation.

## [🧰semiorepo⌨️cli💻maingo🛠️countlinesinfile](semiorepo://definition/semio-repo/cli/main.go/CountLinesInFile)

CountLinesInFile performs the count lines in file operation.

## [🧰semiorepo⌨️cli💻maingo🛠️countlinesatcommit](semiorepo://definition/semio-repo/cli/main.go/CountLinesAtCommit)

CountLinesAtCommit performs the count lines at commit operation.

## [🧰semiorepo⌨️cli💻maingo🛠️readtextfileatcommit](semiorepo://definition/semio-repo/cli/main.go/ReadTextFileAtCommit)

ReadTextFileAtCommit reads and returns text file at commit from the source.

## [🧰semiorepo⌨️cli💻maingo🛠️listfilesatcommit](semiorepo://definition/semio-repo/cli/main.go/ListFilesAtCommit)

ListFilesAtCommit returns a list of files at commit entries.

## [🧰semiorepo⌨️cli💻maingo🛠️filterticketworkspacefiles](semiorepo://definition/semio-repo/cli/main.go/FilterTicketWorkspaceFiles)

FilterTicketWorkspaceFiles returns the subset of ticket workspace files matching the criteria.

## [🧰semiorepo⌨️cli💻maingo🛠️saveticket](semiorepo://definition/semio-repo/cli/main.go/SaveTicket)

SaveTicket persists ticket to the data store.

## [🧰semiorepo⌨️cli💻maingo🛠️readticket](semiorepo://definition/semio-repo/cli/main.go/ReadTicket)

ReadTicket reads and returns ticket from the source.

## [🧰semiorepo⌨️cli💻maingo🛠️listtickets](semiorepo://definition/semio-repo/cli/main.go/ListTickets)

ListTickets returns a list of tickets entries.

## [🧰semiorepo⌨️cli💻maingo🛠️streamtickets](semiorepo://definition/semio-repo/cli/main.go/StreamTickets)

StreamTickets streams tickets entries through the callback.

## [🧰semiorepo⌨️cli💻maingo🛠️invalidatetechnologycache](semiorepo://definition/semio-repo/cli/main.go/InvalidateTechnologyCache)

InvalidateTechnologyCache invalidates the cached technology cache.

## [🧰semiorepo⌨️cli💻maingo🛠️loadtechnologies](semiorepo://definition/semio-repo/cli/main.go/LoadTechnologies)

LoadTechnologies loads and returns technologies from the data source.

## [🧰semiorepo⌨️cli💻maingo🛠️loadcommits](semiorepo://definition/semio-repo/cli/main.go/LoadCommits)

LoadCommits loads and returns commits from the data source.

## [🧰semiorepo⌨️cli💻maingo🛠️loadbundles](semiorepo://definition/semio-repo/cli/main.go/LoadBundles)

LoadBundles loads and returns bundles from the data source.

## [🧰semiorepo⌨️cli💻maingo🛠️gettechnologies](semiorepo://definition/semio-repo/cli/main.go/GetTechnologies)

GetTechnologies retrieves and returns the technologies.

## [🧰semiorepo⌨️cli💻maingo🛠️streambundles](semiorepo://definition/semio-repo/cli/main.go/StreamBundles)

StreamBundles streams bundles entries through the callback.

## [🧰semiorepo⌨️cli💻maingo🛠️streamtechnologies](semiorepo://definition/semio-repo/cli/main.go/StreamTechnologies)

StreamTechnologies streams technologies entries through the callback.

## [🧰semiorepo⌨️cli💻maingo✂️streamoptions](semiorepo://definition/semio-repo/cli/main.go/StreamOptions)

StreamOptions holds the data fields for a stream options record.

## [🧰semiorepo⌨️cli💻maingo🛠️streamfolders](semiorepo://definition/semio-repo/cli/main.go/StreamFolders)

StreamFolders streams folders entries through the callback.

## [🧰semiorepo⌨️cli💻maingo🛠️streamfiles](semiorepo://definition/semio-repo/cli/main.go/StreamFiles)

StreamFiles streams files entries through the callback.

## [🧰semiorepo⌨️cli💻maingo🛠️streamsections](semiorepo://definition/semio-repo/cli/main.go/StreamSections)

StreamSections streams sections entries through the callback.

## [🧰semiorepo⌨️cli💻maingo🛠️streamdefinitions](semiorepo://definition/semio-repo/cli/main.go/StreamDefinitions)

StreamDefinitions streams definitions entries through the callback.

## [🧰semiorepo⌨️cli💻maingo🛠️resolvebundleforpath](semiorepo://definition/semio-repo/cli/main.go/ResolveBundleForPath)

ResolveBundleForPath resolves and returns the bundle for path.

## [🧰semiorepo⌨️cli💻maingo🛠️progressticket](semiorepo://definition/semio-repo/cli/main.go/ProgressTicket)

ProgressTicket performs the progress ticket operation.

## [🧰semiorepo⌨️cli💻maingo🛠️finishticket](semiorepo://definition/semio-repo/cli/main.go/FinishTicket)

FinishTicket performs the finish ticket operation.

## [🧰semiorepo⌨️cli💻maingo🛠️reopenticket](semiorepo://definition/semio-repo/cli/main.go/ReopenTicket)

ReopenTicket performs the reopen ticket operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolticketopen](semiorepo://definition/semio-repo/cli/main.go/ToolTicketOpen)

ToolTicketOpen performs the tool ticket open operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolticketlist](semiorepo://definition/semio-repo/cli/main.go/ToolTicketList)

ToolTicketList performs the tool ticket list operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolticketread](semiorepo://definition/semio-repo/cli/main.go/ToolTicketRead)

ToolTicketRead performs the tool ticket read operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolticketclose](semiorepo://definition/semio-repo/cli/main.go/ToolTicketClose)

ToolTicketClose performs the tool ticket close operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolticketreopen](semiorepo://definition/semio-repo/cli/main.go/ToolTicketReopen)

ToolTicketReopen performs the tool ticket reopen operation.

## [🧰semiorepo⌨️cli💻maingo🛠️tooldraftcreate](semiorepo://definition/semio-repo/cli/main.go/ToolDraftCreate)

ToolDraftCreate performs the tool draft create operation.

## [🧰semiorepo⌨️cli💻maingo🛠️tooldraftlist](semiorepo://definition/semio-repo/cli/main.go/ToolDraftList)

ToolDraftList performs the tool draft list operation.

## [🧰semiorepo⌨️cli💻maingo🛠️tooldraftdelete](semiorepo://definition/semio-repo/cli/main.go/ToolDraftDelete)

ToolDraftDelete performs the tool draft delete operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolgoalcreate](semiorepo://definition/semio-repo/cli/main.go/ToolGoalCreate)

ToolGoalCreate performs the tool goal create operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolgoallist](semiorepo://definition/semio-repo/cli/main.go/ToolGoalList)

ToolGoalList performs the tool goal list operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolgoalclose](semiorepo://definition/semio-repo/cli/main.go/ToolGoalClose)

ToolGoalClose performs the tool goal close operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolgoalreopen](semiorepo://definition/semio-repo/cli/main.go/ToolGoalReopen)

ToolGoalReopen performs the tool goal reopen operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolcontributoradd](semiorepo://definition/semio-repo/cli/main.go/ToolContributorAdd)

ToolContributorAdd performs the tool contributor add operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolcontributorlist](semiorepo://definition/semio-repo/cli/main.go/ToolContributorList)

ToolContributorList performs the tool contributor list operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolcontributorremove](semiorepo://definition/semio-repo/cli/main.go/ToolContributorRemove)

ToolContributorRemove performs the tool contributor remove operation.

## [🧰semiorepo⌨️cli💻maingo🛠️tooltechnologylist](semiorepo://definition/semio-repo/cli/main.go/ToolTechnologyList)

ToolTechnologyList performs the tool technology list operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolbundlelist](semiorepo://definition/semio-repo/cli/main.go/ToolBundleList)

ToolBundleList performs the tool bundle list operation.

## [🧰semiorepo⌨️cli💻maingo🛠️tooltechnologytree](semiorepo://definition/semio-repo/cli/main.go/ToolTechnologyTree)

ToolTechnologyTree performs the tool technology tree operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfoldercreate](semiorepo://definition/semio-repo/cli/main.go/ToolFolderCreate)

ToolFolderCreate performs the tool folder create operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfoldermove](semiorepo://definition/semio-repo/cli/main.go/ToolFolderMove)

ToolFolderMove performs the tool folder move operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfolderdelete](semiorepo://definition/semio-repo/cli/main.go/ToolFolderDelete)

ToolFolderDelete performs the tool folder delete operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfolderlist](semiorepo://definition/semio-repo/cli/main.go/ToolFolderList)

ToolFolderList performs the tool folder list operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfoldertree](semiorepo://definition/semio-repo/cli/main.go/ToolFolderTree)

ToolFolderTree performs the tool folder tree operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfilecreate](semiorepo://definition/semio-repo/cli/main.go/ToolFileCreate)

ToolFileCreate performs the tool file create operation.

## [🧰semiorepo⌨️cli💻maingo🛠️fileheaderid](semiorepo://definition/semio-repo/cli/main.go/FileHeaderId)

FileHeaderId performs the file header id operation.

## [🧰semiorepo⌨️cli💻maingo🛠️agpllicensetext](semiorepo://definition/semio-repo/cli/main.go/AGPLLicenseText)

AGPLLicenseText performs the a g p l license text operation.

## [🧰semiorepo⌨️cli💻maingo🛠️fileheaderuri](semiorepo://definition/semio-repo/cli/main.go/FileHeaderUri)

FileHeaderUri returns the artifact URI for the given file path.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionheaderid](semiorepo://definition/semio-repo/cli/main.go/SectionHeaderId)

SectionHeaderId returns the section identification string.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionheaderuri](semiorepo://definition/semio-repo/cli/main.go/SectionHeaderUri)

SectionHeaderUri returns the section artifact URI.

## [🧰semiorepo⌨️cli💻maingo🛠️definitionheaderid](semiorepo://definition/semio-repo/cli/main.go/DefinitionHeaderId)

DefinitionHeaderId returns the definition identification string.

## [🧰semiorepo⌨️cli💻maingo🛠️definitionheaderuri](semiorepo://definition/semio-repo/cli/main.go/DefinitionHeaderUri)

DefinitionHeaderUri returns the definition artifact URI.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfilemove](semiorepo://definition/semio-repo/cli/main.go/ToolFileMove)

ToolFileMove performs the tool file move operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfiledelete](semiorepo://definition/semio-repo/cli/main.go/ToolFileDelete)

ToolFileDelete performs the tool file delete operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfilelist](semiorepo://definition/semio-repo/cli/main.go/ToolFileList)

ToolFileList performs the tool file list operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfiletree](semiorepo://definition/semio-repo/cli/main.go/ToolFileTree)

ToolFileTree performs the tool file tree operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolsectioncreate](semiorepo://definition/semio-repo/cli/main.go/ToolSectionCreate)

ToolSectionCreate performs the tool section create operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolsectionmove](semiorepo://definition/semio-repo/cli/main.go/ToolSectionMove)

ToolSectionMove performs the tool section move operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolintegrate](semiorepo://definition/semio-repo/cli/main.go/ToolIntegrate)

ToolIntegrate performs the tool integrate operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolextract](semiorepo://definition/semio-repo/cli/main.go/ToolExtract)

ToolExtract performs the tool extract operation.

## [🧰semiorepo⌨️cli💻maingo🛠️updateagentsdocspath](semiorepo://definition/semio-repo/cli/main.go/UpdateAgentsDocsPath)

UpdateAgentsDocsPath modifies an existing agents docs path entry.

## [🧰semiorepo⌨️cli💻maingo🛠️removeagentsdocsentry](semiorepo://definition/semio-repo/cli/main.go/RemoveAgentsDocsEntry)

RemoveAgentsDocsEntry removes the specified agents docs entry.

## [🧰semiorepo⌨️cli💻maingo🛠️splitheader](semiorepo://definition/semio-repo/cli/main.go/SplitHeader)

SplitHeader splits the header into parts.

## [🧰semiorepo⌨️cli💻maingo🛠️mergeheaders](semiorepo://definition/semio-repo/cli/main.go/MergeHeaders)

MergeHeaders combines the headers entries into one.

## [🧰semiorepo⌨️cli💻maingo🛠️uniquestrings](semiorepo://definition/semio-repo/cli/main.go/UniqueStrings)

UniqueStrings performs the unique strings operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolsectiondelete](semiorepo://definition/semio-repo/cli/main.go/ToolSectionDelete)

ToolSectionDelete performs the tool section delete operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolsectionlist](semiorepo://definition/semio-repo/cli/main.go/ToolSectionList)

ToolSectionList performs the tool section list operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolsectiontree](semiorepo://definition/semio-repo/cli/main.go/ToolSectionTree)

ToolSectionTree performs the tool section tree operation.

## [🧰semiorepo⌨️cli💻maingo🛠️tooldefinitionlist](semiorepo://definition/semio-repo/cli/main.go/ToolDefinitionList)

ToolDefinitionList performs the tool definition list operation.

## [🧰semiorepo⌨️cli💻maingo🛠️tooldefinitiontree](semiorepo://definition/semio-repo/cli/main.go/ToolDefinitionTree)

ToolDefinitionTree performs the tool definition tree operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolupdatemetabolism](semiorepo://definition/semio-repo/cli/main.go/ToolUpdateMetabolism)

ToolUpdateMetabolism performs the tool update metabolism operation.

## [🧰semiorepo⌨️cli💻maingo✂️exportresult](semiorepo://definition/semio-repo/cli/main.go/ExportResult)

ExportResult holds the data fields for a export result record.

## [🧰semiorepo⌨️cli💻maingo🛠️exporttosqlite](semiorepo://definition/semio-repo/cli/main.go/ExportToSQLite)

ExportToSQLite exports the to s q lite to the target format.

## [🧰semiorepo⌨️cli💻maingo🛠️toolexport](semiorepo://definition/semio-repo/cli/main.go/ToolExport)

ToolExport performs the tool export operation.

## [🧰semiorepo⌨️cli💻maingo✂️repocontext](semiorepo://definition/semio-repo/cli/main.go/RepoContext)

RepoContext defines the interface for repo context operations.

## [🧰semiorepo⌨️cli💻maingo✂️resolver](semiorepo://definition/semio-repo/cli/main.go/Resolver)

Resolver holds the data fields for a resolver record.

## [🧰semiorepo⌨️cli💻maingo🛠️newresolver](semiorepo://definition/semio-repo/cli/main.go/NewResolver)

NewResolver creates and returns a new resolver instance.

## [🧰semiorepo⌨️cli💻maingo🛠️newresolverwithcontext](semiorepo://definition/semio-repo/cli/main.go/NewResolverWithContext)

NewResolverWithContext creates and returns a new resolver with context instance.

## [🧰semiorepo⌨️cli💻maingo🛠️newdefaultcontext](semiorepo://definition/semio-repo/cli/main.go/NewDefaultContext)

NewDefaultContext creates and returns a new default context instance.

## [🧰semiorepo⌨️cli💻maingo🛠️newrepocontext](semiorepo://definition/semio-repo/cli/main.go/NewRepoContext)

NewRepoContext creates and returns a new repo context instance.

## [🧰semiorepo⌨️cli💻maingo🛠️getrootdir](semiorepo://definition/semio-repo/cli/main.go/GetRootDir)

GetRootDir retrieves and returns the root dir.

## [🧰semiorepo⌨️cli💻maingo🛠️getfileid](semiorepo://definition/semio-repo/cli/main.go/GetFileID)

GetFileID retrieves and returns the file i d.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolderid](semiorepo://definition/semio-repo/cli/main.go/GetFolderID)

GetFolderID retrieves and returns the folder i d.

## [🧰semiorepo⌨️cli💻maingo🛠️getbundles](semiorepo://definition/semio-repo/cli/main.go/GetBundles)

GetBundles retrieves and returns the bundles.

## [🧰semiorepo⌨️cli💻maingo🛠️gettechnologies](semiorepo://definition/semio-repo/cli/main.go/GetTechnologies)

GetTechnologies retrieves and returns the technologies.

## [🧰semiorepo⌨️cli💻maingo🛠️getcommits](semiorepo://definition/semio-repo/cli/main.go/GetCommits)

GetCommits retrieves and returns the commits.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolders](semiorepo://definition/semio-repo/cli/main.go/GetFolders)

GetFolders retrieves and returns the folders.

## [🧰semiorepo⌨️cli💻maingo🛠️getfiles](semiorepo://definition/semio-repo/cli/main.go/GetFiles)

GetFiles retrieves and returns the files.

## [🧰semiorepo⌨️cli💻maingo🛠️getdefinitions](semiorepo://definition/semio-repo/cli/main.go/GetDefinitions)

GetDefinitions retrieves and returns the definitions.

## [🧰semiorepo⌨️cli💻maingo🛠️getsections](semiorepo://definition/semio-repo/cli/main.go/GetSections)

GetSections retrieves and returns the sections.

## [🧰semiorepo⌨️cli💻maingo🛠️getcontributors](semiorepo://definition/semio-repo/cli/main.go/GetContributors)

GetContributors retrieves and returns the contributors.

## [🧰semiorepo⌨️cli💻maingo🛠️gettickets](semiorepo://definition/semio-repo/cli/main.go/GetTickets)

GetTickets retrieves and returns the tickets.

## [🧰semiorepo⌨️cli💻maingo🛠️getgoals](semiorepo://definition/semio-repo/cli/main.go/GetGoals)

GetGoals retrieves and returns the goals.

## [🧰semiorepo⌨️cli💻maingo🛠️goalcreate](semiorepo://definition/semio-repo/cli/main.go/GoalCreate)

GoalCreate performs the goal create operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️updategoaltitle](semiorepo://definition/semio-repo/cli/main.go/UpdateGoalTitle)

UpdateGoalTitle modifies an existing goal title entry.

## [🧰semiorepo⌨️cli💻maingo🛠️goalchange](semiorepo://definition/semio-repo/cli/main.go/GoalChange)

GoalChange performs the goal change operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️goalclose](semiorepo://definition/semio-repo/cli/main.go/GoalClose)

GoalClose performs the goal close operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️goalreopen](semiorepo://definition/semio-repo/cli/main.go/GoalReopen)

GoalReopen performs the goal reopen operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketchange](semiorepo://definition/semio-repo/cli/main.go/TicketChange)

TicketChange performs the ticket change operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️goaldelete](semiorepo://definition/semio-repo/cli/main.go/GoalDelete)

GoalDelete performs the goal delete operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketdelete](semiorepo://definition/semio-repo/cli/main.go/TicketDelete)

TicketDelete performs the ticket delete operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️getdrafts](semiorepo://definition/semio-repo/cli/main.go/GetDrafts)

GetDrafts retrieves and returns the drafts.

## [🧰semiorepo⌨️cli💻maingo🛠️draftcreate](semiorepo://definition/semio-repo/cli/main.go/DraftCreate)

DraftCreate performs the draft create operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️draftdelete](semiorepo://definition/semio-repo/cli/main.go/DraftDelete)

DraftDelete performs the draft delete operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️getpolicies](semiorepo://definition/semio-repo/cli/main.go/GetPolicies)

GetPolicies retrieves and returns the policies.

## [🧰semiorepo⌨️cli💻maingo🛠️getstatutes](semiorepo://definition/semio-repo/cli/main.go/GetStatutes)

GetStatutes retrieves and returns the statutes.

## [🧰semiorepo⌨️cli💻maingo🛠️analyze](semiorepo://definition/semio-repo/cli/main.go/Analyze)

Analyze performs the analyze operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️fix](semiorepo://definition/semio-repo/cli/main.go/Fix)

Fix performs the fix operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketopen](semiorepo://definition/semio-repo/cli/main.go/TicketOpen)

TicketOpen performs the ticket open operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketprogress](semiorepo://definition/semio-repo/cli/main.go/TicketProgress)

TicketProgress performs the ticket progress operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketclose](semiorepo://definition/semio-repo/cli/main.go/TicketClose)

TicketClose performs the ticket close operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketreopen](semiorepo://definition/semio-repo/cli/main.go/TicketReopen)

TicketReopen performs the ticket reopen operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️foldercreate](semiorepo://definition/semio-repo/cli/main.go/FolderCreate)

FolderCreate performs the folder create operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️foldermove](semiorepo://definition/semio-repo/cli/main.go/FolderMove)

FolderMove performs the folder move operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️folderdelete](semiorepo://definition/semio-repo/cli/main.go/FolderDelete)

FolderDelete performs the folder delete operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️filecreate](semiorepo://definition/semio-repo/cli/main.go/FileCreate)

FileCreate performs the file create operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️filemove](semiorepo://definition/semio-repo/cli/main.go/FileMove)

FileMove performs the file move operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️filedelete](semiorepo://definition/semio-repo/cli/main.go/FileDelete)

FileDelete performs the file delete operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️sectioncreate](semiorepo://definition/semio-repo/cli/main.go/SectionCreate)

SectionCreate performs the section create operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionmove](semiorepo://definition/semio-repo/cli/main.go/SectionMove)

SectionMove performs the section move operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️sectiondelete](semiorepo://definition/semio-repo/cli/main.go/SectionDelete)

SectionDelete performs the section delete operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️integrate](semiorepo://definition/semio-repo/cli/main.go/Integrate)

Integrate performs the integrate operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️extract](semiorepo://definition/semio-repo/cli/main.go/Extract)

Extract extracts the extract from the source.

## [🧰semiorepo⌨️cli💻maingo🛠️contributoradd](semiorepo://definition/semio-repo/cli/main.go/ContributorAdd)

ContributorAdd performs the contributor add operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️contributorremove](semiorepo://definition/semio-repo/cli/main.go/ContributorRemove)

ContributorRemove performs the contributor remove operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️syncmanagement](semiorepo://definition/semio-repo/cli/main.go/SyncManagement)

SyncManagement performs the sync github operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️getrootdir](semiorepo://definition/semio-repo/cli/main.go/GetRootDir)

GetRootDir retrieves and returns the root dir.

## [🧰semiorepo⌨️cli💻maingo🛠️getbundles](semiorepo://definition/semio-repo/cli/main.go/GetBundles)

GetBundles retrieves and returns the bundles.

## [🧰semiorepo⌨️cli💻maingo🛠️gettechnologies](semiorepo://definition/semio-repo/cli/main.go/GetTechnologies)

GetTechnologies retrieves and returns the technologies.

## [🧰semiorepo⌨️cli💻maingo🛠️getcommits](semiorepo://definition/semio-repo/cli/main.go/GetCommits)

GetCommits retrieves and returns the commits.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolders](semiorepo://definition/semio-repo/cli/main.go/GetFolders)

GetFolders retrieves and returns the folders.

## [🧰semiorepo⌨️cli💻maingo🛠️getfiles](semiorepo://definition/semio-repo/cli/main.go/GetFiles)

GetFiles retrieves and returns the files.

## [🧰semiorepo⌨️cli💻maingo🛠️getdefinitions](semiorepo://definition/semio-repo/cli/main.go/GetDefinitions)

GetDefinitions retrieves and returns the definitions.

## [🧰semiorepo⌨️cli💻maingo🛠️getsections](semiorepo://definition/semio-repo/cli/main.go/GetSections)

GetSections retrieves and returns the sections.

## [🧰semiorepo⌨️cli💻maingo🛠️getcontributors](semiorepo://definition/semio-repo/cli/main.go/GetContributors)

GetContributors retrieves and returns the contributors.

## [🧰semiorepo⌨️cli💻maingo🛠️gettickets](semiorepo://definition/semio-repo/cli/main.go/GetTickets)

GetTickets retrieves and returns the tickets.

## [🧰semiorepo⌨️cli💻maingo🛠️getpolicies](semiorepo://definition/semio-repo/cli/main.go/GetPolicies)

GetPolicies retrieves and returns the policies.

## [🧰semiorepo⌨️cli💻maingo🛠️getstatutes](semiorepo://definition/semio-repo/cli/main.go/GetStatutes)

GetStatutes retrieves and returns the statutes.

## [🧰semiorepo⌨️cli💻maingo🛠️analyze](semiorepo://definition/semio-repo/cli/main.go/Analyze)

Analyze performs the analyze operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️fix](semiorepo://definition/semio-repo/cli/main.go/Fix)

Fix performs the fix operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketopen](semiorepo://definition/semio-repo/cli/main.go/TicketOpen)

TicketOpen performs the ticket open operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketprogress](semiorepo://definition/semio-repo/cli/main.go/TicketProgress)

TicketProgress performs the ticket progress operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketclose](semiorepo://definition/semio-repo/cli/main.go/TicketClose)

TicketClose performs the ticket close operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketreopen](semiorepo://definition/semio-repo/cli/main.go/TicketReopen)

TicketReopen performs the ticket reopen operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketchange](semiorepo://definition/semio-repo/cli/main.go/TicketChange)

TicketChange performs the ticket change operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️foldercreate](semiorepo://definition/semio-repo/cli/main.go/FolderCreate)

FolderCreate performs the folder create operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️foldermove](semiorepo://definition/semio-repo/cli/main.go/FolderMove)

FolderMove performs the folder move operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️folderdelete](semiorepo://definition/semio-repo/cli/main.go/FolderDelete)

FolderDelete performs the folder delete operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️filecreate](semiorepo://definition/semio-repo/cli/main.go/FileCreate)

FileCreate performs the file create operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️filemove](semiorepo://definition/semio-repo/cli/main.go/FileMove)

FileMove performs the file move operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️filedelete](semiorepo://definition/semio-repo/cli/main.go/FileDelete)

FileDelete performs the file delete operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️sectioncreate](semiorepo://definition/semio-repo/cli/main.go/SectionCreate)

SectionCreate performs the section create operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionmove](semiorepo://definition/semio-repo/cli/main.go/SectionMove)

SectionMove performs the section move operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️sectiondelete](semiorepo://definition/semio-repo/cli/main.go/SectionDelete)

SectionDelete performs the section delete operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️integrate](semiorepo://definition/semio-repo/cli/main.go/Integrate)

Integrate performs the integrate operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️extract](semiorepo://definition/semio-repo/cli/main.go/Extract)

Extract extracts the extract from the source.

## [🧰semiorepo⌨️cli💻maingo🛠️contributoradd](semiorepo://definition/semio-repo/cli/main.go/ContributorAdd)

ContributorAdd performs the contributor add operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️contributorremove](semiorepo://definition/semio-repo/cli/main.go/ContributorRemove)

ContributorRemove performs the contributor remove operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️syncmanagement](semiorepo://definition/semio-repo/cli/main.go/SyncManagement)

SyncManagement performs the sync github operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️getgoals](semiorepo://definition/semio-repo/cli/main.go/GetGoals)

GetGoals retrieves and returns the goals.

## [🧰semiorepo⌨️cli💻maingo🛠️goalcreate](semiorepo://definition/semio-repo/cli/main.go/GoalCreate)

GoalCreate performs the goal create operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️goalchange](semiorepo://definition/semio-repo/cli/main.go/GoalChange)

GoalChange performs the goal change operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️goalclose](semiorepo://definition/semio-repo/cli/main.go/GoalClose)

GoalClose performs the goal close operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️goalreopen](semiorepo://definition/semio-repo/cli/main.go/GoalReopen)

GoalReopen performs the goal reopen operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️goaldelete](semiorepo://definition/semio-repo/cli/main.go/GoalDelete)

GoalDelete performs the goal delete operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketdelete](semiorepo://definition/semio-repo/cli/main.go/TicketDelete)

TicketDelete performs the ticket delete operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️getdrafts](semiorepo://definition/semio-repo/cli/main.go/GetDrafts)

GetDrafts retrieves and returns the drafts.

## [🧰semiorepo⌨️cli💻maingo🛠️draftcreate](semiorepo://definition/semio-repo/cli/main.go/DraftCreate)

DraftCreate performs the draft create operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️draftdelete](semiorepo://definition/semio-repo/cli/main.go/DraftDelete)

DraftDelete performs the draft delete operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️gettodos](semiorepo://definition/semio-repo/cli/main.go/GetTodos)

GetTodos retrieves and returns the todos.

## [🧰semiorepo⌨️cli💻maingo🛠️todocreate](semiorepo://definition/semio-repo/cli/main.go/TodoCreate)

TodoCreate performs the todo create operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️todochange](semiorepo://definition/semio-repo/cli/main.go/TodoChange)

TodoChange performs the todo change operation on the default context.

## [🧰semiorepo⌨️cli💻maingo🛠️tododelete](semiorepo://definition/semio-repo/cli/main.go/TodoDelete)

TodoDelete performs the todo delete operation on the default context.

## [🧰semiorepo⌨️cli💻maingo✂️executor](semiorepo://definition/semio-repo/cli/main.go/Executor)

Executor holds the data fields for a executor record.

## [🧰semiorepo⌨️cli💻maingo🛠️newexecutor](semiorepo://definition/semio-repo/cli/main.go/NewExecutor)

NewExecutor creates and returns a new executor instance.

## [🧰semiorepo⌨️cli💻maingo🛠️newexecutorwithcontext](semiorepo://definition/semio-repo/cli/main.go/NewExecutorWithContext)

NewExecutorWithContext creates and returns a new executor with context instance.

## [🧰semiorepo⌨️cli💻maingo🛠️execute](semiorepo://definition/semio-repo/cli/main.go/Execute)

Execute executes the ute operation.

## [🧰semiorepo⌨️cli💻maingo🛠️executejson](semiorepo://definition/semio-repo/cli/main.go/ExecuteJSON)

ExecuteJSON executes the ute j s o n operation.

## [🧰semiorepo⌨️cli💻maingo🛠️validatequery](semiorepo://definition/semio-repo/cli/main.go/ValidateQuery)

ValidateQuery checks the query for correctness and returns any errors.

## [🧰semiorepo⌨️cli💻maingo🛠️getoperationtype](semiorepo://definition/semio-repo/cli/main.go/GetOperationType)

GetOperationType retrieves and returns the operation type.

## [🧰semiorepo⌨️cli💻maingo🛠️query](semiorepo://definition/semio-repo/cli/main.go/Query)

Query executes the query query.

## [🧰semiorepo⌨️cli💻maingo🛠️drafts](semiorepo://definition/semio-repo/cli/main.go/Drafts)

Drafts performs the drafts operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️node](semiorepo://definition/semio-repo/cli/main.go/Node)

Node performs the node operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️repo](semiorepo://definition/semio-repo/cli/main.go/Repo)

Repo performs the repo operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️technologies](semiorepo://definition/semio-repo/cli/main.go/Technologies)

Technologies performs the technologies operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️technology](semiorepo://definition/semio-repo/cli/main.go/Technology)

Technology performs the technology operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️bundles](semiorepo://definition/semio-repo/cli/main.go/Bundles)

Bundles performs the bundles operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️folders](semiorepo://definition/semio-repo/cli/main.go/Folders)

Folders performs the folders operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️files](semiorepo://definition/semio-repo/cli/main.go/Files)

Files performs the files operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️sections](semiorepo://definition/semio-repo/cli/main.go/Sections)

Sections performs the sections operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️definitions](semiorepo://definition/semio-repo/cli/main.go/Definitions)

Definitions performs the definitions operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️contributors](semiorepo://definition/semio-repo/cli/main.go/Contributors)

Contributors performs the contributors operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️todos](semiorepo://definition/semio-repo/cli/main.go/Todos)

Todos performs the todos operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️tickets](semiorepo://definition/semio-repo/cli/main.go/Tickets)

Tickets performs the tickets operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️interactions](semiorepo://definition/semio-repo/cli/main.go/Interactions)

Interactions aggregates all interactions from tickets and goals.

## [🧰semiorepo⌨️cli💻maingo🛠️policies](semiorepo://definition/semio-repo/cli/main.go/Policies)

Policies performs the policies operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️statutes](semiorepo://definition/semio-repo/cli/main.go/Statutes)

Statutes performs the statutes operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️breachs](semiorepo://definition/semio-repo/cli/main.go/Breachs)

Breachs performs the breachs operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️bundle](semiorepo://definition/semio-repo/cli/main.go/Bundle)

Bundle performs the bundle operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️folder](semiorepo://definition/semio-repo/cli/main.go/Folder)

Folder performs the folder operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️file](semiorepo://definition/semio-repo/cli/main.go/File)

File performs the file operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️section](semiorepo://definition/semio-repo/cli/main.go/Section)

Section performs the section operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️definition](semiorepo://definition/semio-repo/cli/main.go/Definition)

Definition performs the definition operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️contributor](semiorepo://definition/semio-repo/cli/main.go/Contributor)

Contributor performs the contributor operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️ticket](semiorepo://definition/semio-repo/cli/main.go/Ticket)

Ticket performs the ticket operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️policy](semiorepo://definition/semio-repo/cli/main.go/Policy)

Policy performs the policy operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️statute](semiorepo://definition/semio-repo/cli/main.go/Statute)

Statute performs the statute operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️analyze](semiorepo://definition/semio-repo/cli/main.go/Analyze)

Analyze performs the analyze operation on the query resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️mutation](semiorepo://definition/semio-repo/cli/main.go/Mutation)

Mutation performs the mutation operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️syncmanagement](semiorepo://definition/semio-repo/cli/main.go/SyncManagement)

SyncManagement performs the sync github operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️fix](semiorepo://definition/semio-repo/cli/main.go/Fix)

Fix performs the fix operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️draftcreate](semiorepo://definition/semio-repo/cli/main.go/DraftCreate)

DraftCreate performs the draft create operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️draftdelete](semiorepo://definition/semio-repo/cli/main.go/DraftDelete)

DraftDelete performs the draft delete operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketopen](semiorepo://definition/semio-repo/cli/main.go/TicketOpen)

TicketOpen performs the ticket open operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketclose](semiorepo://definition/semio-repo/cli/main.go/TicketClose)

TicketClose performs the ticket close operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketreopen](semiorepo://definition/semio-repo/cli/main.go/TicketReopen)

TicketReopen performs the ticket reopen operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketchange](semiorepo://definition/semio-repo/cli/main.go/TicketChange)

TicketChange performs the ticket change operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️goalcreate](semiorepo://definition/semio-repo/cli/main.go/GoalCreate)

GoalCreate performs the goal create operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️goalchange](semiorepo://definition/semio-repo/cli/main.go/GoalChange)

GoalChange performs the goal change operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️goalclose](semiorepo://definition/semio-repo/cli/main.go/GoalClose)

GoalClose performs the goal close operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️todocreate](semiorepo://definition/semio-repo/cli/main.go/TodoCreate)

TodoCreate performs the todo create operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️todochange](semiorepo://definition/semio-repo/cli/main.go/TodoChange)

TodoChange performs the todo change operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️tododelete](semiorepo://definition/semio-repo/cli/main.go/TodoDelete)

TodoDelete performs the todo delete operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️goalreopen](semiorepo://definition/semio-repo/cli/main.go/GoalReopen)

GoalReopen performs the goal reopen operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketprogress](semiorepo://definition/semio-repo/cli/main.go/TicketProgress)

TicketProgress performs the ticket progress operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️goaldelete](semiorepo://definition/semio-repo/cli/main.go/GoalDelete)

GoalDelete performs the goal delete operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketdelete](semiorepo://definition/semio-repo/cli/main.go/TicketDelete)

TicketDelete performs the ticket delete operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️contributoradd](semiorepo://definition/semio-repo/cli/main.go/ContributorAdd)

ContributorAdd performs the contributor add operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️contributorremove](semiorepo://definition/semio-repo/cli/main.go/ContributorRemove)

ContributorRemove performs the contributor remove operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️foldercreate](semiorepo://definition/semio-repo/cli/main.go/FolderCreate)

FolderCreate performs the folder create operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️foldermove](semiorepo://definition/semio-repo/cli/main.go/FolderMove)

FolderMove performs the folder move operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️folderdelete](semiorepo://definition/semio-repo/cli/main.go/FolderDelete)

FolderDelete performs the folder delete operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️filecreate](semiorepo://definition/semio-repo/cli/main.go/FileCreate)

FileCreate performs the file create operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️filemove](semiorepo://definition/semio-repo/cli/main.go/FileMove)

FileMove performs the file move operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️filedelete](semiorepo://definition/semio-repo/cli/main.go/FileDelete)

FileDelete performs the file delete operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️sectioncreate](semiorepo://definition/semio-repo/cli/main.go/SectionCreate)

SectionCreate performs the section create operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionmove](semiorepo://definition/semio-repo/cli/main.go/SectionMove)

SectionMove performs the section move operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️sectiondelete](semiorepo://definition/semio-repo/cli/main.go/SectionDelete)

SectionDelete performs the section delete operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️integrate](semiorepo://definition/semio-repo/cli/main.go/Integrate)

Integrate performs the integrate operation on the mutation resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️extract](semiorepo://definition/semio-repo/cli/main.go/Extract)

Extract extracts the extract from the source.

## [🧰semiorepo⌨️cli💻maingo🛠️repo](semiorepo://definition/semio-repo/cli/main.go/Repo_)

Repo* performs the repo* operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️bundles](semiorepo://definition/semio-repo/cli/main.go/Bundles)

Bundles performs the bundles operation on the repo resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️folders](semiorepo://definition/semio-repo/cli/main.go/Folders)

Folders performs the folders operation on the repo resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️files](semiorepo://definition/semio-repo/cli/main.go/Files)

Files performs the files operation on the repo resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️sections](semiorepo://definition/semio-repo/cli/main.go/Sections)

Sections performs the sections operation on the repo resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️definitions](semiorepo://definition/semio-repo/cli/main.go/Definitions)

Definitions performs the definitions operation on the repo resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️contributors](semiorepo://definition/semio-repo/cli/main.go/Contributors)

Contributors performs the contributors operation on the repo resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️todos](semiorepo://definition/semio-repo/cli/main.go/Todos)

Todos performs the todos operation on the repo resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️tickets](semiorepo://definition/semio-repo/cli/main.go/Tickets)

Tickets performs the tickets operation on the repo resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️policies](semiorepo://definition/semio-repo/cli/main.go/Policies)

Policies performs the policies operation on the repo resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️statutes](semiorepo://definition/semio-repo/cli/main.go/Statutes)

Statutes performs the statutes operation on the repo resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️breachs](semiorepo://definition/semio-repo/cli/main.go/Breachs)

Breachs performs the breachs operation on the repo resolver.

## [🧰semiorepo⌨️cli💻maingo✂️queryresolver](semiorepo://definition/semio-repo/cli/main.go/QueryResolver)

QueryResolver defines the interface for query resolver operations.

## [🧰semiorepo⌨️cli💻maingo✂️mutationresolver](semiorepo://definition/semio-repo/cli/main.go/MutationResolver)

MutationResolver defines the interface for mutation resolver operations.

## [🧰semiorepo⌨️cli💻maingo✂️reporesolver](semiorepo://definition/semio-repo/cli/main.go/RepoResolver)

RepoResolver defines the interface for repo resolver operations.

## [🧰semiorepo⌨️cli💻maingo🛠️getargs](semiorepo://definition/semio-repo/cli/main.go/getArgs)

Argument parsing utilities for CLI and MCP commands.

## [🧰semiorepo⌨️cli💻maingo🛠️requirefilepath](semiorepo://definition/semio-repo/cli/main.go/requireFilePath)

Path resolution utilities for file and folder operations.

## [🧰semiorepo⌨️cli💻maingo🛠️scopetofiles](semiorepo://definition/semio-repo/cli/main.go/ScopeToFiles)

ScopeToFiles performs the scope to files operation.

## [🧰semiorepo⌨️cli💻maingo🪨gitindexref](semiorepo://definition/semio-repo/cli/main.go/GitIndexRef)

GitIndexRef is the git ref for the staging index. Used for unstaged-only diffs (index vs working tree).
Specs: ticket close and interaction finish use only unstaged diffs; git diff runs without tree-ish for index vs working tree.

## [🧰semiorepo⌨️cli💻maingo🛠️computeticketfiles](semiorepo://definition/semio-repo/cli/main.go/ComputeTicketFiles)

ComputeTicketFiles computes and returns the ticket files.
Uses unstaged diffs only (index vs working tree) for complete, current working state.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitdifflines](semiorepo://definition/semio-repo/cli/main.go/GetGitDiffLines)

GetGitDiffLines retrieves and returns the git diff lines.
For unstaged-only diffs use baseCommit GitIndexRef (index vs working tree).

## [🧰semiorepo⌨️cli💻maingo🛠️cancloseticket](semiorepo://definition/semio-repo/cli/main.go/CanCloseTicket)

CanCloseTicket performs the can close ticket operation.

## [🧰semiorepo⌨️cli💻maingo🛠️getbundlebypath](semiorepo://definition/semio-repo/cli/main.go/GetBundleByPath)

GetBundleByPath retrieves and returns the bundle by path.

## [🧰semiorepo⌨️cli💻maingo🛠️guesssectionname](semiorepo://definition/semio-repo/cli/main.go/GuessSectionName)

GuessSectionName performs the guess section name operation.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitdiffsectionlinemetrics](semiorepo://definition/semio-repo/cli/main.go/GetGitDiffSectionLineMetrics)

GetGitDiffSectionLineMetrics retrieves and returns the git diff section line metrics.

## [🧰semiorepo⌨️cli💻maingo🛠️flattensections](semiorepo://definition/semio-repo/cli/main.go/FlattenSections)

FlattenSections flattens the nested sections into a single level.

## [🧰semiorepo⌨️cli💻maingo🛠️buildgitdiffargs](semiorepo://definition/semio-repo/cli/main.go/BuildGitDiffArgs)

BuildGitDiffArgs constructs and returns the git diff args.
GitIndexRef as baseCommit yields unstaged-only diff (index vs working tree) with no tree-ish.

## [🧰semiorepo⌨️cli💻maingo✂️gitdiffstatus](semiorepo://definition/semio-repo/cli/main.go/GitDiffStatus)

GitDiffStatus holds the data fields for a git diff status record.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitdiffstatus](semiorepo://definition/semio-repo/cli/main.go/GetGitDiffStatus)

GetGitDiffStatus retrieves and returns the git diff status.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolderchildren](semiorepo://definition/semio-repo/cli/main.go/GetFolderChildren)

GetFolderChildren retrieves and returns the folder children.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolderfiles](semiorepo://definition/semio-repo/cli/main.go/GetFolderFiles)

GetFolderFiles retrieves and returns the folder files.

## [🧰semiorepo⌨️cli💻maingo🛠️analyzefile](semiorepo://definition/semio-repo/cli/main.go/AnalyzeFile)

AnalyzeFile performs the analyze file operation.

## [🧰semiorepo⌨️cli💻maingo🛠️parsecontributoridentity](semiorepo://definition/semio-repo/cli/main.go/ParseContributorIdentity)

ParseContributorIdentity parses and returns the contributor identity from the input.

## [🧰semiorepo⌨️cli💻maingo🛠️listcontributors](semiorepo://definition/semio-repo/cli/main.go/ListContributors)

ListContributors returns a list of contributors entries.

## [🧰semiorepo⌨️cli💻maingo🛠️streamcontributors](semiorepo://definition/semio-repo/cli/main.go/StreamContributors)

StreamContributors streams contributors entries through the callback.

## [🧰semiorepo⌨️cli💻maingo🛠️getcontributoravatarpath](semiorepo://definition/semio-repo/cli/main.go/GetContributorAvatarPath)

GetContributorAvatarPath retrieves and returns the contributor avatar path.

## [🧰semiorepo⌨️cli💻maingo🛠️getcontributoravatarroundpath](semiorepo://definition/semio-repo/cli/main.go/GetContributorAvatarRoundPath)

GetContributorAvatarRoundPath retrieves and returns the contributor avatar round path.

## [🧰semiorepo⌨️cli💻maingo🛠️getcontributorpath](semiorepo://definition/semio-repo/cli/main.go/GetContributorPath)

GetContributorPath retrieves and returns the contributor path.

## [🧰semiorepo⌨️cli💻maingo🛠️createcontributor](semiorepo://definition/semio-repo/cli/main.go/CreateContributor)

CreateContributor creates a new contributor entry.

## [🧰semiorepo⌨️cli💻maingo🛠️loadcontributor](semiorepo://definition/semio-repo/cli/main.go/LoadContributor)

LoadContributor loads and returns contributor from the data source.

## [🧰semiorepo⌨️cli💻maingo🛠️savecontributor](semiorepo://definition/semio-repo/cli/main.go/SaveContributor)

SaveContributor persists contributor to the data store.

## [🧰semiorepo⌨️cli💻maingo🛠️removecontributor](semiorepo://definition/semio-repo/cli/main.go/RemoveContributor)

RemoveContributor removes the specified contributor.

## [🧰semiorepo⌨️cli💻maingo🛠️getregisteredpolicies](semiorepo://definition/semio-repo/cli/main.go/GetRegisteredPolicies)

GetRegisteredPolicies retrieves and returns the registered policies.

## [🧰semiorepo⌨️cli💻maingo🛠️bundles](semiorepo://definition/semio-repo/cli/main.go/Bundles)

Bundles performs the bundles operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️folders](semiorepo://definition/semio-repo/cli/main.go/Folders)

Folders performs the folders operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️files](semiorepo://definition/semio-repo/cli/main.go/Files)

Files performs the files operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️sections](semiorepo://definition/semio-repo/cli/main.go/Sections)

Sections performs the sections operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️definitions](semiorepo://definition/semio-repo/cli/main.go/Definitions)

Definitions performs the definitions operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️contributors](semiorepo://definition/semio-repo/cli/main.go/Contributors)

Contributors performs the contributors operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️policies](semiorepo://definition/semio-repo/cli/main.go/Policies)

Policies performs the policies operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️statutes](semiorepo://definition/semio-repo/cli/main.go/Statutes)

Statutes performs the statutes operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️breachs](semiorepo://definition/semio-repo/cli/main.go/Breachs)

Breachs performs the breachs operation on the resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️toolanalyze](semiorepo://definition/semio-repo/cli/main.go/ToolAnalyze)

ToolAnalyze performs the tool analyze operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfix](semiorepo://definition/semio-repo/cli/main.go/ToolFix)

ToolFix performs the tool fix operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolpolicylist](semiorepo://definition/semio-repo/cli/main.go/ToolPolicyList)

ToolPolicyList performs the tool policy list operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolpolicytree](semiorepo://definition/semio-repo/cli/main.go/ToolPolicyTree)

ToolPolicyTree performs the tool policy tree operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolpolicycheck](semiorepo://definition/semio-repo/cli/main.go/ToolPolicyCheck)

ToolPolicyCheck performs the tool policy check operation.

## [🧰semiorepo⌨️cli💻maingo🛠️toolpolicybreachlist](semiorepo://definition/semio-repo/cli/main.go/ToolPolicyBreachList)

ToolPolicyBreachList performs the tool policy breach list operation.

## [🧰semiorepo⌨️cli💻maingo✂️benchmarkresult](semiorepo://definition/semio-repo/cli/main.go/BenchmarkResult)

BenchmarkResult holds the data fields for a benchmark result record.

## [🧰semiorepo⌨️cli💻maingo✂️hookevent](semiorepo://definition/semio-repo/cli/main.go/HookEvent)

HookEvent represents a lifecycle event kind for hooks.

## [🧰semiorepo⌨️cli💻maingo🪨allhookevents](semiorepo://definition/semio-repo/cli/main.go/AllHookEvents)

AllHookEvents lists every valid hook event slug.

## [🧰semiorepo⌨️cli💻maingo✂️hookkind](semiorepo://definition/semio-repo/cli/main.go/HookKind)

HookKind categorizes a hook as either a git hook or an agent hook.

## [🧰semiorepo⌨️cli💻maingo🛠️hookeventkind](semiorepo://definition/semio-repo/cli/main.go/HookEventKind)

HookEventKind returns the hook kind for the given event.

## [🧰semiorepo⌨️cli💻maingo✂️hookcontext](semiorepo://definition/semio-repo/cli/main.go/HookContext)

HookContext carries event metadata and a codebase handle for hook handlers.

## [🧰semiorepo⌨️cli💻maingo✂️hookplanstep](semiorepo://definition/semio-repo/cli/main.go/HookPlanStep)

HookPlanStep represents a single step in a plan/task list update event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresult](semiorepo://definition/semio-repo/cli/main.go/HookResult)

HookResult represents the outcome of a hook invocation with event-specific data.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultbase](semiorepo://definition/semio-repo/cli/main.go/HookResultBase)

HookResultBase provides common fields for all hook results.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagentbase](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentBase)

HookResultAgentBase provides shared fields for all agent hook results.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagentstarted](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentStarted)

HookResultAgentStarted represents the result of an agent started event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagentended](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentEnded)

HookResultAgentEnded represents the result of an agent ended event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagentpromptsubmitting](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentPromptSubmitting)

HookResultAgentPromptSubmitting represents the result of an agent prompt submitting event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagentcompacting](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentCompacting)

HookResultAgentCompacting represents the result of an agent compacting event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagenttoolstarting](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentToolStarting)

HookResultAgentToolStarting represents the result of an agent tool starting event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagenttoolended](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentToolEnded)

HookResultAgentToolEnded represents the result of an agent tool ended event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagenttoolplanupdating](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentToolPlanUpdating)

HookResultAgentToolPlanUpdating represents the result of an agent tool plan updating event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagenttoolsearching](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentToolSearching)

HookResultAgentToolSearching represents the result of an agent tool searching event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagenttoolcodeediting](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentToolCodeEditing)

HookResultAgentToolCodeEditing represents the result of an agent tool code editing event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagenttoolcodeedited](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentToolCodeEdited)

HookResultAgentToolCodeEdited represents the result of an agent tool code edited event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagenttoolterminalstarting](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentToolTerminalStarting)

HookResultAgentToolTerminalStarting represents the result of an agent tool terminal starting event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultagenttoolterminalended](semiorepo://definition/semio-repo/cli/main.go/HookResultAgentToolTerminalEnded)

HookResultAgentToolTerminalEnded represents the result of an agent tool terminal ended event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultgitcommitstarting](semiorepo://definition/semio-repo/cli/main.go/HookResultGitCommitStarting)

HookResultGitCommitStarting represents the result of a git commit starting event.

## [🧰semiorepo⌨️cli💻maingo✂️hookresultgitcommitended](semiorepo://definition/semio-repo/cli/main.go/HookResultGitCommitEnded)

HookResultGitCommitEnded represents the result of a git commit ended event.

## [🧰semiorepo⌨️cli💻maingo✂️hooklogentry](semiorepo://definition/semio-repo/cli/main.go/HookLogEntry)

HookLogEntry pairs the invocation context with its result for audit logging.

## [🧰semiorepo⌨️cli💻maingo🪨shellsegmentre](semiorepo://definition/semio-repo/cli/main.go/shellSegmentRE)

shellSegmentRE splits a command string by common shell operators.

## [🧰semiorepo⌨️cli💻maingo🛠️splitcommandsegments](semiorepo://definition/semio-repo/cli/main.go/splitCommandSegments)

splitCommandSegments splits a shell command string by operators (&&, ||, ;, |) into individual segments.

## [🧰semiorepo⌨️cli💻maingo🛠️iscommandsegmentblocked](semiorepo://definition/semio-repo/cli/main.go/isCommandSegmentBlocked)

isCommandSegmentBlocked checks whether a single command segment starts with a blocked pattern.

## [🧰semiorepo⌨️cli💻maingo🛠️istoolblocked](semiorepo://definition/semio-repo/cli/main.go/IsToolBlocked)

IsToolBlocked checks whether a tool invocation matches a blocked pattern.
Uses segment-start matching: only blocks if a command segment STARTS WITH a blocked pattern,
preventing false positives from grep/echo commands that mention blocked patterns in arguments.

## [🧰semiorepo⌨️cli💻maingo🛠️extractcommandfromstdin](semiorepo://definition/semio-repo/cli/main.go/extractCommandFromStdin)

extractCommandFromStdin parses native client JSON from stdin and extracts the command string for blocking checks.

## [🧰semiorepo⌨️cli💻maingo🛠️extracttoolnamefromstdin](semiorepo://definition/semio-repo/cli/main.go/extractToolNameFromStdin)

extractToolNameFromStdin parses native client JSON from stdin and extracts the tool_name field.

## [🧰semiorepo⌨️cli💻maingo🛠️extracthookeventnamefromstdin](semiorepo://definition/semio-repo/cli/main.go/extractHookEventNameFromStdin)

extractHookEventNameFromStdin parses native client JSON from stdin and extracts the hookEventName field.

## [🧰semiorepo⌨️cli💻maingo✂️toolkind](semiorepo://definition/semio-repo/cli/main.go/ToolKind)

ToolKind classifies a tool invocation for routing native client events to neutral hook events.

## [🧰semiorepo⌨️cli💻maingo🛠️classifytool](semiorepo://definition/semio-repo/cli/main.go/classifyTool)

classifyTool categorizes a tool name into a ToolKind for inlet routing.

## [🧰semiorepo⌨️cli💻maingo🛠️resolvepretooluse](semiorepo://definition/semio-repo/cli/main.go/resolvePreToolUse)

resolvePreToolUse maps a ToolKind to the correct neutral event for pre-tool-use.

## [🧰semiorepo⌨️cli💻maingo🛠️resolveposttooluse](semiorepo://definition/semio-repo/cli/main.go/resolvePostToolUse)

resolvePostToolUse maps a ToolKind to the correct neutral event for post-tool-use.

## [🧰semiorepo⌨️cli💻maingo🛠️resolvecopilotevent](semiorepo://definition/semio-repo/cli/main.go/resolveCopilotEvent)

resolveCopilotEvent maps a VS Code / Copilot Chat native event to a neutral HookEvent.

## [🧰semiorepo⌨️cli💻maingo🛠️resolvecursorevent](semiorepo://definition/semio-repo/cli/main.go/resolveCursorEvent)

resolveCursorEvent maps a Cursor native event to a neutral HookEvent.

## [🧰semiorepo⌨️cli💻maingo🛠️resolvewindsurfevent](semiorepo://definition/semio-repo/cli/main.go/resolveWindsurfEvent)

resolveWindsurfEvent maps a Windsurf native event to a neutral HookEvent.

## [🧰semiorepo⌨️cli💻maingo🛠️resolveclaudecompatibleevent](semiorepo://definition/semio-repo/cli/main.go/resolveClaudeCompatibleEvent)

resolveClaudeCompatibleEvent maps a Claude Code / Droid / Codex / Antigravity native event to a neutral HookEvent.

## [🧰semiorepo⌨️cli💻maingo🛠️resolvehookevent](semiorepo://definition/semio-repo/cli/main.go/ResolveHookEvent)

ResolveHookEvent resolves an event string (neutral or native) to a neutral HookEvent.
Inlet adapter: native client events are resolved based on client and tool classification.

## [🧰semiorepo⌨️cli💻maingo🛠️vscodeeventfromhookevent](semiorepo://definition/semio-repo/cli/main.go/vsCodeEventFromHookEvent)

vsCodeEventFromHookEvent maps a neutral HookEvent back to the VS Code hookEventName (outlet adapter).

## [🧰semiorepo⌨️cli💻maingo🛠️formatvscodehookoutput](semiorepo://definition/semio-repo/cli/main.go/formatVSCodeHookOutput)

formatVSCodeHookOutput produces VS Code-compatible JSON output for hook results.

## [🧰semiorepo⌨️cli💻maingo🛠️loghook](semiorepo://definition/semio-repo/cli/main.go/logHook)

logHook writes the hook context and result to ./semio-repo/📜/YYMMDDHHMMSS_client_hook-kind.json.

## [🧰semiorepo⌨️cli💻maingo🛠️dispatchhook](semiorepo://definition/semio-repo/cli/main.go/dispatchHook)

dispatchHook routes the hook event to its handler and returns the specific result.

## [🧰semiorepo⌨️cli💻maingo🛠️runhook](semiorepo://definition/semio-repo/cli/main.go/RunHook)

RunHook executes the hook logic for the given context and logs the invocation.

## [🧰semiorepo⌨️cli💻maingo🛠️validatehookevent](semiorepo://definition/semio-repo/cli/main.go/ValidateHookEvent)

ValidateHookEvent checks if the given string is a valid hook event.

## [🧰semiorepo⌨️cli💻maingo🛠️hookcommand](semiorepo://definition/semio-repo/cli/main.go/hookCommand)

hookCommand creates the `hook <event> <client>` cobra command.

## [🧰semiorepo⌨️cli💻maingo✂️clienthookmapping](semiorepo://definition/semio-repo/cli/main.go/ClientHookMapping)

ClientHookMapping maps client names to their native event configuration format.

## [🧰semiorepo⌨️cli💻maingo🛠️configurecommand](semiorepo://definition/semio-repo/cli/main.go/configureCommand)

configureCommand creates the `configure` cobra command.

## [🧰semiorepo⌨️cli💻maingo✂️dependabotconfig](semiorepo://definition/semio-repo/cli/main.go/DependabotConfig)

DependabotConfig holds the data fields for a dependabot config record.

## [🧰semiorepo⌨️cli💻maingo✂️updateconfig](semiorepo://definition/semio-repo/cli/main.go/UpdateConfig)

UpdateConfig holds the data fields for a update config record.

## [🧰semiorepo⌨️cli💻maingo✂️constraint](semiorepo://definition/semio-repo/cli/main.go/Constraint)

Constraint holds the data fields for a constraint record.

## [🧰semiorepo⌨️cli💻maingo🛠️movefile](semiorepo://definition/semio-repo/cli/main.go/MoveFile)

MoveFile performs the move file operation.

## [🧰semiorepo⌨️cli💻maingo🛠️copyfile](semiorepo://definition/semio-repo/cli/main.go/CopyFile)

CopyFile performs the copy file operation.

## [🧰semiorepo⌨️cli💻maingo🛠️getrepogoalsdir](semiorepo://definition/semio-repo/cli/main.go/GetRepoGoalsDir)

GetRepoGoalsDir retrieves and returns the repo goals dir.

## [🧰semiorepo⌨️cli💻maingo🛠️listgoals](semiorepo://definition/semio-repo/cli/main.go/ListGoals)

ListGoals returns a list of goals entries.

## [🧰semiorepo⌨️cli💻maingo🛠️readgoal](semiorepo://definition/semio-repo/cli/main.go/ReadGoal)

ReadGoal reads and returns goal from the source.

## [🧰semiorepo⌨️cli💻maingo🛠️streamgoals](semiorepo://definition/semio-repo/cli/main.go/StreamGoals)

StreamGoals streams goals entries through the callback.

## [🧰semiorepo⌨️cli💻maingo🛠️streamstatutes](semiorepo://definition/semio-repo/cli/main.go/StreamStatutes)

StreamStatutes streams statutes entries through the callback.

## [🧰semiorepo⌨️cli💻maingo🛠️streamcommits](semiorepo://definition/semio-repo/cli/main.go/StreamCommits)

StreamCommits streams commits entries through the callback.

## [🧰semiorepo⌨️cli💻maingo🛠️savegoal](semiorepo://definition/semio-repo/cli/main.go/SaveGoal)

SaveGoal persists goal to the data store.

## [🧰semiorepo⌨️cli💻maingo🛠️resolvecontributorcontributions](semiorepo://definition/semio-repo/cli/main.go/ResolveContributorContributions)

ResolveContributorContributions resolves and returns the contributor contributions.

## [🧰semiorepo⌨️cli💻maingo🛠️gettodos](semiorepo://definition/semio-repo/cli/main.go/GetTodos)

GetTodos retrieves and returns the todos.

## [🧰semiorepo⌨️cli💻maingo🛠️scantodos](semiorepo://definition/semio-repo/cli/main.go/ScanTodos)

ScanTodos scans and collects todos from the input.

## [🧰semiorepo⌨️cli💻maingo🛠️parsetodomarkdown](semiorepo://definition/semio-repo/cli/main.go/ParseTodoMarkdown)

ParseTodoMarkdown parses and returns the todo markdown from the input.

## [🧰semiorepo⌨️cli💻maingo🛠️parsetodocomments](semiorepo://definition/semio-repo/cli/main.go/ParseTodoComments)

ParseTodoComments parses and returns the todo comments from the input.

## [🧰semiorepo⌨️cli💻maingo🛠️todocreate](semiorepo://definition/semio-repo/cli/main.go/TodoCreate)

TodoCreate performs the todo create operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️todochange](semiorepo://definition/semio-repo/cli/main.go/TodoChange)

TodoChange performs the todo change operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️tododelete](semiorepo://definition/semio-repo/cli/main.go/TodoDelete)

TodoDelete performs the todo delete operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️todototicket](semiorepo://definition/semio-repo/cli/main.go/TodoToTicket)

TodoToTicket performs the todo to ticket operation on the repo context.

## [🧰semiorepo⌨️cli💻maingo✂️semanticid](semiorepo://definition/semio-repo/cli/main.go/SemanticId)

SemanticId holds the data fields for a semantic id record.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String returns the string representation of the semantic id.

## [🧰semiorepo⌨️cli💻maingo✂️artifactref](semiorepo://definition/semio-repo/cli/main.go/ArtifactRef)

ArtifactRef holds the data fields for a artifact ref record.

## [🧰semiorepo⌨️cli💻maingo🛠️parseartifactref](semiorepo://definition/semio-repo/cli/main.go/ParseArtifactRef)

ParseArtifactRef parses and returns the artifact ref from the input.

## [🧰semiorepo⌨️cli💻maingo🛠️unslugify](semiorepo://definition/semio-repo/cli/main.go/UnSlugify)

UnSlugify performs the un slugify operation.

## [🧰semiorepo⌨️cli💻maingo🛠️findsectionbyslug](semiorepo://definition/semio-repo/cli/main.go/FindSectionBySlug)

FindSectionBySlug locates and returns the matching section by slug.

## [🧰semiorepo⌨️cli💻maingo🛠️resolvesectionname](semiorepo://definition/semio-repo/cli/main.go/ResolveSectionName)

ResolveSectionName resolves and returns the section name.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionidvaluetouripath](semiorepo://definition/semio-repo/cli/main.go/SectionIdValueToUriPath)

SectionIdValueToUriPath performs the section id value to uri path operation.

## [🧰semiorepo⌨️cli💻maingo🛠️definitionidvaluetouripath](semiorepo://definition/semio-repo/cli/main.go/DefinitionIdValueToUriPath)

DefinitionIdValueToUriPath performs the definition id value to uri path operation.

## [🧰semiorepo⌨️cli💻maingo🛠️parsesectionuripath](semiorepo://definition/semio-repo/cli/main.go/ParseSectionUriPath)

ParseSectionUriPath parses and returns the section uri path from the input.

## [🧰semiorepo⌨️cli💻maingo🛠️statuteidtouripath](semiorepo://definition/semio-repo/cli/main.go/StatuteIdToUriPath)

StatuteIdToUriPath performs the statute id to uri path operation.

## [🧰semiorepo⌨️cli💻maingo🛠️statuteuripathtoid](semiorepo://definition/semio-repo/cli/main.go/StatuteUriPathToId)

StatuteUriPathToId performs the statute uri path to id operation.

## [🧰semiorepo⌨️cli💻maingo🛠️getartifactid](semiorepo://definition/semio-repo/cli/main.go/GetArtifactID)

GetArtifactID retrieves and returns the artifact i d.

## [🧰semiorepo⌨️cli💻maingo🛠️getartifacturi](semiorepo://definition/semio-repo/cli/main.go/GetArtifactURI)

GetArtifactURI retrieves and returns the artifact u r i.

## [🧰semiorepo⌨️cli💻maingo🛠️idtouri](semiorepo://definition/semio-repo/cli/main.go/IdToUri)

IdToUri performs the id to uri operation.

## [🧰semiorepo⌨️cli💻maingo🛠️uritoid](semiorepo://definition/semio-repo/cli/main.go/UriToId)

UriToId performs the uri to id operation.

## [🧰semiorepo📚go💻emitgo](semiorepo://file/semio-repo/go/emit.go)

#region 🔖Header
[🧰semiorepo📚go💻emitgo](semiorepo://file/semio-repo/go/emit.go)
2025 Ueli Saluz <ueli@semio-tech.com>
GPL-3.0
Client helper to POST events to the semio-repo server.

## [🧰semiorepo📚go💻emitgo🛠️emit](semiorepo://definition/semio-repo/go/emit.go/Emit)

Emit posts an event to the semio-repo server. No-op when SEMIO_SERVER_ADDR is unset.

## [🧰semiorepo📚go💻eventsgo](semiorepo://file/semio-repo/go/events.go)

#region 🔖Header
[🧰semiorepo📚go💻eventsgo](semiorepo://file/semio-repo/go/events.go)
2025 Ueli Saluz <ueli@semio-tech.com>
GPL-3.0
Shared event kinds and payloads for CLI→server event-based communication.

## [🧰semiorepo📚go💻eventsgo🔖eventkind](semiorepo://section/EventKind)

EventKind identifies a changing interaction. CLI emits; server subscribes and notifies.

## [🧰semiorepo📚go💻eventsgo🔖event](semiorepo://section/Event)

Event is the canonical envelope for a changing interaction sent from CLI to server.

## [🧰semiorepo📚go💻eventsgo🔖payloads](semiorepo://section/Payloads)

TicketPayload holds common ticket identifiers.

## [🧰semiorepo📚go💻eventsgo✂️eventkind](semiorepo://definition/semio-repo/go/events.go/EventKind)

EventKind identifies a changing interaction. CLI emits; server subscribes and notifies.

## [🧰semiorepo📚go💻eventsgo✂️event](semiorepo://definition/semio-repo/go/events.go/Event)

Event is the canonical envelope for a changing interaction sent from CLI to server.

## [🧰semiorepo📚go💻eventsgo✂️ticketpayload](semiorepo://definition/semio-repo/go/events.go/TicketPayload)

TicketPayload holds common ticket identifiers.

## [🧰semiorepo📚go💻eventsgo✂️ticketopenpayload](semiorepo://definition/semio-repo/go/events.go/TicketOpenPayload)

TicketOpenPayload payload for ticket.open.

## [🧰semiorepo📚go💻eventsgo✂️ticketclosepayload](semiorepo://definition/semio-repo/go/events.go/TicketClosePayload)

TicketClosePayload payload for ticket.close.

## [🧰semiorepo📚go💻eventsgo✂️ticketreopenpayload](semiorepo://definition/semio-repo/go/events.go/TicketReopenPayload)

TicketReopenPayload payload for ticket.reopen.

## [🧰semiorepo📚go💻eventsgo✂️ticketchangepayload](semiorepo://definition/semio-repo/go/events.go/TicketChangePayload)

TicketChangePayload payload for ticket.change.

## [🧰semiorepo📚go💻eventsgo✂️goalpayload](semiorepo://definition/semio-repo/go/events.go/GoalPayload)

GoalPayload holds common goal identifiers.

## [🧰semiorepo📚go💻eventsgo✂️goalopenpayload](semiorepo://definition/semio-repo/go/events.go/GoalOpenPayload)

GoalOpenPayload payload for goal.open.

## [🧰semiorepo📚go💻eventsgo✂️goalclosepayload](semiorepo://definition/semio-repo/go/events.go/GoalClosePayload)

GoalClosePayload payload for goal.close.

## [🧰semiorepo📚go💻eventsgo✂️goalreopenpayload](semiorepo://definition/semio-repo/go/events.go/GoalReopenPayload)

GoalReopenPayload payload for goal.reopen.

## [🧰semiorepo📚go💻eventsgo✂️goalchangepayload](semiorepo://definition/semio-repo/go/events.go/GoalChangePayload)

GoalChangePayload payload for goal.change.

## [🧰semiorepo📚go💻eventsgo✂️contributorpayload](semiorepo://definition/semio-repo/go/events.go/ContributorPayload)

ContributorPayload holds contributor identifiers.

## [🧰semiorepo📚go💻eventsgo✂️commitpayload](semiorepo://definition/semio-repo/go/events.go/CommitPayload)

CommitPayload payload for commit (GitHub push).

## [🧰semiorepo📚go💻eventsgo✂️todopayload](semiorepo://definition/semio-repo/go/events.go/TodoPayload)

TodoPayload holds todo identifiers.

## [🧰semiorepo📚go💻eventsgo✂️todocreatepayload](semiorepo://definition/semio-repo/go/events.go/TodoCreatePayload)

TodoCreatePayload payload for todo.create.

## [🧰semiorepo📚go💻eventsgo✂️todochangepayload](semiorepo://definition/semio-repo/go/events.go/TodoChangePayload)

TodoChangePayload payload for todo.change.

## [🧰semiorepo📚go💻eventsgo✂️tododeletepayload](semiorepo://definition/semio-repo/go/events.go/TodoDeletePayload)

TodoDeletePayload payload for todo.delete.

## [🧰semiorepo📚go💻eventsgo✂️workitem](semiorepo://definition/semio-repo/go/events.go/WorkItem)

WorkItem represents a single item a contributor is working on (technology, bundle, folder, file, section, definition, ticket, goal, todo).

## [🧰semiorepo📚go💻eventsgo✂️contributorwork](semiorepo://definition/semio-repo/go/events.go/ContributorWork)

ContributorWork holds all work items for one contributor.

## [🧰semiorepo📚go💻eventsgo✂️draftpayload](semiorepo://definition/semio-repo/go/events.go/DraftPayload)

DraftPayload holds draft identifiers.

## [🧰semiorepo📚go💻eventsgo✂️filepayload](semiorepo://definition/semio-repo/go/events.go/FilePayload)

FilePayload holds file operation identifiers.

## [🧰semiorepo📚go💻eventsgo✂️folderpayload](semiorepo://definition/semio-repo/go/events.go/FolderPayload)

FolderPayload holds folder operation identifiers.

## [🧰semiorepo📚go💻eventsgo✂️sectionpayload](semiorepo://definition/semio-repo/go/events.go/SectionPayload)

SectionPayload holds section operation identifiers.

## [🧰semiorepo📚go💻eventsgo✂️integratepayload](semiorepo://definition/semio-repo/go/events.go/IntegratePayload)

IntegratePayload holds integrate operation identifiers.

## [🧰semiorepo📚go💻eventsgo✂️extractpayload](semiorepo://definition/semio-repo/go/events.go/ExtractPayload)

ExtractPayload holds extract operation identifiers.

## [🧰semiorepo🛂graphql💻schemagraphql](semiorepo://file/semio-repo/graphql/schema.graphql)

graphql/repo/schema.graphql

## [🧰semiorepo⌨️server💻maingo](semiorepo://file/semio-repo/server/main.go)

GraphQL server for the monorepo management API.

## [🧰semiorepo⌨️server💻maingo🔖config](semiorepo://section/Config)

Config holds all server configuration values.

## [🧰semiorepo⌨️server💻maingo🔖models](semiorepo://section/Models)

Ticket represents a tracked work item with lifecycle status.

## [🧰semiorepo⌨️server💻maingo🔖database](semiorepo://section/Database)

Database wraps a sql.DB connection to the SQLite store.

## [🧰semiorepo⌨️server💻maingo🔖eventbus](semiorepo://section/EventBus)

EventHandler is a callback invoked when an event of a subscribed type is published.

## [🧰semiorepo⌨️server💻maingo🔖diffparsing](semiorepo://section/DiffParsing)

hunkHeader is a regex pattern matching unified diff hunk headers.

## [🧰semiorepo⌨️server💻maingo🔖indexing](semiorepo://section/Indexing)

IndexCache holds in-memory caches of indexed scopes partitioned by file path.

## [🧰semiorepo⌨️server💻maingo🔖claims](semiorepo://section/Claims)

mapClaims maps diff hunks to overlapping scopes and returns claimed IDs.

## [🧰semiorepo⌨️server💻maingo🔖warnings](semiorepo://section/Warnings)

buildConflictWarnings creates warning records from detected scope conflicts.

## [🧰semiorepo⌨️server💻maingo🔖server](semiorepo://section/Server)

Server is the main HTTP server holding configuration, database, event bus, and caches.

## [🧰semiorepo⌨️server💻maingo🔖processing](semiorepo://section/Processing)

ProcessResult holds the outcome of a diff processing operation.

## [🧰semiorepo⌨️server💻maingo🔖webhooks](semiorepo://section/Webhooks)

GitHubComment stores a cached GitHub issue comment for correlating close/reopen events.

## [🧰semiorepo⌨️server💻maingo🔖discord](semiorepo://section/Discord)

notifyDiscord sends a message to the configured Discord webhook.

## [🧰semiorepo⌨️server💻maingo🔖utilities](semiorepo://section/Utilities)

newID generates a unique identifier from the current timestamp and a random value.

## [🧰semiorepo⌨️server💻maingo🔖main](semiorepo://section/Main)

main initializes the server and starts listening for HTTP requests.

## [🧰semiorepo⌨️server💻maingo✂️config](semiorepo://definition/semio-repo/server/main.go/Config)

Config holds all server configuration values.

## [🧰semiorepo⌨️server💻maingo🛠️loadconfig](semiorepo://definition/semio-repo/server/main.go/loadConfig)

loadConfig reads server configuration from environment variables with fallback defaults.

## [🧰semiorepo⌨️server💻maingo🛠️envordefault](semiorepo://definition/semio-repo/server/main.go/envOrDefault)

envOrDefault returns the environment variable value or the fallback if empty.

## [🧰semiorepo⌨️server💻maingo🛠️envordefaultint64](semiorepo://definition/semio-repo/server/main.go/envOrDefaultInt64)

envOrDefaultInt64 returns the parsed int64 environment variable or the fallback.

## [🧰semiorepo⌨️server💻maingo✂️ticket](semiorepo://definition/semio-repo/server/main.go/Ticket)

Ticket represents a tracked work item with lifecycle status.

## [🧰semiorepo⌨️server💻maingo✂️scope](semiorepo://definition/semio-repo/server/main.go/Scope)

Scope represents a code region (file, section, or definition) with line range.

## [🧰semiorepo⌨️server💻maingo✂️warning](semiorepo://definition/semio-repo/server/main.go/Warning)

Warning represents a detected issue such as a scope conflict between tickets.

## [🧰semiorepo⌨️server💻maingo✂️breach](semiorepo://definition/semio-repo/server/main.go/Breach)

Breach represents a policy breach detected in source code.

## [🧰semiorepo⌨️server💻maingo✂️event](semiorepo://definition/semio-repo/server/main.go/Event)

Event represents a system event persisted to the event log.

## [🧰semiorepo⌨️server💻maingo✂️linerange](semiorepo://definition/semio-repo/server/main.go/LineRange)

LineRange represents a contiguous range of line numbers.

## [🧰semiorepo⌨️server💻maingo✂️diffhunk](semiorepo://definition/semio-repo/server/main.go/DiffHunk)

DiffHunk represents a single hunk with old and new line ranges from a unified diff.

## [🧰semiorepo⌨️server💻maingo✂️difffile](semiorepo://definition/semio-repo/server/main.go/DiffFile)

DiffFile represents a single file entry in a unified diff with its hunks.

## [🧰semiorepo⌨️server💻maingo✂️diffresult](semiorepo://definition/semio-repo/server/main.go/DiffResult)

DiffResult aggregates all parsed diff files from a patch.

## [🧰semiorepo⌨️server💻maingo✂️filesnapshot](semiorepo://definition/semio-repo/server/main.go/FileSnapshot)

FileSnapshot holds the full content of a file for snapshot-based indexing.

## [🧰semiorepo⌨️server💻maingo✂️ticketopenrequest](semiorepo://definition/semio-repo/server/main.go/TicketOpenRequest)

TicketOpenRequest is the JSON payload for opening a new ticket.

## [🧰semiorepo⌨️server💻maingo✂️ticketcloserequest](semiorepo://definition/semio-repo/server/main.go/TicketCloseRequest)

TicketCloseRequest is the JSON payload for closing a ticket.

## [🧰semiorepo⌨️server💻maingo✂️ticketreopenrequest](semiorepo://definition/semio-repo/server/main.go/TicketReopenRequest)

TicketReopenRequest is the JSON payload for reopening a closed ticket.

## [🧰semiorepo⌨️server💻maingo✂️diffingestrequest](semiorepo://definition/semio-repo/server/main.go/DiffIngestRequest)

DiffIngestRequest is the JSON payload for ingesting a diff patch.

## [🧰semiorepo⌨️server💻maingo✂️diffingestresponse](semiorepo://definition/semio-repo/server/main.go/DiffIngestResponse)

DiffIngestResponse holds the results of a diff ingestion operation.

## [🧰semiorepo⌨️server💻maingo✂️precommitrequest](semiorepo://definition/semio-repo/server/main.go/PrecommitRequest)

PrecommitRequest is the JSON payload for a pre-commit check.

## [🧰semiorepo⌨️server💻maingo✂️precommitresponse](semiorepo://definition/semio-repo/server/main.go/PrecommitResponse)

PrecommitResponse holds the result of a pre-commit check.

## [🧰semiorepo⌨️server💻maingo✂️indexfilerequest](semiorepo://definition/semio-repo/server/main.go/IndexFileRequest)

IndexFileRequest is the JSON payload for indexing a single file.

## [🧰semiorepo⌨️server💻maingo✂️database](semiorepo://definition/semio-repo/server/main.go/Database)

Database wraps a sql.DB connection to the SQLite store.

## [🧰semiorepo⌨️server💻maingo🛠️opendatabase](semiorepo://definition/semio-repo/server/main.go/openDatabase)

openDatabase opens an SQLite database and runs schema migrations.

## [🧰semiorepo⌨️server💻maingo🛠️migrate](semiorepo://definition/semio-repo/server/main.go/migrate)

migrate creates database tables if they do not already exist.

## [🧰semiorepo⌨️server💻maingo🛠️close](semiorepo://definition/semio-repo/server/main.go/Close)

Close closes the underlying SQL database connection.

## [🧰semiorepo⌨️server💻maingo🛠️insertevent](semiorepo://definition/semio-repo/server/main.go/insertEvent)

insertEvent persists a new event record.

## [🧰semiorepo⌨️server💻maingo🛠️upsertticket](semiorepo://definition/semio-repo/server/main.go/upsertTicket)

upsertTicket inserts or updates a ticket record.

## [🧰semiorepo⌨️server💻maingo🛠️listtickets](semiorepo://definition/semio-repo/server/main.go/listTickets)

listTickets queries tickets optionally filtered by status.

## [🧰semiorepo⌨️server💻maingo🛠️getticket](semiorepo://definition/semio-repo/server/main.go/getTicket)

getTicket retrieves a single ticket by ID.

## [🧰semiorepo⌨️server💻maingo🛠️replacescopes](semiorepo://definition/semio-repo/server/main.go/replaceScopes)

replaceScopes deletes existing scopes for the file and inserts the new ones.

## [🧰semiorepo⌨️server💻maingo🛠️listscopesbyfile](semiorepo://definition/semio-repo/server/main.go/listScopesByFile)

listScopesByFile retrieves all scopes for a given file path.

## [🧰semiorepo⌨️server💻maingo🛠️upsertclaim](semiorepo://definition/semio-repo/server/main.go/upsertClaim)

upsertClaim inserts or updates a ticket-scope claim record.

## [🧰semiorepo⌨️server💻maingo🛠️listclaimsbyticket](semiorepo://definition/semio-repo/server/main.go/listClaimsByTicket)

listClaimsByTicket retrieves all scopes claimed by a ticket.

## [🧰semiorepo⌨️server💻maingo🛠️replacewarnings](semiorepo://definition/semio-repo/server/main.go/replaceWarnings)

replaceWarnings removes conflict warnings and inserts the new set.

## [🧰semiorepo⌨️server💻maingo🛠️listwarnings](semiorepo://definition/semio-repo/server/main.go/listWarnings)

listWarnings retrieves warnings optionally filtered by ticket ID.

## [🧰semiorepo⌨️server💻maingo🛠️listbreachs](semiorepo://definition/semio-repo/server/main.go/listBreachs)

listBreachs retrieves breachs optionally filtered by ticket ID.

## [🧰semiorepo⌨️server💻maingo🛠️listconflicts](semiorepo://definition/semio-repo/server/main.go/listConflicts)

listConflicts finds scopes claimed by more than one open ticket.

## [🧰semiorepo⌨️server💻maingo✂️eventhandler](semiorepo://definition/semio-repo/server/main.go/EventHandler)

EventHandler is a callback invoked when an event of a subscribed type is published.

## [🧰semiorepo⌨️server💻maingo✂️eventbus](semiorepo://definition/semio-repo/server/main.go/EventBus)

EventBus is a buffered channel-based event dispatcher with persistent storage.

## [🧰semiorepo⌨️server💻maingo🛠️neweventbus](semiorepo://definition/semio-repo/server/main.go/NewEventBus)

NewEventBus creates a new event bus backed by the given database.

## [🧰semiorepo⌨️server💻maingo🛠️subscribe](semiorepo://definition/semio-repo/server/main.go/Subscribe)

Subscribe registers a handler for the given event type.

## [🧰semiorepo⌨️server💻maingo🛠️publish](semiorepo://definition/semio-repo/server/main.go/Publish)

Publish persists an event and dispatches it to subscribers.

## [🧰semiorepo⌨️server💻maingo🛠️start](semiorepo://definition/semio-repo/server/main.go/Start)

Start launches the event dispatch goroutine.

## [🧰semiorepo⌨️server💻maingo🛠️stop](semiorepo://definition/semio-repo/server/main.go/Stop)

Stop cancels the event bus context and waits for the dispatch goroutine to finish.

## [🧰semiorepo⌨️server💻maingo🪨hunkheader](semiorepo://definition/semio-repo/server/main.go/hunkHeader)

hunkHeader is a regex pattern matching unified diff hunk headers.

## [🧰semiorepo⌨️server💻maingo🛠️parseunifieddiff](semiorepo://definition/semio-repo/server/main.go/parseUnifiedDiff)

parseUnifiedDiff extracts file paths and hunk ranges from a unified diff patch.

## [🧰semiorepo⌨️server💻maingo🛠️parsehunkint](semiorepo://definition/semio-repo/server/main.go/parseHunkInt)

parseHunkInt parses a hunk header integer value.

## [🧰semiorepo⌨️server💻maingo🛠️parsehunkintwithdefault](semiorepo://definition/semio-repo/server/main.go/parseHunkIntWithDefault)

parseHunkIntWithDefault parses a hunk header integer or returns the fallback.

## [🧰semiorepo⌨️server💻maingo✂️indexcache](semiorepo://definition/semio-repo/server/main.go/IndexCache)

IndexCache holds in-memory caches of indexed scopes partitioned by file path.

## [🧰semiorepo⌨️server💻maingo🛠️newindexcache](semiorepo://definition/semio-repo/server/main.go/newIndexCache)

newIndexCache creates an empty IndexCache with initialized maps.

## [🧰semiorepo⌨️server💻maingo🛠️buildscopesforfile](semiorepo://definition/semio-repo/server/main.go/buildScopesForFile)

buildScopesForFile parses a file into file, section, and definition scopes.

## [🧰semiorepo⌨️server💻maingo🛠️parsesections](semiorepo://definition/semio-repo/server/main.go/parseSections)

parseSections extracts section scopes from region markers and markdown headings.

## [🧰semiorepo⌨️server💻maingo🛠️parseregionmarker](semiorepo://definition/semio-repo/server/main.go/parseRegionMarker)

parseRegionMarker detects region start/end markers in a line.

## [🧰semiorepo⌨️server💻maingo🛠️parsemarkdownheading](semiorepo://definition/semio-repo/server/main.go/parseMarkdownHeading)

parseMarkdownHeading parses a markdown heading line into level and title.

## [🧰semiorepo⌨️server💻maingo🛠️assignsectionpaths](semiorepo://definition/semio-repo/server/main.go/assignSectionPaths)

assignSectionPaths updates section IDs to include the file path.

## [🧰semiorepo⌨️server💻maingo🛠️parsedefinitions](semiorepo://definition/semio-repo/server/main.go/parseDefinitions)

parseDefinitions extracts definition scopes using language-specific patterns.

## [🧰semiorepo⌨️server💻maingo🛠️definitionpatterns](semiorepo://definition/semio-repo/server/main.go/definitionPatterns)

definitionPatterns returns language-specific regex patterns for extracting definitions.

## [🧰semiorepo⌨️server💻maingo🛠️mapclaims](semiorepo://definition/semio-repo/server/main.go/mapClaims)

mapClaims maps diff hunks to overlapping scopes and returns claimed IDs.

## [🧰semiorepo⌨️server💻maingo🛠️filterscopesbyfile](semiorepo://definition/semio-repo/server/main.go/filterScopesByFile)

filterScopesByFile returns scopes matching the given file path.

## [🧰semiorepo⌨️server💻maingo🛠️rangesoverlap](semiorepo://definition/semio-repo/server/main.go/rangesOverlap)

rangesOverlap tests whether two line ranges overlap.

## [🧰semiorepo⌨️server💻maingo🛠️appendifmissing](semiorepo://definition/semio-repo/server/main.go/appendIfMissing)

appendIfMissing appends a string to a slice only if it is not already present.

## [🧰semiorepo⌨️server💻maingo🛠️buildconflictwarnings](semiorepo://definition/semio-repo/server/main.go/buildConflictWarnings)

buildConflictWarnings creates warning records from detected scope conflicts.

## [🧰semiorepo⌨️server💻maingo✂️server](semiorepo://definition/semio-repo/server/main.go/Server)

Server is the main HTTP server holding configuration, database, event bus, and caches.

## [🧰semiorepo⌨️server💻maingo🛠️newserver](semiorepo://definition/semio-repo/server/main.go/NewServer)

NewServer creates a new Server with the given config, database, and event bus.

## [🧰semiorepo⌨️server💻maingo🛠️newrequestcontext](semiorepo://definition/semio-repo/server/main.go/newRequestContext)

newRequestContext creates a request-scoped context with a 15-second timeout.

## [🧰semiorepo⌨️server💻maingo🛠️requireauth](semiorepo://definition/semio-repo/server/main.go/requireAuth)

requireAuth checks the bearer token against the configured server token.

## [🧰semiorepo⌨️server💻maingo🛠️decodejson](semiorepo://definition/semio-repo/server/main.go/decodeJSON)

decodeJSON reads and decodes a JSON request body with size limits.

## [🧰semiorepo⌨️server💻maingo🛠️writejson](semiorepo://definition/semio-repo/server/main.go/writeJSON)

writeJSON writes a JSON response with the given status code.

## [🧰semiorepo⌨️server💻maingo🛠️responderror](semiorepo://definition/semio-repo/server/main.go/respondError)

respondError writes a JSON error response.

## [🧰semiorepo⌨️server💻maingo🛠️handleevents](semiorepo://definition/semio-repo/server/main.go/handleEvents)

handleEvents accepts CLI event payloads and persists/publishes them.

## [🧰semiorepo⌨️server💻maingo🛠️handlehealth](semiorepo://definition/semio-repo/server/main.go/handleHealth)

handleHealth responds with 200 OK for liveness checks.

## [🧰semiorepo⌨️server💻maingo🛠️handleticketopen](semiorepo://definition/semio-repo/server/main.go/handleTicketOpen)

handleTicketOpen creates a new ticket from the request payload.

## [🧰semiorepo⌨️server💻maingo🛠️handleticketclose](semiorepo://definition/semio-repo/server/main.go/handleTicketClose)

handleTicketClose closes an existing ticket with a summary.

## [🧰semiorepo⌨️server💻maingo🛠️handleticketreopen](semiorepo://definition/semio-repo/server/main.go/handleTicketReopen)

handleTicketReopen reopens a closed ticket with a new prompt.

## [🧰semiorepo⌨️server💻maingo🛠️handleticketsquery](semiorepo://definition/semio-repo/server/main.go/handleTicketsQuery)

handleTicketsQuery lists tickets optionally filtered by status.

## [🧰semiorepo⌨️server💻maingo🛠️handleticketdetail](semiorepo://definition/semio-repo/server/main.go/handleTicketDetail)

handleTicketDetail returns a single ticket by its path-extracted ID.

## [🧰semiorepo⌨️server💻maingo🛠️handleticketclaims](semiorepo://definition/semio-repo/server/main.go/handleTicketClaims)

handleTicketClaims returns scope claims for a ticket.

## [🧰semiorepo⌨️server💻maingo🛠️handlediffingest](semiorepo://definition/semio-repo/server/main.go/handleDiffIngest)

handleDiffIngest ingests a diff patch, indexes changed files, maps claims, and returns results.

## [🧰semiorepo⌨️server💻maingo🛠️handleprecommit](semiorepo://definition/semio-repo/server/main.go/handlePrecommit)

handlePrecommit runs a pre-commit check against a diff patch.

## [🧰semiorepo⌨️server💻maingo🛠️handlereindex](semiorepo://definition/semio-repo/server/main.go/handleReindex)

handleReindex walks the repo and re-indexes all files.

## [🧰semiorepo⌨️server💻maingo🛠️handleindexfile](semiorepo://definition/semio-repo/server/main.go/handleIndexFile)

handleIndexFile indexes a single file from the request payload.

## [🧰semiorepo⌨️server💻maingo🛠️handlewarnings](semiorepo://definition/semio-repo/server/main.go/handleWarnings)

handleWarnings returns warnings optionally filtered by ticket ID.

## [🧰semiorepo⌨️server💻maingo🛠️handlebreachs](semiorepo://definition/semio-repo/server/main.go/handleBreachs)

handleBreachs returns breachs optionally filtered by ticket ID.

## [🧰semiorepo⌨️server💻maingo🛠️handlescopes](semiorepo://definition/semio-repo/server/main.go/handleScopes)

handleScopes returns scopes for a given file query parameter.

## [🧰semiorepo⌨️server💻maingo✂️processresult](semiorepo://definition/semio-repo/server/main.go/ProcessResult)

ProcessResult holds the outcome of a diff processing operation.

## [🧰semiorepo⌨️server💻maingo🛠️processdiff](semiorepo://definition/semio-repo/server/main.go/processDiff)

processDiff parses the patch, indexes changed files, maps claims, and detects conflicts.

## [🧰semiorepo⌨️server💻maingo🛠️uniquefiles](semiorepo://definition/semio-repo/server/main.go/uniqueFiles)

uniqueFiles extracts deduplicated file paths from a diff result.

## [🧰semiorepo⌨️server💻maingo🛠️snapshotmap](semiorepo://definition/semio-repo/server/main.go/snapshotMap)

snapshotMap converts a slice of file snapshots into a path-to-content map.

## [🧰semiorepo⌨️server💻maingo🛠️updateindexforfile](semiorepo://definition/semio-repo/server/main.go/updateIndexForFile)

updateIndexForFile builds scopes from file content and updates both the database and cache.

## [🧰semiorepo⌨️server💻maingo🛠️buildscopeid](semiorepo://definition/semio-repo/server/main.go/buildScopeID)

buildScopeID generates a deterministic scope ID from the scope's kind and path.

## [🧰semiorepo⌨️server💻maingo🛠️walkrepofiles](semiorepo://definition/semio-repo/server/main.go/walkRepoFiles)

walkRepoFiles walks the repo root and returns all non-hidden file paths.

## [🧰semiorepo⌨️server💻maingo✂️githubcomment](semiorepo://definition/semio-repo/server/main.go/GitHubComment)

GitHubComment stores a cached GitHub issue comment for correlating close/reopen events.

## [🧰semiorepo⌨️server💻maingo🛠️handlegithubwebhook](semiorepo://definition/semio-repo/server/main.go/handleGitHubWebhook)

handleGitHubWebhook processes incoming GitHub webhook events.

## [🧰semiorepo⌨️server💻maingo🛠️verifygithubsignature](semiorepo://definition/semio-repo/server/main.go/verifyGitHubSignature)

verifyGitHubSignature validates the HMAC-SHA256 signature of a webhook payload.

## [🧰semiorepo⌨️server💻maingo🛠️cachegithubcomment](semiorepo://definition/semio-repo/server/main.go/cacheGitHubComment)

cacheGitHubComment stores a GitHub comment for correlating subsequent events.

## [🧰semiorepo⌨️server💻maingo🛠️handlegithubissueevent](semiorepo://definition/semio-repo/server/main.go/handleGitHubIssueEvent)

handleGitHubIssueEvent processes GitHub issue close/reopen events.

## [🧰semiorepo⌨️server💻maingo🛠️findcachedcomment](semiorepo://definition/semio-repo/server/main.go/findCachedComment)

findCachedComment retrieves a recently cached GitHub comment for the given issue.

## [🧰semiorepo⌨️server💻maingo🛠️extractissuecomment](semiorepo://definition/semio-repo/server/main.go/extractIssueComment)

extractIssueComment extracts issue number, repo, actor, and body from a webhook payload.

## [🧰semiorepo⌨️server💻maingo🛠️extractissuenumber](semiorepo://definition/semio-repo/server/main.go/extractIssueNumber)

extractIssueNumber extracts the issue number from a GitHub webhook payload.

## [🧰semiorepo⌨️server💻maingo🛠️extractrepofullname](semiorepo://definition/semio-repo/server/main.go/extractRepoFullName)

extractRepoFullName extracts the repository full name from a GitHub webhook payload.

## [🧰semiorepo⌨️server💻maingo🛠️extractactorlogin](semiorepo://definition/semio-repo/server/main.go/extractActorLogin)

extractActorLogin extracts the sender login from a GitHub webhook payload.

## [🧰semiorepo⌨️server💻maingo🛠️notifydiscord](semiorepo://definition/semio-repo/server/main.go/notifyDiscord)

notifyDiscord sends a message to the configured Discord webhook.

## [🧰semiorepo⌨️server💻maingo🛠️registernotifications](semiorepo://definition/semio-repo/server/main.go/registerNotifications)

registerNotifications subscribes to ticket lifecycle events and sends Discord notifications.

## [🧰semiorepo⌨️server💻maingo🛠️newid](semiorepo://definition/semio-repo/server/main.go/newID)

newID generates a unique identifier from the current timestamp and a random value.

## [🧰semiorepo⌨️server💻maingo🛠️main](semiorepo://definition/semio-repo/server/main.go/main)

main initializes the server and starts listening for HTTP requests.

## [🧰semiorepo🛂sqlite💻schemasql](semiorepo://file/semio-repo/sqlite/schema.sql)

sql/sqlite/repo/schema.sql

## [🧰semiorepo🛂sqlite💻schemasql🛠️contributor](semiorepo://definition/semio-repo/sqlite/schema.sql/contributor)

#region 🔖Repo

## [🧰semiorepo🖱️vscode🗃️codegen💻fragmentmaskingts🛠️usefragmen](semiorepo://definition/semio-repo/vscode/codegen/fragment-masking.ts/useFragmen)

return nullable if `fragmentType` is undefined

## [🧰semiorepo🖱️vscode🗃️codegen💻fragmentmaskingts🛠️usefragment](semiorepo://definition/semio-repo/vscode/codegen/fragment-masking.ts/useFragment)

return nullable if `fragmentType` is nullable

## [🧰semiorepo🖱️vscode🗃️codegen💻fragmentmaskingts🛠️usefragment](semiorepo://definition/semio-repo/vscode/codegen/fragment-masking.ts/useFragment)

return nullable if `fragmentType` is nullable or undefined

## [🧰semiorepo🖱️vscode🗃️codegen💻fragmentmaskingts🛠️usefragment](semiorepo://definition/semio-repo/vscode/codegen/fragment-masking.ts/useFragment)

return array of non-nullable if `fragmentType` is array of non-nullable

## [🧰semiorepo🖱️vscode🗃️codegen💻fragmentmaskingts🛠️usefe](semiorepo://definition/semio-repo/vscode/codegen/fragment-masking.ts/useFe)

return readonly array of nullable if `fragmentType` is array of nullable

## [🧰semiorepo🖱️vscode💻codegents](semiorepo://file/semio-repo/vscode/codegen.ts)

Code generation script for VS Code extension GraphQL types.

## [🧰semiorepo🖱️vscode💻codegents🔖configuration](semiorepo://section/Configuration)

GraphQL code generation configuration for the VS Code extension.

## [🧰semiorepo🖱️vscode💻codegents🪨config](semiorepo://definition/semio-repo/vscode/codegen.ts/config)

GraphQL codegen configuration targeting the schema and query documents.

## [🧰semiorepo🖱️vscode💻extensionts](semiorepo://file/semio-repo/vscode/extension.ts)

VS Code extension providing monorepo navigation, analysis and commands.

## [🧰semiorepo🖱️vscode💻queriests](semiorepo://file/semio-repo/vscode/queries.ts)

GraphQL query document constants for the VS Code extension.
