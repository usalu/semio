# Result: CRATE-ROLLOUT-R7C (prefix `r7c`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r7c-tools/` (`step.sh <tag> <cmd...>`, `checks.sh`, `r7c-census.ts`, `plan-dump.ts`, `seq-*`, `lib-imports.ts`, `restore-orig.sh`, `revert-apply.py`, `orig-*` snapshots). Logs: `F/🗑️generated/r7c-*`. Sandboxes: `/Users/ueli/.cache/r7c/`. Started 2026-10-17 12:55.

## 0. Verdict (kept current)

| Package (`✏️s/🔌️plugins/…`) | Verdict | Notes |
| --- | --- | --- |
| sequence sequence (`🎬️sequence/🗿️artifacts/🎬️sequence`) | DONE (static) | layered into 5 crates + facade, all inside the gate (lib and test); `--verify --oracle` exit 0; `--check-applied` exit 0, 0 / 0 extras; cfg(test) scan clean |
| writer writer (`✒️writer/🗿️artifacts/✒️writer`) | DONE (static) | 8 crates + facade, all inside the gate (lib and test); `--verify --oracle` exit 0; `--check-applied` exit 0, 0 / 0 extras; cfg(test) scan and `cfgtest.ts` 0 |
| sourcing curation (`🪵️sourcing/🗿️artifacts/🗂️curation`) | DONE (static) | 6 crates + facade, all inside the gate (lib and test); `--verify --oracle` exit 0; `--check-applied` exit 0, 0 / 0 extras; cfg(test) scan and `cfgtest.ts` 0 |

## 1. Baseline

Read-only baseline from R7 (`F/🗑️generated/r7-census-base0.tsv`): sequence-sequence lib 93 / 465,539 / 7,657 and test 135 / 637,390 / 9,837 (files / bytes / lines). The planner (before the layering) gave six crates; only the editor crate was over (lib 23 / 305,089 / 5,341, test 32 / 398,903 / 6,907) and held an 18-slice cycle.

## 2. Sequence sequence

### 2.1 Layering design

Cause found with `F3_SLICES=1 … --explain` plus `scc.py`: the editor root file (`✏️editor/🦀️.rs`) was a hub. It mounted the six layers (`🧱️base`, `🖥️host`, `🧵️retained`, `🔁️persistent`, `🎞️import`, `🎹️app`) as private modules, re-exported all of them with `pub use self::x::*`, and carried a 100-line `use` list. Every layer started with `use super::*;`, and every command, window and panel reached the root helpers as `crate::editor::sequence::sequence_edit_emit`. So each lower layer depended on the root, and the root depended on every layer. The `host -> commands` edge of R7 was not real: the host only matched the names `connect_steps`, `add_step` and `reorganize`, which the root list had imported from the commands.

Target layering (dependencies point downward only):

| layer | modules | depends on |
| --- | --- | --- |
| schema | `schema`, `io` codecs, `snapshot`, `diff`, `mutations` | framework |
| inferences | schema inferences, `edit_rules`, `terminology`, `presence`, `config`, `examples` | schema |
| base + host | `editor::sequence::base` (ids, typed media I/O, faults, camera, child-leaf emit), `host` (`SequenceHost`), `wasm` | schema, inferences |
| leaves | `commands/*`, `modes::edit::windows::{main, script, compiled}`, `panels/*` | base, host, edit_rules |
| top | `retained` (the `SequenceCommand` enum, caps), `persistent`, `import`, `app` | all of the above |
| facade | the nest, `standards::v1`, hoisted integration tests, `editor_unit_tests` harness | everything |

`retained`, `persistent`, `import` and `app` stay in one crate on purpose: the job factories name the app type (`ArtifactReservedToolJobRequest<EditorApp<SequencePlayApp>>`) and `app` names the factories, `retained` holds an inherent `impl SequencePersistentWork`, and an inherent impl cannot span crates.

### 2.2 Source changes (no aliases, every caller re-pointed in the same change)

Scripts in `F/r7c-tools/`, originals in `orig-seq/`, `orig-seq-tests/`.

