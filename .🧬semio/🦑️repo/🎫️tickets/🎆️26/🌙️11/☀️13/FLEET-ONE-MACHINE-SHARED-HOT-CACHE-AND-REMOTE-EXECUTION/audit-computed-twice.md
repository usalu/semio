# Audit: What Is Computed On Every Machine And Not Covered By Packages A to E

Read-only exploration of `/Users/Shared/semio` on 2026-11-13. No builds, no git writes, no services touched, no sub-agents.

Abbreviations: `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, `PROC` = `🧰️framework/🔨️modules/🏃️process/`. Paths are relative to the repo root. Facts carry file and line; **(estimate)** marks my judgement; **(guess)** marks unverified inference. "Covered" means: packages A to E in `design.md` address it.

Packages recap: A = fleet cache endpoint and Nx remote cache on. B = Rust unit cache remote tier and claims. C = slot retention. D = load reports and scheduler. E = Rust offload.

## Headlines

1. The biggest uncovered item is the Nx project graph. Every daemonless `nx` rebuilds a 158 MB graph of 1271 projects on each machine, 3.5 to 6 min under fleet load (memory note, 2026-10-07). Nothing shares it. None of A to E covers it.
2. Boot-time repo walks repeat too: 3 to 5 full discovery walks per dev boot, 80 to 210 s each under load (memory note, 2026-09-05). The generated catalogs (`🤖️generated/🔌️plugins.json`, `🎠️playgrounds.json`) are gitignored, so every machine regenerates them.
3. Wasm plugin builds are only half-cacheable. The 45 `wasm` targets with `cache: true` declare only `.🧾️store/wasm.json` as output. The payload (example: 123 MB in `.cache/b/p/m/browser/prototype/wasm-packages/…`) is not declared, so an Nx hit restores a receipt without the payload. Package B covers only rustc units.
4. Nx has no single-flight (`map-build-path.md` headline 1): two machines running the same Nx task compute it twice. Package B's claims cover Rust units only.
5. Target hygiene blocks sharing: 632 targets have no `cache` flag and 657 set `cache: false`. About 500 of the uncached ones are `test` or `check` (nx.json has no `targetDefaults`). 2393 targets are `cache: true` with no declared outputs, so a hit replays nothing.
6. Dependency installs are per machine and not Nx-routed: `.cache/b/d` is not a cacheable partition (`LIB/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`). The bun cache is 6.4 GB per machine (measured). Installed `node_modules` are absolute symlinks into `/Users/ueli/Documents/semio/...`, which do not resolve on other machines (`.gitignore:681` acknowledges this).
7. Per-user toolchain caches are large and duplicated per machine (measured here): `~/.rustup` 9.2 GB, `~/.cache/uv` 23.5 GB, `~/.bun/install/cache` 6.4 GB, `~/go/pkg/mod` 3.7 GB, `~/.cargo/registry` 1.7 GB, `~/Library/Caches/ms-playwright` 0.57 GB. No mirror is configured; `RUSTUP_DIST_SERVER` defaults to static.rust-lang.org.
8. The LaTeX print pipeline downloads a 2.88 GB TeX bundle (`LIB/…` print module `📚️bundle/🔒️dependencies.json:2-3`, pinned by sha256) plus the tectonic 0.16.9 binary from GitHub, once per machine. Both sit in the uncached `tools` partition, and the print targets are `cache: false`. A mirror needs only a changed base URL, because the sha256 pins already exist.
9. Cargo lanes: each agent compiles in its own lane (`.cargo/config.toml:9-11`, one target and build dir per lane). Only rustc units are shared. Build-script execution, linking and the component-model steps are outside the unit cache (rustc link crate types are bypassed, `map-remote-cache.md` 2e). **(guess)** these repeat per lane.
10. Generation and codegen targets are the second Nx-hygiene risk: 13 `generate` targets, 36 `🏭️generator` and 32 `🏭️generator-manifests` targets are `cache: false`, and 29 `preview-generated` targets are `cache: true` with no outputs. A cache hit would skip regeneration without restoring its files.
11. Gitignored inputs are invisible to Nx hashing (memory note 2026-09-11; `.gitignore:97` ignores `🤖️generated`). Sharing any `check-*` target against generated files is unsafe without a runtime byte-digest input.
12. Smallest high-value order (estimate): (a) publish the Nx project graph by digest, (b) declare the wasm payload in outputs, (c) mirror the TeX bundle and toolchains on the group server, (d) add Nx claims, (e) make generated catalogs Nx outputs. Report: `…/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/audit-computed-twice.md`.

---

## 1. Ranked inventory

Savings are fleet-wide estimates over 8 machines. "Nx-routed" means the work runs as an Nx task, so package A (Nx remote cache) can share it once the outputs are right.

| Rank | Expensive work | Per-machine cost (source) | Nx-routed | Machine-independent output | Covered by A to E | Smallest change | Fleet saving (estimate) |
|---|---|---|---|---|---|---|---|
| 1 | Nx project graph construction | 158 MB, 1271 projects, 3.5 to 6 min per daemonless `nx` under load (memory note 2026-10-07) | no (it precedes Nx) | yes, if keyed by project.json and plugin digests and relative paths | no | Publish `project-graph.json` into the group store keyed by a digest of project.json, nx.json and plugin sources, and reuse it through the existing `SEMIO_BUILD_GRAPH_DIGEST` switch (`LIB/⚡️caching/🚀️bootstrap/📜️script.ts:253`) | Very high (estimate): every `nx` run on 8 machines |
| 2 | Discovery walks at boot | 80 to 210 s per walk under load, 3 to 5 walks per dev boot (memory note 2026-09-05; fix reads `🤖️generated/🔌️plugins.json`) | no | yes, by the generated catalog | no | Make the generated catalogs Nx outputs of their generator targets and share them with A | High for dev boot (estimate) |
| 3 | Wasm plugin payloads and activation | 45 `wasm` targets `cache: true`, payload 123 MB per prototype package (`🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/.🧾️store/wasm.json`) | yes, but outputs wrong | yes per target triple and profile | partly (B covers rustc units, not the Nx output) | Declare the payload directory (`.cache/b/p/…`, a cacheable prefix) in `outputs` next to `.🧾️store/wasm.json`; set `cache: true` on `activate-lanes-prototype` (currently unset) | High (estimate) |
| 4 | Nx task single-flight | two machines compute the same Nx task concurrently (`map-build-path.md` headline 1: single-flight `missing`) | yes | n/a | no (B claims are Rust-only) | Use the B claim API keyed by the Nx hash | Medium (estimate) |
| 5 | Target hygiene: uncached and no-output tasks | 632 with no flag, 657 `cache: false`, 2393 `cache: true` with no outputs (sampled below) | yes | varies | no | Declare outputs for generators; enable cache on `test` and `check` after adding runtime digests where needed | Medium to high (estimate); unknown per-task cost |
| 6 | TeX bundle and tectonic download | 2.88 GB tar plus binary, once per machine (`📚️bundle/🔒️dependencies.json:2-3`, `🔧️toolchain/🔣️.json:3`); stored in the `tools` partition (`📦️artifacts/📥️material/📤️producer/🟦️.ts:23`) | no (print targets are `cache: false`) | yes, sha256 pinned | no | Point the `url` base to a fleet mirror with the same sha256 check | Medium (estimate): 3 GB per machine, one time |
| 7 | Toolchains and package caches | rustup 9.2 GB, uv 23.5 GB, bun 6.4 GB, go mod 3.7 GB, cargo registry 1.7 GB, playwright 0.57 GB (measured) | no | yes per platform | no | Environment overrides to a group mirror: `RUSTUP_DIST_SERVER`, cargo source replacement, `GOPROXY`, `BUN_INSTALL_CACHE_DIR`, `UV_CACHE_DIR` | Medium (estimate): mostly bandwidth and disk |
| 8 | Dependency install store `.cache/b/d/m/j/<hash>` | per machine, not a cacheable partition | no | yes per lock hash and platform (native addons differ) | no | A shared install store keyed by lock hash, plus relative symlinks instead of the absolute `/Users/ueli/Documents/…` ones | Medium (estimate) |
| 9 | Go builds | `go build -p N -o artifact` (`PROC/📦️artifacts/🐹️go-build/🟦️.ts`); `GOCACHE` per repo cache dir (`PROC/🌿️environment/🟦️.ts:40`) | partly (5 `go` targets `cache: true`) | yes per GOOS and GOARCH | no | `GOCACHE` and `GOMODCACHE` on the group store, `GOPROXY` to a mirror | Low to medium (estimate) |
| 10 | Nx `cache: true` tasks whose generated files are not restored | 29 `preview-generated`, 3 `typegen` without outputs | yes | yes | no | Declare the generated paths as outputs | Medium (estimate), correctness first |
| 11 | CMake C++ (`cpp-setup`, `cpp-configure`, `cpp-build`, `cpp-test`) | `cache: false` on all four, plus two `cache: true` | no | toolchain-specific | no | Route through Nx with declared build dir outputs | Low (estimate); no timing data found |
| 12 | Playwright browsers | 568 MB per machine (measured); `test-e2e` `cache: false` (3 targets) | no | yes per version | no | Browsers from a mirror | Low |
| 13 | Bazel | 9 Bazel test commands, all `cache: false`; the backend is not the default (`LIB/🏗️build/🔌️backends/📋️registry/🔣️.json`) | no | n/a | no | None unless Bazel becomes the default | Negligible now |

## 2. Facts per item

### 2.1 Nx project graph (rank 1)
- `nx.json:6` sets `"useDaemonProcess": true`, but the backend registry sets `NX_DAEMON=false` for the `nx` row (`map-build-path.md` headline 1, citing `LIB/🏗️build/🔌️backends/📋️registry/🔣️.json`). Under fleet load, the daemon failed to start within 60 s (memory note).
- `LIB/⚡️caching/🚀️bootstrap/📜️script.ts:253`: `SEMIO_BUILD_GRAPH_DIGEST` forces `NX_FORCE_REUSE_CACHED_GRAPH=false`. This is the existing hook for a digest-keyed graph.
- `LIB/🏗️build/🕸️graph/🕰️host/🟦️.ts:112` and `:118`: the semio graph generation exchange is on only when a remote contract exists (`LIB/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts:92-96`). Its key `generationExchangeKeyV1` (line 27) includes the absolute workspace root. It covers semio's own generations, not `.nx/workspace-data/project-graph.json`. A grep for `project-graph` in non-test files under `🕸️graph` returned nothing.
- `.nx/workspace-data` is gitignored (`.gitignore:65`) and local to each machine.
- Smallest change: write `project-graph.json` into the group store under a digest of project.json, nx.json and plugin sources (the existing `buildGraphGenerationIdentityV1` at `🏷️retention/🟦️.ts:18` is the pattern), and reuse it when the digest matches.

### 2.2 Discovery and generated catalogs (rank 2)
- Memory note `project-dev-boot-repo-walks-and-taxonomy-churn` (2026-09-05): `ensurePluginRegistry` plus `writePlaygroundSession` walked 3 to 5 times. One walk took 80 to 210 s under load. A fix reads `🤖️generated/🔌️plugins.json` and `🎠️playgrounds.json`.
- `.gitignore:97` ignores `**/🤖️generated/`, so the catalogs are per machine.
- Smallest change: declare the catalogs as outputs of their generator targets (so Nx restores them) and let A share them.

### 2.3 Wasm plugins and activation (rank 3)
- 45 `wasm` targets have `cache: true`. Their only output is `{projectRoot}/.🧾️store/wasm.json` (example: `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🧠️lsp/📦️packages/🦀️rust/📋️project.json`, target `wasm`).
- The receipt (`🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/.🧾️store/wasm.json`, 369 bytes) names a 123 MB directory `.cache/b/p/m/browser/prototype/wasm-packages/d30e56a055ea2d5bde08382c` (7 files). That payload is not in `outputs`.
- Consequence (estimate from declared outputs): an Nx hit would restore the receipt but not the payload. The task must re-verify or rebuild, which is what `verifyProducedWasmPackageV1` (`PROC/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts`) is for. A hit is therefore not a full hit.
- `activate-prototype` has `cache: true` and output `dist/♻️activation/prototype`. `activate-lanes-prototype` has no cache flag. `serve*` and `dev*` are `cache: false`, which is correct for long-running tasks.
- Rust compile per plugin is one cargo pass per workspace per profile (`LIB/⚡️caching/🦀️cargo/🟦️.ts:57`, `componentPassPlanV1`). Rust units are covered by B. Component linking and jco transpile steps (`generate-jco-package-adapter` at `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json`) are not rustc units.

### 2.4 Nx task single-flight (rank 4)
- `map-build-path.md` headline 1: Nx `single-flight` capability is `missing`.
- Package B's claims cover rustc units only. Package A makes the Nx cache reachable but does not stop two cold runs.

### 2.5 Target hygiene (rank 5)
Counts from the 1562 `project.json` files under `🧰️framework`, `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech`, `♻️mit-bestand`, `👴️leutwiler` (root `📋️project.json` included in the 1563 total):
- 3997 targets: 2708 `cache: true`, 657 `cache: false`, 632 no flag.
- Uncached (no flag): `test` 228, `check` 165, `test-snapshot-sqlite` 61, `test-snapshot-sqlite-native` 57, `test-snapshot-sqlite-source` 57.
- `cache: true` with no outputs: `test` 286, `check` 69, `test-snapshot-sqlite-native` 44, `canonical-architecture` 33, `preview-generated` 29, `build` 28, `typecheck` 17.
- `cache: true` with in-workspace outputs (not `.cache` and not `.🧾️store`): 55, for example `graph-generate` 9, `build` 9, `typegen` 3.
- `nx.json` has no `targetDefaults`. Per Nx semantics, unflagged targets are uncached (**guess**, not checked against this Nx version).
- The plugin rule `nxOutputsCacheableV1` (`LIB/🟨️.mjs:90`) downgrades outputs outside `rust` and `products` to `cache: false` (`🟨️.mjs:95`). It is applied only in `projectBuildTargetForNxV1`, which reads `execution` metadata. 47 targets in `🧰️framework/🔨️modules/🏃️process/📋️project.json` carry `execution`, most plugin targets do not. **Unverified:** whether plain `repo:script` targets with in-workspace outputs are downgraded. If not, the 55 above are cached with outputs Nx may restore outside the cacheable partitions.
- Cacheable partitions are `rust` and `products` only (`LIB/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`, partitions at lines 34 to 60). `tools`, `dependencies`, `graph`, `diagnostics`, `nx` and `leases` are excluded.
- Gitignored generated inputs are invisible to Nx hashing (memory note 2026-09-11, `project-nx-hasher-blind-to-gitignored-inputs`). Checks against `🤖️generated` need a runtime digest input.

### 2.6 TeX bundle and tectonic (rank 6)
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/🔒️dependencies.json:2-3`: `url` `https://data1b.fullyjustified.net/tlextras-2022.0r0.tar`, `archiveBytes` 2881562112, sha256 pinned, per-file offsets and sha256.
- `…/🔧️toolchain/🔣️.json:3`: tectonic `0.16.9` from `github.com/tectonic-typesetting/tectonic/releases`.
- `…/📦️packages/🟦️typescript/📋️project.json`: `deps-tectonic` and `deps-tex` `cache: false`; `build`, `watch` and `build-viz` `cache: false`; `test-toolchain` `cache: true`.
- Print material lands in the `tools` partition (`📦️artifacts/📥️material/📤️producer/🟦️.ts:23`, `context.store("tools")` for tectonic and tex), which is not Nx-cacheable.
- The same tectonic and TeX downloads also exist in `~/Library/Caches/Tectonic` (99 MB here).

