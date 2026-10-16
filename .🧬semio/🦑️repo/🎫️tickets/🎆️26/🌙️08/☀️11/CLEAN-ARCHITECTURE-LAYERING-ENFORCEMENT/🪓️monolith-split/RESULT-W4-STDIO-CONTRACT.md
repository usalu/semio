# Wave 4 Result: `semio-s-artifact-stdio-contract` (11 producers behind a 57-line facade)

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command (the harness auto-stages; I ran only read-only git), no ticket tool (the ticket is left to the coordinator).
Brief and rules followed: `📓️WAVE4-BRIEF.md` (incl. the god-object rule), `📓️EXECUTOR-RULES.md`, `📓️CARVING-RULES.md`, `📓️RESOLUTION-ORACLE.md`, `PRODUCER-CRATE-RECIPE.md`, `AUDIT-PLUGIN-CRATE-CARVING.md` §4 to §6 (WP-HUB), `RESULT-PLUGIN-CROSS-EDGES.md`, `RESULT-W4-CANVAS.md` and `RESULT-CARVE-PETS.md` as worked examples.
`C` = `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract` (the node of the crate), `S` = `✏️s/🔌️plugins/🗄️stdio` (its Cargo workspace).
Tools and logs: `stdioc-tools/` (stage scripts, oracles, verifier) and `🗑️generated/stdioc-*.log`.

## 0. Verdict

The hub crate is carved. `semio-s-artifact-stdio-contract` keeps its name, its path and every public path the 108 direct dependents use, and is now a 57-line facade with explicit `pub use` items (no glob, no mount). Its former 9,967 lines (19 files, 546,250 bytes; test closure 13,405 lines in 62 files) are spread over 11 producer crates, every one of them inside 3,500 lines, 28 files and 230,000 bytes for both the lib and the test closure (largest: 3,096 lines in the details test closure, 17 files in the assembly test closure, 197,359 bytes in the patch test closure).

Nothing was compiled or executed: the sanctioned launcher is down (`Finite original maintenance authority required`, tried at start and at end), and the Nx producer route failed twice before the project graph, on peer faults (section 6.9). Everything below is therefore analysis-only evidence: a full-resolution differential against the original monolith (per crate and as one union crate, native, test, `wasm32-wasip2` and `wasm32-unknown-unknown`), seeded sensitivity checks, a definition scan, an orphan scan, losslessness, rustfmt, a static verifier and the census. The first real `check` and `test` of the 12 packages is still the actual verdict.

No dependent was edited: the 694 Rust files outside the contract tree that use the crate resolve against the facade (section 5 and 6.3).

## 1. What the investigation found

- The crate was five hand-structured modules glued by `#[path]` (no generator output: no `🤖️generated` input, no generator header, the plugin-wave-0 generator work does not own any of it). Of its 9,967 lines only five files exceeded 1,200 lines: the root `🦀️.rs` (1,735 active lines), `✏️editing` (1,517), `🪟️details` (2,118), `🩹️patch` (1,327) and `📐️part21` (1,284). The root held three unrelated concerns (wire helpers, artifact assembly, window-kit conventions) behind one `pub use` list.
- **Hidden cycle.** The root's window-kit region called `editing::snapshot_edit_source`, and the editing core called the window-kit's `load_example_effect`. A plain "one crate per module" split would have been cyclic. The cause is that `snapshot_edit_source` is a generic JSON print of a snapshot. It now lives in the wire crate (new region `🧾️SnapshotSource`), below both. `editing::snapshot_edit_source` is still a public path, re-exported by the facade.
- **Macros.** Six `#[macro_export]` macros expanded through `$crate::editing::…`, `$crate::list_delta::…` and `$crate::kernel/value/diagnostic`. They now expand through the crate that defines them. The details macro `snapshot_details_editor_support!` used nothing from the details module (only editing core items), so it moved into the editing core. Dependents keep invoking all six by their old facade names.
- **Pre-existing defects found (not caused by this carve):**
  - `🧪️tests/📇️contributions/🟦️.ts` loaded its removal vectors from `../../🧫️fixtures/…`, a directory that does not exist, so the TS twin could never run. I fixed the three paths (now 23 checks pass).
  - `✏️editing/♻️close/🧪️tests/🟦️.ts` (`runOriginalCopyCloseChecks`) fails two source-text assertions about `🎬️media-export/🦀️.rs` (`close.indexOf(…)<close.indexOf("self.cursor.take()")` and the dispose twin). I proved this on the pristine copy of the original tree: the assertion row is `true,true,true,true,false,false,true,true,true` before and after. The file is untouched by this carve.
  - The general-assembly "deletion proof" in `S/📦️packages/🦀️rust/📜️script.ts` rejected every `semio-s-artifact-stdio-*` crate except the facade as a "concrete artifact". It would have rejected all 11 producers, so the rule now also admits the `semio-s-artifact-stdio-contract-` prefix.

