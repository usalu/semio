# Result: CRATE-ROLLOUT-R8C (prefix `r8c`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r8c-tools/` (`step.sh <tag> <cmd...>`, `r8c-census.ts`). Logs: `F/🗑️generated/r8c-*`. Sandboxes: `/Users/ueli/.cache/r8c/`. Started 2026-10-17 09:31.

## 0. Verdict (kept current)

| Package (`✏️s/🔌️plugins/…/🗿️artifacts/…`) | Verdict | Notes |
| --- | --- | --- |
| mathematical equation (`➗️mathematical/➗️equation`) | DONE (static) | 9 producer crates plus facade; all inside the gate; `--check-applied` exit 0, 0/0 extras |
| dag dag (`🕸️dag/🕸️dag`) | DONE (static) | 8 producer crates plus facade; all inside the gate (lib and test); `--check-applied` exit 0, 0/0 extras. `::graph::manifest` does not occur in the tree (`git grep`), nothing to fix |
| reasoning wires (`💡️reasoning/🔌️wires`) | DONE (static) | 8 producer crates plus facade; all inside the gate; `--check-applied` exit 0, 0/0 extras |

## 1. Baseline census (cgc7 reader, host cfg, `F/🗑️generated/r8c-census-base0.tsv`)

| crate | lib files/bytes/lines | test closure files/bytes/lines |
| --- | --- | --- |
| semio-s-artifact-mathematical-equation | 169 / 841,766 / 14,789 | 311 / 1,322,131 / 20,551 |
| semio-s-artifact-dag-dag | 112 / 317,308 / 5,041 | 153 / 487,570 / 7,370 |
| semio-s-artifact-reasoning-wires | 99 / 331,671 / 4,830 | 139 / 534,989 / 7,190 |

Precondition: all three artifacts only carry staged (index) changes of earlier slices; no unstaged or untracked files.

## 2. Per package

### 2.1 dag dag (`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag`, 09:40 to 10:35)

- Plan (`r8c-dag-plan0.log`): `problem: moved root items DagContentChild … genesis_dag_child_pack cannot be hosted without a crate cycle`. Cause: the root regions `ContentBridge` and `Scene` formed one strongly connected unit that is referenced by `schema::snapshot` (below) and itself names `DagSnapshot`, `DagMutation` and the demo example (above).
- Source change (root `🦀️.rs`, minimal): `DagScene` moved into the `ContentBridge` region; the `Scene` region split into `Scene` (`dag_scene_from_children`, `dag_scene`), `ChildEmit` (`dag_child_emit`, needs `DagMutation`) and `DerivableScene` (`dag_derivable_scene`, `genesis_dag_child_pack`, called by the inferences crate). Plan then has no problems (8 crates).
- Generator bug 1 (fixed, test `sees the aliases that a use statement inside a function body binds`): `aliasChains` only read top-level `use` items, so the `use crate::…::io::export::serializers::artifacts as export;` inside `fn io()` produced no edge from the schema crate to the lowest crate that holds the inline `io::export`/`io::import` leaves; the carved layout then reported `E0433 cannot find export in io`. It now reads every `use` statement.
- Generator bug 2 (fixed, test `retargets the absolute self path of a hoisted test to its new mount`): a hoisted test (`io/sqlite/snapshot/tests`) kept `use crate::…::sqlite::snapshot::tests::*` naming its own former mount, which no longer exists after the hoist (`E0432`, and the union stopped before late resolution). The hoist now rewrites the former absolute path in the hoisted file subtree to `crate::hoisted_…`.
- Source change 2: facade unit test `🧪️tests/🔬️unit/🦀️.rs` named `SemioGraphSnapshot` through `use super::*` (a private root import that moved with the bridge items); it now imports it explicitly.
- `--verify --oracle` (`r8c-dag-verify4.log`, exit 0): census within the hard budget, trees equal, visibility 0, orphan 0, glue 0, unresolved crates 0; oracle native 1028 vs 1028, `--test` 1533 vs 1518 (`onlyCandidate` 0); layout `onlyCombined` 0 / 0.
- Apply (`r8c-dag-apply.log`, exit 0, backup `F/🗑️generated/r8c-backup-dag*`): 80 files, 8 crates, 8 owner rows. `--check-applied` (`r8c-dag-check.log`): exit 0, native base 1028 / combined 1035 / extra 0, test 1532 / 1525 / 0, admission, shared, sharedForeign empty.
- Repo checks (`r8c-dag-checks.log`): both launch JSONs parse; owners scan 2710 packages / 2838 owners, no dag row missing; taxonomy `dag`: mounts 1, tree 1 (plugin root `dag` has no manifest, existing); `lib-wiring-budget`: the dag root is 236 lines, still pinned (701), no dag finding; the 2 failing tests list only foreign roots (jpg 527 to 600, forms 984 to 995, stale pins of semio graph and drawing).
- Census after (`r8c-census-dag-after.tsv`, lib / test files/bytes/lines):

