# Audit i18n: repo build library (audit-i18n)

Read-only audit, fleet ticket C, auditor `audit-i18n`. Scope: tracked files under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, excluding `🧪️tests` and `🧫️fixtures`: 1683 files, 1107 TS/MJS/CJS sources, 436 JSON. Helper scripts: `audit-i18n-tools/` (`scan_hardcoded.py`, `count_sites.py`, `check_tables.py`, `summarize.py`). Raw hit lists stay in the scratchpad. No git writes, no sub-agents, no repo/bun/cargo/bazel runs.

Method and caveat: static literal scan. A hit is an English-prose string literal as the first argument of `Error(`, `console.*(`, `*.write(`, or a text-like property (`message:`, `summary:`, ...). A sample of 30 Error hits were all English prose. Most Error hits are developer invariants ("Closed complete current Bun ownership DATA required"); they reach users only when the CLI prints them. Counts are literal-first sites, not runtime-reached ones.

## 1. Localization mechanism that exists

- Catalog pattern: JSON with `"languages": ["en","de"]` (EN first) and leaves `{"en": "...", "de": "..."}` under `labels`, `messages`, `flags[].description`, `examples`. Examples: `🏗️build/⚙️operations/🔣️catalog.json` (174 bilingual leaves), `🎛️commands/🗂️registry/🔣️.json`, `🏗️build/🪞️parity/🔣️help.json`, `⚡️caching/💾️storage/🧾️ledger/📖️texts/🔣️.json` (47 keys, `{placeholder}` values), `🏗️build/📏️incremental/📋️scenarios/🔣️.json`, `🏗️build/📏️incremental/🤝️concurrent/🎬️workloads/🔣️.json`.
- Language choice: shared resolver `resolveLanguageV1` in `🧰️framework/🔨️modules/🏃️process/🏗️build/🗣️language/🟦️.ts` (order: `--lang`, `SEMIO_LANGUAGE`, `LANGUAGE`, `LC_ALL`, `LC_MESSAGES`, `LANG`, OS list). It refuses bilingually when nothing resolves. Used by help (`🏗️build/⚙️operations/📖️help/🟦️.ts`), the registry router, incremental, and the vite session.
- Best existing example: `🏗️build/⚙️operations/🔣️catalog.json` with `📖️help/🟦️.ts`. Leaves are declarative and bilingual, the language is passed explicitly into `renderBuildHelpV1(..., language)`, and the resolver refuses instead of defaulting. Second: `🗄️stdio-ledger/🟦️.ts` (lines 368-369) enforces the exact `["en","de"]` descriptor set. `⚡️caching/🌐️vite/🧾️session/🟦️.ts:142-147` shows the progress-line pattern on the shared resolver.

## 2. Counts per module (English prose literal sites)

Totals: 7678 sites = 6747 Error, 587 text properties (449 in `⚖️laws`), 341 console, 3 stream. User output (console/stream) is 344 sites, 286 of them in CLI-channel files (command, process, script, bootstrap, verify, report). Error sites in the CLI channel: 2072 (36 in `⚖️laws`). The bilingual helper `both(...)` covers 22 of 57 Error sites in `📏️incremental`; nothing else user-facing is bilingual.

| module (top level) | console/stream | of which CLI | Error | of which CLI | text prop | total |
|---|---|---|---|---|---|---|
| ⚡️caching | 50 | 32 | 3109 | 1299 | 13 | 3172 |
| 🏗️build | 14 | 13 | 1272 | 372 | 7 | 1293 |
| 🧹️normalization | 11 | 7 | 933 | 26 | 17 | 961 |
| ⚖️laws | 0 | 0 | 36 | 36 | 449 | 485 |
| 🎛️commands | 205 | 205 | 137 | 137 | 4 | 346 |
| 🔍️discovery | 7 | 0 | 288 | 2 | 71 | 366 |
| 🗂️workspaces | 0 | 0 | 286 | 5 | 0 | 286 |
| 🏃️process | 2 | 2 | 141 | 141 | 1 | 144 |
| 🕸️dependencies | 15 | 0 | 175 | 1 | 10 | 200 |
| 🏗️authoring | 14 | 13 | 74 | 17 | 6 | 94 |
| 🗄️stdio-ledger | 0 | 0 | 66 | 0 | 0 | 66 |
| 🎮️playground | 1 | 1 | 21 | 21 | 0 | 22 |
| 🎨️styling | 0 | 0 | 42 | 0 | 0 | 42 |
| 🧪️testing | 2 | 0 | 28 | 1 | 0 | 30 |
| other 16 dirs | 23 | 13 | 139 | 14 | 9 | 171 |

