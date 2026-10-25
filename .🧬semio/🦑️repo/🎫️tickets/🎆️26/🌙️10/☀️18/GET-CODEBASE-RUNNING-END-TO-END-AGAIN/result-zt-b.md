# result-zt-b (executor zt-b, wave 1023b)

STATUS 03:05 (10/23): DONE, LIVE. Any bun on PATH (1.3.14 here) now runs repo scripts and `bun test` as the pinned 1.4.2; `repo tools verify` is clean (20 of 20, exit 0) and names the running Bun (EN/DE).

## For executor repo-tests-b (pinned bun is now the default)
`bun test ./<file> [flags]` through ANY bun (e.g. `~/.bun/bin/bun` 1.3.14) started in the repository root re-runs itself, with the exact flags, under the pinned 1.4.2 (stderr shows two `bun test v...` header lines, the second is 1.4.2; no segfault path). The documented tool-neutral form is `repo tools exec --light bun -- test ./<file>`. Needs: cwd = repo root (bunfig.toml is read only from the start directory), pinned bun provisioned (`repo --version` says "ready"; else `repo` on PATH provisions it).

## Trace (where bun was resolved)
| Entry | Before | Now |
|---|---|---|
| Go launcher `repo`, `repo ./x.ts` | pinned (launcher-zt) | unchanged; runtime dir leads the child PATH |
| `bun repo ...` (package.json script = `repo <door>`) | pinned via launcher; 127 if `repo` not on PATH | unchanged (launcher-first keeps the resident fast path); no-`repo` case = stage-0 installer, open item 1 |
| nested spawns (675 `process.execPath` sites, tool registry `bun` = runtime locator = execPath) | pinned iff the entry was pinned | pinned always: guard re-exec puts the pinned dir first on PATH (proof below) |
| `bun ./x.ts` / `bun -e` / `bun test` with a host bun | host bun (1.3.14, segfaults) | guard re-runs the exact command line under the pinned bun |
| e2e driver preflight `orch.py` | WARN "bun version 1.3.14 (packageManager 1.4.2)" | checks the launcher-provisioned runtime + proves a PATH bun runs as pinned |
| `repo tools verify` | no statement of the running bun unless it differed | always prints "Running Bun X (path); pinned Bun Y." / "Laufendes Bun ..." |
| git hooks (checked-in `🪝️hooks/*`, stale Sep-14 copies with `compose_resolve_bun`/`command -v bun`) | PATH bun | regenerated from `renderMicroCommitGitHook` (repo client only, no bun); `.git/hooks/*` NOT touched (local config; `repo config` regenerates) |
| Rust twins (`system-verbs` `look_path("bun")`, `hooks/system-ports` `bun_candidates`, BUN_INSTALL first) | host bun | NOT edited (cargo gated, unverifiable here); functionally covered: they run `bun ./📜️script.ts` with cwd = repo root, so the bunfig guard re-runs it pinned |

## Design (one schema-declared pin, two implementations, one fixture)
* Pin = launcher registry `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json` (bun 1.4.2 + per-host SHA-256; toolchain-pins law ties it to package.json). Go launcher reads it (`runtime-provision.Locate`); the new TS guard `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📌️runtime-pin/{🟦️.ts engine, ⌨️command/🟦️.ts system host, 📜️script.ts preload}` reads the same file: same host selection (platform/arch/libc), same stores (tools partition `.cache/b/t/m/bun/<sha>`, then user cache `SEMIO_RUNTIME_CACHE`/OS cache dir), same install verification (regular executable, exec bit, receipt `.toolchain.json` naming the archive, size when recorded).
* `bunfig.toml` (root): `preload` (scripts, `-e`) and `[test] preload` (test runner; verified empirically that top-level preload does NOT cover `bun test`) = the guard.
* Guard decision (`guardDecisionV1`, table of 64 combinations in the fixture): running == pin -> proceed; guard marker `SEMIO_RUNTIME_GUARD` set -> refuse (no loops; marker cleared in the pinned child so later nested starts are guarded again); test runner without readable command line -> refuse; provisioned -> re-exec exact command line (Linux `/proc/<pid>/cmdline`, macOS `KERN_PROCARGS2` via `bun:ffi`, Windows `GetCommandLineW` + C-runtime splitter), exit code and signals forwarded, pinned dir leads PATH; else `repo` on PATH gets the script as script route / tests via `repo tools exec --light bun -- test ...` (it provisions, SHA-256 pinned); else EN/DE refusal (exit 2) with the install command. No language assumed (both shown when unresolved). Guard errors fail open with one `[runtime guard] skipped` line.
* Registry gains EN/DE texts `guardTest`, `guardMissing`, `guardLoop` (Go `RequiredMessages` extended so completeness is enforced there too).
* Hermetic runtime inputs (`🏗️build/🏃️execution/🔒️hermetic/🔣️.json`) gain the guard dir, registry and language protocol: a sandbox that copies `bunfig.toml` would otherwise die with "preload not found" (verified bun behaviour).

