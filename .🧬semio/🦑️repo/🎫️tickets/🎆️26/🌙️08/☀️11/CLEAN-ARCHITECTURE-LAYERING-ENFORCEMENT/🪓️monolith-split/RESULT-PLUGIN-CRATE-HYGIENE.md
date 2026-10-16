# Result: Plugin Crate Hygiene (slice PLUGIN-CRATE-HYGIENE, prefix `pch`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. Run 2026-10-16 11:35 to about 15:00 CEST under machine load 15 to 60.
Folder F = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/pch-tools/` (all read-only except the `g-*.ts` appliers, `rewire.ts`, `publish-write.ts`). Logs: `F/🗑️generated/pch-*`. Library root `L = 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## 0. Verdict

| Defect | Verdict |
|---|---|
| F4 (117 orphan manifests) | Not orphans in the cut-run sense and not half-carved: every one of the 117 belongs to a finished generator carve whose `--check-applied` passed at apply time. The real defect was a wiring disagreement the generator never checked: Cargo lists them as workspace members, but the repository's own member discovery (`cargoWorkspaceMembers`, the source of `bun repo cargo -p …`) did not see them, because every artifact workspace kept the blanket exclusion `**/🏅️standards/**` in `exclude-patterns`. Fixed in the generator (so it cannot recur), and repaired in 45 workspaces (the 22 of the 117, plus 23 older artifact workspaces with the same defect, 393 more hidden crates). Result: 0 hidden plugin crates; all 117 are in the launcher inventory. |
| F7 (116 shared files) | 116 -> 22 files reached by more than one package at the last scan (14:36): 15 framework-owned (other slices) and 7 in the generation plugin (the 3-file generation-native template and 4 generation3d test supports, §3.3). Plugin-owned non-test sharing: 16 -> 3 (the native template). In every plugin I touched except generation (wfc, puzzle, imperative, stdio, sourcing, process catalog oracle) the scan shows 0 shared plugin-owned files. The process3d, lowpoly, cad, remodel and energy test supports that were still shared when I measured them (11:50 to 12:35) are no longer shared: a concurrent peer deduped them while I worked (new crate `semio-s-artifact-lowpoly-lowpoly-mut-laws`, consolidated lowpoly editor crates, process3d carve); I did not touch those files. |
| Real compile | Not run. The launcher refused two attempts (12:10, 12:47) with `Insufficient physical compiler initial headroom`, and the third (15:07) failed in launcher infrastructure (`Exact canonical persistent Native publication required`, `…/🏗️native-build/💾️storage/🧹️cleanup/🎛️command`, not mine). Everything below is static evidence. |

The carve generator did not produce the sharing. It mirrors mounts that already existed in the original sources (§3.1). It did produce the workspace-wiring defect, and its gate (`--check-applied`) could not see it, nor could it see any carve against an already-committed carve. Both gaps are closed (§2.2).

## 1. Found state (recovery section for F4)

### 1.1 Which run created the 117 manifests

All 117 are `A` in auto-commit `1bae6115bc5` (2026-10-16 04:57) against `f856dff0813`; `git diff --diff-filter=A f856dff0813 1bae6115bc5 -- '✏️s/🔌️plugins/**/Cargo.toml'` lists 128 additions: the 117 plus 11 stdio contract crates (another slice, members, not orphans). The commit only captured a working tree in which every generator chain had already finished:

| Artifacts (manifests) | Run | Last apply | Last `--check-applied` of that run |
|---|---|---|---|
| tsv 5, binary 4, csv 5, html 5, ply 4, stl 5 | pw0 re-carve (`pw0-rollout-*3.log`, `pw0-recarve*.sh`) | 01:56 to 02:16 | exit 0 (`pw0-applied-*.log`) |
| space 4 | pw0 pilot, re-carved (`pw0-backup-space-space-3`) | 10-15 18:01 | exit 0 |
| bcf 4, las 5, md 5, obj 5 | F1 (`f1-*-apply1.log`) | 03:16 to 04:14 | exit 0 (`f1-*-check*.log`) |
| avi 5, bmp 7, epw 6, gif 8, mp3 5, mp4 7, png 6, txt 5, wav 5 | F3 (`f3-queue2.sh` -> `f3-carve.sh`, chains 1 to 6) | 03:18 to 04:41 | exit 0 (`f3-check-*.log`) |
| playbook 6, procedure 6 | pf2 (hand fixes `pf2-*-fixes.py` on a pw0 carve) | 03:51 to 04:20 | no pf2 check log; my run below is exit 0 |

