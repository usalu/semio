# Result c6plug2 (census 6 packages P47, P48, P54, P55, P58)

Executor Sonnet 5.5, foreground, no sub-agents, no cargo / `bun repo` / nx / bazel, no modifying git command, no ticket tool. `C` = this folder. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `C/c6plug2-tools/` (descendant of `c6db1-tools`, `c6plug-tools` and `cg1-tools/pos`; snapshots `orig/<tag>.rs`, specs `specs/<tag>.py`, chunk tables `specs5.py`, tag table `files.tsv`, crate table `crates.json`). Sandboxes and stages: `/Users/ueli/.cache/c6plug2/`. Nothing was compiled.

## 0. Verdict

**DONE for 16 of 16 files (static evidence); 15 of 16 are now under 1,000 lines everywhere, one child stays at 1,108 lines because it is a single function.**

- `🧩️extensions/📐️brep/🦀️.rs` (`fb`, P48): the 2,479-line file became a 370-line root and four operator children of 235 to 375 lines. The fifth child `🧾️register-operators` is 1,108 lines because `pub fn register` is one function of 1,088 lines. Per the brief a function above 1,000 lines is reported, not rewritten.
- `cg2` is carving `semio-s-artifact-flow-flow` (the schema crate). The two P48 files are in `semio-s-artifact-flow-flow-editor` and `semio-s-plugin-flow-brep`, so no P48 file was skipped.
- Mechanism: 13 module splits (`modsplit.py`, `#[path]` children, explicit re-export lists, no root glob, no `pub use child::*`) and 3 `include!` fragment splits for flat test roots (`frag.py`: `pbu`, `put`, `pbh`; the fragments keep module identity, nothing is widened). Children are `<emoji>️<kebab>/🦀️.rs` next to their root; 90 new folders.
- 29,000 lines in 16 files became 16 roots (12 to 370 lines) and 90 children (96 to 1,108 lines, all others at most 600).

## 1. What was cut

| Pkg | Tag | Before | Root after | Children | Largest child |
| --- | --- | ---: | ---: | ---: | ---: |
| P47 | `fm` fem 2d model window | 1,255 | 78 | 4 | 572 |
| P47 | `fe` fem 2d editor | 1,584 | 201 | 5 | 600 |
| P47 | `fs` fem 2d session | 2,664 | 95 | 8 | 523 |
| P48 | `fl` flow editor | 2,749 | 273 | 9 | 493 |
| P48 | `fb` flow brep extension | 2,479 | 370 | 5 | **1,108 (`register`)** |
| P54 | `pf` puzzle 2d precompute fill | 1,876 | 66 | 6 | 471 |
| P54 | `pbu` puzzle 2d brush unit tests (fragments, inner `mod tests`) | 1,681 | 17 | 6 | 379 |
| P54 | `ps2` puzzle 2d snapshot | 1,715 | 118 | 7 | 424 |
| P54 | `pe` puzzle 2d editor | 1,155 | 152 | 3 | 456 |
| P54 | `put` puzzle 2d editor unit tests (fragments) | 1,564 | 141 | 7 | 310 |
| P54 | `ps3` puzzle 3d snapshot (crate lib root) | 1,921 | 126 | 7 | 344 |
| P55 | `pbh` puzzle 2d board-host unit tests (fragments) | 1,026 | 12 | 3 | 403 |
| P55 | `pr` puzzle 2d retained | 1,284 | 47 | 3 | 498 |
| P55 | `p5` puzzle 5d editor | 1,412 | 211 | 3 | 523 |
| P58 | `wj` wfc engine job | 2,182 | 73 | 8 | 406 |
| P58 | `we` wfc 2d editor | 1,183 | 98 | 4 | 443 |

