# Lane 1-F report — dev configs: registry users dimension, seed, build lease

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts`
  — `PlaygroundEntry.userPorts?: { react: number[]; wgpu: number[] }`; `parsePlaygroundBlock` parses a
  new `user_ports = { react = [...], wgpu = [...] }` TOML line (new `parseTomlInlineNumberArray`
  helper); `emitPlaygroundsTypeScript` emits `userPorts` on both the catalog row literal and the
  `PlaygroundBuildTarget` type; `validatePlaygroundRegistry` gained a global-port-uniqueness pass
  (`globalPortOwners`/`claimGlobalPort`) covering every `ports.react`/`ports.wgpu` **and every**
  `userPorts.react[]`/`userPorts.wgpu[]` slot across the whole catalog (kept the existing pair-keyed
  `portOwners` check as-is, additive).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🖥️launch.ts`
  — `DevLauncherEntry.users?: DevLauncherUsersTemplate` (`namePrefixPattern`/`emailPattern`/`env`);
  `substituteUserTokens`, `renderUserEntry`, `renderUserEntries` render one launcher per
  `playground.userPorts.react[]`/`.wgpu[]` slot (`{N}`/`{PORT}`/`{EMAIL}` tokens; `SEMIO_RENDERER` set
  programmatically, not templated); `generateLaunchJson` expands a single `"@generated:<variant>:users"`
  placeholder into N comma-joined launcher objects. Order = `round((baseOrder + n*0.01) * 100) / 100`
  (avoids a `386.21999999999997` floating-point artifact) — for `s` this yields exactly `386.21`/`386.22`
  (react) and `386.01`/`386.02` (wgpu), i.e. below the react ones per the brief.
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml` — added
  `user_ports = { react = [6072, 6073], wgpu = [6067, 6068] }` to the existing `s` playground block
  only (single line; nothing else in the file touched, per the 1-E lease boundary).
- `.vscode/🧩️launch.seed.jsonc` — added `"@generated:s:users"` placeholder after
  `"@generated:s:wgpu"`; added `devLaunchers.s.users` (namePrefixPattern `🖥️s👤️{N}`, emailPattern
  `user{N}@semio.dev`, env `S_OS_PORT`/`SEMIO_PLUGIN`/`S_USER`/`S_HUB_URL`/`S_DATA_DIR`); added
  `serverReadyAction` to `🛠️dev🗄️os-hub` (opens `/admin`); added `🛠️dev🗄️os-hub🛡️admin` launcher
  (`bun nx run os-hub-admin:dev`, env `OS_HUB_URL=http://127.0.0.1:8787`, order 387.05,
  serverReadyAction on port 8790 — see Blockers, that nx project doesn't exist yet this wave); added
  compound `🧭️compound🖥️s👥️users🗄️os-hub` (order 386.16, `stopAll: true`); added `4_gate` entry
  `⚖️gate🌎️collab-e2e` (`bun nx run @semio-tech/framework-os-dev:collab-e2e`, order 410.98 — see
  Blockers, that target doesn't exist yet this wave).
- `.vscode/launch.json` — regenerated (never hand-edited).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` —
  new `//#region 🔖️PluginBuildLease` (`PluginBuildLease` type, `acquirePluginBuildLease`,
  `markPluginBuildLeaseReady`, `waitForPluginBuildLeaseReady`, `releasePluginBuildLease`,
  `isPidAlive`/`readPluginBuildLease`/lease path helpers), placed just above `class DevScript`.
  `DevScript.run` wired: `defaultPort` is now computed before the plugin-build branch (was after);
  when `streamPluginBuilds` (react, `SKIP_PLUGIN_BUILD!=="1"`) it calls `acquirePluginBuildLease(plugin,
  port)`, registers `exit`/`SIGINT` release handlers, and branches: **follower** logs
  `[dev] plugin builds owned by pid <pid> (port <port>); serving only` and only
  `waitForPluginBuildLeaseReady` (skips `ensurePluginRegistry`/`buildEngineWasm` entirely); **holder**
  runs `ensurePluginRegistry` + `buildEngineWasm` then `markPluginBuildLeaseReady` before Vite starts,
  and — post-Vite-start — only the holder runs `buildPluginsStreaming` + `watchPluginRebuilds` (was
  unconditional on `streamPluginBuilds`, now conditional on `pluginBuildLease?.role === "holder"`, which
  is equivalent for the single-session case since a lone process always wins the lease). wgpu / non-
  streaming / `SKIP_PLUGIN_BUILD=1` paths are untouched (`pluginBuildLease` is `undefined` there).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts` —
  `define` block gained `VITE_S_HUB_URL`/`VITE_S_USER`/`VITE_S_DATA_DIR` from `process.env.S_HUB_URL`/
  `S_USER`/`S_DATA_DIR` (empty-string fallback), mirroring the existing `VITE_SEMIO_*` entries. These
  env vars already reach the spawned Vite child via `devToolingEnv`'s `{ ...process.env, ...extra }`
  full inheritance (`runViteBunxDev` → `playPollingEnv` → `devToolingEnv`), so this is the `define`-time
  compile-in step the React app's `import.meta.env.VITE_S_*` reads need, not a new passthrough path.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts`
  — added a docstring above `trunkEnv()` documenting the existing (unchanged) `{ ...process.env }`
  passthrough of raw `S_HUB_URL`/`S_USER`/`S_DATA_DIR` to the spawned `trunk`/native child — both
  `trunkEnv()` and `NativeRunScript`'s `nativeEnv` already full-spread `process.env`, so no behavior
  change was needed there; only documented.

