# 📓️ Opus — Go test triage and `⌨️cli` godfile split

Scope: the nine Go packages `🔨️modules/{🏠️workspace,🪪️identity,🧾️yaml,🔎️search,📐️model,🗣️languages,📡️events,🚚️move,⌨️cli}/📦️packages/🐹️go`
(in `⌨️cli`, everything except the `loc`/`benchmark` functions, which another owner holds).
Environment: `GOWORK=C:/git/semio/go.work`, Windows 11, `go 1.25`.

## 1. Result

| package | before (failing tests) | after | `go vet` |
| --- | ---: | ---: | --- |
| `🏠️workspace` | 2 | **0** | ok |
| `🪪️identity` | 0 | **0** | ok |
| `🧾️yaml` | 0 | **0** | ok |
| `🔎️search` | 0 | **0** | ok |
| `📐️model` | 51 | **0** | ok |
| `🗣️languages` | 1 | **0** | ok |
| `📡️events` | 0 | **0** | ok |
| `🚚️move` | 5 | **0** | ok |
| `⌨️cli` | 111 (15 top-level) | **0** | ok |

Final sweep (`go vet ./...` then `go test ./... -count=1` in each package directory):

```
🏠️workspace   vet=ok  ok  github.com/usalu/semio/repo/workspace  32.105s
🪪️identity    vet=ok  ok  github.com/usalu/semio/repo/identity    0.650s
🧾️yaml        vet=ok  ok  github.com/usalu/semio/repo/yaml        0.361s
🔎️search      vet=ok  ok  github.com/usalu/semio/repo/search      0.346s
📐️model       vet=ok  ok  github.com/usalu/semio/repo/model       0.431s
🗣️languages   vet=ok  ok  github.com/usalu/semio/repo/languages   0.388s
📡️events      vet=ok  ok  github.com/usalu/semio/repo/events      0.831s
🚚️move        vet=ok  ok  github.com/usalu/semio/repo/move        0.599s
⌨️cli         vet=ok  ok  github.com/usalu/semio/repo/cli        12.129s
```

Harness, after the code changes (`subject fundamental --implementation go`, then `parity`):

```
🗣️languages   go: cases=5 executed=12 passed=12 failed=0   parity: executed=26 passed=26 parity=16/16
🏠️workspace   go: cases=5 executed=6  passed=6  failed=0   parity: executed=14 passed=14 parity=10/10
📐️model       go: cases=3 executed=6  passed=6  failed=0   parity: executed=13 passed=13 parity=8/8
🪪️identity                                                  parity: executed=5  passed=5  parity=4/4
🚚️move                                                      parity: executed=28 passed=28 parity=12/12
```

## 2. Code fixes — the moved Go code diverged from the contract (category b/c)

1. **`🗣️languages` — the Go language plugin lost the variation selector on its section marker.**
   `NewGoLanguage()` carried `"// #region \U0001F516%s"` (bare `🔖`, no `U+FE0F`) while every other
   language in the table carries `🔖️`. The same defect sat in the shared single source of truth
   `🗣️languages/🧬️schema/🔣️languages.json`, whose `go` entry was the only one of twelve missing
   `\uFE0F` — so the Rust twin (which embeds that file with `include_str!`) reproduced it and the
   two implementations agreed on the wrong marker. Fixed in both, hence no parity break;
   `🧾️header-roundtrip` (`header-per-language`, a Go↔Rust differential) still passes 16/16.
   Fixes `TestFormatHeaderEmptyRequirements`.

