# 📓️ W2-W-norm-3 — Wire witnesses for EN 1992/1993/1997/1998/1999, EN 1994/1995, ISO 16757, VDI 3805 and the norm results config

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-W-norm-3, 2026-09-30. Contract: `📋️design.md` §6, §11
(including the negative-witness addendum), `🧭️plan.md` "W2-W brief", `📓️w2-s-report.md` F10, F11, F16 and F17.

## 1. Outcome

| Target | Result |
|---|---|
| `schema mutation-payloads`, whole scope, `unwitnessed` included | **0 findings.** Before: 126 findings, all `unwitnessed`: en1993 32, en1992 29, en1998 23, en1997 21, en1999 19, results 2. Now every leaf is witnessed: en1993 49/49, en1992 28/28, en1998 29/29, en1997 20/20, en1999 18/18, en1994 25/25, en1995 66/66, iso16757 29/29, vdi3805 19/19, results 1/1. Norm as a whole: 555/555 witnessed, 0 findings. |
| `schema mutation-inputs`, whole scope | **0 findings** in every artifact of the scope and in the results config. |
| Crate tests, including the payload law | **Green.** Details in §5. |
| EN 1998 Python oracle | **Fixed.** The feature, the Python oracle, the Rust adapter, the catalog and the manifest all now cover the current 29 kinds. The oracle phase passes 59/59 through the repository harness. |
| EN 1998 case, all phases | **Green through the repository harness.** Oracle 59/59, subject 59/59, parity 59/59 (118 executed). Earlier attempts were blocked by peers' in-flight edits to `semio-framework-ui`, `semio-framework-plugin` and `semio-framework-os-kernel`; they were rerun once those crates compiled again. |

No norm feature had wire-form rows (`rows = 0`). Every norm case compares committed `(before, mutation, after, outcome)`
vectors, so the F10 row conversion does not apply here. Each leaf that had no evidence got one of two things:

- a payload-only wire witness, which the crate asserts (§2);
- for EN 1998, a full catalogued vector, because the explicit task was to repair the EN 1998 case (§3).

## 2. Payload-only wire witnesses

98 witnesses were added:

- EN 1993: 32, the 16 insert/remove leaf pairs.
- EN 1992: 28, all leaves. Its old vectors were deleted in 652.
- EN 1997: 20, all leaves.
- EN 1999: 18, all leaves.
- Results config: 1, `change-selected-check-index`.

Each witness is written to `…/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json` as the full aggregate wire:

- `{"<Variant>": payload}` for the externally tagged EN 1992, EN 1993 and results aggregates.
- `{"mutation": "<const>", …}` for the internally tagged EN 1997 and EN 1999 aggregates. The tag is taken from the leaf schema's `mutation.const` (for example `changeMemberMYEd`), never re-derived from the kind.

**How the payloads were built.** They come from Rust-encoded example documents:

- A temporary `[DEBUG]` dump test printed `encode_<std>_snapshot_json` of every committed `📚️examples` DSL document and of the default document, and was then removed.
- The inputs were real records (copied with a new id), existing ids, and realistic domain values.
- Every number is spelled as its Rust type emits it: `f64` always with a fraction, integers without. The spelling is taken from the leaf and snapshot schema types (`🧪️w2-w-norm-3-witnesses.py`).

**Owner-side law.** Each crate has a new module `🧬️schema/🧬️mutations/🧪️tests/🧪️wire-witness/🦀️.rs`, wired under
`//#region 🧾️WireWitness`. Its test `committed_wire_witnesses_are_the_canonical_wire` checks every witness in two ways:

- `store::os_store::test_support::assert_wire_witness::<Aggregate>` (F11): decoding with `FromValue` and re-encoding with `ToValue` gives exactly the committed JSON, including number spelling;
- the decoded op's `descriptor().semantic_kind` is the leaf it witnesses.

The results config asserts its witness in its existing integration test `config_mutation`. The derive-emitted `semio_payload_law_<aggregate>` covers every witness as well.

**Negative check.** Spelling one EN 1997 witness as `30` instead of `30.0` made the law fail with "wire witness is not the canonical Rust wire". It passed again after the file was restored.

## 3. EN 1998 — case repaired, vocabulary made invertible

### 3.1 Why the Python oracle pointed at deleted vectors

Every surface of the case still named the pre-652 flat vocabulary of 49 `change-<field>` kinds:

- the feature, the Python `KINDS`/`VECTORS`, the Rust adapter's `KINDS` and its `include_str!` table;
- the catalog (`kinds` and 49 `vectors`) and the v2 mutation manifest.

W2-S-A deleted those 49 vectors as orphans. Deleting them was correct, because none of those kinds exists any more.

