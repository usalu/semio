# 🧭️ Result PLUGIN-WP0: skeletons, mount hubs and hand-off contract of the `semio-framework-plugin` partition

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Plan: `PLAN-PLUGIN-CRATES.md` (updated in place; contract = sections 12.4 and 12.5, Appendices H to K). Tools: `pwp0-tools/`. Logs: `🗑️generated/pwp0-*`. `PLUG` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`, `PKG` = `PLUG/📦️packages/🦀️rust`.

## 0. Verdict

Phase 0 is done. The census was re-run and the plan reconciled, the crate root is split into seven `include!`d mount hubs (84 of 92 mounts; eight stay in the root), 80 skeleton crates exist with manifests, owner rows, workspace membership and six launch routes in both launch files, three facade macro mounts and 26 lift files prepare the cross-package moves, and the facade manifest carries the transitional dependency block. The tree is still **one** compilation unit and resolution-equivalent: the patched full-resolution differential differs from the baseline by the same three `E0432` diagnostics in all four configurations (section 5, residue explained and bounded), mount equivalence is exact, and the static gate passes. **Nothing was compiled by cargo**: the launcher was down at both ends (section 6). The first run of the differential found six real regressions of my own first cut; they were fixed (section 4).

## 1. Decisions applied (coordinator D1 to D8, recorded in plan 12.4)

D1 yes (7.2 to 7.7 binding). D2 yes: Phase 2 of I1 is a final **WP8**. D3: the dead `ui_wgpu` dependency was **already gone** (a peer's relocation replaced it by `ui-layout` / `ui-declaration` before WP0; verified by the manifest and a comment- and string-stripped scan: 19 files mention it in comments only). D4 yes. D5 keep separate. D6 yes, assigned to WP7 (it removes an item and would have broken the equivalence evidence). D7 yes: three empty facade macro mount modules exist (`PLUG/📣️macros/{🌱️wp1-window, 📜️wp2-declaration, ⚛️wp4-reactor}`, mounted `#[macro_use]` from the root before every hub), the owning package moves its `#[macro_export]` definitions in. D8 Option B: skeleton `reactor-tasks` (WP4, `⚛️reactor/⏲️tasks`); `reactor-core`, `reactor-turn` and the instance family depend on it; WP4 asks WP0 to fall back to Option A if its definition scan refuses.

## 2. Census

Gate counter `readRustSourceClosureV1` (`pwp0-tools/closure.ts`), files / bytes / active lines. Plan = planner (15:20), start = before the first WP0 edit, end = after hubs and macro mounts.

| closure | plan | start | end |
|---|---|---|---|
| lib native | 185 / 6,007,520 / 83,182 | 185 / 6,010,310 / 83,203 | 195 / 6,018,889 / 83,307 |
| lib wasm32-wasip2 | 189 / 6,075,698 / 84,493 | 189 / 6,078,488 / 84,514 | 199 / 6,087,067 / 84,618 |
| lib wasm32-unknown-unknown | 185 / 6,007,520 / 83,256 | 185 / 6,010,310 / 83,277 | 195 / 6,018,889 / 83,381 |
| lib + 4 testing features | 211 / 6,512,773 / 88,660 | 211 / 6,515,563 / 88,681 | 221 / 6,524,142 / 88,785 |
| test native | 729 / 12,793,280 / 145,762 | 729 / 12,796,350 / 145,783 | 739 / 12,804,099 / 145,885 |
| test wasm32-wasip2 | 733 / 12,861,458 / 147,043 | 733 / 12,864,528 / 147,064 | 743 / 12,872,277 / 147,166 |

Drift against the plan (`appendix_a_drift.py`): +21 lines / +2,790 bytes, same file counts; the root went from 940 to 960 lines (peer edit: `InstanceExtensions` alias, `ui-layout` / `ui-declaration` / `ui-scene` re-exports replacing `ui_wgpu::wgpu::*`), world3d +1 line, `🏗️builder/🧩️app` +151 bytes. Appendix A had omitted three closure files (`🧪️tests/🧬️generated-test-contracts`, `🧵️retained-command/♻️metadata`, `🪟️window/🎚️config/🧬️preparation/✏️apply`); rows added. The item graph is unchanged: `plan_tools.py census` reproduces every crate size of 6.1 (196 moves, 3,351 lines vs 3,352) and `home` the same 227 fields / 674 methods / 14,971 lines.

