# Result c6inf (census 6, packages P32 to P35: rust os/infinite, 13 files)

Executor Sonnet 5.5, foreground, no sub-agent, no modifying git command, no cargo, bun repo, nx or bazel. Tools: `C/c6inf-tools/` (descendants of `c6flow-tools`; scripts re-pointed to `/Users/ueli/.cache/c6inf`; `specs/*.py` per file, `specs5.py` for the two test roots, `post.sh <tag>` runs install plus every live oracle). Snapshots of the 13 originals: `c6inf-tools/orig/`. Sandboxes and stage trees: `/Users/ueli/.cache/c6inf/`. The peer auto-commit already captured the split.

## 0. Verdict

**DONE (static) for all 13 files.** 13 files (28,796 lines) became 13 roots (20 to 105 lines) and 138 children in 138 new folders. The largest child is 498 lines (`eng` icon-paint-cache). No crate had to be carved: the worst closure is `semio-framework-os-infinite-board-engine` (16 files, 176,157 bytes, 3,453 active lines, ratio 0.863 of 4,000). The real `bun repo cargo` check is still needed (section 7).

| tag | file (under `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/`) | before | root | children | largest child | mechanism |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| eng | `⚙️engine/🦀️.rs` | 3,307 | 105 | 15 | 498 | `#[path]` modules, the 1,238-line `impl GraphEngine` cut into 4 impls (3 declared cuts) |
| nhost | `🔌️ports/➡️directed/➕️normal/🧭️host/🦀️.rs` | 2,796 | 60 | 10 | 460 | `#[path]` modules, one `impl BoardHost` cut in 2 (1 cut), inert region markers |
| fill | `…/➕️normal/🪣️fill/🦀️.rs` | 3,144 | 74 | 12 | 457 | `#[path]` modules, the 746-line `impl BoardFillJob` cut into 4 impls (3 cuts) |
| focus | `…/➕️normal/🎯️focus/🦀️.rs` | 2,132 | 61 | 9 | 393 | `#[path]` modules, inert region markers |
| scene | `…/➕️normal/🖼️scene/🦀️.rs` | 2,406 | 76 | 11 | 451 | `#[path]` modules |
| nmodel | `…/➕️normal/🗃️model/🦀️.rs` | 2,063 | 78 | 11 | 483 | `#[path]` modules |
| schema | `🧬️schema/🦀️.rs` | 2,548 | 86 | 16 | 304 | `#[path]` modules |
| gest | `…/➕️normal/🕹️gestures/🦀️.rs` | 1,599 | 58 | 9 | 285 | `#[path]` modules, inert region markers |
| bht | `…/➕️normal/🧭️host/🧪️tests/🔬️board-host/🦀️.rs` | 1,644 | 26 | 5 | 410 | `include!` fragments (50 tests before and after) |
| graph | `…/🕸️dag/🕸️graph/🦀️.rs` | 1,987 | 67 | 11 | 309 | `#[path]` modules, one `impl DagGraphState` cut in 2 (1 cut), inert region markers |
| dmodel | `…/🕸️dag/🗃️model/🦀️.rs` | 1,943 | 102 | 13 | 344 | `#[path]` modules, inert region markers |
| dhost | `…/🕸️dag/🧭️host/🦀️.rs` | 1,765 | 78 | 10 | 271 | `#[path]` modules, inert region markers |
| dut | `…/🕸️dag/🧭️host/🧪️tests/🔬️dag-unit/🦀️.rs` | 1,462 | 20 | 6 | 326 | `include!` fragments (62 tests before and after) |

No function above 1,000 lines exists in the package (the longest is `step_delete_planning`, about 245 lines). No compatibility re-export, alias or glob re-export: every root re-exports its children with explicit `pub use child::{names}` / `pub(crate) use` / `use` lists (sorted the way rustfmt sorts them, wrapped at its 250-column limit), children begin with `use super::*;` except four children that use nothing from the root (`glob=False`: nmodel `port-error`, schema `layout-options` and `layout-progress-error-and-dag-options`, dmodel `dag-error-and-port-side`). A `pub(crate)` or private name is only re-exported when another file of the split reads it as a bare identifier (so no `unused import` warning appears).

## 1. Tool changes made on the way (all in `c6inf-tools/modsplit.py`)

- `inert_regions`: the originals of nhost, focus, gest, graph, dmodel and dhost carry region markers that are unbalanced or cross item boundaries (for example `//#region` inside an `impl`, closed after another item). The parser asserted on them. With the spec flag the markers are neutralised while splitting and restored afterwards, so every marker line moves with its item unchanged (marker totals per file are checked by `vmod.py` against the original).
- `cfg_mod`: nhost `legacy-pointer-differential` holds only `#[cfg(any(test, feature = "testkit"))]` items; its `mod` and re-export lines carry the same cfg, otherwise `use super::*;` is unused in a plain lib build.
- Re-export emitter: skips `_` names, drops unneeded crate-visible re-exports, sorts like rustfmt, normalises cfg attribute spacing, uses the repo's `max_width = 250`. Result: the rustfmt hunk multiset of the new files equals the original's for eng, focus, scene, nmodel, gest, graph, dmodel and dhost; nhost differs only because widened visibility makes a few signatures longer than 250 columns, fill only by one region marker line, schema only by blank-line placement.
- Oracle: `oracle5.py` groups for the 13 tags; `census-flow.ts` and `files.tsv` re-pointed to the infinite module.

