# Source boundary: the exact rule and the state of the fleet (2026-11-02)

## The rule (it is NOT a per-file cap)

Message: `Rust crate Source refused before compiler: <crate> (Rust crate Source extent or boundary exceeded before compiler byte allocation: {"reason":"bytes","path":<file>,"observedBytes":32979,"maximumBytes":29401,"remainingFiles":6})`.

- `maximumBytes` is the REMAINING crate budget at the moment that file is read: `maxSourceBytes - bytesAlreadyRead + (previous ?? 0)`. 29401 is not a file limit; the crate had already read 232743 bytes in 26 files and the next file did not fit. Any file can be "the one" that trips it, which is why untouched DIN/ISO files looked affected.
- Gate: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts` (`admitCargoSourceBoundariesV1`), `.../📖️read/🟦️.ts` (per-file read), closure reader `.../📥️source/🟦️.ts` (`readRustSourceClosureV1`).
- Unit of measure: ONE CRATE (the cfg-selected source closure of its `[lib]`/bin entry: every `mod`/`#[path]` mount plus `include!`/`include_str!`/`include_bytes!`), tests excluded only when cfg-gated (`#[cfg(test)]`). Limits (policy `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` `sourceBoundary`):
  - files <= 32 (every mounted file counts, so splitting a module into sub-modules INCREASES the count),
  - raw bytes <= 262144 (all files, embedded inputs included),
  - lines <= 4000 (`selected.split("\n").length`, comments and blank lines included).
- A crate may lower its own limits with `[package.metadata.semio.rust-source-boundary]` (`maxSourceBytes`, `maxSourceFiles`, `maxActiveLines`, all three required); `Math.min(policy, crate)` so it can never raise them. No growth-vs-HEAD budget exists; foreign vs own project does not matter.
- Consequence: splitting a module into smaller files or sub-modules does NOT help. The only fixes are fewer bytes/lines/files in the crate closure: move code into another CRATE (new `Cargo.toml`, workspace member, path dependency, re-export for API stability) or delete code. Aim for <= 230000 bytes / 28 files / 3500 lines (the repo target in `🏗️authoring/📦️crate-tree/🗺️plan/💰️budget/🟦️.ts`).

## Tool

`🧰️scripts/source-boundary-scan.py [root] [-v]` (python3 -I) approximates the closure per crate (cfg(test) stripped) and prints crates over a limit. Run it before a build: `python3 -I "$TICKET/🧰️scripts/source-boundary-scan.py" ✏️s/🔌️plugins/📕️norm`.

## Fixed in this pass (uncompiled except NDP crate, which was compiled standalone with rustc and exercised: 583 entries, lookup works)

1. `semio-s-plugin-norm-compliance` was 431 KB because the generated NDP table alone was 305 KB.
   - New leaf crate `semio-s-plugin-norm-ndp` (`⚖️compliance/📜️annex-and-tables/🔖️ndp/📦️packages/🦀️rust`, lib = `🔖️ndp/🦀️.rs`, 169 KB / 2 files). Types and lookup moved there; `NdpEntry::value(annex)` became trait `NdpEntryAnnex` in compliance (it needs `AnnexChoice`); compliance `📜️annex-and-tables/🦀️.rs` does `pub use semio_s_plugin_norm_ndp::*;` so the public API is unchanged.
   - Generator `🧰️ndp/codegen.py` now emits a compact table (interned `Sn` string consts + `row(..)` const fn, 305 KB -> 166 KB) and refuses output > 200000 bytes with a message to add a further crate.
   - `Cargo.toml` of compliance: path dependency; `✏️s/🔌️plugins/📕️norm/Cargo.toml`: new member.
2. `semio-s-plugin-norm-structural-scene` was 270 KB / 49 files / 6.8k lines.
   - New crate `semio-s-plugin-norm-structural-scene-foundation` (`🎬️structural-scene/🧱️foundation/`) mounts `🧬️schema` and `📈️diagram-kit` (about 20 files, 3.4k lines, 120 KB) with the same `extern crate` aliases; the scene crate does `pub use ...foundation::*;` and keeps colour-maps, tessellation, projection-kit, projection, diff (about 30 files, 3.3k lines). Public paths are unchanged (`structural_scene::diagram_kit`, flat schema re-exports). `schema/validation` helpers became `pub use` (tessellation used them across what is now a crate edge). Workspace member + path dependency added. Scene tests stay in the scene crate (they reach schema through the glob).