### 3.2 Real defect found: seven inserts could not be undone

`insert-bridge`, `insert-assessment`, `insert-silo`, `insert-tank`, `insert-foundation`, `insert-retaining-wall` and
`insert-tower` returned an **empty inverse**. An inserted part could therefore never be undone. `En1998Mutation` had no
matching `remove-*` kind.

Fix: seven new leaves on the exact template of `➖️remove-building`: `remove-bridge`, `-assessment`, `-silo`, `-tank`,
`-foundation`, `-retaining-wall`, `-tower`.

- Each leaf has a struct, a descriptor, a payload schema with en/de `x-semio-ui`, a diff that refuses a missing index with `mutation.target-missing`, and an inverse that re-inserts the removed record.
- They are registered in:
  - the crate-root module tree;
  - the aggregate (variants, `KINDS`), the aggregate schema `oneOf` and binary tags 22–28;
  - the text and binary codecs, and the demo cases;
  - the leaf-taxonomy fixture, regenerated with the plugin's own `mutation-leaf-taxonomy-generate`, which added exactly these 7 rows;
  - the schema catalog: 7 `mutation-leaf` scopes and the parent `dependsOn`, edited by hand on unique anchors.
- Each insert's inverse is now its `remove-*`.

`from_snapshot` was also corrected. It **inserted duplicates** when a part list changed, and it silently dropped
field changes other than the one scalar with a change kind. Now:

- a list that differs only in `vRdN`, `rKN` or `mRdNm` still decomposes into the `change-*` kind;
- any other difference replaces the whole list through the vocabulary: remove every base record back to front, then insert every target record (`replace_all`).

### 3.3 Vectors, tests and case

29 catalogued vectors were added, one per kind. They live at `🧫️fixtures/🧬️mutations/<leaf>/<scenario>/{🦠️mutation,🎯️outcome,📸️snapshot/⬅️before,📸️snapshot/➡️after,🔺️diff}`, 145 files in total.

- **Base.** Every vector starts from the Rust-encoded `🏢️seismic-multipart` example, the one document that carries every EN 1998 part.
- **Payloads.** Authored by `🧪️w2-w-norm-3-en1998-vectors.py stage`.
- **After and diff.** Written by Rust production dispatch through a temporary `[DEBUG]` settle test, which was then removed (`settle`).
- **Tests.** Each vector has a canonical test at `<leaf>/🧪️tests/<scenario>/🦀️.rs` with three checks:
  - the applied document equals the committed after and diff, and it moved;
  - the op's own inverse restores the before-snapshot;
  - the declared outcome holds.
- **Wiring.** The tests are wired through a regenerated `🧪️tests/🔬️fixture/🦀️.rs`. This also wires the existing, previously unwired, `🔬️unit` and `🔬️kinds-catalog` tests.
- **Removed.** The 21 unwired demo-based `🧪️tests/<applies-*>` files. They are replaced by the vector tests.

The case surfaces were regenerated from the same list by `surface`:

- the catalog (`kinds` = 29, `vectors` = 29);
- the manifest: 29 rows, with outcomes from each leaf's `outcomeClasses`;
- the oracle rationale and `fixtureCoverage.vectors` = 29;
- the feature's two Examples tables and its description, which now covers the three addressing shapes;
- the Python `KINDS`/`VECTORS`;
- the Rust adapter's `KINDS` and fixture table.

Two things were also broken in the pre-existing Rust adapter and are fixed:

- `round_trip` called `carrier_projection` unqualified from inside `mod subject`, so the adapter never compiled with the `sut` feature. It is now `super::carrier_projection`, and the top-level imports are made `sut`-conditional.
- The identity scenario read `asset://🏢️seismic-rc-frame/🎒️.pack.semio` without the feature declaring it. It is now declared as the committed binary twin.

### 3.4 Shared Python norm engine (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`)

All changes are additive and generic, and they are documented in the functions' docstrings.

1. **Nested index addressing.** `index_path` and `indexed_record` read `{buildingIndex, storeyIndex|systemIndex|memberIndex}` as `buildings[b].storeys[s]`, for change verbs and their inverse.
2. **Collection lookup falls back to the leading noun prefix.** For example, `change-bridge-v-rd-n {index}` resolves to `bridges`.
3. **Plural spellings.** `collection_names` tries `-s`, then `y → ies`, then the bare noun, so `storey` resolves to `storeys`.
4. **Whole-facet `update-<noun> {<noun>: record}` replacement.** The facet replaces a document field, or the id-matched member of the like-named list (`facet_slot`). The inverse is the old facet.
5. **Bug fix: false derived views.** `derived_view` no longer treats a list nested inside the records it would project as a "derived mirror". It had rebuilt `buildings[0].storeys[0].variables` whenever `buildings` changed.
6. **Bug fix: carrier indentation.** The carrier reader keeps each table row's indentation. Nested record tables are indented deeper, and the old reader re-indented them to 2 spaces, so EN 1998's identity round trip failed.

