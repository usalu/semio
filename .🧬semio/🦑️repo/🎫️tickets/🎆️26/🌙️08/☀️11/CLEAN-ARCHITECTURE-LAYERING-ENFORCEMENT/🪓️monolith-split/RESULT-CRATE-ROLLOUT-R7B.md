# Result: CRATE-ROLLOUT-R7B (prefix `r7b`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r7b-tools/` (`step.sh <tag> <cmd...>`, `crate-refs.py`). Logs: `F/🗑️generated/r7b-*`. Sandboxes: `/Users/ueli/.cache/r7b/`. Started 2026-10-17 11:00, resumed after the usage-limit reset.

## 0. Verdict (kept current)

| Package | Verdict | Notes |
| --- | --- | --- |
| trinity jack | LAYERED AND CARVED (static) | hub carved into the standards tree, 11 producer crates plus facade, all inside the gate for lib and test; `--check-applied --base=dir:` exit 0 with 0 extras; see 3 |
| trinity rewriting | LAYERED AND CARVED (static) | 9 producer crates plus facade, all inside the gate for lib and test; `--check-applied --base=dir:` exit 0 with 0 extras; see 4 |

## 1. Found state (jack, 2026-10-17 11:10)

* R7's unapplied oracle-neutral edits are still in the working tree (65 changed files in the jack artifact: `pub(crate)` to `pub` in the editor tree, window-id constants in their own modules, `transient::` paths). Kept.
* Planner baseline now (`r7b-base0.log`, `F3_SLICES=1 new crates <pkg> --dry-run --explain`, 11 s): exit 0, no problems, 9 crates (inferences 27/80,628/886, snapshot 12/93,714/1,488, window-config, results-window, schema 26/58,456/431, transient, operations 17/49,176/984, jack (editor) 26/158,475/2,605, sqlite-snapshot-declaration-tests).
* But `--verify` (`r7b-ver0.log`) fails: the facade crate `semio-s-artifact-trinity-jack` is **28 files / 265,835 bytes / 5,439 lines (lib) and 35 / 345,062 / 6,663 (test)**, over the hard gate. The planner does not check the facade, and it never carves a *root file module* (`carvable` in `🗺️plan/🟦️.ts` drops every slice whose attach decls are all non-inline modules mounted at the artifact root): `ast`, `lexer`, `executor`, `language_service`, `content`, `content_read`, `manifest`, `errors`, `graph_model`, `registration`, `host`, `graph_manifest` all stay in the facade. Facade closure by concern: query language (ast 132, lexer 235, executor 270+410+423+161+161+267, language-service 32+382+297+210+108+42) about 2,450 lines; host (initialization-authority 365, initialization-contract 337, snapshot-clone 335, mutation-decode 239, snapshot-decode 182, root 44, tests 336) about 1,500; model (graph-model 321, errors 149, registration 141, content 105+56, manifest 88) about 860; artifact root 268.

## 2. Layering design (jack)

Rule used: a module that is mounted at the artifact root as a file module is invisible to the carve, so every concern moves under the `standards::v1::subsets::any::<facet>` wrapper tree (the existing owner paths, e.g. `schema::snapshot::JackSnapshot`) and keeps its on-disk folder; every `crate::X` use is re-pointed to the owner path; the root keeps no `pub use` of moved items.

Dependency direction (lower first):

1. `schema::errors` (`TrinityRamError`), `schema::manifest` (`Manifest`, kind defs), `schema::graph` (`Port`, `Node`, `Edge`, `Camera`, `EntityRef`, port-key helpers, graph constants, `GraphEffect` pure data), `schema::content` types (`JackContentGraph`, `JackContentChild`): no dependency on snapshot.
2. `schema::snapshot` (`JackSnapshot`, its value codec): depends on level 1 only. Inherent methods that need io, composition or the content owner leave the struct (an inherent impl cannot span crates): `to_json`/`from_json` move to the io facet, `with_content`/`nodes`/`edges` become functions of the composition/content layer.
3. `schema::content` owner machinery (`JackContentOwner`, working scene, retirement helpers), `schema::language` (ast, lexer, language service, executor): depend on levels 1 and 2, the executor no longer reaches the host.
4. `schema::{inferences,diff,mutations,operations}`, `io`, host (`snapshot-clone`, `initialization-*`, decoders): above snapshot.
5. editor, viewer, registration (`ArtifactApps`, `definition`, `artifact`), v1: composition level, depend downward only; invert through traits (`JackComposition`) at the composition root.