3. `en1993-checks` is now within limits (another agent split it while I worked).

## NOT fixed: `semio-s-plugin-norm-app-surface` (557 KB, 67 files, 11.2k lines) - blocks every artifact build

Needs about 3 crates, but the modules form ONE strongly connected component (symbol-level scan, `deps2.py` logic): `retained-commands`, `direct-manipulation`, `live-lanes`, `edit-rules`, `command-args-and-dispatch`, `value-path`, `outliner`, `visual-windows/viewport3d`, `visual-windows/checks`, `visual-windows/properties` (about 373 KB together, above the limit). Key back edges to cut with a trait or by moving the symbol down:
- `viewport3d -> live-lanes` (`DragPreview`, `ViewLane`, `LaneFrame`) and `live-lanes -> viewport3d` (`CameraPose`, `fields`);
- `viewport3d -> checks::controller_is_live` (move to a leaf);
- `edit-rules <-> command-args-and-dispatch` (`commit_snapshot_fields`, `NORM_LIVE_MAX_WRITES`) and `edit-rules <-> live-lanes` (`FieldWrite`, `GhostPreview`, `NormEdit`);
- `direct-manipulation <-> live-lanes` (`DragState`, `DragPreview`, `FieldWrite`), `value-path -> viewport3d::value_at`.
Acyclic leaves that can move out first: `diagrams2d` (49 KB, depends only on report-tree), `themes`, `device-profiles` (depend on viewport3d `LabelDensity`/`DeviceClass`, move those two types to a leaf), `accessibility`, `report-tree`, `ids-chrome-and-labels`, `field-meta`, `document-editors`, `catalogue-and-inspection`, `store-ownership-macros`.
Proposed layering (each crate re-exports the lower ones with `pub use lower::*;` so `use crate::*` / `use super::*` in the moved files keep working, and app-surface stays the aggregating crate): (A) `app-surface-foundation` = ids, field-meta, report-tree, document-editors, value-path types, store macros, themes, device-profiles, diagrams2d, accessibility; (B) `app-surface-interaction` = viewport3d (141 KB), direct-manipulation, selection-mirror after the cuts above; (C) `app-surface-live` = live-lanes, edit-rules, command dispatch, retained-commands, properties, checks; (D) `app-surface` = manifest-and-media, macros, root + `pub use`. This needs compiler feedback and must be done by the compile owner (`integrator`); the module breakdown with sizes is in the scan output.

## Other crates over the limit right now (artifact owners)

From the scan at 03:35 (bytes / files / lines):
- `en1992-checks` 385 KB / 24 / 6.4k; `en1997-checks` 243 KB / 17 / 4.5k: split `⚖️checks` into per-part crates (e.g. `design-effects`, `detailing`, ... as separate `📦️packages/🦀️rust` crates re-exported by the checks crate).
- `en1993-model` (snapshot) 245 KB / 5 / 7.3k, `en1993-diff` 241 KB / 5 / 7.4k, `en1993-schema` 187 KB / 5 / 5.8k: generated from the schema; shrink the generator output (shared helper crate or per-record-family crates). `vdi3805-model` (4.3k lines), `iso16757-model` (4.4k lines) and `en1993-diff` (5.2k lines) were already over the 4000-line limit at HEAD.
- `en1998-mut-*` (buildings 115 files, silos-tanks 61, members 49, bridges-towers 49, foundations 46): file count > 32, group the generated files into fewer modules per crate (do not add sub-module files).
Counts for the whole norm tree: `python3 -I 🧰️scripts/source-boundary-scan.py ✏️s/🔌️plugins/📕️norm -v`.

## Practical rules for agents (also in `📌️important/build-loop.md`)

Budget per crate: <= 262144 bytes, <= 32 files, <= 4000 lines (incl. comments/blank), counted over the whole cfg-selected closure. Check with the scan before every build. More files do not help; when a crate reaches ~230 KB / 28 files / 3500 lines, create a sibling crate (new `Cargo.toml` under `📦️packages/🦀️rust`, member entry in the plugin `Cargo.toml`, path dependency, `pub use` in the old crate) instead of a new sub-module. Generated tables must be compact and guarded by a size check in the generator.
