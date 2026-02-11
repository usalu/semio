# Summary

Repo CLI implementation and tests for the Go-based `semio-repo` tooling entrypoint.

# Docs

## cli.go

Repo CLI single-file entrypoint with command registry, GraphQL execution, and three-tier hierarchical GitHub sync where root goals (depth 0) map to milestones, first-generation child goals (depth 1) map to issues with the `goal` label linked to the root milestone, and deeper goals (depth 2+) map to sub-issues of their parent goal's issue without milestone, with enforcement that existing goal issues always have the root milestone at depth 1, a parent sub-issue link and no milestone at depth 2+, and a restored `goal` label when missing. Ticket issues link to root goal milestones. Repository label catalog synchronization plus repository-wide `@` label cleanup for project/bundle tags, and emoji-normalized artifact IDs. Artifact kind derivation: `DeriveBundleKind` reads `bundleKind` from `package.json`/`project.json` at bundle root (fallback `library`); `DeriveFolderKind` classifies `.`-prefixed and manifest-containing folders as `required`; `DeriveFileKind` pattern-matches file names/extensions into `code`/`test`/`config`/`docs`/`resource`/`script`/`license`; `DeriveDefinitionKind` maps language keywords to `implementation`/`interface`/`constant` via `extractDefinitionKeyword` (word-before-name priority, modifier skipping) and `refineDefinitionKind` (arrow function detection). Dual identification system: `GetArtifactID` generates emoji-prefixed primary IDs (with kind-specific emoji helpers `projectKindEmoji`, `bundleKindEmoji`, `fileKindEmoji`, `folderKindEmoji`, `definitionKindEmoji`), `GetArtifactURI` generates `semiorepo://` secondary URIs with `SectionIdValueToUriPath`/`DefinitionIdValueToUriPath` for `#`/`§`→`/SLUG` encoding, `ParseSectionUriPath` for reverse URI→file+slugs parsing. `IdToUri`/`UriToId` bidirectional conversion with emoji normalization (strip `\uFE0E`/`\uFE0F` for matching). Collection types (projects, bundles, folders, files, sections, definitions, tickets, goals, drafts, todos, policies, violationKinds, contributors, commits) have dedicated ID and URI formats. `navigate` MCP tool resolves ID or URI input to both forms. MCP resource templates use slash-based paths for sections/definitions. Relative date rendering for tickets and goals supports RFC3339 and YYYY-MM-DD input formats via a flexible parser that uses the `dustin/go-humanize` package for human-friendly scheduling and history display. `--json` outputs pure data per line (no event wrappers, no `{"data": ...}` GraphQL envelope). Errors go to stderr. Cobra root has `SilenceUsage`/`SilenceErrors` to prevent stdout pollution on errors. Three rendering modes via `--format` flag: `NDJSONRenderer` (json), `HumanRenderer` (text), `MarkdownRenderer` (md, default). `inferEntityKind(key)` maps GraphQL operation names to entity kinds via prefix matching for generic mutation/query result dispatch. `renderEntityMarkdownLink(kind, data)` produces `[<id>](<uri>) - <prop1> - <prop2>` markdown for any entity kind. `renderEntityMarkdown(kind, data)` adds `- ` list prefix. `formatMarkdownResult` and `formatResult` use generic entity-kind dispatch with `inferEntityKind` fallback — no JSON code-block or dump fallback paths. Streaming list items detected via command suffix (`" list"`, `" tree"`) to apply correct `- ` markdown prefix. `repoContext.Fix()` detects violations via `CheckPoliciesWithContext`, filters autofixable ones, groups by file, and delegates to `applyAutofixes()` which applies per-violation-kind handlers bottom-up to avoid line number shifts: empty section removal with surrounding blank cleanup, missing/mismatched section end name resolution via `findMatchingSectionStartName()` stack-based walk, inline comment contiguous block removal with blank line tracking, block/JSDoc comment removal. Post-removal blank line collapse prevents double blank lines. `extractFileFromScope()` strips `#section` and `::definition` suffixes from scope strings. `FileHeaderId` generates emoji-prefixed file artifact IDs for headers using `DeriveFileKind` and `GetArtifactID`; for code files it reads the file content and overrides the kind to `script` when the first line starts with a shebang (`#!`). `ViolationCodeHeaderWrongFileId` (autofixable) replaces `ViolationCodeHeaderMissingFilename`; `headerPolicy` detects wrong/missing file IDs by scanning header lines for file extensions and emoji markers; `applyAutofixes` replaces the identified line with the correct artifact ID comment. `LanguagePlugin` interface with `BaseLanguage` base struct provides language-agnostic comment scanning via `ScanComments` using configurable primitives: `commentPrefix`, `blockCommentStart`/`blockCommentEnd`, `hasJSDoc`, `hasTemplates`, `hasRawBackticks`, `hasTripleQuotes`, `hasVerbatimStrings`, `skipDirectives`. `SkipDirectives()` merges built-in directives (`TODO`, `semio-ignore-`) with language-specific ones. `CommentScanState` tracks scanner state across lines (block comments, string literals, template expressions, triple quotes, raw backticks, verbatim strings, TODO blocks, escape sequences). Language constructors: TypeScript (`hasJSDoc`, `hasTemplates`, skip `eslint-`/`@ts-`/`noinspection`), Go (`hasRawBackticks`, skip `nolint`), Python (`hasTripleQuotes`, skip `noqa`/`type: ignore`/`pylint:`/`pragma:`), C# (`hasVerbatimStrings`, skip `pragma`). `ParseIgnoreDirectives` accepts language-aware `commentPrefix` parameter. `applyAutofixes` inline comment fix uses `PolicySectionStartMatch`/`PolicySectionEndMatch` and `SkipDirectives()` for language-aware region/directive detection. `specsPolicy` scans Header Specs regions and section-start spec comments for implementation-specific syntax (backtick-wrapped code, function call patterns) via `isSpecText` (RFC 2119 keyword detection) and `hasImplementationSyntax`; emits `ViolationCodeSpecsSyntax`. `SpecLines`/`IsSpecLine`/`IsSpecBlock` on `PolicyContext` cache spec line positions per file; `ScanComments` (both `BaseLanguage` and `TypeScriptLanguage`) exempt spec lines and spec blocks from inline/block/JSDoc comment violations.

## main.go