## 2. Evidence (all run on the live tree after installing)

| Check | Tool | Result |
| --- | --- | --- |
| lossless line multiset (modules) | `modsplit.py lossless <tag> <live dir>` x11 | `LOSSLESS OK`, `missing 0`; extras are only wiring, docstrings, re-export lists, cfg lines and declared cut lines (`}` and impl header) |
| lossless (fragments) | `chk.py live bht dut` | `CHK OK`: `gone=0 extra(non-marker)=0`, spliced sequence equal, include literals equal, test fn multiset equal (50 and 62), docstring emoji unique, rustfmt parses every file |
| structure | `vmod.py live` x11 | `VMOD OK`: each child mounted once, region totals equal the original, brace depth 0, path length (repo-relative at most 229 bytes, absolute at most 190 chars), no `[DEBUG]`, rustfmt parse, module names do not clash, emoji distinct from the root and from sibling folders |
| cross-file privacy | `defscan.py`, `ambig.py`, `modclash.py` | 0 leftovers except schema 6, all false positives (trait methods `to_value`, `from_value` of one-line `impl Trait for X{` blocks that start in column 0); 0 ambiguous glob candidates; `modclash OK` |
| resolution differential | `oracle5.py <tag>` (rustc `+nightly-2026-07-20 --emit=metadata`, original restored in sandbox A against the live tree in sandbox B, lib and `--test`, `C5B_NORM=1`) | identical error and warning multisets for 10 files (eng 202/202, nhost 39/39 and 42/42, fill 240/240, focus 16/16, scene 63/63 and 65/65, nmodel 50/50 and 58/58, schema 289/289 and 290/290, gest 34/34, bht 16/16 and 179/179, dut 10/10 and 278/278). Graph, dmodel and dhost: new has one error fewer (111 vs 112, 210 vs 211, 100 vs 101): the root `use graph::…` E0432 disappears because rustc reports the same missing crate `graph` once, now at the first use in a child as E0433. Warnings identical |
| folder names | `alldirs.py` + `rgi.mjs` | 138 folders, all RGI emoji, `bad=0`, every emoji already used as a folder prefix in the repo |
| source gate | `census-flow.ts` (canonical `readRustSourceClosureV1`, lib and `--test`) | section 3 |
| TS text readers | `bun test` and Bun emulations | section 4 |

The oracle is a resolution-level differential with external crates missing. It proves the mount structure and crate-local name resolution, not field or method privacy and not types (rustc stops before typeck on the missing externs).

## 3. Source gate (files / bytes / active lines of the worst closure; limits 32 / 262,144 / 4,000)

| crate | before | after |
| --- | --- | --- |
| `semio-framework-os-infinite-board-engine` | 1 / 168,489 / 3,304 (0.826) | 16 / 176,157 / 3,453 (0.863) |
| `…-board-schema` | 1 / 136,754 / 2,546 (0.636) | 17 / 143,979 / 2,675 (0.669) |
| `…-board-normal-model` | 1 / 86,242 / 2,064 (0.516) | 12 / 92,110 / 2,166 (0.541) |
| `…-board-normal-host` lib / test | 1 / 157,109 / 2,797 (0.699) | 11 / 160,720 / 2,879 (0.720) |
| `…-board-normal-host` board_host test | 1 / 113,922 / 1,514 (0.435) | 6 / 115,293 / 1,530 (0.440) |
| `…-board-normal-scene` | 1 / 114,027 / 2,407 (0.602) | 12 / 119,226 / 2,504 (0.626) |
| `…-board-normal-focus` | 1 / 98,461 / 2,133 (0.533) | 10 / 102,863 / 2,213 (0.553) |
| `…-board-normal-gestures` | 1 / 74,886 / 1,600 (0.400) | 10 / 79,005 / 1,676 (0.419) |
| `…-board-normal-fill` | 1 / 155,644 / 3,145 (0.786) | 13 / 163,024 / 3,276 (0.819) |
| `…-board-dag-model` | 1 / 83,424 / 1,941 (0.485) | 14 / 92,108 / 2,078 (0.519) |
| `…-board-dag-graph` | 1 / 104,875 / 1,988 (0.497) | 12 / 109,566 / 2,078 (0.519) |
| `…-board-dag-host` lib / dag_unit test | 1 / 106,571 / 1,602; 1 / 77,772 / 1,463 | 11 / 110,561 / 1,689 (0.422); 7 / 79,380 / 1,482 (0.370) |

All crates stay inside the gate; the tightest (engine) has 547 active lines of headroom.

## 4. Text readers

