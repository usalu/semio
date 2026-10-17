# Result CENSUS4-PKG-13 (prefix `c4p13`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `M` = `✏️s/🔨️modules/🏗️fem/⚙️engine`. Tools: `F/c4p13-tools/`. Logs: `F/🗑️generated/c4p13-*` (nothing deleted).

## 0. Verdict (kept current)

DONE. All 8 files are split and installed (25 children). Lossless, body-identity, structure and taxonomy gates pass. The typeck oracle (label `a3`) reports NEW 0 on lib and test. `--check-applied` for the fem 2d and 3d engines matches the baseline exactly. Nothing is pending.

## 1. Recovery after the second session restart (about 00:18)

- The earlier version of this file said "no source edit applied yet". That was stale: the install (`c13.py install`) had finished before the restart, so the tree was already edited.
- Re-checked against the tree:
  - all 8 target files differ from the pre-edit snapshots `F/🗑️generated/c4p13-orig/1..8.rs`, as expected, and each root is now 19 to 33 lines;
  - no half-applied edit: every child exists, every mount in the roots resolves, and no stray or empty child folder was left;
  - `git status` under `M` shows only these children and roots, plus a peer's staged TypeScript `📦️packages` additions, which are not mine.
- No rollback was needed. The oracle run `a2` was cut after its `it1 cur/test` line and wrote no report, so it was repeated as `a3`. The 2d `--check-applied` baseline had died with `EMFILE`, so it was run with `ulimit -n 65536`.

## 2. Package files and result

| # | Root (`M/`) | Lines before | Root after | Children (lines) |
| --- | --- | ---: | ---: | --- |
| 1 | `📏️elements2d/🦀️.rs` | 1,087 | 19 | `🪢️bars-and-beams` 408, `🔲️plane-continuum` 459, `🍽️plate-dkt` 222 |
| 2 | `🔢️sparse/🎵️subspace/🦀️.rs` | 2,340 | 33 | `🎼️modal-input` 256, `🌀️job-core` 693, `🚶️iteration-steps` 680, `♻️restore-cursor` 332, `🏃️run-and-publish` 385 |
| 3 | `🔢️sparse/🔁️pcg/🦀️.rs` | 1,126 | 26 | `🪜️job-and-steps` 486, `🌱️construction-and-restore` 467, `🏁️run-and-entry` 168 |
| 4 | `🔢️sparse/🔺️ldlt/🦀️.rs` | 1,132 | 23 | `🔩️factor-and-job` 674, `🔄️restore-cursor` 240, `🚀️run-and-close` 215 |
| 5 | `🔢️sparse/🦀️.rs` | 1,311 | 27 | `🤝️numerical-protocol` 396, `🗃️storage-and-ordering` 393, `📄️checkpoint-pages` 515 |
| 6 | `🕸️mesh/🦀️.rs` | 2,574 | 29 | `🗺️planar-domain` 257, `🔷️triangulation` 878, `🧵️mesh-job` 927, `🕹️mesh-job-run` 512 |
| 7 | `🧮️analyses/🧩️assembly/🦀️.rs` | 2,047 | 33 | `🧷️job-state-and-construction` 849, `🪡️element-build` 651, `🔶️csr-build-and-run` 537 |
| 8 | `🧱️elements3d/🦀️.rs` | 1,094 | 19 | `🌉️bars-and-frames` 460, `🧱️solids` 414, `🐚️shell-facet` 222 |

Every file is now under 1,000 lines (largest: `🧵️mesh-job` 927). Each child is mounted from its root with `#[path = "<child>/🦀️.rs"] mod <name>; pub use <name>::*;`, so the public API of every crate is unchanged. Private definitions that a sibling child names became `pub(super)` (widened: elements2d 3, elements3d 2, subspace 107, pcg 84, ldlt 51, sparse 6, mesh 155, assembly 125). Function bodies are byte-identical.

## 3. Verification (all run after the install, on the live tree)

