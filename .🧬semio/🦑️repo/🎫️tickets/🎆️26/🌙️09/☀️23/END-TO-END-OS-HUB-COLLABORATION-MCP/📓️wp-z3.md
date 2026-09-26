# WP-Z3 — Cross-Platform + Zero-Touch (Linux winit, Hub in Docker, Devcontainer, Windows, Native Linux)

Slice Z3, session 13 (2026-09-26 19:0x), Opus 5.5 executor; successor of Z2 ([`📓️wp-z2.md`](📓️wp-z2.md)); handovers
[`📓️wp-h10.md`](📓️wp-h10.md) item 3 (Docker/Linux), [`📓️wp-h9.md`](📓️wp-h9.md) row D, [`📓️audit-s12-hub.md`](📓️audit-s12-hub.md) P2-3.
Ports: hubs 8100–8109, serves 6600–6609. Private target `.tmp-ticket/wp-z3/target`. Durable data `.🧬semio/🌐hub/s13-z3-*`.
Captures `wp-z3/generated/` (expendable). One-off codemods/probes `wp-z3/*`.

## Session 13

| # | Item | Status | Evidence |
|---|---|---|---|
| 1 | Linux `winit` backends (Z2 B4 / H10 patch) landed + checked native macOS + wasm32; Linux build in Docker | APPLIED 19:1x (3 manifests, root + 12 bridge locks re-resolved, all 49 tracked locks `--locked --offline` green); native check blocked twice (P8 cache poisoning, then a fleet lock pile-up); Docker proof on hold (rule 22) | `wp-z3/winit-linux-backends.py`, `generated/check-native-*.txt` |
| 1b | Shared build-dir poisoned by a scratch clone (found while checking item 1) | FIXED + GATE LANDED: 351 foreign dep-info (all `debug`, P8's clone) invalidated; `cargo-provenance check`/`repair` verb, nx `repo:cargo-provenance-{check,repair}`, `repo:test-cargo-provenance` laws **5/5** (fixture + Ajv, Python ntpath/posixpath oracle, real-cargo poisoning oracle), launch rows | log 20:1x–20:3x |
| 2 | Hub in Docker: cold build + run → `/healthz`, `/readyz`; image size, cold build time; backup/restore drill in the container | OPEN | |
| 3 | Devcontainer zero-touch (`bun install` → `dev s` + local hub + semio MCP) measured inside the container | PREPARED: `docker-in-docker:2` (`moby: false`) feature so V1's `os-hub-ts:backend-*` runs unchanged inside (backends publish on 127.0.0.1) + law row; timed run blocked by rule 22 (no image builds) | log 20:5x |
| 4 | Native Windows audit of `dev s` / hub / semio MCP / fleet `📜️script.ts` verbs; root fixes + laws exercising the Windows branch | 5 ROOT FIXES LANDED + LAWS: owner-only files (TS + Rust twins, one fixture), process table (POSIX + Windows), cache-prune safety on Windows, file-URL pathname, `shell: true` spawn; `repo-lib:test-windows-command-paths` **17/17** (direct + nx); Rust twin **4/4** + `cargo check` for `x86_64-pc-windows-msvc`/linux/wasip2 green; MCP crate check pending (lock convoy) | §4 below |
| 5 | Native Linux proxy (devcontainer image): hub + `dev s` React shell boot | OPEN | |

## Log

- 19:08 started; read AGENTS.md, preambles 13 + 12, `wp-z2.md`, `wp-h10.md`, `wp-h9.md` row D, `audit-s12-hub.md` P2-3.
  Disk 110 GiB free, load 51, 4 rustc, wasm mutex free, no REBUILD START yet.
- 19:1x item 1: both prepared sets dry-run clean on the current tree (Z2's appends at EOF with a redundant
  `all(linux, not(wasm32))` key; H10's covers `🖱️ui` only). Re-derived as `wp-z3/winit-linux-backends.py`: one
  `cfg(target_os = "linux")` table next to each declaration it extends (the ui host reuses its existing Linux table).
  12 standalone `🏭️bridge` workspaces also lock winit and are built `--locked` (`📋️native-orchestration`,
  `🏗️native-build`), so their `Cargo.lock` files need the same re-resolution as the root lock.
- 19:2x item 1 applied (`wp-z3/winit-linux-backends.py <root> --apply`): `🖱️ui` (optional, next to its
  `not(wasi)` declaration), `🖱️ui/🖥️host` (into its existing Linux table), wgpu renderer target (after its winit line).
  Root `Cargo.lock` re-resolved by `cargo metadata` (no `--locked`): **+9 packages, 0 existing entries changed**
  (as-raw-xcb-connection, calloop-wayland-source, smithay-client-toolkit, wayland-csd-frame, wayland-cursor,
  wayland-protocols-plasma/-wlr, x11-dl, xcursor). The 12 `🏭️bridge` locks that contain winit: +17…20 packages each,
  0 removed entries. `📕️norm/🏭️bridge/Cargo.lock` was already out of sync with its manifests before my change (peer
  norm edits, committed): re-resolved too (dependency lists only). Then all 49 tracked non-ticket `Cargo.lock` files pass
  `cargo metadata --locked --offline`.
- 19:2x–19:55 native check (`cargo check -p semio-framework-ui --features wgpu-engine -p semio-framework-ui-host -p
  semio-framework-os-renderer-wgpu --lib --tests`) failed 4× in `semio-framework-os-kernel` on LD's new
  `mutation_envelopes_from_edit_since` although the source had it; `cargo check -p semio-framework-os-kernel --lib`
  alone was green → a feature-specific stale unit.
- 20:1x **root cause: the shared build-dir was poisoned.** `semio-framework-replication`'s unit (`debug`, features
  `default,deflate`, rmeta 15:36) was reported `Fresh` although its source changed at 19:19: its dep-info
  (`fingerprint/dep-lib-protocol`) named `/private/tmp/…/22ef121e…/scratchpad/p8-clone/…` — P8's scratch clone had built
  into the repository's build-dir (Cargo's unit hash of a path package is workspace-relative, so a clone with the same
  layout lands on the same unit; the absolute `..`/`#[path]` sources of the clone then decide freshness). 351
  `fingerprint/dep-*` files (157 crates, all `debug`, written 09:2x–15:54) pointed into the clone; deleted only those
  files at 20:14 (Cargo rebuilds a unit whose dep-info is missing). Coordinator told (any green check of those crates
  since ~16:00 may have run against stale dependencies).
