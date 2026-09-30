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
