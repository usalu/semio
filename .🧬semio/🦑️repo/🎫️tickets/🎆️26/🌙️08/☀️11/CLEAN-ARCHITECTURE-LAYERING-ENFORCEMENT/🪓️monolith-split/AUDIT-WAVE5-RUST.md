# 🔎 Audit WAVE5-RUST: read-only verification of the wave-5 Rust results

Auditor: Haiku 5.5, foreground, no sub-agents, read-only. Run 2026-10-16 about 11:20 to 11:40 CEST.
Scope: the seven RESULT files named in the brief (OS-KERNEL-ENGINE, OS-KERNEL-SQLITE-HOST, PLUGIN-HOST-CARVE, RENDERER-INTEGRATION-2, CARGO-PLUGIN-DIRECTION, RUST-CROSS-COUPLINGS, COMPILE-PASS-2), checked against the disk, git history and the census.
No source file was edited. No compile was run. The only git commands were read-only (`status`, `log`, `show`, `diff`, `ls-files`, `grep`, all with `--no-optional-locks`).

Raw outputs: `🗑️generated/audit5r-*` (plus `🗑️generated/audit5-census.log`, which the brief names). Tools: `audit5-tools/` (`census_diff.py`, `changed_set.py`, `added_scan.py`, `dupmounts5.py`, `modowners.ts`, `ws_membership.ts`, `edges5.ts`).
The census re-run rewrote `🗑️generated/census-summary.json` and `crate-census.csv` (expected).

## 0. Verdict

No wave-5 Rust crate has a compiler verdict. Every RESULT file says so, and this audit confirms it: nothing was compiled, so the findings below are static.

The census is clean on the three things the wave set out to fix: the kernel, store-sync and plugin-host crates are now under the limits, and the store engine matches its RESULT figures exactly. Four things are wrong, and the result files do not report them:

1. The plugin contract crate (`guest-contract`) compiles plugin-owned source, so two copies of the same types exist (F1). This predates wave 5 (commit 2026-10-14), but it sits on the plugin boundary.
2. The renderer top crate and the plugin-host test component are over the 32-file limit on their own, but the result files attribute the overrun to dependencies (F3).
3. 117 new `Cargo.toml` manifests are in no workspace and have no `project.json` (F4). All were added by auto-commit `1bae6115bc5` (2026-10-16 04:57).
4. A dev-dependency points upward in rank (F5), and `[DEBUG] ` logs remain in repo source (F6).

## 1. Ranked findings

Severity: High = wrong type identity, a gate result that is false, or a broken brief rule. Medium = rule violation or a latent risk, no present break. Low = hygiene.

