# Result: CRATE-ROLLOUT-R9B (prefix `r9b`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r9b-tools/` (`step.sh <tag> <cmd...>`, `r9b-census.ts` with `SCOPE`/`TAG`/`DUMP`/`DUMPALL`, `plan-*.ts`, `cfgtest-scan.py`, `g.sh`, `editor-restructure.py`, `checks.sh`, `orig/`). Logs: `F/🗑️generated/r9b-*`. Sandboxes: `/Users/ueli/.cache/r9b/`. Started 2026-10-17 10:17; resumed 12:12 after the usage reset.

## 0. Verdict (kept current)

| Package (`✏️s/🔌️plugins/…/🗿️artifacts/…`) | Verdict | Notes |
| --- | --- | --- |
| demonstrator playground (`🎪️demonstrator/🎪️playground`) | DONE (static) | 4 producer crates plus facade, all inside the gate (lib and test); `--check-applied` exit 0, 0/0 extras; cfg(test) scan clean |
| energy model (`🔋️energy/🔋️model`) | DONE (static) | schema crate 58 -> 5 files, 270,046 -> 102,187 bytes (TypeScript/GraphQL/proto leaves served from their documents); no new crate needed |
| architect program (`🏛️architect/🏛️program`) | DONE (static) | 17 producer crates plus facade, all inside the gate (lib and test); `--check-applied` exit 0, 0/0 extras; cfg(test) scan 0 hits; kept root file modules checked |

## 1. Baseline census (`F/🗑️generated/r9b-census-base0.tsv`, host cfg)

| crate | lib files/bytes/lines | test files/bytes/lines |
| --- | --- | --- |
| semio-s-artifact-demonstrator-playground | 74 / 160,138 / 2,435 OVER(files) | 99 / 220,865 / 3,194 OVER(files) |
| semio-s-artifact-energy-model-schema | 58 / 270,046 / 220 OVER(files) | 59 / 270,974 / 241 OVER(files) |
| semio-s-artifact-architect-program | 179 / 1,604,545 / 23,545 OVER(bytes) | 327 / 2,502,909 / 32,730 OVER(files) |

## 2. Playground (DONE)

- Plan problem 1: schema crate 34 files (limit 32). Fix: `new leaves` on the schema root descriptor served the 12 TypeScript/GraphQL/proto leaves from their documents (`register_schema_leaf_documents` plus `install_default_schema_leaf_reader` with a `no_package_assets` lookup; the guest delivers none). Schema crate 34 -> 23 files.
- Plan problem 2 (layout extra `E0432 crate::central_apply`): the root file module `central_apply` (`bridge_step`) was only used by `io/text/mutations/change-schema`, but a kept root file module cannot be reached from a lower crate. Moved it next to its only caller (`change-schema/🌉️apply/🦀️.rs`, `mod central_apply;` + `use super::central_apply::bridge_step;`); root mount removed.
- Plan problem 3 (test extra `E0603 PlaygroundSnapshot` private import): the sqlite snapshot test named the type through the host's private import; it now imports `schema::snapshot::PlaygroundSnapshot`.
- Verify (`r9b-pg-verify2.log`): exit 0; oracle native 466/466, `--test` 670/628 (onlyCandidate 0); layout 466/466 and 670/628 (onlyCombined 0). Apply (`r9b-pg-apply.log`): 43 files, crates schema, inferences, editor, examples plus facade, 4 owner rows. `--check-applied` (`r9b-pg-check.log`): exit 0, 0/0.
- Repo checks (`r9b-pg-checks.log`): both launch JSONs parse; owners scan has no playground row missing; taxonomy `demonstrator`: only the existing plugin-root manifest finding; `lib-wiring-budget` fails only on foreign roots (svg, wires, forms, brep mesh, gltf schema, semio graph pin).
- Census after (`r9b-census-pg-applied.tsv`, lib / test): facade 1/7,338/157 / 2/29,942/409; editor 20/61,020/1,105 / 28/74,366/1,343; examples 6/28,667/504 / same; inferences 20/25,776/345 / 28/45,628/659; schema 23/44,394/776 / same. All inside the gate.
- cfg(test) cross-crate scan (`F/r9b-tools/cfgtest-scan.py`, `F/🗑️generated/r9b-files-pg-applied.json`): 0 hits (the only cfg(test) items are private `use` lines).
- Kept root file modules after the carve: only `hoisted_snapshot_0` plus the facade tests; `central_apply` no longer exists at the root; no `crate::<kept-root-mod>::` path to a moved item remains (checked with `git grep` for `central_apply`).

