# Audit wave 7: census-4 splits (packages 1, 3, 5, 7, 8, 9, 11, 13, 14, 15, 21)

Auditor: Haiku 5.5, read-only, no sub-agents, no modifying git command. Scratch logs and the checker scripts are in `F/🗑️generated/aud7a-*` (`aud7a-manifest.py`, `aud7a-check3.py`, `aud7a-results3.json`, `aud7a-readers-resolved.txt`, `aud7a-lsfiles.txt`). `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`.

## 0. Verdict

Four classes checked. Confirmed defects: **class 1 (regions) 10**, **class 2 (private items) 0**, **class 3 (readers) 2 confirmed plus 1 unverified risk**, **class 4 (compat re-exports) 133**. Other checks: 44 child paths exceed 240 bytes when measured from the absolute path (0 when measured from the repo root), 1 sibling emoji clash, 6 files of package 15 missing from disk. The RESULT-CENSUS4-PKG-1 claim that no region pair is split across files is contradicted by section 2.1. The package 9 and 14 RESULTs do not mention their source-text readers, and two of those readers see only the parent (section 2.3).

## 1. Table: one row per package

"Files" = parents plus children that exist on disk. "R1..R4" = defect counts by class (section 2). "Other" = docstring, size, path, folder, sibling and missing-file checks.

| Pkg | Files (parents + children) | R1 regions | R2 private | R3 readers | R4 compat re-exports | Other |
|---|---|---:|---:|---:|---:|---|
| 1 | 9 + 31 = 40 | 9 | 0 | 0 | 22 | DEBUG carried from base in 3 unit-2 children (pre-existing, `[DEBUG]`-prefixed; not counted) |
| 3 | 8 + 38 = 46 | 0 | 0 | 0 | 0 | Parent `🔺️diff/🟦️.ts` is 1,013 lines (RESULT admits; a root, not a child). Risk R3-3 (unverified, not counted) |
| 5 | 10 + 37 = 47 | 0 | 0 | 0 | 16 | Consumer `🧪️tests/🔬️unit/🦀️.rs` 2,242 lines (not a split child, pre-existing size) |
| 7 | 3 + 16 = 19 | 0 | 0 | 0 | 5 | none |
| 8 | 8 + 34 = 42 | 0 | 0 | 0 | 22 | 10 child paths > 240 bytes absolute (repo-relative max 233) |
| 9 | 7 + 17 = 24 | 0 | 0 | 1 | 14 | 11 child paths > 240 bytes absolute (repo-relative max 237) |
| 11 | 8 + 29 = 37 | 0 | 0 | 0 | 14 | 9 child paths > 240 bytes absolute; sibling emoji clash 1 (section 2.5) |
| 13 | 8 + 27 = 35 | 0 | 0 | 0 | 24 | none |
| 14 | 9 + 68 = 77 | 0 | 0 | 1 | 0 | Parent `🔺️diff/🟦️.ts` (pdf) starts with `import` at HEAD too (pre-existing, not counted) |
| 15 | 6 + 38 = 44 | 1 group (6 markers lost; 6 files missing) | 0 | 0 | 16 | 6 files missing on disk (root + 5 children of target 4, staged `D`); DEBUG carried 1 (pre-existing) |
| 21 | 8 + 28 = 36 | 0 | 0 | 0 | 0 | 14 child paths > 240 bytes absolute (repo-relative max 239); two `🔣️.json` children have no docstring by format |

Totals: R1 10 (9 child files plus 1 lost group), R2 0, R3 2 confirmed plus 1 risk, R4 133, abs-path 44, sibling clash 1, missing 6.