Skeleton crates (`ONLY=… bun census.ts`, `🗑️generated/pwp0-census-after.log`): all 80 are 1 file and at most 528 bytes, no error. Only `identity`, `test-support` and `tests-01` have a closure inside the gate; everything else sits behind the oversized frontier. Facade row at the census time: 192 files / 6,016,589 bytes / 83,265 lines (now 195 / 6,018,889 / 83,307 with the macro mounts).

## 3. Crate graph

80 crates (Appendix H): 42 lower (41 of 6.1 + `reactor-tasks`), 16 instance family / builder / testing, 21 companion bins, `test-support`. Rank order = 4.3 with `reactor-tasks` after `reactor-app`, the family of 7.5 above the lower crates, the bins last; `verify_static.ts` finds no cycle and no upward edge (1,569 path dependencies resolved, each naming the right package). Class A (free root, final form) 37 crates, class B (occupied root, package-local stub until the carve flips `[lib] path`) 43. Instance-family lower edges are token evidence of the method homing, with the I6 call sites (towards `reactor-lifecycle`, `runtime-ops`, `reactor-jobs`, `reactor-core`, `extension`) replaced by `reactor-app` and `reactor-tasks`. WP split: WP1 11 crates, WP2 9, WP3 12, WP4 14 (incl. `reactor-tasks`), WP5 5, WP6 6, WP7 22, WP0 `test-support`.

## 4. What changed

