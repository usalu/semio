# Audit S11 — Cross-Platform (Windows / Linux / devcontainer / macOS)

Auditor (Sonnet 5, read-only), session 11, 2026-09-25. No edits, no builds/servers/containers run.
Every finding below is a cited file:line or a measured `find`/`grep` result executed in this session
(macOS/darwin). Builds on, and does not re-litigate, `.tmp-ticket-0918/📓️g4-zero-touch-and-run-paths.md`
(2026-09-19, pre-fix), `📓️z1-zero-touch-and-launch-rows.md` (2026-09-19, the fix), `📓️g12-deploy-and-onboarding-audit.md`
(2026-09-20, deploy/onboarding lens) and `.tmp-ticket/📓️audit-s11-build-health.md` (2026-09-25, verification-gate
freshness). Where those reports' claims still hold, this report says so and cites current evidence rather
than re-deriving it; where the current tree diverges from them, that is called out explicitly.

**Headline**: the devcontainer path (Linux container, any host OS) is the only one with a working, intact,
re-verified zero-touch chain. Native Windows and native macOS/Linux bootstrap scripts exist and are
genuinely platform-branched (not stubs), but **no evidence anywhere in ticket history shows either native
path has ever been executed on real Windows or non-container Linux hardware** — every capture in
`.tmp-ticket`/`.tmp-ticket-0918` is from this one macOS machine (confirmed again this session). The findings
below are static-source risks on the unexercised paths, ranked by how certain they are to bite on first run.

---

## Windows (native)