| Reader | What it read | Action |
| --- | --- | --- |
| `🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts` | engine root (`retirement_scene: Cell<…>`, `advance_opaque_scene_retirement`) and the `board-host` test root (slice from `fn board_world_scene_retirement_retains_exact_token_until_backing_is_released`) | **edited**: both now `readModuleTreeSource` (the gestures read already was one). The file needs `fast-json-stable-stringify`, which is not installed, so it cannot run; a Bun emulation of the three board assertions over the new trees passes |
| `📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts` | schema root, two needles (`pub fn is_draggable_body…`, `self.node_id.is_some() && !self.is_screen_path()`) now in the `dag-views-traits-and-facts` child | **edited**: `readModuleTreeSource`. `bun test`: 11 pass, 1 fail; the failure is the pre-existing stale anchor `self.dag.take_graph_edits()` in the flow host (reported by c6flow, unrelated) |
| `…/🔗️node-graph-wire-edit`, `🧪️node-graph-edit-rows`, `🫳️node-graph-gestures` (dag part) | dag model, graph and host roots | **no edit**: already `readModuleTreeSource`; `bun test` wire-edit 10 pass; edit-rows cannot load (`@xyflow/react/dist/style.css` missing), its needles (`sink.text("operation", …)`) were verified with a Bun emulation |
| `🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts`, `✅️verify/🔬️interactivity-lanes` | dag roots through `policyReadRustPolicySource` / `policyReadModuleTreeSource` | **no edit**: the dag roots and all six normal roots are already listed in `POLICY_SPLIT_SOURCE_ROOTS` (tree reads). The self test aborts earlier on an unrelated node-graph source (`scene_retained_tests is not canonical`), and its `dag-weak-terminal-witness` needle does not exist in the unsplit dag text either |
| `📚️library/🔍️discovery/📖️source-access/🟦️.ts` | list of split roots | **no edit**: engine, schema and the two test roots are read by no policy law |
| `🎯️dag-input/🧪️tests/🧫️fixtures/receivers.json`, `🌊️flow/🧪️tests/🧫️fixtures/🔌️port-types/🔣️.json` | path strings in data | **no edit**: no reader found (no file name or loader references them) |
| Rust, Go, Python, scripts, `📋️build-owners.json`, `🔒️dependencies.json` | | none reads these files as text (only comments mention paths) |

## 5. Open risks

- **No real compile yet.** Field and method privacy (E0616, E0624) and types are not covered by the oracle. Mitigation: `widen2` widened 263 private declarations that another file of the split reads (named by identifier, including format-string captures and `"path"` attribute strings), `defscan` shows 0 true leftovers. The first `cargo check` is the proof.
- **wasm-gated code.** `dhost` holds `DagSession` and `DagSessionInner` behind `cfg(all(target_arch = "wasm32", not(target_env = "p2")))` (child `wasm-session-and-input-application`), plus the `future_to_promise` import in the root. Native cargo never compiles them: check `semio-framework-os-infinite-board-dag-host` also for the wasm target (`--target wasm32-unknown-unknown`).
- **`testkit` feature.** `nhost` `legacy-pointer-differential`, `plan_delete_selection`, `board_property_retirement_credits` (nmodel) and the bht/dut tests are cfg'd on `test`/`testkit`: check with `--features testkit` too.
- **Impl re-opening.** `eng`, `fill`, `nhost`, `graph` cut one large `impl` into several `impl` blocks of the same header (declared cuts, `}` and header lines). Methods and associated functions that only another cut used got `pub(super)`.
- **Region markers.** The six inert-region files keep their original (broken) marker lines verbatim, now spread over children; editors may fold oddly. I did not repair them (lossless rule).
- **Formatting debt** is unchanged except the widened signatures noted in section 1.
- Peers (Codex) may still edit these roots; install refuses a root that differs from its snapshot, so re-run `modsplit.py stage|reinstall <tag>` if a peer touched one before the check.

## 6. Files changed

New folders and `🦀️.rs` children under the 13 file directories (138), the 13 roots, and the readers `…/🧪️source-contract/🟦️.ts` and `…/🫳️node-graph-gestures/🟦️.ts`. Nothing else in the repo.

## 7. Crates to compile

`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests` (and once with `--features testkit`) for each of:

`semio-framework-os-infinite-board-engine`, `semio-framework-os-infinite-board-schema`, `semio-framework-os-infinite-board-normal-model`, `semio-framework-os-infinite-board-normal-scene`, `semio-framework-os-infinite-board-normal-focus`, `semio-framework-os-infinite-board-normal-gestures`, `semio-framework-os-infinite-board-normal-fill`, `semio-framework-os-infinite-board-normal-host` (test `board_host`), `semio-framework-os-infinite-board-dag-model`, `semio-framework-os-infinite-board-dag-graph`, `semio-framework-os-infinite-board-dag-host` (test `dag_unit`, plus the wasm32 target).

Consumers re-export the same names, so no consumer edit is expected; compile `semio-framework-os-infinite-board-dag-host` and `…-normal-host` consumers (the world/renderer crates) only if the above passes and a dependent check is wanted.
