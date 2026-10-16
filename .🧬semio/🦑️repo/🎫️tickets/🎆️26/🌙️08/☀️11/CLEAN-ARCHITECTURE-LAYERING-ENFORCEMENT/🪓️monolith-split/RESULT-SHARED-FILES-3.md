# 🧩️ Result SHARED-FILES-3: one owning crate for the generation-native template, the generation3d test supports and the framework test helpers (slice `sf3`)

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Run 2026-10-16 15:10 to about 18:30 CEST under machine load 44 to 62. Folder `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/sf3-tools/` (kept; `touched.txt` lists the 358 written paths). Logs: `F/🗑️generated/sf3-*`. Pre-edit copies of every edited source live in the session scratchpad (`sf3pre`, `sf3pre3d`, `sf3pre3d-consumers`, `sf3-native-pre`, `sf3pre-dsl`), because ticket path plus repo path exceeds the length limit.

## 0. Verdict

**Every target file now has exactly one owning package, except the one file under `🔌️plugin/**` that the brief assigns to slice `pi1`.** No crate was compiled: the launcher fails at its own start (section 6), so all claims rest on static evidence.

| Gate | Start (scan 15:08) | End (scan 16:47, `sf3-modowners-3.tsv`) |
|---|---|---|
| Files reached from more than one package | 18 | **1** (`🔌️plugin/🧪️testing/🎟️caller/🦀️.rs`, slice `pi1`) |
| of which in the areas I touched | 17 | **0** |
| New crates | 0 | 13 (8 os/framework crates with `📋️project.json`, owner row, member and 6 launch routes each; 5 plugin crates) |
| Consumer packages re-pointed from `#[path]` mount to a dev-dependency | 0 | 77 (framework helpers and space helpers) + 30 test files of the generation3d supports |
| `new crates <pkg> --check-applied --base=f856dff0813` | generation2d exit 1 (3 shared files) | generation2d **exit 0**, generation3d **exit 0**; all 65 touched plugin artifacts in section 4.7 |

| # | Target | Result |
|---|---|---|
| 1 | Generation-native template (3 plugin files shared by generation2d and 3d) | **Done.** New crate `semio-s-procedural-core-generation-sqlite-native` owns the template and the seven DSL twins; each artifact implements a trait on a local marker type (section 2.1). |
| 2 | Four generation3d test supports | **Done.** Four test crates in the generation3d workspace below their consumers; the catalogue and the registry are passed in by the callers, so no support crate reaches a consumer through normal dependencies (section 2.2). |
| 3 | Framework test helpers (`original-snapshot-owners`, `typed-input-law`, sqlite snapshot `backing`) and the remaining framework files | **Done** for 10 of the 11 framework files; the 11th is the `pi1` file. Three helper crates (60 + 8 + 12 consumer packages), plus presence retirement, three space sqlite helpers, two dsl test files and the native runtime manifest module (section 2.3, 2.5). |
| 4 | `DUPLICATE-MOUNTS-2` hand-offs to `pch` (10 files) | **Done**: procedural helpers (7: targets 1 and 2), `backing`, `typed-input-law`, `original-snapshot-owners`. |
| 5 | `okwp8` hand-offs (6 files) | **Done.** `RESULT-OS-KERNEL-WP8.md` reads `COMPLETE` (rewritten 15:12, nothing listed as in progress), so I took the two dsl kernel test files, the three space sqlite test files and the presence retirement helper (section 2.4). |

## 1. Found state (before the first edit, 15:08)

* `sf3-modowners-0.tsv`: 18 shared files (`dm2-tools/modowners.ts`, the unchanged smg scan): 3 generation-native template files, 4 generation3d supports, 3 framework helpers (`backing` 12 packages, `typed-input-law` 8, `original-snapshot-owners` 60), presence retirement (3), native-entrypoint module (2), plugin caller (3), dsl `producer` and `intrinsic-bytes` (2 each), space `preflight`, `oracle`, `native-caller` (3, 3, 5).
* `RESULT-PLUGIN-CRATE-HYGIENE.md` §3.3 and `RESULT-DUPLICATE-MOUNTS-2.md` §4 were read in full. Rule I used from them: a shared file becomes a crate only if that crate does not reach its consumers through normal dependencies; the lock graph includes dev edges, so I recomputed reachability from the manifests' `[dependencies]` only (section 4.6).
* generation2d `--check-applied` (pch §6): exit 1, only the template as `shared`. generation3d: 4 shared (the supports).