**Regression check.** The in-process probe (`🧪️w2-w-norm-3-oracle-probe.py`) was run over all 15 norm cases with the HEAD engine and the new engine. No case got worse:

| Case | HEAD engine | New engine |
|---|---|---|
| en1998 | 31/59 | 59/59 |
| en1991 | 148/161 | 161/161 |
| en1993 | 3/35 | 13/35 |
| en1995 | 55/133 | 57/133 |
| din4108 | 37/67 | 41/67 |
| all others | unchanged | unchanged |

## 4. Also fixed

The norm plugin schema `🧬️schema/🔣️.json` pinned `NormMutationLeafTaxonomy.rows` to exactly 392 rows. That was stale since 652, when there were 547 rows, so `mutation-leaf-taxonomy-check` failed. The pin and the description are now 554, and the check passes ("fresh: 554 payloads, AJV schema and hostile vectors passed").

## 5. Verification (run, foreground, gated, private `target-nde-w2w-norm3`, `CARGO_INCREMENTAL=0`)

| Check | Result |
|---|---|
| `cargo test --lib` en1998 | **166/166**, before and after the scenario-directory rename in §7. Includes 87 vector tests, `kinds_match_the_enum_and_the_catalog`, `every_variant_op_text_round_trips` and `semio_payload_law_en1998_mutation`. |
| `cargo test --lib` en1992, en1993, en1997, en1999 | **100/100, 161/161, 99/99, 80/80.** In each crate, `committed_wire_witnesses_are_the_canonical_wire` and `semio_payload_law_*` pass. en1992, en1993 and en1999 were rerun at the end, on top of W3-CODES' remapped en1992 diff codes, and are still green. My changes do not touch those codes. |
| `cargo test -p semio-s-artifact-norm-contract --lib --test config_mutation` | **51/51 and 2/2**, including the new `committed_wire_witness_is_the_canonical_wire` and `semio_payload_law_norm_results_window_config_mutation` |
| Negative check of the witness law | Fails as intended, then passes after the restore |
| Repository harness `oracle exhaustive --case 🫨️mutate-en1998-1` | **59/59 passed** |
| Harness `subject exhaustive --case 🫨️mutate-en1998-1` | **59/59 passed** |
| Harness `parity exhaustive --case 🫨️mutate-en1998-1` | **118/118 executed, parity 59/59** |
| In-process oracle probe after the rename | en1998 59/59 |
| Both lints over the whole scope (census) | 0 / 0. See `🗑️generated/w2w-norm3/census-after.txt`. Norm as a whole: payloads 555/555 witnessed with 0 findings; inputs 935/935 with 0 findings |
| `mutation-leaf-taxonomy-check` | Passes |
| `verify taxonomy report --scope …/🫨️en1998` | `path-too-long` 155 → 14 and `mutation-pair-path-budget` 24 → 4 after shortening scenario names. What remains is §7 |

## 6. Not verified (reason)

- **The runtime inventory cache for en1998 is stale.** The contract phase now reports 48 `runtime-only`, 28 `manifest-only` and 1 `outcome-mismatch` for en1998.
  - The cached `⚡️cache/tests/results/🏭️inventory/s.norm.en1998@1@any.json` dates from 09-25 and still lists the 49 pre-652 kinds. Every norm inventory is equally stale.
  - Regenerating it needs `bun ./📜️script.ts test inventory --artifact s.norm.en1998 --standard 1 --subset any`. That builds the standalone `🏭️bridge` workspace, which covers all 15 norm crates, so it was not run.
  - The manifest's outcomes come from the same leaf descriptors the bridge reads.
- **Build operations.** Four `cargo test` processes deadlocked in `prebuild_lock_exclusive` for 12–27 minutes each, with no rustc child: mine, the xlsx test, the energy-model test and the en1996/din16798 test. `sample` confirmed the deadlock. All four were killed per pid, following the memory rule. Peers' commands for the other three will need a rerun.

## 7. Open items (outside this work package or blocked)

