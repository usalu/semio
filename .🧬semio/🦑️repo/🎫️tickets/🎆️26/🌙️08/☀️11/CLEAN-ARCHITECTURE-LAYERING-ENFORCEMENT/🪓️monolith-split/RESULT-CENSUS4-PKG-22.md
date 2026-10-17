# Result CENSUS4-PKG-22 (prefix `c4p22`): stdio artifacts (obj, gltf, ply) seven flat Rust files split into cohesive children

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. `S` = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts`. Tools: `F/c4p22-tools/`. Logs: `F/🗑️generated/c4p22-*`.

## 0. Verdict

**DONE (static; real compile queued).** Wave 7 (2026-10-17 07:27-08:45) verified the installed state, found and fixed one missed source-text reader (the gltf exports crate const `DIFF`), and ran every post-install gate on the live tree: losslessness, line sequence, rustfmt parse, census, region markers, test names, duplicate mounts, reachability, readers, facet equality (obj, ply, gltf), the scoped typeck differential on the live content (`POSDIFF IDENTICAL`, lib 3882=3882, test 4746=4746, canary reached, 0 non-stub privacy or trait-scope rows) and `--check-applied` for obj, gltf and ply (exit 0, `onlyCombined` 0 in every layout). 7 roots rewritten, 29 children created (largest child 829 lines, path length at most 225 bytes). No `COMPILE ADMITTED` line existed in `F/📓️FLEET.md` at 08:45, so the real `bun repo cargo` check is recorded for the compile queue in section 6.

## 1. Slice file list (`F/MONOLITH-CENSUS-4.md` section 4, Package 22; list also in `F/c4p22-tools/files.txt`)

| # | File (relative to `S`) | Lines |
|---|---|---|
| 1 | `🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/🔺️diff/🦀️.rs` | 1,496 |
| 2 | `🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔮️oracles/🦀️.rs` | 2,511 |
| 3 | `🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/💾️binary/🔺️diff/🧵️wire/🦀️.rs` | 1,178 |
| 4 | `🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/📝️text/🔺️diff/🧵️wire/🦀️.rs` | 1,323 |
| 5 | `🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🔨️geometry-kernel/🦀️.rs` | 1,049 |
| 6 | `🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🔺️diff/🦀️.rs` | 2,389 |
| 7 | `🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs` | 1,117 |

## 2. Found state (session 1, before any edit)

* No `RESULT-CENSUS4-PKG-22.md`, no `c4p22-tools`, no `c4p22-*` logs existed. Folder and tools created by this session.
* Live line counts equal the census (1,496 / 2,511 / 1,178 / 1,323 / 1,049 / 2,389 / 1,117). Existing git state of the target folders: obj `diff` has 2 staged-modified siblings (`🔣️.json`, `🟦️.ts`), ply `diff` has 1 (peer work on siblings, not my files).

* Baseline of the typeck oracle on the untouched tree (label `pre`, `F/🗑️generated/c4p22-pos-pre.log`): canary reached (`E0308, E0616, E0624`), 0 non-stub privacy rows, lib 3882 rows, test 4746 rows.
* Tool folder `F/c4p22-tools/` (copied from `c4p9-tools` / `c4p5-tools` and adapted: `plans.py`, `c4psplit.py`, `c4pinstall.py`, `c4plossless.py`, `c4pcheck.py`, `c4pparse.py`, `posrun.py`, `posdiff.py`, `injlib.py`, `c4p22.json`). Pre-edit byte copies of the 7 files: `F/🗑️generated/c4p22-orig/{objd,gor,gbw,gtw,ggk,gdf,plyd}.rs` (equal to `HEAD` `b84bf526c7b`, `git diff HEAD` clean for all 7). Stage dir (regenerable with `python3 c4p22-tools/c4psplit.py stage`): `scratchpad/c4p22-work/stage`.
* None of the 7 files is generated (all are handcrafted, header says so); no generator renders them.

## 3. Plan (boundaries are computed from item and region names of the pre-edit blob)

| key | file | root keeps | children |
| --- | --- | --- | --- |
| objd | obj `diff` | header, imports, tests, re-exports | `🧮️indexed-collections`, `🏷️named-collections-and-diff`, `🧵️codec-builders-and-demos` |
| gor | gltf oracle | header, tests | `🧰️glb-container-and-json-helpers`, `🎬️scene-node-and-collection-kinds` + `🎨️asset-and-material-resource-kinds` (region `Kinds`, 1,113 lines, repeated balanced in both), `🏗️structure-kinds`, `🧭️dispatch-and-projection` |
| gbw | gltf binary wire | imports | `🧱️scalar-and-enum-codecs`, `🧊️scene-mesh-and-accessor-codecs`, `🎨️material-and-resource-codecs`, `🎬️animation-and-camera-codecs`, `🔺️delta-diff-and-writer-codecs` |
| gtw | gltf text wire | imports | same five names with `print-codecs` for the last |
| ggk | gltf geometry kernel | header, aliases, imports, tests | `🧮️vector-math-and-part-decoding`, `🕸️pair-geometry-hull-and-topology`, `🧊️shell-metrics-and-geometry-context` (the original region `Kernel` was never closed; it is repeated balanced in each child) |
| gdf | gltf `diff` | header, `json_presence`, imports, the codec husk region (only comments and empty regions, it cannot be a child: a doc comment without item), tests | `🧮️collection-algebra` (`#[macro_use]`, defines `absorb_slot!`), `📐️structural-element-diffs`, `🎨️resource-element-diffs`, `🧊️gltf-diff-aggregate` |
| plyd | ply `diff` | header, tests, re-exports | `🧮️indexed-triple-algebra`, `📊️row-and-element-diffs`, `🧊️ply-diff-aggregate`, `🧵️codec-and-demo-cases` |