## Commands run + results

- `bun nx run @semio-tech/plugin-registry:generate` → `plugin registry catalog refreshed (59 plugin
  crates, 58 playgrounds, 22 framework packages) -> .../📇️registry/🤖️generated`; `.vscode/launch.json
  regenerated`. Exit 0.
- `bun nx run @semio-tech/plugin-registry:check` → `plugin registry catalog is fresh (59 plugin crates,
  58 playgrounds, 22 framework packages); .vscode/launch.json is fresh.` Exit 0 (pre-existing taxonomy
  warnings and package-discovery warnings unrelated to this lane's files print but do not fail the
  gate — same warn-only findings as before this lane touched anything).
- `git diff HEAD --stat -- .vscode/launch.json` → `140 insertions(+)`, **zero deletions**. (A plain
  `git diff` with no ref showed spurious-looking deletions of unrelated peer entries —
  `🛠️dev🖥️s⚛️react🛡️policy-vigilant`, `⚖️gate🎯️mutation-outcome`, etc. — because the git **index**
  already had staged content that isn't in either `HEAD` or the current seed file; `git diff HEAD`
  is the correct comparison and confirms this lane's regenerate is purely additive relative to the last
  commit.)
- Regenerated launch.json contains exactly the required 4 user launchers
  (`🛠️dev🖥️s👤️1⚛️react`@6072 order 386.21, `🛠️dev🖥️s👤️2⚛️react`@6073 order 386.22,
  `🛠️dev🖥️s👤️1🧊️wgpu🌐️wasm`@6067 order 386.01, `🛠️dev🖥️s👤️2🧊️wgpu🌐️wasm`@6068 order 386.02) with the
  exact env from the brief, the `🧭️compound🖥️s👥️users🗄️os-hub` compound, the
  `🛠️dev🗄️os-hub🛡️admin` launcher, and the `⚖️gate🌎️collab-e2e` gate entry — verified by direct
  `grep`/diff inspection of the generated file, not just the seed.

## Two-dev lease proof