- 20:2x **permanent gate** `⚡️caching/🦀️cargo/🧾️provenance/🟦️.ts`: owned decoder of Cargo's encoded dep-info (version 1,
  exact end-of-buffer), foreign classification under POSIX and Windows path rules, cancellable scan of every
  `<target>/build/<package>/<hash>/fingerprint/dep-*`, repair = delete the foreign units' dep-info. Verb
  `bun ⚡️caching/📜️script.ts cargo-provenance check|repair [--json]`, nx `repo:cargo-provenance-check` (exit 1 on any
  foreign unit), `repo:cargo-provenance-repair`, `repo:test-cargo-provenance`; launch rows
  `⚖️gate🦀️cargo🧾️build-dir-provenance` (4_gate −11.4), `📦️test🦀️cargo🧾️provenance` (206.178),
  `🧹clean🦀️cargo🧾️provenance` (206.179). Laws `🧪️tests/🧾️cargo-provenance` **5/5** (direct and through nx): fixture
  `🧫️fixtures/🧾️cargo-provenance` + schema `🧬️schema/🧾️cargo-provenance` (Ajv strict), Python `ntpath`/`posixpath`
  containment oracle (Windows drive letters, UNC, case folding, `..`), and a real-cargo oracle: two copies of a crate
  sharing a build-dir → a type error in the second copy stays `Fresh` (poisoning reproduced), the gate names the first
  copy's source, repair → Cargo recompiles and reports `E0308`; rustc's own `.d` checksums equal the decoded ones.
  Live after repair: 6 992 dep-info decoded across 10 targets, **0 foreign, 0 undecodable**.
- 20:3x restored the launch row `🧪️test⚡️cache-command-source` (dropped from both launch files by the 09-15 commit
  `6f33e31`; the command-source law demanded it): command-source law 10/11 → **11/11**.
- Rule for scratch clones (proposed to the coordinator): a clone sets `CARGO_BUILD_BUILD_DIR` and `CARGO_TARGET_DIR`
  inside itself and never builds into the repository's build-dir; `repo:cargo-provenance-check` detects violations.
- 20:4x stopped my own native check 61592 (waited > 15 min, no rustc child: fleet lock convoy, preamble rule 25); item 1's
  native + wasm32 checks retry when the convoy clears.
