# RESULT OS-RUN-VCS-IO-SPLITS (WP-H, Sonnet executor, prefix `orv`, 2026-10-16 13:35 to 14:35)

Paths are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/` (`M`) unless they start with `.`, `/` or `🧰️`. Tools: `orv-tools/` (originals in `orv-tools/orig/`, the specification in `orv-tools/spec.py`). Logs and sandboxes: `🗑️generated/orv-*` (nothing deleted). No sub-agent, no ticket tool, no modifying git command.

## 1. Verdict

All five files are split and installed. Each root keeps only its header, its imports, its real top-level API (types, exported macros, `pub use` lines), the mounts of its own production sub-modules, the test mounts of the root and a `🔖️Children` wiring block. Everything else moved verbatim into 34 cohesive children `<emoji>️<kebab>/🦀️.rs`, each at most 983 lines (target: about 1,200).

| File | Before | Root now | Children | Largest child |
|---|---:|---:|---:|---|
| `M/🏃️run/🦀️.rs` | 2,463 | 105 | 8 | `🧠️wasmtime-node-host` 970 |
| `M/🌿️vcs/🦀️.rs` | 1,909 | 67 | 7 | `🗃️history-ledger` 870 |
| `M/🎒️pack/🌱️value/🦀️.rs` | 1,898 (1,895 at the start, see section 6) | 75 | 6 | `📖️record-decode` 427 |
| `M/🚪️io/🦀️.rs` (os-io crate root) | 1,736 | 157 | 7 | `🧭️io-mechanism` 983 |
| `M/🧠️neural/⚙️engine/🧱️model/🦀️.rs` | 1,622 | 63 | 6 | `⚙️operator-record` 541 |

Static verification is complete and clean (section 4): the line multiset is lossless, the scoped patched differential is identical in all six configurations (lib, `--test`, `wasm32-unknown-unknown` lib and test, `wasm32-wasip2` lib and test) for all five files, plus a deeper variant for lib and test. **No cargo build or test of the five crates was run**: the real check is queued (section 7). `pack::encode_json_value` behaviour is untouched: `🎒️pack/🌱️value` was split verbatim, with only `pub(super)` widenings and path-literal prefixes (section 2).

## 2. Layout and method


**run**: `M/🏃️run/🦀️.rs` 2463 lines -> root 105 lines plus 8 children

| Child folder | mounted as | raw | active | widened | content |
|---|---|---:|---:|---:|---|
| `🚧️run-errors` | `run_errors` | 139 | 111 | 0 | `🔖️Types` |
| `📡️channel-ports` | `channel_ports` | 135 | 71 | 2 | `🔖️AppChannelHost`, `🔖️MediaCache`, `🔖️BlobStore` |
| `🎞️media` | `media` | 258 | 158 | 8 | `🔖️MediaArtifact`, `🔖️MediaConverters` |
| `🧾️run-context` | `run_context` | 83 | 42 | 1 | `🔖️RunContext` |
| `🗃️space-bundle` | `space_bundle` | 290 | 190 | 1 | `🔖️SpaceBundle` |
| `🧭️topology-and-edges` | `topology_and_edges` | 90 | 59 | 2 | `🔖️Topology`, `🔖️EdgeValidation` |
| `🏃️space-runner` | `space_runner` | 469 | 314 | 4 | `🔖️SpaceRunner` |
| `🧠️wasmtime-node-host` | `wasmtime_node_host` | 970 | 656 | 3 | `🔖️WasmtimeNodeHost`, `🔖️OpeningCommands` |

**vcs**: `M/🌿️vcs/🦀️.rs` 1909 lines -> root 67 lines plus 7 children

| Child folder | mounted as | raw | active | widened | content |
|---|---|---:|---:|---:|---|
| `🪪️checkpoint-records` | `checkpoint_records` | 116 | 48 | 0 | the author, change, composition pin, checkpoint and alternative record |
| `📚️history-page-stack` | `history_page_stack` | 480 | 389 | 14 | the paged, bounded-allocation stack that stores one history catalog an |
| `🪟️group-visibility` | `group_visibility` | 83 | 59 | 7 | `🧩️GroupHistoryVisibility` |
| `🗃️history-ledger` | `history_ledger` | 870 | 726 | 20 | the generation-keyed history ledger, its reservations, iterators and t |
| `🌳️artifact-vcs` | `artifact_vcs` | 89 | 70 | 5 | the artifact version graph with its read view and value conversions. |
| `🚨️vcs-errors` | `vcs_errors` | 196 | 119 | 0 | `🔖️Errors` |
| `🧮️diff-and-mutation` | `diff_and_mutation` | 71 | 34 | 0 | `🔖️CollectionDiff`, `🔖️Identified`, `🔖️Mutation` |

**pack-value**: `M/🎒️pack/🌱️value/🦀️.rs` 1898 lines -> root 75 lines plus 6 children

| Child folder | mounted as | raw | active | widened | content |
|---|---|---:|---:|---:|---|
| `🧱️canonical-forms` | `canonical_forms` | 283 | 218 | 18 | `🔖️Canonical` |
| `📝️record-encode` | `record_encode` | 307 | 252 | 2 | `🔖️Encode` |
| `📖️record-decode` | `record_decode` | 427 | 358 | 23 | `🔖️Decode` |
| `🧮️table-soa` | `table_soa` | 260 | 234 | 9 | `🔖️Table` |
| `🔑️schema-hash` | `schema_graph` | 380 | 335 | 0 | `🔖️SchemaHash` |
| `📄️document-codec` | `document_codec` | 224 | 170 | 0 | `🔖️Document` |

**neural-model**: `M/🧠️neural/⚙️engine/🧱️model/🦀️.rs` 1622 lines -> root 63 lines plus 6 children

| Child folder | mounted as | raw | active | widened | content |
|---|---|---:|---:|---:|---|
| `📚️dictionary` | `dictionary` | 403 | 315 | 1 | `🔖️Dictionary` |
| `🏷️schema-model` | `schema_model` | 252 | 192 | 0 | `🔖️Schema`, `⚠️ Errors`, `🔖️SchemaComponent` |
| `🌳️neuron-tree` | `neuron_tree` | 171 | 141 | 0 | `🔖️Tree`, `🔖️Contract` |
| `⚙️operator-record` | `operator_record` | 541 | 423 | 0 | `🔖️OperatorRecord`, `🔖️Cardinality` |
| `🔏️hash-and-dirty-set` | `hash_and_dirty_set` | 128 | 106 | 0 | `🔖️Cache`, `🔖️DirtyPropagation` |
| `🧮️evaluator` | `evaluator` | 128 | 112 | 0 | `🔖️Evaluator` |

**io**: `M/🚪️io/🦀️.rs` 1736 lines -> root 157 lines plus 7 children

| Child folder | mounted as | raw | active | widened | content |
|---|---|---:|---:|---:|---|
| `📇️composer-entries` | `composer_entries` | 55 | 25 | 0 | the composer entry preflight, registration and listing. |
| `🌉️compose-dispatch` | `compose_dispatch` | 94 | 44 | 0 | `🔖️Dispatch` |
| `🛡️subset-validator` | `subset_validator` | 142 | 74 | 1 | `🔖️SubsetValidator` |
| `🎚️io-fidelity` | `io_fidelity` | 65 | 48 | 0 | `🔖️IoFidelity` |
| `🔌️wire-framing` | `wire_framing` | 193 | 128 | 0 | `🔖️Wire` |
| `📚️format-registry` | `format_registry` | 77 | 39 | 0 | `🔖️FormatCatalog` |
| `🧭️io-mechanism` | `io_mechanism` | 983 | 763 | 0 | `🔖️Traits`, `🔖️ArchiveChildren`, `🔖️DslTxtCarrier`, `🔖️Entry`, `🔖️Registry`, `🔖️Route`, `🔖️Identify` |

"widened" counts the private items, fields and inherent members that got `pub(super)` (lists in `🗑️generated/orv-<name>-widen.tsv`). The count is a name-based superset (a few members share a name with an item of another file), so some widenings are harmless extras.

How it was done (`orv-tools/split.py`, specification in `orv-tools/spec.py`):

1. **Line ranges, claimed once.** Every claimed line has exactly one owner; each cut lies between top-level items and never between an item and its attached doc comments or attributes. `//#region`/`//#endregion` marker lines are not treated as attached comments, so a region stays whole inside one child.
2. **Regions kept.** Every original region marker is present. The only markers added are the `🔖️Children` pair in the roots that use a main block. Where a region mixes imports with items (run `🔖️Types`, 32 to 197), the child claims the start marker plus the item range, and the imports stay in the root. Where an outer region spans several children (vcs `🔖️Schemas`, io `🔖️ErasedRegistry`), the outer marker pair stays in the root and the mounts sit between the markers (spec option `into`).
3. **Imports stay in the root**, each child starts with `use super::*;` (convention of `RESULT-REPO-CRATE-ROOT-SPLITS.md`). Checked: trait imports `as _` reach the children through the glob (probe in the scratchpad, rustc nightly-2026-07-20).
4. **Production mounts stay in the root, cluster tests move.** `#[path]` production modules inside regions (`original_lookup`, `dictionary_projection`, `native_controlled`, `original_failure_projection` in neural; `controlled_schema` in pack-value) stay in the root, so their own `super::` paths keep their meaning; the child sees them through the glob. `#[cfg(test)]` cluster mounts move with their cluster: vcs `group_history_visibility_tests`, io `io_fidelity_tests`, neural `value_type_reference` (its owner `FieldSpec` uses `serde(with = "value_type_reference")`). The root `mod tests`/`fault_params_tests`/`intrinsic_media_tests` mounts stay. Test ids of the moved mounts gain the child module as prefix; a `git grep` over the repo found no exact-name filter that names them.
5. **Relative path literals** in moved code (`#[path = ".."]`, `include!`, `include_str!`, `include_bytes!`) got one extra `../` automatically, because every child sits one directory deeper (the verifier undoes it for the losslessness check). Examples: io `include_str!("../../../../../🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🗄️.sql")`, `include!("../🧪️tests/🔬️io-mechanism-laws/🦀️.rs")`, vcs `#[path = "../🧪️tests/🔬️group-history-visibility/🦀️.rs"]`.
6. **io `io_mechanism`.** It was a 998-line inline module `#[path="."] pub mod io_mechanism { .. }` inside the root. Its body is now the file `M/🚪️io/🧭️io-mechanism/🦀️.rs` (dedented by four spaces outside string and comment tokens), mounted as `#[path = "🧭️io-mechanism/🦀️.rs"] pub mod io_mechanism;` directly under the original doc comment. The path `io::io_mechanism::*` is unchanged. The three lines `#[path="."]`, `pub mod io_mechanism {` and the closing `}` are the only non-moved original lines that no longer exist (listed as intentionally dropped by the verifier). No widening inside it (the module was not split).
7. **Visibility widening: `pub(super)` only, never wider** (planner of `RESULT-REPO-CRATE-ROOT-SPLITS.md`, extended): `others` are the root, the other children and every file mounted below the original file (unit tests, genesis, retirement, `🚪️io` of vcs, lifted io, ...), minus the files a child mounts itself; the anonymous `const _` is skipped. Counts: run 21, vcs 46, pack-value 52, io 1, neural-model 1.
8. **Name hygiene.** Child module names were checked against every item and import of the root and the other children (`orv-tools/namecheck.py`: 0 collisions) and against the 528 literal slug patterns of `📚️library/🔣️taxonomy.json` (`orv-tools/spec.py` names `schema-model`, `neuron-tree`, `compose-dispatch`, `wire-framing`, `format-registry`, `schema-graph` because plain `schema`, `tree`, `dispatch`, `wire` are reserved semantic directory slugs and `format_catalog` is also an imported function of os-io). Emoji: RGI, followed by U+FE0F, already used as folder prefix in the repo (3 to 260 times), unique among the siblings of each directory, none repeats a sibling file.