| crate | lib | test |
| --- | --- | --- |
| semio-s-artifact-dag-dag (facade) | 1/11,469/236 | 5/38,549/593 |
| -inferences | 27/66,418/1,024 | 29/70,121/1,089 |
| -config | 14/22,843/449 | 19/27,826/568 |
| -diff | 27/46,319/720 | 28/47,231/749 |
| -tests | 2/3,431/75 | 7/13,598/259 |
| -schema | 27/56,267/779 | 28/57,004/794 |
| -document-behavior-tests | 14/43,061/808 | 20/57,323/1,081 |
| -dag (editor) | 9/103,338/1,609 | 17/200,470/2,988 |
| -v1 | 3/10,155/178 | 3/10,155/178 |

Before: 112/317,308/5,041 lib, 153/487,570/7,370 test. No crate is over any limit.

## 2.2 reasoning wires (`✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires`)

- Plan 0 (`r8c-wires-plan0.log`): schema family cycle of 4 groups (48 files) and an editor cycle (35 files, test 266,271 bytes). Causes found with `F3_SLICES=1`: (a) root-item region closure: `WiresContentChild` shared region `ContentBridge` with `genesis_wires_child_pack(&WiresSnapshot)`; (b) the text snapshot codec globbed the schema root (`use crate::…::schema::*;`, unused).
- Source changes: root `🦀️.rs` new region `ContentChild` (alias plus its `pub use` of the graph types) split off `ContentBridge`; the unused glob removed from `🚪️io/📝️text/📸️snapshot/🦀️.rs`; editor `unit_tests` and `unit_tests::context` widened `pub(crate)` to `pub` (cfg(test) support, see §3 rule); window_transient decoupled from the editor root: its `WINDOW_KIND_ID` now names `…windows::canvas::WIRES_PLAY_WINDOW_CANVAS` (the owner) and its two io codecs import the public `editor::wires::window_transient::*` instead of the private `component::window_transient::{mutations,*}` path. This moved window_transient (10 files) into its own crate and brought the editor crate test closure from 266,922 to 248,000 bytes.
- Generator work: `mergeFileChains` now looks at every file decl of a slice as the outer module (it only used attach decls), which merges the sandwich `canvas::config` (file) / `config::io` (file, other slice) / `io::*::mutations` (same slice as config) into one crate; before, the viewer crate mirror defined `canvas` twice (E0428) and `io` was missing under `config`.
- `--verify --oracle` (`r8c-wires-verify4.log`) exit 0: oracle native 1076/1076, `--test` 1804/1809 (`onlyCandidate` 0); layout 0/0. Apply (`r8c-wires-apply.log`): 81 files, 9 crates. `--check-applied` (`r8c-wires-check.log`) exit 0: native 1064/1092 extra 0, test 1785/1825 extra 0.
- Repo checks: launch JSONs parse; owners scan 2833 packages / 2957 owners, no wires row missing; taxonomy `reasoning`: mounts 1, tree 1 (plugin root has no manifest, existing); `lib-wiring-budget` fails only for foreign roots (svg, tiff, forms, brep mesh, stale pins of semio graph and drawing).
- Census after (`r8c-census-wires-after.tsv`, lib / test files/bytes/lines): facade 2/7,854/175 / 7/54,106/521; snapshot 11/65,562/383 / same; inferences 24/36,783/486 / 26/38,764/543; schema 25/61,771/899 / 28/65,198/963; snapshot-2 (terminology tests) 6/7,673/180 / 7/8,185/192; windows 11/19,853/395 / 11/19,853/406; window-transient 10/18,412/395 / 10/18,412/396; viewer 6/25,457/500 / 12/34,326/643; wires (editor) 14/111,150/1,869 / 21/247,977/3,706; v1 3/10,263/188 / 4/12,648/216. Before: 99/331,671/4,830 lib, 139/534,989/7,190 test.

