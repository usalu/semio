# Result: CRATE-ROLLOUT-R7 (prefix `r7`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r7-tools/` (`step.sh <tag> <cmd...>`, `r7-census.ts`). Logs: `F/🗑️generated/r7-*`. Sandboxes: `/Users/ueli/.cache/r7/`. Started 2026-10-17 09:15.

## 0. Verdict (kept current)

| Package (`✏️s/🔌️plugins/…`) | Verdict | Notes |
| --- | --- | --- |
| trinity jack | BLOCKED | see 5.1: facade hub (5,424 lines) not carvable by the generator |
| trinity rewriting | BLOCKED | see 5.2 |
| writer writer | BLOCKED | see 5.2 |
| sourcing curation | BLOCKED | see 5.2 |
| vcs vcs | DONE | 7 crates + facade, all inside the gate (lib and test), `--check-applied` exit 0, 0 extras |
| space home | DONE | 7 crates + facade (plus the pre-existing mutation-bridge), `--check-applied` exit 0, 0 extras, no source change needed |
| sequence sequence | BLOCKED | see 5.3 |
| flow brep (extension) | DONE | facade + mesh + tests crates (hand carve, generator used as oracle), `--check-applied` exit 0 with 0 extras |

Order: jack, rewriting, writer, then the independent ones.

## 1. Baseline census (cgc7 reader, host cfg, scoped copy `r7-tools/r7-census.ts`, `F/🗑️generated/r7-census-base0.tsv`)

| crate | lib files/bytes/lines | test closure files/bytes/lines |
| --- | --- | --- |
| trinity-jack | 153 / 723,284 / 12,295 | 192 / 895,728 / 14,694 |
| trinity-rewriting | 165 / 500,210 / 7,112 | 269 / 961,044 / 11,040 |
| writer-writer | 155 / 428,307 / 7,483 | 251 / 783,792 / 11,705 |
| sourcing-curation | 128 / 491,278 / 6,943 | 187 / 754,618 / 10,000 |
| vcs-vcs | 128 / 321,118 / 4,833 | 202 / 524,993 / 7,574 |
| space-home | 112 / 369,320 / 5,467 | 154 / 575,897 / 8,134 |
| sequence-sequence | 93 / 465,539 / 7,657 | 135 / 637,390 / 9,837 |
| flow-brep | 7 / 303,720 / 4,040 (OVER bytes) | refused: `Missing Rust source input … 📐️brep/🔣️.json` |

Already inside the gate: trinity-jack-graph-query 4/10,501/167, trinity-jack-shell 1/4,608/126, space-home-mutation-bridge 1/2,315/34, flow-brep-component 1/300/4.

Precondition note: every artifact has staged (index) changes from earlier slices (file splits, package Cargo.toml, editor roots); this is the fleet baseline.

## 2. Per package

### 2.1 vcs vcs (DONE)

Plan fit at once (exit 0), but `--verify --oracle` failed (20 candidate-only test diagnostics, 25 layout extras). Hand source changes (all scripted with sed/python, each re-verified):

1. Editor children of the c4p file split (`✏️editor/🕹️command-surface-and-job-factories`, `✍️edit-command-work`, `📬️store-preparation-and-proofs`): `pub(super)` widened to `pub` (the detached unit tests live in another crate), root globs `use edit_command_work::*`/`use store_preparation_and_proofs::*` became `pub use`.
2. `✏️editor/🧪️tests/🔬️unit/🦀️.rs`: inline `mod context` used `use super::super::*`; changed to `use super::*` (the test detach rewrites `super::super` into the wrapper path, which hides private host imports; `super::*` reaches the carried imports of the file scope).
3. `🚪️io/🦀️.rs` and the operations unit test: the function-local `use …::export::serializers::artifacts as export;` / `import` aliases hid the io-root -> serializer-leaf edges from the planner (csv/xlsx leaves were placed in the tests crate while the io declaration in schema named them: a cycle). Replaced with full paths.
4. `🏅️standards/…/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`: `use crate::VcsSnapshot` instead of the host's private import.
5. `central_apply`: the module at the artifact root was named by the six mutation fixture tests that live in lower crates (E0432 in the layout differential). Moved the `pub mod central_apply` mount from the artifact root to the mutation aggregate (`🧬️schema/🧬️mutations/🦀️.rs`, same pattern as generation2d/3d), updated the six tests and `🧪️tests/🌿️mutate-vcs-1` to `…::mutations::central_apply`, `🌉️apply/🦀️.rs` uses `super::VcsDemoMutation`.

Commands: `r7-vcs-verify6` exit 0 (oracle native 0/0, test 0/0; layout 0/0), `r7-vcs-apply` exit 0 (82 files, 7 crates, owner rows registered), `r7-vcs-check` (`--check-applied`) exit 0, onlyCombined 0 for lib and test. Owners scan: `missing` has 14 framework paths of other slices, none under vcs. Taxonomy `pw0-taxcheck.ts`: one pre-existing plugin-root finding (missing manifest at plugin root). `lib-wiring-budget`: the two failures are `stdio/jpg` (R5A) and `forms`, plus a stale pin on stdio semio graph (R4); none is vcs. Both launch JSON files parse.