### P0 — MAX_PATH / long-path failure is not a risk, it is already measured on disk
The repo's own emoji-taxonomy tree plus normal build/test output already produces relative paths well past
Windows' default 260-character `MAX_PATH`. Measured this session (`find … | awk '{print length}' | sort -rn`):
```
308  🌎️hub/📦️packages/🦀️rust/🗑️generated/test-artifacts/os-hub-test-admin-presence-target-recovery-<uuid>/db/artifact-cas/v1/<64-hex>/manifest/<64-hex>
357  .🧬semio/🦑️repo/🎫️tickets/…/text-fixtures/✏️s__🔌️plugins__💠️lowpoly__🗿️artifacts__…__🗣️.dsl.semio
```
These are **relative-to-repo-root** lengths from a real hub test run and a real generated fixture — add any
real Windows clone path (`C:\Users\<name>\...\semio\`, typically 25–45 chars) and both exceed 260 comfortably.
Source-tree paths alone (excluding generated output) run up to ~180–200 chars deep
(`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/…`), leaving little headroom before test/build output pushes
over the limit. No opt-in was found anywhere in the repo:
- No `git config core.longpaths true` in `setup-git`/`SetupScript.runGit` (`📜️script.ts:325-380`).
- No `HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem LongPathsEnabled` registry write in
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1` (894 lines, grepped for
  `LongPathsEnabled`/`longPathAware`/`\\?\` — zero hits).
- No `\\?\`-prefixed path construction anywhere in the Node-side path-joining code that would let Win32 APIs
  bypass MAX_PATH per-call.

**Acceptance**: a fresh Windows 10/11 machine with **no manual registry edit** can `git clone` this repo,
run `.\🔵️.ps1 setup`, and run `cargo test -p os-hub` / `bun nx run os-hub:test-quick` without an
ENOENT/ENAMETOOLONG failure on any generated path.
**Fix direction**: (a) `setup-git` should set `git config core.longpaths true` for this workspace (needed
regardless of the registry key, since Git-for-Windows has its own 260-char default independent of the OS
setting); (b) `🔵️.ps1`'s setup should detect and, if not elevated, clearly warn about
`LongPathsEnabled` rather than silently proceeding; (c) the durable, schema-first fix is shortening the
deepest generated-path segments (e.g. the hub's `artifact-cas/v1/<hash>/manifest/<hash>` nesting) — a
registry/git workaround treats the symptom, not the taxonomy depth that causes it.

### P1 — `🔵️.ps1` has no UTF-8 BOM but runs under Windows PowerShell 5.1, not `pwsh`
`🔵️.ps1` (894 lines) opens with `#region 🧲️Header` and is emoji-commented throughout. Verified with `xxd`:
the file starts `23 72 65 67 69 6f 6e 20 f0 9f a7 b2 ...` (`#region ` + raw UTF-8 emoji bytes) — **no
`EF BB BF` BOM**. `📜️script.ts:299` (`NativeOsScript`) invokes it as:
```
runCmd("powershell.exe", ["-NoProfile", "-ExecutionPolicy", "Bypass", "-File", ps1, cmd], { cwd: this.root, env });
```
`powershell.exe` is **Windows PowerShell 5.1**, which (unlike `pwsh`/PowerShell 7) does not default to
UTF-8 for BOM-less script files — it falls back to the system's active ANSI code page, which is not UTF-8
on the large majority of non-US-English Windows installs. This risks mangled comments at best and, for any
multi-byte sequence the tokenizer treats specially, a `ParserError` at worst — untested on real Windows
this session (no non-container Windows box available), so this is a static-source risk, not a confirmed
failure.
**Acceptance**: `powershell.exe -NoProfile -ExecutionPolicy Bypass -File 🔵️.ps1 setup` parses and runs
without a `ParserError` on a non-en-US-locale Windows machine (e.g. `de-DE`, code page 1252).
**Fix direction**: save `🔵️.ps1` with a UTF-8 BOM (PowerShell 5.1 honors a BOM unconditionally), or switch
`📜️script.ts:299` to invoke `pwsh.exe` (PowerShell 7, UTF-8-default, already a standard cross-platform
install) — the latter is the schema-first choice since it also removes the Windows-only "which PowerShell
build am I on" branch entirely.

### P1 — native setup destructively overwrites the user's global `~/.cargo/config.toml`
`🔵️.ps1:847-854` (region `🌐️GlobalCliInstall`):
```powershell
$cargoConfigPath = Join-HomePath @(".cargo", "config.toml")
@"
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"']
"@ | Set-Content -Path $cargoConfigPath -Encoding UTF8
```
`Set-Content` **replaces** the whole file. Any pre-existing content in the developer's global cargo config
(registry mirrors, `[net]`/auth settings, flags for unrelated projects) is silently destroyed. There is no
equivalent write anywhere in `🐚️.sh` (macOS/Linux) — native setup is destructively asymmetric across
platforms. It is also redundant: the repo's own `.cargo/config.toml:29`
(`[target.wasm32-unknown-unknown]`) already sets a superset of the same flag
(`--cfg getrandom_backend="wasm_js"` plus a stack-size link-arg), and Cargo's config-file precedence gives
the repo-local file priority over `$CARGO_HOME` for any build run inside the repo.
**Acceptance**: running native Windows setup on a machine with a pre-existing, non-empty
`~/.cargo/config.toml` preserves every key that file had before setup ran.
**Fix direction**: delete this block entirely (the repo-local config already covers the in-repo build
case) rather than attempt a safer merge — one portable implementation (the existing repo-local
`.cargo/config.toml`) already does the job cross-platform; the Windows-only global write is the thing to
remove, not fix.

### P2 — `rustup target add` on Windows adds only one of the two required wasm targets
`🔵️.ps1:849`: `Invoke-RepoCommand -FilePath $rustupPath -ArgumentList @("target", "add", "wasm32-unknown-unknown") …`
— but `rust-toolchain.toml:4` requires **both** `wasm32-unknown-unknown` and `wasm32-wasip2`. The second is
presumably added later by `bun nx run workspace:setup`'s `deps-wasm` step (cross-platform TS, confirmed
still wired into `setup.dependsOn` — see Devcontainer section), so this is likely masked in practice, not a
hard break — flagged because the explicit, incomplete call is misleading about what it accomplishes and
should either add both targets or be removed in favor of `deps-wasm` doing it once.

---

## Linux (native, non-devcontainer)

### P0 — every native cargo build hard-fails on non-Debian/Ubuntu distros: `mold` is required but never installed
`.cargo/config.toml` (repo root) unconditionally sets, for the **native host targets** (not just
cross-compiling):
```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[target.aarch64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```
with **no fallback linker**. `mold` is installed in exactly one place in the entire repo:
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh:428,431`
(`ensure_cpp_toolchain`, Linux branch), gated on `command -v apt-get`:
```sh
sudo apt-get install -y --no-install-recommends build-essential cmake ninja-build pkg-config uuid-dev mold
```
On Fedora, RHEL/CentOS, Arch, Alpine, openSUSE, or any other apt-less native Linux (a large share of real
"native Linux dev" machines), this branch is a no-op — `mold` is never installed anywhere, and the very
first native `cargo build`/`cargo check`/`cargo test` in the repo fails at the link step
(`error: linker 'mold' not found` or equivalent), with no repo-provided remediation or message pointing at
the cause. The devcontainer is unaffected (`mold` is baked into `.devcontainer/Dockerfile:67`).
**Acceptance**: a native Fedora or Arch Linux machine can `cargo check -p <any workspace crate>` after
running `🐚️.sh setup`, or receives a clear, actionable repo-authored error instead of a bare
linker-not-found failure.
**Fix direction**: schema-first — make the mold rustflag conditional on mold's actual presence rather than
distro-specific installer branches: probe `command -v mold` (or the Rust-side build script equivalent)
once, in one place, and fall back to the platform's default linker when absent; separately, add `dnf`/
`pacman`/`zypper` branches to `ensure_cpp_toolchain` alongside the existing `apt-get` branch so the
"install what you can" intent that already exists for apt also exists for the other package managers.

### P1 — the same apt-get install has no failure guard, so a transient apt hiccup kills the entire native setup
`🐚️.sh:8` sets `set -euo pipefail` for the whole script. The Linux `mold`/build-essential install at
lines 428/431 has **no** `|| true` / `|| log …` guard — contrast with `install_linux_fuse_deps`
(same file, ~15 lines above `ensure_cpp_toolchain`), which wraps its own `apt-get install` in
`|| log "Optional apt packages skipped (non-fatal)."`. Any apt failure here (a lock held by
`unattended-upgrades`, a stale package index, an LTS release whose repos don't carry `mold`) aborts the
**entire** bootstrap script immediately — before `ensure_native_neo4j`, before `repo_bootstrap` (`bun
install`, `bun nx run workspace:setup`) ever runs. A single missing apt package on this line silently
prevents zero-touch setup from reaching any of the steps that don't need it.
**Acceptance**: an apt failure in this step is logged and setup continues with the rest of
`repo_bootstrap`, or fails with a message scoped to "C++/mold toolchain," not a bare `set -e` abort.
**Fix direction**: apply the same `|| log "..."` pattern already used one function away in the same file —
no new mechanism needed, just consistency within the file that already has the right pattern once.

### P2 — Neo4j Desktop native install is apt/Homebrew-only (documented, graceful degradation — not a break)
`install_neo4j_desktop_linux_appimage` (`🐚️.sh`) additionally checks `uname -m == x86_64` and prints a
manual install URL for other architectures rather than failing; `install_linux_fuse_deps` degrades the same
way on non-apt systems. Listed for completeness per the task's "every manual step" ask, not because it is a
concrete break — it already fails soft with an actionable message, unlike the mold issue above.

---

## Devcontainer (Linux container, any host OS)

### Verified intact — zero-touch chain unregressed since the 2026-09-19 Z1 fix
Re-checked live this session, 6 days and ~2,700 files of concurrent fleet edits after Z1 landed it:
- `.devcontainer/devcontainer.json:37`: `"postCreateCommand": ["bun", "nx", "run", "workspace:setup"]` —
  unchanged.
- `📋️project.json:623-636`: `setup.dependsOn` still lists all nine `deps-*` targets plus `setup-git`,
  `prepare`, `repo-mcp:build`, `@semio-tech/framework-os-mcp-rs:build` — the exact set Z1 landed, still
  present verbatim.
- `.devcontainer/devcontainer.json:25-27`: the Rust feature is still pinned to `"version": "none"`, so
  `rust-toolchain.toml`'s nightly pin remains the single source of truth inside the container (no
  redundant/mismatched second toolchain, closing G4's P1 finding for good inside the container).
- `mold` is present in `.devcontainer/Dockerfile:67`, so the native-Linux P0 above does not reach the
  devcontainer.

No action needed here; recorded per AGENTS.md's "validate, don't assume" rather than trusting the 6-day-old
Z1 report at face value.

### P2 — `postStartCommand`/`postAttachCommand` hardcode `bash` (correctly scoped, not a real risk)
`devcontainer.json:38-39`: `"postStartCommand": "bash .devcontainer/post-start.sh"`,
`"postAttachCommand": "bash .devcontainer/post-attach.sh"`. These execute only inside the container itself
(a fixed Debian-based image with bash guaranteed present), never on the host, so this is not a
cross-platform break — listed only because the task explicitly asks to surface every shell-specific
invocation for completeness.

### P1 — the devcontainer cannot substitute for native-platform verification (process gap, not a code gap)
Carried forward from `.tmp-ticket/📓️audit-s11-build-health.md` §2, re-confirmed this session: every one of
the Windows/Linux findings above (P0 #1, P0 #6, P1 #2/#3/#7) is a **static-source risk**, not an observed
failure — no ticket capture anywhere shows either native path actually executed on real hardware. The
devcontainer being solid does not close the AGENTS.md "zero-touch and cross-platform … for devcontainer,
native windows, native macos and native linux" requirement; it closes exactly one of the four legs.
**Acceptance**: at least one real (or CI-provisioned) native-Windows and native-Linux run of
`setup`/`dev s`/`os-hub:dev`, captured with a timestamp, exists in ticket history.

---

## macOS (native)

### Verified clean — no new break found in the files scoped to this pass
- `signExecutableForDistribution` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts:19-23`)
  correctly no-ops off-Darwin (`if (process.platform !== "darwin") return;`) before shelling out to
  `codesign --force --sign - <binary>`; its caller `installExecutable` (same file, lines 30-36) does
  remove-then-copy-then-sign, avoiding the documented in-place-Mach-O-overwrite `SIGKILL` trap (project
  memory `project-macos-inplace-binary-overwrite-sigkill.md`).