## 2b. Found state (wave 7, 2026-10-17 07:27)

* Installed state matches the journal: 7 roots plus 29 children on disk (`c4pcheck.py` `CHECKS OK`, `c4pseq.py` `SEQUENCE OK`, `c4plossless.py` `LOSSLESS OK`, `c4pparse.py live` `PARSE OK files=36`). Nothing was rolled back.
* The session-1 logs after 07:03 show the obj and ply descriptor `concat!` re-points were already applied (`c4p22-facetcheck.log`: 4 and 5 files, `equal:true`). The first `--check-applied` post run died with `ENFILE` (machine file table overflow, `c4p22-ca-post-obj.log`) and the wasm baseline (`prew`) died the same way, so neither produced a result.
* New finding in wave 7 (reader scan `c4p22-tools/readers.py`): the gltf diff facet is not a descriptor `include_str!` but the const `DIFF` in the exports crate (`🧬️schema/📤️exports/🦀️rust/📦️packages/🦀️rust/🦀️.rs`), which still read only the diff root. Re-pointed in wave 7 (see section 3b).

## 3b. Wave 7 change: gltf diff facet reader

* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📤️exports/🦀️rust/📦️packages/🦀️rust/🦀️.rs`: `pub const DIFF` was `include_str!` of the diff root only (the gltf schema descriptor takes its Rust diff facet from `semio_s_artifact_stdio_gltf_exports_rust::DIFF`). It is now `concat!(` root plus the four children `🧮️collection-algebra`, `📐️structural-element-diffs`, `🎨️resource-element-diffs`, `🧊️gltf-diff-aggregate)`. Same pattern as the obj and ply descriptors (installed in session 1). One consumer re-point, no other file touched.
* Proof: `c4p22-tools/facetcheck2.ts` (module-tree text of the diff root versus the concatenated facet): `{"files":5,"facetBytes":101886,"treeBytes":101886,"equal":true}` (`F/🗑️generated/c4p22-facetcheck-gltf.log`). obj `{"files":4,"facetBytes":74020,"equal":true}` and ply `{"files":5,"facetBytes":53518,"equal":true}` from session 1 (`c4p22-facetcheck.log`); the live descriptor lines were re-read at 07:29 and carry the `concat!` re-points.

## 4. Wave 7 gate results (live tree, real results)

| Gate | Command | Result |
| --- | --- | --- |
| live checks | `python3 c4p22-tools/c4pcheck.py` | `CHECKS OK`; 7 roots, 29 children; closure missing=[], duplicate-mounts={}, duplicate-first-emoji={}; region markers orig=new for all 7 (ggk and gor repeat the unclosed/balanced region as recorded); test fn names identical; MAXLINES 829, MAXBYTES 225 |
| line sequence | `python3 c4p22-tools/c4pseq.py` | `SEQUENCE OK` (objd 1303, gor 2378, gbw 1175, gtw 1320, ggk 1015, gdf 2040, plyd 950 lines identical in order) |
| losslessness | `python3 c4p22-tools/c4plossless.py` | `LOSSLESS OK removed=100 added=253` (only widened visibilities and `use`/`mod`/`pub use` wiring lines differ) |
| rustfmt parse | `python3 c4p22-tools/c4pparse.py live` | `PARSE OK files=36 bad=0` (`c4p22-parse-live.log`) |
| census | `bun c4p22-tools/census2.ts <7 roots>` | closures: 1465, 2552, 1223, 1368, 1077, 2187, 1088 active lines in 4, 6, 6, 6, 4, 5, 5 files, all `hard:true` (inside 4000 / 32 / 262144); `c4p22-census-w7.log`; pre-edit 1445 / 2510 / 1179 / 1324 / 1048 / 2385 / 1060 in 1 file each |
| readers | `python3 c4p22-tools/readers.py` | 31 hits over 3,559 files; the code readers are the descriptor `concat!`s (obj, ply), the exports `DIFF` (fixed above) and `#[path]` mounts of the roots (`📦️packages/🦀️rust` crates: obj diff `../../🦀️.rs`, gltf `../../🔺️diff/🦀️.rs`, ply and gltf inferences mounts of the diff and geometry-kernel roots, `🔮️oracles` wrapper), which mount the root and so follow its children; the rest are doc comments naming a root file |
| E0451 literal scan | `rpv` scan (session 1, `rpv-c4p22fin-e0451.json`) | `[]`, `0 candidate E0451 sites; 167 brace structs; 189 definitions` |
| typeck differential (live content, overlay of the 36 live files) | `C4P22_OVERLAY=<live copy> POS_PREFIX=c4p22 python3 c4p22-tools/posrun.py run c4p22-tools/c4p22.json --cfgs lib,test --label post3 --no-base`, then `posdiff.py pre post3` | lib rows pre=3882 post3=3882, only-pre=0, only-post3=0; test rows 4746=4746, 0/0; canary `E0308, E0616, E0624` reached; privacy 0, trait-scope 0 non-stub; `POSDIFF IDENTICAL` (`c4p22-pos-post3.log`, `c4p22-posdiff-post3.log`) |
| wasm differential | not run | none of the 7 pre-edit files has `wasm`/`target_arch`-gated code (`gdf` has one comment line mentioning wasm), so there is no wasm-only code path to move |
| `--check-applied` obj | `bun ./📜️script.ts new crates ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📦️packages/🦀️rust --check-applied --base=f856dff0813 --sandbox=<scratch>` | exit 0; crates 5; native 1027=1027, `--test` 1315 vs 1291 (equal to the pre-edit baseline in `c4p22-ca-pre-obj.log`), onlyCombined 0/0; `c4p22-ca-w7-obj.log` |
| `--check-applied` gltf | same with `🧊️gltf` | exit 0; crates 9 (baseCrates 9); native base 5048 vs combined 3051, `--test` 3248 vs 3245; onlyCombined 0/0, admission [], shared []; `c4p22-ca-w7-gltf.log` |
| `--check-applied` ply | same with `🧱️ply` | exit 0; crates 4; native 787=787, `--test` 990 vs 1000; onlyCombined 0/0; `c4p22-ca-w7-ply.log` |