Census after (`r7-census-vcs-after.tsv`, lib / test files/bytes/lines): inferences 28/60,540/939 / 29/61,169/948; viewer 22/39,301/608 / 25/41,247/650; schema 21/47,036/186 / 28/103,075/1,018; tests 15/43,676/728 / 29/74,812/1,123; mutations 27/36,490/755 / 28/47,436/937; editor 25/117,041/2,027 / 31/186,692/3,184; v1 2/3,455/75; facade 1/11,987/239 / 4/45,213/447. All inside 32/262,144/4,000 for lib and test.

### 2.2 space home (DONE)

No source change. `--verify --oracle --keep` exit 0 (oracle native/test 0/0, layout 0/0), apply (79 files, 7 crates, owner rows), `--check-applied` exit 0 with onlyCombined 0/0. Owners scan: no missing row under plugins. Taxonomy: only the pre-existing plugin-root manifest finding. Census after (lib / test files/bytes/lines): editor 9/36,598/697 / same; viewer 2/22,616/423 / 5/107,957/1,515; examples 26/71,901/1,035 / 28/82,412/1,186; operations 14/20,359/375 / 20/36,372/615; kinds-conformance 23/171,749/2,448 / 27/234,547/3,412; inferences 28/42,628/649 / 29/43,311/665; schema 23/22,242/253 / 28/34,488/487; facade 1/8,601/184 / 2/26,517/275. All inside the gate for lib and test.

### 2.3 flow brep extension (DONE, hand carve validated by `--check-applied`)

The planner cannot split this extension: with `--lift=::*` it puts mesh, tessellate and the geometry inference into one crate and the unit tests into it (test closure 331 KB), and without lift it makes a tests crate with no dependency on the facade (broken in a real build). So the carve is manual:

* New crate `semio-s-plugin-flow-brep-mesh` (`📐️brep/🥽️mesh/📦️packages/🦀️rust`, lib path `../../🦀️.rs`): the former `mod mesh` (indexed polygon mesh operators). `use super::*` became explicit imports, all private items and members widened to `pub` (the mesh laws drive jobs and states from another crate), the `retire_geometry_capture!` macro moved here with `#[macro_export]` and the facade imports it.
* New crate `semio-s-plugin-flow-brep-tests` (`📐️brep/🧪️tests/🔬️unit/📦️packages/🦀️rust`): mounts the unit laws, the evaluate-budget law, the extension-guest laws (formerly `include!`d into `extension_guest`, now a normal module) and the mesh laws (moved out of the mesh crate because they drive the whole registry and the unit-test helpers). It depends only on the public API of the facade (`register`, `module_registry`, `extension_manifest_json`, `geometry_inference`, `extension_guest`, `NODE_KERNEL_METHOD`, `operation_quality`) and the mesh crate. `extension_guest::bundle` and `scoped_bundle` are `pub` production functions now (`scoped_bundle` used to be `cfg(test)`).
* Embedded assets leave the test closure: every `include_str!` of the four test files became a runtime read (`runtime_fixture_text`, 672 KB component descriptor included). Generator fix: `fixturesToRuntime` now converts every embedded text of a file below a `🧪️tests` directory, not only texts whose path passes through one (test added).
* Workspace member rows, `Cargo.lock` blocks, owner rows (`r7-tools/owner-row.ts`, uses the generator inserter) and the extension router test package list (generator fix below) are updated.
* Cross-crate cfg(test) (coordinator rule): `INTENTIONALLY_UNEXPOSED` (a `cfg(test)` const of the facade) moved into the unit laws; `GEOMETRY_INFERENCE_CALLS` (a `cfg(test)` static read by the guest laws) became a `testkit` feature of the facade (`#[cfg(feature = "testkit")]`, `pub static`), enabled by the tests crate through its dependency `features = ["testkit"]`. `r7-tools/cfgtest.ts` proves it: every crate compiled with the other crates' `cfg(test)` switched off, new errors in its own files: 0 (the tool detects a seeded probe).

Commands: `r7-brep-check3` (`--check-applied`) exit 0, onlyCombined 0/0; owners scan no brep row missing; taxonomy: only the pre-existing plugin-root manifest finding. Census after (lib / test files/bytes/lines): facade 6/198,897/2,816 / 6/198,897/2,817; mesh 1/104,302/1,212 / same; tests 1/1,142/16 / 5/224,331/2,415; component 1/300/4. Before: lib 7/303,720/4,040 (over by bytes), test closure refused (missing input). All inside the gate.

### 2.4 Cross-crate cfg(test) check of the applied packages

`r7-tools/cfgtest.ts` over the kept combined layouts of vcs (8 crates), space home (8 crates) and brep (4 crates): 0 references from one crate to another crate's `cfg(test)` items.

## 3. Generator changes (all with tests in `L/🧪️tests/📦️crate-tree/🟦️.ts`, 60 pass)

