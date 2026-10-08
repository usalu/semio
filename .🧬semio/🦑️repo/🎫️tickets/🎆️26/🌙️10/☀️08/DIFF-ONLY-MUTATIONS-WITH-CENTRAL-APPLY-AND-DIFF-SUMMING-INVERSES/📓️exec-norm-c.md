# 📓️ exec-norm-c — iso16757, en1992, en1994, en1997, vdi3805, din18599, en1999 (158 kinds) + 🪟️results

Status: **WRITTEN BUT UNVERIFIED in Rust (no norm crate was ever reached by cargo).** Five gated `cargo check --target wasm32-wasip2` attempts (cwd = artifact workspace `🗿️artifacts/🏛️en1992`, last log `T/🗑️generated/norm-c/out/check-en1992.txt`): four `Killed: 9` (swap 4.7/6 GB, kills landed 20-36 min in at `semio-framework-replication`), one stopped at `semio-framework-replication` itself (12 E0308 errors `RetainedCloneGrant`/`SharedOwner`, peer mid-change in `🌱️value/🗂️ordered`, not my crate). Verified without cargo: (a) `bun test` of a scratch copy of `🧪️tests/🧪️wire-twins` restricted to the seven artifacts —
**758 pass / 0 fail** (Ajv draft-07 strict on every committed mutation/snapshot/diff, TS twin round-trip byte-exact, undeclared-member refusal); (b) a standalone `rustc`
harness (`T/🗑️generated/norm-c/harness`, stub `protocol`, real generated diff files + real entity structs) running **4000 randomized two-step histories per family**
(en1994 positional rows + nested rows, en1992 id rows with `order`/replace + nested rows, vdi3805 map rows + nested keyed rows + scalar-valued maps, din18599 section patches +
Option wrappers + id rows, iso16757 id/positional/map/Option-scalar rows) asserting: apply succeeds, `apply(inverse(d))` restores, `apply(absorb(d1,d2)) == apply(d2)∘apply(d1)`,
`apply(inverse(absorb(d1,d2)))` restores, `apply(between(a,b),a) == b`, `between(a,a)` empty — all green.

## Design (shared by all seven artifacts)
- Each artifact's `🧬️schema/🔺️diff/🦀️.rs` is rewritten: scalar setters, `*Rows` keyed collection diffs, `*Patch` per-field patches (flat typed fields, nested keyed child rows),
  `Option` wrapper records for Option-typed fields, per-field section patches for din18599/vdi3805/iso16757 sub-documents. Removed: `artifact` whole-snapshot field,
  `diff_set_snapshot`, `apply_to_artifact`, `*List{values}` wrappers.
  - id-keyed lists: `added` rows, `removed` ids, `modified` patches, `order` (final id order when it deviates from base-minus-removed-plus-added).
  - index-keyed lists (en1994, iso selection constraints): `removed` base positions, `inserted` rows at final positions, `modified` base-position patches; absorb translates positions.
  - maps: `added` entries, `removed` keys, `modified` entries/patches. `absorb` coalesces patch∘patch, create∘modify (folds into the added row), create∘delete (cancels), delete∘create (replace).
  - `DiffAlgebra::inverse` concrete (negative diff, restores exact order), `between` covers the modelled vocabulary (unmodelled residual records of iso16757/vdi3805 — e.g. accessories, dictionary relationships, `catalog.extensions` aside from a replace slot — are not reachable by mutations nor by `between`).
  - `MutationDiff::apply(&self, base, capability)` is the only snapshot writer; no leaf calls it. `apply_<art>_mutation`, io builders `mutate`/`absorb`, all tests use `protocol::apply_diff`.
- Leaves (`🔺️diff/🦀️.rs`): 130 regenerated declaratively (checks kept verbatim, then a sparse literal); 28 scalar-setter leaves unchanged (already sparse).
  Behaviour changes: `change-element-u` now rejects an unknown element (`mutation.target-missing`, was a silent no-op); list-setter kinds (`change-members`, `replace-zones`, …) reject duplicate ids in the payload (`mutation.duplicate-id`);
  `reorder-members` with an unchanged order is `mutation.no-op`; `reorder-members` inverse uses the clamped destination.
