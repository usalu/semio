# 🧭 Wave 1122-dist preamble (binding for every lane)

Goal (user): a semio-tech build is fully distributable: every compile unit (native + wasm32, library units + component finals) can be offloaded to any ready fleet worker via remote execution; ONE fleet cache keyed only by the coordinator's synced (checked-in) sources + toolchain/flags, so a crate compiled anywhere is never compiled again anywhere; entries for sources no longer current are evicted; proven with numbers (second build on another worker: 100 % hits / 0 compiles; one-crate edit recompiles only that crate + dependents).

Repo: `/Users/ueli/Documents/semio` — ONE shared live worktree replicated to 50+ machines within ~1 s.
- NEVER run git commands that modify anything (commit, stash, checkout, reset, restore, clean, worktree…). Read-only git (grep, log, diff, show) is fine.
- Re-read a file right before each edit; use small anchored Edit calls, never whole-file Write on existing files. Edits must be compile-atomic (a cold native release build of 🏢️semio-tech/🎡️play is running from this tree right now — do not break it, do not kill `native-chain.sh`, `native-play.sh`, `native-build-launch` or their cargo/rustc children).
- Use the `repo` binary (or repo MCP tools) for build/test; `repo test-file <file>` runs one test file. macOS has no `timeout` command. Run long commands in the FOREGROUND (background children die with your turn).
- Do NOT spawn sub-agents. Do NOT open/close/reopen tickets. Do NOT delete or sweep any `🗑️generated` folder.
- Ticket folder: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/` (T). Your wave folder: `T/wave-1122-dist/`. Prior maps worth reading: `T/map-remote-cache.md`, `T/map-remote-execution.md`, `T/map-build-path.md`, `T/map-cas-gc.md`, `T/audit-computed-twice.md`, `T/audit-wasm-and-still-twice-1114.md`, `T/design.md`, `T/landings.md`, `T/wave-1122/coordination.md` (tail), `T/wave-1122/result-E74.md`, `result-E65.md`, `result-E66.md`.
- Paths: LIB = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`; unit cache UC = `LIB/⚡️caching/🦀️cargo/🗃️unit-cache/`; remote cache = `LIB/🌐️remote-cache/`; remote execution = `LIB/🌐️remote-execution/` (verify — paths may have moved).
- Fleet: scheduler http://192.168.178.70:8981, REAPI gRPC :8980, fleet cache http://192.168.178.70:6094. 11 darwin-arm64 workers registered.
- Code style (CLAUDE.md): concise code, no comments inside definitions, docstrings start with a unique emoji, schema-first, EN+DE texts, no legacy/compat layers, `[DEBUG] ` prefix on temporary logs.
- Landing rule for code lanes: before editing a build-system file, `mkdir T/wave-1122-dist/lock.<area>` with an `owner` file (your lane id + time); remove after landing. Append each landing to `T/landings.md` (time, files, sha256 before/after, tests run). Run the test files of every touched member before releasing the lock.
- Report: write `T/wave-1122-dist/<lane>.md` (findings with file:line, numbers, what you changed, tests run with pass/fail counts, open items). Final message: 5 lines max + the report path.

## Binding addendum 21:08 (play release session request)
- While the play release chain runs (until a `[chain exit` line appears in `.🧬semio/🌐hub/play-cdn-1122/native/supervisor.out`), NEVER run eviction/GC/collection against the live fleet cache server store (`.cache/b/p/m/fleet/cache-server/v1`) or the local unit cache. New retention code must land disabled or in dry-run mode against live data; prove it on scratch stores in tests.
- Coordinator disk is ~94 % (≈56 GiB free): do not grow caches; keep experiment stores small and under the ticket `🗑️generated` or the session scratchpad.

## Binding addendum 21:55 (host ENFILE incident ~21:20 killed the play build)
- Never run `bun test` / `bun` with a bare path (scans the whole checkout) — only `repo test-file <file>`. No full-tree walks, hashers or cache scans over the checkout (`find .`, `git grep` without pathspec on huge trees is OK only read-only and once). Scope every walk to the member folder.
- Check `sysctl kern.num_files` before any heavy step; if > 60000, wait.
- New code that walks trees (retention digest index, input shipping) must bound open files (sequential or ≤ 64 concurrent handles).

## Binding addendum 22:20 (host load 63/10 cores, play attempt 3 in flight)
- Until `[ok] site-build` in the play supervisor.out: at most 1 local cargo/test run per lane, always prefixed `taskpolicy -b`.
