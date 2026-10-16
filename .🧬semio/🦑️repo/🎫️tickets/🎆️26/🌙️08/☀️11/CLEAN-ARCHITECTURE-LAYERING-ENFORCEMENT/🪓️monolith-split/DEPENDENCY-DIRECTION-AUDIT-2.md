# 🧭 Dependency Direction Audit, Second Pass (2026-10-16)

Read-only re-run of the baseline `DEPENDENCY-DIRECTION-AUDIT.md` (2026-10-15) on today's tree. No source file was edited. No modifying git command was run. No worktree was created. No ticket tool was called. No other agent's process was killed.

Raw outputs are in `🗑️generated/dda2-*` next to this file. Those are the scripts and their output files. Nothing in any `🗑️generated` folder was deleted.

Note on the baseline raw files: the baseline names `gate-*`, `depcruise-*`, `cargo-*`, `rust-include-*`, `ts-*` and `plugin-dir-*` files in `🗑️generated/`. None of them exist there now. The "before" column below therefore comes from the baseline document itself, not from re-reading the old raw files.

## 1. Commands run

All long commands were capped with `perl -e 'alarm N; exec @ARGV'`. Commands over 120 s ran in the background with a status file or a Monitor watch.

| # | Command (abbreviated) | Exit | Counts | Raw output |
|---|---|---|---|---|
| 1 | `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config …/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type err` (alarm 900) | **37** (cruiser: violations found) | 16421 error lines, 16748 modules, 44465 dependencies | `dda2-depcruise-err.txt`, `dda2-depcruise-status.txt`, `dda2-depcruise-rule-counts.txt` (170 distinct rules) |
| 2 | `bun ./📜️script.ts verify dependencies literal-external` (alarm 900) | **1** (target 0 not met) | target 0, current 300; rust 109, js 158, python 36, go 0 literal-external (25 first-party) | `dda2-literal-external.txt`, `dda2-literal-external-status.txt` |
| 3 | `bun repo run-many -p @semio-tech/repo-lib -t lint-dependency-direction lint-rust-source-direction lint-cargo-dependency-direction --skip-cache` (the `verify layering` argument shape, `📜️script.ts:8231`) | **1** | Refused by the runner's argument validator before any task ran: `Unexpected build argument before --: lint-rust-source-direction` | `dda2-layering-runner.txt`, `dda2-layering-runner-status.txt` |
| 4 | Same as 3 with repeated `-t` flags (alarm 900) | **142** (SIGALRM, timer expired) | Passed argument parsing. Ran build stages (`build:discover`, `build:command-source-imports`, `build:cargo-owners`, `build:cargo-targets`, `build:playground-*`) and was still in progress at the cap. Zero lines mention a lease. The block is slowness under load, not a lease refusal. | `dda2-layering-runner-2.txt` (3.9 MB) |
| 5 | `git -c core.quotepath=false ls-files '*Cargo.toml'` | 0 | 2640 tracked manifests, of which 176 are under `.🧬semio/` ticket folders | `dda2-cargo-manifests.txt` |
| 6 | `bun dda2-cargo-scan.ts` (reads manifests with `Bun.TOML.parse`, re-applies the `cargoDependencyDirections` policy from `🔣️taxonomy.json`) | 0 | 2464 non-ticket manifests parsed, 31371 path declarations resolved, 0 missing targets. 121 strict violations, 281 metadata problems. | `dda2-cargo-scan.ts`, `dda2-cargo-scan.txt` |
| 7 | `git -c core.quotepath=false grep -n -E '#\[path\|include_str!\|include_bytes!\|include!\(' -- '…/**/*.rs'` | 0 | 67922 matching lines; 31465 remain after dropping comment lines and test or fixture paths | `dda2-rust-include-all.txt`, `dda2-rust-include-noncomment.txt` |
| 8 | `bun dda2-rust-include-scan.ts` (resolves each literal and classifies by area) | 0 | 69049 directives (307 comment lines skipped) | `dda2-rust-include-scan.ts`, `dda2-rust-include-scan.txt` |
| 9 | `git -c core.quotepath=false grep -n -E '"s\.(stdio\|framework\|space\|note\|wfc)…"'` in framework source | 0 | 7 hits, 4 outside comments | `dda2-framework-s-ids.txt` |
| 10 | Grouping, sorting and counting of `dda2-depcruise-err.txt` with `/usr/bin/grep`, `sed`, `awk` | 0 | see section 2 | `dda2-fixpackages-depcruise.txt` |