The first attempt of the obj `--check-applied` hit the foreground 570 s cap (exit 142, no result) under machine load 27 to 94; it was re-run with a 2700 s alarm and finished (each artifact took about 10 to 14 minutes). The earlier ENFILE failures (session 1) no longer occur.

## 6. Compile queue (not run: no `COMPILE ADMITTED` line in `F/📓️FLEET.md`; the compile mutex was held by another slice)

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-s-artifact-stdio-obj-diff -p semio-s-artifact-stdio-gltf-test-oracle -p semio-s-artifact-stdio-gltf-model -p semio-s-artifact-stdio-gltf-inferences -p semio-s-artifact-stdio-ply-inferences -p semio-s-artifact-stdio-gltf-exports-rust --lib --tests --keep-going --message-format=short
```
(Run it once under the mutex. The artifact-level schema descriptor crates for obj, gltf and ply also embed the re-pointed facets, so include their crates in the same call if the compile queue lists them.)

## 7. Foreign errors

None observed in this slice's gates. Foreign churn seen in the same folders (not mine, left alone): staged peer edits to `Cargo.lock`, `package.json`, `📸️snapshot/🟦️.ts`, the gltf `🏭️generator` recipe split, editor `🦀️.rs` files and the `🔺️diff` `🟦️.ts`/`🔣️.json` siblings. `git` prints `error: daemon terminated` on each call (fsmonitor), harmless.

## 8. Files touched in wave 7

* Edited: the gltf exports crate file named in section 3b.
* Added (tools and logs only): `F/c4p22-tools/readers.py`, `F/c4p22-tools/facetcheck2.ts`, logs `F/🗑️generated/c4p22-{ca-w7-obj,ca-w7-gltf,ca-w7-ply,facetcheck-gltf,parse-live,census-w7,pos-post3,posdiff-post3}.log` plus the `c4p22-c4p22-post3-*` oracle outputs.
