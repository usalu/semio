# Runtime and toolchain status (T05)

## repo-mcp install (FIXED)
- Failure: `repo run repo-mcp:install` -> `Native installation refuses a non-file destination`. `~/.local/bin/repo` was a symlink to the launcher `~/.local/share/semio/launcher/bin/repo`, which is the documented single-copy `ln -sf` layout (launcher README). The launcher refuses `repo mcp` and needs the full native command.
- Fix: `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📜️script.ts` now has `nativeRepoLauncherLinkV1`; a symlink resolving to the launcher binary is atomically replaced (rename over the link) instead of refused. Other non-file destinations still refuse.
- Rerun: exit 0, `[repo] Installed native repository command: /home/deck/.local/bin/repo` (now a regular 20 MB file). Log: `🗑️generated/repo-mcp-install-2.log`.
- Verified: `repo mcp` answered MCP `initialize` with protocolVersion 2025-06-18, serverInfo repo, 55 tools (stdin held open 3 s; closing stdin immediately returns `session closed`, expected). Claude Code needs a session restart to reconnect its `repo` MCP.
- Note: the launcher link is gone; the native command embeds the launcher.

## Dashboard / hub target bugs (FIXED, not run)
- Root cause: generator emits `dev-<variant>-react-<prototype|release>`, `-wgpu-`, `run-<variant>-native-<profile>`; dashboard asked for `-dev`.
- Changed `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/📚️registry-record/🦀️.rs` (`playground_target` -> `-prototype`) and its tests/fixtures: `🎮️registry/🧪️tests/🔬️unit/🦀️.rs`, `🧪️tests/🎮️registry/🟦️.ts`, `🧪️tests/🧫️fixtures/🎮️registry/🏗️workspace.json`, `🧪️tests/🧫️fixtures/🗣️launch-axes/🔣️.json`, `🧪️tests/🌳️command-tree-projection/🛝️playgrounds.feature`. Cargo tests NOT yet run.
- Hub: `🌎️hub/🎛️commands/🏗️build-and-dev/🟦️.ts` secure suite now passes `serve s react prototype`.

## App startup
- Real startup: `repo run @semio-tech/framework-os-dev:activate-s-react-prototype` then `repo run @semio-tech/framework-os-dev:dev-s-react-prototype` -> Vite on http://127.0.0.1:6070/ (serve script `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts`, hub optional via `repo run os-hub:dev` port 8787).
- STATUS: activation running (started under `repo capacity run`, log `🗑️generated/activate-s.log`). It is stuck/retrying in "Store dependency sync" (`Bun current ownership changed after refreshed continuation`) because other workers run dependency work concurrently (store-ownership leases). No dev server yet.

## Browser prerequisites
- `~/.cache/ms-playwright` already holds chromium-1248, chromium_headless_shell-1248, ffmpeg-1013. `ldd` on `chrome-headless-shell` and `chrome`: 0 libraries not found. No pacman needed (`sudo -n` needs a password, so it would be blocked anyway). No host blocker for headless Chromium. `repo tools list` shows playwright ^1.57.0 as a package-script tool (runs under repo's bun).

## backbone-storage file URL (NOT a bug, no change)
- The URI contract is `file://<raw absolute path>` / `folder://<raw absolute path>`: `buildFileBackboneUri`/`buildFolderBackboneUri` (`🧰️framework/🔨️modules/🛰️backbone/🟦️.ts:66-74`) and the store workers (`folder://${binding.path}`) never percent-encode; the whole URI is only `encodeURIComponent`-ed as a query value and decoded by the HTTP layer once. So `uri.slice(...)` returns the exact raw path for emoji and space paths. Adding `fileURLToPath`/decodeURIComponent would break any path containing a literal `%` (e.g. `100%25x` would be double decoded). Proper fix, if wanted, is a producer-side change to real RFC 8089 URIs in builders plus all consumers at once; not done without a failing reproduction.

## bun on PATH
- Not needed. `repo tools exec bun -- test <file>` (already used by workers) runs repo's pinned bun (`~/.cache/semio/runtimes/bun/<hash>/bun`, bun 1.4.2). TS tests: `repo run <project>:test` or `repo tools exec bun -- test <path>`. Use `repo tools exec bun -- <args>` in place of bare `bun`. Under current host load (load avg 28, capacity queue 8 waiting) these queue for minutes.

## Current blocker for the dev server (as of check)
- Activation PID 813892 (`repo capacity run -- repo run @semio-tech/framework-os-dev:activate-s-react-prototype`, log `🗑️generated/activate-s.log`) loops in "Store dependency sync" with `Bun current ownership changed after refreshed continuation` and `dependency generation receipt stale ... input-changed w:package.json`. Cause: workers (and the worktree) change root `package.json`/`bun.lock` while the sync runs; the sync only converges once those inputs stop changing. Host is also saturated (load ~28, 8 capacity waiters).
- Needed: a window where no one edits `package.json`/`bun.lock` or runs dependency commands. Dependency changes should be routed to T05 only. Then activation completes and `dev-s-react-prototype` can be started (http://127.0.0.1:6070/). No dev server is running yet: no PID/URL to hand to T06.

## Dependency churn investigation (T05, 08:00)
- storybook 10.6.5: `node_modules/storybook` mtime 06:20:23, `✏️s/node_modules/storybook` 06:39:08, `✏️s/bun.lock` rewritten 06:36. All before the freeze and before my first dependency command; root `package.json`/`bun.lock` untouched since 11-03 23:35. Not my activation sync. An earlier pre-fleet install/lock pass is the origin; T04's captures older than that saw 10.6.2.
- Live JS dependency owner: PID 939199 `repo run @semio-tech/stdio-plugin:composition-prepare` (a worker's run, ~48 min) holds exclusive `javascript-dependency-build` and progresses. My activation and `dependencies ensure` waited behind it. I stopped my activation. Non-fleet sessions (cursor-agent 697140, `claude -p ... agents.json` 740402) hold no dependency leases.
- Plan: after 939199 ends, one `repo dependencies ensure`, then activation, then announce one stable generation.

## Rust build churn
- (a) Not shared: each artifact workspace gets its own CARGO_TARGET_DIR `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/u/l/p<hash>/t` (dxf p81527b 2.7G, bmp pd715ff, idml p1bfde6, p363ec2 4.4G), so framework crates rebuild once per artifact workspace. CARGO_HOME is shared, and a rustc-wrapper unit cache (`⚡️caching/🦀️cargo/🗃️unit-cache`) is active.
- (d) Every cargo runs with CARGO_BUILD_JOBS=1 and `-Zthreads=0 -Clinker-features=+lld`. Host has 12 cores and load average ~51: CPU is oversubscribed, so jobs=1 is not the main limiter; concurrency is.
- No config change applied: lane hashing and gate parallelism are repo-core policy and I could not measure before/after under this load.
- Oracle-crate dependency pass: crates.io reachable (HTTP 200), pass not started; it waits behind the dependency lease.