### 2.7 Toolchains and package caches (rank 7)
Measured on this machine:
- `~/.rustup` 9.2 GB; `~/.cargo/registry` 1.7 GB (`~/.cargo` also holds `config.toml`, `env`).
- `~/.cache/uv` 23.5 GB. The repo sets `UV_CACHE_DIR` into the dependencies partition (`LIB/🏗️build/🧰️tool-setup/⌨️command/🟦️.ts:167`), so two uv caches exist.
- `~/.bun/install/cache` 6.4 GB.
- `~/go/pkg/mod` 3.7 GB; `~/Library/Caches/go-build` 105 MB.
- `~/Library/Caches/ms-playwright` 568 MB.
- `~/Library/Caches/semio/runtimes` 60 MB (bun, binaryen).
- `~/.cache/semio` 8.6 GB (host CAS, which also holds the remote-cache blobs, `map-remote-cache.md` section 1).

Download sources, none mirrored:
- `static.rust-lang.org` (rustup and nightly dist): `LIB/⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/📥️bootstrap/🟦️.ts` hard-codes `RUSTUP_DIST_SERVER:"https://static.rust-lang.org"`. The compiler wrapper passes `RUSTUP_DIST_SERVER` through from `generation.environment` (`LIB/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts`), so a mirror is a configuration change.
- `github.com/oven-sh/bun/releases/download/bun-v1.4.2` and `WebAssembly/binaryen` releases (bootstrap).
- `registry.npmjs.org`, `pypi.org` and `files.pythonhosted.org` (dependencies).
- Cargo: `cargo fetch --locked` (`PROC/🦀️cargo/📥️dependencies/🟦️.ts:45`). The registry home is bounded; there is no `[source]` replacement in `.cargo/config.toml`.