1. `seq-lower-root.ts` (+ `lib-imports.ts`): every `use super::*;` of the six layers is now an explicit import list (external crates by the root statements pruned to the used names, siblings by `crate::editor::sequence::<layer>::{…}`); the two globs of the unit-test harness likewise. Every root path `crate::editor::sequence::Name` (and the `semio_s_artifact_sequence_sequence::editor::sequence::Name` form of the dev and hub consumers) names the owning layer: 28 files, among them the hub composition, the dev composition runtime tests and the subset root (`editor::app::SequencePlayApp`). The six `SEQUENCE_PLAY_BODY_*` re-exports of `base` (an upward edge from the lowest layer to windows and panels) are gone; users name the owning window or panel; `ui_node_list` is named from `semio_framework_plugin`.
2. `seq-mount.py`: the artifact nest mounts `base`, `host`, `retained`, `persistent`, `import`, `app` as `pub mod` next to `commands`, `modes`, `panels`; the editor root file `✏️editor/🦀️.rs` owned no item any more and was removed (its description moved onto the `pub mod sequence` nest as a docstring).
3. The mounted-app test harness (`✏️editor/🧪️tests/🔬️unit`, 810 lines) is mounted by the artifact root as `#[cfg(test)] pub mod editor_unit_tests` (it drives the whole app, so it belongs above every layer). `seq-tests-explicit.ts` made the ten leaf tests that use it self-contained (`use super::*` and `super::x` replaced by absolute imports of the host module; `split_endpoint` widened to `pub`), so the carve hoists them to the facade (`hoisted_*`). This also resolves the `new_app_with_registry_wired` hand-off of CFG-TEST-CROSS-CRATE: the helper and every user are in the facade crate now.
4. The wasm32-only guest bridge (`wasm/🧩️component.rs`, mounted with `#[cfg(target_arch = "wasm32")]`) names `SequenceHost` and its `protocol` child. See the generator change 5.1: the planner did not scan it and had put it into a crate below `host`.
5. `SequenceSnapshot::to_host_snapshot` (schema crate, `#[cfg(test)]`, used by the facade tests and by a test of the `tests` crate) is now `#[cfg(any(test, feature = "testkit"))]`; the package manifest declares `testkit = []`; the generator forwards the feature (carved crates declare it, siblings are dev-dependencies with `features = ["testkit"]`; the facade manifest got the same rows, section 5.1).
6. `split_endpoint` of the main window is `pub` (test import); the R7 `pub(crate)`/`pub(super)` to `pub` widenings in the editor tree were verified present in the working tree and are kept (no restricted visibility remains in the plugin; `grep 'pub(crate|super|in '` finds only the sqlite public-law helpers and the harness field).

### 2.3 Planner result and crates

`F3_SLICES=1 … --dry-run --explain` after the changes: no cycle (the 18-slice SCC is gone), exit 0. Crates (the generator names a crate after its first slice; names such as `-tests` and `-editor` do not describe the layer, the same defect as R2):

| crate | holds | lib files/bytes/lines | test files/bytes/lines |
| --- | --- | --- | --- |
| `…-sequence-schema` | schema, snapshot, diff, mutations, io codecs | 31 / 83,292 / 1,095 | 32 / 85,124 / 1,135 |
| `…-sequence-inferences` | inferences, edit_rules, terminology, examples, `wasm::protocol` | 25 / 48,321 / 778 | 29 / 52,156 / 858 |
| `…-sequence-tests` | `base`, `host`, `wasm` (component), viewer | 21 / 105,489 / 1,959 | 29 / 136,065 / 2,467 |
| `…-sequence-editor` | commands, windows, panels | 15 / 52,089 / 1,001 | 18 / 58,172 / 1,088 |
| `…-sequence-v1` | `retained`, `persistent`, `import`, `app`, subset root | 7 / 193,559 / 3,152 | 9 / 200,754 / 3,267 |
| `semio-s-artifact-sequence-sequence` (facade) | nest, hoisted tests, harness | 1 / 14,000 / 246 | 16 / 126,527 / 1,816 |

Before: one editor crate lib 23 / 305,089 / 5,341, test 32 / 398,903 / 6,907. All crates are inside 32 / 262,144 / 4,000 for lib and test (`F/🗑️generated/r7c-census-seq-after.tsv`: `crates 7 over-or-error 0`, the seventh is the existing mutation bridge).

### 2.4 Gates

