# Result CENSUS4-PKG-8 (prefix `c4p8`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/c4p8-tools/`. Logs: `F/🗑️generated/c4p8-*` (nothing deleted). Scratch (stages, sandboxes): `scratchpad/c4p8/`.

## 0. Found state (written first, after the session restart)

- The Claude Code session restarted while the work was running. Recovery check done before anything else:
  - `F/c4p8-tools/` complete (splitter `c4p8.py`, specs, `orig/*.rs.txt` pre-edit snapshots, oracle, census, `mkspec.py`).
  - The install of all 8 files had finished before the restart: no half-applied split. Every parent file differs from its snapshot, all 34 child files exist, `c4p8.py lossless` on the live tree: 8 of 8 `LOSSLESS OK`, 0 missing lines.
  - The only unfinished command was the `--check-applied` run after the install: its log stopped without an `exit=` line (kept as `c4p8-checkapplied-after-run1-cut.log`). It showed `native onlyCombined 38` (all `E0432` on `schema::mutations::<x>::<X>`, files that are not mine) and `combined 2146` against `3180` before. It is re-run below (section 5).
- Status will be kept current in section 1.

## 1. Status and verdict (kept current)

- Verdict: all 8 files of package 8 are split (34 new child files in 34 new folders, all at most 793 lines; parents 20 to 195 lines). Lossless 8 of 8, typeck oracle clean (canary reached, 0 new rows, lib and test), rustc differential clean for lib, test and wasm32-wasip2, `lit.py` 0 new, rustfmt 42 of 42 parse, 0 duplicate file mounts, census inside the gate. `--check-applied` after the install: see section 5 (re-run after the restart).
- Real `bun repo cargo` was not run (compile admission is closed for the fleet in wave 6); the commands for the compile queue are in section 7.

## 2. What changed

Method: item-level splitter `c4p8-tools/c4p8.py` (descendant of the c4p6 tool). Every top-level item keeps its text; children are `#[path = "<folder>/🦀️.rs"] mod <snake>;` in the parent plus `pub use`/`use <snake>::*;` (a `pub use` only where the child has a `pub` item, so the public paths are unchanged). Private definitions used by another file of the split became `pub(super)` (widening planner `widen2.py`: items, fields, impl members, driven by identifier presence in the other children and in the test files that read the parent through `super::*`). Regions (`// #region`) are carried into the children, balanced; empty region shells left in a parent were removed, the region names all live on in a child. Children start with `//! <emoji>` equal to their folder emoji and `use super::*;` (dropped where the child uses no parent name: h264 `encoder`, camera `intrinsics`).

| file (under `.../✏️editor/⚙️engine/`) | lines before | parent lines after | widened definitions | children (lines) |
| --- | ---: | ---: | ---: | --- |
| `🌟️feature/🦀️.rs` | 1301 | 37 | 21 | `🔎️detectors` 630; `🧬️descriptors-matching-and-flow` 345; `🌀️akaze-scale-space` 331 |
| `🌫️dense/🦀️.rs` | 1835 | 130 | 18 | `🧩️patch-match-stereo` 692; `🧊️fusion-and-tsdf` 544; `☁️point-cloud-analysis` 492 |
| `🎥️video/🎞️h264/🦀️.rs` | 2725 | 195 | 107 | `📖️bitstream-and-headers` 362; `🔤️cavlc` 261; `🧮️reconstruction` 557; `🧼️deblocking-and-output` 396; `🍰️slice-data` 756; `✍️encoder` 207 |
| `🏃️motion/🦀️.rs` | 1003 | 43 | 9 | `🎯️tracking-and-trajectories` 294; `🎼️kinematics-modal-and-sync` 288; `🪞️camera-motion-compensation` 246; `🫧️non-rigid-and-pose` 166 |
| `🏭️reconstruction/🦀️.rs` | 2291 | 51 | 115 | `📥️frame-ingest` 218; `🎛️engine-vocabulary` 264; `🏗️engine-core` 317; `🔬️sparse-stages` 771; `🧊️dense-and-meshing-stages` 466; `👁️preview-and-products` 272 |
| `🏭️reconstruction/🧪️tests/🔬️unit/🦀️.rs` | 1477 | 20 | 12 | `📥️ingest-gate` 87; `🧊️synthetic-scene` 237; `🔗️pipeline-contracts` 344; `🔎️orbit-diagnostics` 556; `♻️dense-replay` 274 |
| `📷️camera/🦀️.rs` | 1052 | 58 | 18 | `🔭️intrinsics` 173; `🧭️poses-and-rigs` 429; `📐️calibration` 412 |
| `📸️sfm/📐️geometry/🦀️.rs` | 1796 | 83 | 16 | `🧮️matrix-helpers` 103; `🔀️two-view` 793; `📍️triangulation-and-pnp` 451; `🛤️tracks-global-and-loop-closure` 380 |