Other notes:
- `lint dependency-direction`, `lint rust-source-direction` and `lint cargo-dependency-direction` as standalone nx targets were not run. The baseline showed they need the `repo` invocation envelope, and run 4 shows the envelope route is not finishing on this machine right now.
- `verify gate` was not run end to end.
- `git grep` printed `error: daemon terminated` on stderr several times. This is git's fsmonitor daemon, and it did not change any exit status.
- The Cargo scan is a superset of the gate's inventory. The gate walks workspace members and their path closure. This scan takes every tracked non-ticket manifest. Treat the Cargo problem counts as a scope-widened estimate. The Cargo direction policy was re-applied as written, including the `package.metadata.semio.role` lookups.

## 2. Before and after, per rule and per gate

"Before" is the baseline document. "After" is this run. "Prod" means the edge is outside `🧪️tests`. Counts for a rule family use the cruiser's error lines.

| Rule / gate | Baseline (2026-10-15) | Now (2026-10-16) | Status |
|---|---|---|---|
| Cruiser boundary run, total error lines | 14441 (15725 modules, 38437 deps) | **16421** (16748 modules, 44465 deps) | red, worse |
| R1 TS framework/os → s (`framework-no-implementation`, `framework-no-s`, `framework-no-plugin-packages`, `no-cross-technology-🧰️framework-to-✏️s`) | 17 (all test) | **3** (all test, 0 prod) | improved |
| R1 Rust `#[path]`/`include*!` framework → s | 0 | **0** (0 non-test directive edges) | held |
| R1 Cargo path dependency framework → s or hub | 0 | **0** (`cargo-framework-no-implementation`) | held |
| R1 hard-coded `s.` ids in framework production code | about 10 id sites, 21 path lines in 9 files | **4** non-comment `s.` id literals remain (2 in os store `durable-group/🦀️.rs`, 2 in repo `🧪️test` and `🧾️provenance`). The carrier and kernel constants from the baseline are gone. Path lines into `✏️s/🔌️plugins` were **not re-counted** in this run. | improved, not fully re-verified |
| R2 `no-cross-plugin-*` (plugin → other plugin) | 32 | **4** (3 prod, plus 1 same edge with a mojibake rule name). All are `layout → stdio`. | improved |
| R3 `plugin-no-extension-or-artifact-*` | 2 (test) | **0** | improved |
| R4 `framework-no-products` (general → repo/os product) | 77 | **131** (105 prod) | worse |
| R4 `ui-no-framework-packages` | 231 | **256** (207 prod) | worse |
| R4 `renderer-hosts-only-ui` | 466 | **661** (457 prod) | worse |
| Rust `include*!` framework → os product | not measured | **7** (`🌱️value/📸️snapshot/👁️read/🦀️.rs` ×6, `🔤️json/📥️retained/🧬️schema/🦀️.rs` ×1) | new measurement |
| Rust `#[path]` staged plugin sources inside the os renderer tree (`📥️staging/🧲️plugins/…`) | not measured | 16 directives (all inside `🛍️products/💻️os`, none into `✏️s`) | new measurement, flag |
| s → framework internals (`no-cross-technology-✏️s-to-🧰️framework`, `plugins-framework-sdk-only`) | 1758 / 1688 (overlapping) | **1807 / 1763** (997 / 1234 prod, overlapping) | worse |
| s → os store (`no-state-outside-os`) | 80 | **86** (55 prod) | worse |
| Hub → framework (`no-cross-technology-🌎️hub-to-🧰️framework`) | 164 | **177** | worse (hygiene) |
| Hub → s | not listed | **16** (2 prod) | hygiene |
| mit-bestand → framework | 62 | **72** (pattern total, covering the framework subset) | hygiene |
| Cycles (`no-circular`) | 518 | **626** (547 prod) | worse |
| `no-core-path` | 867 | **880** (607 prod) | worse (hygiene) |
| `not-to-unlisted` | 823 | **845** (144 prod) | worse (hygiene) |
| `no-cross-package-relative-*` (per-package boundary rules) | not listed separately | **9085** (5299 prod) | new listing, hygiene/SDK |
| `verify dependencies literal-external` | target 0, current 2543, oracle-conflicts 3 | **target 0, current 300**, oracle-conflicts **4** (`js:i18next`, `js:xstate`, `rust:image`, `rust:serde_json`) | red, large drop. The baseline count may use a different measure, and I cannot reconcile it without the old raw files. |
| Cargo strict direction (`cargo-*` rules, static re-run) | not run; baseline static scan found 0 framework → s | **121** strict violations: `cargo-plugins-no-artifacts` 102, `cargo-modules-no-plugins` 19, `cargo-framework-no-products` 0, `cargo-framework-no-implementation` 0 | new, static estimate |
| Cargo metadata problems (static re-run) | not run | **281**: `missing-role` 233, `owner-role-mismatch` 41, `unclassified-role-owner` 5, `unknown-role` 1, `unclassified-owner` 1 | new, static estimate |
| `lint-dependency-direction`, `lint-rust-source-direction`, `lint-cargo-dependency-direction` (repo-lib run) | not run | **not completed**. Refused by the argument validator (run 3). Ran out of time at 900 s (run 4). No lease refusal was observed. | blocked by runner |