No chain was cut: every `f3-chain*.log` ends with a complete `check-applied … exit=0` line, the last write before the commit is `f3-check-html.log` (04:44) and the oracle queue for zip/json/tiff/pdf (04:48), which only verified. No process of those runs is alive. So no package needed a restore from a backup; all 22 are completed carves.

### 1.2 What was wrong

1. The audit's "zero workspaces" holds under the repository's rule, not under Cargo's. By Cargo's workspace rule an explicitly listed member stays a member even when `exclude` names it (raw cargo is refused here, so I applied the rule myself), so under Cargo semantics all 117 are members (`pch-tools/membership.ts`, `🗑️generated/pch-zero-ws-cargo-semantics.tsv`: 117 of 117).
2. The repository does not use Cargo's rule. `L/🗂️workspaces/🦀️cargo/🟦️.ts` discovers members as `member-manifests` minus `[workspace.metadata.semio.repository] exclude-patterns`, and `publishCargoWorkspaceMembership` treats `members` as a published copy of that discovery. Every artifact workspace listed `**/🏅️standards/**` in `exclude-patterns`, so the carved crates under `🏅️standards/` were invisible: for bmp 8 declared members and 1 discovered, gif 9/1, space 5/1 (`pch-tools/repomembers.ts`). Healthy workspaces (fem 2d, gltf, generation2d) have declared == discovered and no standards exclusion.
3. The generator wrote `members` (`addWorkspaceMembers`) and never touched the exclusions. `--check-applied` compared name resolution only.
4. `project.json`: none of the carved crates has one, and none should. In the whole plugin tree no crate under `🏅️standards/` has a `📋️project.json` at its `📦️packages/🦀️rust` (0 of 1,223 plugin members; the 37 under standards are `🏭️generator` and one `🔬️probes`). Ownership is the owner row.

### 1.3 Repair

Applied with the generator's own function: `pch-tools/rewire.ts` (lifts the hiding exclusion entries), then the repository's publisher through `pch-tools/publish-write.ts` (`publishSelectedCargoWorkspaceMembershipV1`, strict).

| Set | Workspaces | Hidden crates before -> after |
|---|---:|---:|
| the 22 artifacts of the 117 (playbook, procedure, space, 19 stdio) | 22 | 117 -> 0 |
| same defect, older artifacts (15 norm, energy model, stdio ifc, xlsx, docx, pptx, block 2d/5d/3d) | 23 | 393 -> 0 |