1. `🧪️test-closure`: `fixturesToRuntime` converts every `include_str!` in a file below a `🧪️tests` directory (brep: a 672 KB descriptor was embedded in a test).
2. `📝️render`: `patchRouterPackages` also extends the package list of an extension router's `runRepositoryCargoTests([...])` call (brep: "Router call not found" aborted the apply).

## 4. Final verdict

DONE (inside the gate for lib and test, `--check-applied` exit 0, 0 extras, 0 cross-crate cfg(test) references): vcs vcs, space home, flow brep. BLOCKED, no source of these five was applied and nothing is left failing `--check-applied`: trinity jack, trinity rewriting, writer writer, sourcing curation, sequence sequence. Each needs a design-level refactor of an editor/hub god-module that the generator cannot do and that I could not finish safely in this slice; the exact findings and tools are below.

## 5. Blocked packages: findings and next steps

Tools (all in `F/r7-tools/`): `scc.py <log> --lib` (strongly connected components of the `F3_SLICES=1 … --dry-run --explain` slice graph, with the edges that hold them together), `closure.ts` (per-file source closure of a crate), `orig-oracle.ts` (plain full-resolution oracle on a sandbox copy; `SRC_DIR` override for differentials), `cfgtest.ts` (cross-crate cfg(test) check), `owner-row.ts`, `fixtures-runtime.ts`.

### 5.1 trinity jack
* Baseline planner: the editor crate is one 42-file coherence group (windows families with their io/mutation codecs, commands, panels, play-app, retained commands, manifest) because every part names the editor root (`editor::jack`: window ids, `jack_action`, `ui_label`, `content_to_workflow`, `reset_document_effect`) and the root mounts them back.
* Source changes made (unapplied, verified neutral: the full-resolution oracle on the plugin with and without them has the identical error multiset for native and `--test`): `pub(crate)` widened to `pub` in the editor tree and artifact root; `transient::component::…` paths replaced by `transient::…` (the planner rejects any explicit `component` segment) with `editor_window`/`results_window` made `pub mod`; `WINDOW_KIND_ID` of the graph and results windows localised in their own files (like the editor window), the root constants point at them.
* With `--lift=editor::jack` the planner now succeeds (exit 0): graph-manifest, snapshot, window-config 11/18,137/386, results-window 10/20,668/380, schema 27/61,249/393, inferences 17/35,187/513, transient 10/15,156/317, operations (editor) 25/170,195/2,850, v1 10/28,857/553, all inside the gate. But `--verify --oracle` fails (oracle native 19, test 20, layout 64/91 extras) for two reasons: (a) the lift moves `play_app` out of its host while the host reaches `retained_commands` through a private glob (`use retained_commands::*` must be `pub use`), (b) the artifact root is a hub (`ast`, `executor`, `language_service`, `lexer`, `host`, `graph_model`, `content`, `errors`, `registration`, `manifest`, 28 files, 264,960 bytes, 5,424 active lines) whose items are named as `crate::Graph`, `crate::Node`, `crate::Camera`, `crate::port_key` … by the carved crates; root items that arrive through glob re-exports are invisible to the planner, and the planner does not check the facade budget (it passes at exit 0 although the facade is over the gate).
* Next step: carve the hub by hand into two lower crates (query language: ast/lexer/executor/language_service ≈ 2,400 lines; model: graph_model/content/errors/manifest/registration) and replace the root globs by explicit `pub use` lists, then re-run the lift plan. Generator improvement worth doing first: check the facade measure in `planArtifactCrates` and resolve names re-exported by root globs.

### 5.2 trinity rewriting, writer writer, sourcing curation
Planner (`--dry-run --explain`, `F3_SLICES=1`): the schema group (rewriting 175 files / 6,403 lines, writer 171 / 7,100, curation 134 / 6,013) is held together by cycles ("mutation, other, parent-child") through the artifact hub and the editor root (`editor::rewriting`/`writer`/`sourcing`, 1,300 to 1,440 lines each). `scc.py` on the curation log shows the editor SCC of 11 slices (41 files, 2,850 lines): commands (16 files), the grid/pool/curated windows, the `v1`/`any` wrappers and the root all reference `editor::sourcing`. Same cure as jack: lift plus lowering of the editor-root items (and the hub) into a base crate; none of the three is mechanical.

### 5.3 sequence sequence
Only one crate is over budget (editor: lib 23 files / 305,089 bytes / 5,341 lines). With `--lift=editor::sequence` the 18-slice SCC (40 files, 5,452 lines) remains: `host` calls the command types while the commands call the root helpers, and `base`, `persistent`, `retained`, `app` glob the root. Widening of `pub(crate)` in the editor tree and root (unapplied, oracle-neutral like jack) removed the visibility blockers. A real layering (host/base below commands) is needed.

## 6. Coordinator rule (no cross-crate cfg(test) items)

Applied to all produced crates (vcs, home, brep) with `r7-tools/cfgtest.ts` (0 references). Fixed in brep: `INTENTIONALLY_UNEXPOSED` moved into the unit laws, `GEOMETRY_INFERENCE_CALLS` behind the facade's non-default `testkit` feature enabled by the tests crate, `scoped_bundle`/`bundle` made production `pub` functions, shared test helpers and every consumer in one tests crate.