## 2. What changed

### 2.1 Generation-native template (target 1)

Crate `semio-s-procedural-core-generation-sqlite-native` at `✏️s/🔌️plugins/🌀️procedural/🫀️core/🧬️generation/🪶️sqlite/🚦️native` (core workspace, `📦️packages/🦀️rust`, owner row, core member, `[workspace.dependencies]` entry in generation2d and generation3d, dependency of both `…io-text` crates). The template folder already existed beside the core crate; it is now the crate:

| File | Lines | Content |
|---|---:|---|
| `🦀️.rs` (root) | 48 | `GenerationFields` (the eight decoded document fields), trait `GenerationNative`, re-exports; stays under the 150-line wiring budget |
| `🧬️records/🦀️.rs` | 243 | the seven DSL twins (`ValueDsl`, `DictEntryDsl`, `CameraJsonDsl`, `WidgetLayoutDsl`, `SynapseSpecDsl`, `WidgetDsl`, `FormGenerationDsl`) and their conversions, moved from the generation2d snapshot; fields `pub(crate)` |
| `🧱️document/🦀️.rs` | 114 | the former template root, generic over `K: GenerationNative`: `encode::<K>` / `decode::<K>` |
| `📏️rows/🦀️.rs` | 86 | row census; takes `&FlowHostSnapshot` and `&GenerationPlayState` instead of the artifact snapshot |
| `🌱️value/🦀️.rs` | 36 | unchanged |

Trait `GenerationNative`: `type Snapshot`, `ENVELOPE_ID`, `PACK_PREFIX`, `spec_producer()`, `fields(record, control) -> GenerationFields`, `host(&Snapshot)`, `state(&Snapshot)`, `assemble(host, generation)`. Each artifact implements it on a **local marker type** (`Generation2dNative`, `Generation3dNative`) in its `io/📝️text/📸️snapshot/🦀️.rs`. A marker is needed because the snapshot types live in the model crates, so neither the trait (new crate) nor the type (model crate) is local to `io-text` (orphan rule). The per-artifact `Generation{2,3}SnapshotDsl` twin stays in each artifact: its `#[dsl(id=…)]` attribute and the artifacts' handcrafted `ArtifactDsl` impls need a local type, and `fields()` moves its eight fields into `GenerationFields`.

The two artifacts re-export the fifteen names their consumers import (`pub use semio_s_procedural_core_generation_sqlite_native::{…}`), so no consumer path changed (7 files import them from `…io_text::snapshot::*`). Decisions and deviations:

* The record blocks of 2d and 3d are identical after removing doc comments and `semio_framework_value::` qualifiers (checked in `g3_artifact_edits.py`, 7,739 normalised characters). Only `FormGeneration` conversions differed: 2d clones the `DslValue`, 3d calls `ToValue::to_value` / `FromValue::from_value(..).ok()`. `impl ToValue for DslValue` is `self.clone()` and `impl FromValue for DslValue` is `Ok(value)` (`🌱️value/🔁️codec`), so the shared version uses the 2d form with no behaviour change.
* `decode_sqlite_native` / `encode_sqlite_native` were `pub(crate) use controlled::…` yet are called from the separate `…io-sqlite` crates (`…io_text::snapshot::decode_sqlite_native`): a pre-existing visibility error. They are now `pub fn` wrappers over `encode::<K>` / `decode::<K>`.
* The template's private `fn error(e) -> TextError` had no caller and was dropped (dead code).
* The template names flow and playbook types through the contract crates; those only re-export the artifact crates (`os-flow-contract`, `os-playbook-contract`), so the types are identical.
* 2d snapshot: unused imports pruned (`Atom`, `Dictionary`, `NeuralValue`, `CameraJson`, `SynapseSpec`, `WidgetLayout`); `Widget` and `FormGeneration` became `#[cfg(test)]` imports because only the unit tests name them through `use crate::snapshot::*`. generation2d snapshot 565 → 371 lines, generation3d 547 → 365.

### 2.2 The four generation3d test supports (target 2)

Cycle analysis: oracle-support imported `io_text::snapshot::catalogue::catalogue()` (assembled from the 27 category crates) and construction-support imported `geometry::registry::lookup`; both sit above the consumers. The supports now take them as arguments:

