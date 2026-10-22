# 🧪️ CLI smoke audit 1022 (auditor `cli-smoke`)

Scope: live repo CLI after round 6 (`result-integrate3.md`, LIVE 10:45). Both entries: native `repo …` (launcher, Go, runtime bun 1.4.2) and `bun repo …` (script form, which the native launcher runs as `repo ./…script.ts repo …`). Logs: `🗑️generated/cli-smoke-1022/<name>.log`. Host `bun --version` is 1.3.14; nested bun is 1.4.2 (`repo --version` prints `runtime bun 1.4.2: ready`; `Bun v1.4.2` footers in `version-bun`, `tools-verify-native`, `tools-verify-bun`).

## Results

| Command | Entry | Exit | Wall | First error line | Nested bun 1.4.2 evidence |
|---|---|---|---|---|---|
| `--help` | native | 0 | 0 s | none | not printed |
| `--help` | bun | 0 | 0 s | none | delegated to native launcher |
| `--version` | native | 0 | 0 s | none (`repo launcher 1.0.0`, `runtime bun 1.4.2: ready`) | yes (`ready`) |
| `--version` | bun | 1 | 1 s | `Unknown repo command "--version"` | footer `Bun v1.4.2` |
| `tools verify` | native | 1 | 22 s | `Expected repo tools setup [nx\|bazel\|semio\|toolchains\|all] or repo tools verify [tool...]` | footer `Bun v1.4.2` |
| `tools verify` | bun | 1 | 17 s | same as native | footer `Bun v1.4.2` |
| `leases status` | native | 0 | 0 s | none (28 held, 2 waiting, 0 deadlock cycles, 10 recent order violations) | not printed |
| `leases status` | bun | 0 | 1 s | none | not printed |
| `cache status` | native | 0 | 15 s | none | not printed |
| `cache status` | bun | 0 | 13 s | none | not printed |
| `capacity status` | native | 0 | 0 s | none (11268 MB available, 4 running, 6 waiting) | not printed |
| `capacity status` | bun | 0 | 0 s | none | not printed |
| `show projects --json` | native | BLOCKED | 200 s (first try, alarm) + 2177 s (rerun, rc 1) | `Original Script process terminated` (killed by coordinator, sh 98496 → 15881; graph re-inference in progress) | n/a |
| `show projects --json` | bun | NOT RUN | n/a | BLOCKED: graph commands not restarted per coordinator | n/a |
| `show targets --json` (arch, play, demo) | both | BLOCKED | n/a | Not run: graph re-inference (coordinator-owned). Note: `show targets` lists all targets, no per-project filter; per-project counts need `show project <name> --json`. Arch run was cancelled by SIGTERM (`Host capacity wait cancelled by SIGTERM`) when the chain was killed. | n/a |
| `dependencies --help` | native | 0 | 0 s | none | not printed |
| `dependencies --help` | bun | 0 | 0 s | none | not printed |
| `incremental list` | native | 0 | 11 s | none (103 log lines) | not printed |
| `incremental list` | bun | 0 | 25 s | none | not printed |
| `concurrency list` | native | 0 | 16 s | none (90 log lines) | not printed |
| `concurrency list` | bun | 0 | 13 s | none | not printed |
| `run @teaching/proctor:build --configuration=release --dry-run` | both | BLOCKED | n/a | Not run: graph commands stopped per coordinator | n/a |

Project and target counts: not obtained (BLOCKED). Per-project target counts for architecture-course, semio-tech-play and mit-bestand-demonstrator: not obtained.

## FAIL list

1. `repo tools verify` (native and bun, exit 1): the advertised subcommand throws. `repo tools verify` is listed in `⚙️operations/🔣️catalog.json` (`tools verify [tool...]`, around line 936) and in the help text, but the live dispatch in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts:68` only accepts `tools setup` (`if(resolved.args[1]!=="setup"…) throw`). Stack traces also show a second `tools` branch that throws the same message. Owner: langs-pin (per integrate3, `tools verify` was theirs). Repro: `repo tools verify`.
2. `bun repo --version` (exit 1): `Unknown repo command "--version"`. The script form of the build CLI has no `--version`; only the native launcher answers it. Either the bun entry should forward `--version`, or the expectation in the brief should be native-only. Repro: `bun repo --version`.
3. Graph-backed commands (`show projects|project|targets`, `run … --dry-run`) could not be verified in this wave. The native `show projects --json` ran 2177 s in the capacity queue plus full graph re-inference, then was killed by the coordinator. Not a code FAIL, but the round-6 smoke is incomplete for these.

## Open / informational

- `show targets --json` has no `--project` filter (`repo show --help`). Counts per project need `repo show project <name> --json` (not yet run).
- `leases status` shows 10 recent order violations and 2 waiting leases at the time of the run.
- Other agents (coordinator pids 25619, 31976; proctor and demonstrator builds) were running at the time; load averages 14–19.
- All processes I started are gone (checked: 98197, 98200, 98496, 15881 not alive).