## Changed files
New: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📌️runtime-pin/{🟦️.ts,⌨️command/🟦️.ts,📜️script.ts}`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧫️fixtures/📌️runtime-pin/🔣️.json`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📌️runtime-pin-contract/{🟦️.ts,🐹️.go,🥒️.feature,🐍️oracle/🐍️.py}`.
Edited: `bunfig.toml`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🗂️registry/🔣️.json` (3 messages); `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go/🗂️runtime-registry/🐹️.go` (RequiredMessages); `🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/README.md` (EN+DE section "Runtime guard"/"Laufzeit-Wächter", tests list); `🏗️build/🧰️tool-setup/{🔣️.json,🧬️schema/🔣️.json,🟦️.ts,⌨️command/🟦️.ts,🧪️tests/🟦️.ts}` (running-runtime line); `🏗️build/🔌️backends/{🟦️.ts,🧪️tests/🟦️.ts}` (bug below); `🏗️build/🏃️execution/🔒️hermetic/🔣️.json`; `🧰️framework/🛍️products/🦑️repo/🪝️hooks/{post-checkout,post-commit,post-merge,post-rewrite,prepare-commit-msg}` (regenerated); `C/e2e-orch-tools/orch.py` (new `runtime_checks`, replaces the "bun version" WARN).
Tools: `C/zt-b-tools/{gen_fixture.py,nested.ts,render-hooks.ts,law-spawn.ts}`.

## Bug found and fixed on the way (not in the goal text, blocked verify)
`repo tools setup toolchains` and `repo tools verify <tool>` died with "Unknown build tool: toolchains/bun (nx|bazel|semio)": `buildBackendBootstrapV1` treated `args[2]` of ANY `tools` subcommand as a backend id. Now only `tools setup <backend|all>` does; `tools setup toolchains` and every other subcommand launch no backend tool. Test rows added to the backends suite (8 pass).

## Evidence (all live, macOS arm64, host bun 1.3.14 at ~/.bun/bin/bun, load ~9)
* `repo --version` and `bun repo --version`: identical, "runtime bun 1.4.2: ready, .cache/b/t/m/bun/90987a3a.../bun".
* Nested spawn (`C/zt-b-tools/nested.ts`: prints `process.versions.bun` and runs `sh -c 'bun --version'` with cwd /tmp): `repo ./nested.ts`, `~/.bun/bin/bun nested.ts`, `bun nested.ts`, `env -i PATH=/usr/bin:/bin ~/.bun/bin/bun nested.ts` all print entry=1.4.2 nested=1.4.2.
* `repo tools verify --lang=en|de` (logs `C/🗑️generated/zt-b/verify2.log`): "All 20 pinned tools match their pins." / "Alle 20 gepinnten Werkzeuge entsprechen ihren Pins.", exit 0, then "Running Bun 1.4.2 (<path>); pinned Bun 1.4.2." / "Laufendes Bun ...". chromium was the 20th: this shell exports PLAYWRIGHT_BROWSERS_PATH=<repo>/node_modules/.cache/ms-playwright (from ~/.zshrc, not touched); `repo tools setup toolchains` downloaded the pinned 1248 there (setup-toolchains-3.log).
* Tests (all with pinned bun, and the runtime-pin suite also through host 1.3.14 -> guard): runtime-pin-contract 15 pass (incl. live: PATH bun 1.3.14 runs a script and a `bun test` file as 1.4.2; independent Python oracle equal on select/locate/cache/decide/split/procargs/cmdline; Go launcher agrees on the real path via `repo --version`), tool-setup 22, backends 8, registry-schema 5, toolchain-pins 12, hermetic 7, direct-tool-declarations law 14; raw-process-spawn law 0 breaches. Go launcher suite (full, flattened overlay with the pinned Go 1.26.9, `go test ./...` + vet): ok, incl. the new Go twin of the fixture (`TestRuntimePinSelectionAndLocation`, `TestRuntimePinUserCacheDirectory`). The graph run `repo run @semio-tech/repo-launcher-go:test` was queued 13 deep behind the fleet and was killed by me (pid 32484); not needed.
* `sh C/e2e-orch-tools/run.sh course --phase preflight --run-id ztb-pre`: verdict OK; "pinned bun OK 1.4.2 provisioned: <path>", "bun runs pinned OK bun on PATH is 1.3.14 (...) and runs the checkout as 1.4.2 (pinned 1.4.2)" (report file removed).

## Open / not verified
1. A dev with a bun but NO `repo` launcher: `bun repo` -> `repo: command not found` (127). Deliberate: the package script stays launcher-first (resident fast path); stage 0 is the installer (README). `bun ./x.ts` in that case is refused by the guard with the install command (or re-run pinned if the tools partition already holds it).
2. Linux, Windows, devcontainer code paths (/proc reader, GetCommandLineW/FFI, Windows splitter, PATH delimiter) are fixture-tested (splitter against list2cmdline + MS doc examples) but never executed there. The FFI readers are only exercised on macOS.
3. Rust twins unchanged (see table). If you want them off `BUN_INSTALL`/`look_path`: have `micro_commit` start `current_exe()` with `./📜️script.ts micro-commit ...` like the Go twin; needs a cargo check.
4. bunfig is read only from the directory a bun starts in: `cd sub && bun test x` is not guarded (use repo root or `repo tools exec`).
5. e2e-orch `selftest.py` has 2 pre-existing failures unrelated to bun (plan.schema strictRequired "minFreeDiskGbCold").
6. `.git/hooks/*` still hold the old stale hook text (not touched on purpose).
7. No process of mine is left running.