## 2. Resulting layout

Downward DAG (normal edges; dev edges marked). `verify.ts` runs a DFS over normal and dev edges: no cycle, no producer depends on the facade.

```
payload-wire ──> window-kit ──> editing ──> editing-patch
      │               │            └──────> editing-details (also -> wire, window-kit)
      └─(dev)─────────┴─ editing (tests)
testing ──(dev)──> part21        assembly   list-delta   media-export   native-print      (leaves)
semio-s-artifact-stdio-contract (facade) ──> all eleven (testing optional, behind feature `testing`)
```

| Crate (`semio-s-artifact-stdio-contract-…`) | Node under `C` (lib root `<node>/🦀️.rs`, package `<node>/📦️packages/🦀️rust`) | Content | Depends on |
| --- | --- | --- | --- |
| `wire` | `🧾️payload-wire` (new) | operation payload JSON, apply-mutation helpers, draft splices, ordered rows, double-option decoder, `snapshot_edit_source`, Base64, semantic fingerprint, `impl_serde_op_codec!` | none |
| `window-kit` | `🪟️window-kit` (new) | example switching, argument readers, addressed/structural/stable tables, revisions, media import, `window_kit_splices_argument` | wire |
| `assembly` | `🏗️assembly` (new) + mounted `🪜️definition-hierarchy` and `📇️registry/🦀️.rs` | schema definitions, executable declarations, native codec receipts, capability ledgers, `StdioArtifactDefinition`, `ContributionRegistry`, `STDIO_REGISTRY_SCHEMA_DOCUMENTS` | none |
| `editing` | `✏️editing` | edit events and pointer operations, schema validation, edit actions and tool-job factories, retained copies, `rules`, `raster`, `bytes`, four editing macros plus the details macro | window-kit; dev: wire |
| `editing-patch` | `✏️editing/🩹️patch` | `SnapshotPatch`, apply, inverse, input schema, borrowed codec | editing |
| `editing-details` | `✏️editing/🪟️details` | schema-driven details provider and window controls | editing, window-kit, wire |
| `part21` | `📐️part21` | ISO 10303-21 codec and controlled construction | dev: testing |
| `list-delta` | `🪡️list-delta` | positional list-delta wire types and `stdio_list_delta!` | none |
| `media-export` | `🎬️media-export` | incremental media serializer lifecycle | none |
| `native-print` | `🖨️native-print` | original-caller native text printing | none |
| `testing` | `🧰️testing` | original caller authorities for tests | none |
| `semio-s-artifact-stdio-contract` (facade) | `C` (lib `C/🦀️.rs`, package `C/📦️packages/🦀️rust`) | explicit `pub use` of every formerly public name; inline `pub mod editing` that merges three crates; two integration tests of the facade API (`details_arena_headroom`, `table_arena_headroom`) | all eleven |

God-object rule: no inherent `impl` spans crates (`orphan.py`: only two foreign-trait-on-local-type impls, `From<MutationRefusal> for String` and `From<Part21Error> for ValueError`, both legal and both unchanged). `DslSnapshotDetailsProvider`, `InversePlanner` and the Part-21 builders keep their data and impl in one crate.

Why 11 and not fewer: the editing family cannot be one crate (5,900 lines), the details and patch test closures are as large as their sources, and the root's three concerns are independent. The small leaves (`native-print` 37 lines, `list-delta` 195) stay their own crates because each is a separate domain node with its own public module path.