### 2.8 Dependency installs (rank 8)
- Install store: `.cache/b/d/m/j/<lock-hash>/w/<n>/node_modules`. Each package has `node_modules -> /Users/ueli/Documents/semio/.cache/b/d/m/j/…` (for example `🧰️framework/🛍️products/🦑️repo/node_modules`, `…/🦀️rust/…/node_modules`). `.nx/installation` is the same kind of link.
- Those absolute paths are the old checkout location, so on the other 7 macOS machines they resolve only if the path exists. **(guess)**: the replica daemon may not carry symlinks at all; not verified.
- Bun manager: `LIB/⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🍞️bun/🟦️.ts`. Uv manager: `…/🐍️uv`. Go: `…/🐹️go`. Cargo: `…/🦀️cargo`.
- `.gitignore:681` states the install symlink points to an absolute path.

### 2.9 Go (rank 9)
- `PROC/📦️artifacts/🐹️go-build/🟦️.ts`: `go build -p=<granted parallelism> [-o artifact]`.
- `LIB/🏃️process/🌿️environment/🟦️.ts:40`: `GOCACHE ??= repoCacheDirectory(repoRoot, "go")`. Per machine.
- `LIB/🧪️testing/🐹️go-plan/🟦️.ts` and `pinnedGoEnvironmentV1`: pinned Go environment. Not read in full.
- Five `go` targets are `cache: true` (for example `test-go-build-mode` in `PROC/📋️project.json`). `test-go-executable-acceptance` is `cache: false`.