Method for each package (short): parents and children come from the installed journals (`🗑️generated/c4pN-installed.json` for 1, 5, 9, 11, 13, 15), from the lists or specs (`c4p21-tools/new-files.txt`, `c4p14-tools/ts-new-files.txt` plus `c4p14-tools/specs/*.json`, `c4p7-tools/specs/*.py`, `c4p8-newfolders.txt`, `c4p3-taxdirs.txt`), with parent derived from the child folder when no list gives it (packages 3, 8, 14, 21). Region balance is checked per file (opens vs closes, stack never negative) and per group against the pre-split text: `c4p1` = `git show b84bf526c7b` (journal sha256 verified for every root), `c4p3-snap/*.orig.*`, `c4p7-tools/orig`, `c4p8-tools/orig`, `c4p15-tools/orig` (sha256 in `SHA256`); for packages 5, 13, 14 and 21 there is no snapshot, so `b84bf526c7b` is used as a fallback (marked as such in `aud7a-results3.json`). Package 9 process3d root has a journal sha mismatch with HEAD, so its base comparison is not reliable.

## 2. Defects, with exact locations and fixes

### 2.1 Class 1: region markers (R1 = 10)

Rule: every child is balanced on its own (each `//#region` has its `//#endregion` in the same file, and the reverse). Region totals per group equal the base (no marker lost), so the damage is pairing across files. Package 1 RESULT section 1 says "every region that has both markers in the original keeps both in one file": this is false for the nine children below.

| # | File:line of the unpaired marker | Markers in this file (opens / closes) | Fix |
|---|---|---|---|
| 1 | `W/🌉️bridge/🎞️scene-bridge/🦀️.rs` (lines 5 open, - close) | 1 / 0 | the region `World3dSceneBridge` opened at line 5 is never closed here; its close is in `camera-and-retirement` line 563. Close it at the end of this child and repeat an open+close pair around the cluster in every child it spans, or move the cluster so the pair stays in one file. |
| 2 | `W/🌉️bridge/🎥️camera-and-retirement/🦀️.rs` (lines 565, 697 open, 563, 695, 801 close) | 2 / 3 | closes at line 563 (`WorldCameraSync`) have no open in this file (open is in `scene-bridge` line 5); closes at 695 and 801 are fine. Open the region at the first line of this child and close it at the last. |
| 3 | `W/🗃️model/🔖️interaction-transaction/🦀️.rs` (lines 5 open, - close) | 1 / 0 | open at line 5 is not closed in this file; its close is in another child. Repeat the balanced pair around the moved cluster. |
| 4 | `W/🗃️model/🗂️registries/🦀️.rs` (lines 5 open, - close) | 1 / 0 | open at line 5 is not closed in this file; repeat the balanced pair around the moved cluster. |
| 5 | `W/🧩️components/🪄️placeholder-mesh-and-gumball/🦀️.rs` (lines 5, 27, 80, 81, 683, 705 open, 78 close) | 6 / 1 | 6 opens (5, 27, 80, 81, 683, 705) against 1 close (78): the root keeps the closes of regions the child now owns. Close each open at the end of its cluster in this child. |
| 6 | `W/🧵️interaction/🌪️vortex-relocate-and-terrain/🦀️.rs` (lines 5 open, 117, 346, 464, 582, 603 close) | 1 / 5 | open at line 5 is closed at 117 but closes at 346, 464, 582, 603 have no matching opens here (their regions opened in `interaction` root or siblings). Repeat each region balanced around its cluster in this child. |
| 7 | `W/🧵️interaction/📜️plan-and-publish/🦀️.rs` (lines - open, 536, 577 close) | 0 / 2 | closes at 536 and 577 have no opens in this file. Open the regions at the first line of their cluster in this child. |
| 8 | `W/🎛️host/🧪️tests/🔬️unit/🖼️reference-and-pick-lane-laws/🦀️.rs` (lines 277, 369, 434, 471 open, 432, 469, 503 close) | 4 / 3 | 4 opens (277, 369, 434, 471) against 3 closes (432, 469, 503): one region is split. Close it inside this fragment. |
| 9 | `W/🎛️host/🧪️tests/🔭️unit-2/🥽️brush-environment-and-hover-laws/🦀️.rs` (lines 185, 603 open, 183, 373, 601, 655, 854 close) | 2 / 5 | 2 opens (185, 603) against 5 closes (183, 373, 601, 655, 854): regions opened in the root or another fragment are closed here. Repeat the pair in this fragment. |