- 20:4x–21:1x **item 4, Windows audit** (static; no Windows machine): import closure of the fleet entrypoints (root router →
  `dev s`, os dev → local hub, hub Rust/TS scripts, semio MCP script, repository cache + Nx bootstrap) = 774 TS files
  (`wp-z3/windows-closure-scan.ts`); Rust: 147 workspace crates of the `x86_64-pc-windows-msvc` closure of `semio-hub` +
  `semio-framework-os-mcp` (6 434 source files from rustc dep-info) scanned for unix-only cfgs. Most paths were already
  Windows-aware (taskkill branches, junctions, `.exe` names, `cfg(not(unix))` twins). Root-fixed:
  1. **Owner-only secrets had no Windows protection.** `chmod 0600/0700` is a no-op on Windows: the session-broker record
     (bearer secret), the local hub data root, delegated agent credentials and MCP bridge offers inherited the parent's
     ACL (under `C:\src` that includes Authenticated Users). New owner `📚️library/🏃️process/🔐️owner-only/🟦️.ts`
     (`protectOwnerOnly`, `assertOwnerOnly`): POSIX exact modes; Windows `icacls /inheritance:r /grant:r *<SID>:F`
     (directories `(OI)(CI)`, SID from `whoami /user`) then `Get-Acl` SDDL read-back (path via environment, no quoting)
     judged by `sddlOwnerOnlyViolations`. Rust twin `🌉️mcp/🔐️owner-only/🦀️.rs` (bridge offers: empty file restricted
     before the secret is written; agent credential refused unless owner-only on every platform). Routed: hub
     `local-bootstrap` run root + session-broker record, dev `local-hub` data root (3 sites), vite credential install.
  2. **`clean` could prune the cache under an active build on Windows**: the guard ran `ps` (absent) and read its missing
     output as "no build". New `🏃️process/📋️process-table` (POSIX `ps`, Windows `Get-CimInstance` JSON forced to UTF-8
     so emoji command lines survive); unreadable table → prune refused. One shared `ACTIVE_SEMIO_TECH_BUILD` pattern
     (also matches Windows command lines with a quoted `nx.js`) instead of two copies.
  3. `repoRootFromHere` used `new URL(import.meta.url).pathname` (`/C:/…` on Windows) → `fileURLToPath`.
  4. `spawnBun` spawned `process.execPath` with `shell: true` (breaks on `C:\Users\Jane Doe\…`, cmd.exe re-parses args) →
     no shell.
  Laws: `📚️library/🧪️tests/🪟️windows-command-paths` **17/17** (direct 7 s, `nx run @semio-tech/repo-lib:test-windows-command-paths`
  with `NX_DAEMON=false` because the daemon's graph had not picked up the new target): fixtures `🔐️owner-only`,
  `📋️process-table` + schemas (Ajv strict); Windows branches driven by recording hosts (exact icacls argv, Get-Acl path via
  env, leaking DACL / failing icacls / missing whoami are errors); POSIX branch on the real host with a Python `os.stat`
  oracle; process table vs Node's `process.ppid`; static closure laws (no file-URL pathname, no `execPath` + `shell:true`,
  process-group signals and `ps`/`lsof` only in files with a Windows branch, credential owners use `protectOwnerOnly`).
  .NET `RawSecurityDescriptor` oracle runs wherever `pwsh`/`powershell.exe` exists (every Windows); here none: the
  PowerShell container (`mcr.microsoft.com/powershell:latest`, local, arm32 layer) crashed under emulation
  (`wp-z3/sddl-dotnet-oracle.sh`), so that oracle is unexecuted. Rust twin in a standalone harness
  (`wp-z3/owner-only-harness`, `#[path]` onto the module incl. its `🧪️tests/🔬️unit` laws on the shared fixture):
  **4/4** native; `cargo check --tests` for **x86_64-pc-windows-msvc** (compiles the `cfg(windows)` icacls/Get-Acl glue),
  x86_64-linux and `--lib` wasm32-wasip2: 0 warnings (the 0.3 s wasip2 check of this std-only harness ran outside the wasm
  mutex — noted). Hub `🧱️foundation-source`: 10/12, the 2 failures are not mine (credential-issuance's `Bun.serve` under
  `types:["node"]` at HEAD; the hub router's new `ensureHubBackend`/`LOCAL_HUB_DEVELOPMENT_*` imports vs the fixture).
- 20:5x **item 3 prep:** `.devcontainer/devcontainer.json` gains `ghcr.io/devcontainers/features/docker-in-docker:2`
  (`moby: false`: Docker CE, Ubuntu noble) — V1's backends publish on `127.0.0.1`, which a host socket
  (docker-outside-of-docker) would not reach from inside the container. Law row in `🐳️containers/🧪️tests/🚀️runtime-bootstrap`
  (+ fixture `dockerDaemonFeature`): PASS.