### 2.10 Codegen and generators (rank 10)
- `🏭️generator` 36 `cache: false`, `🏭️generator-manifests` 32 `cache: false`, `🏭️generator-encryption` 4 `cache: false`.
- `generate` 13 targets, `generate-*` mixed.
- `preview-generated` 29 `cache: true` with no outputs (example `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust/📋️project.json`).
- `typegen` 3 with no outputs.

### 2.11 Cargo lanes and build scripts (cargo layer)
- `.cargo/config.toml:1-4`: every agent compiles in its own lane (`m/u/l/<lane>`). Target and build dirs are per lane (`.cargo/config.toml:10-11`).
- The unit cache covers rustc invocations. Rustc link crate types are bypassed (`rustcBypassV1`, `map-remote-cache.md` 2e). Build-script execution (`run-custom-build`) is cargo-level. **(guess)**: it repeats per lane and is not in the unit cache.
- Cargo fingerprint and dep-info are per lane and not portable (`map-build-path.md` section 5).
- Profiles are in the root `Cargo.toml`, which I did not read in full.

### 2.12 Nx hygiene and the other families
- Test families: `test-cargo-*` targets with `cache: false` include `test-cargo-build-lease`, `test-cargo-build-custody`, `test-exact-cargo-storage`, `test-cargo-target-recipe`, `test-cargo-overlap`, `test-cargo-workspace`, `test-cargo-units`. These are process and lease tests, so uncached is likely deliberate.
- `test-storybook-subjects` and `test-storybook-catalog` are `cache: true` (bun and vitest).
- Playwright: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/…` uses `playwright install` paths. `test-e2e` in the demonstrator is `cache: false`.

## 3. Machine-local by design (no sharing wanted)
- `.cache/b/l/m` (APFS image `fixed.dmg`, 64 GB, mounted `disk5s1`) holds leases. `.cache/b/o/m` (`disk7s1`, 64 GB) holds diagnostics. Leases must stay local.
- `.cache/b/r`, `.cache/b/t`, `.cache/b/d`, `.cache/b/p`, `.cache/b/n` and `.cache/b/g` do not exist on this machine right now (`ls .cache/b` shows only `c`, `l`, `o`). The rust, products and dependencies partitions are created on demand or mounted separately. This is an observation; I did not trace the mount logic.
- Disk: `/System/Volumes/Data` 926 GiB, 804 GiB used, 99 GiB free, 90 percent (measured). Rust partition capacity is 96 GiB (`policy.json` partitions.rust `capacityBytes` 103080263680). Retention by slot (package C) is the relevant lever here.

## 4. Open questions (unverified)
1. Whether plain `repo:script` targets with in-workspace outputs are downgraded to `cache: false` (section 2.5). This decides whether the 55 targets are really cached.
2. Whether the replica daemon copies symlinks. Decides whether dependency installs are shared (section 2.8).
3. Whether Nx 23 treats unflagged targets as uncached without `targetDefaults` (section 2.5).
4. The exact wasm argv and link steps (`PROC/📦️artifacts/🏗️native-build/`), not read.
5. Whether `buildGraphGenerationIdentityV1` (`🏷️retention/🟦️.ts:18`) is stable across the 8 machines (it hashes owner roots and source paths, which look relative; not checked against a second machine).
6. Build-script (`run-custom-build`) cost per lane (section 2.11).
7. The sealed partition (`🏛️sealed/README.md`) and vite storage (`LIB/⚡️caching/🌐️vite/`) were not read.

## 5. Sources read
- `design.md` and the four `map-*.md` files in this ticket folder.
- `nx.json` lines 6, 143 to 144 and namedInputs; no `targetDefaults` key.
- `LIB/🟨️.mjs` lines 88 to 96 (cacheability rule and target projection).
- `LIB/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` (partitions).
- `LIB/⚡️caching/🦀️cargo/📦️units/🔌️nx/🟦️.ts` (rust unit Nx projection, `rustUnitNxProjectV1`, `cache: boundedCache`).
- `LIB/🏗️build/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts` lines 27, 92 to 106; `LIB/🏗️build/🕸️graph/🕰️host/🟦️.ts` lines 112, 118; `LIB/🏗️build/🕸️graph/🏷️retention/🟦️.ts` line 18.
- `LIB/⚡️caching/🚀️bootstrap/📜️script.ts` lines 253 to 262.
- Print module under `🧰️framework/🛍️products/📓️print/` (bundle, toolchain, producer).
- `.cargo/config.toml`, `.gitignore` lines 13, 20, 65 to 67, 97, 681.
- User memory notes (cited as memory, not verified here): nx graph construction, dev boot walks, Nx hasher gitignored inputs.
- Measurements: `du` and `ls` on the caches above, `mount`, `df`. No builds, no git writes.