Also: 3 PKG-1 roots (`🎛️host`, `🧩️components`, `🧱️scene`) were already unbalanced at HEAD with the same counts, so they are pre-existing and not counted. The label mismatch `World3dSceneBridge` / `WorldCameraSync` is already in the base (HEAD line 78 open, line 2287 close) and should be fixed with the pairs.

Package 15, target 4 (`🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit`): base has 6 region markers, the 6 files (root and 5 children) are gone from disk. Section 2.5.

### 2.2 Class 2: private free items used from a sibling or parent (R2 = 0)

Static scan of every moved `struct`/`enum`/`fn`/`const`/`type`/`trait` (top-level, and private methods of inherent impls, not trait impls or trait bodies) that is used by name in another file of the same group. Comment-only mentions were ignored. Every hit left is in a group mounted with `include!`, so all fragments share one module scope and privacy does not apply (packages 1, 5, 11, 15). No `#[path]`-mounted child has a private item used elsewhere. Not covered: private struct fields and methods reached through a different receiver name, and anything that needs a compiler (the RESULT compile queues were not run by me).

### 2.3 Class 3: source-text readers that see only the parent (R3 = 2 confirmed, 1 risk)

Method: every literal `include_str!(…)`, `?raw` import and `readFileSync(…)` in the repo was resolved relative to its file and matched against the split parents and children (`aud7a-readers-resolved.txt`). Helper-based readers (`read_world_tree`, `readModuleTreeSource`, `read_joined`, `concat!` lists) were checked by hand where they touch a split file.