| Gate | Command | Result |
| --- | --- | --- |
| Lossless | `python3 c4p13-tools/c13.py lossless live` | `LOSSLESS OK`: every removed line is accounted for as moved or widened; region markers balanced in all 8 |
| Bodies | `c13.py bodies live` | `BODIES OK`: fn items 100, 85, 73, 52, 41, 117, 87, 64 equal before and after; signature plus body multiset identical |
| Structure | `c13.py check` | `CHECKS OK`: no duplicate mount or module name, no missing file, test fn count equal, docstring emojis unique |
| Typeck oracle | `PYTHONHASHSEED=0 POS_PREFIX=c4p13 python3 pos-tools/pos.py run pos-tools/families/c4p13.json --cfgs lib,test --label a3` (log `c4p13-oracle-a3.log`) | lib cur 1744 = base 1744, NEW 0, GONE 0; test cur 4228 = base 4228, NEW 0, GONE 0; privacy, trait-scope, resolve, type, other all 0; canary E0308, E0616, E0624 reached on both sides |
| `--check-applied` 3d | `bun ./📜️script.ts new crates '<fem 3d engine dir>' --check-applied --base=HEAD` (log `c4p13-check-applied-3d-after.log`) | exit 0, crates 21; native 2262 = 2262, `--test` 944 = 944, `onlyCombined` 0; admission and shared empty (identical to the baseline) |
| `--check-applied` 2d | same for the 2d engine dir (log `c4p13-check-applied-2d-after.log`) | exit 0, crates 20; native 2490 = 2490, `--test` 1060 = 1060, `onlyCombined` 0; admission and shared empty |
| Source-boundary census | `bun c4p13-tools/census.ts` (canonical `readRustSourceClosureV1`; budget 32 files, 262,144 bytes, 4,000 lines) | all within budget, see below |
| Taxonomy admission | `bun frs-tools/taxcheck.ts 🗑️generated/c4p13-dirs.txt` | `checked=45 unresolved=0` (child folders are `crate-module-children`) |
| Path length | longest file under `M` | 101 chars relative, 129 absolute (limit 256) |

The `--check-applied` folder argument is the engine directory without `📦️packages/🦀️rust`; the full manifest path fails with `ENOENT`.

Census (files / bytes / active lines), before to after:

| Crate | lib | test |
| --- | --- | --- |
| elements2d | 1/42,278/1,088 to 4/42,916/1,112 | same |
| sparse-subspace | 1/96,669/2,256 to 6/98,935/2,300 | 4/148,971/3,051 to 9/151,237/3,095 |
| sparse-pcg | 1/52,976/1,125 to 4/54,574/1,149 | 5/135,116/1,957 to 8/136,714/1,981 |
| sparse-ldlt | 1/51,838/1,131 to 4/52,922/1,154 | 3/86,018/1,567 to 6/87,102/1,590 |
| sparse | 1/56,339/1,310 to 4/57,072/1,333 | 3/83,245/1,609 to 6/83,978/1,632 |
| mesh | 1/122,122/2,573 to 5/124,750/2,606 | 4/172,782/3,421 to 8/175,410/3,454 |
| analyses-assembly | 1/95,150/2,046 to 4/97,211/2,072 | 6/148,367/2,783 to 9/150,428/2,809 |
| elements3d | 1/48,128/1,095 to 4/48,748/1,119 | same |

The split adds only wiring, child docstrings and widened visibility (about 0.6 to 2 KB and 20 to 30 lines per crate); no closure is over budget before or after.

## 4. Consumers checked

- `✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts` and the `🎯️tool-job-coverage` law read the `sparse`, `mesh` and `elements2d` roots through `policyReadModuleTreeSource`, which follows the mounted children, so they still see the whole text. The `analyses` and `model` roots they read as single files are not part of this package.
- `POLICY_SPLIT_SOURCE_ROOTS` only lists plugin and renderer sources, so no registration is needed for these module crates.
- The 2d oracle JSON mentions `⚙️engine/📏️elements2d/🦀️.rs` and `⚙️engine/🔢️sparse/🦀️.rs` in prose only; no code path depends on them.

## 5. Limits and notes

- No cargo compile was run (the compiler ingress gate refuses raw cargo). Compile-level proof is the static `rustc --emit=metadata` differential above, which covers name resolution but not privacy, trait or coherence beyond the scans listed (all 0). A real `cargo check` is for the queued compile window.
- The `[DEBUG]` prints in `M/🕸️mesh/📐️surface/🧪️tests/🦀️.rs` belong to the existing surface tests, outside this split; I did not touch them.
- Root files keep their `use` lines; children reach them through `use super::*`, which is why the widened `pub(super)` set is name-driven and may over-approximate.

## 6. Files

- Edited (8 roots): the files in the table of section 2.
- Created (25 children): the folders in the table of section 2, each holding `🦀️.rs`.
- Tools, kept: `F/c4p13-tools/c13.py` (stage, lossless, bodies, check, install, restore), `meth.py`, `census.ts`; family `F/pos-tools/families/c4p13.json`.
- Logs and snapshots, kept in `F/🗑️generated/`: `c4p13-orig/`, `c4p13-installed.json` (install journal), `c4p13-census-before.txt`, `c4p13-census-after.txt`, `c4p13-dirs.txt`, oracle logs `a0` to `a3`, and the `--check-applied` logs before and after.