## 3. Energy model (DONE)

- The artifact was already carved by hand into 70 crates; only `semio-s-artifact-energy-model-schema` was over: 58 files because its descriptor `include_str!`s 53 mutation-group TypeScript files plus 6 leaf documents (258 KB).
- Generator extensions (tests added): `externalizeFacetLeaves` now serves a `concat!(include_str!(..), ..)` field as several ordered documents and accepts an `id:` constant expression as scope (`L/🏗️authoring/📦️crate-tree/🧾️facet-leaves`; tests `serves a concat of embedded companions as several ordered documents under an id constant`). The schema registry (`🧰️framework/🔨️modules/🧬️schema/📇️registry/🦀️.rs` region `SchemaLeafDocuments`) concatenates the bodies of documents registered for the same `(scope, export, format)` in registration order (test `several_served_documents_of_one_leaf_resolve_as_their_concatenation_in_registration_order`; type-checked with `rustc --emit=metadata` for lib and `--test`, not run).
- Applied `new leaves` to `🧬️schema/🧾️descriptor/🦀️.rs` (53 documents) and installed the default reader with a `no_package_assets` lookup (JSON schemas and the Rust leaves stay embedded). Census after (`r9b-census-en1.tsv`): schema 5 / 102,187 / 234 lib, 6 / 103,115 / 255 test.
- Energy schema root stays at 150 lines (`lib-wiring-budget` unchanged).

## 4. Architect program (DONE, static evidence)

Hand source changes (all minimal, each driven by a planner or oracle finding; none leaves a compatibility layer):
- `ARCHITECT_PROGRAM_SCHEMA` moved from the schema root to `schema::snapshot` (its owner), and `normalize_pair` from the schema root to `schema::kernel`; every caller updated (13 and 8 sites). This removed the child-to-parent edges that welded snapshot, diff, mutations and the schema root into one cycle.
- Unused `use ...::mutations::*` / `diff::ProgramDiff` / `snapshot::ProgramSnapshot` lines in the `io/text` snapshot and mutation codecs removed (they were the fake edges snapshot -> mutations/diff).
- Registers: the shared patch algebra (`PatchRow`, `impl_identified_header!`, `impl_patchable!`, the `TraceLink` impl) moved from the registers root into `schema::kernel` (`#[macro_export]` plus `pub use <macro>;` so the macros are importable by module path); the five family wrappers import explicitly instead of `use super::*`; the family modules are `pub mod`. Registers then split by the planner without a cycle.
- `io::sqlite::snapshot`: the 183 KB DDL `include_str!` moved into its own module `🧱️ddl` (`pub use ddl::SCHEMA;`), so it is its own slice and crate instead of 183 KB inside the codec crate.
- `io::tables` and its items widened to `pub` (were `pub(crate)`), so the CSV/TSV serializers and importer stop merging with the editor.
- Editor: the root file (1,567 lines, children referencing parent items) split into `🧱️base` (app id, `ui_*` helpers, interaction constants, `reset_document_effect`) and `🔧️behavior` (the former inline `behavior` module, `include!` path re-based), with every consumer using the explicit `editor::architect::base::` path (tool `r9b-tools/editor-restructure.py`, original in `r9b-tools/orig/`); report window items widened from `pub(crate)` to `pub`; the window-ownership test (it builds the whole app) moved from the register window config to the editor root (`✏️editor/🧪️tests/🔬️window-ownership`, its TypeScript twin stays beside the fixtures); the shared `project_render` test helper is now a private function in each window and panel test file instead of a cfg(test) module of another crate.
- Presence and config components: `mod mutations;` is `pub mod`, the codec files import `presence::mutations::*` instead of `presence::component::mutations::*`.
- `RegisterKey`/`RegisterRow`/`RegisterLabel` are imported from `schema::registers::catalog` (the inline nest re-export is not an item of any crate); one test fixture include rewritten from `env!(CARGO_MANIFEST_DIR)` to a relative `include_str!`.

Generator changes (tests in `L/🧪️tests`, all pass; crate-tree 74, mutation-leaf-generics 9):
- `🧬️mutation-families`: the converted inverse companion writes the payload of a sibling leaf as an absolute path (`super::super::leaf::Payload` was left behind and kept every leaf coupled to its siblings, so 14 leaves stayed one 50-file slice).
- `🧪️test-closure` detach: a detached test that globs its host now carries the host's private glob imports (names reached through `use kernel::*` in the host were lost: 8 oracle extras).
- `🧾️facet-leaves` and the registry (section 3).