- **R3-1 (package 9, confirmed).** `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:90` reads `rust: include_str!("🦀️.rs")`, the root only (209 lines; HEAD 2,037). The three moved children `🎁️outputs-and-reports/🦀️.rs`, `📊️status-search-and-analysis/🦀️.rs` and `🔀️adjacency-and-validation/🦀️.rs` (same folder) are not read by any include, so the rust leaf loses them. **Fix:** make the leaf a `concat!` of the root and the three children in the monolith order, the way `vdi3805` does at its line 61 (its four children are all included, checked). Then verify the concatenation equals the HEAD text.
- **R3-2 (package 14, confirmed).** `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:140` reads `typescript: include_str!("🧬️mutations/🟦️.ts")`, the root only (358 lines; HEAD 2,548). The 48 group leaves `🧬️mutations/<group>/🟦️.ts` (2,646 lines, the payload interfaces the root imports from `./⏯️run/🟦️.ts` and the others) are not in the leaf text. **Fix:** `concat!` the root with the 48 group leaves in HEAD order, as package 3 did for BIM mutations (`🧬️schema/🦀️.rs` concat at its lines 57 to 63). Check the concatenation equals the HEAD text.
- **R3-3 (package 3, risk, not counted).** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🗂️documents/🟦️.ts:40` and `:138` read schema leaves with `readFileSync(path, "utf8")`. The architect schema root `…/🧬️schema/🟦️.ts` is now 750 lines (HEAD 2,447) with four children. RESULT-CENSUS4-PKG-3 section 4 rolled back the reader fix and relied on a scoped check that does not prove the projection sees the children. **Fix:** make `SourceStore` read split TypeScript leaves as module trees (the rolled-back `parser-documents.mine.ts` in `🗑️generated/c4p3-snap`), or prove the projection output is equal with and without the children.
Checked and fine: package 3 BIM mutations descriptor (`concat!` of root and six children), package 11 `vdi3805` concat (all four children present), package 7 `process3d` TS reader of child `🏭️production-envelope/🦀️.rs`, package 1 test roots `🗃️model/🧪️tests/🔬️unit/🦀️.rs` and `🧱️scene/🧪️tests/🔬️unit/🦀️.rs` call their own `read_world_tree` helper (defined in each file, 3 call sites per file); its splice behaviour is as RESULT-CENSUS4-PKG-1 section 2 states and was not re-verified by me.

### 2.4 Class 4: compatibility re-exports (R4 = 133)

Rule (memory `feedback-splitters-drop-regions-and-struct-visibility`): a parent that keeps `pub use child::*;` to keep old import paths is a compat re-export. The explicit `pub(crate) use child::*;` wiring is allowed. Every line below is a glob `pub use <child>::*;` (not `pub(crate)`) in a split parent. **Fix:** change to `pub(crate) use <child>::*;` where needed inside the crate, re-point the importers to the child path, and drop the glob if no importer remains (compile to confirm, not run by me). Zero TypeScript barrels were found for children. The one `export { … } from "@semio-tech/framework-os/…"` in package 3 `🧬️schema/🟦️.ts:10` is already in HEAD and is not counted.

- Package 1: 22 lines (full list in section 4).
- Package 5: 16 lines (full list in section 4).
- Package 7: 5 lines (full list in section 4).
- Package 8: 22 lines (full list in section 4).
- Package 9: 14 lines (full list in section 4).
- Package 11: 14 lines (full list in section 4).
- Package 13: 24 lines (full list in section 4).
- Package 15: 16 lines (full list in section 4).

### 2.5 Other findings (not one of the four classes)

- **Missing output, package 15 target 4.** The root `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs` and five children (`⏸️suspend-resume-laws`, `📨️frame-budget-and-bridge-laws`, `🚀️pump-and-spawn-laws`, `🚦️lane-priority-and-replay-laws`, `🧰️shard-harness`) are absent from disk. `git status` shows them staged as deleted (`D `). The journal `c4p15-installed.json` (`dist`) still lists them and RESULT-CENSUS4-PKG-15 section 7 calls the file an orphan that was "split as asked". **Fix:** decide with the owner whether the deletion was intended. If yes, remove the `dist` entries from the journal and the RESULT and drop the claim. If no, restore the HEAD file (`c4p15-tools/orig/4.rs`) and re-run the target-4 split.
- **Path length.** Repo-relative, every child is at most 239 bytes (limit 240): pass. Measured from the absolute path on this machine, 44 child files exceed 240 bytes (listed in section 4). Fix only if the absolute reading is intended: shorten the kebab names of the parent folders or the emoji-prefixed chain.
- **Sibling emoji clash (package 11).** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📦️design-effects/🦀️.rs` uses `📦️`, the same emoji as the sibling folder `📦️packages`. **Fix:** give `design-effects` a different emoji that is not used by a sibling.
- **Carried `[DEBUG]` lines (not split defects).** `println!("[DEBUG] …")` lines in the three unit-2 children and in `🔏️trusted-catalog/🧪️tests/🔬️unit/📦️plugin-module-laws/🦀️.rs` already exist at HEAD (24 and 1). They carry the prefix AGENTS requires, so they are not counted.
- **Unbalanced parents at HEAD (pre-existing, not counted).** Package 1 `🎛️host`, `🧩️components`, `🧱️scene` roots; package 5 editor root (8 opens, 7 closes at HEAD fallback); package 11 `⚖️checks` roots `en1997` (2/0) and `en1992` (3/1).
- **Base unreliable.** Package 9 process3d editor root: journal `base_sha` does not match HEAD, so its base check says nothing.

## 3. Limits

No compiler, no `tsc`, no `cargo`, no test run (I am read-only and the RESULT says compile admission is closed). Class 2 is a name-based static scan; it does not check private fields or receiver-named methods. Class 3 resolves only literal paths and the helper names listed in section 2.3; a reader that builds its path from parts would be missed. Derived parents (packages 3, 8, 14, 21) follow the folder rule and were checked by existence, not by reading the old parent for each child. Snapshot baselines for packages 5, 13, 14 and 21 are HEAD `b84bf526c7b`, not the pre-split state, so the region totals for those packages are approximate.

## 4. Exact lists

