# Result c6flow (census 6, packages P30 and P31: rust os/flow)

Executor Sonnet 5.5, foreground, no sub-agent, no modifying git command, no cargo or build. Tools: `C/c6flow-tools/` (descendants of `c6db1-tools`/`c5b-tools`; `specs/*.py` per file, `specs5.py` for the two flat test roots, `census-flow.ts` new). Snapshots of the 11 originals: `c6flow-tools/orig/`. Sandboxes and stage trees: `/Users/ueli/.cache/c6flow/`. Nothing was generated inside the ticket besides this file and the tools.

## 0. Verdict

**DONE (static) for all 11 files of P30 and P31.** 11 files (18,098 lines) became 11 roots (17 to 167 lines) and 83 children in 83 new folders. The largest child is 453 lines (`actb` 📤). No crate had to be carved: every owning crate stays inside the gate (worst ratio 0.884, `semio-framework-os-flow-host-core`, active lines 3,537 of 4,000). The real `bun repo cargo` check is still needed (section 6).

| tag | file (under `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/`) | before | root | children | largest child | mechanism |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| vcs | `🌿️vcs/🦀️.rs` | 1,916 | 167 | 8 | 451 | `#[path]` modules, one 740-line impl cut into 5 impls (4 declared cuts), explicit re-exports |
| draw | `🖍️drawing/🦀️.rs` | 1,105 | 75 | 6 | 263 | `#[path]` modules |
| evs | `🖥️host/⏱️eval-session/🦀️.rs` | 1,078 | 24 | 7 | 228 | one 1,074-line `impl FlowEvalSession` cut into 7 impls (6 cuts) |
| hauth | `🖥️host/✏️host-authoring/🦀️.rs` | 1,253 | 27 | 8 | 262 | one 1,249-line `impl FlowHost` cut into 8 impls (7 cuts) |
| hhist | `🖥️host/🌳️host-evaluation-history/🦀️.rs` | 1,356 | 22 | 6 | 305 | one 1,351-line `impl FlowHost` cut into 6 impls (5 cuts) |
| snap | `🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs` | 1,149 | 86 | 6 | 277 | `#[path]` modules |
| xwasm | `🧩️extensions/🕸️wasm/🦀️.rs` | 1,098 | 77 | 7 | 305 | `#[path]` modules |
| vt | `🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs` | 2,004 | 17 | 10 | 308 | `include!` fragments (35 tests before and after) |
| actb | `🕸️wasm/🎞️actions-b/🦀️.rs` | 2,018 | 52 | 6 | 453 | `#[path]` modules (actions 2561 to 2610 by theme) |
| acta | `🕸️wasm/🎬️actions-a/🦀️.rs` | 2,044 | 54 | 6 | 441 | `#[path]` modules (actions 2504 to 2560 by theme) |
| ht | `🖥️host/🧪️tests/🔬️unit/🦀️.rs` | 3,077 | 27 | 13 | 352 | `include!` fragments (128 tests before and after) |

No function above 1,000 lines exists in the package (the longest, `FlowAction*`/`collapse_selection`, are under 160 lines). No compatibility re-export, alias or glob re-export: every root re-exports its children with explicit `pub use child::{names}` / `use child::{names}` lists, children begin with `use super::*;` (the same as census 5).

Children (folder, lines) are listed by `ls` of each directory; docstring emoji equals the folder emoji, is unique among siblings and different from the root's (`vmod.py`, `chk.py`).

## 1. One defect fixed on the way (declared, not lossless by design)

`🕸️wasm/🎬️actions-a/🦀️.rs` is a crate root (`[lib] path = "../../🦀️.rs"`), but declared `pub(super) fn dag_input_failure`. rustc rejects that (`E0433 too many leading super keywords within crate`, reproduced with the oracle and a one-line probe), a leftover of the crate carve. The only changed line is `pub(super)` to `pub(crate)` (declared in the spec: `patch`, `expect_extra`, `expect_missing`); the function is read by `🧵️dag-input` through the root glob. The oracle shows the error only on the original side (734 versus 733 errors).

## 2. Evidence (all run on the live tree after installing)