## 2.3 mathematical equation (`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation`)

- Plan 0 (`r8c-eq-plan0.log`): a 27-group cycle of 178 files (575,014 bytes, 7,852 lines). Causes: (a) root region `Document` held the pure types together with `pub use crate::editor::…::graph::config::EquationCamera;` and `pub use crate::snapshot::schema::{…}`, so the region closure tied every user of `EquationGraph` to the editor graph window and the snapshot; (b) nested region `ChildTypes` lived inside `Composition`; (c) the mutation aggregate reached `standards::v1::subsets::graph::…` through `use crate::standards::v1::subsets;`, which the planner resolved to the owner of the prefix `standards::v1` (a slice that sits above the editor).
- Source changes (root `🦀️.rs`): new top-level regions `CameraType` and `CarrierTypes` (the two `pub use` lines), `ChildTypes` moved out of `Composition`. Eight cas-internals files (`sums`, `integrate`, `polybridge`, `ode`, `rootof`, `matrix`, `solve`, `simplify`) named `crate::polynomial::{PolyU, Monomial, MonomialOrder, PolyM, AlgebraicReal, isolate_real_roots, factor_integer_poly}`, i.e. items of the root-level file module `polynomial`, which the facade keeps; they now use the child module paths (`polynomial::univariate::PolyU`, `multivariate::{…}`, `algebraic::AlgebraicReal`, `roots::isolate_real_roots`, `factor::factor_integer_poly`). The aggregator keeps its `pub use` list (public API unchanged). The oracle reported only the expression paths (E0433); the `use` lines were hidden by the unlinked-cascade filter (`dropUnlinkedCascades`), so this was checked by hand: no `crate::cas::<item>` or `crate::polynomial::<item>` path remains.
- Generator work: `ownerOfReachedChain` (below), cfg(test) coherence (below), hoist retarget of private-import paths (below).
- `--verify --oracle` (`r8c-eq-verify3.log`) exit 0: oracle native 1340/1357, `--test` 2618/2639 (`onlyCandidate` 0); layout 0/0 (before the fixes the layout showed `E0433 cannot find polynomial` and the oracle `E0603 enum import EquationNodeKind is private`). Apply (`r8c-eq-apply.log`): 128 files, 9 crates. `--check-applied` (`r8c-eq-check.log`) exit 0: native 1340/1362 extra 0, test 2612/2644 extra 0.
- Repo checks: launch JSONs parse; owners scan no mathematical row missing; taxonomy `mathematical`: mounts 1, tree 1 (plugin root has no manifest, existing); `lib-wiring-budget`: no finding for the equation root (431 lines, pinned at 1047).
- Census after (`r8c-census-eq-after.tsv`, lib / test): facade 3/24,171/431 / 9/69,571/897; examples 18/88,707/2,188 / 26/107,303/2,691; schema 24/141,783/931 / 27/153,670/1,204; snapshot 20/180,010/3,307 / 28/196,729/3,751; inferences 24/75,549/1,352 / 29/92,086/1,654; mutations 16/85,241/1,411 / 26/227,762/3,288; leaf-tests 10/37,793/585 / 16/79,582/1,143; create-node 23/84,647/1,866 / 27/99,847/2,242; update-graph-algorithm 25/59,951/1,323 / 29/69,728/1,521; editor 22/147,131/2,394 / 25/217,155/3,396. Before: 169/841,766/14,789 lib, 311/1,322,131/20,551 test. Largest test closures: snapshot 3,751 lines (94 percent of 4,000), editor 217,155 bytes.