Details of each step, the before and after census and the gate results are appended below as they are done.


## 3. Jack: what was done (2026-10-17, tools in `F/r7b-tools/`)

### 3.1 Layering as implemented (the root file keeps no `pub use` of any moved item)

All former root hub modules are mounted inside the `standards::v1::subsets::any::schema` wrapper (a root file module is never carved, the planner keeps it in the facade), their on-disk folders unchanged, every `crate::X` use re-pointed to the owner path by `repoint.py` (name map `map-crate.json`, group-aware, 78 + 13 files; the same map re-pointed 34 external files, see 3.4).

| module (`…::any::schema::`) | content | depends on |
| --- | --- | --- |
| `errors` | `TrinityRamError` | external only |
| `manifest` | `Manifest`, kind defs (+ their `artifact_retire_struct!` impls, moved out of the schema `retirement` module because an impl cannot live in another crate than its type) | graph manifest framework crate |
| `graph_manifest` | generated Nakagin registry | framework |
| `graph` | `Port`, `Node`, `Edge`, `Camera`, `EntityRef`, `GraphEffect` (moved out of the executor so the content read traits and the host need no query code), port-key helpers, schema/query/dialect constants | external only |
| `json_transport` | `convert`, `decode_camera` (were in the io JSON codec, called by the snapshot: a snapshot to io edge) | graph |
| `content`, `content_read` | graph capability child, owner, working scene, retirement drivers; read traits (no more `JackSnapshot`); new narrow port `JackContentFactory` (`child_from_working`) | graph |
| `snapshot` | `JackSnapshot`, its inherent impl (was in the graph model file), `jack_working_scene`, `empty_trinity_graph_snapshot`, `jack_child_restore_projection`, its `RetireOwned` impl (was in `schema::retirement`, deleted) | graph, manifest, content |
| `composition` | `JackComposition: JackContentFactory` (invert: the composition root supplies members, reads, examples), scene and genesis reads through it | snapshot, content |
| `memory` | in-memory `Graph` (the subgraph snapshot builder moved to the executor, the only caller) | snapshot, composition |
| `ast`, `lexer`, `language_service`, `executor` | the query language | memory, snapshot, host |
| `runner` | `run`, `run_json*`, `encode_query_result_json` (were in `io::text::snapshot`, which made the snapshot codec depend on the whole query language: the cycle that held the old schema group together) | executor, language service, operations |
| `host` | retained store initialization, snapshot clone/decode, mutation decode | mutations, snapshot, content |
| `any::registration` | `ArtifactApps`, `pilot_languages`, `definition`, `artifact`, `artifact_kind` | v1 and any only |

Other layering edits: `apply_trinity_graph_mutation` moved from the root into `operations`; the editor leaf commands are now `commands::<fn>::<fn>` modules instead of `pub use leaf::fn` lines inside the inline `commands` wrapper (those re-exports were lost by the facade weaving: layout showed 9 `E0425`); `play_app` and `retained_commands` import `commands` explicitly (a glob-provided alias hides the edges from the planner); root `test_identity!` cfg(test) macro stays in the facade, its users are hoisted there.

### 3.2 Generator-facing source fixes (no generator edit by me)

* Tests that name `crate::test_support` (a cfg(test) root module) were never hoisted because they started with `use super::*`; they now glob the absolute host path, so the hoist applies (11 test modules now live in the facade, which is why the facade test closure is 18 files / 144,727 bytes / 2,342 lines).
* `examples::demo` metadata test no longer uses `super::`; its two inference laws moved to the inference unit tests; sqlite snapshot declaration test mount renamed `sqlite_snapshot_tests` (the hoist guard matched the textual `snapshot::tests` of the binary and text snapshot tests).
* Stale test fixed on the way: `JackQueryWork` is generic, the job test used it bare (E0107).
* Visibility widened where hoisted tests need it: host child modules and `pub(super)` items, `retained_commands`, execution `emit_*` functions, owned-bytes helpers (`pub`).

### 3.3 cfg(test) hand-off (coordinator, `test_support`)