Go repo CLI/runtime implementation with custom `Interaction` JSON decoding that normalizes legacy object-form and current string-form `author` payloads into a single author string for ticket and goal interaction reads. Lifecycle status guards: `FinishTicket` rejects non-open tickets with "ticket is not open" before file/summary validation; `ReopenTicket` rejects already-open tickets with "ticket is already open"; `GoalClose` rejects already-closed goals with "goal is already closed"; `GoalReopen` rejects already-open goals with "goal is already open". MCP tool handlers call `Tool*` functions directly instead of GraphQL mutations to ensure consistent output between MCP and CLI. MCP resource handlers use validated GraphQL queries with correct field names (`range { start end }` for sections/definitions, `interactions` for tickets, `emails` for contributors, `id sha title date` for commits). `goalType` GraphQL object includes `uri` field resolved via `Goal.GetURI()`. `emojiText` strips variation selectors on all artifact ID emojis. `loadProjectsInternal` detects both `@`-prefixed and non-prefixed project directories, excluding hidden dirs and `node_modules`. `TreeNodeKind` enum and `TreeNode` struct model the complete monorepo hierarchy (project, bundle, folder, file, section, definition, goal, ticket, draft, policy, violationKind, contributor, commit, category). `TreeFilter` supports kind-level (`OnlyKinds`/`ExcludeKinds`), sub-kind-level (`OnlySubKinds`/`ExcludeSubKinds`), date (`OnlyYears`/`ExcludeYears`/months/days), status, and contributor filtering with case-insensitive matching. `BuildMonorepoTree` streams all data sources concurrently (projects, goals, tickets, drafts, policies, contributors, commits, folders, files) with a single filesystem walk, assigns folders/files to bundles by path prefix, and optionally parses sections via `TreeBuildOptions.IncludeSections`. `FilterMonorepoTree` recursively filters nodes and `collapseFilteredKinds` promotes children of excluded kinds to parent level. `SearchMonorepoTree` indexes all node attributes into a bleve in-memory index, performs fuzzy match queries, and `pruneUnmatched` preserves parent chains. `RenderMonorepoTree` outputs connector-based text trees and `RenderMonorepoTreeMarkdown` outputs nested markdown bullets; `treeCommand` selects JSON, markdown (default), or text rendering from `Config.Format`. `bindTreeFlags` registers `--only-<kind>`/`--no-<kind>` for 12 kinds, `--only-<subkind>`/`--no-<subkind>` for bundle/folder/file/definition sub-kinds, `--only-open`/`--only-closed`/`--open`/`--closed`, `--only-year`/`--no-year`/month/day int slices, and `--only-contributor-name`/`--no-contributor-name` string slices. `ArtifactRef` struct with `Kind` (file/folder/section), `Path`, and `SectionParts` fields parsed by `ParseArtifactRef` from emoji-prefixed artifact IDs (📁→folder, 💻/📄→file, 🔖→section with `#`-delimited slugs). `UnSlugify` converts `UPPER-KEBAB` slugs to title case. `FindSectionBySlug` case-insensitively matches section names against slugified forms. `ResolveSectionName` attempts slug-to-section resolution via file parsing, falling back to `UnSlugify`. `moveCommand` dispatches on source/target kind pairs: file→file, folder→folder, section→section (rename), file→section (integrate+delete+`RemoveAgentsDocsEntry`), section→file (extract). `integrateCommand` accepts artifact ID positional args or `--file`/`--target-file`/`--target-section`/`--parent-section` flags. `extractCommand` accepts artifact ID positional args or `--file`/`--section`/`--target-file` flags. `ToolProjectList` emits `Project` entities sourced from `LoadProjects` (sorted by name) for type-consistent project tool payloads. `ToolProjectTree` emits `Project` entities via `StreamProjects` through the `toolResultFromEvents` event pipeline, sorting events by `formatMarkdownResult` output for stable rendered-line ordering. `ToolBundleList` emits `Bundle` entities sourced from `LoadBundles` (sorted by name) for bundle-specific listing. `runProjectTree` sorts project events by `formatMarkdownResult` output to match `ToolProjectTree` ordering. `UpdateAgentsDocsPath` replaces old path prefixes with new ones in `AGENTS.md` `## ` headers under `# Codebase`. `RemoveAgentsDocsEntry` removes matching `## ` header and its content paragraph from `AGENTS.md`. `ToolFileMove`/`ToolFolderMove` call `UpdateAgentsDocsPath` after successful rename. MCP handlers `sectionExtract` and `artifactMove` expose extract and move functionality. `sanitizeProp` strips `\r\n`/`\n`/`\r`, replaces backticks with single quotes, collapses double spaces, and trims; used by `collectEntityProps` for all property values. `sanitizeSingleLine` strips `\r\n`/`\n`/`\r` as a final rendering safety net; used by `renderEntityMarkdownLink` and `renderEntityHuman` to guarantee single-line output.

## cli_test.go

Consolidated Go test suite for the repo CLI and tooling behavior, structured for fast/slow lane execution with explicit slow-test sharding for parallel CI distribution. `executeCommand` separates stdout/stderr with 3-return signature. Wrong-argument tests cover all command categories (ticket, goal, todo, policy, folder, file, section, definition, contributor, graphql, top-level move/extract/integrate). `TestCliWrongArgs_ErrorMessages` verifies expected error message content for 36 cases across all command categories. `TestCliJsonPureData` validates no event wrappers or `{"data": ...}` envelopes. `TestCliJsonErrorsToStderr` validates empty stdout on errors for 22 error cases. Wrong-lifecycle tests in `TestCliE2E_TicketLifecycle_Syntaxes_NoGithub` and `TestCliE2E_GoalLifecycle_Syntaxes_NoGithub` verify that closing already-closed tickets/goals and reopening already-open tickets/goals return correct error messages. Fix tests cover all autofixable violation kinds (empty section, missing end name, name mismatch, inline/block/JSDoc comments), nested sections, idempotency, multiple violations per file, non-autofixable detection, GraphQL mutation, RepoContext integration, scope extraction, violation kind metadata, and fixture-based end-to-end validation via `file_fixable.tsx`/`file_fixable_expected.tsx`. GraphQL fix mutation tests are scoped to avoid fixture side effects. `TestArtifactIDAndURI` covers all artifact and collection types. `TestIdToUri`/`TestUriToId` validate bidirectional ID↔URI conversion for all types including edge cases. Markdown output tests. Per-language comment scanning tests. `TestSpecsViolation` covers `isSpecText` RFC 2119 keyword detection, `hasImplementationSyntax` backtick and function call pattern detection, `specsPolicy` violation emission for Header Specs regions and section-start spec comments, spec comment exemption from inline and JSDoc comment violations, non-spec comment non-exemption, and `ViolationCodeSpecsSyntax` metadata presence.