Note on the cargo scope: my `framework_to_products_edges=4256` figure in the raw scan is a scope mistake. It counts edges inside the os product. The policy rule `cargo-framework-no-products` applies only to `🔨️modules` and `📦️packages` under the framework, and it has 0 violations.

## 3. Remaining violating edges, grouped into fix packages

Ranked by production edge count. Edge counts are cruiser error lines outside `🧪️tests` unless marked. The same edge can be in more than one rule family, so counts overlap across packages.

### P1. s → framework internals (SDK boundary), 1234 prod (rule `plugins-framework-sdk-only`; overlaps `no-cross-technology-✏️s-to-🧰️framework`, 997 prod)

Largest flows:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts` → `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot` (173)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts` → `🧰️framework/📦️packages/🟦️typescript/🌱️value` (63)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts` → `sqlite-snapshot` (45) and `🌱️value` (14)
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts` → `🌱️value` (22); `🀄️wfc` → `🌱️value` (18)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts` → `🧬️schema/🗿️artifact-reference` (20) and `🏃️process/📦️packages` (16)

Fix (cleanest long-term): the framework exposes package `exports` subpaths. Plugins import only `@semio-tech/framework-…` specifiers. Run a mechanical codemod and add a lint rule. This is the baseline's items 7 and 8.
Files: `✏️s/🔌️plugins/*/🗿️artifacts/**` and `✏️s/🔌️plugins/*/🧩️extensions/**`. Owner: the plugin crate fleet.

### P2. Renderer hosts only specific things, 457 prod (rule `renderer-hosts-only-ui`)

- 125 edges resolve to `.cache/b/d/m`. Investigate this first. It is probably a generated or cached path that the resolver follows and is not a real source edge.
- 112 intra-os edges (`🛍️products/💻️os/🔨️modules` → itself) and 56 to `💻️os/📦️packages`. These are mostly the os cycle in P5.
- Framework specifics: `🎠️kernel` (16), `📡️replication/📦️packages` (15), `🏃️process/📦️packages` (15), `🎭️actor/📮️shard-client` (15), `🎚️config` (12), `🖼️canvas/🎨️react-renderer` (10), `🎭️actor/🖼️wire-turn` (10), `⏱️trace` (9), `🎭️actor/📦️packages` (7), `🪃️continuation` (5), `🥽️mesh` (4), others.

Fix: the os host injects the hub, session and replication ports into the renderer. The renderer imports only the `ui` contract and the injected ports.
Files: `🛍️products/💻️os/🔨️modules/📺️renderer/**`. Owner: renderer and os fleet. Actor and Media edges are parent-owned.

### P3. UI imports framework root barrel and other packages, 207 prod (rule `ui-no-framework-packages`)

- `🖱️ui/🎨️styling` → `🏃️process/📦️packages` (30), `🧬️schema/✅️validator` (3), `🖼️assets/🔍️resolver` (3)
- `🖱️ui/🧬️contract` → `🌱️value/📦️packages` (21), `🏃️process/📦️packages` (10), `🛂️manifest/🟦️.ts` (7), `🎭️actor/📦️packages` (4), `🎭️actor/📮️shard-client` (3), `🛍️products/🦑️repo/🔨️modules` (8)
- `🖱️ui/🧱️elements` → `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` (15), `🖼️assets/📦️packages` (9), `🕹️interaction/🟦️.ts` (3)
- `🖱️ui/🌐️locale` → `🏃️process/📦️packages` (8); `🖱️ui/🎬️scene` → `🏃️process/📦️packages` (7)
- `🖱️ui/🔨️modules` → root barrel (5); `🖱️ui/🎯️targets` → root barrel (3)

Fix: replace barrel imports with specific subpath imports. Move `🧬️dsl` and ownership types down into the contract leaf if needed.
Files: `🧰️framework/🔨️modules/🖱️ui/**`. Skip `🖱️ui/🪟️viewport/📦️packages/🦀️rust/📜️script.ts`. It imports repo cargo admission and is parent-owned.

### P4. Cycles, 547 prod (rule `no-circular`; 626 total)

- `🛍️products/💻️os/🔨️modules` 296 (largest strongly connected component)
- `🖱️ui/🧬️contract` 40
- `🛍️products/🦑️repo/🔨️modules` 33
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts` 31
- `🛍️products/💻️os/🎚️config` 14
- `📡️replication/🚪️io` 10; `🖼️assets`, `🎠️kernel`, `◻️2d`, `🏃️process/🔒️leases` (3 each)

Fix: run `depcruise --output-type dot` on `os/🔨️modules`, find the smallest cut, and break it. Skip any strongly connected component that touches `🏪️store`, `🎭️actor`, `🪟️member` or `🎬️media`. Those are parent-owned.

### P5. Framework → repo product, 105 prod (rule `framework-no-products`; 131 total)

Largest sources: `🖱️ui/🧬️contract` (8); `🧵️job/📬️outcome` (4), `🧵️job/🔭️preview`, `🧵️job/📦️packages`, `🧵️job/👷️worker` (2 each); `📡️replication/📦️packages` (4); `🎒️pack/📦️packages` (4); `🧬️schema/🧩️composition` (3); `🖼️canvas/📦️packages` (3); plus about 15 more framework modules with 2 each.

Fix: framework code should not depend on repo build infra. Move the cargo admission, caching and storage use out of the framework, or invert it through a port that the repo product implements. Baseline says most of these are Cargo build-infra and parent-owned.
Files: `🧰️framework/🔨️modules/**` (the listed modules).

### P6. Cargo plugin-internal direction, 121 strict (static estimate, see section 4)

- `cargo-plugins-no-artifacts` (102): `✏️s/🔌️plugins/<plugin>/🏭️bridge/📦️packages/🦀️rust` → its own `🗿️artifacts/*` crates. Largest: `📕️norm` (30), `🌀️procedural` (12+2), `🔋️energy` (5), `🏗️fem` (5), `🀄️wfc` (5), `🧱️block` (3), `🧩️puzzle` (3). `🫀️core` of `🪐️space` also depends on the os `🗿️artifacts/🪐️space`.
- `cargo-modules-no-plugins` (19): stdio contract crates `🗄️stdio/📇️registry/🧬️contract/✏️editing`, `🪟️details`, `🩹️patch`, `🪟️window-kit`, `🧾️payload-wire` depend on each other. These are roled `s-module` and reference `🔌️plugins` paths.

Fix: a bridge crate should not depend on artifact crates directly. Move the artifact-facing code into the artifact crate, or route it through a framework registration. For the stdio contract crates, either re-role them or merge them into one crate.
Files: `✏️s/🔌️plugins/*/🏭️bridge/📦️packages/🦀️rust/Cargo.toml` and `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/**/Cargo.toml`. Owner: the plugin-crate fleet (`PLAN-PLUGIN-CRATES.md`).

### P7. Framework Rust → os product, 7 include edges and 1 s → os product include (R4 at source level)

- `🧰️framework/🔨️modules/🌱️value/📸️snapshot/👁️read/🦀️.rs` lines 6, 9, 804, 806, 808, 815 include `🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/…`
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️retained/🧬️schema/🦀️.rs:4` includes `💻️os/🔨️modules/🏪️store/🚪️io/🧬️schema/⚠️diagnostic/🦀️.rs`