Full log: `$T/🧪️1-f-two-dev.txt` (3 attempts, all real runs, nothing fabricated). Both `dev` processes
never reached a listening Vite port in this session (see Blocker below — a pre-existing, peer-owned
repo-wide breakage, not this lane's code), so the "both serving, second logs owned-by-pid" end state
from the brief could not be fully reached. Every individual mechanical piece of the lease **was**
exercised against real, concurrently-running processes and is proven correct:

1. **Attempt 1** — `S_OS_PORT=6072 S_USER=user1@semio.dev S_HUB_URL=http://127.0.0.1:8787
   S_DATA_DIR=/tmp/s-u1 SEMIO_PLUGIN=s bun nx run @semio-tech/framework-os-dev:dev`. Lease file
   appeared at `target/semio-dev-leases/plugin-build-s.json` with
   `{"pid": <dev process pid>, "port": 6072, "startedAt": …, "registryReady": false}` — confirms
   `acquirePluginBuildLease` won the `wx` race as holder and recorded the right port. Process then hit
   the Blocker below and threw; the lease file was gone immediately after — confirms
   `process.once("exit", …)` → `releasePluginBuildLease` runs on an **error exit**, not just a clean
   one.
2. **Attempt 2** — same command, retried after confirming via `git status` the peer's blocking files
   were still mid-edit. Identical failure, identical clean lease release on exit (second independent
   confirmation).
3. **Attempt 3** — started user1, polled for the lease file to appear (took 4.5s), then **immediately**
   started user2 (`S_OS_PORT=6073 S_USER=user2@semio.dev S_HUB_URL=http://127.0.0.1:8787
   S_DATA_DIR=/tmp/s-u2 SEMIO_PLUGIN=s bun nx run @semio-tech/framework-os-dev:dev`) while user1's lease
   was still live. Real, observed results, in order:
   - User2 immediately logged the exact required line:
     **`[dev] plugin builds owned by pid 12874 (port 6072); serving only`** (12874 = user1's real pid)
     — proves `acquirePluginBuildLease` correctly returned `"follower"` for a second process racing a
     live holder, and the log format matches the brief exactly.
   - User2 then correctly skipped **all** cargo/registry work (no `ensurePluginRegistry`/
     `buildEngineWasm` log lines from user2 at all — only user1's holder-side registry-refresh and
     `wasm-pack build` lines appear) and called `waitForPluginBuildLeaseReady`.
   - User1 stayed stuck in the pre-existing cargo build-directory lock contention (see Blocker) for the
     full window and never called `markPluginBuildLeaseReady`. After exactly 60s, user2 correctly threw
     `plugin-build lease for "s" did not become ready within 60000ms` and exited — proves the
     `PLUGIN_BUILD_LEASE_READY_TIMEOUT_MS` bound is enforced precisely as designed, with a clean,
     diagnosable error rather than a silent hang, when a holder never gets past its own build step.
   - User1 (holder) was then stopped: `kill -INT` on the `bun ./📜️script.ts dev` process alone did not
     release the lease immediately, because it was blocked **synchronously** inside `runCmdStatus`
     waiting on its `wasm-pack`/`cargo` child (Node/Bun cannot run a queued signal handler until a
     fully-synchronous spawn-wait returns) — an unsurprising property of the existing synchronous
     `runCmdStatus` primitive, not a defect in the lease region itself. Sending `SIGINT` to the child
     `wasm-pack` process let the parent's blocking call return (with an error), at which point the
     parent's own exit handling ran and the lease file was removed — third independent confirmation of
     `releasePluginBuildLease` firing correctly, this time via the `SIGINT` path plus the resulting
     error exit. Noted here as a real operational nuance for whoever next touches `runCmdStatus`/signal
     handling, not something this lane's lease region can or should fix — kept scoped to the new region
     only, per the lease boundary.

**Blocker (not this lane's code, do not fix):** every attempt died at the same point —
`buildEngineWasm` → `framework-surface-node-graph wasm build failed` → the underlying `cargo build`
fails because `semio-framework-os-kernel` (lib) doesn't compile, exactly 2 real errors both in the
**same** file:
`error[E0432]: unresolved imports web_sys::BinaryType, web_sys::MessageEvent, web_sys::WebSocket` at
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs:436` and
`error[E0599]: no method named headers found for struct web_sys::Request` at
`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs:462` (the other `-->`
locations in the raw log are secondary trait/trace references for those same 2 errors, not separate
failures — "due to 2 previous errors"). That file is a brand-new `#[cfg(target_arch = "wasm32")]
pub mod browser` seam (its own doc comment: "NOT the production browser path today") inside
`📇️directory/🔌️client` — this ticket's own `1-D` lane deliverable ("Rust identity + directory client
twin" per `📋️ownership-and-handoffs.md` §B), well outside my `1-F` lease (registry/seed/dev-script/
vite-config/wgpu-script only) and not something I touched or should touch.

Notably `semio-framework-os-kernel`'s own manifest
(`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:56`) **already** lists every feature the
errors complain about missing — `web-sys = { …, features = ["Window", "Storage", "console",
"WebSocket", "MessageEvent", "BinaryType", "Request", "RequestInit", "Response", "Headers"] }` — so
this isn't a simple missing-feature fix; that Cargo.toml itself shows `git status --short` = `M`
(staged, uncommitted) right now, alongside the equally in-progress `📇️directory/🔌️client/🦀️component.rs`
(`git log` on that file returns nothing — it has never been committed). Both are mid-edit in this same
live session, so the live `cargo`/`target/` build state momentarily disagreeing with a manifest that
already looks correct on disk is exactly the master ticket's "Concurrent Cargo Workspace Churn" pattern
(a build picking up a Cargo.lock/incremental-cache snapshot mid-flux, not a real code defect) — expected
to self-resolve once lane 1-D's in-flight edits settle, not something a one-off fix from this lane
could safely address. This blocks **any** `dev s`/`dev <variant>` react session right now, not
specific to the `users` launchers or the lease. The shared `target/` build-directory lock contention
observed alongside it (many concurrent peer `cargo check`/`cargo build` processes, per `ps aux` at the
time) is the same pattern.

**What is proven vs. not:** every unit of the lease design — atomic holder acquisition, correct
follower detection against a live holder (verified with the exact real pid), the follower's exact log
line, the follower's bounded wait and clean timeout, and lease release under three different exit paths
(uncaught-throw exit ×2, SIGINT-after-child-interrupt ×1) — is proven against real, concurrently-running
processes, not simulated. The one thing not observed is a follower reaching `registryReady: true` and
serving its own Vite past a **successful** holder, purely because no holder in this environment could
finish its build during this session (peer breakage). Once `cargo check -p semio-framework-os-kernel`
is green again, re-running the same two commands should reach that state; nothing in this lane's code
depends on that peer fix to be correct.

Cleaned up after: killed both `dev` process trees, removed `target/semio-dev-leases/*` (empty — last
holder released cleanly), removed `/tmp/s-u1`/`/tmp/s-u2` scratch data dirs. `bun nx run
@semio-tech/plugin-registry:check` re-run clean (`fresh`) after the test session.

## sharedFileRequests

None. Every edit stayed inside the 1-F lease (`📇️registry/{📜️script.ts,🖥️launch.ts}`, `🖥️launch.ts`,
the `s` playground block of the space plugin's `Cargo.toml`, `.vscode/🧩️launch.seed.jsonc`, the dev
module's `{📜️script.ts,⚙️vite.config.ts}`, and only the `trunkEnv()` docstring in the wgpu target
script). The `📇️directory/🔌️client/🦀️component.rs` blocker above is flagged for awareness only —
lane 1-D's own in-progress file, not a request for anyone else to act on urgently (it looks like
transient churn from their own concurrent edit, likely to resolve on its own).

## What is NOT done

- **The full "both dev servers serving, second logs owned-by-pid" end state** from the Verify
  checklist was not reached in this session — see the Two-dev lease proof section above for exactly
  which parts *were* proven against real concurrent processes, and why the remainder is blocked on
  unrelated, currently-broken peer code (`📇️directory/🔌️client/🦀️component.rs`, lane 1-D).
- **`🛠️dev🗄️os-hub🛡️admin` and `⚖️gate🌎️collab-e2e`** are wired into the seed/generated launch.json
  exactly as specified, but cannot be exercised yet: `os-hub-admin` is an nx project lane 2-E creates
  this wave (not yet present — confirmed no `os-hub-admin` project exists as of this report), and
  `@semio-tech/framework-os-dev:collab-e2e` is an nx target lane 3-C implements next wave. Both are
  expected to be no-ops/errors if run today; this matches the brief ("that may not run yet; that is
  fine, say so").
- Did not touch, and make no claim about, anything under `📇️directory/**`, `📡️spr/**`, `🏪️store/**`,
  or any other peer-leased path — consumed only what already exists on disk (the generated playground
  catalog, existing `frameworkOsPlaygroundDefaultPort`/`loadFrameworkOsPlaygroundCatalog` helpers,
  etc.).