Special cases:
- `🏭️reconstruction/🦀️.rs`: the 1,138-line `impl ReconstructionEngine` was cut into four inherent impl blocks at three method boundaries (before `step_extracting_features`, `step_finalize_reconstruction`, `progress`) by a verified-unique-anchor pre-edit (`specs/recon.py`), so the driver, the sparse stages and the dense/meshing stages sit in different children. The only added lines are 3 x `}` and 3 x `impl ReconstructionEngine {`.
- `🏭️reconstruction/🧪️tests/🔬️unit/🦀️.rs` (hoisted into the engine facade): 15 `#[test]` names identical (`testnames.py`: orig 15, new 15). The nested `dense_dump::encode` became `pub(in super::super)` because the orbit diagnostics now live in another child.
- Item-level `mod x;` lines with `#[path]` (the `terminal_sparse_chunk_tests` mount) stay in the parent, so their relative path is unchanged.
- Crate roots (`📦️packages/🦀️rust/🦀️.rs`) and all Cargo.toml files are untouched; the fleet files `🔌️plugin/**`, the stdio/layout files of `ldd` and the video unit test were not edited.

## 3. Data and tables (run-time fixtures)

No file was changed for this item. Measured: the only data in the 8 files are the 15 H.264 CAVLC / deblocking / CBP / chroma-QP constant tables (about 110 lines, 2.5 KB, now in `🔤️cavlc` and `🧼️deblocking-and-output` and `🍰️slice-data`) and a few `const` arrays of at most 16 elements elsewhere; the test file has no inline data of size (the long lines are `eprintln!` diagnostics). The tables are production decoder constants of a wasm plugin (no file system at run time), so they stay compile-time; moving them to a run-time file would add I/O to a pure decoder. Bytes are therefore identical by construction (lossless multiset). No test fixture was created.

## 4. Source-text readers

Searched with `git grep -F` over the repo for every moved file path and for `include_str!`/`include_bytes!`/`read_to_string` in the engine tree: no reader of the 8 files exists (only `Cargo.toml` workspace member lines and `📋️build-owners.json` crate directories, which are crate-level and unchanged). The one hit for `📷️camera/🦀️.rs` is a doc comment in the animate plugin. Nothing to re-point.

## 5. Verification (every command was run; results are the real ones)

All paths are relative to `c4p8-tools/` unless stated; logs are `F/🗑️generated/c4p8-*`.

