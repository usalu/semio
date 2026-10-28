# result-langpick (lane `langpick`, mirror `lp`, wave 1027b, 10-27)

C = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`. Task: ONE schema-first language protocol (the `resolveLanguageV1` JSON) read by every runtime (TypeScript, Go launcher, Rust dashboard, MCP via the launcher package); remove the picker copies (result-progress F8).

## FOR coordinator
STATUS 19:55: DONE. 7 batches staged, green on mirror `lp` (failure sets equal to base for every touched suite; new laws green; the copies law red on live with 51 findings, the vector law red on live with 140 disagreeing (copy, vector) pairs).
```
python3 -I C/langpick-tools/build.py                      # REBUILD all batches from the CURRENT live files first (anchored transforms; each batch is based on the ones before it); Go files need gofmt: LP_GO=<go 1.26.9>
sh C/langpick-tools/land.sh apply [ROOT]                  # core, copies, imports, dashboard, routing in order (27+33+35+16+9 files); land.sh revert [ROOT] undoes in reverse
python3 -I C/langpick-tools/staged/<batch>/stage.py check|apply|revert [ROOT]   # one batch (mode-keeping; `new: null` removes a file, revert restores it with its mode)
sh C/langpick-tools/staged/<batch>/verify.sh <mirror-root>                       # core|copies|imports|routing: scan.ts parse check + suites (LP_BASE=1 on an unpatched root writes the base failure sets); launcher|launcher2: Go cases + vet windows/linux/darwin + MCP compile (LP_GO=<go 1.26.9>)
```
- Land order and steps: (1) `core` (moves the protocol to `🧰️framework/🔨️modules/⚙️runtime/🗣️language/`; edits `nx.json` + `🏗️build/🔣️policy.json` scheduling exclusions, the Bazel scope table, the hermetic table, the launcher registry path -> expect ONE graph re-inference); (2) `copies`; (3) `imports` (33 importers to `@semio-tech/framework/runtime/language`, the process forwarding entry is deleted); (4) `dashboard` (Rust; prove with a dashboard build); (5) `routing` (progress lines, first-failure, heartbeat + the copies law). (6) `launcher` (Go source), then rebuild and reinstall the launcher: `bash "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🐚️.sh"` and the MCP binary (`bun repo run repo-mcp:install`); only after that (7) `launcher2` (drops the 3 registry copies of the refusal texts; an older installed launcher still requires them).
- Product runs that prove it: course hot no-op after the re-inference (the language module sits in the scheduling exclusions like before); `SEMIO_LANGUAGE=de bun repo leases status` (German; was "English default"); `env -u SEMIO_LANGUAGE -u LANG -u LC_ALL bun repo --help` (macOS: the OS language; Linux: the bilingual refusal); one dashboard build (`dash`), `print-rs:test` once (no change expected).
- Not run by me: no official product build, no dashboard binary build (only the `repo-dashboard-preferences` crate: 26/26 on the mirror with a private target dir).

## Live evidence (red)
- `bun C/langpick-tools/disagree.ts <root>` runs every live picker copy on the shared vectors against the independent Python oracle: **15 copies, 140 disagreeing (copy, vector) pairs on live** (`C/langpick-tools/evidence-disagree-live.md`); after the batches **0** (the dashboard is checked by its own Rust law). Examples: `LANGUAGE=de` ignored by 11 copies; `LC_MESSAGES=de` ignored by 6; `LC_ALL=fr LANG=de` -> copies pick de although POSIX lets LC_ALL hide LANG; `SEMIO_LANGUAGE=fr` -> copies silently fall through (the protocol refuses); the retired `SEMIO_DASHBOARD_LOCALE`/`SEMIO_LOCALE` still chose a language in 5 places (progress lines, graph farm, graph memory, the Rust dashboard, developer tools); leases order, testing toolchain, progress lines and the dev hub defaulted to English (no-default rule).
- Copies law (`⚙️runtime/🗣️language/🧪️tests/🟦️.ts` "no other module picks a language"): **51 findings on live** (`C/langpick-tools/evidence-scan-live.txt`), 0 after. It flags subtag idioms in TS/JS/Python/Rust/Go, the retired variables, and JSON arrays that copy the variable order; readers allowed: this module, the Go `🗣️language/🐹️.go`, the Rust `🌐️language/🦀️.rs`, the language-catalogs law.

## Design
- One protocol `🧰️framework/🔨️modules/⚙️runtime/🗣️language/🔣️.json` (schema `🧬️schema/🔣️.json`): flag, `--` end, `SEMIO_LANGUAGE`, `LANGUAGE` + separator, POSIX order (first non-empty only), subtag separators, neutral tags, `withoutChoice` {commands: operatingSystemThenRefuse, notices: everyOffered}, macOS command + parse pattern, Windows user locale, refusal texts per kind (none/flag/variable). It lives in the framework runtime because `📁️filesystem` and `⚙️runtime/🗣️messages` sit below `🏃️process`. Public entry `@semio-tech/framework/runtime/language`.
- TypeScript API: `chooseLanguageV1` (no throw), `resolveLanguageV1` (a command: refuses bilingually), `noticeLanguagesV1` (a notice: the chosen language or every offered one; never reads the OS list, so one environment gives the same notice on every machine; the door exports a command's choice as SEMIO_LANGUAGE, so children speak it). A progress line without a choice is written once per language, each line with its own `[script] ` prefix.
- Go launcher: reads every rule from the protocol (separators, flag, macOS command and pattern via `Host.System(OsLocale)`, refusal texts via `LanguageProtocol.Refuse`), no hard-coded `defaults`/regex/refusal copies.
- Rust dashboard: `include_str!` of the protocol; SEMIO_LANGUAGE (replaces the retired SEMIO_LOCALE), `LANGUAGE`, first set POSIX variable, else the interactive choice; its `systemVariables` copy is gone. It does not read the OS list itself: `repo dashboard` runs through the door, which does and exports the choice.
- Laws: protocol law 38 tests (Ajv schema, 29 shared vectors incl. 12 new disagreement rows, Python oracle `🐍️.py` row by row, Intl.Locale subtags, Go registry reads this file, copies law); Go `TestLanguageResolutionFixture`/`TestLanguageFlag`/`TestLanguageSystemPattern` on the same vectors; Rust `the_declared_chain_resolves_the_shared_environments` on the same vectors; the farm, memory and churn Python oracles import the protocol oracle.

## Verification on mirror `lp` (base failure sets from the same mirror before the batch)
- core: protocol law 38/0, catalog 25/0, progress 8/0, error 10/0, toolchains 26/0, runtime-pin contract 15/0, Go language+registry ok.
- copies (21 suites): failure sets equal base (pre-existing: runtime 3, toolchain 1, standing 1, format-stamps 1, opening-reset 5, developer-tools 1, door startup 1); trampoline 8/0, sealed store 15/0, churn 12/0, farm 22/0, memory 9/0, remote-cache 19/0, doctor 17/0, authorizers 3/0, fleet-status 10/0.
- imports: 33 importers parse; ledger 11/0, bazel sessions 44/1 (= base), ci-report (= base), door startup (= base); `bun repo --help` en/de/OS in the mirror.
- routing: progress 25/0 (was 24 + the new "every offered language" test), expiry 6/0, watchdog 10/2 and owned-execution 3/4 (= base, envelope class).
- launcher/launcher2: all Go launcher cases ok, vet windows/linux/darwin ok, MCP module compiles.
- dashboard: Rust preferences crate 26/0, TS language law 7/0.
- Tools: `C/langpick-tools/` (`build.py`, `land.sh`, `stage.py`, `verify/*.sh`, `disagree.ts`, `gotest.ts`, `cargotest.py`, `rustclosure.py`, `axes.ts`, `reset.py`, `src/`).

## Left open (outside the repo tool or a separate decision)
- `🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts devHubLocaleV1`: a picker with an English default (OS product).
- The dashboard's preference flag is `--language`, the protocol's flag is `--lang`; `🌀️daemon/✉️ipc/🦀️.rs TASK_LANGUAGE_ENVIRONMENT` still spells "SEMIO_LANGUAGE" as a literal (a reuse test matches it).
- MCP `🧱️results/🐹️.go` detects a language refusal by text (`"no language"`); it could compare against the protocol texts.