| ID | Sev | Finding | Primary evidence |
|---|---|---|---|
| F1 | High | `semio-framework-plugin-guest-contract` compiles `plugin/⚛️reactor/🎟️context`, `plugin/🚪️io/🛂️authority` and `plugin/🧬️schema/🪶️sqlite` as its own `pub mod`s. The plugin lib tree reaches the same files. The plugin-host facade re-exports the guest-contract copy. Two copies of the same types. | `🔌️plugin/🧬️guest-contract/🦀️.rs` lines 10 to 15; `🔌️plugin/⚛️reactor/🦀️.rs` line 17; `🔌️plugin/🦀️.rs` line 87 (`include!` of `🪝️mounts/⚛️wp4-reactor`); host facade `🔌️plugin/🖥️host/📦️packages/🦀️rust/🦀️.rs` (`pub use semio_framework_plugin_guest_contract::original_actor_context`). Introduced by `7c8af563c74` (2026-10-14), before wave 5. |
| F2 | High | `🌉️abi` (`semio-framework-abi`) is compiled a second time inside the webgpu renderer and the UI host: `#[path]` mounts of the abi root as a module. Neither UI crate depends on the abi package. Type split. | `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust/🦀️.rs` lines 25 to 26 (`#[path = "../../../../../../🌉️abi/🦀️.rs"] pub mod abi;`); `🖱️ui/🖥️host/🦀️.rs:16`. Introduced `fe7c8a8f8bd` (2026-09-05). |
| F3 | High | Gate reporting is wrong for two crates. `renderer-wgpu` binds on its own 37 files (limit 32), not on its dependency. The test component binds on its own 41 files (20 of them embedded). RESULT-RENDERER-INTEGRATION-2 says "still OVER-GATE through its oversized dependencies"; RESULT-PLUGIN-HOST-CARVE says the test component is refused "only through" `plugin`. | `crate-census.csv`: renderer-wgpu `binding=files`, `files=37`, `oversizedDeps=1`; test-component `binding=files`, `files=41`, `embeddedFiles=20`. |
| F4 | High | 117 new `Cargo.toml` (all `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/…`, added by `1bae6115bc5`) belong to zero workspaces and have no `📋️project.json`. Brief rule 6 (exactly one workspace, owner row) fails for these. | `🗑️generated/audit5r-zero-ws.txt` (117 paths); the stdio workspace `✏️s/🔌️plugins/🗄️stdio/Cargo.toml` lists sibling gif crates explicitly but not these, and its `exclude-patterns` includes `🗿️artifacts/**`. |
| F5 | Medium | Upward rank edge: `os-store-space-schema` (rank 8) has a dev-dependency on `os-store-canonical-json` (rank 15). RESULT-OS-KERNEL-ENGINE's "0 back edges" holds for normal dependencies only. | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/📦️packages/🦀️rust/Cargo.toml` line 49 under `[dev-dependencies]`; rank from `okwp0-tools/okcrates.py` `RANK`. |
| F6 | Medium | `[DEBUG] ` leftovers. Among lines added in the last day (3 commits, uncommitted diff, untracked files): 107 unique lines in repo source across 49 files, 859 in test code, 280 in ticket-folder scripts. Production-reachable Rust: a `#[cfg(debug_assertions)]` `eprintln!` in the store lifted copy, time-travel `instance` `eprintln!`s (lines 197 and 745), and three native `log_debug("[DEBUG] …")` calls in `winit-app` (lines 886, 913, 920). Env-gated `console.log("[DEBUG] …")` in build tooling. EXECUTOR-RULES require removal before finishing. | `🗑️generated/audit5r-debug-repo-source.tsv`, `audit5r-added-hits.tsv`. |
| F7 | Medium | Plugin artifacts share source files across sibling crates: `wfc/🪶️sqlite/🎨️bitmap` (2d and grid2d artifacts), `🫀️core/🧬️generation/🪶️sqlite/🚦️native` (generation2d and generation3d, plus the text io of both), `puzzle/🎮️commands/📤️jobs` (2d and 5d), `space` collection and space artifacts sharing `🚪️io/🪶️sqlite/📸️snapshot`, and others. Same source compiled into two crates. | `🗑️generated/audit5r-modowners-multi.tsv` (116 files reached from more than one package; 39 with at least one non-test mount site). |
| F8 | Medium | Compatibility aliases added by the wave. The brief asks for `pub use old as new` patterns: 183 unique added lines (76 files, outside tool folders), e.g. the kernel store root `pub use semio_framework_os_store_test_support as test_support;`, `pub use semio_framework_os_pack as os_pack;`, `pub use semio_framework_os_store_engine_owners as document_store_owners;`, `pub use semio_framework_os_semio as semio_format;`; the stdio contract root `pub use semio_framework_os_kernel as kernel;`; `pub(crate) use semio_framework_trace_memory_testing as test_allocation;` in about 15 pack files. These keep old module names alive, which AGENTS forbids. No `#[deprecated]` appears in added lines. | `🗑️generated/audit5r-added-hits.tsv` (kind `shim`, `pub-use-as`). |
| F9 | Medium | Kernel test build mounts the whole store-testing tree as a module of the store root: `#[cfg(test)] #[path = "🧪️testing/🦀️.rs"] mod fixture_mutations;` (store root lines 1228 to 1230). 34 files are shared with the `semio-framework-os-store-testing` package. Test-only, but type identities split inside kernel tests. | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` lines 1228 to 1230; `audit5r-modowners-multi.tsv`. |
| F10 | Medium | Path length (brief rule 5 and AGENTS). By UTF-8 bytes, 11 repo-relative paths in the changed set exceed 240 (max 258, all under `✏️s/🔌️plugins/` in the mathematical, fem, architect, remodel and draw artifacts). Absolute paths: 381 exceed 240 bytes and 64 exceed 256 bytes. By code points, none exceeds 256 (max relative 174, max absolute 202). Emoji folder names take 4 bytes each, so the byte limit bites first. AGENTS speaks of 256 characters, so the code-point figure is the one it means. | `audit5-tools/changed_set.py` output, `🗑️generated/audit5r-changed-set.txt`. |
| F11 | Low | `git status` is not the wave-5 changed set. Auto-commits absorbed most of it: `36fbe9da786` (11:13, 3,089 paths), `1bae6115bc5` (04:57, 9,106), `f856dff0813` (10-15 14:15, 3,050). `git status` shows 123 entries. Results that cite status as the change list under-report. | `🗑️generated/audit5r-commits-24h.txt`, `audit5r-status-nofsm.txt`. |
| F12 | Low | Plugin-host DAG is incomplete: 14 normal or dev edges (`imports` to `engine`, `operation`, `wit`; `guest-laws` and `shard-laws` dev edges) are not ordered by the DAG stated in RESULT-PLUGIN-HOST-CARVE section 2, so they cannot be checked against it. No violation found. | `🗑️generated/audit5r-edges.tsv`, verdict `unordered-by-DAG`. |
| F13 | Low | Stray leftover file `🏪️store/🦀️.rs.orig` (868,716 bytes, 03:48). It sits beside the store root and is not in any result list. | `ls -l` of that path. |
| F14 | Low | Facade numbers differ by metric. RESULT-PLUGIN-HOST-CARVE gives the facade as 11 files / 1,451 lines (a maximum over five configurations). The census (lib closure, native) gives 4 files / 436 lines. Not a defect, but not reproducible from the census. | `crate-census.csv` row `semio-framework-plugin-host`. |

## 2. Census (brief item 1)

Re-run: `cd 🗑️generated && bun census.ts > audit5-census.log`, exit 0.

| Metric | Baseline 04:50 (`coord-census-summary.prev.json`) | Now 11:23 |
|---|---:|---:|
| crates (nodes) | 712 | 727 |
| workspace members | 471 | 486 |
| compilable (`closureCompilable`) | 413 | 627 |
| oversized (`overLimit`) | 6 | 3 |
| carvable leaves | 2 | 1 |

Oversized now (limits 32 files, 262,144 bytes, 4,000 active lines):

| Crate | Files | Bytes | Lines | Binding | Own over | Notes |
|---|---:|---:|---:|---|---|---|
| `semio-framework-plugin` | 185 | 3,300,847 | 44,574 | bytes | yes | carvable leaf, still 5.8 times the file limit (baseline 206 files) |
| `semio-framework-os-renderer-wgpu` | 37 | 146,504 | 1,846 | files | yes | F3 |
| `semio-framework-plugin-host-test-component` | 41 | 52,210 | 626 | files | yes | 20 embedded files; F3 |

Cleared since baseline: `semio-framework-os-kernel` (5 files, 116,845 bytes, 1,624 lines), `semio-framework-os-store-sync` (2 files, 114,905 bytes, 2,000 lines), `semio-framework-plugin-host` (4 files, 23,631 bytes, 436 lines). All 14 plugin-host producer crates are under the limits.

## 3. Facade claims (brief item 2)

| Claim | Measured | Result |
|---|---|---|
| os-kernel: 5 files, about 1,624 lines | 5 files, 116,845 bytes, 1,624 active lines | Confirmed |
| renderer-wgpu top: 37 files, 1,847 lines | 37 files, 146,504 bytes, 1,846 active lines | Numbers confirmed; "over-gate only through dependencies" is wrong (F3) |
| plugin-host facade | census 4 files, 436 lines (lib closure); result claims 11 files, 1,451 lines (max over configs) | Not reproduced (F14) |
| store root `🏪️store/🦀️.rs` about 1,345 lines | 1,336 newline lines, 94,228 bytes | Within about 1 percent |
| store engine 4 files, 254,589 bytes, 3,452 lines | identical | Confirmed |
| sync 2 files, 114,905 bytes, 2,000 lines | identical | Confirmed |
| sync-wasm 2 files, 55,889 bytes (wasm) | census (native cfg) 1 file, 8 lines | Different configuration, not a contradiction (the crate is wasm-gated) |

## 4. Duplicate `#[path]` mounts (brief item 3)