1. **Four EN 1998 kinds cannot have a vector within the 240-byte path budget.**
   - The kinds are `change-system-base-shear-resistance-n`, `change-building-elevation-regular`, `change-member-detailing-compatible` and `change-building-masonry-wall-area-ratio`.
   - Their leaf directory names leave 3 to 7 bytes for a scenario directory under the canonical pair budget (fixture root + 42 bytes ≤ 240). Even a payload-only witness path would be 239–243 bytes.
   - Their vectors are committed with short names and are the remaining 14 `path-too-long` and 4 `mutation-pair-path-budget` findings.
   - The fix is a kind or leaf rename, or a grouped mutation-domain layout. That is a vocabulary change, and it is routed to the coordinator.
   - The other 25 vectors fit the budget; their scenario names were shortened.
   - Peers' fresh norm vectors are far over budget: en1991 335 files, en1996 135, en1990 94, din16798 93.
2. **`directory-kind-unresolved` on norm `🧫️fixtures/🧬️mutations/<leaf>` directories** is systemic. It already appears in the committed en1995 tree (65/66). A taxonomy registration for the norm fixture tree is missing.
3. **Other norm mutate cases in scope are pre-existing red.** Oracle probe results with the new engine:

   | Case | Oracle probe | Cause |
   |---|---|---|
   | en1992 | 1/57 | stale vocabulary, deleted vectors |
   | en1993 | 13/35 | the feature and the Python `VECTORS` name the pre-rename scenario directories |
   | en1994 | 3/45 | same as en1993 |
   | en1995 | 57/133 | inverse `newValue` addressing |
   | en1997 | — | the adapter has no `adapter()`; the feature is a single catalog-length scenario |
   | en1999 | 1/53 | stale vocabulary, deleted vectors |
   | iso16757 | 14/59 | vector path drift; the carrier nesting is refused |
   | vdi3805 | 26/37 | same as iso16757 |

   The contract gate also still carries stale catalogs and manifests for en1992, en1995, iso16757 and vdi3805, and `binary-protocol-drift` in en1992 and en1999.

   The EN 1998 recipe restores each of these cases: the generator's `stage`/`settle`/`surface` steps plus the engine rules above. Scope: one case per artifact.
4. **`schema generate`** is still owed repo-wide and is coordinator-gated. The catalog hashes of the en1998 aggregate schema and of the norm plugin schema are now stale; they are hash-only.
5. **Peer edits seen in scope, not mine:**
   - en1992 `🔺️diff/🦀️.rs` ×21;
   - en1998 `🧬️schema/🦀️.rs` (`mutation.apply` → `build.apply`);
   - staged snapshot-schema edits in every norm artifact.

## 8. Files

- **EN 1998 production and schema:**
  - `🗿️artifacts/🫨️en1998/🦀️.rs` (module tree);
  - `…/🧬️schema/🧬️mutations/{🦀️.rs,🔣️.json,💾️binary/📡️.protocol.semio,📝️text/🦀️.rs}`;
  - 7 insert `↩️inverse/🦀️.rs`;
  - 7 new `➖️remove-*` leaves, 5 files each.
- **EN 1998 tests:**
  - `…/🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs`;
  - 29 `<leaf>/🧪️tests/<scenario>/🦀️.rs`;
  - 21 removed `<leaf>/🧪️tests/<applies-*>/🦀️.rs`;
  - 145 files under `…/🧫️fixtures/🧬️mutations/`.
- **EN 1998 case:** `…/🔮️oracles/🔣️.json` and `…/🧪️tests/🫨️mutate-en1998-1/{🥒️.feature,🐍️.py,🦀️.rs}`.
- **Witnesses:**
  - 98 `🧾️wire-witness/🦠️mutation/🔣️.json` under the en1993, en1992, en1997 and en1999 `🧫️fixtures/🧬️mutations/`, and under `🪟️results/🎚️config/🧫️fixtures/🧬️mutations/☑️change-selected-check-index/`;
  - 4 `🧪️tests/🧪️wire-witness/🦀️.rs`, plus their wiring in each `🧬️schema/🧬️mutations/🦀️.rs`;
  - `🪟️results/🎚️config/🧪️tests/🎚️config/🦀️.rs`.
- **Shared:**
  - `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`;
  - `✏️s/🔌️plugins/📕️norm/🧬️schema/🔣️.json`;
  - `✏️s/🔌️plugins/📕️norm/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json` (generated);
  - `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` (7 scopes and `dependsOn`).
- **Ticket scripts:**
  - `🧪️w2-w-norm-3-dump.py`, `🧪️w2-w-norm-3-witnesses.py`;
  - `🧪️w2-w-norm-3-en1998-remove-leaves.py`, `🧪️w2-w-norm-3-en1998-vectors.py` (`stage|settle|surface`);
  - `🧪️w2-w-norm-3-oracle-probe.py`.
- **Scratch:** `🗑️generated/w2w-norm3/`: dumps, census, test, harness and taxonomy logs, and the HEAD engine copy used for the regression check.