| check | command | result |
| --- | --- | --- |
| lossless line multiset (live tree) | `python3 c4p8.py lossless <spec> <live dir>` for 8 specs (`c4p8-live-lossless.log`) | 8 of 8 `LOSSLESS OK`, 0 missing lines. Extras are only the wiring (`#[path]`, `mod`, `use x::*`), the child docstrings, `use super::*;`, region markers carried into several children (recon `Pipeline` x4, `Preview` x2, reconunit +1) and the 3+3 declared impl-split lines. Region markers balanced (open = close) in every file. |
| test names | `python3 testnames.py <8 specs>` | 15 `#[test]` in the unit file before and after, identical set; 0 tests in the other 7 files |
| typeck oracle (pos, prefix c4p8, 17 crates: my 7 plus direct dependents video, engine, sfm, geo, mesh, mesh-surface, mesh-atlas, mesh-repair, cmd-ingest, session; base `b84bf526c7b`) | `c4p8-tools/run-oracle.sh pre|post lib,test` = `POS_PREFIX=c4p8 python3 pos-tools/pos.py run c4p8-tools/families/c4p8.json --cfgs lib,test --label <L>` | pre (before the edit): lib 1902, test 3277 errors, NEW 0. post (after): lib 1902 errors, test 3277 errors, base equal, **NEW 0, GONE 0**, privacy/trait-scope/resolve/type/other all 0 (non-stub, new), **canary reached** (`E0308 E0616 E0624` on both sides, lib and test). Neutralised trait paths 10 > 11: the extra one is the stub `ArtifactDsl` (E0404) appearing in one more file because the unit test was split; stub noise. |
| rustc resolution differential (`+nightly-2026-07-20 --emit=metadata -o /dev/null`, sandbox with the original restored; `oracle.py <group> lib,test,p2`; staged and `C4P8_LIVE=1` runs, `c4p8-diff-stage1.log`, `c4p8-diff-live.log`) | 8 groups x lib, `--test`, `wasm32-wasip2` | errors orig = new in all 24 runs (for example recon 75/81/75, engine 28/270/28), only-new 0. Only difference: engine `--test` has 2 new `unused import: super::*` warnings, a sandbox artefact (the external glob `reconstruct::...::reconstruction::*` of the test root is neutralised to `const _`, so the children that import it see nothing); the children use `FrameAcceptance`, `IngestParams`, `EngineParams`, `EngineStatus`, `ReconstructionEngine` from it, so the glob is needed. The original has 1 such warning too. One real finding on the way was fixed before install (`dense_dump::encode` needed `pub(in super::super)`, E0603). |
| E0451 scan | `POS_PREFIX=c4p8 python3 rpv-tools/lit.py c4p8-tools/families/c4p8.json pre|post` | **8 sites before, the same 8 after, 0 new.** All 8 are `Intra4Neighbors { top, left, top_right, corner }` literals in the foreign `🎥️video/🧪️tests/🔬️unit/🦀️.rs` (lines 744 to 767) against the private struct of the h264 crate; that file also names the private `BitReader`, `predict_intra4x4`, `read_te`, so the test cannot compile today (typeck oracle: E0433 `cannot find type BitReader` on both sides). Not mine and not touched, see section 6. |
| private declarations used across files | `python3 defscan.py <spec>` x8 | 0 leftovers (55, 34, 70, 30, 41, 31, 21, 54 private declarations scanned) |
| rustfmt | `rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout --config skip_children=true` on 42 files (`c4p8-rustfmt.log`) | 42 of 42 parse, 0 failures |
| paths | `python3 c4p8.py paths <spec>` x8 | longest repo-relative path 233 UTF-8 bytes (<= 240), longest absolute path 185 characters (<= 256); absolute bytes reach 261 only for the `🧪️tests/🔬️unit/` children because the unit folder itself is already 224 bytes deep |
| duplicate mounts | `python3 ttc-tools/dupmounts.py '✏️s/🔌️plugins/📸️remodel/'` (`c4p8-dupmounts.log`) and a per-child check | 0 duplicate for any `*.rs` target; each of the 34 children is mounted exactly once and exists. The 48 reported duplicates are all `.../📦️packages/🦀️rust` directory targets of the `#[path = "."]` facade chains (untouched, same before). |
| folder names | `bun c4p5-tools/rgi.mjs` and `bun c4p1-tools/c4ptaxcheck.ts` on the 34 names; `bun c4p8-tools/tax.ts` (live `semanticDirectoryKindId`) | 34 of 34 RGI emoji + U+FE0F, 0 unresolved in the taxonomy (`crate-module-children`, and `members-of-tests` for the 5 test folders); no emoji repeated among siblings; all emoji are already used as folder prefixes in the repo (`folder-emojis.txt`) |
| census inside the gate | `c4p8-tools/census.sh before|after` (copy of the coordinator census with `WS_DIR` for the artifact workspace; canonical `readRustSourceClosureV1`; native, wasm32-unknown-unknown, wasm32-wasip2; lib and cfg(test)) | all 10 measured crates stay inside 32 files / 262,144 bytes / 4,000 active lines in every configuration, `overLimit` false. Example (native lib, before > after, files / bytes / lines): feature 2/68,766/1,315 > 5/69,964/1,360; dense 2/85,050/1,849 > 5/86,062/1,875; h264 2/119,835/2,744 > 8/122,432/2,759; motion 2/48,420/1,017 > 6/49,653/1,055; reconstruct 2/126,844/2,222 > 8/130,162/2,296; camera 2/48,611/1,066 > 5/49,492/1,089; sfm-geometry 2/86,641/1,815 > 6/88,039/1,833. cfg(test), engine: 5/123,079/2,163 > 10/124,741/2,209. Worst ratio after: 0.704 (sfm test closure, untouched), h264 0.690. `c4p8-census-before.txt`, `c4p8-census-after.txt`. |
| `new crates <pkg> --check-applied --base=f856dff0813` | `bun ./📜️script.ts new crates '✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/📦️packages/🦀️rust' --check-applied --base=f856dff0813 --sandbox=<scratch>` | CHECKAPPLIED_PLACEHOLDER |
| debug logs | `grep -c '\[DEBUG\]'` over the 42 files | none added |

## 6. Foreign errors and hand-offs