Owning crates (lib roots): `fm` `semio-s-artifact-fem-2d-editor-model-window`, `fe` `-fem-2d-editor`, `fs` `-fem-2d-editor-session`, `fl` `semio-s-artifact-flow-flow-editor`, `fb` `semio-s-plugin-flow-brep`, `pf` `semio-s-artifact-puzzle-2d-tests`, `pbu` `-puzzle-2d-connect-handles-preparation`, `ps2` `-puzzle-2d-mut-any-10`, `pe` `-puzzle-2d-v1`, `put` `-puzzle-2d-unit-tests`, `ps3` `-puzzle-3d-editor-snapshot`, `pbh` `-puzzle-2d-board-host`, `pr` `-puzzle-2d-retained`, `p5` `-puzzle-5d-puzzle5d`, `wj` `semio-s-plugin-wfc-engine-job`, `we` `semio-s-artifact-wfc-2d-v1`.

Decisions

- Impl re-opening (declared pre-edit, same header repeated, lossless modulo `}` + header): `fs` `impl MountedModelBuild` (1 cut) and `impl MountedState` (2 cuts), `wj` `impl<T: Topology + Clone> WfcJob<T>` (2 cuts; the header occurs twice, the cut table uses line numbers).
- Statics and mounts stay in the root: `fs` `thread_local! MOUNTED`, the `app_commands!` enums of `fe`/`fl`, the `mod` mounts, the inline `artifact_items_N` modules and every `extern crate` alias.
- Macros: `fb` keeps the five operator macros in the root (textual scope reaches the children declared after them) and the `boolean_operation` macro moves with its invocations. Their structs are macro-generated, so the macro bodies got the minimal visibility edit `struct $name(SessionCapture)` to `pub(super) struct $name(pub(super) SessionCapture)` (6 lines, declared) and the root imports the generated names (`use <child>::{BoxPrim, ...}`, 114 names). `ps2`/`ps3` move `puzzle{2,3}d_command_variants!` with its invocation and impl into `🧵️command-variants` and re-export the macro-declared enum with one explicit `pub use command_variants::Puzzle{2,3}dCommand;`.
- Orphan region markers: `ps2` had `//#region 🧵️RetainedCommands` without an end (an `//#endregion` is added before `//#region 🧩️EditorContract`); `pe` had an `//#endregion 🧵️RetainedCommands` without an opener that closed `PlayApp` by name mismatch (`//#region 🧵️RetainedCommands` added before `PUZZLE2D_RETAINED_PAYLOAD_SCHEMA`). Both are one-line marker repairs (declared extras); region counts balance.
- `wj`: `🧭️protocol-and-determinism` names nothing of its parent (the sandbox reported the unused glob), so it has no `use super::*;`.
- `pr` was first cut into 5 children and re-cut into 3 because the crate closure of `semio-s-artifact-puzzle-2d-retained` reached 32 of 32 files (ratio 1.000). Now 29 lib / 30 test files (0.906 / 0.938).

## 2. Evidence (all on the live tree after the install)