## Session 2 — 2026-10-01

Executor S2-NORM (WP-1 NORM-CLOSE + WP-6 NORM-TS-TWINS, `📓️resume-evidence.md` §5), successor of norm-3. This section
covers EN 1992–1995, EN 1997–1999, ISO 16757, VDI 3805 and the results config, and holds the norm-wide closure table.
Scratch: `🗑️generated/s2-norm/`.

Status: IN PROGRESS (started 17:43; usage cuts ~18:00–21:30 and ~23:00–02:30; cargo hold from 02:45, fleet rule 26). Session-1 work not recorded above (en1995 conversion 07:23–07:55, en1992
`change-action-vk/nk` renames, option B) is reconstructed in `📓️resume-evidence.md` §2.8.

### S2.0 Closure table (WP-1 + WP-6)

| # | item | state | evidence |
|---|---|---|---|
| 1 | din4108 outcome vectors + class reconciliation | **done** 22:03 | 16 vectors (59), crate 168/168, oracle 103/103, lints 0; `📓️w2-w-norm-2-report.md` S2.1 |
| 2 | en1992 `change-anchor-a-s` identity | **done** 22:15 (crate test in item 3) | S2.2 below |
| 3 | `cargo test --lib` 15 crates + contract; wasm32 check | din4108 green (168/168); rest **blocked** (gate ≥ 8 rustc 22:05–22:50, then cut, then cargo hold) | S2.3 |
| 4 | oracle/subject/parity exhaustive, 15 cases | **oracle 15/15 cases, 1197/1197**; subject/parity blocked (kernel red) | S2.6 |
| 5 | bridge inventory refresh + contract phase | pending | |
| 6 | binary-protocol-drift en1992/en1999/en1996/din18599 | pending | |
| 7 | scoped `verify taxonomy report` per artifact | din4108 run (28 errors, 0 from this WP); en1992 cut at budget by load | S2.6 |
| 8 | `oracle-source` suite | 4/6 (was 3/6); 2 left: vdi3805 compliance script in an adapter slot, launch row (coordinator) | S2.6 |
| 9 | WP-6 TS twins (245 parser-missing + 13 incomplete), witness test, strict tsc | **done** 03:40: 690 twins, `--check` 0 drift, strict tsc 0 (970 files), witness **1901/1901**, census 246 → 30 (all framework `surface-schema`) + 1 catalog-stale | S2.4, S2.5 |

### S2.2 en1992 anchor identity and the broken norm composition script