1. **Mount hubs** (`PLUG/🪝️mounts/<hub>/🦀️.rs`, 7 files): `🌱️wp1-foundation` 28 units, `📜️wp2-declaration` 21, `👥️wp3-owners` 21, `⚛️wp4-reactor` 4, `⏯️wp5-features` 0, `🧱️wp6-instance-tier1` 1, `🧵️wp7-instance-tier2` 9; path literals rebased by `../../`, facet regions repeated and balanced, seven `include!` lines in the root. Eight mounts stay in the root (Appendix I, `root`): `generated_test_contracts`, `wasip2`, `window_transient`, `app_emission`, `tool_contract`, `plugin_runtime` (their files define `#[macro_export]` macros) and the two macro-scope tests. **Defect of my first cut, found by the differential**: `window_transient` and `tool_contract` were in hubs and the test configuration reported six new `macro_expanded_macro_exports_accessed_by_absolute_paths` errors (`crate::transient_root!`, `crate::window_transient_owners!`, `$crate::window_transient_transfer!`, `crate::bounded_first_step_tool_proofs!` x4). I returned every macro-exporting unit to the root (rule now generated in `hubsplit.py`) and re-ran all four configurations.
2. **80 skeleton crates**: `Cargo.toml` (recipe triple at the ceiling, `workspace` = repository root, features of 6.4, first-party edges of Appendix H, external crates by token evidence with the old specs, dev-dependencies by hosted tests), `📋️project.json`, stub root (+ test stub where the directory has no tests folder): 248 files. Longest manifest path 185 bytes.
3. **26 lift files** (`🛬️lifted/<source module>/🦀️.rs`, docstring-only, Appendix J: 3,342 lines cross-package, plus 28 intra-package groups of 1,661 lines) and **3 macro mount files**.
4. **Registries**: root `Cargo.toml` members 340 to 420; `📋️build-owners.json` +80 rows (`owners-tools/validate.ts`: registry valid; `scan.ts`: `missing: []`; `closure.ts`: only the known `""` false positive); 480 routes in each of `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (order keys build 900.3, check 922.062, test 903.6011 in unused windows, identical in both files, `env` object included).
5. **Facade manifest**: 58 transitional dependencies (`testing-laws` / `testing-acceptance` optional, `reactor-wit-bridge` in the wasip2 table), the four features and `canvas-runtime-assets` forward to the carrying crates, boundary triple 262144 / 32 / 4000. The `[[test]]` targets and all old dependencies stay.
6. **Plan**: intro note, 1.1 re-measure, 1.3, 6.1 note, 8 note, 12.2 deviations, new 12.4 (decisions) and 12.5 (contract, 11 subsections), 15.2 resolution, Appendix A rows, new Appendices H, I, J, K.
7. **Profiles**: no `wasm-prototype` rows. The repository has such rows (146 manifests, `iterateHotPackages` fixture) only for the neural dag, replication and pack crates; neither the plugin crate nor a neighbour has a `[profile.*.package.semio-framework-plugin]` row anywhere, so the split loses no optimisation level. Reasoning and the escalation path are in 12.5.7.
8. **Taxonomy**: no row written. The children of `🔌️plugin` have no member kind; every existing plugin directory is `directory-kind-unresolved` (checked on `🧵️instance`: 8 errors), and the new `🧷️companions` (23) and `🪝️mounts` (8) report only that same code. All new folder emoji pass the RGI / uniqueness check of `verify_static.ts` (the `📥️` of the plan collides with the existing `📥️poll` and `📥️receipt`, so hubs and lifts use `🪝️mounts` and `🛬️lifted`).
9. **Not done by WP0** (and why, plan 12.2): cutting the 11 exported macros, `owned_abi` and `component_persistent_local` out of their files (textual scope; WP1/WP2/WP4 fill the prepared macro modules, WP1 cuts the root regions); step S0 file cuts (owner-disjoint, done by WP4/WP5/WP6 as their first step); `Plugin::builder` removal (WP7); `Cargo.lock`; the 25 source embeds and 532 path literals.

## 5. Verification (commands run, real results)

| check | result |
|---|---|
| `verify_static.ts` (80 manifests via Bun TOML: name, id, role, boundary keys, workspace, lints, lib path by class, frozen first-party dependency set, DAG and ranks, features, docstring emoji, path length <= 240 B, project files; members unique and byte-sorted; owner rows sorted; 6 routes x 80 x 2 files with exact command, group and key order, order keys unique, unused elsewhere and identical in both files; folder emoji RGI and unique among siblings; facade dependency / optional / feature-forward / triple / no `ui_wgpu`) | `manifests 80, path dependencies resolved 1569, crates 80, failures 0`, `ALL STATIC CHECKS PASSED` (re-run after the final tree) |
| `verify_hub.py` | 92 original mounts = 92 in root and hubs (same module, absolute target file, cfg attributes); the only additions are the 3 facade macro mounts; all targets exist; regions balanced; rustfmt parses all 8 files; `PASSED 0` |
| rustfmt (nightly, `skip_children`) on the 114 new `.rs` stubs and lift files | 0 parse failures, 0 diffs after one double-blank-line fix |
| TS module tree (`tree_compare2.ts`): live vs original root | file sets equal plus the 7 hub files; no line lost except the 89 rebased `#[path]` literals |
| `policy_roots.ts` (registry guard "every split source root reads as one text") | `roots 27 bad 0` (the plugin root is not in the list) |
| 10 TS readers of the plugin root text (script and tests): searched string literals | 0 of 15 literals found in the original root are lost; the readers search text that left the root long ago, so they are unaffected |
| `owners-tools` | `validate.ts`: `registry valid, owners: 2384`; `scan.ts`: `missing: []` |
| **Differential** (`oracle.py hub2`; sandbox patches of `📓️RESOLUTION-ORACLE.md`; baseline = same tree with hubs inlined) | lib 9,522 vs 9,523; `--test` 22,057 vs 22,058; `wasm32-wasip2` 9,813 vs 9,814; lib + 4 features 10,370 vs 10,371. In **every** configuration `onlyNew=1`, `onlyOld=2`: three `E0432` diagnostics, nothing else. Self-check (no change): 9,522 vs 9,522, 0 diffs |
| Attribution of the residue | `crate::ArtifactOwnedDisposer` x2 (`presence_retirement`, `transient_retirement`, hub 2) is reported at baseline only; `super::window_config::WindowConfigApply{Cursor,Edit,Mutation}` (one grouped message) by the hub tree only (hub 1). Inlining hub 2 alone returns the first pair (`inl2`: onlyOld 0, onlyNew 1); inlining hub 1 alone removes the second (`inl1`: onlyNew 0, onlyOld 2). Both names exist only behind the neutralised external globs or the empty `store` stub, so they fail in both trees; swapping two adjacent baseline mounts changes nothing (`noise1`: 9,523 vs 9,523, 0 diffs). The shift is therefore how rustc attributes a failed import chain when the module is declared in an included file. It is bounded (3 diagnostics in 10,000 to 22,000) and every other class is identical |
| Seeded sensitivity (`seed1`: broken `#[path]`; `seed2`: removed glob re-export and renamed module) | caught: `couldn't find file …` as only-new; 51 new and 8,137 old diagnostics |
| Taxonomy verifier on the new folders | only `directory-kind-unresolved`, the code every plugin folder already has |

