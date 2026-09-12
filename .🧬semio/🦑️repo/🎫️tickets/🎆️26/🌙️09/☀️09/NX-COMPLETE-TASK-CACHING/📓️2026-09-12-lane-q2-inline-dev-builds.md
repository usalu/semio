# Lane Q2 — Inline Dev Builds (os-hub build-dev + repo-wide continuous sweep) (2026-09-12)

Scope: phase 2 follow-up to `📓️2026-09-12-lane-p3-continuous.md`'s one deferred finding —
`os-hub`'s dev-profile `cargo build` still ran inline on every `dev`/`dev-secure-*` launch instead
of through a cached Nx package target — plus a repo-wide re-sweep of every `continuous: true`
target's implementation for the same anti-pattern.

## Task 1 — `os-hub` dev-profile build

### Before

`DevScript.run` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`) ran, unconditionally on every single
`dev`/`dev-secure-suite`/`dev-secure-native`/`dev-secure-mcp`/`dev-secure-admin` launch:

```ts
buildAdminSpa(this.repoRoot);
runCargo(["build", "--manifest-path", "Cargo.toml"], this.root);   // uncached, into the shared cargo target-dir
```

`startLocalHub` then defaulted to `hubBinaryPath(repoRoot)` = `<shared-cargo-target-dir>/debug/os-hub`
— a directory several *other* feature-variant builds in this same file also write into (test-support,
sqlite+native-artifact-execution, etc.), so it could not simply be declared as an Nx `outputs` path
(same shared-mutable-directory problem lane P2 solved for wgpu's plugin-module root).

### After

1. **New Nx-cached target `os-hub:build-dev`** (`🌎️hub/📦️packages/🦀️rust/📋️project.json`), mirroring
   `@semio-tech/framework-renderer-wgpu:native-build`'s `dist/native-<profile>` pattern exactly:
   ```json
   "build-dev": {
     "executor": "nx:run-commands",
     "cache": true,
     "outputs": ["{projectRoot}/dist/build-dev"],
     "options": { "cwd": "🌎️hub/📦️packages/🦀️rust", "command": "bun ./📜️script.ts build-dev" }
   }
   ```
   Backed by a new `BuildDevScript` class in `📜️script.ts` that calls the shared
   `buildCargoArtifacts` helper directly (same function `native-build` uses, imported from
   `⚡️caching/🦀️cargo/📜️script.ts`) with an explicit `{ output: "dist/build-dev" }`, so the dev-profile
   binary stages into a directory exclusive to this one Nx target — safe to cache, unlike the shared
   cargo target-dir.
2. **`dev`/`dev-secure-suite`/`dev-secure-native`/`dev-secure-mcp`/`dev-secure-admin`** each gained
   `"build-dev"` in their `dependsOn` array (alongside the pre-existing `os-hub-admin:build` dep).
3. **`DevScript.run`**: removed the unconditional `runCargo(["build", ...])`; added
   `hubDevBinaryPath(this.root)` (mirrors wgpu's `nativeBinaryPath` — resolves
   `{projectRoot}/dist/build-dev/os-hub[.exe]`, throws a clear "run: bun nx run os-hub:build-dev" error
   if the staged binary is missing instead of building it inline) and passes it as `binaryPath` to
   `startLocalHub`, so the actual server process now execs the Nx-staged artifact.
4. **Security check preserved exactly**: the *only* other thing in `DevScript` that needed a Hub
   binary is the cold-bootstrap trusted-stdio+GIS validation (`if (!trustedCatalog) { ... }`), which
   only fires once per fresh `.🧬semio/🌐hub` dataRoot and calls
   `validateAndPublishTrustedStdioGisCandidate` → `proveTrustedGisColdMapComponentV1`, which uses
   `validation.cargoTargetDir` as a *real* `CARGO_TARGET_DIR` env var to build+test an unrelated crate
   (`semio-s-plugin-gis`) for its own independent proof. Repointing that at the new `dist/build-dev`
   staging directory would have silently turned this security proof's `CARGO_TARGET_DIR` into a
   throwaway folder instead of the real shared cargo cache — a behavior change to a security-critical
   invariant (`dirname(dirname(binaryPath)) === cargoTargetDir`, asserted in
   `validateAndPublishTrustedStdioGisCandidate`) I judged out of scope for a "reduce inline builds"
   ticket. Left this one branch's own `hubBinaryPath(this.repoRoot)` + a scoped `runCargo(["build", ...])`
   call exactly as before, but moved *inside* the `if (!trustedCatalog)` branch only — it now runs once
   per fresh dataRoot instead of on every dev launch, and the literal call
   `validateAndPublishTrustedStdioGisCandidate(this.repoRoot, this.root, dataRoot, receipt, validation)`
   is byte-for-byte unchanged (verified against the source-fence meta-assertions in this same file at
   lines ~9070-9078 and ~12997-13048, which read `DevScript`'s own source text and assert this exact
   call, its exact ordering relative to `materializeTrustedStdioGisBundle`/`startLocalHub`, and the
   absence of a direct `publishTrustedBootstrapCurrent` call — all still pass by inspection since none
   of the asserted substrings were touched).
5. Router registration: `.register("build-dev", BuildDevScript)` added before `.register("dev", DevScript)`.

### Files changed

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — import `buildCargoArtifacts`; new `hubDevBinaryPath`;
  new `BuildDevScript`; `DevScript.run` rewired as above; router registration.
- `🌎️hub/📦️packages/🦀️rust/📋️project.json` — new `build-dev` target;
  `dependsOn: ["build-dev", ...]` added to `dev`, `dev-secure-suite`, `dev-secure-native`,
  `dev-secure-mcp`, `dev-secure-admin`.

`secure-local-smoke` (not `continuous`, cache:false by design, a one-off smoke test) keeps its own
`runCargo(["build", ...])` untouched — out of this lane's "dev/serve" scope.

### Verification

- `python3 -c "import json; json.load(...)"` on the edited `project.json` — valid.
- `bun build --target=bun --no-bundle 🌎️hub/📦️packages/🦀️rust/📜️script.ts` — exit 0.
- `NX_DAEMON=false bunx nx show projects` — exit 0 (run repeatedly through this lane).
- `NX_DAEMON=false bunx nx show project os-hub --json`: `build-dev` → `{cache:true,
  outputs:["{projectRoot}/dist/build-dev"]}`; `dev`/`dev-secure-native` →
  `dependsOn:["build-dev", {target:"build",projects:["os-hub-admin"]}]`, `cache:false,
  continuous:true` (correctly still uncached/continuous — the *target* stays a long-running server,
  only its build dependency is now cached).
- `SEMIO_TICKET_DIR=<ticket> bunx nx run repo:audit --skip-nx-cache` → `violations=0` both before and
  after (700 projects, 7134→7152 commands — the delta is the new `build-dev` target's own generated
  command rows).

### Blocked: could not get a live cache-hit-twice or dev-boot for `os-hub` — pre-existing, unrelated breakage

Attempting the real verification run hit **two independent, pre-existing compile failures**, neither
touched by this lane and both reproducible on the untouched `main`-tracked source:

1. **`os-hub` (Rust) does not currently compile, dev or release profile.**
   `cargo build --manifest-path 🌎️hub/📦️packages/🦀️rust/Cargo.toml --bin os-hub` (dev) **and**
   `cargo check --release --bin os-hub` (matching the pre-existing, unmodified `os-hub:build` target)
   both fail with the *same* two errors:
   - `error[E0432]` at `🚀️bin.rs:27-31`: `directory::os_directory::{DirectorySessionAuthorityV1,
     DirectorySessionKindV1}` — neither name is re-exported by
     `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs`'s `pub use schema::{...}` block.
   - `error[E0308]` at `🚀️bin.rs:3788`: `expected Option<DocumentOpenCheckpointV1>, found
     DocumentOpenCheckpointV1`.
   Both files' mtimes (2026-09-09) predate this ticket entirely, and the error reproduces identically
   regardless of `--release` vs dev — conclusively not a profile-specific regression from the new
   `build-dev` target. Flagged as `task_c984dfed` (spawn_task) for a dedicated fix.
   Evidence: `🗑️generated/q2/build-dev-run1-cold-errors.txt`, `🗑️generated/q2/release-cargo-check-same-errors.txt`.
2. **`os-hub-admin:build` (the admin SPA `dev`/`dev-secure-*` already `dependsOn`) also fails**, for an
   unrelated reason: Vite's config loader (Node's native strip-only TS mode) chokes on a TypeScript
   parameter-property constructor in `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2532`, reached via a
   non-type import somewhere in `🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/⚙️vite.config.ts`'s
   import graph (`SyntaxError [ERR_UNSUPPORTED_TYPESCRIPT_SYNTAX]`). Same bug class as a previously
   fixed os-dev `vite.config.ts` issue (see project memory note "Vite Config Node Strip-Only TS").
   Flagged as `task_2da6f0c5` (spawn_task).

Given both, `os-hub` currently cannot reach a running `dev` server at all, through no fault of this
lane's changes — confirmed by running `NX_DAEMON=false bunx nx run os-hub:build-dev` twice: run 1
fails with the E0432/E0308 errors above (3m9s); run 2, with no `--skip-nx-cache`, **re-attempts the
build rather than falsely reporting `[local cache]`** (2m44s, "Cache: 1/4 hit" — the 1 hit is an
unrelated upstream `generate` target, not `build-dev` itself), which is the correct, desired behavior:
Nx does not cache a failed task, so this new target cannot silently serve a stale/broken binary either.

**What was verified instead, given the above blocker:**
- Static target shape (cache/outputs/dependsOn) — see "Verification" above.
- Code review of `hubDevBinaryPath`/`BuildDevScript`/`DevScript`'s new call sequence, and of every
  other `hubBinaryPath(...)` call site in the file (6 total) to confirm none of them regressed — all
  five non-`DevScript` call sites are one-off test/check scripts (`proveGisMapProposalProcess`,
  `TrustedStdioGisBootstrapScript`, `AdminBackendCheckScript`-adjacent, etc.), not `continuous`
  targets, and were left untouched.
- Direct invocation `bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts dev` was attempted to observe the new
  `hubDevBinaryPath` guard firing; it instead surfaced the *upstream* `os-hub-admin:build` failure
  first (since `buildAdminSpa(this.repoRoot)` still runs before the new binary-path check, unchanged
  from the original ordering) — i.e., even this direct probe is blocked by finding #2 above before it
  can reach the code this lane changed. `dist/build-dev/` was confirmed empty on disk (no prior staged
  artifact anywhere to fall back to), so `hubDevBinaryPath` would throw its intended clear error the
  moment it is reached.
- This lane's `dev-boot on a free port` demonstration (Task 3) was performed on the general mechanism
  instead — see below — since `os-hub` itself cannot boot in the current repo state for reasons
  entirely unrelated to Nx caching.

## Task 2 — Repo-wide continuous-target sweep

Re-enumerated all `continuous: true` targets fresh (post Task 1 edits):
`SEMIO_TICKET_DIR=<ticket> bunx nx run repo:audit --skip-nx-cache` → `🗑️generated/nx/projects.json`
→ **624 continuous targets** (unchanged count from lane P3's audit — my edits only added `dependsOn`,
they did not add/remove any continuous target), across 17 projects:
`framework-os-dev` (489, lane P2 territory, excluded), `print` (89), `workspace` (19 — `dev`,
`dev-mcp`/`-engine`/`-repo`, `dev-storybook` + 13 named variants, `start`), `mit-bestand-bericht` (3),
`mit-bestand-demonstrator` (3), `mit-bestand-praesentation-projektetage` (1),
`framework-renderer-wgpu` (2), `os-hub` (5), `os-hub-admin` (1), `ui-react` (1), `assets` (1),
`repo-client` (1), `repo-coordinator` (2), `repo-cli-rs` (1), `repo-mcp-go` (1),
`framework-os-mcp-rs` (1), and **`repo-vscode` (1)** — this last project was not named in lane P3's
family table.

Grepped every distinct `📜️script.ts` reachable from these targets for
`runCmd("cargo"`/`spawn("cargo"`/`"build", "-p"`/`cargo run`/`bun build`/`vite build`/`transpile`/
`wasm-opt`/`go build`/`tectonic`:

| project:target | finding | disposition |
| --- | --- | --- |
| `@semio-tech/repo-vscode:dev` | `DevScript.run` calls `buildExtension(this.root, true)` → Vite's `build()` **API with `{ watch: {} }`** (rollup watch mode), i.e. a genuine continuous rebuild-on-source-change loop, not a one-shot `vite build` before serving | **verified correct — watcher rebuild-on-change, allowed by the ticket's own carve-out**; no fix needed |
| `os-hub:dev*` (5) | inline dev-profile `cargo build` on every launch | **fixed — Task 1 above** |
| `workspace:dev`/`start`/`dev-storybook*`/`dev-mcp*` (19) | `os`/`semio` router subcommands (`cargo run -p semio-framework-os-run`/`semio-framework-os-kernel-semio`) live in the same root `📜️script.ts` but are **not** `continuous` targets (confirmed via `nx show project workspace --json`: `os`/`semio` have no `continuous` flag, are `cache:false` `uncachedExact` mutating router commands per the 2026-09-11 coordinator-log review fix 6) — unreachable from any of the 19 continuous rows; `dev storybook` → `bunx storybook dev` (self-contained); `dev mcp`/`mcp engine` → `npx @modelcontextprotocol/inspector`; `dev mcp repo` → `requireRepoMcpBinary()` gate (lane P3's fix, confirmed intact, still no `buildRepoMcpClient`/inline `go build` anywhere in the file) | re-verified correct, unchanged since P3 |
| `@semio-tech/framework-os-dev:{dev,serve-*}` (489) | `DevScript`/`ServeScript` delegate to `nx run …:activate-<variant>-<renderer>-<profile>` (lane P2's cached chain); `ServeScript` itself only calls `runViteBunxDev` | re-verified correct (lane P2 territory), unchanged |
| `@semio-tech/framework-renderer-wgpu:{serve,dev,native,native-release}` | `ensureTrunk()`/`ensureWasmTarget()` (`cargo install trunk`, `rustup target add`) are idempotent **toolchain-provisioning** guards (`trunk --version`/`rustup target list` probed first, only installs if absent) called from `setup`, not a per-launch project build; `native`/`native-release` exec the staged `dist/native-{dev,release}` binary (lane P3's `native-build` fix) | re-verified correct, unchanged |
| `@semio-tech/print:watch-*` (89), `mit-bestand-bericht:watch-*` (3), `mit-bestand-demonstrator:{dev,serve,serve-e2e}` (3), `mit-bestand-praesentation-projektetage:dev` (1), `os-hub-admin:dev` (1), `ui-react:dev` (1), `assets:logo-dev` (1), `repo-client:dev` (1), `repo-coordinator:{dev,start}` (2), `repo-cli-rs:daemon` (1), `repo-mcp-go:dev` (1), `framework-os-mcp-rs:dev` (1) | zero matches for any of the flagged patterns | re-verified correct (lane P3's prior fixes/findings all still hold), unchanged |

No new inline-build regressions found anywhere in the 624-target continuous surface beyond the
`os-hub:dev*` family fixed in Task 1.

## Task 3 — Verification

- `NX_DAEMON=false bunx nx show projects > /dev/null` — exit 0, run repeatedly through this lane
  (after every edit and again at the end).
- `SEMIO_TICKET_DIR=<ticket> bunx nx run repo:audit --skip-nx-cache` — `violations=0` (700 projects,
  7152 commands, 8017 artifacts) at the end of this lane.
- Cache-hit-twice: demonstrated on the general mechanism (`os-hub:build-dev` uses the exact same
  `buildCargoArtifacts` staging function as `framework-renderer-wgpu:native-build`, whose
  cache-hit-twice behavior lane P3 already proved live) — for `os-hub:build-dev` itself, demonstrated
  the *complementary* half instead: a failed run is correctly **not** cached (see Task 1's "Blocked"
  section) — full positive cache-hit-twice on `os-hub:build-dev` is blocked by the two pre-existing,
  unrelated compile breaks documented above and flagged for separate fix (`task_c984dfed`,
  `task_2da6f0c5`).
- Dev-boot on a free port: could not be performed for `os-hub` itself (blocked, see above). Port
  hygiene was still checked before any attempt: `lsof -iTCP -sTCP:LISTEN -P` showed only
  peer-owned/unrelated ports (`6013`, `6018`, `6019`, `6118`, `3283`, `5000`, `7000`, `49163`) in use;
  none were touched. The two backgrounded verification builds (`os-hub:build-dev` cold + retry) and
  the two `cargo build`/`cargo check` probes used to confirm the pre-existing breakage were the only
  processes this lane started, and all ran to completion/exit on their own (no server process was
  left running; nothing needed to be stopped).

## Files changed

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/📦️packages/🦀️rust/📋️project.json`

## Left for follow-up (found, not fixed — reasons given)

- `task_c984dfed` (spawn_task) — `os-hub` bin.rs `E0432`/`E0308` compile errors, pre-existing,
  unrelated to caching, blocks every os-hub Rust target (dev and release alike).
- `task_2da6f0c5` (spawn_task) — `os-hub-admin` Vite config load failure (strip-only-TS parameter
  property in `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts`), pre-existing, unrelated to caching, blocks
  every os-hub-admin build and transitively `os-hub:dev*`'s admin-SPA dependency.
- Once both are fixed, a follow-up should re-run `NX_DAEMON=false bunx nx run os-hub:build-dev` twice
  (expect run 2 → `[local cache]`) and boot `os-hub:dev` on a free port to confirm the staged-binary
  exec path end to end — the wiring is verified correct by static inspection and code review in this
  lane, but not yet by a live green run.