Evidence:
- Plan (`r9b-ar-notes4.log`): 16 crates, no problems (mutation families converted 14/14 leaves). `--verify --oracle` (`r9b-ar-verify3.log`, exit 0): trees equal, visibility 0, orphan 0, glue 0, unresolved crates 0; oracle native 4265 vs 4281 and `--test` 6862 vs 6865 (onlyCandidate 0, the 3 onlyOriginal rows are baseline sqlite names); layout native 4130/4146 and test 6727/6730 (onlyCombined 0).
- Apply (`r9b-ar-apply.log`, exit 0): 179 files, 17 crates, owner rows for each. The first `--check-applied` (`r9b-ar-check.log`, exit 1) found 1 native and 3 test extras: `pub use crate::{macro}` is not resolvable in the combined mount, and the schema unit test lost `use super::*`. Fixed in place (`pub use <macro>;`, restored glob) and re-checked: `r9b-ar-check2.log` exit 0, native 4261/4281, test 6826/6863, onlyCombined 0/0, admission, shared, sharedForeign empty.
- Census after (`r9b-census-ar-applied.tsv`, lib files/bytes/lines | test): facade 1/16,577/274 | 4/61,393/574; architect (editor) 2/74,309/1,061 | 5/168,844/2,320; panels 11/89,443/1,601 | 18/130,259/2,287; config 6/18,811/390; presence 6/16,141/344; inferences 14/204,938/3,420 | 16/208,608/3,517; schema 22/163,226/1,123 | 26/172,875/1,282; snapshot 14/220,284/1,656 | 16/227,778/1,699; registers 6/62,529/1,453 | 8/63,373/1,481; context-requirements 8/124,876/2,915; program-structure 7/107,420/2,516; catalog 13/136,656/1,997 | 25/220,655/3,482; mutations 11/56,060/1,038 | 28/219,620/3,145; mut-any 28/47,409/792; tests-exchange 21/165,692/3,070 | 26/189,958/3,507; tests 10/226,284/906 | 11/228,143/939; tests-2 24/55,864/230 | 26/57,199/270; tests-3 2/3,347/62 | 6/192,932/3,502. `crates 18 over-or-error 0`. Before: lib 179/1,604,545/23,545, test 327/2,502,909/32,730.
- Repo checks (`r9b-ar-checks.log`, `r9b-owners.log`): both launch JSONs parse; owners scan has no architect/demonstrator/energy row missing (the 16 missing rows are foreign); taxonomy `architect` and `energy`: only the existing plugin-root manifest finding; `lib-wiring-budget`: foreign breaches only (svg, presentation, gismap, forms, shooting, brep mesh, 5d support).
- cfg(test) cross-crate scan (`cfgtest-scan.py`, `r9b-files-ar-applied.json`): the two hits are two crates each declaring their own `contract_vectors` module (same name, different path); no crate references another crate's cfg(test) item. Kept root file modules: only `art_program_demo_tests`, `tests`, `hoisted_snapshot_0` (cfg(test), referenced by nobody); no `crate::<kept-root-mod>::` path to a moved item.

## 5. Open items and risks

- No real compile ran (compiler ingress gate). Real-build risks the resolution oracle cannot see: the `#[macro_export]` macros `impl_patchable!`/`impl_identified_header!` used across the registers family crates (resolved by name through `pub use <macro>;` and glob imports; verified only with a one-file `rustc` check), and the registry change (type-checked, not run). Command for the compile queue: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-s-artifact-architect-program -p semio-s-artifact-architect-program-registers -p semio-s-artifact-demonstrator-playground -p semio-s-artifact-energy-model-schema -p semio-framework-schema-registry --tests --keep-going --message-format=short`.
- `new crates --backup=<dir>` joins the dir onto the repository root; an absolute path creates `Users/ueli/...` under the repo root (my two backups were moved back to `F/🗑️generated/r9b-backup-{ar,pg}`; the stray `Users/` tree at the root also holds 486 files of other agents and was left alone). Pass a relative path.
- Hand-written crate-name quirks (`tests`, `tests-2`, `tests-3`, `tests-exchange`, `mut-any`) come from the generator's naming of the first slice; not renamed.