Evidence after: `pch-publish-check-1.log` and `-2.log`: all 45 report `strict: current` (the repository's membership check, which several of them failed before); `pch-tools/resolve-names.ts` on the 117: inventory 1,894 packages at 11:50 (2,294 at 12:35 after peer growth), 117 known, 0 unknown; no duplicate package name in the inventory; plugin scan `pch-orphans-plugins-2.tsv`: 1,357 plugin packages, every one a member or its own workspace root, 0 hidden from the launcher (`pch-tools/hidden-list.ts`: 0).

Owner rows: `owners-tools/scan.ts` -> `packages 2492, owners 2639, missing []`. Router packages (`📦️packages/🦀️rust/📜️script.ts` `packages:`): every carved crate of the 22 listed (`pch-tools/router.ts`, 0 missing after adding the new mp3 crate). Reachability: every carved crate of the 22 is reachable from its facade in the artifact lock graph, 0 unreached (`pch-tools/reach.ts`). Locks: no member of the 22 is missing from its lock; the only `member/NOT-IN-LOCK` packages in the plugin tree are 11 `block` crates (`semio-s-artifact-block-{2d,5d}-io-*`, `-3d-editor-*`), carved earlier by another slice, not touched here: their artifact locks need `update --workspace --offline`.

## 2. Generator changes (library authoring module `L/🏗️authoring`)

### 2.1 Code

| File | Change |
|---|---|
| `📦️crate-tree/📝️render/🟦️.ts` | `hidingExcludePattern`, `workspaceAdmissionProblems`; `addWorkspaceMembers` now lifts the exclusion entries (cargo `exclude` and `exclude-patterns`) that would hide an added member. Every carve and every subset scaffold goes through it. |
| `📦️crate-tree/🔬️verify/🟦️.ts` | `sharedMountFiles`: files reached by more than one package (lib, test and bin roots, `#[path]` followed, missing generated files tolerated). |
| `📦️crate-tree/🧫️sandbox/🟦️.ts` | `siblingPathCrates`, `rewriteCrateAliases`; `verifyAppliedLayout` now builds the base first, copies into it the outside directories that only the base mounts (a dedupe removes the working-tree mount, so the old base lost its helper file and aborted early), and mounts as modules the path crates that a carved crate depends on when the base tree compiled the same sources in-crate (plugin-local ones transitively, other repository crates as direct dependencies only). Without this a dedupe to a sibling crate showed up as unresolved names. The report lists `siblings`. |
| `🎮️command/🟦️.ts` | `new crates <pkg> --check-applied [--base=<git-ref>] --sandbox=<dir>`: new `--base` (HEAD was hard-wired, which makes the gate vacuous once an auto-commit has absorbed the carve; the pre-carve ref is the meaningful base), plus two new failure conditions next to the layout differential: `admission` (a workspace member hidden from repository discovery) and `shared` (a plugin-owned source file reached by two packages). `sharedForeign` lists files outside the plugin (framework test helpers) for information. |
| `🧪️tests/📦️crate-tree/🟦️.ts` | 3 new tests (admission lift, shared-mount gate, sibling crates and alias rewrite). `bun test ./🧪️tests/📦️crate-tree/🟦️.ts`: 40 pass, 0 fail (it was 1 fail before: a peer's `demoteDanglingOuterDocs` test lacked its import; I added the one missing import name). |
| `🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json` | Removed the stale pin of `stdio/🗿️artifacts/🪟️bmp/🦀️.rs` (its root shrank below 150 lines in the F3 bmp carve). |

### 2.2 Gate semantics now

`--check-applied --base=f856dff0813` (the last commit before `1bae6115bc5`) is a real differential for all 22: carved layout against the original single-crate layout. Against `HEAD` it was vacuous for every carve that an auto-commit had already absorbed (HEAD is the carved tree). Space is special: against the pre-pilot tree `7af1259dba8` it reports 5 native / 6 test extra names, but those names (`::semio_framework_async::poll` in `📇fold-directory-events`, the macro-generated `SpaceIndexCommand`) belong to source files added after that base and are present identically in the pilot carve; against `f856dff0813` and `7c8af563c74` the differential is 0/0.

## 3. Shared source files (F7)

### 3.1 Classification of the 116 (scan `pch-tools/modowners.ts`, 20 s, `🗑️generated/pch-modowners-multi-{0,1,2}.tsv`)

| Class | Files | Note |
|---|---:|---|
| Plugin-owned, non-test mount site | 16 | wfc bitmap (2 incl. its pixel helper), generation native (3) and sqlite support, puzzle jobs/retained/scene (3), imperative extensions (5), mp3 metadata (2), drawing oracle support (1), csv trace allocator (1 framework-owned, csv side) |
| Plugin-owned, test-only | 21 | test supports and templates |
| Framework-owned (guest-contract 20, store testing 34, abi, ui, pack, value, test helpers, space sqlite) | 79 | other slices (finding F1, F2, F9) |

The generator did not create any of these. Each sharing predates the carve: `git show f856dff0813:` shows the procedure root already `#[path]`-mounting the five extension roots, the mp3 snapshot already mounting `🏷️metadata/🦀️.rs`, and the puzzle and wfc artifact roots (not carved) mounting the jobs, retained, scene and bitmap sources. The carve mirrored them. The mp3 `metadata-laws` crate is a hand-written oracle crate added 10-09 (`da607013f32`) that re-mounts two production files by path. What the generator could do, it now does: the gate in §2.2 fails an applied carve that leaves a plugin-owned file with a second owner.

### 3.2 Dedupe applied (every file now has exactly one owning crate; the others depend on it)

Technique: the mount `#[path = "…"] [pub] mod x;` becomes `[pub] use <owner crate> as x;` at the same place, so no caller changes and the public module path of an artifact stays; the owner is added as dependency (dev-dependency with `features = ["testing"]` where the test API is gated), workspace dependency table, lock entry, owner row, workspace member (through the repository publisher).

| Shared file(s) | Owner crate | Consumers | Differential (`diffcrates.ts`, base HEAD, onlyCombined native / test) |
|---|---|---|---|
| `🧩️puzzle/🎮️commands/📤️jobs/🦀️.rs` | `semio-s-plugin-puzzle-jobs` (existing) | puzzle 2d, 5d | 0 / 0 each |
| `🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` | `semio-s-plugin-puzzle-retained` (existing) | 2d, 5d | 0 / 0 |
| `🧩️puzzle/🔨️modules/🎲️board/🎬️scene/🦀️.rs` and its tests | `semio-s-plugin-puzzle-board-scene` (new, glue root `📦️packages/🦀️rust/🦀️.rs`, 217 lines of domain stay at owner path to meet the wiring budget) | 2d, 5d | 0 / 0 |
| `🀄️wfc/🪶️sqlite/🎨️bitmap/🦀️.rs` and `🧪️tests/🦀️.rs` | `semio-s-wfc-sqlite-bitmap` (new; feature `testing` exposes `pixel_laws`) | wfc 2d, grid2d | 0 / 0 each |
| `📜️imperative/🧩️extensions/{🎮️control,📣️effect,🧠️logic,🧮️math,📝️text}/🦀️.rs` and the effect unit test | the five `semio-s-plugin-imperative-*` crates (existing; the effect test used `crate::extensions::effect::*`, which only exists inside the snapshot crate, so it is now `use super::*`) | procedure snapshot | 0 / 0; `--check-applied` with `siblings` = the 5 crates: 0 / 0 |
| `🎵️mp3/…/🚪️io/🏷️metadata/🦀️.rs` and `…/📸️snapshot/🏷️metadata/🦀️.rs` | `semio-s-artifact-stdio-mp3-io-metadata` (new, the ID3 IO leaf) and the existing `semio-s-artifact-stdio-mp3-metadata`; the old `metadata-laws` crate is dissolved into the io-metadata crate as unit tests (its owner row, `Cargo.toml`, `Cargo.lock` removed; `metadata-native` command re-pointed; router `packages:` extended) | mp3 mutations crate | 0 / 0 (the base also compiled the mutations crate with unresolved `Id3*` imports; the move removes 4 diagnostics) |
| `🗄️stdio/🔮️oracles/🖊️drawing/🧰️support/🦀️.rs` | drawing oracle crate (`pub mod reference_support`) | dxf subset oracle (already its dependent) | 0 / 0 |
| `⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs` | `semio-framework-trace-memory-testing` (existing) | csv snapshot (dev-dependency) | 0 / 0 |
| `🏭️process/🧩️extensions/🧪️tests/🔬️catalog-json/🦀️.rs` | `semio-s-plugin-process-catalog-json-oracle` (new standalone workspace, with a lock closure) | metal, robotic, concrete, wood | 0 / 0 (metal) |
| `🪵️sourcing/🧩️extensions/🧪️tests/🔬️unit/🔮️oracle/🦀️.rs` | `semio-s-plugin-sourcing-catalog-oracle` (new standalone workspace) | slabs, windows, beams | 0 / 0 (slabs) |
| `🌀️procedural/🫀️core/🧬️generation/🪶️sqlite/🧪️tests/🧰️support/🦀️.rs` | `semio-s-procedural-core-generation-sqlite-test-support` (new, core workspace member) | generation 2d, 3d io-sqlite | 0 / 0 (2d) |
| `🧰️framework/…/🪐️space/🪶️sqlite/🦀️.rs` | `semio-framework-os-space-sqlite-cells` (existing) | framework space collection and space artifacts | not run: the framework lock lists neither artifact, so only sources and manifests changed |

Census rows (files / bytes / active lines, lib and test closure; `pch-census-rows.tsv`): scene 1/14,446/215 and 4/33,052/314; bitmap 1/9,537/97; mp3 io-metadata 1/9,360/79 and 6/214,455/127; process oracle 2/1,922/16; sourcing oracle 2/3,059/14; generation support 1/2,987/11. Consumers: mp3 mutations 19/66,335/981 and 23/89,327/1,213; csv snapshot 14/54,441/605; procedure snapshot 20/56,565/985. Puzzle 2d (340 files), 5d (316), wfc 2d (138), grid2d (110) were over the file limit before and are not carved yet (pre-existing, outside this slice).

Duplicate-mount scan after (`pch-modowners-multi-3.tsv`, 14:36; `-2` at 12:55 showed 42), files reached by more than one package: 116 -> 22. By plugin: wfc 2 -> 0, imperative 6 -> 0, stdio 3 -> 0, puzzle 4 -> 0, sourcing 1 -> 0, procedural 8 -> 7; process 2 -> 0, lowpoly 2 -> 0, cad 1 -> 0, remodel 1 -> 0, energy 1 -> 0 (the last five: one file of mine in process, the rest peer work). Framework 85 -> 15, all by peers.

### 3.3 What stays shared, and why (the analysis of 12:10 covered 13 plugin files; 7 remain after the peer work)

Rule used (it explains why I left the editor, mutation-law and generation supports alone, and why the generation3d ones remain): a shared file can become a dependency only if the crate it would become does not reach, through the artifact lock graph, any of its consumers. If it does, the consumers' tests would link a second, non-test copy of the crate under test through the dev-dependency, and every type that crosses the helper boundary splits (this is why these helpers are mounted into the test build). `pch-tools/cycles.ts` computes it:

| File (shared by) | Reaches consumers? |
|---|---|
| generation3d `⏱️phased-job` support (7), `construction-support` (4), `oracle-support` (28), `mesh-harness` (10) | yes: the helpers name the very geometry crates that mount them |
| process3d editor support (3), lowpoly editor support (5), lowpoly mutation laws (3), cad editor support (3), remodel mutation vectors (4) | yes (editor/mutation sub-crates of the artifact the helper instantiates) |
| energy binary mutation unit test (2) | not a library: a test template using `super::*`, mounted on purpose against two different parent scopes |
| generation native template `🚦️native` (3 files, generation2d and 3d) | not mechanical: it uses `super::*` of two host modules that each define the same seven DSL record types and the controlled snapshot aliases. A single owner needs those records moved into the owner and the snapshot access abstracted by a trait; derive macros and no compiler admission make that unsafe to do blind. Hand-off. |

The remaining framework-owned test helpers that plugin tests mount (`🧪️test/🛬️original-snapshot-owners` in 60 packages, `⚖️typed-input-law` 8, `⚖️transient-preparation-law` 3, `🚪️io/🪶️sqlite-snapshot/…/💰️backing` 12) are one file each with many plugin mount sites; each wants one framework test crate. Not done here (blast radius ~80 mount sites in ~65 plugin crates); the new gate reports them as `sharedForeign` and does not fail on them.

## 4. Verification (commands and real results)

| Check | Result |
|---|---|
| `bun test ./🧪️tests/📦️crate-tree/🟦️.ts` (in `L`) | 40 pass, 0 fail |
| `ONLY=<15 crate names> bun census.ts` (coordinator census, `🗑️generated/census.ts`) | stdout `crates 728, members 487, missing 0, oversized 1, compilable 628, carvableLeaves 1, withErrors 0` (audit 11:23: 727 / 486 / 3 oversized / 627). The filter returned no row for any of my new or changed crates: the census does not cover the plugin artifact workspaces, and with a filter it writes no file (the 11:23 `crate-census.csv` and `census-summary.json` are unchanged). My rows therefore come from the same closure counter (`pch-tools/measure.ts`, §3.2). |
| `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` | 2 pass, 1 fail: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧰️testing/🦀️.rs` is 152 lines (stdio contract slice, not touched here). Before my change the scene root had broken it too (fixed with the glue root) and the bmp pin was stale (removed). |
| `bun test ./🧪️tests/🔬️rust-mount-emitters/🟦️.ts` | 1 fail, foreign: a peer's new emitter `L/⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome/🟦️.ts` is not in the audited list. |
| `owners-tools/scan.ts` | 12:35: `missing: []` (2492 packages, 2639 owners); 14:30: 2513 packages, 2658 owners, `missing` = 2 framework packages that peers created since (`…/🧊️wgpu/⌨️native-entrypoint/…/📥️reader`, `🛎️services/🧪️tests/🍎️companion`); none of mine |
| `publish-check` (the repository's own membership check) on the 45 repaired workspaces plus puzzle, wfc and procedural core | 14:36: all strict `current`; declared == discovered |
| launcher inventory knows the 117 and the 6 new crates | 117/117, 6/6; no duplicate name in 2,294 packages |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | both parse as JSON (7,843,622 and 6,123,164 bytes) |
| 33 changed `.rs` files | `rustfmt --config skip_children=true --check`: 0 parse errors |
| 161 changed `Cargo.toml`/`Cargo.lock` files | all parse as TOML (peers' included) |
| `bun repo cargo … check -p <puzzle scene, jobs, retained> --lib` (mutex held, 12:10, 12:47, 15:07) | refused twice for headroom; the third attempt died in the launcher's cleanup/publication code (foreign); mutex released each time; nothing compiled |
| Resolution differentials of the dedupes (`pch-tools/diffcrates.ts`) | §3.2: onlyCombined 0 in lib and test for puzzle 2d/5d, wfc 2d/grid2d, imperative, mp3, drawing, csv, process metal, sourcing slabs, generation 2d |

## 5. Hand-offs and foreign findings

1. Run `-- update --workspace --offline` for every lock I touched by hand once compile admission returns: the 30 lock files in `🗑️generated/pch-lock-touched.txt` (artifact, bridge, hub and dev locks that carry the new packages), and the 11 stale `block` locks. Hand-patched entries follow what Cargo records for path dependencies (normal dependencies, optional ones only where a dev-dependency enables the feature), but none was checked by Cargo. The framework lock does not list the space artifacts (`semio-framework-artifact-space-*`) or the cells crate at all: framework workspace members are stale (`members --write`).
2. The generation native template (3 files) and the 4 generation3d test supports of §3.3 (consumers' dependency cycle).
3. Framework findings F1, F2, F9 and the framework test helpers (§3.3) are untouched; a peer sweep reduced framework shared files from 85 to 15 during the run.
4. Foreign test failures: lib-wiring-budget (stdio contract testing root, 152 lines) and rust-mount-emitters (mutation-outcome emitter).
5. The sandbox gate's source reader fails on puzzle 2d for an absent fixture (§6); fix the fixture path in the original, then `--check-applied` runs there.
6. Standalone oracle locks (`…/📦️packages/🦀️rust/Cargo.lock` of the process and sourcing oracle) are git-ignored by `.gitignore:623`, as the drawing oracle's lock already is; they exist on disk and are regenerated by `update`.
7. Pre-existing `[DEBUG] ` prints remain in test files I moved (for example the mp3 laws); I added none.
8. Generated files the sandbox cannot see: the 2d puzzle test closure needs a missing fixture (`…/import-snapshot/…/📄️payload-input/🔣️.json`) and the generated registry module; both sides of every differential carry the same errors and cancel.

## 6. `--check-applied` for every artifact I touched (real results)

Command, per artifact, from the repository root: `nice bun ./📜️script.ts new crates <artifact>/📦️packages/🦀️rust --check-applied --base=f856dff0813 --sandbox=<dir outside the repository>`. Logs: `🗑️generated/pch-check-final-<artifact>.log`, summary `pch-check-final.summary`. Exit 0 means `onlyCombined` 0 for lib and test, `admission` empty, `shared` empty.

| Artifact | Base | Exit | Carved crates | Native base / combined / extra | Test base / combined / extra | Hidden members / shared files |
|---|---|---:|---|---|---|---|
| `📖️playbook` | `f856dff0813` | 0 | 6 (+0 siblings) | 823 / 835 / 0 | 1345 / 1262 / 0 | 0 / 0 |
| `📜️procedure` | `f856dff0813` | 0 | 6 (+5 siblings) | 1050 / 1052 / 0 | 1596 / 1502 / 0 | 0 / 0 |
| `🪐️space` | `f856dff0813` | 0 | 4 (+0 siblings) | 792 / 792 / 0 | 1305 / 1315 / 0 | 0 / 0 |
| `☁️las` | `f856dff0813` | 0 | 5 (+0 siblings) | 775 / 775 / 0 | 1020 / 1026 / 0 | 0 / 0 |
| `🌐️html` | `f856dff0813` | 0 | 5 (+0 siblings) | 527 / 527 / 0 | 877 / 881 / 0 | 0 / 0 |
| `🌦️epw` | `f856dff0813` | 0 | 6 (+0 siblings) | 628 / 628 / 0 | 898 / 880 / 0 | 0 / 0 |
| `🎞️gif` | `f856dff0813` | 0 | 8 (+0 siblings) | 1574 / 1575 / 0 | 1996 / 2006 / 0 | 0 / 0 |
| `🎥️mp4` | `f856dff0813` | 0 | 7 (+0 siblings) | 714 / 714 / 0 | 927 / 931 / 0 | 0 / 0 |
| `🎵️mp3` | `f856dff0813` | 0 | 6 (+0 siblings) | 452 / 452 / 0 | 622 / 649 / 0 | 0 / 0 |
| `💬️bcf` | `f856dff0813` | 0 | 4 (+0 siblings) | 796 / 795 / 0 | 1113 / 1118 / 0 | 0 / 0 |
| `💾️binary` | `f856dff0813` | 0 | 4 (+0 siblings) | 437 / 437 / 0 | 702 / 703 / 0 | 0 / 0 |
| `📊️csv` | `f856dff0813` | 0 | 5 (+1 siblings) | 656 / 656 / 0 | 1097 / 1099 / 0 | 0 / 0 |
| `📑️tsv` | `f856dff0813` | 0 | 5 (+0 siblings) | 585 / 585 / 0 | 908 / 911 / 0 | 0 / 0 |
| `📝️md` | `f856dff0813` | 0 | 5 (+0 siblings) | 583 / 583 / 0 | 908 / 912 / 0 | 0 / 0 |
| `📷️png` | `f856dff0813` | 0 | 6 (+0 siblings) | 877 / 877 / 0 | 1214 / 1219 / 0 | 0 / 0 |
| `📼️avi` | `f856dff0813` | 0 | 5 (+0 siblings) | 672 / 672 / 0 | 863 / 867 / 0 | 0 / 0 |
| `🔊️wav` | `f856dff0813` | 0 | 5 (+0 siblings) | 797 / 797 / 0 | 1233 / 1237 / 0 | 0 / 0 |
| `🔤️txt` | `f856dff0813` | 0 | 5 (+0 siblings) | 622 / 622 / 0 | 1047 / 1053 / 0 | 0 / 0 |
| `🔺️stl` | `f856dff0813` | 0 | 5 (+0 siblings) | 527 / 527 / 0 | 793 / 802 / 0 | 0 / 0 |
| `🗽️obj` | `f856dff0813` | 0 | 5 (+0 siblings) | 1030 / 1030 / 0 | 1318 / 1294 / 0 | 0 / 0 |
| `🧱️ply` | `f856dff0813` | 0 | 4 (+0 siblings) | 790 / 790 / 0 | 993 / 1003 / 0 | 0 / 0 |
| `🪟️bmp` | `f856dff0813` | 0 | 7 (+0 siblings) | 634 / 634 / 0 | 876 / 886 / 0 | 0 / 0 |

22 of 22 artifacts exit 0 with 0 extra diagnostics (lib and test), no hidden member and no plugin-owned shared file.

Touched artifacts the generator did not carve (base `HEAD`, the tree before my dedupe edits; same command with `--base=HEAD`). Logs `pch-check-other-*.log`:

| Artifact | Exit | Siblings found | Native base / combined / extra | Test base / combined / extra | Hidden members | Shared plugin-owned files |
|---|---:|---|---|---|---|---|
| puzzle 5d | 0 (894 s) | board-scene, jobs, retained | 3891 / 3891 / 0 | 3241 / 3504 / 0 | 0 | 0 |
| wfc 2d | 0 (210 s) | semio-s-wfc-sqlite-bitmap | 1336 / 1336 / 0 | 2558 / 2558 / 0 | 0 | 0 |
| wfc grid2d | 0 (123 s) | semio-s-wfc-sqlite-bitmap | 1294 / 1294 / 0 | 2356 / 2356 / 0 | 0 | 0 |
| generation2d | 1 (28 s) | none | 11 / 11 / 0 | 11 / 11 / 0 | 0 | 3: exactly the generation-native template (`🚦️native/🦀️.rs`, `🌱️value`, `📏️rows`), documented in §3.3 |
| puzzle 2d | cannot run | | | | | The gate's source reader stops at `Missing Rust source input: …/🎮️commands/📥️import-snapshot/🧪️tests/🧫️fixtures/📄️payload-input/🔣️.json` (an original-tree defect: the fixture is absent). The dedupe is covered by the differential in §3.2 (0 / 0). |

`sharedForeign` is `🧰️framework/…/🧪️test/🛬️original-snapshot-owners/🦀️.rs` for each of these (framework-owned helper, §3.3).

Notes: 13 of the 22 (playbook, space, las, html, epw, gif, mp4, bcf, binary, tsv, obj, ply, bmp) were last run before two refinements of the sandbox (copying outside directories into the base, and mounting direct outside path dependencies), which only matter where a mount was replaced by a dependency; the three artifacts where I did that (procedure, mp3, csv) were re-run with the final code, and so were md, png, avi, wav, txt, stl. The first attempts of several artifacts died within seconds because a peer's half-written module (`🏃️process/💾️state/🟦️.ts`, `🏃️process/🧪️testing/🦀️cargo/🎯️exact/…`) broke the root script; `pch-check-robust.sh` retries until a report exists (`tries=` in the summaries).

## 7. Files changed (summary)

Generator and tests: the six files of §2.1. Data: 45 artifact workspace manifests (`Cargo.toml`), `📋️build-owners.json` (6 rows added, 1 removed), the Cargo.lock files named in §5, `✏️s/🔌️plugins/{🧩️puzzle,🀄️wfc,🌀️procedural/🫀️core}/Cargo.toml` (members), new crate manifests (scene, bitmap, mp3 io-metadata, process and sourcing oracle with their locks, generation support), the source mount edits of §3.2, and `pch-tools/` plus `🗑️generated/pch-*`.