## 3. Census (canonical `readRustSourceClosureV1`, `stdioc-tools/census.ts`, host cfg, `debug_assertions`; limits 4000 lines / 32 files / 262,144 bytes; target 3,500 / 28 / 230,000)

Before (`🗑️generated/stdioc-census-before.log`): lib 19 files / 546,250 bytes / 9,967 lines (ratio 2.49 on lines, OVER); test 62 / 893,853 / 13,405 (OVER).

After (`🗑️generated/stdioc-census-after.log`; the `testing` feature adds nothing beyond `test`). Every row is below the target:

| Crate | Lib: files / bytes / lines | Test closure (cfg `test`): files / bytes / lines |
| --- | --- | --- |
| facade | 1 / 6,431 / 57 | 1 / 6,431 / 57 |
| `wire` | 1 / 17,085 / 302 | 2 / 17,386 / 313 |
| `window-kit` | 1 / 33,957 / 517 | 3 / 35,422 / 536 |
| `assembly` | 4 / 76,360 / 1,332 (incl. the 6,082-byte schema) | 17 / 125,951 / 1,600 |
| `editing` | 4 / 122,143 / 2,359 | 13 / 186,165 / 3,070 |
| `editing-patch` | 3 / 88,212 / 1,476 | 7 / 197,359 / 2,125 |
| `editing-details` | 1 / 99,503 / 2,031 | 11 / 170,856 / 3,096 |
| `part21` | 4 / 75,379 / 1,473 | 9 / 118,391 / 2,050 |
| `list-delta` | 1 / 10,607 / 195 | 1 / 10,607 / 195 |
| `media-export` | 1 / 20,074 / 245 | 1 / 20,074 / 245 |
| `native-print` | 1 / 2,283 / 37 | 1 / 2,283 / 37 |
| `testing` | 1 / 8,545 / 149 | 1 / 8,545 / 149 |

The dependents' first-party closures now pass through a 57-line facade and eleven in-limit producers instead of one over-limit crate, so this hub should no longer count as an oversized dependency for the 108 direct dependents (not re-measured: the dependents' own census rows are unchanged by this carve).

## 4. Files changed

Created (each Rust file starts with an emoji docstring): `C/🧾️payload-wire/🦀️.rs`, `C/🪟️window-kit/🦀️.rs`, `C/🏗️assembly/🦀️.rs`, `C/🧪️tests/🧾️payload-wire/🦀️.rs`, `C/🧪️tests/🪟️window-kit/🦀️.rs`, and for each of the 11 producers `<node>/📦️packages/🦀️rust/Cargo.toml` plus `📋️project.json`.

Moved without change of content except the intended edits (`lossless.py`: 12,808 non-blank baseline lines against 12,867 after; the 134 baseline lines that differ are exactly the `crate::`/`super::` path rewrites, mounts, the facade, the `$crate` rewrites, the moved allocator and the widened items, `🗑️generated/stdioc-lossless.log`):
- `C/🦀️.rs` (1,742 lines) became the facade. Its body is in `🧾️payload-wire`, `🪟️window-kit` and `🏗️assembly`. `include_str!("../🧬️schema/🔣️.json")` became `../../🧬️schema/🔣️.json`.
- `C/✏️editing/🦀️.rs`: details and patch mounts and re-exports removed, `snapshot_edit_source` removed (now wire), `snapshot_details_editor_support!` added, every `$crate::editing::` prefix of the macros dropped, `use crate::{kernel, value_derive}` replaced by crate imports, the test-only `#[global_allocator]` moved in from the old root, and seven helpers widened from private to `pub` because patch and tests now cross a crate edge (`decode_pointer`, `array_index`, `validate_value`, `values_equivalent`, `snapshot_schema_descriptor_for_dialect`, `SNAPSHOT_SCHEMA_VALIDATORS`, `check_publication_limits`). The facade does not re-export them.
- `C/✏️editing/🩹️patch/🦀️.rs` and `🧪️tests/🦀️.rs`, `C/✏️editing/🪟️details/🦀️.rs` and `🧪️tests/🔬️unit/🦀️.rs`: `super::`/`crate::` paths became imports from the editing, window-kit and wire crates; the details macro left.
- `C/📐️part21/🦀️.rs` (crate imports, test-only `#[global_allocator]` because its controlled tests meter the heap), `C/📐️part21/🧪️tests/🚦️sqlite-cohort/🦀️.rs` (18 `crate::testing::decode_control` calls now import the testing crate), `C/🪡️list-delta/🦀️.rs` (16 `$crate::list_delta::` prefixes dropped, `pub use … as kernel` added for the macro).
- `C/🧪️tests/🔬️unit/🦀️.rs` split into three files by subject (assembly, wire, window-kit). `C/🧪️tests/📇️contributions/🟦️.ts`: reads the assembly file instead of the facade and has the three fixture paths fixed.
- `C/📦️packages/🦀️rust/Cargo.toml` (facade manifest over the producers, `testing = ["dep:…-testing"]`, the two `[[test]]` kept), `C/📦️packages/🦀️rust/📜️script.ts` (test and check run all 12 packages through the existing `packages` option; the ownership laws are split per package: `controlled::tests::…` in part21 and `tests::retained_copy_…` in editing).

