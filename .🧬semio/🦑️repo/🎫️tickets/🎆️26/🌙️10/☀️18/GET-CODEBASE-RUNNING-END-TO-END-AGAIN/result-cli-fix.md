# Result cli-fix (2026-10-19)

Executor `cli-fix`, input `audit-cli-1019.md`. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `B` = `L/🏗️build`.

## Decisions

- Help is schema-first. The operation catalogue (`B/⚙️operations/🔣️catalog.json`, schema `⚙️operations/🧬️schema/🔣️.json`) now carries `languages` (en, de), `labels`, a global `flags` list (spelling, aliases, value shape, owning commands, EN+DE description) and, per operation, `summary`, `subcommands`, `flags` (references) and `examples` (EN+DE). `bun repo --help` and `bun repo <cmd> [<sub>] --help` are rendered from it; no usage text is left in code.
- Language: `--lang=en|de`, then `SEMIO_LANGUAGE`, then `LC_ALL`/`LC_MESSAGES`/`LANG`, then the first catalogue language (no code default). Unknown `--lang` and unknown command with `--help` are refused.
- `--graph` vs `--dry-run` (code fixed, docs aligned): `--graph` is the portable alias of `--dry-run` on both tools. `nxBuildArgumentsV1` now translates bare `--graph`, `--graph=true` and `--graph <file>` (bare/true become `--graph=stdout`, `=false` is dropped, a file is kept). Bazel already honoured it. Catalogue and README list `--dry-run` and `--graph[=<file>]` as two flags.
- Neutral exec environment: `SEMIO_PROJECT_NAME` and `SEMIO_PROJECT_ROOT`. `buildExecEnvironmentV1` (used by the Bazel `repo-exec` task) exports them; the Nx names exist only in `⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts` (`nxExecEnvironmentMappingV1`, `nxExecWrapperV1`): Nx `exec` commands run inside a Bun wrapper (metacharacter-free program text, because Nx double-quotes every argument and runs it through a shell) that copies `NX_PROJECT_NAME`/`NX_PROJECT_ROOT_PATH` onto the neutral names.
- Registry verbs: `preflight` is `inspect`, `dependencies sync|lock --all` are `setup` (both verbs are already in the `build-tool` axis law). `--scope <owner>` stays CLI-only.

## Changes (files)

- `B/⚙️operations/🔣️catalog.json`, `B/⚙️operations/🧬️schema/🔣️.json` (new definitions Language, LocalizedText, Labels, Flag, Subcommand, Example; `ExecEnvironment` now SEMIO_*).
- NEW `B/⚙️operations/📖️help/🟦️.ts` (renderer, language and request resolution).
- `B/⚙️operations/🌱️exec-environment/🟦️.ts` (neutral names).
- `B/📜️script.ts` (line 90: help renders from the catalogue; no usage string in code).
- `B/📥️caller/📤️child/🟦️.ts` (preflight argument check skipped for help requests; fixes the `preflight --help` crash).
- `B/🧭️invocation/🟦️.ts` (`--graph` translation).
- `L/⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🟦️.ts` (Nx exec mapping and wrapper).
- `📋️project.json` (tools `build-preflight`, `build-deps-sync`, `build-deps-lock`), `L/🧪️tests/🎛️dashboard-control/🧫️fixtures/🎮️owners.json` (3 rows).
- README `🧰️framework/🛍️products/🦑️repo/README.md`: EN (help bullet, exec row, full flags table with commands column, Nx translation, verification row, registry row) and DE (command block, help bullet, operations table split into one row per EN row, flags table, Nx translation, registry row, Bazel example in Produktketten, new sections Developer Dashboard and Go-Layout, verification text).
- Tests/fixtures: `B/⚙️operations/🧪️tests/🟦️.ts` (rewritten help check, 5 new tests), NEW `B/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json` (snapshot of all 22 help screens x 2 languages), `L/⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🧪️tests/🟦️.ts` + its `🧫️fixtures/🔣️.json` (planning and exec corpora), `B/🔧️tools/🏗️bazel/⚙️operations/🧪️tests/🟦️.ts` (neutral names).

## Tests added (language-agnostic, with third-party cross-checks)

- Snapshot per command and language from the schema-rendered help; every synopsis and every flag/alias is in the root help in both languages; every text is translated (de differs from en).
- Documented flags equal the flags `validateBuildTaskArgumentsV1` accepts (names parsed from its source) and every flag an operation lists passes the validator.
- Real CLI (`bun 📜️script.ts repo …`): `preflight --help`, `cache clear --help`, `dependencies lock --help --lang=de` exit 0 with the own usage.
- Neutral `ExecEnvironment` schema (Ajv) rejects NX names; catalogue and schema contain no `NX_` name.
- Nx corpus for `--dry-run`/`--graph`; exec wrapper run through `sh -c` in Nx's quoting style; cross-check that Nx's own `exec.js` source still exports the two names the mapping reads.

## Verification

- `bun repo <cmd> --help` for 22 forms (root, 13 commands, 8 subcommands, run in a loop): all exit 0 with their own usage.
- `bun test ./…/⚙️operations/🧪️tests/🟦️.ts`: 10 pass, 0 fail.
- `bun test ./…/⚡️caching/🚀️bootstrap/🛠️tools/⚙️operations/🧪️tests/🟦️.ts`: 7 pass.
- `bun test ./…/🔧️tools/🏗️bazel/⚙️operations/🧪️tests/🟦️.ts -t "exec plans"`: 1 pass (other tests of that file not run: they need the Bazel binary).
- `bun test ./…/🧭️selection/🧪️tests/🟦️.ts` 6 pass; `…/🧭️invocation/🧪️tests/🟦️.ts` 3 pass; `…/🩺️preflight/🧪️tests/🟦️.ts` 3 pass.
- `bun test ./…/🧪️tests/🎛️dashboard-control/🟦️.ts`: 9 pass, 1 fail. The failure is the documented, pre-existing row `@semio-tech/repo-dashboard-rs:workflow status` (README Command registry notes it); it does not touch these changes.
- `bun build --target=bun ./…/🏗️build/📜️script.ts` (623 modules) and the root `./📜️script.ts` (2494 modules): ok.
- `bun ./📜️script.ts verify zero-touch`: passed.
- Not run: `…/📥️caller/🧪️tests/📥️birth` and `…/🧭️selection/👁️watch/🧪️tests` fail when run directly with `bun test` (need the script-runner envelope, "Original parent Script process invocation required"); they are unrelated to the changed lines and were not run through `bun repo test`. No Nx or Bazel product run, no real `nx exec` run (the wrapper is tested against Nx's quoting style, not against a live Nx).

## Open items

- Audit item 4 (stage lines `[script] {...}` and the `$ bun …` echo around `--help` output; CI parses `CacheStatus` after them) is not changed: the source is the routing layer (`runScriptMain`), outside this task.
- Audit item 9 (affected-only flags are silently ignored on `run`/`exec`) is documented in the flags table (column "Commands") but not refused.
- `--lang` is accepted only with a help request; other commands refuse it as an unsupported option.
- Windows branch of the Nx exec wrapper (`shell: true`) is unverified.