`test_support` is re-gated `#[cfg(any(test, feature = "testkit"))]` at the artifact root, its items are `pub`, `testkit = []` is declared in the artifact `Cargo.toml` (the generator forwards `testkit` to every producer crate), the executor's `fixture_*_owned_bytes` re-export is gated the same way, and the shell crate enables the feature in `[dev-dependencies]` and names `semio_s_artifact_trinity_jack::test_support::FixtureComposition` (its test used a nonexistent `crate::test_support`). `ctc-scan.ts --only <jack crates>`: 2 hits, both sanctioned (the shell, gated by testkit); after the shell path fix the shell references the owner explicitly.

### 3.4 External consumers re-pointed

`repoint.py` with prefix `semio_s_artifact_trinity_jack` (and the alias `jack`/`super::jack`) over writer (4 files), trinity bridge, hub composition (2), dev/trinity graph (7), rewriting (34), the shell: every moved name now uses its owner path; framework types the root used to re-export (`PortDirection`, `PropertyBag`, `PropertyDef`, `PropertyKind`, `PropertyValue`, `GraphManifest`, `ManifestValidator`) and the graph query types are named from their own crates (`semio_framework_graph_manifest`, `semio_s_artifact_trinity_jack_graph_query`); dev/trinity graph and the shell got the two path dependencies; dev/trinity `SemioJackComposition` implements the new `JackContentFactory` port; dev graph tests use explicit imports instead of the old root glob.

### 3.5 Gate results

* Planner: `new crates <pkg> --dry-run --explain` exit 0, no problem. `--verify --oracle` (`r7b-ver6.log`) exit 0: single-crate oracle native 1759/1759 and `--test` 2285/2228 with 0 only-candidate, layout native 0/0 and `--test` 0/0.
* Apply with backup (`r7b-apply1.log`, `F/🗑️generated/r7b-backup-jack`): 77 files, 10 new crates plus the facade, owner rows registered.
* `--check-applied --base=dir:/Users/ueli/.cache/r7b/jack-pre` (`r7b-check1.log`) exit 0: native 1745 base / 1745 combined / 0 extra, test 2279 / 2222 / 0, admission, shared, sharedForeign empty. The base is a copy of the pre-carve working tree (git HEAD lacks every uncommitted layering edit).
* Census after (`r7b-census-jack-after.tsv`, lib / test files/bytes/lines): facade 1/12,517/219 / 18/144,727/2,342; errors (model base) 25/96,153/1,467 / 28/97,811/1,545; graph-manifest-tests 2/1,103/38 / 3/1,597/55; window-config 11/18,617/402; inferences 18/117,529/1,682; language-service 28/109,444/2,354; schema 27/149,178/1,743 / 28/154,307/1,813; executor 18/95,859/1,926 / 22/120,041/2,276; results-window 10/21,394/390; transient 10/15,156/317; jack (editor) 21/158,089/2,434 / 24/165,588/2,617; graph-query 4/10,501/167 / 7/24,177/270; shell 1/4,604/126. Over-or-error 0. Before: facade 28/265,835/5,439 lib and 35/345,062/6,663 test (over the gate), whole artifact lib 153/723,284/12,295.

## Hand-off from BASELINE-DEFECTS-9 (graph crate migration, appended 15:00)

`semio_framework_graph` no longer exists; its items live in `semio-framework-graph-engine` (`Storage`, `GraphView`, `NodeId`, `EdgeId`, `HandleId`, `Normal`, `Ported`, `Directed`, `Undirected`, `Directedness`, `PortModel`, `AttrView`, `AttrWeight`, `EdgeRef`, `EdgeWeights`, `EdgeSubgraphView`, `SubgraphView`, `GraphError`, `pairwise`) and `semio-framework-graph-manifest` (`PropertyBag`, `PropertyValue`, `GraphManifest`). The stdio semio graph subset was migrated by bd9 (`bd9-tools/graph_migrate.py` plus the import rewrite described in `COMPILE-DEFECT-LEDGER.md` bd9-06): roots use `extern crate semio_framework_graph_engine as graph_core;` and `extern crate semio_framework_graph_manifest as graph_manifest;`, imports of the four property names go through `graph_manifest`, the workspace manifest declares both framework crates and the lock gained their two blocks. Trinity (jack, rewriting, `🧑‍💻dev/🔱️trinity`) was NOT edited; its remaining `semio_framework_graph` mention is a doc comment in `🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests/✏️editor/🦀️.rs:976`. Apply the same two-line root change and the Property* import split there.