- `🌎️hub/🗿️artifact-authority/🔒️file-fence/🦀️.rs` hand-implements the *same* file-lock abstraction twice:
  `#[cfg(unix)]` via a raw `flock(2)` FFI shim, `#[cfg(windows)]` via `LockFileEx`/`UnlockFileEx` FFI — no
  external crate, both real platforms covered, matching AGENTS.md's "no runtime deps on external
  libraries" + multi-implementation rules exactly. This is the pattern the process-kill finding below (P1
  #13) should be brought up to.
- macOS remains the only platform continuously exercised — every capture across `.tmp-ticket` and
  `.tmp-ticket-0918` (hundreds of files, six days of ticket history) is from this one darwin machine.

---

## Cross-platform (all three native targets equally)

### P1 — process kills target a single PID, not a process group/job; this exact failure class has already bitten this repo's own fleet
`🌎️hub/📦️packages/🦀️rust/📜️script.ts`:
- line 9345: `if (mode === "signal") process.kill(process.pid, "SIGKILL");`
- line 10442: `const terminate = (reason: string): void => { if (!failed) failed = reason; child.kill("SIGKILL"); };`
- line 11187: `if (child.exitCode === null) child.kill("SIGKILL");`

None of these spawn with `detached: true` + group-kill (POSIX: `process.kill(-child.pid, …)`) or a Windows
Job Object (`CREATE_NEW_PROCESS_GROUP` + `TerminateJobObject`) — each kills exactly one PID. Any grandchild
the killed process spawned (a `cargo` invocation spawning `rustc`, a wgpu-native child spawning a further
worker) survives on both POSIX and Windows. This is not speculative for this repo: project memory records
at least three separate, real incidents describing exactly "kill the wrapper, the build child survives and
holds a lock" on this same fleet — "Killing Wrapper Orphans Cargo" (pkill on a wrapper orphans cargo; must
kill the ppid=1 child instead), "Killed Build Orphan Holds Target Lock" (cargo+rustc survive an orchestrator
kill and keep holding the shared target-dir lock), "Fleet Cut Leaves Deadlocked Cargo Orphans" (idle cargos
plus zero rustc = an flock cycle requiring a manual kill set). All three are instances of the same missing
mechanism these three call sites also lack.
**Acceptance**: killing a hub-spawned build/test child (the `deliverNativeCredentialEnvelope`/
`buildCargoArtifacts` families this file also drives) leaves zero orphaned descendant processes, verified
by a live process-tree check on macOS, Linux, and (when available) Windows.
**Fix direction**: one portable spawn/kill wrapper (schema-first: same shape as the file-fence lock's
`#[cfg(unix)]`/`#[cfg(windows)]` split, but at the Node child-process layer since these are TS-spawned
processes) — `detached: true` + negative-PID kill on POSIX, a Job Object on Windows — used everywhere this
file currently calls `.kill("SIGKILL")` directly, rather than three separate ad-hoc call sites.