Method: `git grep -F '#[path'` over tracked files (39,239 lines; 37,618 in source trees after excluding ticket folders), plus the one untracked `.rs` file. A first pass resolved literals naively and gave false positives for inline modules, so it was discarded. The final method (`audit5-tools/modowners.ts`) walks every tracked and untracked package from its lib, bin and test roots, following explicit `#[path]` and `include!` (cfg is recorded, not applied). Result: 2,479 manifests, 2,809 roots, 116 files reached from more than one package.

- 77 of 116 are reached only through test sites (cfg(test) or test folders).
- 39 have at least one production-looking mount site. Those are F1, F2, F7, and the rest of the plugin artifacts.
- Kernel and store-testing (34 files) is test-only (F9).
- `mcp/foundation` mounts a trace testing module under `cfg(any(test, feature = "testkit"))`. That gate is not `cfg(test)`, so it is production-reachable with the `testkit` feature. Not a finding on its own.

## 5. Leftovers and shims (brief item 4)

- `[DEBUG] ` in added lines (last day, all three commits, uncommitted diff, untracked): 1,246 unique lines. Detail in F6 and `🗑️generated/audit5r-debug-repo-source.tsv`.
- Compatibility shims: no `#[deprecated]` or `#[allow(deprecated)]` in added lines. "legacy" and "compat" hits are comments, tests or laws that police shims, not shims themselves. The real shims are the `pub use X as Y` aliases in F8.