## 3. Localized tables missing DE or EN

- JSON (`check_tables.py`, 404 catalog and schema JSON files, 629 bilingual leaves): zero leaves missing `en` or `de`, zero extra languages, every `languages` array is `["en","de"]`. One intentional empty pair: `gc.mode.apply` = "" / "" in `⚡️caching/💾️storage/🧾️ledger/📖️texts/🔣️.json`.
- TS inline bilingual tables (5 files): symmetric (`🎛️commands/✅️verify/🚦️gates` 6/6, `🏗️authoring/🗂️collection-catalog` 1/1, `🏗️build/⏱️profile/🟨️.mjs` 7/7). The two asymmetric hits are `startsWith("de")` false positives.
- Gaps the table check cannot see:
  - `🏗️build/🧾️ci-report/🟦️.ts:74-76` indexes `texts[language][key]` with no fallback. Line 165 casts runtime strings with `problem as keyof typeof texts.en`, which is unchecked. `CI_TEXT_KEYS_V1` (line 62) is exported but unused, so no key-parity assertion runs.
  - No law in `⚖️laws` checks catalog key or placeholder parity. The only bilingual law (`🔬️interactivity-audit/🧭️all-app`) checks descriptor labels.
  - English-only internal throws: `🏗️build/📏️incremental/🟦️.ts:189` and `🤝️concurrent/🟦️.ts:327`.

## 4. Default-language sites (violate "no default language")

- D1 `⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts:68`: `laneLanguageV1` returns `"en"` unless DE is named. No resolver, no refusal.
- D2 `⚡️caching/💾️storage/🧾️ledger/🟦️.ts:140`: `resolveCacheLanguageV1` returns `languages[0]` (en) when no source names a language. Its doc comment says so.
- D3 `⚡️caching/🚀️bootstrap/📦️dependencies/🚦️concurrent/🟦️.ts:106`: `concurrentLanguageV1` returns `protocol.languages[0]` (en). Doc line 103 states it.
- D4 `🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🟦️.ts:74`: `bazelSessionLanguageV1` returns `"en"` unless DE. Also line 84: `request.language??"en"`.
- D5 `🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs:31`: `selectGenerationLanguageV1` ends in `??"en"`, and reads `SEMIO_DASHBOARD_LOCALE`, a variable outside the shared order (also typed in `🟨️.d.mts:3`).
- D6 `🏗️build/🧾️ci-report/🟦️.ts:64-70`: refuses (good), but it is a second resolver (2-char prefix, no `LANGUAGE`) and its refusal at line 70 is English-only.
- D7 `🏗️build/🔧️tools/🏗️bazel/🔁️probe/🏭️runner/🟦️.ts:34`: forces `LC_ALL` and `LANG=en_US.UTF-8` on the Bazel child. This is an external-tool locale, not our text. Keep it only as a written exception.
- Checked, not defaults: `🏗️build/📥️inputs/🔒️lock/🟨️.mjs:29` (`localeCompare(..., "en")` is collation). `🏗️build/📏️incremental/⌨️command/🟦️.ts:71` (`requested ?? resolveLanguageV1(...)`, refusal intact). `🏗️build/📜️script.ts:267` (sets `SEMIO_LANGUAGE` only after resolution).

## 5. Top 80 user-facing sites

Selection (deterministic, `summarize.py`): 48 console/stream sites from the CLI channel and 32 Error sites from command, process, invocation, incremental, parity and playground files, spread evenly over the file-sorted lists. `X` marks an interpolation. Full lists are in the scratchpad (`out/top80.tsv`, `hits.jsonl`).