## main_test.go

Go test suite includes interaction-author shape coverage to validate JSON decoding for both legacy object payloads and current string payloads. MCP tool tests cover all `Tool*` functions used by MCP handlers. Monorepo tree tests. `TestCollectEntityPropsConsistency` covers prop field presence plus sanitization. `TestSingleLineOutput` verifies all rendering paths produce single-line output. `TestSpecsViolation` covers spec text detection and exemption from comment violations.

## Code Hygiene Hooks

The code hygiene hook enforces comment, license, region, and file header policies before changes are shared.
File headers use a standardized structure: a Header region containing the file artifact ID (emoji-prefixed path like `💻semio/js/src/index.ts`), an optional summary line, contributor lines (year + name + email), a nested License subregion wrapping the full AGPL license text, and a nested Specs subregion for file-level requirements. The `FormatHeader` method on `BaseLanguage` programmatically builds this structure from five arguments (filePath, summary, contributors, license, specs) so all languages produce consistent headers. Languages that support headers set `supportsHeaders=true` in their constructor. The `headerPolicy` validates that both License and Specs subregions exist inside Header, and the `sectionPolicy` exempts these two subregions from empty-section violations so they can remain content-free placeholders. The `fix` command replaces wrong artifact IDs automatically.
It helps keep the codebase clean by removing obsolete inline or block comments, while respecting TODOs, active contiguous comment blocks following them, and specs region content.
Comment scanning is language-agnostic: every language uses the same `BaseLanguage.ScanComments` implementation configured with language-specific primitives (comment prefix, block comment delimiters, string literal flavors, JSDoc support, skip directives). This means adding comment scanning for a new language only requires setting the right fields in its constructor.
It treats empty regions as invalid structure and removes them automatically in fix mode so region blocks stay meaningful and concise.
It ignores configuration files to avoid breaking structured data with comment prefix injections.
All code must sit inside named regions; orphan definitions outside any section are reported as code:section:orphan-definition so you can relocate them as full definition blocks.
Inline and block comment fixes are precise: they only remove the comment portion of a line when code is present, preserving the logic integrity.

## Repo Tooling Sync

