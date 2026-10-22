# result-json-stdout (executor json-stdout, wave 1023a)

Bug: `bun repo show targets --json` (and every `--json` command) printed waiting/progress text on stdout, so the document was invalid JSON under contention.

## Root cause (three layers)
1. The central progress writer (`🗣️language/📣️progress`) sent every `info` event to stdout (`console.log`) and only `warn`/`error` to stderr. Every `sayBootstrapV1`/`sayVerifyV1`/`sayCommitV1` line was therefore a stdout line.
2. Individual writers chose stdout on their own: dependency-build wait reporter (`Another process is building the dependencies ...`, `writeOwnedCommandOutputV1(process.stdout, ...)`), lease fallback (`Waiting for <mode> access to ...`, `console.log`), `capacity run` (the result writer was handed to the capacity gate as its progress writer, so `[capacity] waiting ...` went to stdout), plus about 100 `console.log("[tag] ...")` progress lines across commands.
3. There was no one output contract: result and diagnostics shared stdout by convention only.

## The contract (schema first)
`🧰️framework/🔨️modules/🏃️process/🏗️build/📤️output/` (`🔣️.json` protocol, `🧬️schema/🔣️.json` schema, `🟦️.ts` implementation, `🚪️entry/🟦️.ts` side-effect installer, re-export in `📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts`):
- stdout carries only the command result; every progress, waiting, log and other diagnostic line goes to stderr.
- Under the machine flag (`--json` before the `--` separator) `installOutputContractV1` diverts, process wide, `process.stdout.write` and `console.log|info|debug|table|dir|dirxml` to stderr. The only way to the real stdout is the result writer (`writeResultLineV1` / `machineOutputV1`). So no call site, present or future, can corrupt the document; a missed result emitter fails loudly (empty stdout), not silently (mixed stdout).
- The door script imports `📤️output/🚪️entry/🟦️.ts` as its first import, so the contract is installed before any other module evaluates (Bun's `console.log` bypasses `process.stdout.write`, so both are patched). `runBuildMainV1` installs it too (idempotent, state on `globalThis[Symbol.for("semio.output.contract.v1")]`).
- The progress writer is now stderr only for all levels (`ProgressWriteV1 = (line) => void`, `processWriteV1 = writeDiagnosticLineV1`); the progress schema/corpus no longer has a stdout stream.

## Changed files (all under `🧰️framework/`)
New: `🔨️modules/🏃️process/🏗️build/📤️output/{🔣️.json,🧬️schema/🔣️.json,🟦️.ts,🚪️entry/🟦️.ts,🐍️... see tests}`, `🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/📤️output/🟦️.ts`.
Progress writer: `🏗️build/🗣️language/📣️progress/{🟦️.ts,🧬️schema/🔣️.json,🧪️tests/🟦️.ts,🧪️tests/🔣️.json}`, `🗣️language/🚨️error/🧪️tests/🟦️.ts`.
Diagnostics to stderr: `🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` (reporter exported for its test), `🔨️modules/🏃️process/🔒️leases/🟦️.ts`, `📚️library/🏗️build/🌡️host-capacity/⌨️command/🟦️.ts` (`capacity run` gate gets `writeDiagnosticLineV1`).
Result writer (replaces `console.log`/`process.stdout.write` for results): door (`🚪️door/🟦️.ts`, `🚪️door/📜️script.ts`), `🏗️build/📜️script.ts`, `⌨️command/⚙️runtime/🟦️.ts` (show/graph document via `machineOutputV1`, resident `stdout:`), `⌨️command/💾️cache/🟦️.ts`, leases/doctor/cache-gc/tool-setup commands, incremental, conformance, units, parity, the three `⚖️laws` commands, verify taxonomy-lanes and interactivity-lanes, cache pruning report, cargo provenance, authoring command.
Startup law budget: `🚪️door/🧪️tests/🚦️startup/{🟦️.ts,🔣️.json}` (door now loads 2 more modules: 11 -> 13; the static-closure comparison ignores the entry installer like it ignores `main`). Parity test captures `process.stdout.write` instead of `console.log`.
E2E driver: `e2e-orch-tools/orch.py` `capture(..., machine=True)` keeps stdout and stderr apart for `show targets|projects --json`; `json_part` is kept (still tolerant) and now sees a pure document.

## Tests (language-agnostic corpus + independent oracle)
- `📤️output/🧪️tests/` (7 tests): corpus `🔣️.json` (schema `🧬️schema/🔣️.json`, Ajv strict), Python oracle `🐍️oracle.py` (own argument parser, `json.loads` for result streams), child fixture `🎭️fixture/🟦️.ts`. Cases: flag detection (before/after `--`, `--json=1`, `--json-lines`), child process stdout/stderr per mode (noise via console.log/info/debug/process.stdout.write; progress event is stderr with and without the flag).
- `🚪️door/🧪️tests/📤️output/` (4 tests): real door with a noisy `--preload` (starts after the contract is installed): `capacity status --json`, `leases status --json` stdout parses (Python `json.loads`), noise on stderr; control without the flag keeps noise on stdout. Forced capacity wait: fixture drives `withBuildCapacityV1(["show","projects","--json"])` with a fake governor (availability 100 MB until a release file appears); the test waits for `[capacity] waiting` on stderr, then releases; stdout is exactly the JSON result, stderr has `[capacity] waiting`, `[capacity] admitted show`, and the body noise.
- `…/💾️storage/🧪️tests/📣️report/🟦️.ts` (3 tests): dependency wait reporter writes to stderr only (EN/DE).

## Commands and outcomes
- `bun test ./<file>` for: output unit (7 pass), door output (4 pass), storage report (3 pass), progress (8), error (9), door (7), door startup (5), parity (13), laws x3 (14/10/9), conformance (14), cache-gc (19), doctor command (9), leases command (4), host-capacity args/gate/main (6/16/18), tool-setup command (5), tool-registry (4), incremental/concurrent/bench (44/24/38), verify/commit/bootstrap texts (3 each): all pass.
- Not mine, failing: `🧩️units/🧪️tests/⌨️command` "catalogue declares exactly the options the parser accepts" (the catalogue row of `--json` lists only `service` in `operations`; peer catalogue edit), `🧪️tests/🧱️root-clean-scaffold-source` (owner projection policy digest drift, artifact env).
- Live: `bun repo capacity status --json 2>/dev/null | python3 -c json.load` OK; `bun repo leases status --json` OK; with a noisy preload both stay pure.
- Live `bun repo show projects --json`: see section "Live proof" below.

## Live proof (show projects --json)
Detached run `json-stdout-tools/verify-show.sh` (wrapper pid 68977, bun wrapper 68982, child 68983), outputs in `🗑️generated/json-stdout/projects.{json,err,rc,verdict}`. At 01:39 the run was waiting for host capacity (`[capacity] waiting 40s for show: ...` on stderr, `projects.json` still 0 bytes: nothing on stdout while waiting).

## Open
- `🏗️build/.profiled-script.ts` (tracked profiled duplicate of the CLI) still has the old `console.log` emitters; it is not executed by any entry I found.
- A child process that inherits fd 1 (`stdio: "inherit"`) is not covered by the in-process diversion; JS-side forwarders (`process.stdout` pipes) are. No such child runs on the `show`/`graph`/status paths.
- Human (non-`--json`) output of long-running commands still mixes progress and report on stdout; the contract guarantee is for machine output. Moving the remaining ~100 `console.log("[tag] ...")` progress lines to the progress writer is a separate sweep.