## 4. Rewriting: what was done

Baseline planner: schema crate 115 files / 390,433 bytes / 5,602 lines (test 124 / 534,933 / 7,364), one 58-slice cycle through the schema component, the io text snapshot codec and the editor root.

### 4.1 Layering as implemented (no `pub use` of a moved item left at the root)

| module | content | notes |
| --- | --- | --- |
| `schema::errors`, `schema::model`, `schema::graph_manifest` | `TrinityRewritingError`; `LayoutPoint`, `REWRITE_RULE_SCHEMA`, `TRINITY_REWRITING_DIALECT`, `artifact_kind()`; generated registry | `artifact_kind` moved out of the registration so the editor no longer depends on the registration, v1 and any (that edge made the registration/v1/any/editor knot) |
| `schema::rule`, `schema::layout`, `schema::snapshot` | rule records with the `Pattern` to Jack-pattern impl (was in the schema component: the coherence merge that tied snapshot and component together), `default_parameter_bindings`, their `artifact_retire_struct!` impls (were in `schema::retirement`, deleted; the document-retirement tests are mounted from the snapshot), `RewritingSnapshot`, `rewriting_child_restore_projection` | consumers named `schema::Lhs` etc. through the component re-exports; now `schema::rule::Lhs`, `schema::layout::RuleLayout` (the component keeps no `pub use`) |
| `schema::application` | `apply_rule`, `ApplyRuleResult`, `RuleQueryResult`, `build_rule_query` and its helpers, `apply_rule_json`, `rule_query_json`, `parse_bindings_json` (were split between the component and the io text snapshot codec, which made the codec depend on the component and the component on the codec) | its tests (`rule-application`) are mounted from it |
| `schema::test_support` | `nakagin_fixture`, `fixture_rule_state` (were `cfg(test)` items of the editor root named by the codec, operation and application tests: a cross-crate cfg(test) reference) | `#[cfg(any(test, feature = "testkit"))]`, `testkit = []` in the artifact `Cargo.toml`, the generator forwards the feature and enables it from the facade's `[dev-dependencies]` (no self dev-dependency). `ctc-scan.ts`: 13 hits, all sanctioned |
| `editor::rewriting::{ids, ui, document}` | surface/body/window ids; action and UI value helpers and the semantic rule-graph renderer; default/reset document, compiled query, semantic graphs | the 9 windows, 3 panels and 12 commands name these instead of the editor root; `window_config` no longer uses `super::` (it unioned every kid of the inline editor wrapper) |
| `editor::rewriting::commands::<fn>` | the leaf commands as `pub mod <fn>` (was `mod <fn>_leaf` plus `pub use`, which the facade weaving dropped) | callers use `commands::<fn>::<fn>`; `play_app`, `retained_document_commands`, `manifest` import `commands`, `window_config`, `edit` explicitly |
| `any::registration`, `any::content` | `ArtifactApps`, `pilot_languages`, `definition`, `artifact`; the composed-child owner | `apply_rewrite_rule_mutation` moved into `operations` |

Mutation leaf imports of the aggregate (`mutations::{edit_lhs, ...}`) point to the leaf modules (`mutations::edit_lhs::edit_lhs`), `pub(crate)` of the editor, content and viewer trees and of the root wrappers is `pub`, `retained_document_commands`, `args_bridge`, `manifest`, `content::read` are `pub mod`.

### 4.2 Stale references found while re-pointing (not introduced here)

* `semio_s_artifact_trinity_jack::materialize_jack_snapshot` (55 mentions in 12 files) does not exist in jack; it lives in `semio-s-dev-trinity-jack`. Left as written, a rewriting to dev dependency would be a cycle. The rule-application tests also named `jack::executor::run` (now `schema::runner::run`), `Graph::host_snapshot_json` without the composition parameter and `JackSnapshot::with_content` without a type argument.
* Generated manifest registries (`🤖️generated`, gitignored) imported the removed `semio_framework_graph::manifest::Manifest`; the four files of jack and rewriting now use `semio_framework_graph_manifest::GraphManifest`. No tracked generator emits that line (`git grep` finds none), the next regeneration must emit the new import.

### 4.3 Gate results