- Wave-2 rulings: removal inverses restore the ORIGINAL index everywhere (`Insert*{index}`, `AddMember{index}`, `Introduce*{index: Some(pos)}`, `AddProduct{index: Some(pos)}`); the two kinds that appended on undo got an optional `index` payload field:
  `iso16757 add-selection-constraint` and `vdi3805 add-geometry-connection` (rust payload, diff/inverse leaves, text+binary codecs, leaf JSON schema + TS twin, fixtures carry `"index": null`, `from_snapshot` sites).
  `🧭️middle` law scenario (three-row document, delete/move the MIDDLE row) committed for 22 ordered-collection kinds (`🧫️fixtures/🧬️mutations/<kind>/🧭️middle`) and run by `🧬️mutations/🧪️tests/🔬️middle-row` per artifact (diff == committed, inverse restores position, `assert_mutation_inverse_sum_law`).
- norm-a seam: `commit_value_tree_edit` is gone — every editor command now resolves through `crate::mutations::resolve_edit(document, edit)` = row-id removals (`ID_REMOVALS`, because `RemoveItemRule` addresses rows by index only)
  falling back to `EDIT_RULES` (`🧬️mutations/🧭️edit-rules/🦀️.rs`, **derived from the committed vectors by `T/🗑️generated/norm-c/gen/edit_rules.py`**). Map-keyed collections (vdi3805 geometry/curves/edition profile, iso16757 geometry objects/part-number inputs) and `reorder-members`, iso `change-script-limits` (3 payload fields) have no setField path — the editor cannot raise them through the generic commands (they remain reachable as concrete kinds).
- din18599: 17 stub `✅apply` tests replaced by the full vector test (committed diff, apply, inverse, outcome, canonical JSON) + law test and mounted in the fixture harness; `update-climate` law test added in `🧪️tests/🔬️unit`.
- Each artifact has `🧬️schema/🔺️diff/🧪️tests/🔬️unit` (mutation-driven coalescing/inverse/between laws). `Cargo.toml` of iso16757 and vdi3805 gained the `protocol-laws` dev-feature.

## Restore inverses (retained, by ruling)
`update-renewables/cooling/ventilation/lighting`, `specify-heating-system/dhw-system`, `update-climate`, `replace-zones/elements` (din18599), `change-members/sections/connections/materials/fire-scenarios/fatigue-details/cold-formed/shells` (en1999):
inverse = the same kind with the base entity value (accepted by the wave-2 ruling); their diffs are sparse per-field/per-row so the sum law holds.

## NOT done / open
1. **Rust verification**: nothing compiled or run (see status). First thing to run in each artifact workspace (`cd ✏️s/🔌️plugins/📕️norm/🗿️artifacts/<art>`): `gate cargo check --target wasm32-wasip2 --message-format=short`, then (when test builds are allowed) `cargo test --lib diff mutations`. Expect first-compile fixes in: `DslRecord`/`ToValue` derives on the generated structs (`usize`, `Option<Vec<String>>`, `[f64;12]`), `ArtifactSchema` on the root diff, crate-root re-exports used by the generated type paths (`crate::snapshot::*` for en1999, `crate::part_N::*` for iso16757), `semio_framework_async_macros::async_test` in new tests.
2. `set-snapshot` command / `from_snapshot` / `import_media` `model:in` decomposition are **still present** in all seven artifacts: removing them needs a contract change (`import_media(wrap)`, `commit_snapshot_fields`, `set-active-example`, editor unit tests that seed documents through `ReplaceSnapshot`) owned by norm-a. `from_snapshot` emits concrete kinds (no diff, no `between`), so it is not a diff violation — decision needed.
3. The shared `🪡️list-delta` module was not reused (the families were designed before it landed; shapes are equivalent: keyed added/removed/modified + coalescing absorb + randomized law test). Candidate follow-up: port the id-keyed rows onto `Parts<R,Q>`.
4. Python independent engine (`🧪️tests/<id>/🐍️.py` via shared `🐍️` vocabulary) was not re-run; it does not model diffs, but it must accept the new optional `index` on the two kinds above.
5. `between` is partial for iso16757/vdi3805 (see above); no framework caller uses it for artifacts.