## 6. Real compile

`bun repo cargo --store=shared --configuration=prototype -- check -p … --message-format=short --lib`, one cargo at a time. Start (`-p semio-framework-plugin-mounted-owner`, before any edit): `cargo fetch --locked --manifest-path …/Cargo.toml` exits 101, `cannot update the lock file … because --locked was passed`. End (`-p semio-framework-plugin-identity`): `error: Finite original maintenance authority required`. Neither run reached a compiler; `Cargo.lock` is untouched, so the 80 new packages and the facade's 58 new edges are missing from it until the first `update --workspace --offline` of the compile window.

## 7. Files

Created: 80 x (`Cargo.toml`, `📋️project.json`, stub root) + 8 test stubs = 248 under `PLUG/**/📦️packages/🦀️rust` and the class-A roots; 26 lift files; 7 hub files; 3 macro mount files; `pwp0-tools/*` and `🗑️generated/pwp0-*` of this ticket; this file.
Edited: `PLUG/🦀️.rs` (hubs, macro mounts), `PKG/Cargo.toml`, root `Cargo.toml` (+80 members), `📋️build-owners.json` (+80 rows), `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (+480 routes each), `PLAN-PLUGIN-CRATES.md`.
Not touched: `Cargo.lock`, `🔣️taxonomy.json`, `🧑‍💻dev/**` fixtures, any peer-owned crate, any source file of a package (except the root).

## 8. Residual risks and hand-offs

1. **No cargo evidence and no `Cargo.lock`** (section 6); every claim about edges is token-level or name-resolution-level until the first real `check`.
2. **Differential residue** (3 `E0432` diagnostics, section 5). If the first real compile shows a name resolved differently through the hubs, the cure is to move the unit named there back into the root (as done for the macro-exporting units); the hubs are mechanically reversible (`oracle.py` baseline builder, `hubsplit.py`).
3. **External and first-party edge lists over-approximate** (names in docs count; instance-family edges come from homing, not from compiling); each package prunes and adds. Four statics-dependent edges (D8) wait for WP4's definition scan.
4. **Cross-package edit points** are explicit but numerous: 26 lift files, 15 foreign mount lines (Appendix K, among them the three `pub mod tool_run / tool_machine / time_travel` mounts WP5 must remove from WP3's `👷️job/🦀️.rs`), and the six root-resident units. They rely on re-reading before each edit (12.3).
5. **WP6 before WP7 on the instance family** (step S0, 12.5.9); WP7 starts with bundles, `builder` and the testing crates meanwhile.
6. **Test relocation is not frozen beyond the bins** (11.3): the 25 source embeds and 532 path literals are untouched, `test-support` is a skeleton (WP1 implements the spec in 12.5.10), `testhosts.py` only suggests hosts.
7. **Concurrency**: peers edit plugin files (`⚛️reactor`, `🧪️testing`, instance bands) and the launch files; every registry write re-read its file and asserted unique anchors, but a peer's normalizer or auto-commit may touch the same blocks; `verify_static.ts` (about 30 s) re-checks all of it.
8. The coordinator's load limits were respected after the notice: no recursive grep or find over the repo (git grep with pathspecs only), the stopped background search, one rustc differential or one cargo at a time. Large `pwp0-oracle-*.jsonl` outputs and the scratch sandboxes were deleted; the summaries and logs stay.
