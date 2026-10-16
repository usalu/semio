# 🪞️ Result DUPLICATE-MOUNTS-2: one owner per shared source file (second pass)

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Slice prefix `dm2`. Folder F = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/dm2-tools/` (`modowners.ts` = the unchanged smg scan, `summarize.py`, `dd.ts`+`lib.ts` edit and lock helpers, `g1-host.ts` to `g9-media.ts` appliers, `movetests.ts`, `register_routes.py`, `diff_oracle.py`, `closure.ts`, `checks.py`, `sweep.sh`, `rundiff.sh`; lists `touched.txt` (359 paths) and `new-files.json`). Logs: `F/🗑️generated/dm2-*`. Pre-edit copies of every edited file are in the session scratchpad (`scratchpad/dm2pre/<repo path>`; ticket paths plus repo paths would exceed the length limit). Sources: `AUDIT-WAVE5-RUST.md` §4, `RESULT-SINGLE-MOUNT-AND-GATE.md`, `RESULT-PLUGIN-CRATE-HYGIENE.md` (appeared at 13:42, read before the final classification).

## 0. Verdict

**25 of the 42 shared files now have exactly one owning package. 17 remain by instruction (hand-offs to `pi1`, `okwp8`, `pch`). Five more rows appeared during the run from peers' carves (one renderer bin crate at 13:09, four `os-store-tests-NN` store test crates of `okwp8` at about 14:00); they are not touched.** No crate was compiled: the launcher is refused by the physical-headroom gate (Wave 6), so every claim rests on static evidence (scan, scoped differential, source-reader sweep, rustfmt, path, TOML, JSON and lock checks), listed in section 5.

| Gate | Start (13:00, same `modowners.ts` as smg) | End (14:02) |
|---|---|---|
| Files reached from more than one package | 42 (smg left 45; `pch` fixed 3 more) | **22** = 17 hand-off rows + 5 rows created by peers during the run |
| of which inside the files this slice may touch | 25 | **0** |
| Macro-template mounts introduced | 0 | 1 (`intrinsic_media_wire_laws!`, justified in §3.9b) |
| New crates | 0 | 13 (11 framework with `📋️project.json`, owner row, `🧰️framework/Cargo.toml` member and 6 launch routes each; 2 plugin-artifact crates) |
| `[[test]]` targets moved to their owner | 0 | 31 (process3d 5, lowpoly 20, cad 4, plugin-host 2) |
| Lock files edited (textually, sorted, checked) | 0 | 180 (no new problem against `HEAD`, §5) |

## 1. Baseline and classification (42 files, scan `🗑️generated/dm2-modowners-0.tsv`, summary `dm2-summary-0.txt`)

Rule for the exclusions: `pi1` = `🔌️plugin/🪝️mounts/**`, the WP3 to WP5 roots and the facade; `okwp8` = os store tests and testing trees (the kernel test modules and their companions); `pch` = `🗄️stdio/🗿️artifacts/**` plus the plugins `RESULT-PLUGIN-CRATE-HYGIENE.md` names (wfc, puzzle, procedural, space, stdio; its §3.3 lists the framework helpers it left open).

| # | File (short) | Owners at start | Disposition |
|---|---|---|---|
| 1 | `🔌️plugin/🖥️host/⏱️deadline/🦀️.rs` | host-engine, interpreter | fixed §3.1 |
| 2 | `…/🔔️wake/🎟️root/🦀️.rs` | host-wake, interpreter | fixed §3.1 |
| 3 | `…/🖥️host/🧪️tests/🛂️receiving/🦀️.rs` | host-operation, interpreter-runtime | fixed §3.1 |
| 4 | `…/🏃️runtime/🧪️tests/🧫️fixtures/🌱️channel/🦀️.rs` | interpreter-runtime, plugin-host | fixed §3.1 |
| 5-6 | `⏱️trace/…/📥️requests/🦀️.rs` and its `🧪️tests` | trace-memory-testing, mcp-foundation | fixed §3.2 |
| 7 | `🏃️process/…/🧮️compiler/🦀️.rs` | value-derive, dsl-derive | fixed §3.3 |
| 8 | `🧭️routing/🏠️workspace/🦀️.rs` | os-run, os-store-sync, repo-test-host | fixed §3.3 |
| 9 | `🖱️ui/🧪️testing/🎨️paint-policy/🦀️.rs` | ui, os-renderer-wgpu | fixed §3.4 |
| 10 | `🖱️ui/🧪️testing/🔤️text-reference/🦀️.rs` | ui, ui-text | fixed §3.4 |
| 11 | `🗣️dsl/…/🏛️ownership/🧪️tests/🦀️.rs` | ui-viewport, artifact-reference | fixed §3.4 |
| 12 | `🌱️value/🧪️testing/💾️allocator/🦀️.rs` | 16 value crates | fixed §3.5 |
| 13 | `🌱️value/♻️retirement/🏭️factory/🧪️testing/🦀️.rs` | value-factory, value | fixed §3.5 |
| 14 | `🌱️value/🧬️retained-clone/🔗️source/🧪️testing/🦀️.rs` | value, retained-map, retained-clone | fixed §3.5 |
| 15 | `🎒️pack/🌱️value/🏭️schema/🗂️index/🦀️.rs` | pack-record-schema, os-pack | fixed §3.6 |
| 16 | `🎒️pack/🌱️value/🧪️tests/🎞️intrinsic-media/🦀️.rs` | pack, os-pack | fixed §3.6 (macro template) |
| 17 | `📡️spr/📜️history/🧪️testing/♻️retirement/🦀️.rs` | os-history, history-record-decode | fixed §3.7 |
| 18 | `🧊️3d/🧪️tests/🧰️support/🦀️.rs` | 9 brep crates | fixed §3.8 |
| 19 | `🧪️test/⚖️transient-preparation-law/🦀️.rs` | writer, reasoning-wires, forms | fixed §3.9 |
| 20 | process3d `✏️editor/🧪️tests/🧰️support/🦀️.rs` | modes, panels, editor | fixed §3.10 |
| 21 | lowpoly `✏️editor/🧪️tests/🧰️support/🦀️.rs` | modes, commands, panels, options, editor | fixed §3.10 |
| 22 | cad `✏️editor/🧪️tests/🧰️support/🦀️.rs` | transform, panels, cad | fixed §3.10 |
| 23 | lowpoly `🧬️mutations/🧪️tests/⚖️laws/🦀️.rs` | mut-paint, mut-geometry, mut-objects | fixed §3.11 |
| 24 | remodel `🧬️mutations/🧪️tests/🔬️vectors/🦀️.rs` | mut-params, mut-media, mut-calib, mut-results | fixed §3.11 |
| 25 | energy `💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` | energy mutations, mutation-fixtures | fixed §3.12 |
| 26 | `🔌️plugin/🧪️testing/🎟️caller/🦀️.rs` | runtime-ops, reactor-patches, plugin facade | hand-off `pi1` |
| 27-28 | `🗣️dsl/…/🏭️producer/🧪️tests`, `🧬️intrinsic-bytes/🧪️tests` | os-kernel, os-dsl-tests | hand-off `okwp8` |
| 29-31 | `🪐️space/🧪️tests/🪶️sqlite/{📏️preflight,🔬️oracle,🫴️native-caller}` | os-kernel with space-collection and space-space (native-caller: dag, workflow, space) | hand-off `okwp8` (kernel test tree) and `pch` (space) |
| 32 | `🏪️store/👥️presence/🧪️testing/♻️retirement/🦀️.rs` | value-snapshot-read, os-kernel, os-store-ephemeral | hand-off `okwp8` |
| 33-39 | procedural `generation3d` helpers (4) and `generation` `🚦️native` template (3) | procedural 2d/3d crates | hand-off `pch` (their §3.3 documents why) |
| 40 | `🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs` | gis, forms, layout, cad-model, stdio xlsx/pdf/docx/pptx/wav/semio, note, workflow-run | hand-off `pch` (stdio is excluded) |
| 41 | `🧪️test/⚖️typed-input-law/🦀️.rs` | writer, sequence, reasoning, forms, cad-model, os-flow, space-collection, space-space | hand-off `pch` (space is excluded) |
| 42 | `🧪️test/🛬️original-snapshot-owners/🦀️.rs` | 18 plugin crates incl. wfc, procedural, puzzle, space, stdio | hand-off `pch` |
| new | `…/🧊️wgpu/⌨️native-entrypoint/📦️modules/🦀️.rs` | native-runtime-reader (new bin crate, a peer's staged add at 13:09), program-bridge | not touched, hand-off to the peer that is carving it |
| new | `🏪️store/📦️codec/🪶️snapshot-capability/{🪶️native-decoding,🪶️native-encoding}/🧪️tests`, `🏪️store/📬️command/♻️retirement/🧪️tests`, `🏪️store/🪆️child/🧪️tests/🛫️encoding` (4 files) | os-kernel, `os-store-tests-10`/`-06` (new, `okwp8`'s carve; seen by the final scan at 14:05) | hand-off `okwp8` |

## 2. Method

Every fix keeps the repo rules: one owner, the others depend on it; no alias or compatibility layer (callers were re-pointed in the same step); no new script files outside `F/dm2-tools`; no comments inside definitions. Three shapes were used, in this order of preference:

1. **Move the test to its owner** when the shared file is a test harness of one crate's stack and the other mounting sites are `[[test]]` targets that already dev-depend on that crate (no new crate, no lock change except dev-edges): §3.1, §3.10, §3.12.
2. **Export from the owner** under `cfg(any(test, feature = "testkit"))` or depend on an existing owner: §3.1, §3.2, §3.4 (text reference), §3.5 (factory fixture).
3. **New single-owner crate** (role `test` or `framework`) when the helper is genuinely shared by sibling crates and does not reach them through the dependency graph in a way that splits types: §3.3 to §3.9 and §3.11. For unit tests mounted inside a crate's own lib a dev-dependency that depends back on that crate would split types; every helper below was checked against that: it names only crates below its consumers, or its consumers mount it from `[[test]]` targets (integration tests link the lib once).

Locks: `lib.ts` `propagateEdges` adds normal edges to every lock that lists the consumer and dev edges only to locks where the consumer is a workspace member (membership computed from the explicit `package.workspace` pointer, the nearest `[workspace]` root and `exclude`); new package blocks carry the sorted dependency list. Region caveat: the plugin-host carve crates (`plugin-host-engine`, `-wake`, `-operation`, …) are absent from every lock file today (stale locks, `RESULT-SINGLE-MOUNT-AND-GATE.md` hand-off 2), so dev-edges to them were not written (20 skip rows in the g1 apply output; the lock writer skips a dependency package that no lock lists); `update --workspace --offline` regenerates them.

## 3. What changed

### 3.1 Plugin host (rows 1 to 4)
* `original_host_deadline` and `original_host_root` moved from the `plugin-interpreter` manifest to `plugin-host-engine` and `plugin-host-wake`; the two test files now import `semio_framework_plugin_host_engine::original_deadline` and `semio_framework_plugin_host_wake::{GuestRelayOriginalRoot, GuestRelayRootPolicy}` instead of mounting the source. Engine gains the dev-dependency `serde_json`.
* `test_native_authority` (row 3): `plugin-interpreter-runtime` no longer mounts it; it re-exports `semio_framework_plugin_host_operation::test_native_authority` (dev-dependency with `testkit`; host-operation does not depend on the runtime). This also fixes a base defect: the mounted file named `semio_framework_os_vcs`, which the runtime does not depend on (base differential: unresolved crate).
* `channel_fixture` (row 4): exported from the runtime as `pub mod channel_fixture` under `any(test, feature = "testkit")` (new feature `testkit`); the runtime's own test and `plugin-host`'s `original_host_factory` test import it; `plugin-host` dev-depends on the runtime with `testkit`.

### 3.2 Requested-allocation observer (rows 5 and 6)
`mcp-foundation` mounted the source of `trace-memory-testing` as `pub mod test_allocation` behind a `testkit` feature. The mount, the feature and the optional and dev dependency on `semio-framework-trace` are gone; `mcp-plugin-runtime` and `mcp` dev-depend on `semio-framework-trace-memory-testing` and name `semio_framework_trace_memory_testing::…` directly (4 files re-pointed, 13 uses). The repo convention (`pack`, `dxf`, test component) is the same.

### 3.3 Proc-macro and routing helpers (rows 7 and 8)
Two self-contained pub modules became crates beside their files (the files stay in place; TS tests read them by path): `semio-framework-native-build-compiler-resources` (dep `semio-framework-hash`; used by `value-derive` and `os-kernel-dsl-derive`, normal edge, about 180 locks each) and `semio-framework-process-routing-workspace` (std only; used by `os-run`, `os-store-sync`, `semio-repo-test-host`). The mount became `use <crate> as compiler_resources;` / `as workspace_identity;`, so `crate::compiler_resources::…` paths keep resolving.

### 3.4 ui and dsl test mechanisms (rows 9 to 11)
* `semio-framework-ui-testing-paint-policy` (std only, `pub fn scan`): dev-dependency of `semio-framework-ui` and `semio-framework-os-renderer-wgpu`.
* The text wrapping reference was an `include!` of two test modules. Its natural owner is `ui-text` (it names `FontAtlas` and `LINE_BREAK_FIT_EPSILON`): it is now `pub mod text_reference` of `ui-text` under `any(test, feature = "testkit")` (new feature `testkit`, `use super::FontAtlas;` added, items `pub`); the unit test imports it from `super`, the wgpu text test from `semio_framework_ui_text::text_reference::KERNED_PANGRAM` (new dev-dependency of `ui` on `ui-text` with `testkit`). This also removes a base defect: an `include!`d file starting with an inner doc comment (E0753).
* `semio-framework-dsl-record-ownership-oracle` (deps `dsl-record`, `serde_json`): dev-dependency of `ui-viewport` and `artifact-reference`.

### 3.5 Value test support (rows 12 to 14)
* `semio-framework-value-testing-allocator`: the `#[global_allocator]` observer that 16 crates (`value`, `value-factory`, `-retirement`, `-retirement-queue`, `-original-aliases`, `-shared-issuance`, `-intrinsic`, `-paged-list`, `-paged-text`, `-ordered`, `-native-encoding`, `-native-decoding`, `-clone-source`, `-retained-map`, `-retained-clone`, `mutation-diff`) mounted privately. Items are `pub`; each lib has `#[cfg(test)] use semio_framework_value_testing_allocator as allocation_testing;` and the existing `crate::allocation_testing::…` paths (95 uses) are untouched. Each crate dev-depends on it, so a test binary links exactly one copy; it is never a normal dependency of a crate that another test binary links.
* `semio-framework-value-clone-source-testing` (deps `value-clone-source`, `value-retirement`, the allocator crate): `FixtureSource` and `FixtureSourceBirth`, used by `value`, `retained-map` and `retained-clone`. It names only crates below its consumers.
* The factory fixture is exported by its owner: `value-factory` has `pub mod original_factory_fixture` under `any(test, feature = "testkit")` (new feature; `OwnedFactory`, `ScalarFactory`, `grant` are `pub`); its own test uses `crate::original_factory_fixture::*`, `value`'s test uses `semio_framework_value_factory::original_factory_fixture::*` with a dev-dependency feature `testkit`.

### 3.6 pack (rows 15 and 16)
* `semio-framework-pack-schema-index` (deps `dsl-record`, `value`): the slot index used by two copies of `controlled_schema` (`pack-record-schema` and `os-pack`). `Key`, `Index` and its four methods are `pub`; both schema files use `semio_framework_pack_schema_index::Index`; normal edge for both consumers (152 and 178 locks). The duplicate `controlled_schema` and `PackSchema*` types in `os-pack` are a separate drift (§6).
* `semio-framework-pack-intrinsic-media-laws`: see the template justification in §3.9b.

### 3.7 History (row 17)
`semio-framework-os-history-testing-retirement` (deps `value`, `trace-memory-testing`; `close_original` is `pub`) replaces two private `test_retirement` mounts in `os-history` and `os-history-record-decode`; both dev-depend on it.

### 3.8 3d (row 18)
`semio-framework-3d-testing-support` (dep `semio-framework-trace`): the tessellation allocation observer. Nine crates (`semio-framework-3d` and the eight brep crates) replaced `pub(crate) mod test_support;` by `pub(crate) use semio_framework_3d_testing_support as test_support;` and dev-depend on it (statics and fns are `pub`).

### 3.9 Laws
* **Transient preparation law (row 19)**: `semio-framework-test-transient-preparation-law` (deps `os-kernel`, `plugin`, `value`, with the feature sets the consumers use); `writer`, `reasoning-wires` and `forms` replaced `mod transient_preparation_law;` by `use semio_framework_test_transient_preparation_law as transient_preparation_law;` and dev-depend on it. This is the generic `transient_preparation_law<O: WindowTransientOwner>` the hygiene result lists as open (§3.3 of that file); the sibling `typed-input-law` stays with `pch` because two of its eight consumers are `space` crates.
### 3.9b The one macro template
* **`semio-framework-pack-intrinsic-media-laws`** (`semio-framework-pack-intrinsic-media-laws`, `intrinsic_media_wire_laws!`). Justification: the file is a conformance law of the neutral value wire (`DecodeOptions`, `decode_value_record_body_exact*`, `PackRefusal`, `EncodeOptions`, `crate::test_allocation`, `use super::*`), run against two independent implementations of the same API, the framework `pack` and the monolith `os-pack` copy. It names items of the *invoking* scope, so it cannot be a function over one crate. It is invoked from `pack`'s `record-laws` test and from `os-pack`'s test module with the three fixtures as `include_str!` literal arguments (the compile-ingress reader rejects `include_str!(concat!(…))`, `closure.ts` error "Unsupported Rust compile expression", so the first draft with a `$root` literal was replaced; `g9-media.ts`). Parse and expansion were proven with a stub (`rustc` over the macro plus an invocation: only name-resolution errors, no macro errors) and the reader sweep accepts the invoking crates (`pack` test `pack_record_laws`: 17 files, 13 embedded). It disappears when `os-pack` stops duplicating `pack` (§6).

### 3.10 Editor harnesses (rows 20 to 22)
The harness (`context`: the app fixture, `settle`, `render`, …) instantiates the whole editor stack of its artifact, and the mounting sub-crate tests already dev-depended on the top editor crate (dev-dependency cycles of integration tests do not split types). So the sub-crate `[[test]]` targets whose closure contains the harness moved to the owning editor crate (`movetests.ts`, the closure decides, test files do not move): process3d 5 targets (`modes` 1, `panels` 4) to `process3d-editor`; lowpoly 20 targets (`modes` 2, `commands` 14, `panels` 3, `options` 1) to `lowpoly-editor`; cad 4 targets (`transform` 1, `panels` 3) to `cad-cad`. The owners gained the dev-dependencies those tests use (process3d: `inferences`; lowpoly: `base64_codec`, `-io`, `-kit`, `-mut-geometry`, `-mut-objects`, `serde`, `semio-framework-ui-contract`; cad: `machine`, `semio-framework-diagnostic`, `-tool-machine`, `-cad-testing`, `-cad-picking`, `semio-framework-ui-contract`); locks got the matching dev-edges. No Rust text changed. No script, launch entry or TS reads a moved target name.

### 3.11 Mutation law libraries (rows 23 and 24)
Moving these targets to the aggregate crate would break 42 leaf tests that read fixtures with `CARGO_MANIFEST_DIR` plus a fixed number of `..` (the group crates are one level deeper), so they became crates: `semio-s-artifact-lowpoly-lowpoly-mut-laws` and `semio-s-artifact-remodel-remodeling-mut-vectors` (plugin-artifact style: member, `[workspace.dependencies]` table, owner row with `Cargo.toml`, no `project.json`). The lowpoly file gained `use semio_framework_os_kernel as protocol;`; the remodel file also, and its fixture root has one more `..` (the crate sits one level deeper than the group crates: `../../../../../../🧪️tests/🧫️fixtures/🧬️mutations`). The seven group test roots replaced `#[path] mod laws/vectors;` by `use <crate> as laws/vectors;`. The helpers depend on the aggregate mutations crate, which depends on the groups; the groups' integration tests dev-depend on the helper (cycle through a `[[test]]`, same as the existing dev-dependency on the aggregate).

### 3.12 Energy (row 25)
`binary_codec` (the codec round trip over `wire_probes()`) is owned by the fixtures crate's `aggregate-laws`. The mount inside `energy-model-mutations` could never resolve `wire_probes` (base differential: `E0425 wire_probes`), so it was a stale duplicate and its `//#region 🧪️Tests` block was removed. `RESULT-PLUGIN-CRATE-HYGIENE.md` §3.3 calls this mount deliberate; the base error contradicts that and the test still runs in the fixtures crate.

### 3.13 Registration
11 framework crates: owner rows (`@semio-tech/<id>-rs`, manifest `📋️project.json`), `🧰️framework/Cargo.toml` members (the history helper in root `Cargo.toml`), 6 launch routes each (66 in `launch.json` and in the seed, unique names and orders, `register_routes.py`). 2 plugin crates: owner rows with `Cargo.toml`, members and workspace dependency tables of their artifacts. Taxonomy: all new directory chains resolve (`smg-tools/taxo.ts`, 13 RESOLVED).

## 4. Hand-offs (not edited by this slice)

| To | Rows | What it needs |
|---|---|---|
| `pi1` | `🔌️plugin/🧪️testing/🎟️caller/🦀️.rs` | owner = the facade's `🧪️testing`; `runtime-ops` and `reactor-patches` should import it through a `testkit` export like §3.1 row 3 |
| `okwp8` | dsl `🏭️producer` and `🧬️intrinsic-bytes` tests; presence `🧪️testing/♻️retirement`; space `📏️preflight`, `🔬️oracle`, `🫴️native-caller` | kernel test trees. `os-dsl-tests` and the space artifacts should dev-depend on one owner (shape 2 or 3); the presence file is the 129-line `Factory`/`observed_close` helper |
| `pch` | procedural helpers (7), `backing`, `typed-input-law`, `original-snapshot-owners` | each wants one framework test crate (their §3.3); `transient-preparation-law` is now the worked example (`semio-framework-test-transient-preparation-law`) |
| peer (renderer carve) | `⌨️native-entrypoint/📦️modules/🦀️.rs` | the staged bin crate `semio-framework-os-native-runtime-reader` mounts the production module of `program-bridge`; either the reader depends on `program-bridge` or `program-bridge` depends on the owner of the module |

## 5. Verification (every command and its real result)

1. Scan: `bun F/dm2-tools/modowners.ts F/🗑️generated dm2-modowners-N.tsv` (cwd repo root): N=0 (start) 42 files, N=1 (after the crates) 18, N=2 (after the template fix, 13:58) **18** (2,501 manifests, 2,831 roots; exactly rows 26 to 42 and the renderer peer row), N=3 (final, 14:05) **22**: the same 18 and the four `os-store-tests-NN` rows of `okwp8`'s store carve that did not exist at 13:58 (`comm` of N=2 and N=3: only those four differ; `python3 dm2-tools/summarize.py`). Inside the files this slice may touch the count is 0 at every step after the last fix.
2. Scoped differential (`dm2-tools/diff_oracle.py`, RESOLUTION-ORACLE method: `rustc +nightly-2026-07-20 --edition 2021 --emit=metadata -o /dev/null`, sandboxes in the scratchpad, extern crates and external globs neutralised the same way on both sides; base = the pre-edit files). 71 runs (70 completed; the `os-renderer-wgpu --test` run stopped in the source reader on the foreign missing fixture, §6), logs `🗑️generated/dm2-diff-{g1,g2,g3,g4,g5,new}.log`, specs `dm2-diff-*.spec`, sandbox logs `dm2-oracle-*`. Result: **no new row except unresolved extern names of the moved owners** (`semio_framework_value_testing_allocator` 16x, `…_clone_source_testing` 3x, `…_3d_testing_support` 9x, `…_os_history_testing_retirement` 2x, `…_pack_schema_index` 4x, `…_native_build_compiler_resources` 2x, `…_process_routing_workspace` 2x, `…_dsl_record_ownership_oracle` 2x, `…_trace_memory_testing` 15x, plugin-host owners 2x, `…_pack_intrinsic_media_laws` 1x). Everything else is "gone": base errors that vanish because the mounted file no longer needs the extern name (for example 47 for `os-pack` tests, 11 each for `ui-viewport` and `artifact-reference`).

   Two groups of rows need a reading:
   * `value --test`: 3 new rows `OwnedFactory`/`grant` not found in `🔗️source/🧪️tests/🏭️factory` come from the neutralised glob `use semio_framework_value_factory::original_factory_fixture::*;` (each name is `pub` in the fixture).
   * `os-kernel-dsl-derive`: 3 new E0277 rows. Control experiment: the base sandbox with its mount replaced by the same unresolved `use … as compiler_resources;` gives 9 E0277 lines, exactly the count of the current sandbox (base 6), so the rows are a type-inference cascade of the unresolved extern, not a regression (`dm2exp` experiment in the scratchpad).

   Per crate (base / now / new / gone), native lib and `--test` where the crate has tests: plugin-interpreter-runtime 137/137/0/0, test 296/286/2/12; plugin-host-wake 85/85/0/0, test 117/117/0/0; plugin-host-engine 14/14/0/0 both; mcp-foundation 168/168/0/0, test 223/214/0/9; mcp-plugin-runtime 512/512/0/0, test 561/561/8/8; mcp 483/483/7/7 (test); value-derive 423/423/1/1; dsl-derive 132/135/4/1; os-run 276/276/0/0; os-store-sync 194/195/1/0; repo-test-host (clean both); ui (clean both, rustc reached emission); ui-viewport test 330/320/1/11; artifact-reference test 97/87/1/11; value-factory 163/163/0/0, test 277/278/1/0; the other value crates (queue, aliases, retirement, sealed, intrinsic, list, paged, ordered, encode, decode, clone-source, mutation-diff) test: +1 each (the allocator crate); value test 458/461/5/2; retained-map test 340/341/2/1; retained-clone test 858/859/2/1; mutation-diff test 22/23/1/0; the nine 3d crates test: 1 new, 2 gone each; os-history test 441/439/1/3; history-record-decode test 159/157/1/3; pack-record-schema lib/test 15/14/1/2; os-pack lib 244/243/1/2, test 717/672/2/47; pack test 105/105/0/0; ui-text lib 38/38/0/0, test 82/81/0/1, `testkit` 38/38/0/0; energy-mutations lib 341/341/0/0, test 343/341/0/2 (gone: `wire_probes`, `semio_framework_async_macros`).
   New crates (current only): compiler-resources 1 error (extern `semio_framework_hash`), routing-workspace, paint-policy, value-testing-allocator: clean (rustc reached metadata emission and only failed on the sandbox's `/dev/rmeta…` temp file), ownership-oracle 11, clone-source-testing 3, 3d-testing-support 2, history-testing-retirement 3, schema-index 2, lowpoly-mut-laws 20, remodel-mut-vectors 13 (one is `CARGO_MANIFEST_DIR` undefined in the sandbox), transient-preparation-law 6: all unresolved extern names.
   Not run: the `--test` closure of `os-renderer-wgpu` (the reader stops on the missing TextEditor fixture, §6), the test roots of the group, editor and `plugin-host` crates (integration tests are outside the oracle's library closure; the moved ones have unchanged text).
3. Source-reader sweep (the compile-ingress `readRustSourceClosureV1`, `dm2-tools/sweep.sh`): all 82 touched packages read lib, bins and `[[test]]` targets (log `🗑️generated/dm2-sweep.log`): **81 ok**, 1 fail = `os-renderer-wgpu` (`Missing Rust source input: …/✏️TextEditor/🧫️fixtures/📝️explicit-draft/🔣️.json`, foreign and pre-existing, also reported by smg).
4. `python3 dm2-tools/checks.py dm2-tools/touched.txt` (359 paths: 81 Rust, 86 Cargo.toml, 180 locks, 12 JSON): rustfmt parse failures 0; `#[path]`/`include!` literals checked 371, missing 0; TOML and JSON parse failures 0; longest new path 160 characters / below 240 bytes; 3 existing test files that I edited are 241, 246 and 248 bytes (pre-existing names, not created here).
5. Lock check (`scratchpad/lockcheck.ts`, against `git show HEAD:<lock>`): 180 locks, packages and dependency lists sorted, every token resolves, no duplicate package: **0 new problems** (4 locks carry pre-existing unresolved tokens, unchanged).
6. Macro proof: `rustc` over `intrinsic_media_wire_laws!` plus an invocation: expansion succeeds (34 unresolved-name errors, none about the macro).
7. `[DEBUG]` lines added: 0 (two existing `[DEBUG]` prints sit on lines I re-pointed in `guest-input-slot.rs`).
8. Not run: any cargo build or test. Exact real check for the compile queue (hold the fleet compile mutex; one launcher call per workspace):
   * root workspace: `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-history -p semio-framework-os-history-record-decode -p semio-framework-os-history-testing-retirement -p semio-framework-os-kernel-dsl-derive -p semio-framework-os-mcp -p semio-framework-os-mcp-foundation -p semio-framework-os-mcp-plugin-runtime -p semio-framework-os-pack -p semio-framework-os-renderer-wgpu -p semio-framework-os-run -p semio-framework-os-store-sync -p semio-framework-plugin-host -p semio-framework-plugin-host-engine -p semio-framework-plugin-host-wake -p semio-framework-plugin-interpreter -p semio-framework-plugin-interpreter-runtime -p semio-repo-test-host --lib --tests --keep-going --message-format=short`
   * framework workspace: same call with `-p semio-framework-3d -p semio-framework-3d-brep-boolean -p semio-framework-3d-brep-construction -p semio-framework-3d-brep-engine -p semio-framework-3d-brep-offset -p semio-framework-3d-brep-representation -p semio-framework-3d-brep-sweep -p semio-framework-3d-brep-tessellation -p semio-framework-3d-brep-validation -p semio-framework-3d-testing-support -p semio-framework-artifact-reference -p semio-framework-dsl-record-ownership-oracle -p semio-framework-mutation-diff -p semio-framework-native-build-compiler-resources -p semio-framework-pack -p semio-framework-pack-intrinsic-media-laws -p semio-framework-pack-record-schema -p semio-framework-pack-schema-index -p semio-framework-process-routing-workspace -p semio-framework-test-transient-preparation-law -p semio-framework-ui -p semio-framework-ui-testing-paint-policy -p semio-framework-ui-text -p semio-framework-ui-viewport -p semio-framework-value -p semio-framework-value-clone-source -p semio-framework-value-clone-source-testing -p semio-framework-value-derive -p semio-framework-value-factory -p semio-framework-value-intrinsic -p semio-framework-value-native-decoding -p semio-framework-value-native-encoding -p semio-framework-value-ordered -p semio-framework-value-original-aliases -p semio-framework-value-paged-list -p semio-framework-value-paged-text -p semio-framework-value-retained-clone -p semio-framework-value-retained-map -p semio-framework-value-retirement -p semio-framework-value-retirement-queue -p semio-framework-value-shared-issuance -p semio-framework-value-testing-allocator --manifest-path 🧰️framework/Cargo.toml`
   * plugin artifact workspaces (same launcher call, each with its own manifest and these arguments after `--`):
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer`: `-- check --manifest-path ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/Cargo.toml -p semio-s-artifact-writer-writer --lib --tests --keep-going --message-format=short`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d`: `-- check --manifest-path ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml -p semio-s-artifact-process-process3d-editor -p semio-s-artifact-process-process3d-modes -p semio-s-artifact-process-process3d-panels --lib --tests --keep-going --message-format=short`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly`: `-- check --manifest-path ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/Cargo.toml -p semio-s-artifact-lowpoly-lowpoly-commands -p semio-s-artifact-lowpoly-lowpoly-editor -p semio-s-artifact-lowpoly-lowpoly-modes -p semio-s-artifact-lowpoly-lowpoly-mut-geometry -p semio-s-artifact-lowpoly-lowpoly-mut-laws -p semio-s-artifact-lowpoly-lowpoly-mut-objects -p semio-s-artifact-lowpoly-lowpoly-mut-paint -p semio-s-artifact-lowpoly-lowpoly-options -p semio-s-artifact-lowpoly-lowpoly-panels --lib --tests --keep-going --message-format=short`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires`: `-- check --manifest-path ✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.toml -p semio-s-artifact-reasoning-wires --lib --tests --keep-going --message-format=short`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms`: `-- check --manifest-path ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.toml -p semio-s-artifact-forms-forms --lib --tests --keep-going --message-format=short`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad`: `-- check --manifest-path ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.toml -p semio-s-artifact-cad-cad -p semio-s-artifact-cad-cad-panels -p semio-s-artifact-cad-cad-transform --lib --tests --keep-going --message-format=short`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling`: `-- check --manifest-path ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml -p semio-s-artifact-remodel-remodeling-mut-calib -p semio-s-artifact-remodel-remodeling-mut-media -p semio-s-artifact-remodel-remodeling-mut-params -p semio-s-artifact-remodel-remodeling-mut-results -p semio-s-artifact-remodel-remodeling-mut-vectors --lib --tests --keep-going --message-format=short`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model`: `-- check --manifest-path ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml -p semio-s-artifact-energy-model-mutations --lib --tests --keep-going --message-format=short`
   Expect the first failures to be the foreign ones in §6 (stale plugin-host locks need `-- update --workspace --offline` first).

## 6. Foreign and pre-existing findings

* `os-pack` has no `crate::test_allocation` and no `trace-memory-testing` dev-dependency, but two of its mounted tests (`schema-hash/storage`, and the intrinsic-media template from `pack`) use it: its test build is red before and after this slice (base differential: `E0433 cannot find test_allocation`, 6x).
* `os-pack` duplicates `PackSchemaShape/Field/Graph` and `controlled_schema` of `pack-record-schema` (two implementations, two type sets); after §3.6 only the slot index is shared. The clean fix is to make `os-pack` re-export `pack-record-schema` (needs a compiler).
* The plugin-host carve crates (`engine`, `wake`, `operation`, …) are missing from all lock files (so the dev-edges of §3.1 were not written).
* `os-renderer-wgpu`: the TextEditor fixture json is absent (reader and test closure fail); the `🧪️tests/🎨️wgpu-paint-policy` edit there is covered by the sweep of the crate only up to that error.
* `RESULT-PLUGIN-CRATE-HYGIENE.md` §3.3 argues the five artifact helpers (process3d, lowpoly, cad, lowpoly laws, remodel vectors) must stay shared because a dependency would split types. That holds for unit tests mounted in a lib; here every mounting site is an integration `[[test]]` target, which links the lib once, so the single owner is sound. Their §3.3 entry for the energy test is contradicted by the base differential (§3.12).
* Three plugin test paths exceed 240 bytes (241, 246, 248; 139 to 163 characters, below the AGENTS 256-character limit): the `⚖️retirement-law` tests of `writer`, `reasoning-wires`, `forms` (pre-existing).

## 7. Files

Touched list: `F/dm2-tools/touched.txt` (359: 81 `.rs`, 86 `Cargo.toml`, 180 `Cargo.lock`, 12 JSON/JSONC); created (24, `F/dm2-tools/new-files.json`): 13 `Cargo.toml`, 11 `📋️project.json`. No `.rs` file was created (the shared files became the lib roots of the new crates). Shared registries edited: `📋️build-owners.json` (13 rows), `🧰️framework/Cargo.toml` (11 members), root `Cargo.toml` (1 member), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (66 routes each), 2 artifact workspace manifests (member and dependency table).