## 3. Source-text readers and edits outside the five files

Found with `orv-tools/readers.py` (string literals ending in `🦀️.rs` that resolve to a target) plus name searches. Every reader that sliced or searched the old file now reads the module tree (`readModuleTreeSource` / `policyReadModuleTreeSource`, "parent first, then children in declaration order", so the slices and `includes` checks see what the unsplit file held):

| File | Edit |
|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts` | `POLICY_SPLIT_SOURCE_ROOTS` += the five split roots (5 lines). This also makes `policyReadRustPolicySource` and the interactivity audit (`INTERACTIVITY_AUDIT_RUN_FILE`) read the run tree |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏁️run/🟦️.ts` | `vcs` and `runHost` use `policyReadModuleTreeSource` (2 lines; the `channel` line next to them is a peer's edit) |
| `M/🚪️io/📸️typed-snapshot/🧪️tests/🟦️.ts` | 4x `read("../../🦀️.rs")` -> `readModuleTreeSource(new URL("../../🦀️.rs",import.meta.url))` (import was already there) |
| `M/🏃️run/🧪️tests/🎟️original-call/🟦️.ts` | `readFileSync` -> `readModuleTreeSource` + import |
| `M/🔌️plugin/🖥️host/🔔️wake/📃️policy/🧪️tests/🟦️.ts` | run file read through `readModuleTreeSource` + import |
| `M/🔌️plugin/🧠️interpreter/🏃️runtime/🏗️startup/🧪️tests/🟦️.ts` | run file read through `readModuleTreeSource` + import |
| `M/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement/🟦️.ts` | vcs read through `readModuleTreeSource` + import |
| `🧰️framework/🔨️modules/🚪️io/📦️codec/🧪️tests/🟦️.ts` | `original` (os-io root) read through `readModuleTreeSource` + import |

Not edited, only comments or text that remains true: `🎚️config/🧬️schema/🦀️.rs:6`, `🧩️plugin-consistency/🚪️artifact-io/🟦️.ts:470`, the `Cargo.toml` `[lib] path` lines, `🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` (`entry` stays the root). No `?raw` import, no `.vscode` entry and no other fs read of the five files exists.

Reader test baseline (`orv-tools/readers_tests.py baseline`, `bun test ./<file>`, before the install) versus after (`after`, `after2`, `orv-tools/passfail.py`):

| Test file | Baseline | After |
|---|---|---|
| os-io codec | 1 pass | 1 pass |
| os-io typed-snapshot | 7 pass, 1 fail | identical (same failing test and message: `TypedFileSettlement` no longer lives in the os-io text) |
| store retirement | 25 pass, 38 fail | identical in `after`; `after2` shows 3 extra failures `ENOENT .../🏪️store/🧪️tests/🔬️unit/🦀️.rs`: a peer removed that store file meanwhile, not caused by this slice |
| run original-call | 1 pass | 1 pass |
| wake policy | 2 pass, 1 fail | identical |
| interpreter startup | 4 pass | 4 pass |
| `📚️library/🔍️discovery` rust-policy-source-evidence (registry law, includes "every declared split source root names a real file that reads as one text") | not run before | 6 pass, 0 fail |

`orv-tools/predicates.ts` evaluates the two tool-job predicates that read vcs and run (`toolJobArtifactStoreStructuralOwnersExact`, `toolJobPagedIngressExact`) on the old text, the module tree and the single file: `false` in all three (pre-existing, unchanged).

## 4. Verification actually run

| Check | Command | Result |
|---|---|---|
| Baseline | `python3 orv-tools/snapshot.py` (module-tree walk of every crate closure, copied to `🗑️generated/orv-work/<name>/snap`) before any edit; `pack-value` re-snapshotted after a peer's edit | taken |
| Lossless line multiset, item set, regions | `python3 orv-tools/verify.py all real` (also `stage` before the install) | 0 lost non-blank lines in all five; items lost 0 (io: 1 = the doc comment that was fused with the first inner `pub use` of the inline module, now doc + mount); every original region marker present, added only `🔖️Children`; extras are docstrings, `use super::*;` and mounts |
| Patched full-resolution differential | `python3 orv-tools/oracle.py all real1 lib,test,wasmu,wasmu-test,wasip2,wasip2-test` (`rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null [--test] [--target wasm32-unknown-unknown\|wasm32-wasip2] [--cfg 'feature="deflate"' for pack]` from the real crate lib root of a sandbox holding the baseline versus a sandbox holding the live tree; extern globs and `extern crate` patched alike on both sides) | **IDENTICAL in all 30 configurations** on the installed tree: run 276/373/95/192/95/192 errors, vcs 150/240/150/240/150/240, pack-value 243/672/243/672/243/672, io 366/417 (x3), neural-model 172/278/172/277/172/277 (baseline = current in every cell). After the later renames: io and neural-model again identical (`final`, `final2`) |
| Deeper differential (external `use` lines and serde attributes dropped identically, attributes orphaned by the drop removed too) | `ORV_DEEP=1 ORV_DEEP2=1 python3 orv-tools/oracle.py all deep2 lib,test`; io and neural-model once more after the renames (`deepfinal`) | IDENTICAL in all ten cells (run 553/894, vcs 519/634, pack-value 950/1944, io 1245/1455, neural 742/915) |
| Oracle sensitivity | the same differential on trees staged without the widening (`ORV_NOWIDEN=1`) | run 276 -> 306 and 373 -> 416 (E0425), vcs 150 -> 154, pack-value 243 -> 360 (E0425, E0433), io 366 -> 368; neural-model unchanged (its only cross-file private name is the `Dictionary.pairs` field). So the oracle sees top-level names and paths; it cannot see field and method privacy |
| Definition scan | `python3 orv-tools/defscan.py real` | 0 further widenings proposed (idempotent planner) |
| Public API | `python3 orv-tools/apicheck.py real` | public item sets identical (run 20, vcs 31, pack-value 21, io 88 + the `io_mechanism` mount, neural 53); impl blocks 13/60/7/12/56 unchanged; 0 top-level name defined twice |
| Name collisions | `python3 orv-tools/namecheck.py real` | clean |
| Path and include check | in `verify.py` | 9/13/12/15/14 `#[path]`/`include*!` literals resolve (new `../` prefixes included); longest child path 134 bytes (limit 240) |
| Duplicate-mount scan | `python3 orv-tools/dupmounts.py real` | **CLEAN, 0 new**: every mount target is mounted once, every original target is still mounted, every child is mounted once, no tracked source outside the five trees mentions a new child path |
| rustfmt and emoji | `python3 orv-tools/verify_fmt_emoji.py real` (`rustfmt +nightly-2026-07-20 --config-path rustfmt.toml --check --config skip_children=true` per file, RGI check through bun) | 0 parse failures in 39 files; diff-hunk counts (the baselines are not clean): run 21 -> 21, vcs 42 -> 39, pack-value 39 -> 39, neural 39 -> 40 (a regrouped hunk of the root's pre-existing `#[path="..."]` spacing), io 64 -> 61; 34/34 emoji OK |
| Taxonomy slugs | inline script over `🔣️taxonomy.json` | 0 of the 34 slugs is a reserved literal slug |
| Census (own closure: files, bytes, active lines) | `python3 orv-tools/census.py` | run 3 -> 11 files, 234,092 -> 236,919 B, 2,370 -> 2,402 lines; vcs 19 -> 26, 193,730 -> 196,440, 2,373 -> 2,401; **pack-value 12 -> 18, 295,282 -> 297,681, 4,093 -> 4,117**; io 9 -> 16, 188,210 -> 186,667, 1,687 -> 1,710; neural 10 -> 16, 167,745 -> 169,706, 2,167 -> 2,191 |
| Crate-boundaries law | `bun test --timeout 200000 ./🧪️tests/📦️crate-boundaries/🟦️.ts` in `💻️os` | 109 pass, 0 fail (with the default 5 s timeout 4 tests time out under machine load 50; they pass with the longer timeout) |
| Pack wire laws (language-agnostic) | `bun test` of `💻️os/🧪️tests/🎁️pack-value-codecs/🟦️.ts` and `🎒️pack-wire-materialization/🟦️.ts` | 49 pass and 1 pass |

## 5. Not verified, and why

- **No real compile or test.** Machine load was about 50, the headroom gate refuses raw compiles, and Wave 6 prescribes the static route. The differential cannot resolve external crates, so rustc stops before type checking in all five crates and does not report field and method privacy (E0616, E0624). That part rests on the superset name scan and the idempotent definition scan.
- Derive and attribute macros (`RetireOwned`, `ToValue`, `FromValue`, `serde`, `value`) were never expanded: the external derives are neutralised in the sandbox. They are applied to the same items as before, now inside a child module; macros that name items by a path relative to the call site (not `$crate`) would show up only in a real build. The three exported macros of os-io stay in the root and expand against `$crate::` names that the root still provides.
- Test ids of the moved `#[cfg(test)]` mounts changed (section 2, item 4).

## 6. Peers and foreign observations

- A peer edited the tests region of `M/🎒️pack/🌱️value/🦀️.rs` at 13:54 (the `intrinsic_media_wire_laws!` call now takes three `include_str!` arguments). That region stays in the root; I rebased (`orv-tools/orig/pack-value.rs.txt`, new snapshot, new stage) and re-ran the checks before installing. `🌱️value/🏭️schema/🦀️.rs` was also modified by a peer; it is mounted by the root and untouched by me.
- **pack gate:** the own closure of the crate `semio-framework-os-pack` already exceeded the source-boundary gate before this slice (295,282 B, 4,093 active lines against 262,144 B and 4,000); the split adds 6 files, 2.4 KB and 24 lines (docstrings and mounts). The gate is the pack carve's job, not a regression of this slice.
- `🖼️assets/📽️nested-cargo-package-projection/🔣️.json` lists `M/🏃️run/🦀️.rs` as a `wgpu-owner-glue` consumer with `occurrenceCount` 5. The file holds 4 occurrences of the destination form (none of the source token), so the row was already stale; now 1 sits in the root and 3 in `🧠️wasmtime-node-host`. Not edited (authored-consumer contract of the future projection tool).
- Pre-existing failures in reader tests are listed in section 3.

## 7. Real-check commands for the compile queue

All five crates are members of the root workspace (`Cargo.toml` lines 90, 92, 93, 382, 429), so no `--manifest-path` is needed. Hold the compile mutex (`🗑️generated/cargo-mutex.lock`) for one batched call. Prefix of every call:

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 --
```

1. Lib: `check -p semio-framework-os-run -p semio-framework-os-vcs -p semio-framework-os-pack -p semio-framework-os-io -p semio-framework-os-kernel-neural-engine-model --lib --keep-going --message-format=short`
2. Tests (compile): the same with `--tests` instead of `--lib`.
3. Wasm: the lib call with `--target wasm32-unknown-unknown` (run, vcs, pack, io, neural-model all build for it).
4. **Pack tests** (the heavy `pack::encode_json_value` path): `test -p semio-framework-os-pack --lib --keep-going` and once `test -p semio-framework-os-pack --lib --features record-codec-laws`; then `test -p semio-framework-os-vcs --lib`, `test -p semio-framework-os-io --lib`, `test -p semio-framework-os-kernel-neural-engine-model --lib --features testkit`, `test -p semio-framework-os-run --lib`.
5. Direct dependents (`check ... --lib --tests`): `-p semio-framework-os-kernel -p semio-framework-os-command -p semio-framework-os-dsl -p semio-framework-os-spr -p semio-framework-os-store-document -p semio-framework-os-store-space -p semio-framework-os-store-space-history -p semio-framework-os-store-text-io -p semio-framework-os-store-codec-registry -p semio-framework-plugin-host -p semio-framework-os-kernel-neural-engine -p semio-framework-os-kernel-neural-engine-budgeted -p semio-framework-os-kernel-neural-engine-registry -p semio-hub-note -p semio-hub-bim -p semio-s-fem-analyses-model`.
6. If rustc rejects a private field or method, fix it with `pub(super)` at the definition (never with a qualified path). Expected first candidates: struct fields and inherent members listed in `🗑️generated/orv-<name>-widen.tsv` have already been widened; any further hit is a name the scan missed.

## 8. Hand-offs

**New folders for the final taxonomy sweep (34, none registered):**

- `M/🏃️run/`: `🚧️run-errors`, `📡️channel-ports`, `🎞️media`, `🧾️run-context`, `🗃️space-bundle`, `🧭️topology-and-edges`, `🏃️space-runner`, `🧠️wasmtime-node-host`
- `M/🌿️vcs/`: `🪪️checkpoint-records`, `📚️history-page-stack`, `🪟️group-visibility`, `🗃️history-ledger`, `🌳️artifact-vcs`, `🚨️vcs-errors`, `🧮️diff-and-mutation`
- `M/🎒️pack/🌱️value/`: `🧱️canonical-forms`, `📝️record-encode`, `📖️record-decode`, `🧮️table-soa`, `🔑️schema-hash` (mounted as `schema_graph`, because `schema_hash` is a function of the module), `📄️document-codec`
- `M/🚪️io/`: `📇️composer-entries`, `🌉️compose-dispatch`, `🛡️subset-validator`, `🎚️io-fidelity`, `🔌️wire-framing`, `📚️format-registry`, `🧭️io-mechanism`
- `M/🧠️neural/⚙️engine/🧱️model/`: `📚️dictionary`, `🏷️schema-model`, `🌳️neuron-tree`, `⚙️operator-record`, `🔏️hash-and-dirty-set`, `🧮️evaluator`

Other hand-offs: the Go twins and READMEs were not touched; the corpus row for the five crates needs no change (roots unchanged); nothing was added to `.vscode/launch.json` (no new entry point).

## 9. Files

Edited: the five roots (section 1) and the eight consumer files of section 3. Created: the 34 children `<folder>/🦀️.rs`. Tools (all under `orv-tools/`): `rcrlib.py` (lexer, items, targets), `reach.py` (module-tree walker), `snapshot.py`, `spec.py` (the specification), `split.py`, `install.py`, `rename_install.py`, `verify.py`, `oracle.py`, `apicheck.py`, `defscan.py`, `dupmounts.py`, `namecheck.py`, `verify_fmt_emoji.py`, `census.py`, `readers.py`, `readers_tests.py`, `passfail.py`, `predicates.ts`, `emoji_check.py`, `regions.py`, `outline.py`, `tables.md`, `orig/*.rs.txt`. Regenerate: `python3 split.py all` (stage under `🗑️generated/orv-work/<name>/stage`, never the repo), `python3 install.py all` (refuses when a live root differs from `orig/`).