## Per-kind table (158)
| artifact | kind | before (audit) | after |
|---|---|---|---|
| iso16757 | retire-subject | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | introduce-subject | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | change-part-number-input | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | change-selection-class | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | rename-manufacturer | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | introduce-product-class | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | rename-product | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | rename-catalogue | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | introduce-geometry-object | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | introduce-property-definition | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | introduce-product-series | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | introduce-product | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | change-exchange-process | V1-GENERIC-DIFF | scalar setter (unchanged diff), law test |
| iso16757 | remove-part-number-input | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | introduce-product-index | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | add-selection-constraint | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | remove-selection-constraint | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | rename-product-group | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | retire-geometry-object | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | retire-product-class | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | retire-product-index | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | retire-product-series | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | change-script-limits | V1-GENERIC-DIFF | sparse keyed/patch diff, law test |
| iso16757 | retire-product | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | replace-part-number-rule | clean (V4 only) | scalar setter (unchanged diff), law test |
| iso16757 | change-selection-series | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | retire-product-group | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | introduce-product-group | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| iso16757 | retire-property-definition | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-bar-layer-count | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-member-width | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-member-height | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-action-vk | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | insert-anchor | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | insert-member | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | remove-member | V1-GENERIC-DIFF | sparse keyed/patch diff, law test |
| en1992 | change-action-mk | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-bar-layer-diameter | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-member-span | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-annex | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1992 | change-member-exposure | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-action-nk | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-title | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1992 | change-design-working-life | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1992 | change-anchor-h-ef | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-delta-c-dev | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1992 | change-member-effective-depth | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | reorder-members | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-member-axis-distance | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-member-fire-rating | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-reinforcement-f-yk | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | remove-anchor | V1-GENERIC-DIFF | sparse keyed/patch diff, law test |
| en1992 | change-member-cover | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-cement-type | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1992 | change-concrete-f-ck | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-anchor-as | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1992 | change-member-stirrup-spacing | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-beam-stud-count | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-beam-transverse-as | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-column-kind | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | remove-column | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-beam-stud-spacing-m | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | insert-slab | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | insert-beam | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | remove-slab | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | remove-beam | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | insert-column | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-column-action-force-n | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-beam-stud-diameter-m | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-beam-action-q-area-pa | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-annex | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1994 | change-steel-fy-pa | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1994 | change-structure-kind | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1994 | change-beam-stud-fu-pa | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-slab-thickness-m | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-beam-span-m | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-slab-action-q-area-pa | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-fatigue-detail | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1994 | change-fire-rating | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1994 | change-beam-construction | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1994 | change-insulation-thickness-m | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1994 | change-beam-slab-thickness-m | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | change-footing-width | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | change-slope-angle | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | insert-footing | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | insert-layer | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | remove-footing | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | remove-layer | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | change-footing-embedment | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | change-layer-oedometric-modulus | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | change-annex | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1997 | change-groundwater-level | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1997 | change-design-situation | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1997 | change-pile-length | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | change-layer-phi-prime | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | remove-pile | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | insert-pile | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | change-investigation-depth | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1997 | change-pile-count | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1997 | change-geotechnical-category | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1997 | change-design-approach | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1997 | change-wall-base-width | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | remove-geometry-connection | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | change-product-configuration | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | change-manufacturer-file | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | sparse keyed/patch diff, law test |
| vdi3805 | rename-product | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | change-correction-as-of | clean (V4 only) | scalar setter (unchanged diff), law test |
| vdi3805 | add-curve | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | remove-curve | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | change-curve-points | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | resize-geometry | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | add-product | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | add-geometry-connection | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | change-strict-mode | clean (V4 only) | scalar setter (unchanged diff), law test |
| vdi3805 | change-edition-profile | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | remove-product | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | change-limits | V1-GENERIC-DIFF | sparse keyed/patch diff, law test |
| vdi3805 | remove-geometry | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | add-geometry | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | change-geometry-parameters | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| vdi3805 | remove-edition-profile | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| din18599 | update-renewables | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| din18599 | change-geg-qp-factor | V4-LAW-UNTESTED | scalar setter (unchanged diff), law test |
| din18599 | update-cooling | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| din18599 | change-delta-u-wb | V4-LAW-UNTESTED | scalar setter (unchanged diff), law test |
| din18599 | change-element-u | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY, V4-LAW-UNTESTED | sparse keyed/patch diff, law test |
| din18599 | update-climate | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| din18599 | update-ventilation | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| din18599 | change-automation-class | V4-LAW-UNTESTED | scalar setter (unchanged diff), law test |
| din18599 | change-building-category | V4-LAW-UNTESTED | scalar setter (unchanged diff), law test |
| din18599 | change-use-class | V4-LAW-UNTESTED | scalar setter (unchanged diff), law test |
| din18599 | update-lighting | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| din18599 | change-net-floor-area-m2 | V4-LAW-UNTESTED | scalar setter (unchanged diff), law test |
| din18599 | change-heated-volume-m3 | V4-LAW-UNTESTED | scalar setter (unchanged diff), law test |
| din18599 | specify-heating-system | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| din18599 | replace-zones | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| din18599 | specify-dhw-system | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| din18599 | replace-elements | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| din18599 | change-method | V4-LAW-UNTESTED | scalar setter (unchanged diff), law test |
| din18599 | change-attachment | V4-LAW-UNTESTED | scalar setter (unchanged diff), law test |
| en1999 | change-material-designation | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1999 | change-cold-formed | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| en1999 | add-member | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1999 | remove-member | V1-GENERIC-DIFF | sparse keyed/patch diff, law test |
| en1999 | change-member-my-ed | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1999 | change-annex | clean (V4 only) | scalar setter (unchanged diff), law test |
| en1999 | change-member-n-ed | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1999 | change-members | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| en1999 | change-member-buckling-length | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1999 | change-sections | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| en1999 | change-fatigue-details | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| en1999 | change-connections | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| en1999 | change-fire-scenarios | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| en1999 | change-weld-throat | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1999 | change-bolt-count | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1999 | change-materials | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |
| en1999 | change-plate-thickness | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | sparse keyed/patch diff, law test |
| en1999 | change-shells | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | sparse keyed/patch diff, law test; inverse = own-entity absolute setter (retained) |