1. `🎥️video/🧪️tests/🔬️unit/🦀️.rs` (987 lines, not in package 8): its `H264PixelMathTests` and bit-reader tests use the h264 crate's private `BitReader`, `Intra4Neighbors`, `predict_intra4x4`, `read_te`. They do not compile (carve fallout) and are the 8 `lit.py` sites. Fix: move those tests into an h264 test companion (or make the items `#[doc(hidden)] pub`). I did not widen `Intra4Neighbors` alone because the file would still not compile.
2. The reconstruction unit test (hoisted into the engine facade) names `remodeling_image`, `remodeling_camera`, `remodeling_feature` that the reconstruct crate only imports privately; same on both sides of the oracle (pre-existing).
3. The staged peer changes in the artifact (`🧬️schema/🧬️mutations/🟦️.ts`, `💡️inferences/🟦️.ts`, editor `🦀️.rs`, `Cargo.lock`) are not mine.

## 7. Real-check commands for the compile queue (not run)

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --manifest-path '✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml' -p semio-s-artifact-remodel-remodeling-feature -p semio-s-artifact-remodel-remodeling-dense -p semio-s-artifact-remodel-remodeling-video-h264 -p semio-s-artifact-remodel-remodeling-motion -p semio-s-artifact-remodel-remodeling-reconstruct -p semio-s-artifact-remodel-remodeling-camera -p semio-s-artifact-remodel-remodeling-sfm-geometry -p semio-s-artifact-remodel-remodeling-engine --lib --keep-going --message-format=short
```
Run it again with `--tests` instead of `--lib`, and with `--target wasm32-wasip2` (lib). Hold the fleet compile mutex (`🗑️generated/cargo-mutex.lock`) for the call. Expected foreign errors: the section 6 items 1 and 2 under `--tests`.

## 8. New folders (34, each with a `🦀️.rs`; prefix `.../✏️editor/⚙️engine/`)

- `🌟️feature/🔎️detectors`
- `🌟️feature/🧬️descriptors-matching-and-flow`
- `🌟️feature/🌀️akaze-scale-space`
- `🌫️dense/🧩️patch-match-stereo`
- `🌫️dense/🧊️fusion-and-tsdf`
- `🌫️dense/☁️point-cloud-analysis`
- `🎥️video/🎞️h264/📖️bitstream-and-headers`
- `🎥️video/🎞️h264/🔤️cavlc`
- `🎥️video/🎞️h264/🧮️reconstruction`
- `🎥️video/🎞️h264/🧼️deblocking-and-output`
- `🎥️video/🎞️h264/🍰️slice-data`
- `🎥️video/🎞️h264/✍️encoder`
- `🏃️motion/🎯️tracking-and-trajectories`
- `🏃️motion/🎼️kinematics-modal-and-sync`
- `🏃️motion/🪞️camera-motion-compensation`
- `🏃️motion/🫧️non-rigid-and-pose`
- `🏭️reconstruction/📥️frame-ingest`
- `🏭️reconstruction/🎛️engine-vocabulary`
- `🏭️reconstruction/🏗️engine-core`
- `🏭️reconstruction/🔬️sparse-stages`
- `🏭️reconstruction/🧊️dense-and-meshing-stages`
- `🏭️reconstruction/👁️preview-and-products`
- `🏭️reconstruction/🧪️tests/🔬️unit/📥️ingest-gate`
- `🏭️reconstruction/🧪️tests/🔬️unit/🧊️synthetic-scene`
- `🏭️reconstruction/🧪️tests/🔬️unit/🔗️pipeline-contracts`
- `🏭️reconstruction/🧪️tests/🔬️unit/🔎️orbit-diagnostics`
- `🏭️reconstruction/🧪️tests/🔬️unit/♻️dense-replay`
- `📷️camera/🔭️intrinsics`
- `📷️camera/🧭️poses-and-rigs`
- `📷️camera/📐️calibration`
- `📸️sfm/📐️geometry/🧮️matrix-helpers`
- `📸️sfm/📐️geometry/🔀️two-view`
- `📸️sfm/📐️geometry/📍️triangulation-and-pnp`
- `📸️sfm/📐️geometry/🛤️tracks-global-and-loop-closure`

## 9. Files

Edited (8): the parent files of section 2. Created: the 34 children above. Tools: `F/c4p8-tools/` (`c4p8.py`, `widen2.py`, `rsitems.py`, `defscan.py`, `oracle.py`, `mkspec.py`, `members.py`, `globuse.py`, `testnames.py`, `census.ts`/`census.sh`, `tax.ts`, `run-oracle.sh`, `families/c4p8.json`, `specs/*.py`, `orig/*.rs.txt` pre-edit snapshots). Logs: `F/🗑️generated/c4p8-*`.