| Support (old path under `…/📐️geometry`) | New crate (in `generation3d/🧪️tests/`) | Signature changes |
|---|---|---|
| `🧪️tests/🧰️oracle-support` | `semio-s-artifact-procedural-3d-geometry-oracle-support` (`🔮️geometry-oracle-support`) | `kind_of`, `run_fixture`, `assert_category` take `&'static Catalogue` / `&Catalogue` first |
| `🧪️tests/🧰️construction-support` | `…-geometry-construction-support` (`🏗️…`) | `shape_of`, `inputs_of`, `run_fixture`, `assert_deterministic`, `assert_cancellable`, `first_shape` take `catalogue, lookup`; `pub type Lookup = fn(&str) -> Option<StartFn>`; depends on the oracle crate |
| `⏱️phased-job/🧪️tests/🧰️support` | `…-geometry-phased-job-support` (`⏱️…`) | `run_fixture`, `assert_category`, `refresh` take the catalogue; depends on the oracle crate |
| `🧱️mesh-support/🧪️tests/🧰️mesh-harness` | `…-geometry-mesh-harness` (`🧱️…`) | `run_fixture` takes the catalogue; depends on the oracle crate |

Each crate is `📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}` with a 6-line glue root (`#[path] mod component; pub use component::*;`, the pch pattern) whose domain file stays where it was. The crates sit under `generation3d/🧪️tests/` because the nested location would have been 250 to 268 bytes (limit 240); owner rows, workspace members and `[workspace.dependencies]` entries added.

* Restored pre-existing peer-sweep damage: the three tier-2 supports named `semio_s_artifact_procedural_3d_brep_surface::tests::support::oracle::…` (13 sites), `…brep_feature::tests::support::base::…` (8) and `…mesh_primitive::tests::h::oracle::…` (10), paths into another crate's private `cfg(test)` module (valid at `7af1259dba8` as `oracle::…`, `base::…`). They are `oracle::…` / `base::…` again.
* 30 consumer test files re-pointed (7 oracle, 4 construction, 7 phased, 10 harness including the `mesh-support` lib root, plus `mesh-interchange`, which imported `mesh_support::harness`, a `cfg(test)` item unreachable from another crate, and the `mesh-support` unit test): the mount became `use <crate> as support;`, calls pass `catalogue()` (and `registry::lookup` for construction), the dev-dependency was added to the consumer manifests.
* The catalogue and registry imports stay at the consumers (already their dev-dependencies). The consumer-to-`io-text` dev cycle that existed before remains; it is inherent to the tests needing the full catalogue and is no longer duplicated into the supports.

### 2.3 Framework helpers (targets 3 and 4)

| Helper (framework path) | Crate | Consumer packages (mount sites) |
|---|---|---|
| `🧪️test/🛬️original-snapshot-owners` | `semio-framework-test-original-snapshot-owners` | 60 (69) |
| `🧪️test/⚖️typed-input-law` | `semio-framework-test-typed-input-law` | 8 (8) |
| `🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing` | `semio-framework-io-sqlite-snapshot-testing-backing` | 12 (12) |

Each helper file is the lib root of its crate (`📦️packages/🦀️rust`, `[lib] path = "../../🦀️.rs"`, role `test`, `📋️project.json`, owner row, `🧰️framework/Cargo.toml` member). Every mount `#[path = "…"] mod x;` became `use <crate> as x;` at the same place (cfg and visibility attributes kept). The applier (`g1-framework-helpers.ts`) refuses to write when a consumer is reachable from the helper dependencies in any lock where it is a member or when a mount site is not test-reachable: 0 refusals. Dev-dependencies and lock edges were written for every consumer, 67 locks changed; dev edges are written only where the consumer is a workspace member of that lock, as in `dm2`. `backing` keeps its `#[global_allocator]`; it is linked only by test binaries that name it.

### 2.4 The six `okwp8` hand-offs and presence (target 5)