* `new crates <pkg> --verify --oracle --sandbox=/Users/ueli/.cache/r7c/seq-verify`: exit 0 (`r7c-seq-verify7.log`): module trees equal, visibility 0, orphan 0, glue 0, unresolved crates 0, oracle native 1256 / 1256 and `--test` 1799 / 1805 with `onlyCandidate` 0, layout `onlyCombined` 0 / 0.
* Apply with `--backup=F/🗑️generated/r7c-backup-seq/b` (relative path), 68 files, 5 crates, 5 owner rows (`r7c-seq-apply3.log`).
* `--check-applied` (`r7c-seq-check4.log`): exit 0, native and `--test` `onlyCombined` 0, `admission`, `shared`, `sharedForeign` empty.
* cfg(test) scan `ctc-tools/ctc-scan.ts --only <the six crates>`: 2 weak method hits (`to_host_snapshot`), both sanctioned by the `testkit` feature enabled in the consumers' dev-dependencies; `r7-tools/cfgtest.ts` on the kept combined layout: 0 references from one crate to another crate's `cfg(test)` items.
* Oracle blind spot (brief): the facade keeps the root file modules `tests`, `editor_unit_tests` and `hoisted_*` (all `cfg(test)`); `git grep` shows that the only files naming `editor_unit_tests` are the ten hoisted tests, the harness itself and the root mount, all mounted by the facade; no `crate::<kept-root-mod>::` path points at an item that moved.
* Repo checks (`checks.sh`): owners scan has no sequence row missing (20 foreign missing rows); taxonomy `✏️s/🔌️plugins/🎬️sequence`: 1 mount and 1 tree finding, both `missing Cargo manifest 📦️packages/🦀️rust/Cargo.toml` of the plugin root (existing, same as every other plugin); `lib-wiring-budget` has 2 failing tests, all names foreign (bim model, note, flow brep mesh, puzzle supports, semio object), sequence not listed; I lowered the sequence pin of the ratchet from 802 to 275; both launch files parse (no launch route is needed, no new command).
* wasm32 lane: the combined-layout oracle does not rewrite wasm-only files, so the guest bridge is checked structurally: component.rs sits in the crate with `host`, imports its `protocol` child from the crate below (`pub use …inferences::editor::sequence::wasm::protocol;` inserted by the carve), and the `wasm` wrappers keep their `#[cfg(target_arch = "wasm32")]` in the mirror roots of both crates and in the facade.

## 3. Writer writer

Baseline (R7): lib 155 / 428,307 / 7,483, test 251 / 783,792 / 11,705. The planner (`auto` and `on`) failed with `crate schema exceeds the hard budget: lib 55 files` and `crate host … test 51 files`, plus two cycles.

### 3.1 Findings

* Editor: `editor::writer` (root component, 1,279 lines) held both the shared helpers (`writer_action`, `ui_*`, interaction ids, `writer_io`, `reset_document_effect`) and the app. Commands, windows and panels named the helpers through the root, the root mounted them: an 11-slice cycle (35 files, 2,650 lines).
* Schema: the inline modules of `io/text/snapshot` (`mutations_codec`, `snapshot_codec`) imported `operations::*`, `WriterMutation`, `WriterDiff`, the whole schema root and `WriterDocumentChild` and used none of them; the same in `io/{binary,text}/diff`, `io/binary/mutations`, `schema/operations`. Those imports were the only upward edges of the snapshot and diff families (a 10-group cycle). The test lane added `crate::central_apply`, a kept root file module named by five leaf tests.
* Harness: `unit_tests` (mounted app) lived in the editor root, the tests of ten commands/windows/panels and two root-mounted laws used it from other crates.

### 3.2 Changes (scripts in `F/r7c-tools/`, originals `orig-wri*`)