| # | file:line (under 📚️library) | kind | English text as written (truncated) |
|---|---|---|---|
| 1 | ⚡️caching/🚀️bootstrap/📜️script.ts:166 | console | [nx] Admitted X structural collection manifests |
| 2 | ⚡️caching/🚀️bootstrap/📜️script.ts:459 | console | Could not capture Nx descendants for cancellation |
| 3 | ⚡️caching/🚀️bootstrap/📜️script.ts:588 | console | [nx] Generation receipt refused: X |
| 4 | ⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:62 | console | [DATA] Store dependency installed-paths receipt not sealed |
| 5 | ⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/💾️storage/🟦️.ts:178 | console | [DATA] Store dependency generation receipt failed |
| 6 | ⚡️caching/🚀️bootstrap/🛠️tools/🟦️.ts:204 | console | [nx] Staging cleanup failed after publication refusal |
| 7 | 🎛️commands/✅️verify/🎯️tool-job-lane/🟦️.ts:36 | console | [verify interactivity tool-jobs fixed-operation-fixture] laws=X mutati |
| 8 | 🎛️commands/✅️verify/🎯️tool-job-lane/🟦️.ts:96 | console | [verify interactivity tool-jobs] production-hosts=X production-invocat |
| 9 | 🎛️commands/✅️verify/🏁️gate-lanes/🟦️.ts:65 | console | [verify] schema parser projection freshness… |
| 10 | 🎛️commands/✅️verify/🏁️gate-lanes/🟦️.ts:80 | console | [verify] indexed generated output (wasm/jco/wasm-pack)… |
| 11 | 🎛️commands/✅️verify/🏁️gate-lanes/🟦️.ts:146 | console | [verify] build owner registry… |
| 12 | 🎛️commands/✅️verify/🏁️gate-lanes/🟦️.ts:196 | console | [verify] history closure… |
| 13 | 🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:31 | console | [verify interactivity] severity=X scope: X UI root(s), X runtime-sanct |
| 14 | 🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:53 | console | [verify interactivity] X allowlist entr(y/ies) pre-declared for future |
| 15 | 🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:80 | console | [verify interactivity apps actions] dispositionFailures=X productionJo |
| 16 | 🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:124 | console | [verify interactivity p1w] live-source and hostile mutations clean. |
| 17 | 🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:215 | console | [verify interactivity p3mn] live-source and hostile mutations clean. |
| 18 | 🎛️commands/✅️verify/🚦️gates/🟦️.ts:24 | console | [verify production-placeholders] X X! X:X |
| 19 | 🎛️commands/✅️verify/🚦️gates/🟦️.ts:54 | console | [verify interactivity commands] declarations=X unreachable=X non-liter |
| 20 | 🎛️commands/✅️verify/🚦️gates/🟦️.ts:126 | console | [verify docstrings] emoji-unique X repeats=X X |
| 21 | 🎛️commands/✅️verify/🚦️gates/🟦️.ts:165 | console | [verify agnostic] X |
| 22 | 🎛️commands/✅️verify/🚦️gates/🟦️.ts:204 | console | [verify debug-tags] X |
| 23 | 🎛️commands/✅️verify/🚦️gates/🟦️.ts:236 | console | [verify interface-owners] oracle disagreement X |
| 24 | 🎛️commands/✅️verify/🟦️.ts:671 | console | [verify abstraction-ownership] X: X |
| 25 | 🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts:79 | console | [verify taxonomy X] X X X: X |
| 26 | 🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts:134 | console | [verify composed-child-refs] assets=X mismatched=X waived=X breaches=X |
| 27 | 🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts:157 | console | [verify history-closure] self-test passed (X cases). |
| 28 | 🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts:183 | console | [verify semantic-vocabulary] X X: X |
| 29 | 🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts:204 | console | [verify build-owners] passed. |
| 30 | 🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts:236 | console | [verify generated-corruption] X X:X X |
| 31 | 🎛️commands/🏭️generate/🟦️.ts:45 | console | [generate config] derived configuration is current |
| 32 | 🎛️commands/📚️examples/🟦️.ts:32 | console | [examples] unknown subcommand X |
| 33 | 🎛️commands/📝️commit/📤️emit/🟦️.ts:75 | console | commit: write NEW bundle scopes, dates, and bullets on prepare stdin a |
| 34 | 🎛️commands/📝️commit/📤️emit/🟦️.ts:109 | console | \n=== git diff --stat (overview) ===\n |
| 35 | 🎛️commands/📝️commit/🔀️run-commit/🟦️.ts:121 | console | commit: no prior bundle WIP commit (subject …🔀️) found |
| 36 | 🎛️commands/📝️commit/🔀️run-commit/🟦️.ts:193 | console | commit: compose-commit-message is empty — run prepare with bundle body |
| 37 | 🎛️commands/📝️commit/🪙️micro-commit/🟦️.ts:98 | console | micro-commit: nothing is staged; stage the intended changes before pre |
| 38 | 🎛️commands/🔧️setup/🟦️.ts:147 | console | [setup] Windows long paths are disabled: tracked fixtures and build ou |
| 39 | 🎛️commands/🕸️dependency-lint/🟦️.ts:58 | console | [workspace-styling] X: current source ownership passed |
| 40 | 🎛️commands/🧩️os/🟦️.ts:45 | console | [os.run] build it: bun repo run workspace:components describe X |
| 41 | 🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:73 | stream | [repo cargo] swept isolated store X of a settled owner\n |
| 42 | 🏗️authoring/🎮️command/🟦️.ts:49 | console |  or: bun ./📜️script.ts new crates <artifact-package-dir> [--dry-run] [ |
| 43 | 🏗️authoring/🎮️command/🟦️.ts:174 | console | new crates: X — wrote X file(s) for X crate(s). |
| 44 | 🏗️build/📜️script.ts:29 | console | [build] Completed the X workspace build graph. |
| 45 | 🏗️build/🔧️tools/🏗️bazel/📜️script.ts:788 | stream | [bazel] Cleanup refusal retained after primary: |
| 46 | 🤖️agent-clients/🎮️command/🟦️.ts:18 | console | [agents check] every client file matches the root declaration |
| 47 | 🧹️normalization/🎮️command-contract/🔁️workflow/🟦️.ts:133 | console | [clean taxonomy verify] clean=X errors=X warnings=X${ticketDir ? |
| 48 | 🧼️workspace-cleanup/🎮️command/🟦️.ts:70 | console | [clean empty-folders] X removals=X bytes=X |
| 49 | 🎛️commands/✅️verify/🎯️tool-job-lane/🟦️.ts:18 | error | Expected verify interactivity tool-jobs mounted-dispatch |
| 50 | 🎛️commands/✅️verify/🏁️gate-lanes/🟦️.ts:113 | error | [verify] X standards/subsets vocabulary policy breach(es) |
| 51 | 🎛️commands/✅️verify/🔬️interactivity-lanes/🟦️.ts:85 | error | [verify interactivity apps actions] X descriptor action disposition or |
| 52 | 🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts:129 | error | X tsconfig.json must declare an exclude list of strings |
| 53 | 🎛️commands/✅️verify/🚦️gates/🟦️.ts:151 | error | usage: verify agnostic <framework-names\|utility-machines> |
| 54 | 🎛️commands/✅️verify/🟦️.ts:673 | error | OS-owned preferences or host controls are declared by plugin surfaces. |
| 55 | 🎛️commands/✅️verify/🧾️taxonomy-lanes/🟦️.ts:184 | error | [verify semantic-vocabulary] X banned-vocabulary breach(es) |
| 56 | 🎛️commands/🏭️generate/🟦️.ts:59 | error | [generate taxonomy] expected census or duplicates, got X. |
| 57 | 🎛️commands/📝️commit/📜️body/🟦️.ts:31 | error | commit: bullet must start with {emoji} then description (no space afte |
| 58 | 🎛️commands/🕸️dependency-lint/🟦️.ts:34 | error | Canonical X TypeScript/JavaScript architecture found X forbidden seman |
| 59 | 🎛️commands/🛠️dev/🟦️.ts:17 | error | repo MCP client binary is missing at X; run: repo run repo-mcp:build |
| 60 | 🎛️commands/🪁️typecheck/🟦️.ts:12 | error | Expected typecheck project <workspace-relative configuration.json> [de |
| 61 | 🎮️playground/🖥️native-host/🟨️.mjs:38 | error | Native host declaration is incomplete |
| 62 | 🎮️playground/🟦️.ts:92 | error | unknown playground host kind: X |
| 63 | 🏃️process/🔐️owner-only/🟦️.ts:77 | error | icacls could not restrict X: X |
| 64 | 🏃️process/🟦️.ts:73 | error | Original Cargo relay operation required |
| 65 | 🏃️process/🦀️cargo/⏱️timing/🟦️.ts:14 | error | Actual observational root absent |
| 66 | 🏃️process/🦀️cargo/🏃️execution/🟦️.ts:29 | error | Relative Cargo execution Source path required |
| 67 | 🏃️process/🦀️cargo/🏃️execution/🟦️.ts:53 | error | Declared Cargo execution manifest differs from original caller |
| 68 | 🏃️process/🦀️cargo/🏃️execution/🟦️.ts:88 | error | Current repository Cargo preparation policy required |
| 69 | 🏃️process/🦀️cargo/📥️dependencies/🟢️ready/💾️storage/🟦️.ts:102 | error | Original Cargo ready inputs changed during consumption |
| 70 | 🏃️process/🦀️cargo/📥️dependencies/🟦️.ts:44 | error | Original Cargo dependency network grant required |
| 71 | 🏃️process/🦀️cargo/📥️dependencies/🟦️.ts:83 | error | Exact common declared pass storage required |
| 72 | 🏃️process/🦀️cargo/🚪️launcher/🟦️.ts:63 | error | Repository Cargo launch arguments exceed their declared bounds |
| 73 | 🏃️process/🦀️cargo/🧪️testing/📦️executable/🟦️.ts:41 | error | Bounded Cargo test manifest required |
| 74 | 🏗️build/📏️incremental/🏃️runner/🟦️.ts:85 | error | A repository-relative path inside the disposable root is required: X |
| 75 | 🏗️build/📏️incremental/👁️observers/🟦️.ts:26 | error | Oversized varint in Bazel execution log |
| 76 | 🏗️build/📏️incremental/🟦️.ts:64 | error | Incremental dependency cycle at X |
| 77 | 🏗️build/📏️incremental/🤝️concurrent/🟦️.ts:323 | error | Sessions must be between 1 and X |
| 78 | 🏗️build/📜️script.ts:203 | error | Expected repo show projects\|project <name>\|targets |
| 79 | 🏗️build/🧭️invocation/🟦️.ts:79 | error | Unknown CMake operation: X |
| 80 | 🏗️build/🧭️invocation/🟦️.ts:206 | error | Original nested build native resources changed |

## 6. Recommended minimal schema-first pattern

1. One schema per catalog (`🧬️schema/🔣️.json`). `languages` is const `["en","de"]`. Every text leaf is `{"en": string, "de": string}` with both required and `additionalProperties: false`. Placeholders are named `{name}`.
2. One resolver: `resolveLanguageV1` everywhere. Delete D1 to D6 and the local resolvers. No `?? "en"` and no `[0]` language fallback. The refusal stays bilingual.
3. One renderer per catalog, `text(catalog, key, language, values)`. It throws on an unknown key, an unknown or missing placeholder, or a missing language. Callers pass the language explicitly.
4. Errors and progress carry data, not prose: `RepoErrorV1 { code, key, values }`, rendered at the CLI boundary from the catalog. Developer invariants get keys too, so no English-only literal reaches the CLI.
5. A new law under `⚖️laws` enforces: (a) `languages == ["en","de"]`; (b) every leaf has both languages with equal placeholder sets; (c) no language default (D1 to D6 patterns); (d) no English prose literal in `console.*`, `*.write(` or CLI-reachable `Error(`. `audit-i18n-tools/scan_hardcoded.py` is the seed for (d).
6. Migrate by catalog: the ops catalog is already compliant. Next are verify progress (`🎛️commands/✅️verify`, 144 lines), commit output (`🎛️commands/📝️commit`, 30), and bootstrap progress (`⚡️caching/🚀️bootstrap/📜️script.ts`, 32). Then the resolver duplicates D1 to D6.
Caveats: the tool output is literal-first and not reachability-proven; minified sources lower line precision; `🧪️tests` and `🧫️fixtures` are excluded per scope; the resolver lives outside the library in `🧰️framework/🔨️modules/🏃️process/…` and was read only as a dependency. Nothing in the repo was modified.
