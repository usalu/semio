# Result gofix2: client/module Go de-duplication and client test fixes

Scope root: `🧰️framework/🛍️products/🦑️repo/🔨️modules/` (`M`). Client = `M/💻️client/⌨️cli` (module `repo/client`). Tools and the two schema dumps are kept in `gofix2-tools/`.

## 1. Duplication, measured

### autofix (`💻️client/⌨️cli/🛟️autofix` vs `🔗️graphql/…/🩹️autofix*.go`)
- Client `applyAutofixes` (1,045 lines) vs the pre-decomposition module function: 14 diff hunks, all package qualification (`rootDir` vs `workspace.RootDir`, `GetLanguage`, `generateFileHeader`, `isSpecText`, `AGPLLicenseText`, `Read/WriteTextFile`, `runFormatterAfterAutofix`) plus gofmt spacing. Zero behavioural difference.
- 19 client tests that call it are bodies of the module tests (0.994-0.999 similarity; `TestFixApplyAutofixes` in the module is the hermetic variant).
- Neither side calls it from production code: the client `fix` verb/mutation answers "fix was removed; handle autofix inside script.ts policy export", and the module function is reached only from its tests.

### buildSchema (`💻️client/⌨️cli/🕸️graphql-schema` vs `🔗️graphql` `BuildSchema`)
Both schemas were walked and dumped (`gofix2-tools/schema-dump/{client,module}-schema.txt`, 350 vs 356 facts). The type system is identical except:
- client-only `Mutation.fix` / `FixResult` (removed feature, always errored);
- `TodoCreateInput.parentID` (client) vs `parentId` (module, Rust twin, fixtures): a client bug;
- module-only unreachable `CountMetrics`, `BundleKind`, `FolderKind` and the four built-in scalars.
The client closures (2,759-line function) are the same schema with inline resolvers; the module is the declarative, SDL/Rust-parity-governed form.

## 2. Elimination (client imports the module package; modules never import the client)
- `go.mod`: client now requires `repo/graphql` (+ indirect `codebase contributors events goals identity languages model move providers statutes tickets todos workspace`, relative `replace` like `repo/cli`). `GOWORK=off -mod=mod go build` works; `workspace` is imported directly by tests.
- Deleted from the client: `🛟️autofix`, `🕸️graphql-schema`, `🏗️graphql-executor`, `🗃️query-list-resolvers`, `🗄️query-lookup-resolvers`, `✍️mutation-resolvers`, `🪆️entity-resolvers`, `internal/graphql` (engine). The 7 registry names were removed from `📚️library/🔣️taxonomy.json` (one-line anchored deletions); client dirs are all registered (134/134 checked by script).
- `🌱️root-and-repo-config`: `executor` is `*graphql.Executor`, `ensureExecutor` calls `graphql.NewExecutor(rootDir)` (it also sets the shared workspace root). `🧩️component` `defaultEngineFactory` likewise.
- Cascade removal (measured with a reachability tool, `gofix2-tools/reach`): `Resolver`, `NewResolver*`, `FileListInput`, `ResolveContributorContributions` + 6 contributor tree types, `GetFolderChildren/Files`, `fallbackFormatText`, `runFormatterAfterAutofix`, `normalizeBundleID`, `ContributorLink`. `ListInteractions`/`InteractionResource` stay because pre-existing dead code (`interactionCommand`) still references them.
- Because the client now serves GraphQL through the module, `buildSchema` no longer exists; the requested per-type decomposition is moot (largest module schema file 404 lines, resolvers 489/507).
- Doc: `💻️client/🔌️mcp/README.md` points at the module executor.

## 3. Client tests
Removed 19 autofix duplicates (module twins exist and pass), `TestFixHeaderWithShebang`, `TestFixViaRepoContext`, `TestToolFixScope`, `TestExhaustiveFixViaGraphQL` (feature removed); `TestGraphQLFixMutation` became `TestGraphQLFixMutationIsRemoved`. Dir `🧪️tests/🧩️section-newline-after-region` deleted, `🛠️autofix-sections-and-comments` renamed `🛠️formatter-plans-and-config-comments`.

Short-mode result (canonical overlay, own test binary, cwd = client dir): baseline 74 failing / 416 pass, now **3 failing / 467 pass** (93 skip, all `-short`).