1. `wri-base.ts`: new layer `✏️editor/🧱️base` (nest mount `editor::writer::base`) with the app id, UI value factories, interaction ids, `writer_io`, `WriterChapterPayload`, `reset_document_effect`; the root imports what it uses; 10 files re-pointed (`crate::editor::writer::NAME` to `…::base::NAME`).
2. `prune-imports.ts` (token scan intersected with rustc's unused-import warnings, or crate-local non-trait leaves): removed the unused local imports listed above (6 files), and two unused glob imports of the text snapshot codec by hand.
3. `central_apply` mount moved from the artifact root to the mutation aggregate (`🧬️schema/🧬️mutations/🦀️.rs`, `#[path = "../../../../../../🌉️apply/🦀️.rs"]`), five leaf tests and `mutate-writer-1` re-pointed to `…::mutations::central_apply` (the vcs precedent).
4. `wri-harness.ts`: the harness is mounted by the artifact root as `#[cfg(test)] pub mod editor_unit_tests`, its two root globs are explicit, 12 files re-pointed; `wri-tests-explicit.ts` made the nine harness-driven command, window and panel tests self-contained (no `super::`), so the carve hoists them to the facade (sequence recipe, §2.2). `jack_ast_to_tree_item` of the document panel widened to `pub` for its test.
5. `--mutation-families=on` (the five leaves become generic, converted 5/5).

### 3.3 Result (cgc7 census after, `F/🗑️generated/r7c-census-wri-after.tsv`, lib files/bytes/lines | test)

| crate | lib | test |
| --- | --- | --- |
| `…-writer-tests` (artifact, snapshot, diff, languages of the schema family) | 23 / 16,252 / 316 | 25 / 20,513 / 432 |
| `…-schema` | 30 / 84,068 / 1,283 | 30 / 84,068 / 1,283 |
| `…-inferences` | 25 / 61,045 / 1,021 | 29 / 64,934 / 1,101 |
| `…-rename-writer` (mutation leaf family) | 19 / 41,050 / 780 | 29 / 53,000 / 1,017 |
| `…-operations` | 16 / 40,697 / 840 | 29 / 111,378 / 1,824 |
| `…-editor` (windows, commands, panels) | 28 / 64,557 / 1,223 | 29 / 67,766 / 1,340 |
| `…-tests-2` (viewer, host/app) | 22 / 152,518 / 2,518 | 29 / 203,083 / 3,278 |
| `…-v1` | 6 / 18,019 / 330 | 8 / 35,813 / 550 |
| facade `semio-s-artifact-writer-writer` | 2 / 17,390 / 286 | 21 / 178,858 / 2,480 |

Before: schema group 74 files, host crate lib 39 files and test 51 files. `crates 10 over-or-error 0`.

### 3.4 Gates

`--verify --oracle` exit 0 (`r7c-wri-verify4.log`: oracle 0/0 native and test, layout 0/0); apply with `--backup=F/🗑️generated/r7c-backup-wri/b` (129 files, 8 crates; first attempt reverted with `revert-apply.py` because the harness had to move before the apply); `--check-applied` exit 0, 0/0 extras (`r7c-wri-check2.log`); `cfgtest.ts` 0 cross-crate cfg(test) references; ctc scan of the eight crates and the facade 0 hits; owners scan no writer row missing; taxonomy `✒️writer` only the plugin-root manifest finding; `lib-wiring-budget` fails only on foreign roots, writer ratchet pin 688 lowered to 308; both launch files parse. Kept root file modules: `graph_manifest` (named by no one), `tests`, `editor_unit_tests`, `hoisted_*` (cfg(test)); the only users of the harness are hoisted.

## 4. Sourcing curation

Baseline (R7): lib 128 / 491,278 / 6,943, test 187 / 754,618 / 10,000. Planner before: `crate schema … lib 98 / 415,370 / 6,100` (105 test files), an 11-slice editor cycle (41 files, 2,853 lines) and a 6-slice schema cycle.

### 4.1 Findings

* Editor: the root component (`editor::sourcing`) held the helpers (io, ids, action and table factories, UI value factories, reset effect) and the app; windows and commands named them through the root. Two further upward uses: the commands and the manifest called `standards::v1::subsets::any::examples()` (a function of the subset root, which itself builds the editor surface) and the `set_contributions` command and a window test named `editor::sourcing::component::SOURCING_CURATION_CONFIG_*` (constants of a root child file).
* Schema: the schema root component (866 lines) held the artifact, the descriptor that embeds 21 facet files, and the whole domain (typology, geometry, modules, document helpers, fixtures). The text snapshot codec used eleven of its items, so the root, which mounts the snapshot, was both above and below it. The aggregate and the shims `diff`/`mutations` (each with a nested `schema` alias) were not recognised as alias shims, so the three leaf families reached the aggregate through a separate slice.
* Harness: `unit_tests` in the editor root, used by seven window and command tests, same as writer.

### 4.2 Changes (scripts in `F/r7c-tools/`, originals `orig-cur*`)

1. `cur-base.ts`: new layer `✏️editor/🧱️base` (nest mount `editor::sourcing::base`) with the io surface, ids, action and table factories, UI value factories and `reset_document_effect`; the five file children of the root import what they use; 9 files re-pointed.
2. Hand edits: `examples()` moved from the subset root to `base` (the subset root and the editor call it there); `SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES` and `..._TEXT_BYTES` moved from the config-store-preparation child to `base` (`pub`), `component::` paths re-pointed; `prune-imports.ts` removed unused local imports of the text codec and the text mutation codec.
3. `cur-domain.py`: the domain regions (typology to fixtures, 757 lines) moved from the schema root into the new child `🧬️schema/🧾️domain` (nest mount `schema::domain`, widened `pub(crate)` to `pub`), the root keeps the artifact, the conversions, the descriptor, `Construction` (it names the mutation aggregate) and `pub use` of the domain at the schema path (an absolute path: a `super::` in a nest child unions all siblings); the text snapshot codec imports the eleven names from `schema::domain` instead of the root glob; the domain tests are mounted by the domain.
4. `cur-harness.ts` + `cur-tests-explicit.ts`: the harness is mounted by the artifact root as `editor_unit_tests`, its globs are explicit, the seven harness-driven tests are self-contained and hoisted to the facade; `sourcing_curation_config_mutation_retained_bytes` and the `pub(super)` items of the root children became `pub` (and `pub use`) because the hoisted tests name them from the facade.
5. Generator: nested alias shims, see 5.2.
6. `--mutation-families=on` (3/3 leaves converted).

### 4.3 Result (cgc7 census after, `F/🗑️generated/r7c-census-cur-final.tsv`, lib files/bytes/lines | test)

| crate | lib | test |
| --- | --- | --- |
| `…-curation-examples` | 27 / 56,825 / 614 | 28 / 57,346 / 626 |
| `…-curation-domain` (domain, snapshot, diff, leaves, io) | 28 / 144,748 / 2,293 | 29 / 148,492 / 2,364 |
| `…-curation-operations` | 23 / 59,254 / 1,115 | 29 / 81,109 / 1,547 |
| `…-curation-schema` (root, descriptor) | 24 / 61,936 / 254 | 28 / 99,761 / 775 |
| `…-curation-inferences` | 27 / 87,532 / 1,498 | 29 / 103,105 / 1,760 |
| `…-curation-tests` (editor, viewer) | 13 / 121,166 / 1,769 | 18 / 130,644 / 1,959 |
| facade `semio-s-artifact-sourcing-curation` | 2 / 16,975 / 278 | 16 / 169,937 / 2,048 |

Before: schema crate 98 files, 415 KB, 6,100 lines (test 105 files). `crates 15 over-or-error 0` (the scope also lists the extension and bridge crates).

### 4.4 Gates

`--verify --oracle` exit 0 (`r7c-cur-verify1.log`: oracle 0/0, layout 0/0); apply `--backup=F/🗑️generated/r7c-backup-cur/b` (83 files, 6 crates); the first `--check-applied` had 1 native and 3 test extras (the `pub(super)` items named by the hoisted tests, two moved constants not imported by `sourcing_app` and the unit test), fixed in place, then `r7c-cur-check2.log` exit 0, 0 / 0; `cfgtest.ts` 0; ctc scan of all curation crates 0 hits (one `missingMods` row is the escaped path `\u{1f4b0}\u{FE0F}backing` of the sqlite snapshot tests, which the scanner does not decode; it exists in every carve of this file); owners scan no curation row missing; taxonomy `🪵️sourcing` only the plugin-root manifest finding; `lib-wiring-budget` fails only on foreign roots, curation ratchet pin 814 lowered to 285; both launch files parse. Kept root file modules: `central_apply` (named only by the external `mutate-curation-1` test, which depends on the facade), `tests`, `editor_unit_tests` (cfg(test)); no lower crate names them. The extension crates (`beams`, `windows`, `slabs`) keep naming `schema::windows`, `schema::SourcingModule` etc., which the schema root exports with the domain `pub use`.

## 5. Generator changes (library authoring module `L/🏗️authoring/📦️crate-tree/`)

5.1 sequence:
1. `🗺️plan`: the planner scanned only the host cfg and the test cfg. Code that only the wasm32 build compiles (a `#[cfg(target_arch = "wasm32")]` mount) produced no edges, so such a module was placed in the lowest crate. New `crateTreeWasmCfg()` (cfg of `wasm32-unknown-unknown`), a third measure `info.wasm` computed only when the artifact names `target_arch = "wasm32"`, scanned for edges, root references, path heads and relative chains.
2. `🗺️plan`, `📝️render`, `🟦️`: the `cfg(...)` attributes of a file module (other than `cfg(test)`) were lost where the carve re-declares the module without its declaration: the facade replacement `pub use`, the woven `pub mod` wrapper, and the mirror wrappers and imported leaves of the crates. New `plan.gates` / `MirrorSpec.gates` (effective path to attribute lines) applied in all of those places.
3. `🟦️`: `withDevDependencyRows`: the artifact package manifest now lists every carved crate as a dev-dependency with the forwarded empty features (the crates already did), so the facade tests see the feature-gated helpers of its carved crates.

5.2 curation:
4. `🗺️plan` `rootAliasModules`: an alias module whose nested modules alias the same targets (`pub mod diff { pub use X::*; pub mod schema { pub use X::*; } }`, the pre-migration shim shape of every artifact root) is a shim; chains through the nested alias are normalised (`unnestShim`). Before, such a module became a slice of its own and every leaf reaching the aggregate through it closed a cycle.

Tests (`L/🧪️tests/📦️crate-tree/🟦️.ts`, 92 pass; lift-kids and mutation-leaf-generics 27 pass): nested alias shims (accepted with equal targets, refused with other targets), `crateTreeWasmCfg`, cfg gates kept by the mirror wrapper and imported leaf (and absent without gates), `withDevDependencyRows`. `tsc --noEmit` over the touched generator files: 0 errors. The wasm scan itself is covered by the sequence carve (the guest bridge moved from the `inferences` crate to the crate with `SequenceHost`), not by a fixture test.

## 7. Open items

* No real compile ran (the compiler gate/launcher; no `COMPILE ADMITTED` seen). Evidence is static: the rustc analysis oracles, the combined-layout differential, the census and `cfgtest.ts`. Compile queue: `bun repo cargo --store=shared --configuration=prototype -- check --tests -p semio-s-artifact-sequence-sequence -p semio-s-artifact-writer-writer -p semio-s-artifact-sourcing-curation --keep-going --message-format=short`, plus `--target wasm32-unknown-unknown` for the sequence guest bridge.
* The combined-layout oracle does not rewrite or analyse wasm32-only files, so the sequence guest bridge (`wasm/component.rs`, `wasm/protocol.rs`, protocol tests) is verified structurally only.
* Real-build risks the oracle cannot see: `SequenceCommand` and the other macro-defined items (`app_commands!`, window-transient macros) resolve only after expansion, so every explicit `use …::retained::SequenceCommand` is unproved until a compile; the retained imports used only inside the `app_commands!` table are reported unused by the oracle.
* Crate names follow the first slice (`-tests` holds `base`/`host` of sequence and the schema artifact of writer, `-editor` holds commands/windows/panels, `-v1` the app layer); same defect as R2, not renamed.
* Canonical testkit idiom (coordinator 14:10) is what sequence uses: `#[cfg(any(test, feature = "testkit"))]` on the owner, `testkit = []`, consumers enable it in `[dev-dependencies]`, no self dev-dependency (checked: the only `path = "."` hits are module attributes).
* Seed/ratchet edits: `lib-wiring-ratchet` pins lowered for the three roots (sequence 802 to 275, writer 688 to 308, curation 814 to 285). No launch route was added.
* Tools: `F/r7c-tools/` (`step.sh`, `checks.sh`, `r7c-census.ts`, `plan-dump.ts`, `plug-oracle.sh`/`seq-oracle.sh`, `cmp-short.py`, `lib-imports.ts`, `prune-imports.ts`, `seq-*`, `wri-*`, `cur-*`, `who-uses*.ts`, `restore-orig.sh`, `revert-apply.py`, `orig-*` snapshots).