Fix: the read registry is os store state, which is parent-owned (Store). Move the read types down into the framework, or invert through a registration the os implements. Remove the include.

Also flagged (not an R1–R4 edge): 16 `#[path = "📥️staging/🧲️plugins/…"]` directives in `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`. They load plugin sources from a staging folder inside the framework tree. The clean fix is to wire the plugin crates as dependencies, not to stage source files. The owner is the renderer wgpu fleet.

### P8. Plugin cross edge, 3 prod (rule `no-cross-plugin-📏️layout-to-🗄️stdio`; 4 total)

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/…/🚪️io/🪶️sqlite/📸️snapshot/🟦️.ts` → `🗄️stdio/🗿️artifacts/🧿️semio/…`
- `…/🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts` → stdio artifacts
- `…/🧬️schema/🟦️.ts` → stdio artifacts

Fix: move the shared symbols into the framework `io` module or the stdio SDK. The layout artifact depends on the framework only.
Files: `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/**`. Owner: plugin fleet.

### P9. Hub → s, 2 prod (rule `no-cross-technology-🌎️hub-to-✏️s`; 16 total)

- `🌎️hub/🧩️compositions/🗄️stdio/🧩️composition` → `✏️s/🔌️plugins/🗄️stdio/📇️registry`
- `🌎️hub/🚀️local-relay/🧭️routing/🟦️.ts` → `✏️s/🔌️plugins/🌍️gis/🗿️artifacts`

Fix: the hub uses the registry through a framework port, not by importing s code.
Files: `🌎️hub/🧩️compositions/**`, `🌎️hub/🚀️local-relay/**`.

### P10. Test-only R1 edges, 3 (rule `framework-no-implementation`, test paths only)

- `🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🧪️tests/🟦️.ts` → `🏢️semio-tech/🎡️play/…/📦️publication/🧬️schema/🔣️.json` and `…/📋️contract/🟦️.ts`
- A third edge from the same test tree.

Fix: relocate the test into the owning plugin's test tree, or into a repo integration test root that may depend on s (baseline item 2). No production change.
Files: `🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🧪️tests/**`.

### P11. Hygiene, not R1–R4 (for completeness)

- `no-cross-package-relative-*`: 9085 total, 5299 prod. This is a per-workspace-package boundary family, generated from `🧭️direction/🏗️construction/🟨️.cjs`. It is not one of R1–R4. The baseline did not list it.
- `no-core-path`: 880 total, 607 prod. Mostly `✉️base` and `🧱️base` in stdio (baseline item 14).
- `not-to-unlisted`: 845 total, 144 prod (baseline item 13).
- `no-state-outside-os`: 86 total, 55 prod. Parent-owned Store.

### Ranking summary

| Rank | Package | Prod edges | Main rule | Parent-owned? |
|---|---|---|---|---|
| 1 | P1 s → framework SDK | 1234 | plugins-framework-sdk-only | no |
| 2 | P2 renderer hosts only UI | 457 | renderer-hosts-only-ui | partly (Actor, Media) |
| 3 | P3 UI → framework barrel and packages | 207 | ui-no-framework-packages | one file (viewport cargo) |
| 4 | P4 cycles | 547 | no-circular | partly (Actor, Store, Member, Media) |
| 5 | P5 framework → repo product | 105 | framework-no-products | yes (Cargo build infra) |
| 6 | P6 Cargo plugin-internal | 121 (static) | cargo-plugins-no-artifacts, cargo-modules-no-plugins | no |
| 7 | P7 Rust framework → os product | 8 (7 + 1) | R4 at source level | yes (Store) for the 7 |
| 8 | P8 layout → stdio | 3 | no-cross-plugin | no |
| 9 | P9 hub → s | 2 | hub to s | no |
| 10 | P10 test-only R1 | 3 (test) | framework-no-implementation | no |

Disjoint ownership hints for executors: P1 owns `✏️s/🔌️plugins/*/🗿️artifacts/**`. P2 owns `os/🔨️modules/📺️renderer/**`. P3 owns `framework/🔨️modules/🖱️ui/**`. P6 owns the `🏭️bridge` and stdio contract `Cargo.toml` files. P8 owns `layout/🗿️artifacts/📏️layout/**`. P9 owns `🌎️hub/🧩️compositions/**` and `🌎️hub/🚀️local-relay/**`. P10 owns the `🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🧪️tests/**` tree. Packages P4, P5 and P7 cross parent-owned areas and need coordination first.

## 4. Cargo-level edges

Static re-application of `cargoDependencyDirections` from `🔣️taxonomy.json` to 2464 non-ticket manifests. `cargo metadata` was not run, and nothing was compiled.

a. Framework path dependency into `✏️s` or `🌎️hub`: **0** (rule `cargo-framework-no-implementation`). Same as the baseline.

b. Framework `🔨️modules` or `📦️packages` path dependency into `🛍️products`: **0** (rule `cargo-framework-no-products`). The 4256 figure from the raw scan is a scope bug and is not a violation.

c. `✏️s/🔨️modules` into `🔌️plugins` or `🧩️extensions`: **19** (`cargo-modules-no-plugins`). All are stdio contract crates (see P6).

d. `✏️s` plugin into its own `🗿️artifacts`: **102** (`cargo-plugins-no-artifacts`). See P6.

e. `🛂️manifest` / role problems: 281 metadata problems. 233 are `missing-role`, mostly plugin crates with no `[package.metadata.semio] role`. 41 are `owner-role-mismatch`, mostly `artifact` role on stdio `🏗️ifc`, `📕️xlsx`, `📜️docx`, `📽️pptx` subset crates. The gate's own inventory may exclude many of these, so treat the count as an upper bound.

## 5. Open items

- The repo-lib layering lint (`lint-dependency-direction`, `lint-rust-source-direction`, `lint-cargo-dependency-direction`) has not run. Run 3 was refused by the argument validator, and run 4 hit the 900 s cap while still building. Re-run once the build pipeline is less loaded, using `bun repo run-many -p @semio-tech/repo-lib -t lint-dependency-direction -t lint-rust-source-direction -t lint-cargo-dependency-direction --skip-cache` with a longer cap. The validator may also need a fix so that `verify layering` can run at all.
- The `.cache/b/d/m` target in P2 (125 edges) needs a check before anyone fixes it.
- The literal-external drop from 2543 to 300 needs a reconciliation against the old raw output. Those files are no longer in `🗑️generated`.