## 6. Path length (brief item 5)

See F10. Changed set: 13,305 existing files (`🗑️generated/audit5r-changed-set.txt`). Missing on disk: 288 (deleted paths, not measured).

## 7. Cargo workspaces and owner rows (brief item 6)

710 `Cargo.toml` files were added in the last-day commits or are untracked (`🗑️generated/audit5r-new-cargo.txt`).

- 593 are members of exactly one workspace and have their own `📋️project.json` owner row.
- 117 are members of none (F4). None has a `project.json`.
- No manifest is a member of more than one workspace.

Membership uses `[workspace].members` with the glob matcher and `exclude`. Owner rows are checked against `📋️build-owners.json` by project manifest path, then by package name for the ancestor rows. The per-manifest table is `🗑️generated/audit5r-new-cargo-membership.tsv`.

## 8. Dependency direction (brief item 7)

Rank sources: kernel `okwp0-tools/okcrates.py` `RANK`; renderer `🗑️generated/rwp0-crates.json` `rank`; plugin-host the DAG in RESULT-PLUGIN-HOST-CARVE section 2 (the plan has no table for it).

- Kernel: 283 edges checked, 282 ok, 1 violation (F5, dev).
- Renderer: 1,087 edges, all ok.
- Plugin-host: 25 ok, 14 not ordered by the stated DAG (F12), 16 facade edges (facade is the top; ok).
- Total among new crates: 1,409 edges, 1 violation.

Sample of 20 (deterministic, all edges in `🗑️generated/audit5r-edge-sample.tsv`):

| Family | Edge | Verdict |
|---|---|---|
| kernel | os-dsl to os-semio | ok |
| kernel | os-store-config to os-store-engine | ok |
| kernel | os-store-durable-group-wire to os-store-one-item-publication | ok |
| kernel | os-store-engine-live to os-store-ledger | ok |
| kernel | os-store-initialization-runtime to os-store-read | ok |
| kernel | os-store-owned-schema-fields to os-store-ledger | ok |
| kernel | os-store-space-schema to os-spr | ok |
| kernel | os-store-space-schema to os-store-canonical-json (dev) | **violation (F5)** |
| renderer | agent-elements to doors | ok |
| renderer | hub-workspace to media-slots | ok |
| renderer | shell-directory to shell-history | ok |
| renderer | shell-state to program-bridge | ok |
| renderer | shell-tests-chrome-5 to host-base | ok |
| renderer | shell-tests-input-1 to shell-view-chrome | ok |
| plugin-host | guest-runtime to wit | ok |
| plugin-host | post-turn-relay to guest-runtime | ok |
| plugin-host | process-transport to engine | ok |
| plugin-host | guest-laws to engine (dev) | unordered by DAG (F12) |
| plugin-host | imports to operation (dev) | unordered by DAG (F12) |

## 9. Claims checked against the disk

| Result | Claim | Status |
|---|---|---|
| RESULT-OS-KERNEL-ENGINE | 18 crates carved, kernel 5 files / 1,624 lines, store root 1,345 lines, 0 back edges in RANK | Confirmed except the dev edge (F5); no compile |
| RESULT-OS-KERNEL-SQLITE-HOST | 4 sqlite-host crates and 7 sync crates, edges down | Census sizes confirmed; sync-wasm figure is wasm-cfg; no compile |
| RESULT-PLUGIN-HOST-CARVE | 14 producer crates, all under the limits; test component refused only through `plugin` | Producers confirmed; test component own-over (F3); no compile |
| RESULT-RENDERER-INTEGRATION-2 | 37 files / 1,847 lines, over-gate only through dependencies | Numbers confirmed; attribution wrong (F3); no compile |
| RESULT-CARGO-PLUGIN-DIRECTION | 23 to 3 strict violations, space core fixed | Not re-run (out of brief scope); no compile |
| RESULT-RUST-CROSS-COUPLINGS | 12 to 7 cross-technology includes | Not re-run; no compile |
| RESULT-COMPILE-PASS-2 | fixes not verified by compiler (headroom gate) | Consistent with the static state; no compile run here |

Every wave-5 result says the headroom gate refused the compile. This audit did not try to compile. The census is a syntactic closure count, not a rustc result.

## 10. Limits

- Static only. Nothing was compiled, and cfg was not applied to the mount graph (F9 and F7 are judged from mount sites and folder names).
- The changed set uses git status plus the three last-day commits (F11). Work committed before 2026-10-15 11:20 is out of scope.
- The plugin-host DAG is the one stated in its RESULT file; the plans have no rank table for it.
- Rank table for plugin WP3 to WP5 and stdio contract crates: not in any plan, not checked.
- Owner rows are matched by project manifest path and package name only.
