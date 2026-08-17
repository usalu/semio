# Lane 0-B report — Hub port/env/boot defects

## Changed files

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`
   (~line 1951-1952) — `OS_HUB_PORT` `6070` → `8787`, docstring updated to say why (6070 is the `s`
   react playground's port, per `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml`
   `[[package.metadata.semio.playground]] ports = { react = 6070, wgpu = 6066 }`).

2. `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — two fixes, both confirmed necessary by reading
   `runCargo`/`runCmd`/`runCmdInternal` in the library `📦️index.ts` (`opts.env ?? process.env` — a
   passed `env` object **replaces** `process.env` wholesale, it does not merge):
   - `DevScript.run()` was passing `{ [OS_HUB_PORT_ENV]: String(OS_HUB_PORT) }` as the entire env,
     silently dropping `PATH`, the launcher's `OS_HUB_DATA`, and any launcher-set `OS_HUB_PORT`
     override. Fixed to `{ ...process.env, [OS_HUB_PORT_ENV]: process.env[OS_HUB_PORT_ENV] ??
     String(OS_HUB_PORT) }` with a class-level docstring explaining the inheritance contract (no
     inline comment, per CLAUDE.md).
   - **Second, previously-unreported defect found while verifying**: the module import at the top of
     this file used 5 `../` segments (`"../../../../../🧰️framework/..."`), but
     `🌎️hub/📦️packages/🦀️rust/` is only 3 levels below the repo root, so it resolved to
     `/Users/ueli/🧰️framework/...` (outside the repo entirely) and made `bun nx run os-hub:build` /
     `:dev` fail immediately with `Cannot find module`. Confirmed via `git log -p` that this file was
     authored when the library package sat one level deeper (`📚️lib/⚡️implementations/...`) and the
     `../` count was never adjusted through the later `⚡️implementations` removal and `lib` →
     `library` renames. Cross-checked against sibling `📜️script.ts` files at the same nesting depth
     (e.g. `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`, which correctly uses 5 `../`
     for its depth-5 location) — the convention is "N `../` for N levels deep," and hub is depth 3.
     Fixed to `"../../../🧰️framework/..."`. This was blocking `bun nx run os-hub:build`/`:dev`
     unconditionally (independent of the port/env bugs), so it had to be fixed to complete this
     lane's own verification step; it is a one-line change in the same leased file.

3. `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` (line 819) — `unwrap_or(6070)` → `unwrap_or(8787)`. Touched
   only that literal; no doc comment named 6070 near it. Re-read the region immediately before
   editing (last commit on this file was 2026-08-12, days old, not the "within 30 minutes" caution
   case) and again confirmed no collision after: the file was auto-committed at
   `e648c495c2` (2026-08-16 21:52) together with the peer MUTATION-OUTCOMES lane's
   `merge_policy_from_env`/`ApplyOutcome` work, and `git show e648c495c2 -- 📦️bin.rs` shows only the
   expected one-line `6070`→`8787` diff plus the peer's unrelated additions — no overlap.

## grep for other 6070 references

`grep -rlw "6070"` (word-boundary, code file types) turned up, besides historical ticket-folder
scratch files and `.cursor/plans/*.md` (left untouched — historical/out of scope):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:2038`
  — `process.env.S_OS_PORT ?? "6070"` — the `s` react playground's own default port. Not the hub.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:2328` — a unit test using
  `"remote://host:6070"` as an arbitrary string to test `hub_ws_url`'s parsing; not a hub default.
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml:21` — `ports = { react = 6070, wgpu = 6066 }`
  — explicitly the `s` playground per the contract-freeze doc; untouched by design.
- `.vscode/launch.json:2698,2708` — the `🛠️dev🖥️s⚛️react` entry's `S_OS_PORT: "6070"` and its
  `serverReadyAction` regex — again the `s` playground, not the hub.

None of these are unambiguously about the hub; none were changed. `.vscode/launch.json`'s
`🛠️dev🗄️os-hub` entry (lines 2767-2775) already had `OS_HUB_PORT: "8787"` / `OS_HUB_DATA:
"${workspaceFolder}/.semio/hub-dev/"` before this lane started.

## Commands run + results

- `cargo check -p semio-hub` (default features, no `--all-features`): **succeeded** —
  `Finished \`dev\` profile [unoptimized] target(s) in 9.20s`.
- `cargo check -p semio-hub --all-features`: **fails**, but on a pre-existing, unrelated defect
  outside this lane's lease — `postgres = []` and `neo4j = []` in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml` are declared as empty
  feature flags with no `sqlx`/`neo4rs` wired as optional deps, so `--all-features` tries to compile
  `#[cfg(feature = "postgres"/"neo4j")]` code paths that reference those crates and gets
  `E0433 cannot find crate sqlx`/`neo4rs`. Last touched in commit `20252aa16d` (2026-08-12), well
  before this session. Full log: `🧪️0-b-hub-check.txt`.
- `bun nx run os-hub:build` (the brief's primary verify option): **succeeded** after the import-path
  fix above — `Compiling semio-hub v0.1.0 ... Finished \`release\` profile [optimized] target(s) in
  1m 41s`, `NX Successfully ran target build for project os-hub`. Log tail in `🧪️0-b-hub-check.txt`.
- Boot test: `OS_HUB_PORT=8787 OS_HUB_DATA=/tmp/semio-hub-0b bun nx run os-hub:dev`, run in the
  background, waited out an initial `Blocking waiting for file lock on build directory` (another live
  session's concurrent `cargo` build holding the target lock — expected per project history, waited
  rather than killing anything), then a debug build (`Finished \`dev\` profile ... in 1m 10s`), then
  `Running \`/Users/ueli/Documents/semio/target/debug/os-hub\``. `tracing_subscriber`'s
  `os-hub listening on http://…` line never flushed to the redirected (non-TTY) file, but the process
  is verifiably up: `lsof -nP -iTCP:8787 -sTCP:LISTEN` shows `os-hub  80473 … TCP *:8787 (LISTEN)`,
  and `/tmp/semio-hub-0b/` contains `directory.db`, `db/`, `extension-modules/` — i.e. the launcher's
  `OS_HUB_DATA` was honored (not the `./.semio/hub/` default), proving both the port fix and the
  env-inheritance fix. Process stopped afterward with `kill <pid>` (not `SIGKILL`, no lock held).
  Full transcript + manual verification appended: `🧪️0-b-hub-boot.txt`.

## Blockers / notes (not fixed, out of lease)

- `postgres`/`neo4j` feature-gating bug in `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml`
  (empty `[]` feature entries, no optional `sqlx`/`neo4rs` deps) blocks any `--all-features` build of
  anything depending on that crate, including `semio-hub`. Outside this lane's 3-file lease; flagging
  for whichever lane/coordinator owns that crate.

## sharedFileRequests

None — all three lease files were edited without needing anything outside them, except the
already-covered `📜️script.ts` import-path line (same file, same lease).

## What is NOT done

- Did not attempt `cargo check -p semio-hub --all-features` fix (out of lease, see Blockers).
- Did not touch `.vscode/launch.json` — its `🛠️dev🗄️os-hub` entry was already correct and is now
  confirmed to actually reach a working process end-to-end.
- Did not touch any `s` playground files/ports (6070/6066), per contract.