### Real code defects fixed (client)
- `Technology.GetID` switched on `"user"/"infrastructure"` but kinds are emoji now: every technology id lost its prefix. Now `Emoji → Kind → DeriveTechnologyKind`.
- `🎚️language-go-python-csharp`: header region markers used `\U0001F516` without U+FE0F, so Go headers differed from every other language.
- `🗒️docs-policies` (and 6 other files' doc comments): cp850 mojibake; the README `# Requirements` check compared against the corrupted string, and the AGENTS file/folder section regexes matched garbage. Repaired (decode verified), FE0F restored. Same regex corruption was in `📜️statutes` (fixed there too).
- `internal/command`: unknown subcommand printed help and returned nil; now errors like the module framework.
- `🚫️gitignore-and-formatters`: gitignore cache was not keyed by root, so any root change (tests assign `rootDir` directly) served the previous repo's ignore set. Keyed by root; `invalidateGitignore()` replaces two copies of the invalidation block.
- `🕐️sessions` `StreamSessions` read plain `26/10/18` directories but hook logs are written to emoji-dated directories, so it never found a session. Reader now uses `parseDatedDir`; writer path centralised in `sessionLogDir`. Same bug fixed in `🧑️contributors` (`StreamSessions`) with a regression test that fails without the fix (verified by reverting).
- `🔠️kind-codes` `composeIDToGoalPath` was not variation-selector tolerant.

### Stale tests updated to current truth
Spec/ID tables (FE0F-free ids, bundle names) transplanted from module twins; fixture paths (`G1`, test-dispatch, derived-config, path-emoji); godfile name `📁️files.json`; config file `📋️config.toml`; header marker; session log dir via `sessionLogDir`; hermetic monorepo/section/technology-docs fixtures instead of the old `compose/…` checkout; bootstrap assets (devcontainer, native scripts) as in `repo/cli`; `configure` test pins args; goal/line/lab emoji assertions via `emojiText`. New helpers in `🧪️tests/🎼️helpers`: `withMonorepoFixture`, `withSectionFixture`, `newExecutorFor`, `withTempRepo`.

## 4. Exhaustive (non-short) suite and a pollution incident
Per-test run, 100 s alarm, 6 parallel: 45 timeouts (identical set to the gocomptest baseline: whole-repo walks under load average ~25), 12 failures before my last edits. Fixed since: `TestExhaustiveBundlesNonEmpty`, `…ToolFileTree`, `…ToolFolderTree`, `TestEventIDsUseComposeRepoFormat`, `TestExhaustiveTicketTitleValidation`, `TestExhaustiveCliE2E_TicketLifecycle_Syntaxes_NoManagement`, `TestComputeCompositeFingerprint…` (nested repo needed `commit.gpgsign=false`), `TestExhaustiveLifecycleCommands`.

INCIDENT (mine, repaired): the non-hermetic exhaustive lifecycle/ticket tests ran against the real checkout. Their `ticketOpen` without `noManagement` rewrote about 3,450 `🎫️ticket.json` and 204 `🎯️goal.json` (lossy: dropped `interactions`, `files`, `id`, `llm`, normalised sessions/goal ids) at 04:16, and left test tickets/goals (`E2E-TICKET-POSITIONAL`, `TEST-LIFECYCLE-*`, `TEST-GOAL-*`). Some of this pollution already existed from earlier executor runs (initial `git status` showed modified ticket files). Repair: every ticket/goal json that was rewritten into a lossy re-serialisation of its HEAD content was restored from HEAD content (read-only `git show`, no git state change; verified by semantic comparison, ~1,900 files), test artifacts deleted. Remaining ticket diffs are other agents' monolith-split work and the coordinator ticket. The three tests above now run in a throwaway git repo (`withTempRepo`) with `noManagement`, so a repeat is impossible for them. Other tests still touch the real checkout read-only (and some short tests create then delete goals/tickets); a systematic hermetic pass is open.

## 5. Open
- 3 short-mode failures need the TS toolchain, not Go: `TestCanonicalGoTestDispatcher(+Cancellation)` and `TestMcpStdioInitializeHandshake` run `bun …/📜️script.ts`, which fails with `Cannot find module '@semio-tech/compiler-syntax-ecma-ts'` (`node_modules` absent / library split in progress).
- `TestExhaustiveBreachsNonEmpty` / `TestExhaustiveGraphQLAnalyzeQuery`: `statutes.LoadBreachsFromCache` aborts on the 6 MB ignored cache file `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` (array form, written 10-06 by a TS policy; loader requires the envelope). Needs the producer or a deliberate loader decision; the stale cache file itself is untracked.
- `repo/graphql` `applyAutofixes` has no production caller (only tests); deleting it is a product decision.
- The client still duplicates most of the module stack (832 of 1,067 top-level function names also exist in modules); `Fix` stubs (`ToolFix`, `RepoContext.Fix`, `FixResult`) remain in the client; dead `interactionCommand`/`statuteCommand` remain.
- `composeIDToGoalPath` in `🎯️goals` is still FE0F-sensitive (its test is tautological); not touched.
- Not run: taxonomy gate (`verify taxonomy`, TS not importable), exhaustive-suite re-run end to end after the last edits (the changed tests were run individually).

## Verification (exact)
- Overlay via `gocomp/overlay.py` (port of `canonicalGoPlan`), cwd = `💻️client/⌨️cli`, `GOWORK=<repo>/go.work`: `go build -overlay=<src plan> .`, `go vet -overlay=<test plan> .`, `go test -overlay=<test plan> -c` then `<bin> -test.short -test.v -test.count=1`: 467 PASS / 3 FAIL / 93 SKIP. `gofmt -l` for the client equals the pre-existing list.
- Main package: `go build -overlay=<main plan>` in `📦️packages/🐹️go/📦️main`; `repo graphql '{ repo { id name path } }'` answers through the module; `mutation { fix … }` returns `unknown field "fix" on Mutation`.
- `go vet` + `go test -count=1 .` pass in `🧑️contributors`, `📜️statutes`, `🔗️graphql`.