- **Identity.** The kind `change-anchor-as` is the wire identity (variant `ChangeAnchorAs`, schema `$id`, binary tag 27);
  only the directory spelled it `a-s`. Moved `🧷change-anchor-a-s` → `🧷change-anchor-as` (leaf and fixture mirror) and
  rewrote every path naming it: lib-root `#[path]` ×3, catalog ×2, feature tables ×2 (alignment kept), case adapters
  (`🦀️.rs` `include_str!` ×4, `🐍️.py` ×1), vector-suite mount, canonical test (`include_str!` ×5), descriptor `owner`;
  `displayName` became `Change Anchor A_s` (the standard's notation). Hot files by hand on unique anchors:
  `📚️library/🔣️schema-catalog.json` (path) and `📓️schema-catalog.md` (row). Script: `🧪️s2-norm-en1992-anchor-identity.py`
  (`--check` → 0 pending moves, 0 stale files). No reference to `change-anchor-a-s` remains in norm.
- **Composition script was broken by the layout move.** `🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/📜️script.ts`
  resolved `🗿️artifacts`, `🧬️schema`, `🧫️fixtures`, `🪟️results/🎚️config` and `🖥️app-surface/🦀️.rs` as `../../<x>` from
  the hub package, where none of them exists any more, so `mutation-leaf-taxonomy-generate|check`,
  `results-window-config` and `surface-render` all threw. Fixed with one `PLUGIN_OWNER = "✏️s/🔌️plugins/📕️norm"`
  resolved from `repoRoot` (the stdio composition's pattern); hub-owned paths (`🦀️.rs`, `🖥️app-surface/🧫️fixtures`,
  `Cargo.toml`) stay package-relative; the app-surface source is read from `📇️registry/🧬️contract/🖥️app-surface/🦀️.rs`.
  The generate target's declared `outputs` in `📋️project.json` now names the real fixture path.
- **Taxonomy fixture.** `mutation-leaf-taxonomy-check` passed before the rename (554), the regenerated fixture differs in
  exactly the anchor's `module` and `source`, and the check passes after (554 payloads, AJV + hostile vectors).

### S2.3 Crate tests (item 3) — state at the cargo hold

- din4108: **168/168** (S2.1 in the norm-2 report).
- Batch A (`-p en1992 -p en1990 -p en1991 -p semio-s-artifact-norm-contract --lib`, `-j 4`, private target): the gate
  (`pgrep -x rustc` < 8) did not open between 22:05 and 22:50 (9–22 peer rustc). The one run that started (22:31) spent its
  540 s budget rebuilding peer-changed framework crates (`semio-framework-os-kernel-dsl-derive` …) and was interrupted
  cleanly at the budget (process group SIGINT, no orphans); its compiled units stay in the shared build-dir. No norm test
  binary ran. Owed after the hold: batch A, then en1993–en1999 + iso16757 + vdi3805 + din16798/din18599/en1996, then
  `--target wasm32-wasip2` for the norm component.

### S2.4 WP-6 — norm TypeScript twins, generator-first

**Generator** `T/🧪️s2-norm-ts-twins.ts` (`bun ./🧪️s2-norm-ts-twins.ts [--check] [--only <artifact>]`), schema-first on the
pattern of `🧪️w3-gltf-twins.ts`. Per artifact subset it writes, from the committed JSON Schemas:

- snapshot, diff, artifact and inference twins — the root and every `$defs`/`definitions` record, each `export interface|type
  <Name>` plus `export const parse<Name>: NormWireReader<Name>`; `/** @state … */` from `x-semio-state` is kept;
- the four text twins (`📸️snapshot|🔺️diff|🧬️mutations|💡️inferences/📝️text`): `type <X>Text = string` + `parse<X>Text`;
- one wire twin per mutation leaf (`<leaf>/🧬️schema/🟦️.ts`, 554 leaves) and the aggregate `<A>Mutation` twin, externally
  tagged (`{ "<Variant>": payload }`, 10 artifacts) or internally tagged (`{ "mutation": "<const>", … }`, en1990, din18599,
  en1997, en1998, en1999) exactly as `🧬️mutations/🔣️.json` spells it;
- for iso16757's 29 split-layout leaves, `<leaf>/🦠️mutation/🟦️.ts` becomes a one-line re-export of the generated leaf twin
  (its hand-written payload interfaces were restated copies; each exported exactly the schema title).

Readers live once in the **norm wire contract, TypeScript half** `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🟦️.ts` (new,
sibling of the Rust codec all 15 artifacts share): `NormWireRefusal`, `NormWireReader<T>`, typed member tables
(`NormWireMembers<T>` proves every interface member has a reader with matching optionality), object/tagged/external/any/
nullable/array/map/literal/json readers. Readers judge structure (types, closed members, tags, enumerations, item counts);
value bounds stay the schema's, judged by Ajv. Cross-module `$ref`s resolve through `$id` (norm facets) or the central catalog
(`os/store/child` → framework `parseArtifactChild`).

**Naming.** A `$defs` record the schema withholds from TypeScript (`x-semio-formats` without `🟦️typescript`) is emitted as
`<Prefix><Key>` (`Din4108ThermalZone`), so no twin declares an export its contract withholds. Names the peer's `🪶️sqlite`
companions already import are pinned in a small table (`PUBLISHED`: din16798 `ZoneDocument → Din16798Zone`, en1997's bare
record names, vdi3805 `VdiUnit`) plus two aliases resolved from a schema pointer, never hand-typed (en1992 `DuctilityClass`,
en1990 `ImportanceClass`). The generator fails (`broken`) when a companion imports a name a regenerated twin no longer exports.

**Schema debt fixed on the way (schema-first, wire unchanged).**

1. en1997 snapshot: every collection was `items: {type: object}` although `$defs` existed for three of them and the Rust
   structs, GraphQL types and protobuf messages existed for all six. Items now `$ref` `SoilLayer`, `SpreadFoundation`, `Pile`,
   and the new closed `$defs` `RetainingWall`, `Slope`, `UpliftCase` (Rust field order and types). Script:
   `T/🧪️s2-norm-schema-records.py` (`--check` → 0).
2. en1995 `🔺️diff/📝️text/🔣️.json` was a copy of the diff schema — same `$id` as `🔺️diff/🔣️.json` (a duplicate `$id` in one
   Ajv instance), title `En1995Diff`. Restored to the canonical text facet every sibling declares
   (`…/en1995/diff/text.json`, `En1995DiffText`, `type: string`). The catalog still maps `En1995Diff` to the text file until
   the central `schema generate` (coordinator).

**Consumers repaired (precise types exposed real defects).**

- en1998 `🪶️sqlite`: `en_ground_type`/`en_spectrum_type` are `TEXT NOT NULL`, but Rust skips them when empty
  (`skip_serializing_if = "String::is_empty"`) and all 76 committed en1998 snapshots omit them: the companion wrote
  `undefined`. Now writes `?? ""` and reads an empty column back as an absent member, mirroring Rust.
- vdi3805 `🪶️sqlite`: the `generic` sheet variant's `entries` is optional in the schema; iterated as `a.entries ?? []`.
- en1990 `🪶️sqlite`: `kind` read from TEXT is typed to `PermanentAction["kind"]`; its test now reads the JSON fixture through
  `parseEn1990Snapshot` instead of spreading a widened JSON import.
- `⚖️en1990/…/🧬️schema/🧪parse-en1990-artifact.bun.ts` (run by the Rust io unit test, which checks its stdout) used the
  hand twin's `En1990ParseError`; it now asserts `NormWireRefusal` positions (`$.consequenceClass`) and reasons. Ran:
  `parse-en1990-artifact:ok`.

**Verification so far (run).**

- `bun ./🧪️s2-norm-ts-twins.ts --check` → `subsets=15 twins=689 stale=0 broken=0`.
- Strict tsc (`🗑️generated/s2-norm/tsconfig-twins.json`: contract, every `🧬️schema/**/🟦️.ts`, sqlite `🗄️.d.ts`, snapshot
  tests, `🧪*.bun.ts`; `strict`, bundler resolution): **64 errors → 1** (`tsc-twins-{1..5}.txt`). The 59 pre-existing errors
  were the hand text twins returning an object for a `string` type and din16798's inference twin; the generated twins
  themselves compiled clean from the first run.
- Census replica of the lint (`T/🧪️s2-norm-ts-twin-census.py`, same predicates as `schemaExportCompletenessDiagnostics`) over
  the committed catalog: 246 parser-missing / 6 incomplete before, **30 / 1** after (S2.5).

### S2.5 WP-6 — witness test, wire order, final twin numbers (02:45–03:45, cargo hold)

**Readers refuse what the schema refuses.** The sqlite peer's tests use `parse<A>Snapshot` as the exact validator
("refuses every unsigned32 counterexample / unknown native choice"), so the contract readers now also enforce numeric bounds
(`normWireRange`: `minimum`/`maximum`/`exclusive*`), and an absent optional nullable member reads as `null` (`normWireDefault`),
mirroring Rust `Option` + `#[value(default)]` (vdi3805 `GenericAttribute.unit`). All 15 sqlite companion suites pass on the
generated twins: **330 pass, 0 fail** (`bun test ./…/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts` per artifact; one en1998 case once hit
the 5 s timeout under load and passed 3/3 on rerun).

**Witness test** `✏️s/🔌️plugins/📕️norm/🧪️tests/🧪️wire-twins/🟦️.ts` (`bun test ./…`): for every committed `🦠️mutation` and
`📸️snapshot/{⬅️before,➡️after}` of all 15 artifacts — the strict Ajv oracle (`semioSchemaAjvV1`, every leaf, the aggregate,
snapshot/diff/artifact and foreign `$ref` documents resolved through the catalog) and the twin agree on admission (negative
witnesses are refused by both); an admitted wire decodes and re-encodes **byte-equal** in the committed spelling (two-space
JSON, Python float repr at `number` positions, integers at `integer` positions); a member spliced into the closed payload is
refused by both; an unknown tag is refused by both. **1901 tests, 1901 pass, 8256 assertions, 3.1 s.**
Negative control: swapping two members in the generated din4108 snapshot reader fails 118 tests and `--check` reports the
twin stale; regenerating restores 1901/1901.

**Wire order made canonical** (`T/🧪️s2-norm-wire-order.py`, census + `--apply`). The Rust law compares `serde_json::Value`s,
so member order was pinned nowhere: en1990 and en1991 vectors had been written key-sorted, en1992/en1999 schemas listed some
members in another order than the Rust structs. Rule now: schema property order = Rust struct field order (the `ToValue`
wire order), and every committed mutation/snapshot is stored in schema order.

- 8 schema records reordered to their Rust structs where the member sets agree (en1990 2, en1991 4 — via its records, en1992
  `ReinforcementGrade` + 1, vdi3805 2); `required` follows.
- 307 fixtures rewritten value- and spelling-preserving (en1990 112, en1991 165, iso16757 5, en1993 2, en1994 3, en1995 20).
- en1999 snapshot collections restated their records inline although `$defs` existed with the same members: items now `$ref`
  `AluminiumMaterial`, `AluminiumSection`, `AluminiumMember`, `AluminiumConnection`, `FireScenario`, `FatigueDetail`,
  `ColdFormedSheet`, `AluminiumShell` (`T/🧪️s2-norm-schema-records.py`).
- Backups: `🗑️generated/s2-norm/{json-before-order.tar,twins-before.tar}`.

**Model gaps reported by the order census (not in the witness scope, not changed):** vdi3805 `🔺️diff` schema lacks the
Rust diff field `limits`; en1999 `🔣️.json` (artifact) lacks `coldFormed` and `shells`; iso16757 diff schema carries
`selectedCheckIndex`, which the Rust diff does not have; en1991 and en1996 snapshot schemas are not in canonical JSON layout
(skipped by the reorder, their order already equals Rust).

**Lint census after** (`🧪️s2-norm-ts-twin-census.py`): parser-missing **246 → 30**, incomplete **6 → 1**. The 30 are
`NoConfig`/`NoPresence` under `✏️editor/{🎚️config,👥️presence}`, emitted by the framework `surface-schema` projection
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts` emits interfaces only): a framework
fix + `surface-schema` re-run, not norm. The 1 is en1995 `En1995Diff` still catalogued at the diff-text file: clears with the
central `schema generate`.

### S2.6 Cases, oracle-source, taxonomy, cargo (02:45–03:35)

- **Oracle phase through the harness** (`🧪️test`: `bun ./📜️script.ts oracle exhaustive --case <case>`, needs no cargo): all
  15 cases green, **1197/1197** (en1990 72, en1991 161, en1992 59, en1993 115, en1994 51, en1995 135, en1996 121, en1997 44,
  en1998 68, en1999 38, din16798 89, din18599 43, din4108 103, iso16757 59, vdi3805 39). In-process probe identical.
- **iso16757 identity regression (peer) repaired in the shared engine.** A staged peer change (20:13) re-printed the
  committed demo carrier with lists of records as `[ { … } ]`; the shared Python carrier reader
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`) refused the bare `{` token (58/59). The reader now keeps the record and
  list delimiters `{ } [ ]` of a field line as written (`BRACKETS`), and the printer re-emits them: carrier bytes, no grammar
  inferred. Back to 59/59; the other 14 probes unchanged.