* Planner `--dry-run --explain` exit 0. `--verify --oracle` (`r7b-rver5.log`) exit 0: oracle native 1414/1423 and `--test` 2521/2534 with 0 only-candidate, layout native 0 and `--test` 0. Plain oracle diff against the untouched copy: only a moved `semio_framework_async` line.
* Apply (`r7b-rapply1.log`, backup `F/🗑️generated/r7b-backup-rewriting`): 95 files, 9 crates, owner rows.
* `--check-applied --base=dir:/Users/ueli/.cache/r7b/rw-pre` (`r7b-rcheck1.log`) exit 0: native 1414 / 1423 / 0 extra, test 2521 / 2534 / 0, admission, shared, sharedForeign empty.
* Census after (`r7b-census-rw-after.tsv`, lib / test files/bytes/lines): facade 1/15,725/283 / 1/15,725/284; mut-any (base) 28/42,912/583; layout 15/26,287/359; inferences 23/83,772/979 / 28/124,803/1,290; schema 28/83,408/675; mut-any-2 (mutations) 27/44,159/887 / 29/47,166/951; operations 22/78,740/1,312 / 29/151,064/2,275; editor 28/125,714/1,961; tests-narrows 12/51,695/816 / 22/164,462/2,028; v1 5/23,123/369 / 11/149,430/2,020. Over-or-error 0. Before: schema 115/390,433/5,602 lib, 124/534,933/7,364 test.
* External consumers re-pointed (bridge, hub composition, dev fixture sweep): `artifact`, `artifact_kind`, `RewritingSnapshot` use their owner paths.


## 5. Final repository checks (both packages)

* `--check-applied` re-run for jack after the last source edits (generated registries, shell) with a rebuilt pre-carve base (`r5a-tools/precarve.py`, `r7b-check2.log`): exit 0, native 1744 / 1743 / 0 extra, test 2278 / 2220 / 0, admission, shared, sharedForeign empty.
* `cfgtest.ts` on the kept combined layouts: jack 12 crates, rewriting 9 crates plus facade, 0 references from one crate to another crate's `cfg(test)` items (`r7b-cfgtest.log`, `r7b-rw-cfgtest.log`). `ctc-scan.ts`: jack 2 hits (shell, sanctioned and fixed to name the owner explicitly), rewriting 13 hits, all gated `any(test, feature = "testkit")` with the feature enabled from `[dev-dependencies]`.
* Oracle blind spot (kept root file modules): the only root file modules left are the `cfg(test)` ones (`test_support`, `tests`, hoisted tests); all 14 jack test files naming `crate::test_support` are hoisted into the facade (`mod hoisted_*` in the root, verified by grep), no lower crate names it. Rewriting keeps no root file module at all.
* Owners scan (`owners-tools/scan.ts`): no trinity row missing (18 missing rows, none under trinity). `pw0-taxcheck.ts` trinity: the existing plugin-root manifest finding only. `lib-wiring-budget`: 2 passes, 1 failure that names only foreign roots (bim model, note, flow brep mesh, puzzle 5d and 2d test support), none under trinity. Both launch JSON files parse. Census (`r7b-census-jack-after.tsv`, `r7b-census-rw-after.tsv`): over-or-error 0 for every crate, lib and test.
* Not run: real `cargo` (the launcher is not admitted, nothing was compiled; the gate evidence is the analysis-only oracle, the layout differential and the static checks above). Type-level defects of the stale references listed in 4.2 stay invisible to the oracle.
* Generator: no source edit by me. Observations for the generator owners: crate labels come from the first mounted slice of a group (`-errors`, `-mut-any` name the lowest model crate); a `pub use leaf::fn` inside an inline wrapper is dropped by the facade weaving (jack commands, rewriting commands and mutation imports); a glob-provided alias (`use super::*`) hides the edges to the planner; a `super::` path in one kid unions every kid of its inline parent.

## 0b. Final verdict

DONE (static) for trinity jack and trinity rewriting. Before to after (lib files/bytes/lines): jack facade 28/265,835/5,439 to 1/12,517/219 (test 35/345,062/6,663 to 18/144,727/2,342), whole artifact 153/723,284/12,295 across 13 crates, largest crate 28/109,444/2,354; rewriting schema crate 115/390,433/5,602 (test 124/534,933/7,364) to 10 crates, largest lib 28/126,089/1,963 and largest test 29/151,064/2,275.