Registrations (recipe steps 6, 8, 9, 10): 
- `S/Cargo.toml`: 11 members, byte-sorted (13 rows, verified sorted). Only the `members` block changed; the peer's staged profile rows are untouched.
- `📋️build-owners.json`: 11 rows `@semio-tech/stdio-artifact-contract-<x>-rs`, lexical, `root` = package dir, `packages: []`.
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: 66 routes each (6 per producer, names `🦀️ <crate> 🏗️build|🔎️check|🧪️test 🛠️prototype|🚀️release`, groups `4_build`/`4_gate`/`4_test`), inserted after the contract gate route. Order keys are `900.70001…`, `922.70001…`, `903.70001…`. Two peer fleets had claimed the nearer bands while I worked (`922.05xxx`/`903.59xxx` and `922.06xxx`/`903.60xxx` collided with `ui-runtime` and `os-flow` routes), so I moved to a far band and `verify.ts` asserts no order key is reused in either file. The `run-many … stdio-*-rs` sweep route now excludes `@semio-tech/stdio-artifact-contract*` (it used to exclude only the contract itself).
- `🔒️dependencies.json`: the users of `base64`, `json-patch` and `serde_json` moved from the old contract manifest to the manifests that declare them now.
- `S/📦️packages/🦀️rust/📜️script.ts`: the deletion-proof prefix rule (section 1).
- No corpus row (the boundary corpus is UI-only), no `project.json` targets (inferred), no `rust-source-boundary` keys (no plugin manifest declares them, `plugin-census.csv`).

Consumers re-pointed: none. 694 Rust files use `semio_s_artifact_stdio_contract`; `workspace = true` and `features = ["testing"]` in their manifests keep working because the facade keeps the name, the path and the feature.

## 5. Public surface

`api_uses.py` extracts every path the dependents use (167 distinct, including macros and nested groups). The union probe `use`s each one against the facade and against the original: identical result (section 6.3). Module paths are kept by crate aliases (`pub use <crate> as part21|list_delta|media_export|native_print|testing`, `registry` and `definition_hierarchy` as re-exported modules); the `editing` namespace merges three crates through an inline `pub mod editing` with explicit lists, plus `editing::{patch, details, bytes, raster, rules}` as module paths. Feature `testing` (111 dependents enable it) forwards to the optional testing crate.

## 6. Verification (commands and actual results)