| Check | Result |
| --- | --- |
| Lossless (`modsplit.py lossless`) | `LOSSLESS OK` x13: `missing 0`, every extra line is wiring, a child docstring, `use super::*;`, a declared impl-split pair, a declared macro visibility line or a declared marker; region counts balance (`ps2`, `pe` against the repaired text) |
| Fragments (`chk.py live pbu put pbh`) | `CHK OK`: `gone=0 extra(non-marker)=0`, sequence equality, `#[test]` multiset 29 to 29, 67 to 67, 36 to 36 |
| Structure (`vmod.py live` for the 13) | `VMOD OK`: each child mounted once, docstring emoji = folder emoji, unique, different from the root's, no sibling clash, paths (<= 240 bytes), brace depth, rustfmt parse, no `[DEBUG]` |
| Cross-file private leftovers (`defscan.py`), glob ambiguity (`ambig.py`), module-name clash (`modclash.py`) | 0 leftovers, `AMBIG OK`, `modclash OK` for all 13 |
| Taxonomy | `rgi.mjs` `bad=0` for 90 names; `tax.ts` (live `semanticDirectoryKindId`, path moved to `🔍️discovery/🔐️taxonomy-authority`) resolves 90 of 90 dirs |
| Scoped patched differential (`oracle_c6p2.py`, `rustc +nightly-2026-07-20 --emit=metadata` on the real crate lib root, original restored in a sandbox, cfgs lib, test, lib+testkit, test+testkit) | error multisets equal in every group and cfg: fm 179/212, fe 585/679, fs 66/92, fl 845/859, fb 1876 (labels only: the same 54 + 3 `E0425` with an added "not found in this scope" label), pf 218/366, pbu 42/571, ps2 228/797, pe 403/576, put 30/1110, ps3 257/265, pbh 4/91, pr 379/381, p5 560/621, wj 30/65, we 405/485. Warnings equal except two sandbox artifacts: `fm` `unused import MESH_EDGE_COLOR` (use sits inside the `json!` macro that the sandbox cannot expand) and `fl` `unused import flow_direct_store_contract` (use sits inside `bounded_first_step_tool_proofs!`) |
| Typeck/privacy oracle (`pos`, `c6plug2-tools/pos`, union sandbox of all 16 crates with a stub layer outside, cfgs lib and test, absolute classification, log `/Users/ueli/.cache/c6plug2/pos-all.out`) | typeck canary (E0308, E0616, E0624) reached; **privacy 0 and trait-scope 0 non-stub rows in lib and in test**; the non-stub resolve rows (84 lib, 307 test) are unresolved imports of crates outside the family and `json!`/`bounded_first_step_tool_proofs!` macros of the stub layer, none names a split child; no E0603/E0616/E0624/E0451. No pre-split baseline was run (15 of the 16 crates changed in HEAD since the snapshot or are peer-edited, so a HEAD baseline would be noise); the oracle5 differential above is the before/after proof |
| Source-boundary gate (`census-mine.ts`, canonical `readRustSourceClosureV1`, real crate roots), lib / test ratio of 32 files, 262,144 bytes, 4,000 lines | fm 0.394/0.414, fe 0.683/0.875, fs 0.703/0.751, fl 0.787/0.802, fb 0.775/0.775, pf 0.789/0.910, pbu 0.157/0.859, ps2 0.514/0.910, pe 0.610/0.778, put 0.031/0.889, ps3 0.506/0.510, pbh 0.063/0.371, pr 0.906/0.938, p5 0.799/0.895, wj 0.662/0.815, we 0.727/0.797; all inside the gate |
| Formatting debt (`fmt.py`) | roots grow 2 to 4 rustfmt hunks from the generated `use child::{...}` lists (long single-line lists); parse failures none; no other kind |

## 3. Source-text readers re-pointed