- **`oracle-source`** (`📦️packages/🟦️typescript`: `bun ./📜️script.ts oracle-source`): **4 pass / 2 fail** (session 1: 3/3;
  the en1997 `adapter()` failure is gone). Left: (1) `🏭️vdi3805/…/🧪️tests/⚖️compliance-vdi3805-1/🐍️.py` is a crate-hosted
  compliance oracle script (run by its `🦀️.rs` via `python3 <snapshot>`) sitting in the repo-test adapter slot without a
  feature, so the source-ownership enumeration counts a sixteenth adapter — owner decision: make it a feature-backed
  compliance case, or move the script out of `🧪️tests/<case>/🐍️.py`; (2) `.vscode/launch.json` and `🧩️launch.seed.jsonc` lack
  `bun nx run @semio-tech/norm-js:test-oracle-source` (coordinator).
- **Witness registration:** `bun ./📜️script.ts wire-twins` (norm TS package) and nx target `@semio-tech/norm-js:test-wire-twins`
  (1901/1901 through the package). Launch row owed (coordinator): `bun nx run @semio-tech/norm-js:test-wire-twins`.
- **Taxonomy** (`bun ./📜️script.ts verify taxonomy report --scope <artifact>`, ~5.5 min each at low load): din4108 → 28 errors,
  none on a path this WP created (the 16 new vector directories resolve; 0 `path-too-long`): 20 `directory-kind-unresolved`
  (case and `🔬️*` test dirs, the sqlite peer's `🪶️sqlite` dirs), 4 `path-emoji-presentation` (legacy `🌡`/`🏷` leaf names
  without U+FE0F), 2 `projection-member-unresolved` (`💾️binary`, `📝️text`), 2 sqlite peer files. en1992 was cut by its 570 s
  budget in the last phase (`plan/references 1/510`) under 11 peer rustc; the other 13 are owed.
- **Cargo after the hold:** batch A (`-p en1992 -p en1990 -p en1991 -p norm-contract --lib`) fails in the os-kernel, not in
  norm: `🗣️dsl/🦀️.rs`, `🗣️dsl/🪟️viewport/🦀️.rs`, `🗣️dsl/🧬️schema/🦀️.rs`, `🏪️store/🦀️.rs` — `E0618 expected function, found
  RecordSpecProducer` / `E0308` (a peer's in-progress dsl refactor, as the coordinator warned). Every norm crate depends on
  the kernel, so items 3, 4 (subject/parity), 5 (bridge) and 6 (contract phase) wait for it.