* **dsl `🏭️producer/🧪️tests` and `🧬️intrinsic-bytes`**: mounted by the kernel `[[test]] sqlite_snapshot_native_admission` (its aggregator) and by the `os-dsl-tests` companion. The aggregator never names the mounted modules, and the companion already runs both files, so the aggregator's duplicate mounts were removed (`mod schema_metadata;`, `mod octets;`) and `🧬️octets/🦀️.rs` (imports plus `mod laws;` only) was deleted. The companion owns both files.
* **space `📏️preflight`, `🔬️oracle`, `🫴️native-caller`**: three os test crates `semio-framework-os-space-testing-sqlite-{preflight,oracle,native-caller}` under the same folders (root workspace). Items `pub(crate)`/`pub(super)` became `pub`; the oracle's unused `use super::store;` was dropped. Consumers: space-history snapshot test (`os-store-tests-07`), `space-collection`, `space-space`, `dag`, workflow run companion and workflow schema laws (3, 3, 5 packages). `native-caller` keeps its `include_str!("🎟️original.json")` beside the lib root.
* **presence retirement**: the helper named `semio_framework_value_snapshot_read::…` (the crate under test of one consumer), so one crate would split types. It is now two parts: `semio-framework-value-testing-presence-retirement` (value tier, `🌱️value/🧪️testing/👥️presence-retirement`; `Factory`, `CountedPresence`, `CLOSE_GRANT`, `observed_step`, `observed_close`, `finish`; dependencies `value` and `trace` only) and the registry-bound witnesses (`observed_box_close`, `admit_returned_string`, `finish_box`, `finish_registry`) as module `presence_test_retirement` of `value-snapshot-read` under `cfg(any(test, feature = "testkit"))` (`📸️snapshot/👁️read/🧪️testing/👥️presence-retirement/🦀️.rs`, `crate::` paths, re-exports the testing crate). `ephemeral` and `os-store-tests-06` use `semio_framework_value_snapshot_read::presence_test_retirement` through a dev-dependency with `features = ["testkit"]`; the three `crate::presence_test_retirement::…` call forms stay valid. The Store-tier file was deleted; the rustfmt list in `🏪️store/🧪️tests/🔬️unit/🟦️.ts` points to the new file (still 7 files). Lock edges are written where `value-snapshot-read` is a workspace member (framework lock) and in the root lock (which activates `testkit` through `ephemeral`).

### 2.5 Native runtime manifest module (the last framework file)