| Reader | Change |
| --- | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` | `puzzle5d`, `fem2dSession`, `fem2dEditor`, `fem2dModel` now use `policyReadModuleTreeSource` (were `policyReadFileSafe` of a root) |
| `✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts` | `fem2dModel`, `fem2dSession` (two sites) use `policyReadModuleTreeSource` |
| `…/🧵️session/🧪️tests/🔬️unit/🦀️.rs` (fem) | two `include_str!("../../🦀️.rs")` became `session_source()`: root plus the 8 children, `[…].concat()` |
| flow editor TS tests: `🎮️commands/♻️retirement/🧪️tests/🟦️.ts` (1 site), `🧪️tests/🔬️source-contract/🟦️.ts` (4 sites), `🧪️tests/🎟️contributions/🟦️.ts` (1 site) | root text became `readModuleTreeSource(...)`; imports added. The negative vocabulary regexes, the receiver slice and the `FlowHostEffectPayload` match were re-proved over the module tree with the python twin `modtree.py` (all hold) |
| `…/🧩️puzzle/🧪️tests/🧫️fixtures/🔏️publication-authority/🔣️.json` | `mounted` lists extended: Puzzle2dPlayApp +3 retained children +3 editor children, Puzzle5dPlayApp +3 children (children before the root, as the `UnitTests` cut requires). The audit was ported to python (`pubaudit.py`): 5d passes before and after; 2d fails identically before and after (the retained tool id table already lives in the snapshot file, not in the editor) |
| `flow/🗿️artifacts/🌊️flow/🧪️tests/🔬️unit/🦀️.rs` | `EDITOR_SOURCE` reads root plus 9 children |
| `put` fragment `🧰️history-acceptance-source-laws-and-camera-helper` (`editor_source()`) | the `fixture_text!` list now includes the 3 retained children and the 3 editor children |
| `🧰️framework/…/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json` | pin of `ps3` (1922) removed (root now 127 lines <= budget 150), via `ratchet.py` |
| Not changed on purpose | `fb` readers (`📐️tessellate/🧪️tests/🟦️.ts` reads `extension_guest`, which stays in the root); comment-only mentions; `flow/…/📦️packages/🦀️rust/📜️script.ts:21` (syntax-only check of the root) |

## 4. Open risks

- `fb` `🧾️register-operators` is 1,108 lines (single function `register`, 1,088 lines). Not split by design.
- `flow/🗿️artifacts/🌊️flow/🧪️tests/🔬️unit/🦀️.rs` (crate `semio-s-artifact-flow-flow`, the crate `cg2` is carving) read the flow editor root with `fixture_text!` for the `artifact_schema:` tool-proof lines; they now live in children. The one law `editor_tool_proofs_declare_the_artifact_document_schema` was re-pointed to root plus the 9 children (`LazyLock<String>` of a `[…].concat()`). If `cg2` moves that test file, carry this change along. Add `semio-s-artifact-flow-flow` (`--tests`) to the compile list.
- The TS flow retirement test cannot load (missing module `…/♻️retirement/🧪️tests/🟦️.ts` import), a failure that predates this slice; the Bun workspace aliases (`@semio-tech/*`) did not resolve in this session, so no TS law could be executed. The re-pointing is proved with the python twin only.
- `ps3` is a crate lib root: the 7 children are mounted from the lib root; `pub` API names are re-exported explicitly (`pub use`), `Puzzle3dCommand` explicitly by hand.
- Pre-existing, unchanged: the 2d publication audit failure above; unbalanced region markers were repaired in `ps2` and `pe` only.

## 5. Crates to compile

`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests` (add `--features testkit` for the fem window and the editors with `cfg(any(test, feature = "testkit"))`):

`semio-s-artifact-fem-2d-editor-model-window`, `semio-s-artifact-fem-2d-editor`, `semio-s-artifact-fem-2d-editor-session`, `semio-s-artifact-flow-flow-editor`, `semio-s-plugin-flow-brep`, `semio-s-artifact-puzzle-2d-tests`, `semio-s-artifact-puzzle-2d-connect-handles-preparation`, `semio-s-artifact-puzzle-2d-mut-any-10`, `semio-s-artifact-puzzle-2d-v1`, `semio-s-artifact-puzzle-2d-unit-tests`, `semio-s-artifact-puzzle-3d-editor-snapshot`, `semio-s-artifact-puzzle-2d-board-host`, `semio-s-artifact-puzzle-2d-retained`, `semio-s-artifact-puzzle-5d-puzzle5d`, `semio-s-plugin-wfc-engine-job`, `semio-s-artifact-wfc-2d-v1`, plus `semio-s-artifact-flow-flow` (`--tests`, the re-pointed flow editor law).

Downstream crates that import from these (public API is unchanged, names re-exported): the play/hub compositions of fem, flow, puzzle and wfc; check them in the same window.

## 6. Files

- Edited roots (16): the files of the table in section 1. Created: 90 children (`<emoji>️<kebab>/🦀️.rs`), listed in `c6plug2-tools/newdirs.txt`.
- Edited readers: section 3.
- Tools kept in `c6plug2-tools/`; logs in `/Users/ueli/.cache/c6plug2/` (outside the repo). No `[DEBUG]` log was added.
