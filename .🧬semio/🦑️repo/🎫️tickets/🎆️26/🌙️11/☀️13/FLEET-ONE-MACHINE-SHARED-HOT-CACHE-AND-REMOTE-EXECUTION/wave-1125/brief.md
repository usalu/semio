# 🚀 Wave 1125 Brief: Finish the Fleet Setup, Every Worker Ready

Coordinator: main chat (Opus 5.5 medium). Executors: Sonnet 5.5 medium lanes `F*`. Auditors: Haiku 5.5 medium lanes `A*`. Started 2026-11-25 16:30 CET.

`T` = this ticket folder `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/`, `W` = `T/wave-1125/`, `S` = `/Users/Shared/semio-store/v1/` (fleet store; `S/origins/<origin>/ready.json`, `ready-log-<stage>[-failed].txt`, `memory.json`, `attest.json`).
`B`, `C`, `HC` as in `T/wave-1122/brief.md`.

## Rules
ALL hard rules 1-12 of `T/wave-1122/brief.md` and ALL its addenda (per-area landing locks `T/landing.lock.<area>/owner`, Bash report files, no whole-file writes of shared files, rule 10 = `repo test-file`, claims file, load gate 40 / memory gate 25 % free) are binding, with these replacements:
- Report/result files go to `W/result-<lane>.md`, findings to `W/findings-<lane>.md`, generated output to `W/🗑️generated/<lane>/` (delete when done), kept input scripts to `T/🧰️scripts/<lane>/`, landings to `T/landings.md` with header `## Wave 1125 <lane>: <title>`, test claims to `W/claims-tests.md`.
- Scratch root: `/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/<lane>/`.
- No sub-agents, foreground only, no background processes, never pkill/killall, never git commands that modify anything, never edit AGENTS.md, never open/close/reopen tickets or goals.
- The live worktree is `/Users/ueli/Documents/semio` (the old `/Users/Shared/semio` checkout is gone). It replicates to ~14 machines in ~1 s and their supervisors restart on code change: land compile-atomic, small anchored edits.

## Live state at 16:27 (measured from S/origins/*/ready.json)
Ready 1/14 (g1-m07, refreshing workspace). Failure classes:
1. workspace stage on m01, m02, m10, m12, m14 (and likely the running m03/m06/m08/m11/m13): `@semio-tech/framework-rs:generate:prototype` fails in `framework:cargo` source admission with `refused process.source.crate-refused {"crate":"semio-framework-value-native-receiver-protocol","reason":"Cargo Source outside admitted owner"}` (stack: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts:28`). m02 additionally fails `@semio-tech/ui-styling-tokens:generate:prototype`.
2. m04 (Linux): dependencies stage, `disk-short`; dependency installer `error: failed applying patch file: EACCES: Permission denied (open())`.
3. m12 `disk-unfit` waiting; m05 (Windows) dependencies attempt 46; m09 has no ready.json at all.

## Addendum 17:30: no dangling imports
A new file landed with relative imports one `../` short and broke the resident graph service for the whole fleet (BUILD_SOURCE_UNPARSEABLE). Before releasing any landing lock, run `python3 T/🧰️scripts/coordinator/dangling-imports.py <each folder you touched>` from `/Users/ueli/Documents/semio`; it must exit 0. Create new files compile-atomically: write them complete and resolvable, never empty or half-wired.

## Addendum 18:00: QUIET WINDOW 18:15-19:00
Every landing in build-system code (B, C, HC, 🏃️process, 🚦️ready, graph) and every Cargo.toml/package.json edit restarts worker supervisors and restarts slice passes on 14 machines, so workers never converge. From 18:15 to 19:00: NO landings of product code in those areas and NO manifest edits. Allowed: test-only edits (files under 🧪️tests), reading, measuring, staging patches in T/🧰️scripts/<lane>/. Land staged product patches after 19:00. Exception only by coordinator message.

## Addendum 19:15: atomic writes, no live experiments
Write every source/manifest atomically (temp + rename); a briefly invalid Cargo.toml killed a 2-h whole read. Never disable or alter code in the live tree for an experiment; A/B in a scratch copy.
## Addendum: type check before releasing a landing lock (F65)

Before releasing ANY landing lock, run `repo typecheck <every path you touched>` from `/Users/ueli/Documents/semio`; it must exit 0. It checks the touched TypeScript sources and the files that import them directly (one incremental compiler call, no graph read; a repeat takes seconds, the first call that reaches a new large closure can take about a minute) and compares every touched hand-written `.d.mts` with its `.mjs`.

- Paths: files or folders, relative to the checkout or absolute. A touched `.mjs` stands for its `.d.mts`: when you add, remove or re-sign an export of a `.mjs`, update its `.d.mts` in the same landing (every export declared, nothing declared that the script lacks, same parameter count).
- Exit 0 clean, 1 type errors in a named file or an importer, or a declaration drift, 2 refused (bad path, compiler missing). Errors in other files the program reads are counted as "elsewhere" and do not fail the call; `--all-errors` counts them.
- `repo typecheck --declarations` compares every `.d.mts` pair of the library (about 1 s, no compiler); `--dry-run` lists what would be checked; `--json` prints one structured result (MCP tool `typecheck`, argument `files`).
- Lanes have so far checked with `bun build --no-bundle`, which does not type-check: that is no substitute. `bun test` does not type-check either.
- The build state lives in the machine cache (`SEMIO_TYPECHECK_CACHE` overrides it); nothing in the checkout is written. If a call reports "importers found, only the touched files are checked", the change is widely imported: also run `repo typecheck` on the importing folder you expect to be affected.

## Addendum 05:40: suite tables and project targets
A suite added to any `🗂️suites/🔣️.json` without exactly one matching `test-<id>` target in its `📋️project.json` breaks graph inference of the owning project on every worker (05:30: Ready 10 → 3). When you add, rename or remove a suite or test member: update the suite table AND the project target in the same atomic landing, then run `repo test-file 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/🧪️tests/🟦️.ts` (route table law) before releasing the lock.

## Addendum 14:55 (11/26): Lock Hygiene
- Take a landing lock only for the moment you write files in its area; release it right after the atomic write. Never hold a lock across a test run, a typecheck, or a wait.
- A lock older than 20 min whose holder is not writing may be reported to the coordinator; do not steal it yourself.
- Append to `landings.md` only (never rewrite the whole file).