2. **`🏠️workspace` — `GetGitIgnoredSet` could not see any non-ASCII path.**
   It ran `git check-ignore <paths…>` and matched the echoed pathnames verbatim; with the default
   `core.quotepath=true` git echoes emoji paths as C-quoted octal (`"\360\237\247\260…"`), so every
   path in this repository's own tree was reported as *not* ignored. Now `git -c core.quotepath=off
   check-ignore …`. Fixes `TestBuildBinaryArtifactsGitIgnored`.

3. **`📐️model` — an artifact id repeated the entity emoji for emoji-named files and folders.**
   `GetArtifactID("file"|"folder", …)` flattened the raw basename, so `🧪️index.test.ts` became
   `🥼🧪️indextest` and `🖼️logo.png` became `💾🖼️logo`. A new `stripLeadingEntityEmoji` (built on the
   contract-frozen `ExtractEntityEmoji`) drops the taxonomy emoji the *name* carries, leaving the
   *kind* emoji to name the entity once. Fixes `TestArtifactIDAndURI/{file_test,file_resource}`.

4. **`📐️model` — `Technology.GetID` switched on a kind vocabulary that no longer exists.**
   `TechnologyKind` constants are emojis (`👤️`, `🧰️`, `🔬️`), but `GetID` compared them against the
   godfile-era words `"user"`/`"infrastructure"`/`"research"`; every arm was dead and only the
   `strings.Contains(name, "repo")` fallback still fired, so a `user` technology got no emoji at
   all. It now takes the kind emoji directly and falls back to `DeriveTechnologyKind`, matching
   `resolveTechnologyEmoji`. Fixes `TestTechnologyListIDs`.

5. **`⌨️cli` — an unknown command printed help and exited 0.**
   `Command.execute` fell through to `Help()` whenever the selected command had children and no
   `RunE`, even with unconsumed positional arguments, so `repo policy check` (a verb that no longer
   exists — it is `statute` now) "succeeded". It now returns
   `unknown command %q for %q`, which is also what the JSON-error contract expects (empty stdout on
   error). Fixes `TestCliWrongArgs_PolicyCheck` and `TestCliJsonErrorsToStderr/policy_check_missing_id`.

## 3. Test rewrites — godfile-era paths, assets and vocabulary (category a)

The legacy trees are gone (`repo/` is empty, `compose/` holds only `client` and `dev`, `coda/` only
`client`, `💻️client/` is deleted), so every test that scanned them was asserting against a tree that
no longer exists.

| test | package | what was stale | action |
| --- | --- | --- | --- |
| `TestFindRepoRootPrefersMonorepoMarkerOverLocalGoMod` | `🏠️workspace` | `repo/client/main.go` marker | **deleted** — the behaviour is covered by the language-agnostic `🧭️root-discovery` scenario `the-first-marker-upwards-wins`, whose fixture tree `legacy-cli-entry-point-wins-over-a-nearer-checkout` pins exactly "marker beats a nearer checkout / a local `go.mod`" with the marker path the code actually uses (`repo/cli/main.go`) |
| `TestBuildBinaryArtifactsGitIgnored` | `🏠️workspace` | `repo/client/client`, bare `claude`/`codex`/`mcp`, … | rewritten against the artifacts this repo builds today (`⌨️cli/…/semio-repo[.exe]`, `🖥️server/🎛️coordinator/…/server[.exe]`) |
| `TestSectionListCommand`, `TestDefinitionListCommand`, `TestSectionListIDs`, `TestToolSectionList`, `TestToolDefinitionList` | `🚚️move` | `compose/js/index.ts`, `repo/client/main.go` | rewritten onto a `t.TempDir()` fixture (`withSectionFixture`); the godfile helpers `findTestRepoRoot`/`setupToolTest` are gone |
| `TestBundleListCommand`, `TestFolderListCommand`, `TestFolderTreeCommand`, `TestFileTreeCommand`, `TestSectionTreeCommand`, `TestTechnologyListIDs`, `TestBundleListIDs`, `TestToolTechnologyList`, `TestToolSectionTree` | `⌨️cli` | live scans of `compose/*`, `repo/*`, `coda/*` | rewritten onto a new `withMonorepoFixture(t)` — a throwaway monorepo with three technologies (`compose` user, `repo` infrastructure, `coda` research), three bundles carrying explicit `AGENTS.md` emojis, and two sectioned sources. `TestBundleListIDs`' 18-entry live catalogue shrank to the fixture's three |
| `TestPostgresSchemaIncludesKitVersionControlTables` | `⌨️cli` | `repo/postgres/🛢️schema.sql` | **deleted** — the file was removed with the legacy tree and has no successor anywhere in the repository (the only `*.sql` left is `🪶️sqlite/🧬️schema/📐️schema.sql`, which has none of the `kit_*` tables) |
| `TestToolFixScope` | `⌨️cli` | `ToolFix` now answers `fix was removed; handle autofix inside script.ts policy export` | **deleted** — the behaviour was removed on purpose |
| `TestNativeHookEventMappingWithRealData` (93 subtests) | `⌨️cli` | two stale conventions | (a) the session log directory is `🎆️26/🌙️09/☀️06`, not `26/09/06` — the test now composes it with `model.FormatYearDir`/`FormatMonthDir`/`FormatDayDir`, the same helpers `hooks.writeHookArtifacts` uses; (b) `writeRepoLoggingConfig` wrote `config.toml`, while the layout constant is `workspace.ConfigFileName` = `📋️config.toml`, so `cfg.Logging.Session` was always false and no artifact was ever written |
| `TestConfigureCommandDoesNotGenerateConfigFiles` | `⌨️cli` | dispatched on `os.Args[1:]`, i.e. the `go test` flags (`unknown flag: -test.paniconexit0`) | the test now installs its own argv with `cmd.SetArgs(nil)` |
| `TestSpecExactIDs` (36 subtests) | `📐️model` | literal ids written before `EmojiText` normalisation, and `🏘️` for the user technology | literals regenerated: `U+FE0F` stripped except after the text-default code points `EmojiText` keeps it for (`☀ ⏱ ⚙ ⚖ ✏ ✂ 🏗 ⌨ 🖱 🗃 🏷 🛠 🛡 🗑 👮 ⬅ ⬆ ⬇`), and `🏘️` → `👤️` (`EmojiTechnologyUser`). The normalisation itself is contract-frozen — the identity case `😀️entity-emoji-codec` pins it over the `normalizations` vector, and the sibling `TestAllSpecIDExamples` already agreed with it |
| `TestGetArtifactID_Bundle`, `TestAllSpecIDExamples`, `TestArtifactIDAndURI` | `📐️model` | `coda/example` → bundle code `examples`; bundle `asset` → technology `semio`; `repo://ticket/` for the *tickets* collection | expectations corrected. The `examples`/`semio` mappings exist nowhere in the code — `GetArtifactID("bundle", …)` derives the codes from the name by `SplitN(name, "/", 2)` and no catalogue lookup can change them — so they were unreachable golden data. `repo://ticket/🎫` also contradicted its own neighbours (`repo://contributors/…`, `repo://checkpoints/…` are plural and pass) |

## 4. Fixture updates forced by the `🔖️` fix

Normalising the Go section marker changed recorded bytes in `🚚️move`'s own goldens, which had been
recorded against the buggy marker:

- `🧫️fixtures/📥️file-integrate-trees.json` (16 markers), `📑️section-move-trees.json` (12),
  `🧲️section-extract-trees.json` (12): bare `🔖` → `🔖️`, matching the TypeScript cases in the same
  files, which always carried the selector.
- `📥️file-integrate-trees.json`, `go-keeps-the-target-package`: `Source Header len: 69` → `75`
  (two markers × 3 bytes of `U+FE0F`).

Both implementations diverged from the golden *identically* (parity stayed 12/12 through the whole
episode), which is what identifies the golden rather than either implementation as the stale party.
A repository-wide scan of `*.json` / `*.feature` under `🔨️modules` found no other golden carrying a
bare `🔖`; the two remaining hits are deliberate — `🪪️identity/🧫️fixtures/📡️emoji-vectors.json`
(the `normalizations` vector *is* a list of unnormalised code points) and
`🪪️identity/🧬️schema/🔣️entity-emojis.json` (the text-default table).

## 5. `⌨️cli/📦️packages/🐹️go/🐹️.go` split

The file measured 9 260 lines when this job started (the split report's 10 289 had already been
reduced by concurrent work), and is now cut three ways inside the same `package cli`:

| file | lines | content |
| --- | ---: | --- |
| `🐹️.go` | 6 216 | command wiring: event export, engine, cli adapter, every command factory, the command framework |
| `🖨️render.go` | 724 | `🌩️CLI Renderers` — `StreamRenderer`, the NDJSON/human/markdown renderers, `renderStream`, the tool-result projections |
| `🔌️mcp.go` | 2 370 | `🔌️Mcp Surface` — `🦀️Mcp`, `🎼️Args`, `🔧️Paths`, `🪄️Handlers`, `🏬️Mcp Resources Handlers`, `🗣️McpDescriptions`, `🦀️McpServerFactory`, `🪝️TicketMcpHandlers`, `🔌️Mcp Builders`, `🖥️Mcp Server Adapter` |

Region boundaries were moved verbatim; the duplicated `// #region X` / `// #region X` pairs the
earlier godfile split had emitted were collapsed. The 423 top-level declarations before and after
the cut are the same set (`Compare-Object` over `^(func|type|var|const) ` in the backup versus the
three files: empty). Per-file import lists were derived from actual alias use — `🐹️.go` lost
`bufio`, `math` and `text/template`, which now live only in `🔌️mcp.go`. `gofmt -l` is silent,
`go build`, `go vet` and `go test` pass. The split script is
`$TICKET/🔨️go-split/📜️split-cli.ps1`.

## 6. Left open

1. **`go test -tags exhaustive ./...` in `⌨️cli` exceeds the 600 s `go test` default timeout on this
   host.** `go vet -tags exhaustive ./...` is clean. The exhaustive level is routed to its own
   `test-exhaustive` script target and is not part of `go test ./...`; the earlier split report
   likewise only ever claimed `vet` for it. The suite is 37 live-repo E2E tests that walk the whole
   monorepo. Not investigated further — outside the stated goal, and it needs a raised
   `-timeout` to even produce a verdict.
2. **A stray `🤖️generated.go` (`package client`) was found inside
   `🪪️identity/📦️packages/🐹️go` and deleted.** It broke the build of every package that imports
   identity. It was a copy of the entity-catalog emitter's Go output, whose generator
   (`🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts`) still writes to the deleted
   path `🔨️modules/💻️client/⌨️cli/⚡️implementations/🐹️go/🐹️entity_kinds.g.go`, and nothing in the
   Go tree references `EntityKindCatalog`. Repointing that generator belongs to the wiring agent.
3. **Concurrent edits.** `📐️model/🐹️.go` was rewritten by another agent mid-run and silently
   dropped `stripLeadingEntityEmoji`; it was re-applied and re-verified. `🌳️tree` and
   `🧑️contributors` were uncompilable for stretches of this session (other owners' in-flight work),
   which blocks `⌨️cli`'s build and `🏠️workspace`'s `TestMcpStdioInitializeHandshake`; both were
   green at the final sweep.
4. `📐️model`'s `LookupTechnologies` / `LookupBundles` ports stay nil in that package's own test
   binary, so the bundle-id tests there exercise only the catalogue-free path. The catalogue path is
   covered from `⌨️cli` through `withMonorepoFixture`.
