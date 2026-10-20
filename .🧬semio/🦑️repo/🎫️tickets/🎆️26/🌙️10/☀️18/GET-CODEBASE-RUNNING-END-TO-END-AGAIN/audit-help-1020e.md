# audit-help-1020e: `bun repo` help audit (EN + DE)

Auditor `aud-help-1020e`, foreground, no sub-agents. Only reads and `bun repo <cmd> --help [--lang=en|de]` runs (one at a time). No other `bun repo` command, no build, no git-modifying command. Scratch help dump was deleted after analysis.

Catalogue: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⚙️operations/🔣️catalog.json` (version 1, languages en, de; 17 operations, 44 flags).
Renderer: `.../⚙️operations/📖️help/🟦️.ts`. Dispatcher: `.../📜️script.ts` (help short-circuit at line 96; op dispatch lines listed below).

## 1. Coverage

- 17 operations + 21 subcommands = 38 commands, plus the root page = 39 pages. Each rendered in EN and DE = 78 help runs, all exit 0, no `undefined`/`null` in output.
- Every page prints bun's launcher echo (`$ bun ./…/📜️script.ts repo …`) and two `[script]` JSON lines. All go to stderr; stdout is the help text only.

## 2. Command list with dispatcher evidence

| Command | Dispatch in 📜️script.ts | Notes |
|---|---|---|
| run | 127 | 2 EN-only notes |
| run-many | 127 | none |
| affected | 127, 228 | 4 EN-only notes |
| exec | 127, 210 | 2 EN-only notes |
| watch | 127, 208 | none |
| graph | 186 | none |
| show | 186, 191 | none |
| show projects | 193-194 | none |
| show project | 193 | none |
| show targets | 195 | none |
| inputs | 222 | 1 EN-only note; Bazel implementation is `missing` |
| reset | 174 | none |
| tools | 158 | none |
| tools setup | 159 | none |
| preflight | 97 (runAdmittedBuildInvocationV1) | none |
| incremental | 98, 📏️incremental/⌨️command | none |
| incremental list / plan / prepare / run / discard | same module, operations list | none |
| dependencies | 131 | none |
| dependencies sync / lock / ensure | 131-135 | none |
| cargo | 137 | none |
| cache | 149, 180 | 6 EN-only notes |
| cache status / clear | 180-181 | none |
| cache remote / serve | 149 | none |
| capacity | 259, 🌡️host-capacity/⌨️command:53-54 | 4 EN-only notes |
| capacity status / run | same | none |
| remote-execution | 143, 🏃️execution/🐳️service:205-207 | 4 EN-only notes |
| remote-execution serve / status / env | same | none |

Dispatcher vs catalogue: every catalogue operation and subcommand has a branch. No dispatcher branch names a command missing from the catalogue. `dashboard` (`run @semio-tech/repo-dashboard-rs:…`) and `viz` (Print target suffix in 🧭️invocation) are targets, not commands.

## 3. Missing EN/DE texts

- Catalogue summaries, descriptions, examples, labels, subcommand texts: no missing EN or DE entry, no identical EN=DE pair.
- `notes` arrays (23 strings in run 2, affected 4, exec 2, inputs 1, cache 6, capacity 4, remote-execution 4) are English-only, and the renderer never prints them. DE users cannot see them; EN users cannot either.
- Not rendered at all: `dryRun`, `progress`, `cancellable`, `implementations`. Consequence: `repo inputs --tool=bazel` is unsupported (`missing`) and the help does not say so.

## 4. Untranslated strings

- No EN text in DE output and no DE text in EN output beyond literal values (`on`, `off`, `all`).
- Loanwords kept in DE: "Task(s)", "Target(s)", "Policy", "Locale", "Logs". Acceptable but inconsistent with "Ziel"/"Protokoll" if the glossary wants German terms.

## 5. Option names: help vs catalogue

- All 78 pages match the catalogue: the flag set per page equals the operation's flags plus global flags, and aliases (`-c`, `-p`, `-t`, `--target`, `--exclude-task-dependencies`, `--output-style`, `-h`) render correctly. No name mismatch found.
- Subcommand pages list every parent option and the parent's examples, not their own. Examples:
  - `incremental discard` shows `--scenario`, `--implementations`, `--phases`, `--fresh` and the list/run examples.
  - `dependencies ensure` shows `--all` and `--scope <owner>`. Staged result-conc-deps and result-cli-zt record that ensure refuses both.
  - `cache status` shows `--port`, `--host`, `--directory`, `--max-bytes`.
- Operation synopsis `dependencies sync|lock …` omits `ensure`, which is a third subcommand.
- Subcommand `cache remote` has synopsis `cache remote status`; the trailing `status` is not its own subcommand.
- Parser: `--tool` and `--lang` consume a separate value, `--workers` does not. `bun repo --workers on show --help` fails with the internal message "Undeclared original Repo build selection". `bun repo --tool bazel show projects --help` works.
- Unknown command: `bun repo nosuch --help` prints a clear "Unknown command" error listing the 17 operations, exit 1.

## 6. Tool-specific env vars and "nx is default"

- No `NX_*`, `BAZEL_*` or `CARGO_*` env var appears in any page. The earlier `NX_PARALLEL` mention in capacity is gone.
- Neutral env vars in every page (global flags): `SEMIO_BUILD_TOOL`, `SEMIO_BUILD_WORKERS`, `SEMIO_LANGUAGE`. `SEMIO_BUILD_MODE` appears in root, run, run-many, affected, cargo (EN and DE).
- "Policy default (nx)" appears on all 78 pages (global `--tool` line) plus the root tagline "Nx with Bun (default)". Matches `🔣️policy.json` `defaultTool: "nx"` today. The staged Bazel flip would make this wrong on every page.
- Tool names in neutral pages: `tools setup [nx|bazel|all]`, `cache` ("for Bazel and Nx"), `incremental` (compares Nx and Bazel, by design), `--graph` text ("Nx: …, Bazel: …"), `inputs` example ("the inputs Nx hashes"), `cache clear` example ("Empty the Nx task cache"), `reset` example ("Reset the Bazel state"). Examples only; no `--tool` leak in option names.
- Hard-coded `/tmp/sinc` default for `incremental --root` (EN and DE, every incremental page), with `<drive>:\sinc` on Windows. Not portable to macOS temp dirs as a documented default.
- Language fallback: `--lang` falls back to "the first catalogue language" (en). AGENTS says no default language, SHOULD use English first. Decide whether this fallback is acceptable.

## 7. Staged 1020d/e changes not live

Live = in result-integrate2.md "Applied" list. Not live = in its "NOT applied" list or never listed.

- cache gc: `repo cache status|gc [--json] [--dry-run] [--area] [--lang]` (result-cache-gc.md, READY, not applied). Live catalogue has no `gc`.
- Bazel default flip: policy `defaultTool` nx to bazel, tagline and `--tool` text, help snapshots (result-bzl-default.md flip group). Not applied per integrate2 ("NOT applied: flip"). The prep and setup groups are applied.
- dependencies ensure fix (result-conc-deps.md, 28 anchored ops, not in the applied list): removes the exclusive-lease stall. Help still lists `--all/--scope` on ensure.
- cli-zt (result-cli-zt.md): listed as NOT applied in integrate2, but its effects are visible live (`dependencies ensure`, help for capacity and inputs now works). Ledger status is ambiguous.
- Not in the applied list, command impact not checked: conc-harness, conc-nx, conc-leases, conc-cargo, gran-keys, bzl-hermetic.
- graph-speed: its own result says "APPLIED live 19:0x", but integrate2 lists it as NOT applied. Conflict between result files.
- Applied live (matches help): mem-gov (capacity), bzl-rbe (remote-execution), parity (inputs), bzl-incr (incremental), sync-incr, bzl-default prep/setup/commands.

## 8. Findings to act on (summary)

1. Notes are English-only and not rendered: add DE, render them or delete them.
2. Subcommand help shows parent options and examples: scope them per subcommand (ensure must drop `--all/--scope`).
3. `dependencies` synopsis omits `ensure`.
4. "Nx is default" appears on every page: move to the flip decision (the flip is staged, not applied).
5. `--workers <value>` separate form crashes with an internal error: make the parser skip it like `--tool`, or reject it cleanly.
6. Hard-coded `/tmp/sinc`: derive the default from the platform temp dir.
7. `inputs` Bazel `missing` is invisible in help: render `implementations`.
8. Decide the `--lang` fallback against the AGENTS no-default-language rule.