### 4.1 Compat re-exports (class 4), `file:line: text`

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🌉️bridge/🦀️.rs:80: pub use scene_bridge::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🌉️bridge/🦀️.rs:84: pub use overlays_and_gumball::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🌉️bridge/🦀️.rs:88: pub use camera_and_retirement::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🦀️.rs:258: pub use sync_and_overlay::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🦀️.rs:262: pub use render::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🦀️.rs:266: pub use snapshot_and_input::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs:27: pub use prepared_resources::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs:31: pub use scene_records::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs:35: pub use registries::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs:39: pub use tool_run_trace_layer::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs:56: pub use interaction_transaction::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🦀️.rs:546: pub use face_overlay_and_fit::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🦀️.rs:550: pub use placeholder_mesh_and_gumball::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️cursors/🦀️.rs:256: pub use marquee::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️cursors/🦀️.rs:260: pub use gumball_and_brush::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️cursors/🦀️.rs:264: pub use ray_and_component_pick::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧱️scene/🦀️.rs:131: pub use asset_io::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧱️scene/🦀️.rs:135: pub use brush_lod_and_style::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧱️scene/🦀️.rs:139: pub use environment_and_terrain::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs:247: pub use pointer_and_picking::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs:251: pub use vortex_relocate_and_terrain::*;`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs:433: pub use plan_and_publish::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs:50: pub use self::brush_meshes::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs:59: pub use self::collision_engine::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs:65: pub use self::precompute_session::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs:32: pub use self::fill_contract::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs:40: pub use self::builder_state::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs:60: pub use self::fill_run::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️.rs:64: pub use self::fill_revalidation::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🦀️.rs:40: pub use self::engine_bridge::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🦀️.rs:44: pub use self::action_context::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🦀️.rs:48: pub use self::context_menu::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🦀️.rs:57: pub use self::session_registry::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎟️session/🦀️.rs:64: pub use self::instance_residency::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:135: pub use self::retained_commands::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:149: pub use self::clipboard_job::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:153: pub use self::play_app::*;`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:160: pub use self::manifest::*;`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:237: pub use play_app::*;`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:245: pub use manifest::*;`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:204: pub use contributions::*;`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:322: pub use play_app::*;`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:330: pub use manifest::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌟️feature/🦀️.rs:23: pub use detectors::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌟️feature/🦀️.rs:27: pub use descriptors_matching_and_flow::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌟️feature/🦀️.rs:31: pub use akaze_scale_space::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌫️dense/🦀️.rs:116: pub use patch_match_stereo::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌫️dense/🦀️.rs:120: pub use fusion_and_tsdf::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🌫️dense/🦀️.rs:124: pub use point_cloud_analysis::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🎞️h264/🦀️.rs:59: pub use encoder::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️motion/🦀️.rs:25: pub use tracking_and_trajectories::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️motion/🦀️.rs:29: pub use kinematics_modal_and_sync::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️motion/🦀️.rs:33: pub use camera_motion_compensation::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️motion/🦀️.rs:37: pub use non_rigid_and_pose::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️.rs:19: pub use frame_ingest::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️.rs:23: pub use engine_vocabulary::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️.rs:27: pub use engine_core::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️.rs:39: pub use preview_and_products::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📷️camera/🦀️.rs:20: pub use intrinsics::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📷️camera/🦀️.rs:24: pub use poses_and_rigs::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📷️camera/🦀️.rs:28: pub use calibration::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/📐️geometry/🦀️.rs:41: pub use matrix_helpers::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/📐️geometry/🦀️.rs:45: pub use two_view::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/📐️geometry/🦀️.rs:49: pub use triangulation_and_pnp::*;`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/📐️geometry/🦀️.rs:53: pub use tracks_global_and_loop_closure::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:97: pub use adjacency_and_validation::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:113: pub use outputs_and_reports::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:129: pub use status_search_and_analysis::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🌐️context-requirements/🦀️.rs:7: pub use site_and_organization_requirements::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🌐️context-requirements/🦀️.rs:11: pub use schedule_growth_and_cost_requirements::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🌐️context-requirements/🦀️.rs:15: pub use delivery_risk_and_conflict::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/📋️decisions/🦀️.rs:7: pub use requirements_and_decisions::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/📋️decisions/🦀️.rs:11: pub use quality_change_and_reports::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🔁️operations-requirements/🦀️.rs:7: pub use process_flow_and_resources::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🔁️operations-requirements/🦀️.rs:11: pub use human_factor_and_compliance::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🗃️records/🦀️.rs:7: pub use status_workshop_and_audit_records::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🗃️records/🦀️.rs:11: pub use benchmark_compliance_and_governance::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🧭️program-structure/🦀️.rs:7: pub use project_and_stakeholders::*;`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🧭️program-structure/🦀️.rs:11: pub use elements_and_relationships::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:19: pub use balance_model::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:23: pub use norm_tables::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:27: pub use derived_balance::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:33: pub use document_evaluation::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:19: pub use design_approach_and_parameters::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:23: pub use geotechnical_parts::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:31: pub use project_checks::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:18: pub use comfort_models_and_parsers::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:22: pub use annex_parameters_and_parts::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:26: pub use environment_check_entry::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:20: pub use helpers_and_annex_parts::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:26: pub use design_effects::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:30: pub use member_evaluation::*;`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🦀️.rs:34: pub use anchor_evaluation::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs:11: pub use bars_and_beams::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs:15: pub use plane_continuum::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs:19: pub use plate_dkt::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs:11: pub use bars_and_frames::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs:15: pub use solids::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🦀️.rs:19: pub use shell_facet::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🎵️subspace/🦀️.rs:12: pub use modal_input::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🎵️subspace/🦀️.rs:16: pub use job_core::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🎵️subspace/🦀️.rs:23: pub use restore_cursor::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🎵️subspace/🦀️.rs:27: pub use run_and_publish::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🦀️.rs:12: pub use job_and_steps::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🦀️.rs:16: pub use construction_and_restore::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🦀️.rs:20: pub use run_and_entry::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🦀️.rs:10: pub use factor_and_job::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔺️ldlt/🦀️.rs:14: pub use restore_cursor::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs:13: pub use numerical_protocol::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs:17: pub use storage_and_ordering::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs:21: pub use checkpoint_pages::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs:12: pub use planar_domain::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs:16: pub use triangulation::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs:20: pub use mesh_job::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🦀️.rs:19: pub use job_state_and_construction::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🦀️.rs:23: pub use element_build::*;`
- `✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧩️assembly/🦀️.rs:27: pub use csr_build_and_run::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:30: pub use publisher::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:34: pub use guest_residency::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:38: pub use load_progress::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:42: pub use guest_codec::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:46: pub use native_codec::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:50: pub use verified_catalog::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:54: pub use verification_cache::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:58: pub use loader::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:64: pub use fixture_builders::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧾️descriptor/🦀️.rs:23: pub use bundle_validation::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧾️descriptor/🦀️.rs:27: pub use native_provider::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧾️descriptor/🦀️.rs:31: pub use retained_assets::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧾️descriptor/🦀️.rs:35: pub use open_targets::*;`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧾️descriptor/🦀️.rs:39: pub use descriptor_validation::*;`
- `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:17: pub use ledger_types::*;`
- `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs:21: pub use gis_map_undo::*;`