The repo CLI is the single source of truth for ticket workflows and the GraphQL schema that powers tooling.
The VS Code extension uses the schema mirror to generate typed documents and forwards queries through the CLI so the UI and CLI stay in lockstep.
The repo CLI streams command execution as JSONL events and three rendering adapters decide output format: `NDJSONRenderer` (json) emits raw data lines, `HumanRenderer` (text) renders colored terminal output, and `MarkdownRenderer` (md, default) produces pure markdown links.
Markdown output uses the format `[<id>](<uri>) - <prop1> - <prop2>` for single entities, `- [<id>](<uri>) - <prop1> - <prop2>` for list items, and nested `- ` indentation for tree views. There are no JSON code-block or dump fallback paths — all output resolves to entity-kind-specific renderers.
Every tree and list item is guaranteed to be a single line. Property values pass through `sanitizeProp` which strips all newline variants (`\r\n`, `\n`, `\r`), replaces backticks with single quotes (so backtick-wrapped markdown properties don't break), collapses consecutive spaces, and trims whitespace. The final rendered string from both `renderEntityMarkdownLink` and `renderEntityHuman` passes through `sanitizeSingleLine` as a safety net to guarantee no newlines survive into the output. This two-layer approach (property-level + output-level) ensures that even multi-paragraph summaries or descriptions with embedded code references render as a single continuous line.
The `inferEntityKind(key)` function maps GraphQL operation names (e.g. `ticketOpen`, `goalCreate`) to entity kinds via prefix matching, enabling generic dispatch for mutation results without per-operation switch cases. `renderEntityMarkdownLink(kind, data)` produces the base markdown link format, and `renderEntityMarkdown(kind, data)` wraps it with the `- ` list prefix. Streaming list detection (`" list"` / `" tree"` command suffixes) ensures streamed single-entity events get the correct `- ` prefix in markdown mode.
When `--json` is active, each stdout line is pure domain data (one JSON object per line) without event wrappers or `{"data": ...}` GraphQL envelopes; errors are written to stderr only, and stdout stays empty on failure so downstream consumers can pipe output directly into `jq` or similar tools without filtering boilerplate.
Ticket and goal interaction decoding accepts both legacy object-form and current string-form `author` payloads so historical workspaces remain queryable through GraphQL, CLI trees, and the VS Code extension.
CLI artifact IDs use plain emojis without variation selectors (stripping both U+FE0E and U+FE0F) for consistent cross-platform rendering, letting each terminal or font apply its default emoji presentation.
The codebase uses a dual identification system: emoji-prefixed artifact IDs (primary, for GraphQL/logs/messages/UI) and `semiorepo://` URIs (secondary, for MCP resources and clickable links). `GetArtifactID` and `GetArtifactURI` generate these from kind+data, while `IdToUri` and `UriToId` convert between the two systems. Section and definition URIs encode hierarchy separators (`#`, `§`) as `/`-delimited UPPERCASE-SLUG path segments so URIs remain valid without fragment identifiers. Collection types (projects, bundles, folders, files, sections, definitions, tickets, goals, drafts, todos, policies, violationKinds, contributors, commits) each have dedicated ID and URI formats. The `navigate` MCP tool and `semio.navigate` VS Code command accept either an ID or URI and resolve to the target resource.

## Artifact Kind Derivation

Every artifact in the repo tree (bundles, folders, files, definitions) carries a **kind** that determines its emoji icon, GraphQL enum value, and filter behavior. Kinds are derived from source metadata rather than hardcoded per bundle name.

**Bundle kind** is read from the `bundleKind` field in the bundle root's `package.json` or `project.json`. If neither file exists or the field is missing, the bundle defaults to `library`. Valid values: `library`, `schema`, `binary`, `ui`, `site`, `assets`. This means adding a new bundle only requires setting `"bundleKind": "schema"` (or whichever kind) in its manifest.

**Folder kind** is derived from the folder name. Folders starting with `.` (dotfiles like `.github`, `.vscode`) are `required` because they contain configuration that must be present. Folders containing package manifests (`package.json`, `pyproject.toml`, `go.mod`, `Cargo.toml`, `*.csproj`, `*.sln`) are also `required`. Everything else is `organization`.

**File kind** uses pattern matching on the file name and extension. Test files (`*.test.*`, `_test.*`, `test_*`, `*.spec.*`, `*.stories.*`, `*.benchmark.*`) are detected first. Then config files (JSON, YAML, TOML, XML, INI, and named files like `Dockerfile`, `Makefile`), docs (`.md`, `.txt`, `.rst`), resources (images, fonts, media, archives, binaries), code (a comprehensive list of ~60+ programming language extensions), scripts (`.sh`, `.bash`, `.bat`, `.ps1`), and license files (names containing `license`/`licence`). When generating file header IDs, code files whose first line is a shebang (`#!`) are reclassified as scripts so `#!/usr/bin/env tsx` correctly produces a `📜` header instead of `💻`.

**Definition kind** is derived from the keyword that introduces the definition in source code. The `extractDefinitionKeyword` function inspects the regex match text, finds the word directly preceding the definition name (skipping access modifiers like `public`, `private`, `export`, `pub`), and returns it. `DeriveDefinitionKind` then classifies: interface-like keywords (`interface`, `type`, `trait`, `abstract`, `delegate`, `record`, `union`, `scalar`, `extend *`) → `interface`; constant-like keywords (`const`, `enum`, `var`, `let`, `static`) → `constant`; everything else (`function`, `class`, `struct`, `def`, `func`, `fn`) → `implementation`. A post-processing step `refineDefinitionKind` reclassifies `const`/`let`/`var` declarations as `implementation` when the initializer is an arrow function (`=>`), function expression, or class expression — so `const handler = () => {}` correctly maps to implementation rather than constant.

The repo CLI formats timestamps (RFC3339 tickets, YYYY-MM-DD goals) into relative dates (e.g., "1 week ago", "3 weeks from now") in text and markdown outputs so you can track schedules and progress with human-readable context.
The repo binary is consolidated into a single `semio-repo/cli/cli.go` entrypoint that embeds the engine, CLI command wiring, MCP server mode, and renderers in one place.
Go tests for repo tooling are consolidated into a single `semio-repo/cli/cli_test.go` so CLI behavior, tool helpers, and output format expectations live in one test suite.
Legacy adapter packages are removed so every command is dispatched through the same engine event stream and GraphQL executor.
The streaming core uses a command registry with an emitter that surfaces progress, errors, items, logs, and a terminal done payload so CLI, MCP, and VS Code share one execution model.
Registry invocation accepts JSON inputs and emits item metadata alongside data payloads so tooling can page through large result sets without rehydrating full responses.
The MCP adapter forwards commands through the same streaming registry and supports cursor plus limit paging over item events for list-style tools. MCP tool handlers call `Tool*` functions directly instead of issuing GraphQL mutations, ensuring identical output between MCP and CLI for all operations (contributor, project, folder, file, section, definition, ticket, goal, draft, analyze, fix, policy). MCP resource handlers use validated GraphQL queries matching the actual schema field names.
Project detection scans all non-hidden top-level directories (excluding `node_modules`) as potential projects, treating their subdirectories as bundles. This supports both `@`-prefixed (npm workspace convention) and non-prefixed project directories.
Benchmark, preflight, and dependency update workflows are implemented inside the same single-file entrypoint so operational commands share the unified event pipeline.
The CLI exposes an export command that emits a SQLite snapshot of bundles, folders, files, sections, contributors, tickets, policies, and violations.
Go repo-tooling tests are organized into a fast lane and a slow lane: fast checks cover the same command families with lightweight assertions for tight feedback, while heavy graph/tree/lifecycle/e2e checks run as explicit slow shards in parallel jobs. This keeps the full behavior surface tested while reducing wall-clock time for day-to-day development.
The `sync github` command reconciles local tickets/goals with GitHub using a three-tier hierarchy: root goals (depth 0) map to milestones, first-generation child goals (depth 1) map to issues with the `goal` label linked to the root milestone, and deeper goals (depth 2+) map to sub-issues of their parent goal's issue without milestone linkage. It actively repairs existing goal issues so depth 1 issues always carry the root milestone, depth 2+ issues always have a parent sub-issue link and no milestone, and missing `goal` labels are reattached. Goals are processed in depth order so parents exist before children. The command also migrates child goals from legacy milestones to issues, closes issues for closed tickets, resolves root goal milestones by title via the GitHub API before applying them to ticket issues, synchronizes the GitHub repository label catalog for all valid project and bundle `@` labels (creating missing and deleting invalid), updates stored milestone URLs, and removes invalid `@` labels from both ticket-linked issues and repository issues discovered during a global GitHub issue sweep.
Section management includes `integrate`, `extract`, and `move` commands. `integrate` merges a source file into a target section. `extract` pulls a section out of a file into a new file. `move` is a general dispatcher that accepts emoji-prefixed artifact IDs (📁 folder, 💻/📄 file, 🔖 section) and routes to the appropriate operation — including cross-kind moves like file→section (integrate then delete) and section→file (extract). All file and folder moves automatically update `AGENTS.md` codebase headers to keep documentation in sync.
GraphQL ticket UI inputs accept normalized enum tokens (copilot_chat, claude_code, codex, etc.) so CLI and tooling inputs map cleanly to schema enums.
Section and definition ranges expose line/column start/end positions so editors can locate code precisely.
Range selections always request start/end line/column subfields so Position objects satisfy schema selection requirements in CLI, MCP, and VS Code queries.
Section list queries fetch nested children alongside ranges so tree views can render full section hierarchies.
Repo tool definitions are top-level only (anchored at the start of the line); definitions inside classes, functions, or indents are ignored to keep the scope flat and manageable.
Ticket listing reads from the active `.semio-repo/tickets` workspace and falls back to legacy root `tickets/` directories when needed.
Repo `tree` commands support a `--md` flag that renders the structure as a Markdown nested bullet list with links.
The top-level `repo tree` command defaults to Markdown output; `--text` renders ASCII connectors and `--json` returns the raw tree object.
Project tool listing is sourced from project records (`LoadProjects`) so project outputs and tool return types stay aligned on `Project` entities.
VS Code consumes the JSONL stream, extracts the final `result` payload, and returns the GraphQL response to keep extension data aligned with the CLI engine.
Devcontainer attach uninstalls any existing semio-repo extension across IDE IPC hook CLIs and extensions directories, clears stale VS Code and Cursor caches, installs the freshly packaged VSIX, verifies via list-extensions, and falls back to direct installation into IDE extensions directories with extensions.json updates (including the `$mid` location key) when CLIs report WSL-only usage.
The semio-repo extension targets a VS Code engine range compatible with Cursor (1.105.x) so Cursor can load the bundled extension without version rejections.
VS Code extension packaging requires an unscoped extension name in `semio-repo/vscode/package.json` so `vsce package` can build the local `.vsix`.
VS Code extension sidebar views are consolidated into exactly two views: `Monorepo` (the repo tree) and `Filter` (the global filter state).
The Filter view is not a second tree of options; it is a compact state panel where each filter-kind is represented by one item and the available options are exposed as menu actions on that item.
Filter items render with emoji plus a name for clarity, while filter option menu actions use emoji-only labels with no codeicons; tooltips explain the filter purpose and current search state without crowding the tree.
The Filter view updates the shared filter state, and the Monorepo view consumes that state to hide non-matching items across all branches.
This keeps the UI predictable: expanding nodes always shows structure, while filtering always happens in one place.
VS Code extension tests run through `@vscode/test-cli` and require a display server on Linux; in headless environments the test script wraps `vscode-test` with `xvfb-run` so Electron can boot without a real `DISPLAY`.
Electron launch arguments for stable CI/headless execution are configured in `semio-repo/vscode/.vscode-test.mjs` so local runs and CI share the same test host configuration.
Repo operational artifacts (tickets, contributors, reports) live in `.semio-repo/` so workflow state stays centralized and out of product bundles.
Repo analyze only inspects considered files by honoring `.gitignore`, excluding `.semio-repo/`, and skipping `assets/repo/` fixtures.
Repo file/folder listing and diagnostics apply `.gitignore` patterns directly (including tracked matches) alongside repo metadata exclusions, and CLI analyze/fix commands accept scope arguments so tooling stays consistent across entrypoints.
GraphQL Relay `node(id:)` resolves the canonical IDs emitted by the schema (`semio/...` and `semio-repo/...`) so clients can round-trip IDs without constructing custom `kind:id` strings.
Ticket close derives semantic diffs from a file list after applying the same repo exclusions and `.gitignore` filters, so tooling and tests must pass at least one considered (non-gitignored) path.

## Ticket Workflow Signals

Ticket creation always captures both the LLM and the UI surface (copilot-chat, antigravity, cursor, claude-code, codex, droid) so every interaction records the toolchain context. Parsing is forgiving; for example, `claude-opus-4-5-20251101` resolves to `opus-4-5` and `gpt-5-3-codex-test` resolves to `gpt-5-3-codex` by finding them as substrings of the slugified input.
Ticket issue bodies always start with a `# 🤖 Prompt` heading, and reopen actions add a prompt comment with the same heading so each interaction is surfaced in GitHub.
Ticket closing derives bundle labels from every touched path, adds `semio-repo` when a file falls outside bundles, and posts a semantic change list for bundles, folders, files, sections, and definitions using status icons plus `-removed`/`+added` counts.
Ticket summary comments prepend a `# 🔍 Summary` heading so the close summary is visually consistent in GitHub issues.
Ticket line metrics report full line counts for added and deleted scopes, and diff-based added/removed counts for modified scopes.
Ticket close ignores files inside the active ticket workspace (`ticket.md`) so ticket artifacts never appear in change lists.
Prompt and summary headings are formatted through shared ticket helpers to keep create, reopen, and close flows consistent.
Ticket title updates rename the ticket folder to the new slug so ticket paths stay aligned with their titles.
Ticket GitHub issues are automatically linked to the usalu project 2 during create and reopen flows.
Ticket open respects the `CONTINUE` keyword to resume the latest ticket and the `NOTICKET` keyword to skip ticket creation while still running the task.
These signals keep GitHub issues, metrics, and bundle ownership consistent across CLI, GraphQL, and the VS Code extension.
Ticket bundle labels come from semantic bundle diffs so README/AGENTS changes do not force `semio-repo` labels.
Each ticket interaction stores its own semantic diff payload; tickets no longer keep a top-level diff snapshot.

Semantic ticket diffs are computed against a full codebase snapshot exported to `.semio-repo/reports/codebase.json` when `repo analyze` runs without a scope, keeping bundle/folder/file/section/definition changes grounded in the same snapshot structure.

## Code Report

The repository emits a machine-readable report (`reports/code.json`) that enforces a comment-free codebase (including multi-line and JSDoc blocks, with explicit exemptions), flags temporary `[DEBUG]` logs, auto-adds missing SPDX license headers in `npm run fix`, validates properly nested named regions, checks that `semio/js` files do not import outside the workspace unless they are the shared `elements.tsx`, flags domain-specific terminology inside those shared elements, and includes reason/solution text for each problem to make remediation explicit.
Inline comment violations are grouped per contiguous inline-comment block while comment detection skips markers inside string literals and template literal text.

## Monorepo Tree

The `tree` command renders the complete monorepo as a hierarchical tree covering projects, bundles, folders, files, sections, definitions, goals, tickets, drafts, policies, contributors, and commits.
An optional positional query argument performs fuzzy full-text search via bleve against all node attributes (id, label, description, status, contributor, etc.), preserving the parent chain of matched items while pruning unmatched branches.
Kind-level filtering uses `--only-<kind>` and `--no-<kind>` flags (e.g., `--only-bundle`, `--no-folder`) to include or exclude entire node types. When a kind is excluded, its children are promoted to the parent level (e.g., `--no-folder` makes files appear directly under bundles).
Sub-kind filtering narrows within a kind (e.g., `--only-library` for library bundles, `--no-required` for required folders, `--only-code` for code files, `--only-implementation` for implementation definitions).
Date filtering supports `--only-year`, `--no-year`, `--only-month`, `--no-month`, `--only-day`, `--no-day` for tickets and commits.
Status filtering uses `--only-open`, `--only-closed`, `--open`, `--closed` for goals and tickets.
Contributor filtering uses `--only-contributor-name` and `--no-contributor-name`.
Section and definition parsing is opt-in: sections are only loaded when `--only-section`, `--only-definition`, or a search query is active, keeping the default tree fast.
Data loading uses a single concurrent filesystem walk for folders and files, with parallel streaming of goals, tickets, drafts, policies, contributors, and commits, so the full tree builds in one pass.
The `policy tree` command renders policies with their violation kinds as a nested tree, where each violation kind path (e.g., `code/header/missing-region`) is split on `/` and rendered as a multi-level tree. Redundant root nodes matching the policy ID are stripped. Violation kind IDs use `#`-separated titleized values (e.g., `🚫Code#Header#Missing Region`), violation kind paths use `/`-separated slugs (e.g., `code/header/missing-region`), and violation kind URIs use `/`-separated UPPERCASE slugs (e.g., `semiorepo://violationKind/CODE/HEADER/MISSING-REGION`). `TitleizeSlug` converts hyphenated slugs to title case words. `ViolationKindPathToIdValue` and `ViolationKindIdValueToPath` convert between path and ID value formats. `ViolationKindIdToUriPath` and `ViolationKindUriPathToId` convert between path and URI path formats.

## Section Tree

File sections are modeled as a nested tree derived from language-specific section markers and JSON key paths so tooling can reason about structure instead of raw line ranges.
The repo CLI and VS Code extension request the active file's section tree and use the resolved ranges to jump to sections, rename/move nodes, and create or delete child sections with consistent paths across tools.
The `integrate` command allows wrapping a source file's content into a section marker and inserting it into a target file, either at the end or nested within an existing parent section.

## Shell Language Support

The repo CLI treats `.sh` files as shell language sources so section trees, headers, and comment policies apply to scripts alongside other languages.
Shell files follow the same `# region` and `# endregion` markers as other hash-comment languages, allowing consistent section navigation and ticket line attribution.

## Ticket System

Development work is tracked as tickets composed of interactions. Ticket creation does not create an interaction; interactions are explicitly started and finished, require file lists (`updated`, `created`, `removed`), and interaction finish derives the per-file lists and line stats from git diffs between the last interaction commit (or ticket base) and the current commit. Ticket finish aggregates all interaction files and recomputes total line stats from git against the ticket base commit.
Ticket entry points require prompt text for ticket creation and interaction start, while file arrays can be omitted at entry and still enforced by interaction rules.
Each ticket workspace stores a single `ticket.md` that holds todos, changes, log, and summary sections; closing a ticket writes the summary into the `ticket.md` summary block.
Line totals only include the files declared in the ticket interactions so unrelated diffs stay out of ticket reports.
Section line metrics map added lines using current file sections, map removed lines using base-commit section ranges, and determine affected definitions from added lines only so edits stay attributed to the right sections.
Ticket section ranges are stored as line-only start/end integers with no column data so tooling treats them as line spans.
Temporary scripts, fixtures, and data stay inside the active ticket folder so work-in-progress artifacts remain scoped to the task.

## MCP Tool Gateway

The MCP server validates tool argument types and required fields before invoking the CLI so errors are surfaced at the tool boundary instead of silently proceeding.
File and folder arguments are checked for path correctness so directory paths cannot be passed where files are required.

## Contributors

Contributor activity is derived from ticket frontmatter and source file headers, so authorship and ownership stay aligned with the artifacts people actually touch.
Each contributor aggregates tickets, commits, bundles, files, and line totals, and the list is ordered by ticket volume so the most active contributors surface first.

# Specs

## Code Hygiene

Source files MUST include a Header region with file artifact ID, contributors, a License subregion, and a Specs subregion.

File headers MUST contain the correct file artifact ID (emoji-prefixed path) instead of plain file paths.

File header artifact ID violations MUST be autofixable by replacing the identified line with the correct artifact ID.

File headers MUST contain a License subregion wrapping the AGPL license text.

File headers MUST contain a Specs subregion for file-level requirements (may be empty).

File headers MAY contain a summary line after the file artifact ID.

Header generation MUST programmatically build headers from file path, summary, contributors, license, and specs arguments.

Languages that support headers MUST set supports-headers flag and inherit header generation from the base language.

Header policy MUST validate that License and Specs subregions exist inside Header.

Section policy MUST exempt License and Specs children of Header from empty-section violations.

Source files MUST NOT include inline comments except for license headers, region markers, TODO markers (including contiguous comment blocks following them), specs region content, spec comments after section starts, and comments in configuration files.

Block and JSDoc comments are treated as inline comments.

Comment scanning MUST be language-agnostic using configurable primitives per language.

Each language MUST declare its string literal flavors (templates, raw backticks, triple quotes, verbatim strings), JSDoc support, and skip directives in its constructor.

Comment detection MUST ignore comment markers inside string literals, template literal text, raw backtick strings, triple-quoted strings, and verbatim strings.

Language-specific skip directives MUST be excluded from inline comment violations alongside built-in directives (TODO, semio-ignore-).

Inline comment violations MUST be grouped per contiguous inline-comment block.

Temporary diagnostic logs MUST include the [DEBUG] prefix and are considered removable.

Region blocks MUST be properly nested and MUST be closed with a matching named end marker.

Region blocks MUST NOT be empty.

All code MUST be within sections.

Developer documentation MUST be centralized in the root README.md and AGENTS.md; non-root AGENTS.md files and non-package README.md files are forbidden.

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

Spec comments after section start markers MUST be exempt from inline comment violations when they contain specification keywords (MUST, SHOULD, SHALL, MAY, REQUIRED, RECOMMENDED, OPTIONAL).

JSDoc and block comments that contain specification keywords MUST be exempt from JSDoc and block comment violations.

Spec text MUST NOT contain implementation-specific syntax such as backtick-wrapped code or function call patterns.

## Sections

File section trees MUST be derived from language-aware section parsing per file.

Section data MUST expose file path, section path, range, and parent-child relationships.

Shell scripts MUST use hash-based region markers for section parsing.

## Move, Integrate, Extract

The move command MUST accept two artifact ID arguments (source and target) and dispatch based on artifact kind pairs: file-to-file, folder-to-folder, section-to-section within the same file, file-to-section (integrate then delete source), section-to-file (extract).

Artifact IDs MUST be parsed detecting kind from emoji prefix (folder, file, section) and extracting path and section parts from hash-delimited slugs.

Section slug resolution MUST attempt to match existing section names case-insensitively before falling back to un-slugify conversion.

The integrate command MUST accept either two artifact ID positional arguments (source file, target section) or file, target-file, target-section, parent-section flags.

The extract command MUST accept either two artifact ID positional arguments (source section, target file) or file, section, target-file flags.

File and folder move operations MUST automatically update developer documentation section headers by replacing old path prefixes with new ones.

Cross-kind file-to-section move MUST remove the source file and its documentation entry after successful integration.

MCP tools for move, extract, and integrate MUST expose the same functionality as their CLI counterparts.

## Tree

The tree command MUST render the complete monorepo as a hierarchical tree of categories (projects, goals, drafts, policies, contributors, commits) with nested entity nodes.

The tree command MUST accept an optional positional query for fuzzy full-text search across all node attributes (id, label, description, status, contributor, kind, URI).

Search results MUST preserve the parent chain of matched items and prune unmatched branches.

Kind-level filtering MUST use only-kind and no-kind flags for projects, bundles, folders, files, sections, definitions, goals, tickets, drafts, policies, contributors, and commits.

Excluded kinds MUST collapse, promoting their children to the parent level.

Sub-kind filtering MUST narrow within a kind via only-subkind and no-subkind flags (library, schema, binary, client, site, assets for bundles; organization, required for folders; code, script, config, test, docs, resource, license for files; implementation, interface, constant for definitions).

Date filtering MUST support only-year, no-year, only-month, no-month, only-day, no-day for tickets and commits.

Status filtering MUST support only-open, only-closed, open, closed for goals and tickets.

Contributor filtering MUST support only-contributor-name and no-contributor-name.

Section and definition parsing MUST be opt-in, activated only when only-section, only-definition, or a search query is present.

Tree building MUST use a single concurrent filesystem walk for all folders and files with parallel streaming of all other data sources.

The policy tree command MUST render policies with their violation kinds as a nested tree derived from splitting violation kind paths on the slash separator, stripping redundant root nodes that match the policy ID.

Violation kind paths MUST use slash-separated slugs. Violation kind IDs MUST use hash-separated titleized values prefixed with the violation kind emoji. Violation kind URIs MUST use slash-separated uppercase slugs.

## Tooling

Sidebar view providers MUST be registered once per view with a single shared filter state source.

VS Code extension test runners MUST support headless Linux execution by provisioning a virtual display when DISPLAY is missing.

CLI artifact IDs MUST use plain emojis by stripping variation selectors (U+FE0E and U+FE0F) to ensure consistent cross-platform rendering.

Artifact IDs are the primary identification system and MUST be used in GraphQL, logs, messages, and UI labels.

Artifact URIs (semiorepo:// scheme) are the secondary identification system and MUST be used where IDs are not supported (MCP resources, clickable links).

Section URIs MUST encode section names as UPPERCASE-SLUG path segments replacing hash separators with slashes.

Definition URIs MUST encode section and definition names as UPPERCASE-SLUG path segments replacing hash and section separators with slashes.

Project URIs MUST include the @ prefix in the project code path segment.

Collection artifact types (projects, bundles, folders, files, sections, definitions, tickets, goals, drafts, todos, policies, violation kinds, contributors, commits) MUST have dedicated ID and URI formats.

ID-to-URI conversion MUST convert any emoji-prefixed artifact ID to the corresponding semiorepo:// URI.

URI-to-ID conversion MUST convert any semiorepo:// URI to the corresponding emoji-prefixed artifact ID.

The navigate MCP tool MUST accept either an artifact ID or URI and return both the resolved URI and ID.

The navigate VS Code command MUST accept either an artifact ID or URI and navigate to the corresponding resource.

Repo tool definitions MUST be top-level only (anchored at the start of the line).

The sync github command MUST reconcile local tickets and goals with GitHub by: ensuring root goals (depth 0) have milestones, ensuring first-generation child goals (depth 1) have issues with the goal label linked to the root goal's milestone, ensuring deeper goals (depth 2+) have issues with the goal label linked as sub-issues to their parent goal's issue without milestone, repairing existing goal issues so depth 1 issues always carry the root milestone, depth 2+ issues always have a parent sub-issue link and no milestone, and missing goal labels are restored, migrating child goals from legacy milestones to issues, processing goals in depth-first order so parents exist before children, ensuring issue titles and descriptions match local goal and ticket data, reopening GitHub issues if local tickets or goals are open, linking parent tickets as sub-issues to their parent ticket's issue, closing issues for closed tickets, resolving goal milestones by title via the GitHub API before assigning them to ticket issues, synchronizing repository label definitions for all valid project and bundle labels, updating stored milestone URLs, and removing invalid labels that do not map to current projects or bundles from both ticket-linked issues and repository-wide issue listings.

Go repo-tooling tests MUST support fast and slow execution lanes, and slow-lane suites MUST be shardable across parallel jobs while preserving full command-surface coverage.

CLI JSON output MUST emit pure data per line without event wrappers or GraphQL envelopes; errors MUST go to stderr; stdout MUST be empty on error.

CLI cobra root MUST set silence-usage and silence-errors to prevent stdout pollution on errors.

## Ticket

A ticket is a development artifact that tracks a task.

A ticket has a status of **open** or **finished**.

A ticket MUST store a prompt which is the prompt used to create the ticket.

A ticket MUST store a commit which is the git commit at ticket creation for line stats calculation.

A ticket interaction MUST store started and optional finished timestamps.

A ticket interaction author payload MUST be accepted as either a string or an object when reading persisted ticket and goal histories.

A ticket MUST store a summary when finished.

A ticket MUST store semantic diffs for projects, bundles, packages, folders, files, sections, and definitions with line stats when finished.

Ticket workspaces MUST store a single ticket.md that captures todos, changes, log entries, and the summary.

Ticket workspaces MUST store the content of the draft if provided.

The draft content MUST NOT be duplicated in ticket.md.

Ticket workspaces MUST store an important.md file for remaining compulsory actions. Ticket finish MUST throw an error if important.md is not empty.

Tickets can be reopened to return to **open** status.

Ticket close and reopen actions invoked from the ticket list MUST apply to the selected ticket without additional selection.

Ticket creation MUST require a prompt and a titleized title (e.g. "Some Title on Something"). Slugs or all-caps titles are forbidden.

Ticket LLM and Client inputs MUST be resolved forgivingly by matching allowed values as substrings within the slugified input.

Ticket title updates MUST rename the ticket folder and slug path.

Ticket open MUST interpret a CONTINUE keyword to continue the latest ticket and a NOTICKET keyword to skip ticket creation.

Ticket finish MUST require a summary and a list of files.

Temporary task artifacts MUST be stored inside the active ticket workspace.

Ticket finish MUST derive semantic diffs across projects, bundles, packages, folders, files, sections, and definitions via git diff between the ticket base commit and the current commit, scoped to the files declared on the ticket.

Ticket line metrics MUST map added lines to current scopes and removed lines to base-commit scopes for semantic diffs.

## Goal

A goal is a high-level grouping for tickets.

A goal has a status of **open** or **closed**.

A goal is stored in the repo goals directory.

Goals reflect the hierarchy of goals.

Tickets can optionally be assigned to a goal.

Tickets can optionally be assigned to a parent-ticket for hierarchy.

Root goals (depth 0) are synced as GitHub milestones. First-generation child goals (depth 1) are synced as GitHub issues with the goal label linked to the root goal's milestone. Deeper goals (depth 2+) are synced as GitHub issues with the goal label and linked as sub-issues to their parent goal's issue without milestone. Ticket issues are linked to the root ancestor goal's milestone.

## Repo Dev Server

The repo dev server MUST persist ticket state, scopes, claims, warnings, violations, and event history in a local database.

The repo dev server MUST accept diff ingestion payloads that include unified patches or file snapshots.

The repo dev server MUST recompute scope indexes and claims for files referenced by ingested diffs.

The repo dev server MUST emit conflict warnings when the same scope is claimed by multiple open tickets.

The repo dev server MUST expose HTTP endpoints for ticket lifecycle commands, diff ingestion, precommit checks, indexing, and read-only queries.

The repo dev server MUST support bearer token authentication for non-health endpoints.

The repo dev server MUST verify GitHub webhook signatures when configured.

The repo dev server MUST send outbound notifications formatted with prompt and summary headings.

## Repo Tooling

Ticket open inputs MUST allow optional no-issue and draft fields.

The repo CLI binary MUST be consolidated into a single source file that owns engine, CLI, MCP, and rendering behavior.

Legacy repo CLI adapter packages MUST NOT exist outside the single-file entrypoint.

Repo operational commands (benchmark, preflight, update) MUST live in the single-file repo entrypoint.

Ticket close and reopen MUST address tickets via YYYY/MM/DD/SLUG path identifiers.

Ticket close MUST support an all flag to bulk close all open tickets without summary requirements or GitHub interaction.

Ticket reopen MUST require prompt and client values. LLM is optional.

GraphQL ticket-client inputs MUST accept normalized enum tokens for Client selection.

GraphQL ticket-date fields MUST include started and finished timestamps.

GraphQL interaction queries MUST return a list of Interaction objects with prompt, author, and time bounds.

GraphQL section/definition ranges MUST expose line and column positions for start and end.

GraphQL range selections MUST request Position subfields (line, column) for start and end.

Section list queries MUST include nested children ranges for full tree hydration.

Ticket listing MUST read from the repo tickets directory and fall back to legacy directories when present.

Ticket open MUST require a Goal ID.

Ticket open MUST require a ticket Client enum value.

Repo CLI MUST expose an export command that emits a SQLite snapshot of projects, bundles, packages, folders, files, sections, contributors, tickets, policies, and violations.

Repo section tooling MUST expose an integrate command that merges source files into target sections.

Ticket close MUST apply all affected bundle labels and the repo label for out-of-bundle paths.

Ticket close MUST post a metrics comment listing semantic changes for projects, bundles, packages, folders, files, sections, and definitions with status icons and added/removed counts.

Ticket issue bodies MUST prepend a Prompt heading.

Ticket reopen MUST add a Prompt comment with the latest prompt.

Ticket close MUST prepend a Summary heading to the summary comment.

Ticket GitHub heading formatting MUST be consistent across create, reopen, and close flows.

Ticket line metrics MUST use full line counts for added and deleted scopes, and diff-based counts for modified scopes.

Ticket close MUST ignore files inside the active ticket workspace.

Repo analyze without a scope MUST emit a codebase snapshot for semantic diffing.

Ticket GitHub issues MUST be linked to the project board on create and reopen.

VS Code extension manifests MUST use an unscoped name value for packaging.

Repo CLI commands MUST emit a JSONL event stream with a terminal done payload for machine consumption.

VS Code tooling MUST parse JSONL event streams, surface fatal errors, and use the final result payload as the GraphQL response body.

Repo tooling MUST execute CLI, MCP, and VS Code commands through the streaming registry with emitter events for progress, items, errors, logs, and done payloads.

MCP list tools MUST support cursor and limit paging over streamed item events.

Repo operational artifacts (tickets, contributors, reports) MUST be stored under the repo directory.

Repo analyze MUST exclude gitignored files, the repo directory, and assets/repo/ from analysis.

Repo file/folder listing and diagnostics MUST apply gitignore patterns directly (including tracked matches) and exclude repo directory paths.

Repo tree and list commands MUST support a markdown flag that outputs a nested Markdown bullet list using semiorepo:// URI schemes.

Repo tree command MUST display nested Markdown bullet output by default and MUST support ASCII tree output via text flag.

Repo CLI analyze and fix commands MUST accept scope arguments through flags or positional inputs.

GraphQL node query MUST accept the canonical node IDs emitted by the schema.

Ticket close MUST derive bundle labels from semantic bundle diffs and MUST NOT infer labels from documentation files.

Ticket interactions MUST store their own semantic diff payloads; tickets MUST NOT store diff payloads at the top level.

Ticket close MUST require at least one considered file after applying repo exclusions and gitignore filtering.

## Artifact Kind Derivation

Bundle kind MUST be derived from the bundle-kind field in package manifests at the bundle root, falling back to library.

Valid bundle kinds: library, schema, binary, ui, site, assets.

Folder kind MUST be derived from the folder name: dot-prefixed folders and folders containing package manifests are required; all others are organization.

File kind MUST be derived from the file name and extension using pattern matching: test files, config files, docs, resources, code, scripts, and license files.

File header ID generation MUST override the filename-derived file kind to script when the file exists on disk and its first line starts with a shebang.

Definition kind MUST be derived from the language processor keyword: interface-like keywords map to interface; constant-like keywords map to constant; all others map to implementation.

Definition keyword extraction MUST prioritize the word directly preceding the definition name over fallback keyword scanning, and MUST skip access modifiers.

Definition kind refinement MUST reclassify constant definitions as implementation when the initializer is an arrow function, function expression, or class expression.

## MCP Tools

MCP tool calls MUST validate argument presence and types.

File and folder parameters MUST reference correct path kinds (file vs folder).

Invalid MCP tool arguments MUST return errors at the tool boundary.

## Contributor

Contributor contributions MUST be derived from ticket frontmatter and source file headers.

Contributor ordering MUST be based on ticket contribution count.

Contributor contributions MUST expose tickets, commits, projects, bundles, packages, files, and line totals.

## CLI

Terminal output markers MUST render plain emojis (without variation selectors).

The CLI MUST render relative dates for tickets and goals in both text and markdown outputs.

All CLI tree and list items MUST be single-line. Property values MUST be sanitized (strip newlines, replace backticks with single quotes, collapse multiple spaces, trim). Final rendered output MUST be sanitized (strip newlines). Markdown properties MUST be separated by dashes and wrapped in backticks.

The sync github command MUST report issue closures, milestone reconciliation, repository label create/delete operations, and label removals with warnings on failures.

## Ticket UX

Ticket close output MUST present semantic change lists for bundles, folders, files, sections, and definitions with status icons and line metrics.