## 3. Generator changes (all in `L/🏗️authoring/📦️crate-tree/`, each with a test in `L/🧪️tests/📦️crate-tree/🟦️.ts`; 64 tests pass)

- `🗺️plan/🟦️.ts` `aliasChains`: collects `use` aliases from every statement, including ones inside function bodies (`sees the aliases that a use statement inside a function body binds`).
- `🗺️plan/🟦️.ts` `ownerOfReachedChain` (scan): a path whose next segment is an inline wrapper module that the prefix owner does not own no longer makes an edge to that owner (`does not make a slice depend on the owner of a prefix when the path names an inline wrapper module that the owner does not own`; fails without the change).
- `🗺️plan/🟦️.ts` `mergeFileChains`: outer module taken from all file decls of a slice, not only attach decls (covered by the wires carve; no separate unit test).
- `🗺️plan/🟦️.ts` `restrictedTarget`: a path into a `#[cfg(test)]` module of another slice is a coherence coupling, so the user stays in the crate of that module (`keeps a slice in the crate of the cfg(test) module it reaches, because a dependency never compiles that module`). This is the coordinator's rule on cross-crate cfg(test) items.
- `🧪️test-closure/🟦️.ts` hoist: the hoisted test's absolute path to its former mount is rewritten to the new `hoisted_…` module (`retargets the absolute self path of a hoisted test to its new mount`); paths through a private import of the host (`crate::host::Name`) become the carried bare name (`names a private import of the host by its carried import in a hoisted test instead of through the host path`).
- `🧪️test-closure/🟦️.ts` detach: private host imports are also carried into nested modules that glob the host (`carries the private imports of the host into a nested module of a detached test that globs the host`); `absolutizeSuperPaths` takes the physical module path so a private `component` module counts as one `super` level (`counts the private component module of a host as one super level …`).
- Tools: `F/r8c-tools/` (`step.sh`, `r8c-census.ts`, `checks.sh`, `plan-dump.ts`, `cfgtest-check.ts`). Debug traces added during the work were removed; no `[DEBUG]` remains in the plan module.

## 4. Cross-crate cfg(test) items (coordinator rule)

- `F/r8c-tools/cfgtest-check.ts <artifact> <crate-prefix>` lists names of `#[cfg(test)]` modules, functions, structs, consts and types defined in one produced crate that files of another produced crate mention. Result for the final trees: dag 0 findings (the 3 `law_tests`/`mutation_vectors` hits are same-name modules of each file), wires 0, equation 0. No `#[cfg(test)] pub use` of a sibling producer crate exists in any generated root or mirror.
- Found and removed in wires: the detached tests (canvas, window_transient, reorganize, delete_selection) landed in the facade crate while `editor::wires::unit_tests` (cfg(test) support) lives in the editor crate. The generator now keeps such users in the owner's crate; the editor crate was brought under the gate by decoupling window_transient (§2.2), not by a feature or cfg(any(test, …)).

## 5. Open items / hand-offs

- No real compile was run (raw cargo is refused; the closure of these crates includes other oversized crates). The oracle's cascade filter hides unresolved `use` lines of kept root modules (see §2.3); other plugin rollouts with kept root file modules (`pub mod x;` at the artifact root) should be checked for `crate::<kept>::<item>` paths.
- A peer's concurrent edits to `🗺️plan/🟦️.ts` were merged by re-applying my own small edits to the file as it was; one overwrite from a minutes-old copy happened once (restored my own edits, no peer diff was detected afterwards).
- dag: `use ::graph::manifest` does not occur anywhere in the tree (git grep), nothing to fix.