## Files
- generator + oracle + harness (scratch): `T/🗑️generated/norm-c/gen/*` (`model.py`, `rust_emit.py`, `schema_emit.py`, `sidecars.py`, `oracle.py`, `leaf_*.py`, `edit_rules.py`, `middle_fixtures.py`, `middle_tests.py`, `harness.py`, `laws_*.rs`, `twins.ts`, `wire-twins-mine.test.ts`); pre-change tree: `T/🗑️generated/norm-c/orig-norm-c.tgz`.
- per artifact: `🧬️schema/🔺️diff/{🦀️.rs,🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`, `🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs`, every `🧬️mutations/*/🔺️diff/🦀️.rs` (non-scalar), `🧪️tests/✅apply/🦀️.rs` (apply_diff + law), `🧬️mutations/🦀️.rs` (bridge, edit-rules mod, middle-row mod), `🧬️mutations/🧭️edit-rules`, `🧬️mutations/🧪️tests/🔬️middle-row`, `🚪️io/🦀️.rs`, `✏️editor/🎮️commands/{set-field,insert-item,remove-item,apply-remedy}`, `🧫️fixtures/**/🔺️diff/🔣️.json` (all `✅apply`) + `🧭️middle` scenarios.
- no `🪟️results` change (its only kind is norm-a's `change-selected-check-index`).