### Verified clean — the one `lsof` call in product code is Windows-guarded
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:2929-2939` (`describeDevPortOccupant`, "Reads
who is listening on `port`") is the only place in `🧰️framework`/`🌎️hub` that shells out to `lsof`
(`spawnSync("lsof", ["-nP", \`-iTCP:${port}\`, "-sTCP:LISTEN"], …)` — the exact invocation shape the
session-11 preamble itself uses for port hygiene). It is correctly guarded:
`if (process.platform === "win32") return undefined;` at the top of the function, so its one caller
(`stopTrunkDevPort`) degrades to a harmless no-op on Windows rather than throwing. The real port-reservation
path (`isDevPortInUse`, `🟦️.ts:2859`) uses Node's `net` module, not `lsof`, confirming Z1's
2026-09-19 finding still holds. `.devcontainer/Dockerfile:25` installs `lsof` for the container, consistent
with this being a Unix/macOS/Linux-only diagnostic. Every other `lsof` hit in the repo
(`🪝️hooks/**`, `🏃️test-runner/**`) is a Claude-Code/Codex agent-tool-safety pattern that *blocks*
`kill $(lsof -t -i:PORT)` command injection — unrelated to product runtime cross-platform behavior.

### Verified clean — `.vscode/launch.json` has no POSIX-shell-dependent rows
Checked as explicitly asked by the task ("launch.json runtimeExecutables that assume a POSIX shell"): all
437 `.vscode/launch.json` entries are `"type": "node-terminal"` with a single `"command"` string — **zero**
`"runtimeExecutable"` keys exist in the file. Grepped every `"command"` value for shell metacharacters
(`&&`, `||`, bare `|`, backtick, `$(...)`, `;`) — zero hits. VS Code's `node-terminal` debug type runs the
command in the OS's default integrated-terminal shell regardless of which one that is, and since no command
string uses shell-specific syntax, this works identically under bash, zsh, PowerShell, or cmd.exe. This
item in the task's checklist does not apply to the current tree — worth recording so a future audit doesn't
re-derive it.

---

## Ranked findings

**P0**
1. Windows native: MAX_PATH — 308–357-char relative paths already exist in real build/test output; no
   `core.longpaths`/`LongPathsEnabled`/`\\?\` handling anywhere. Blocks `git clone` → build/test on stock
   Windows. Owner: whoever picks up native-Windows verification (currently nobody has, per Devcontainer §
   P1).
2. Native Linux: `mold` linker required unconditionally by `.cargo/config.toml`, installed only via
   `apt-get`. Blocks the first `cargo build` on any non-Debian/Ubuntu native Linux machine.

**P1**
3. `🔵️.ps1` has no UTF-8 BOM but runs under Windows PowerShell 5.1 (`powershell.exe`, not `pwsh`) —
   mojibake/parse risk on non-US-English Windows.
4. `🔵️.ps1` destructively `Set-Content`s the user's global `~/.cargo/config.toml`, asymmetric with the
   Unix native path and redundant given the repo-local config already wins.
5. Native Linux's `mold`/build-essential `apt-get install` has no failure guard under `set -euo pipefail`
   — one apt hiccup aborts all of setup, unlike the identical pattern one function away that already
   guards correctly.
6. Process kills (`🌎️hub/…/📜️script.ts:9345,10442,11187`) target a single PID, not a process group/job —
   confirmed recurring incident class in this repo's own fleet history (three separate project-memory
   entries describe the same failure shape).
7. No native-Windows or native-Linux run has ever been captured in ticket history — every finding above is
   unexercised on real hardware; the devcontainer's health does not substitute for it.

**P2**
8. `🔵️.ps1:849` adds only `wasm32-unknown-unknown` via `rustup target add`, not `wasm32-wasip2` — likely
   masked by `deps-wasm` in `workspace:setup`, but misleading as written.
9. Neo4j Desktop native install is apt/Homebrew-only — already degrades gracefully with a manual-step
   message, not a hard break.
10. `postStartCommand`/`postAttachCommand` hardcode `bash` — correctly scoped to the container-only
    lifecycle, not a real cross-platform risk.
11. `go.work:1` (`go 1.25`) vs. the devcontainer's Go feature (`1.26`) — carried over from G4 2026-09-19,
    still unresolved, still harmless (Go's toolchain directive treats it as satisfied).

---

## Honest gaps

- No native Windows or native (non-container) Linux machine was available to actually execute anything in
  this session — every Windows/Linux finding above is a static-source read (file:line, `grep`, `find`),
  consistent with this slice's read-only/no-builds mandate, not a reproduced failure.
- Did not read `.devcontainer/Dockerfile` line-by-line beyond confirming `mold`'s presence; a full package
  audit of that file was out of scope for a 45-minute cross-platform pass and is already covered by G4 §1.
- Did not grep the ~26,000-line root `📜️script.ts` or the ~17,000-line hub `📜️script.ts` exhaustively for
  every one of the task's listed patterns (`perl`, `sample`, `lsof`) — targeted greps for `codesign`,
  `chmodSync`, `symlinkSync`/junction, SIGINT/SIGTERM/SIGKILL, `lsof`, and loopback-binding patterns
  completed (against `🧰️framework`+`🌎️hub`+`.vscode`+`.devcontainer`, excluding `⚡️cache`) and came back
  clean/well-guarded everywhere — see the `lsof` finding above. A `perl`/`sample` sweep of the same scope
  was started but did not finish within the time-box (background greps against the full
  emoji-fixture-heavy tree were too slow to complete cleanly). Not re-run given the 45-minute budget;
  flagged rather than silently dropped or guessed at.
- Did not verify the PowerShell-5.1-without-BOM risk (finding #3) by actually running the script on a
  non-US-English Windows box — no such box was available. The finding is the encoding-mismatch mechanism
  plus the confirmed absence of a BOM and the confirmed `powershell.exe` (not `pwsh`) invocation; whether it
  actually mis-parses on a given locale was not observed.
- Line-ending (`.gitattributes`) and executable-bit handling were checked and found clean (`*.sh` forced
  LF, `*.bat`/`*.cmd` forced CRLF, binaries correctly marked, `.sh`/`.ps1` are always invoked as
  `bash <path>`/`powershell.exe -File <path>` rather than executed directly, so the checked-out executable
  bit is never load-bearing) — not written up as a separate section since nothing concrete was found to
  report.

Sources: `AGENTS.md`; `.tmp-ticket/📓️session-11-preamble.md`; `.tmp-ticket-0918/📓️{g4-zero-touch-and-run-paths,z1-zero-touch-and-launch-rows,g12-deploy-and-onboarding-audit}.md`;
`.tmp-ticket/📓️audit-s11-build-health.md`; `.vscode/launch.json`; `.devcontainer/{devcontainer.json,Dockerfile}`;
root `📜️script.ts`; `🌎️hub/📦️packages/🦀️rust/📜️script.ts`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/{🐚️.sh,🔵️.ps1}`;
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`;
`🌎️hub/🗿️artifact-authority/🔒️file-fence/🦀️.rs`; `.cargo/config.toml`; `rust-toolchain.toml`; `bunfig.toml`;
`.gitattributes`; `📋️project.json`; live `find`/`grep`/`xxd` commands run this session.