1. **Baseline** before the first edit: census row above; a pristine copy of the whole `📇️registry` tree in the scratchpad (`baseline/`).
2. **Per-crate differential** (`probe.py`, `report.py`): `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null` from each real lib root, sandbox copy, external-crate and attribute errors filtered. 12 roots × 7 configurations (native, native + `feature="testing"`, `--test`, `wasm32-wasip2`, `wasm32-wasip2 --test`, `wasm32-unknown-unknown`, `wasm32-unknown-unknown --test`) against the baseline: the internal error multiset is the single `E0433 semio_framework` of `🎬️media-export` on both sides, 0 otherwise (`🗑️generated/stdioc-per-crate-differential.log`). The run caught and fixed: missing `kernel`/`value` aliases in window-kit and assembly, the wrong `include_str!` depth, and the window-kit to editing cycle.
3. **Union oracle** (`union.py`): the 11 producers, the facade and the two facade integration tests mounted as modules of one crate, `crate::`, `$crate::` and crate names rewritten to module paths, `pub(crate)` narrowed to the owning module, plus the 167 dependent paths as a probe module; the same for the original tree. Result: internal errors 13 against 13 (native, wasip2, wasm) and 4 against 4 (native + testing, `--test` + testing, wasip2 `--test` + testing, wasm `--test` + testing), no differing message (`🗑️generated/stdioc-union-differential.log`). The 4 or 13 errors common to both sides are probe artifacts: associated-function paths and foreign names in the dependent path list (`ArtifactExecutable::from_function_pointer`, `ContributionRegistry::new`, `SemioMemberStoreProducer`), plus the media-export `semio_framework` path; the 9 extra in the non-testing configurations are the feature-gated `testing` module on both sides.
4. **Seeded sensitivity** (`sensitivity.py`): making `decode_pointer`, `check_publication_limits`, `load_example_effect` or `snapshot_edit_source` private, dropping a facade name, and narrowing `parse_part21` to `pub(crate)` each add a new `E0603`/`E0432` to the union: 6 of 6 detected (`🗑️generated/stdioc-sensitivity.log`).
5. **Definition scan** (`defscan.py`) and **orphan scan** (`orphan.py`): no name imported across a producer edge is non-public in its owner; no `impl` of a foreign type or trait lacks a local type (two legal `From<Local> for Foreign` impls). Method and field privacy on types that cross an edge was checked by reading: `SnapshotEditError`, `SnapshotEditEvent`, `EditRules` members used by patch and details are all `pub` already.
6. **Losslessness** (`lossless.py`), **regions** (`regions.py`): the 22 baseline regions all exist and balance in every file; one region was added (`🧾️SnapshotSource`), none dropped.
7. **rustfmt** (`fmtcount.sh`, repo config): all 37 Rust files of the tree parse; formatting diff hunks 80 against 84 in the baseline, so no formatting debt was added (every `use` statement I wrote is rustfmt-formatted, moved lines stay byte-identical). Regeneration is idempotent: re-running the whole stage pipeline on the finished tree changes no byte (`diff -r` empty).
8. **Static verifier** (`verify.ts`): all 12 manifests parse (`Bun.TOML`), package names, `package.workspace` resolves to `S`, lib path equals the node root, every dependency path exists and names the package it is keyed under, members byte-sorted and present, no cycle, owner rows (11, lexical), 11 `project.json`, 66 launch routes in both files with the exact shape and unique order keys, 66 `#[path]`/`include*!` literals resolve, every Rust file is mounted by exactly one producer (35 files), no unowned source, registry root owned by assembly, facade has no glob and no mount, longest path 148 bytes. Result: `ALL STATIC CHECKS PASSED` (`🗑️generated/stdioc-verify.log`). `owners-closure.ts` emulates the graph's "Canonical native dependency owner required" admission: all 217 path-dependency roots reachable from the 12 packages are owner roots except ten peer roots (`◻️2d` engine crates and `🚪️io/🔢️binary/📥️source`), none of them mine.
9. **Launcher and Nx route.** `bun repo cargo --store=shared --configuration=prototype -- check -p semio-s-artifact-stdio-contract[-wire] --message-format=short`: failed after the cache stage with `error: Finite original maintenance authority required` at the start and at the end. `bun repo run @semio-tech/stdio-artifact-contract-wire-rs:check --configuration=prototype --parallel=1 -- --lib --offline`: first attempt `Canonical native dependency owner required: 🧰️framework/🔨️modules/◻️2d/⚙️engine/📦️packages/🦀️rust` (a peer's unowned crate), second attempt `Active graph readers retain required Nx pool headroom` (`🗑️generated/stdioc-nx-check-wire*.log`). No gate verdict and no real `check` or `test` exists for any of the 12 packages.
10. **TS twins** (`bun -e`): `runContributionChecks()` 23 checks pass (after the fixture-path fix and the assembly re-pointing), `runDefinitionHierarchyChecks()` 15 pass, `runOriginalCopyCloseChecks()` fails on the pre-existing media-export assertions (section 1).

## 7. Residual risks and open items

1. **Nothing was compiled.** The oracles prove resolution, privacy of names and the facade surface, not type checking. Likeliest first-compile surprises: unused-import and `unused_qualifications` warnings (workspace lints are `warn`); a trait method that relied on an import the old root provided (every use I could find was fully qualified); the `mutation-testing` kernel feature (copied as a dev-dependency only to the crates whose tests touch the kernel: editing, patch, details); derive expansions that need `semio-framework-value` in every crate that derives (all of them list it).
2. **Lock files.** 134 tracked `Cargo.lock` files contain the old package (`stdio` itself and the artifact workspaces); none was refreshed. They were already stale (the sampled artifact lock misses `semio-framework-job-fault-publication`). On the next working `update --workspace --offline` the facade entry gains the 11 producers and loses its direct framework edges, and each producer gets its own entry (in the stdio lock also with its dev-dependencies). Until then `--locked` resolves fail, as they did for the stale locks.
3. **Macro paths changed.** Dependents only see the macros through the facade, but the expansion now names the defining crate (`$crate::SnapshotEditingEditor`, `$crate::kernel::…`). `$crate` resolves transitively, so no dependent needs a direct dependency. This is the first thing a real dependent compile should confirm (use `bun repo cargo … -- check -p semio-s-artifact-stdio-binary`, which expands `impl_serde_op_codec!`, `stdio_list_delta!` and the editing macros).
4. **Embedded asset.** `STDIO_REGISTRY_SCHEMA_DOCUMENTS` still `include_str!`s the 6,082-byte `$defs/SnapshotPatch` schema. I did not move it to a runtime channel: it is a `const` consumed by ten plugin builders as `&'static str` leaves, the assembly crate is far under the gate with it (4 files, 76,360 bytes), and a runtime channel would add host plumbing for a wasm guest without removing a gate violation. If the rule is meant to apply to such constants, it is a follow-up for the registry builders, not for this carve.
5. **Allocator witnesses.** The test-only `#[global_allocator]` (`HeapWitness`) is now declared in two test binaries, editing and part21, because both crates meter heap allocations in their tests. It was one declaration for the whole crate before. The four ownership laws in `canonical-architecture` are renamed to their new packages; their names were not executed.
6. **Taxonomy.** The three new folders (`🧾️payload-wire`, `🪟️window-kit`, `🏗️assembly`) will show up as `directory-kind-unresolved` in `verify taxonomy report`, like the new folders of the earlier carves. I did not run the 40-minute report.
7. **Public widening inside producers.** Seven editing helpers are `pub` in the editing crate (section 4) and the wire crate exposes `kernel`, `value` and `diagnostic` aliases for its macro. The facade lists names explicitly, so none of this reaches the dependents.
8. **`snapshot_edit_source` moved owner** (wire instead of editing) to break the cycle; the path `editing::snapshot_edit_source` is unchanged through the facade. Two intra-doc links (`window_kit_splices_argument`) were turned into plain code spans; other rustdoc links across the new crate edges were not audited (rustdoc warnings only).
9. **Peer interference seen** (not mine, not touched): the Nx route errors above, a staged peer edit of `S/Cargo.toml` profiles, and many staged `🗿️artifacts/*/Cargo.toml` edits under the same workspace.
10. Not done on purpose: no `[workspace.dependencies]` entries for the producers (nothing outside the facade depends on them), no direct re-pointing of the 108 dependents to the producers (the facade is the contract; the follow-up that would shrink their closures is a separate sweep).

## 8. How to reproduce

All scripts are in `stdioc-tools/` and read the pristine baseline from the scratchpad: `stage1_root.py` (root cut), `stage2_editing.py`, `stage3_leaves.py`, `stage4_manifests.py` (dependency tables derived from the module closure), `stage5_facade.py` (explicit re-export lists from the baseline's public items), `stage6_facade_manifest.py`, `stage7_owners.py`, `stage8_launch.py`, `stage9_dependencies.py`, `fmtuse.py`, `fmtblock.py`; checks: `probe.py`, `report.py`, `union.py`, `sensitivity.py`, `defscan.py`, `orphan.py`, `lossless.py`, `regions.py`, `fmtcount.sh`, `verify.ts`, `owners-closure.ts`, `census.ts`, `api_uses.py`.