`⌨️native-entrypoint/📦️modules/🦀️.rs` (78 lines, `serde` and `serde_json` only) was mounted by `program-bridge` and by the staged reader binary `semio-framework-os-native-runtime-reader`. New crate `semio-framework-os-renderer-wgpu-native-runtime-modules` (root workspace, owner row, member, routes); `program-bridge` re-exports it as `native_runtime_modules` (`crate::native_runtime_modules::…` and `…program_bridge::native_runtime_modules` stay valid); the reader depends on it. The reader crate itself (a peer's staging from 13:09: no owner row, no workspace member) was only edited at its first lines and its manifest; registering it stays with its owner (section 7).

## 3. Method and tools

`g1-framework-helpers.ts` (framework helpers), `g2_native_files.py`, `g3_artifact_edits.py`, `g4-native-crate.ts` (target 1), `g5a_support_files.py`, `g5b-gen3d-supports.ts` (target 2), `g6-space-helpers.ts`, `g7-presence.ts`, `g8-native-runtime-modules.ts`, `lockdrop.ts`; all build on `dm2-tools/lib.ts` (re-reads every file before the write, sorted lock edits, owner rows through the repository's own generator function). Every applier ran dry first and printed its edit list. Registration of the launch routes: `register_routes.py` (copy of the `dm2` tool with its own crate list): 8 framework crates x 6 routes = 48, unique names, both files parse.

## 4. Verification (commands and real results)

1. **Duplicate-mount scan** (`bun F/dm2-tools/modowners.ts F/🗑️generated sf3-modowners-N.tsv`, cwd repo root): N=0 (15:08) 18 files; N=1 (16:05, after targets 1 to 4 and the WP8 finish) 8; N=3 (16:47, final) **1**, the `🔌️plugin/🧪️testing/🎟️caller` file (`pi1`). Manifests 2,540, roots 2,870.
2. **Scoped differential, generation-native** (`sf3-tools/oracle_native.py`, the `RESOLUTION-ORACLE.md` patches plus the `frs-tools` stub layer, rustc `+nightly-2026-07-20`, metadata to a scratchpad file deleted at once): baseline = pre-edit io-text tree with the original template; current = live tree with the new crate mounted as module `__native` and its crate path rewritten. generation2d and 3d, `lib`, `--test`, `wasm32-unknown-unknown`, `--test wasm32-unknown-unknown`: **8 of 8 runs IDENTICAL** (2d 33/45, 3d 157/176 stub-noise errors on both sides). Warning differential (`--warn`): only `unused import: decode as decode_sqlite_native …` gone and one stub artefact (`store` alias rewritten to an external stub). Negative controls (mutations of the current sandbox only): removing the `Atom` import gave 10x E0433, a changed return type in the trait impl gave E0053, a dropped re-export gave E0425; all detected. Two stub corrections were needed and are recorded in the tool: `os-flow-contract` / `os-playbook-contract` map to the artifact crates they re-export, and the model snapshot is a struct with two `pub` fields (`g3_artifact_edits.py` reads them from `🧬️schema/📸️snapshot`).
3. **Stub-layer typeck of the supports** (`oracle_support.py`; there the stubs leave few resolution errors, so rustc type-checks bodies): the four support files against the pre-edit files. No new E0412/E0425/E0432/E0433/E0603/E0616/E0624 and no new E0061/E0308; the `+current` rows are stub noise (stub `Catalogue` has no `kind`/`category`, stub `StartFn` is a struct). Negative controls: dropping the `catalogue` argument gave `E0061 this function takes 2 arguments but 1 argument was supplied` (construction) and `takes 4 arguments but 3 … argument #1 of type &Catalogue is missing` (phased).
4. **Call-site arity** (`arity_check.py`): 138 consumer calls of the changed support functions pass exactly the declared number of arguments; 0 mismatches; the internal calls of the four supports too.
5. **Line-multiset proof of the template move** (`lossless_native.py`): old template + 2d records block 425 lines, new crate 470; removed 41 / added 86 lines, all explained: 4 mount lines → `mod` lines, `Controlled*` names → `K::…`, struct fields widened to `pub(crate)`, `GenerationFields` and the trait, signature generics, the dropped `fn error`, and `rows::owned` taking host and state. Log `sf3-lossless-native.log`.
6. **Dependency direction**: manifest-only closure (`[dependencies]`) of the four supports: 25 to 27 packages each, reaching none of the 28 consumer packages (brep-*, mesh-*, math-*, analysis-*, phased-job, io-geometry), `io-text` or `geometry`. The seven framework helper crates have lock-based reachability 0 to their consumers (`g1`, `g6`). The core, generation2d and generation3d workspaces: `publish-check` strict **current** (generation2d/3d after one `publish-write`, which only re-sorted `members`; set identical, rest of the manifest identical); `🧰️framework/Cargo.toml` members were unsorted and re-sorted the same way (332 members, set identical).
7. **`new crates <pkg> --check-applied --base=f856dff0813`** for the 65 plugin artifacts whose files I touched: @@CHECKS@@
8. **Other gates.** `python3 F/dm2-tools/checks.py F/sf3-tools/touched.txt` (358 paths: 146 `.rs`, 127 `Cargo.toml`, 73 `Cargo.lock`, 11 JSON/JSONC, 1 TS): TOML and JSON parse failures **0**; rustfmt parse failures 1 (foreign, section 7); `#[path]`/`include!` literals checked 485, 4 reported missing, all written with `\u{…}` escapes the checker does not decode (they exist); longest path 248 bytes / 166 characters (three pre-existing `⚖️retirement-law` test files; every file I created is below 240 bytes). `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` parse as JSON (7,891,931 and 6,181,197 bytes). Lock check (`sf3-tools/lockcheck.ts`, against `git show HEAD:<lock>`): 73 locks, packages and dependency lists sorted, every token resolves, **0 new problems**. Owners scan (`owners-tools/scan.ts`): 2,547 packages, 2,693 owners, `missing` = 1, the peer's unregistered reader crate; none of my 13 crates. Taxonomy (`frs-tools/taxcheck.ts`, whole ancestor chain of 15 new folders): 0 unresolved. `bun test ./🧪️tests/📏️lib-wiring-budget/🟦️.ts` (library root): 2 pass, 1 fail with three breaches in files I did not touch (energy `viewer` 512 lines, `simulation-session` 1,620, remodeling `vectors` root 208); my new plugin roots are 48 and 6 lines.

## 5. Census rows

| Crate | Root | Source files / lines |
|---|---|---|
| `semio-s-procedural-core-generation-sqlite-native` | core | 5 / 527 |
| 4 generation3d support crates | generation3d | domain 4 files (370 / 351 / 500 / 443 lines) + 4 glue roots of 6 lines |
| `semio-framework-test-original-snapshot-owners` | framework | 1 / 115 |
| `semio-framework-test-typed-input-law` | framework | 1 / 66 |
| `semio-framework-io-sqlite-snapshot-testing-backing` | framework | 1 / 247 |
| `semio-framework-value-testing-presence-retirement` | framework | 1 / 91 (+ 1 / 46 in `value-snapshot-read`) |
| `semio-framework-os-space-testing-sqlite-{preflight,oracle,native-caller}` | root | 1 / 93, 1 / 54, 1 / 55 |
| `semio-framework-os-renderer-wgpu-native-runtime-modules` | root | 1 / 78 |

Own source only; the compiler-ingress gate counts the dependency closure, and the kernel-dependent crates are refused by it today for the kernel's own size (unchanged by this slice).

## 6. Real compile (not run) and the commands for the compile queue

* Attempt at 16:14 under the fleet mutex (`F/🗑️generated/sf3-real-check-1.log`; mutex taken and released): `bun repo cargo --store=shared --configuration=prototype -- check -p semio-s-procedural-core-generation-sqlite-native --lib --keep-going --manifest-path …/🫀️core/Cargo.toml` failed before any cargo call with `error: Exact canonical persistent Native publication required` (`🏗️native-build/💾️storage/🧹️cleanup/🎛️command`), the same launcher defect `pch` hit at 15:07.
* Commands: `F/sf3-tools/real-check-new-crates.txt` (five launcher calls: core, generation2d, generation3d, framework, root) and `F/sf3-tools/real-check-consumers.txt` (68 calls, one per workspace, for the 77 consumer packages of the six helper crates). Expect first: `update --workspace --offline` for the locks I edited by hand (73 files, `touched.txt`), the framework lock does not carry the three kernel-dependent helpers (their blocks were removed instead of left dangling), and privacy/import findings only.

## 7. Foreign findings and hand-offs

1. **`pi1`**: `🔌️plugin/🧪️testing/🎟️caller/🦀️.rs` (`runtime-ops`, `reactor-patches`, facade) is the one shared file left; use the `testkit` export shape of `dm2` §3.1.
2. The staged reader crate `semio-framework-os-native-runtime-reader` still has no owner row and no member entry (the only owner-scan miss); its owner should register it (it now depends on the new native-runtime-modules crate, so its `[workspace]` position is the only open point).
3. Pre-existing syntax corruption `assert_eq!*(…)` (a sweep turned `assert_eq!(*x, y)` into a macro call with `*`): 1 site in `brep-topology/🧪️tests/🔬️unit/🦀️.rs` (rustfmt rejects the file before and after my edit) and 7 in `geometry/🧪️tests/🔬️unit/🦀️.rs`. Not touched.
4. The `lib-wiring-budget` breaches (energy, remodeling `vectors`) and the three over-240-byte `⚖️retirement-law` test paths are not mine.
5. `value-snapshot-read` keeps `#[cfg(test)] extern crate self as semio_framework_value_snapshot_read;` (it served the old mount; harmless).
6. The 2d/3d `…io-sqlite` crates called `…io_text::snapshot::{decode,encode}_sqlite_native` across crates while they were `pub(crate)`; with the new `pub fn` wrappers that call compiles; the call sites were not edited.

## 8. Files

Created (13 crates): `…/🚦️native/{🧬️records,🧱️document}/🦀️.rs` + `📦️packages/🦀️rust/Cargo.toml`; four `generation3d/🧪️tests/<emoji>geometry-<support>/📦️packages/🦀️rust/{Cargo.toml,🦀️.rs}`; `Cargo.toml` + `📋️project.json` of the eight framework/os crates (`test-original-snapshot-owners`, `test-typed-input-law`, `io-sqlite-snapshot-testing-backing`, `value-testing-presence-retirement`, `os-space-testing-sqlite-{preflight,oracle,native-caller}`, `renderer-wgpu-native-runtime-modules`); `🌱️value/🧪️testing/👥️presence-retirement/🦀️.rs`; `👁️read/🧪️testing/👥️presence-retirement/🦀️.rs`. Deleted: `🏪️store/👥️presence/🧪️testing/♻️retirement/🦀️.rs`, `…/🪶️native-decoding/🧪️tests/🚪️public/🧬️octets/🦀️.rs`. Rewritten in place: the template root and rows file, the two io-text snapshot files, the four support domain files, 30 generation3d test files, about 90 framework/plugin test files that lost a mount, `📋️build-owners.json`, root/framework/core/generation2d/generation3d `Cargo.toml`, 73 `Cargo.lock`, both launch files, one TS rustfmt list. Complete list: `F/sf3-tools/touched.txt`.