### 4.2 Child paths over 240 bytes from the absolute path

- package 8: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🎞️h264/📖️bitstream-and-headers/🦀️.rs` (242 bytes)
- package 8: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🎞️h264/🧼️deblocking-and-output/🦀️.rs` (242 bytes)
- package 8: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧊️dense-and-meshing-stages/🦀️.rs` (242 bytes)
- package 8: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/📥️ingest-gate/🦀️.rs` (254 bytes)
- package 8: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🧊️synthetic-scene/🦀️.rs` (258 bytes)
- package 8: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔗️pipeline-contracts/🦀️.rs` (261 bytes)
- package 8: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics/🦀️.rs` (260 bytes)
- package 8: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🧪️tests/🔬️unit/♻️dense-replay/🦀️.rs` (254 bytes)
- package 8: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/📐️geometry/📍️triangulation-and-pnp/🦀️.rs` (244 bytes)
- package 8: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/📐️geometry/🛤️tracks-global-and-loop-closure/🦀️.rs` (253 bytes)
- package 9: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🌐️context-requirements/⚠️delivery-risk-and-conflict/🦀️.rs` (253 bytes)
- package 9: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🌐️context-requirements/🏢️site-and-organization-requirements/🦀️.rs` (262 bytes)
- package 9: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🌐️context-requirements/🗓️schedule-growth-and-cost-requirements/🦀️.rs` (265 bytes)
- package 9: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/📋️decisions/🎯️requirements-and-decisions/🦀️.rs` (243 bytes)
- package 9: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/📋️decisions/📈️quality-change-and-reports/🦀️.rs` (243 bytes)
- package 9: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🔁️operations-requirements/⚙️process-flow-and-resources/🦀️.rs` (256 bytes)
- package 9: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🔁️operations-requirements/🛡️human-factor-and-compliance/🦀️.rs` (258 bytes)
- package 9: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🗃️records/📝️status-workshop-and-audit-records/🦀️.rs` (248 bytes)
- package 9: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🗃️records/🧾️benchmark-compliance-and-governance/🦀️.rs` (250 bytes)
- package 9: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🧭️program-structure/🏗️project-and-stakeholders/🦀️.rs` (249 bytes)
- package 9: `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗄️registers/🧭️program-structure/🧱️elements-and-relationships/🦀️.rs` (251 bytes)
- package 11: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧱️pile-wall-slope-and-uplift-checks/🦀️.rs` (241 bytes)
- package 11: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/⚖️compliance/🧮️design-approach-and-resistance-laws/🦀️.rs` (242 bytes)
- package 11: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📚️document-checks/🗂️file-header-and-historical-checks/🦀️.rs` (252 bytes)
- package 11: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧱️sheet-checks/🔥️heat-generator-and-operative-sheet-checks/🦀️.rs` (257 bytes)
- package 11: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧱️sheet-checks/🚰️valve-radiator-and-pump-checks/🦀️.rs` (246 bytes)
- package 11: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📈️part1/🌐️properties-and-multilingual-checks/🦀️.rs` (257 bytes)
- package 11: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📈️part1/🎯️selection-and-bim-checks/🦀️.rs` (247 bytes)
- package 11: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📈️part1/🧩️variants-accessories-and-indexes/🦀️.rs` (255 bytes)
- package 11: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/📈️part1/🪪️header-ids-and-referential-integrity/🦀️.rs` (259 bytes)
- package 21: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/💨️airflow-and-controls/🌡️thermostats-and-humidistats/🐍️.py` (261 bytes)
- package 21: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/💨️airflow-and-controls/💯️ideal-loads/🐍️.py` (245 bytes)
- package 21: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/📆️schedules-and-layers/⏱️schedules/🐍️.py` (242 bytes)
- package 21: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/📆️schedules-and-layers/🧊️glazing-and-materials/🐍️.py` (255 bytes)
- package 21: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🔌️power-and-plant/❄️refrigeration-water-and-faults/🐍️.py` (258 bytes)
- package 21: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🔌️power-and-plant/🚿️hot-water-and-solar-thermal/🐍️.py` (256 bytes)
- package 21: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🔧️zone-systems/🌬️air-loops-and-setpoints/🐍️.py` (249 bytes)
- package 21: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🔧️zone-systems/🏭️plant-loops-and-outdoor-air/🐍️.py` (253 bytes)
- package 21: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🎞️e2e/🟦️.ts` (260 bytes)
- package 21: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/📏️measure/🟦️.ts` (264 bytes)
- package 21: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/📦️box-length/🟦️.ts` (267 bytes)
- package 21: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🕰️history/🟦️.ts` (264 bytes)
- package 21: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🕹️transforms/🟦️.ts` (267 bytes)
- package 21: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🗿️artifact/🧪️tests/🧪️semio-tech-cad-js-core-interactions/🗂️registries/🟦️.ts` (267 bytes)

### 4.3 Missing on disk (package 15 target 4 and its children)

- package 15: `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/⏸️suspend-resume-laws/🦀️.rs`
- package 15: `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/📨️frame-budget-and-bridge-laws/🦀️.rs`
- package 15: `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🚀️pump-and-spawn-laws/🦀️.rs`
- package 15: `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🚦️lane-priority-and-replay-laws/🦀️.rs`
- package 15: `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🧰️shard-harness/🦀️.rs`
- package 15: `🌎️hub/🔨️modules/🛡️admin/🧪️tests/🔬️dist-assets-bll2-7qi-unit/🦀️.rs`