| Check | Tool | Result |
| --- | --- | --- |
| lossless line multiset (modules) | `modsplit.py lossless <tag> <live dir>` x9 | `LOSSLESS OK` x9, `missing 0`; extras are only wiring, docstrings, region markers, re-export lists, declared cuts (`}` and impl header lines) and the one declared acta line |
| lossless (fragments) | `chk.py live vt ht` | `CHK OK`: `gone=0 extra(non-marker)=0`, spliced sequence equal, include literals equal and existing, regions balanced, test fn multiset equal (35 and 128), docstring emoji unique, rustfmt parses every file |
| structure | `vmod.py live <9 tags>` | `VMOD OK`: each child mounted once, region balance, brace depth 0, path length (repo-relative at most 240 bytes, absolute at most 256 chars), no `[DEBUG]`, rustfmt parse, child module names do not clash |
| cross-file privacy | `defscan.py`, `ambig.py`, `modclash.py` | 0 cross-file private leftovers, 0 ambiguous glob candidates, `modclash OK` for all 9 |
| resolution differential | `oracle5.py <group>` (rustc `+nightly-2026-07-20 --emit=metadata`, original restored in sandbox A against live tree in sandbox B, lib and `--test`) | identical error multisets in all 11 comparisons (vcs 77/117, draw 209/228, evs 219/219, hcore 332/332, snap 266/353, xwasm 284/337, acta 734 vs 733 (the fixed defect), actb 701/701, vt 1/731, ht 1/28). Warnings: identical except two oracle artifacts: `unused import super::*` in `🪟️surface-owner-action-and-cursor` (its names come from the root's external glob, which the oracle neutralises) and `unused import optional_path_segments` in the drawing root (used only by a `serde(with = "…")` string that the oracle cannot expand because serde is missing) |
| paths and names | `alldirs.py` + `rgi.mjs` | 83 new folders, all RGI emoji, `bad=0`, every emoji already used as a folder prefix in the repo; longest repo-relative path far below 240 bytes |
| source gate | `census-flow.ts` (canonical `readRustSourceClosureV1`, lib and `--test`) | see section 4 |

The oracle is a resolution-level differential with external crates missing (it proves the mount structure and crate-local name resolution), not a type check.

## 3. Text readers re-pointed

| Reader | What it read | Action |
| --- | --- | --- |
| `🌊️flow/🌐️geometry/🧪️tests/🎟️caller/🟦️.ts` | `🕸️wasm/🎞️actions-b/🦀️.rs`, sliced `struct FlowAction2599` to `struct FlowAction2600` | **edited**: `sliceModuleTreeSource` (the 8 assertions were checked against the new child with a script: all hold) |
| `🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement/🟦️.ts` | `🧩️extensions/🕸️wasm/🦀️.rs`: four slices and one whole-file negative | **edited**: `readModuleTreeSource` for the whole text, `sliceModuleTreeSource` per slice (all 20 assertions checked against the new tree with a script: all hold) |
| `🧪️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts` | host-authoring and host-evaluation-history roots | **edited**: `readModuleTreeSource` (the same two assertions already fail on the unsplit text: anchors `take_graph_edits_json` body and `commit_gesture_history` are stale; equality before and after was checked) |
| `🧪️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🧪️tests/🤏️pinch-wheel/🟦️.ts` | host-authoring root (a regex over the wheel factors) | **edited**: `readModuleTreeSource` plus its import (regex still matches) |
| `✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🏷️ownership/🟦️.ts` | negative assertion over `🖥️host/🧪️tests/🔬️unit/🦀️.rs` | **edited**: `readModuleTreeSource` plus its import, so the negative law keeps covering the 13 fragments |
| `🌊️flow/🕸️wasm/🧪️tests/🔬️component-domain-laws/🦀️.rs` (Rust, `production_source`) | roots of actions-a and actions-b (negative laws over the production text) | **edited**: the list now names the 12 children as well |
| `🌊️flow/📦️packages/🦀️rust/📜️script.ts` | vcs root, needle `P:crate::forms::FlowFormProducer` | **no edit**: `forms_bridge` stays in the vcs root |
| `📏️retirement/🟦️.ts` (`readModuleTreeSource` of the vcs root) | `initial_page_birth_demand`, `admit_initial_page` | **no edit**: already a module tree reader (checked: both names are in the children) |
| port-sides fixture, `receivers.json`, `ownership/compute` fixture, comments in renderer twins | path strings in prose or unused data | **no edit**: no reader (checked by grep of the file names) |

The TS tests could not be executed here (no `node_modules`; `ajv` and the workspace packages are unresolved without the dependency image), so they were verified by re-running their assertions with a Python emulation over the module tree.

## 4. Source gate (files / bytes / active lines, ratio of the worst of 32 / 262,144 / 4,000)

| crate | closure | before | after |
| --- | --- | --- | --- |
| `semio-framework-os-flow-vcs` | lib / test | 3 / 93,218 / 1,976 (0.494); 6 / 103,036 / 2,092 | 11 / 97,985 / 2,084 (0.521); 14 / 107,803 / 2,200 (0.550) |
| `semio-framework-os-flow-vcs-laws` | test | 2 / 134,446 / 2,024 (0.513) | 12 / 137,121 / 2,055 (0.523) |
| `semio-framework-os-flow-drawing` | lib / test | 1 / 48,835 / 1,102 (0.276); 4 / 66,703 / 1,542 | 7 / 51,515 / 1,163 (0.291); 10 / 69,383 / 1,603 (0.401) |
| `semio-framework-os-flow-host-session` | lib | 13 / 216,747 / 2,571 (0.827) | 20 / 219,101 / 2,632 (0.836) |
| `semio-framework-os-flow-host-core` | lib / test | 10 / 199,651 / 3,405 (0.851); 3,415 | 24 / 204,142 / 3,527 (0.882); 3,537 (0.884) |
| `semio-framework-os-flow-host` | test | 8 / 193,043 / 3,088 (0.772) | 21 / 196,554 / 3,127 (0.782) |
| `semio-framework-artifact-flow-flow-model` | lib / test | 7 / 86,502 / 1,824 (0.456); 16 / 107,245 / 2,038 | 13 / 90,096 / 1,911 (0.478); 22 / 110,839 / 2,125 (0.688) |
| `semio-framework-os-flow-extensions` | lib / test | 5 / 205,819 / 1,537 (0.785); 6 / 206,201 / 1,621 | 12 / 211,545 / 1,664 (0.807); 13 / 211,927 / 1,748 (0.808) |
| `semio-framework-os-flow-wasm-actions-a` | lib | 2 / 110,806 / 2,028 (0.507) | 8 / 113,620 / 2,087 (0.522) |
| `semio-framework-os-flow-wasm-actions-b` | lib | 1 / 97,465 / 1,875 (0.469) | 7 / 100,213 / 1,940 (0.485) |

`semio-framework-os-flow-host-core` is the tightest crate (460 active lines of headroom); all others have at least 11% headroom.

## 5. Open risks

- **No real compile yet.** The differential has external crates neutralised. The residual risks are the ones census 5 documented: a private inherent method or field read across the new module boundary that the name scan missed (the scans reported 0 leftovers), and `use super::*` imports of names that only come from `use … as _` (drawing root: `Engine as _`, a trait whose associated const `ENGINE_ID` is read by two children; a glob import of an underscore import is legal and is what diesel-style preludes rely on).
- `impl` re-opening: vcs, evs, hauth, hhist cut one huge `impl` into several `impl` blocks of the same header (declared cuts; region markers that crossed a cut were moved to the nearest item boundary, hhist `History` end marker moved after the closing brace; ht moved the dangling `PortSides` region pair before its orphan doc comments, otherwise the orphan `///` lines ended a fragment).
- `pub(super)` handling: `nest` is on for evs, hauth, hhist, snap, xwasm (originals became `pub(in super::super)` in the child, which equals the old reach). vcs, draw, acta, actb have no `pub(super)` besides the fixed defect.
- The live taxonomy resolver (`library/🔍️discovery/🟦️.ts`, used by `tax.ts` in census 5) no longer exists in the tree (the library moved to Go), so folder names were checked only by the RGI test and the used-as-folder-prefix test. They follow the exact pattern of the 119 folders census 5 resolved.
- One renderer TS law (`node-graph-gestures`, gesture journal) fails on stale anchors before and after the split; not touched.

## 6. Crates to compile

`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests` for each of:

`semio-framework-os-flow-vcs`, `semio-framework-os-flow-vcs-laws`, `semio-framework-os-flow-drawing`, `semio-framework-os-flow-host-session`, `semio-framework-os-flow-host-core`, `semio-framework-os-flow-host`, `semio-framework-artifact-flow-flow-model`, `semio-framework-os-flow-extensions`, `semio-framework-os-flow-wasm-actions-a`, `semio-framework-os-flow-wasm-actions-b`, and the consumer `semio-framework-os-flow-wasm` (it glob-imports both action crates and its domain-laws test now reads the children).
