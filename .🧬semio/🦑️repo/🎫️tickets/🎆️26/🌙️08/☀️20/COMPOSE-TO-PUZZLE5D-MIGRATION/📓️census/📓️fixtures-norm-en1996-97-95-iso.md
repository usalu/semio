# 🧪️ Handcrafted mutation fixtures — `📘️en1996` · `📘️en1997` · `📓️iso16757` · `📘️en1995`

85 mutation leaves, 85 handcrafted cases, one per leaf. Every `➡️after`, every
`🔺️diff/🔣️component.json` and every assertion was derived from a direct read of that leaf's own
`🔺️diff/🦀️component.rs` (the oracle), its `🦠️mutation/🦀️component.rs` payload struct + serde
attributes, and its `↩️inverse/🦀️component.rs`. No shared harness, no macro, no generic sweep, no
loop over mutations in any committed file: each of the 85 `🦀️component.rs` bodies names its own
field/entity, its own value, its own guard branch and its own sibling-invariance claim.

| tree | leaves | cases | outcome | tag style |
| --- | --- | --- | --- | --- |
| `📘️en1996` | 22 | 22 | all `applied` | internally tagged (`{"mutation": "changeMEdKnm", …}`) |
| `📘️en1997` | 22 | 22 | all `applied` | internally tagged (`{"mutation": "changeDFM", …}`) |
| `📓️iso16757` | 21 | 21 | all `applied` | **externally tagged** (`{"RenameProduct": {…}}`) |
| `📘️en1995` | 20 | 20 | all `applied` | **externally tagged** (`{"ChangeFC0K": {…}}`) |

## 🔤️ Serde shapes verified per tree (read, not assumed)

- **`En1996Mutation` / `En1997Mutation`** carry `#[serde(tag = "mutation", rename_all = "camelCase")]`.
  Serde's camelCase on a *variant* lowercases only the first character, so `ChangeFKMpa` →
  `changeFKMpa`, `ChangeQBKpa` → `changeQBKpa`, `ChangeDFM` → `changeDFM`.
- **`En1995Mutation` and `Iso16757Mutation` carry NO `#[serde(...)]` container attribute at all** →
  they are **externally tagged**: `{"ChangeAVertMS2": {"newAVertMS2": 0.5}}`,
  `{"CreateSubject": {"subject": {…}, "index": null}}`. Copying en1996's internally-tagged shape here
  would have been wrong; these two trees are the pin on that.
- Payload field names: en1996/en1997/en1995 payload structs all carry
  `#[serde(rename_all = "camelCase")]` (`new_f_c_0_k` → `newFC0K`, `new_a_vert_m_s2` → `newAVertMS2`,
  `new_d_f_m` → `newDFM`). **iso16757's payload structs carry none** → snake_case payload keys
  (`new_name`, `property_definition`, `new_exchange_process`).
- All four snapshots and all four diffs are `#[serde(rename_all = "camelCase")]`; each diff adds
  `default` but **no `skip_serializing_if` on any field**, so the committed diff JSON emits **every**
  key with `null` for the untouched ones — 24 keys for en1996/en1997, 22 for en1995, 10 for
  iso16757. Verified field-by-field against each struct's declaration order.
- iso16757's *nested* types carry no rename at all: only `Iso16757Snapshot`'s own eight keys are
  camelCase (`partNumberRule`, `scriptLimits`, …); everything inside stays snake_case
  (`product_groups`, `short_name`, `required_property_ids`, `max_steps`, `si_factor`).
- `CatalogueValue` and `PartNumberRule` are internally tagged on `kind` with camelCase **variants**
  (`Decimal` → `"decimal"`, `Script` → `"script"`); serde's `rename_all` renames variants only, so a
  struct-variant's own fields stay snake_case (`function_id`).
- `AnnexChoice`, `MasonryClass`, `DesignSituation`, `ExposureClass`, `MortarClass`, `ExchangeProcess`,
  `SubjectKind`, `PropertyKind`, `ConstraintOperator`, `EditionProfile` carry `#[dsl(key = …)]` for
  the DSL but **no serde rename**, so the wire spellings are the bare Rust variant names (`"En"`,
  `"Class4"`, `"Mx3"`, `"M10"`, `"DetermineProduct"`, `"ProductSpecialization"`, `"LessThan"`).
- Numeric typing is pinned in the JSON: `u32`/`u64`/`usize` fields (`storeys`, `fireResistanceMin`,
  `pileNProfiles`, `max_steps`, `index`) are bare integers; every `f64` carries an explicit `.0`.
  `serde_json::Number` compares `PosInt` and `Float` as unequal, so the canonical-JSON assertion is
  what enforces this. Note en1995's `fire_duration_min` and `n_cycles_bridge` are `f64`, **not**
  integers, unlike en1996's `u32` `fire_resistance_min` — the fixtures spell them `60.0`/`2000000.0`.

## ⚠️ `Option<Option<T>>` — pinned, not asserted away

Every one of the four diff structs carries exactly one presence-lane field,
`selected_check_index: Option<Option<u32>>`. **No mutation in any of these four trees writes it**, so
it stays `None` and encodes as `null`; on the way back `null` deserializes to the OUTER `None`, so
the round trip holds. `None` and `Some(None)` are indistinguishable in these committed diffs — every
test file's module doc states this explicitly and nothing asserts otherwise.

## 📐️ Values

Physical-quantity schemas, so the values are dyadic where possible (0.25, 0.375, 0.5, 0.625, 2.5,
6.25, 12.5, 22.5, 26.5) and the magnitudes stay in serde's plain-decimal range (no exponent ever
appears: `9500000.0`, `3000000.0`, `2000000.0`, `640000.0`, `0.001`). The committed base documents
are real design cases, hand-authored rather than copied from `Default`:

- **en1996** — a 240 mm clay-unit loadbearing pier, M5 general-purpose mortar, MX1 exposure, DE
  annex, `Class3` control, 2 storeys, R60.
- **en1997** — a 2.0 m square pad footing 1.5 m deep on φ'=30° / c'=0 sand (γ = 18 kN/m³), plus a
  0.5 m bored pile 12 m long (q_s = 80 kPa, q_b = 2500 kPa), approach `da1str`.
- **en1995** — a 200 × 300 mm GL24h glulam beam (W = 3 000 000 mm³, A = 60 000 mm²), service class 1,
  medium-term duration, R30, 500 000 bridge fatigue cycles.
- **iso16757** — a compact radiator catalogue: 1 group, 1 class, 1 series, 1 product, 1 property
  definition, 1 dictionary subject, 2 selection constraints, 2 part-number inputs, a `Literal`
  part-number rule and the default 10000/64/50 script budgets.

## 🎯️ What each tree's cases actually pin

**en1996 / en1997 / en1995** are pure `change-<field>` vocabularies over flat, id-less document
roots, so every case is the same *shape* but a different *claim*. Each one asserts, by name:

1. the mutated field lands on its own new value;
2. a deliberately chosen **coupled sibling** does NOT move — the physics of the pairing is what makes
   the assertion load-bearing, not the fact that a sibling exists. Examples: `change-m-ed-knm` pins
   that `z_mm3` (the resistance side of the same flexure check) stays; `change-fire-resistance-min`
   pins that `wall_thickness_mm` is NOT auto-raised to satisfy the new EN 1996-1-2 Table 5.1 demand;
   `change-exposure` pins that M5 mortar SURVIVES the move to MX3 so the EN 1996-2 Annex B durability
   check can *fail* instead of the mutation hiding it; `change-h-mm` (en1995) and
   `change-section-depth-mm` are mirror images pinning that the geometric depth and the k_h
   size-effect depth are two independent fields that merely start equal; `change-bm`/`change-b-mm`
   pin that b, h, A and W are four independent declared inputs and no identity is back-solved;
   `change-pile-lm` pins that `z_investigated_m` (what the ground investigation actually reached)
   never follows design geometry;
3. exactly which guard branch of that leaf's oracle was bypassed and why (`mutation.invariant`
   finiteness fatal + `mutation.no-op` equality warning, or equality-only where the field is a
   `String`/enum/integer — `change-storeys`, `change-fire-resistance-min`, `change-unit`,
   `change-annex`, `change-masonry-class`, `change-design-situation`, `change-exposure`,
   `change-mortar`, `change-pile-n-profiles`, `change-design-approach`, `change-service-class`,
   `change-load-duration` have **no** finiteness guard, and each of those tests says so).

`change-c-kpa` (en1997) is the one case whose committed BEFORE value is `0.0`; it exists to pin that
the oracle's guard is a plain `base == payload` equality and does not special-case a falsy zero.

**iso16757** spans eight verbs and is where the interesting behaviour lives:

| leaf | case | what it pins |
| --- | --- | --- |
| `🌱change-part-number-input` | `raises-the-height-part-number-input-to-750` | clone-and-insert: the sibling `length` entry survives, the map does not grow |
| `🌿remove-part-number-input` | `drops-the-length-part-number-input` | removal is the SHORTER whole map, never a tombstone |
| `🌲rename-catalogue` | `restamps-the-catalogue-as-the-2026-edition` | the German alternative locale and the catalogue id are untouched |
| `🌳rename-manufacturer` | `adds-the-ag-suffix-to-the-manufacturer` | `manufacturer.names` is a DIFFERENT `Names` from `metadata.names` |
| `🌴change-selection-class` | `retargets-the-selection-at-the-towel-radiator-class` | no referential guard — retargeting at a class that does not exist yet is legal |
| `🌼change-selection-series` | `narrows-the-selection-to-the-pr-plus-series` | `Option<String>` payload, `Some`→`Some`; the guard compares whole `Option`s |
| `🛁add-selection-constraint` | `appends-a-width-under-800-constraint` | push lands at index 2; inverse addresses `base.len()` |
| `🛋️remove-selection-constraint` | `drops-the-trailing-length-constraint` | index-addressed; the push-based inverse is exact ONLY for the trailing index (stated in the test) |
| `🌵create-subject` | `appends-a-towel-radiator-subject-under-the-radiator-parent` | `index: null` takes the append arm, so `mutation.clamped` cannot fire |
| `🌻delete-subject` | `removes-the-radiator-subject-from-the-dictionary` | **no cascade**: `product_groups[0].dictionary_subject_id` keeps its now-dangling pointer |
| `🌷update-script-limits` | `doubles-the-step-budget-and-quintuples-the-timeout` | the tree's one `update-<facet>` verb: whole-struct rebuild, whole-struct no-op guard |
| `🌸delete-product` | `removes-the-pr600-product-from-the-catalogue` | no cascade into series or group; inverse is a POSITIONED `CreateProduct` |
| `🌹delete-product-group` | `removes-the-radiators-group-and-strands-its-class` | the class keeps a dangling `group_id` — deliberate |
| `🌺delete-property-definition` | `removes-the-height-property-definition` | both the class requirement and the selection constraint keep naming the deleted id |
| `🌾create-property-definition` | `appends-a-selection-scoped-length-property` | `PropertyKind::Selection` + optional cardinality, distinct from the committed mandatory `Static` |
| `🍀create-product-group` | `appends-a-towel-radiators-group` | an unmapped group stays unmapped (`dictionary_subject_id: null`) |
| `🍁create-product` | `appends-a-pr900-product-to-the-existing-series` | id UNIQUENESS is checked, referential validity is not |
| `🍂replace-part-number-rule` | `swaps-the-literal-rule-for-a-height-driven-script` | whole tagged-enum swap `Literal` → `Script`; inputs and budgets untouched |
| `🍃change-exchange-process` | `advances-the-exchange-stage-to-determine-product` | the only leaf whose diff container is a bare scalar; stage ORDER is not enforced |
| `🚿rename-product-group` | `renames-the-radiators-group-to-panel-radiators` | id is identity and never follows the label; both guards present |
| `🛏️rename-product` | `renames-pr600-to-the-compact-variant-name` | the part-number rule is a different fact from the display name |

`Iso16757Diff` is a per-**container** delta (whole `Catalogue`, whole `Dictionary`, whole
`SelectionRequest`, whole input map), not a per-item patch, so 15 of the 21 committed diffs carry a
full container. Each of those tests asserts *both* that the container carries the change *and* that
a named untouched neighbour rides through it — which is precisely the "rewrote the whole snapshot"
bug the diff assertion exists to catch.

## 🔌️ Wiring — `📦️glue.rs` NOT touched

`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` is shared with the other norm artifact lanes
running concurrently, so it was deliberately left alone. Each artifact self-wires its own cases in a
new `//#region 🧪️FixtureTests` at the bottom of its own mutations-root `🦀️component.rs`:

```rust
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📐change-m-ed-knm/🧪️tests/raises-the-design-bending-moment-to-12-5-knm/🦀️component.rs"]
    mod tests_change_m_ed_knm_raises_the_design_bending_moment_to_12_5_knm;
    // …one pair per case
}
```

A `#[path]` on a module declared at the **top level of a non-mod-rs file** resolves relative to that
file's own directory, and `#[path = "."]` on the enclosing inline module keeps the children anchored
there — the same construction `📦️glue.rs` itself uses (`#[path = "."] pub mod en1996 { #[path =
"../../🗿️artifacts/…"] mod component; }`), and the same one the `🛡️change-merge-policy` facet uses
in the small-plugins lane.

Counts: en1996 22 · en1997 22 · iso16757 21 · en1995 20. Every `#[path]` was resolved against the
filesystem by the emitter and re-checked by the scoped lint.

While adding the region, one clause in each of the four mutations-root module docs was amended from
"no self-wiring `#[path = "."]` blocks are needed here" to "…needed for the TRIADS", because that
sentence became false the moment the fixture region landed. Nothing else in those files changed.

## ✅️ Verification

`cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust && bun ./📜️script.ts fixtures lint --by-tree`:

```
🧬️ 115 artifact mutation trees · 1558 mutations · 1281 covered · 277 uncovered
```

None of the four trees appears anywhere in the `--by-tree` uncovered list, and none appears in the
error list. Because the CLI truncates its error list at 40 rows **repo-wide** (the 299 errors are
other lanes' trees), the lint's own rules were transcribed and re-run scoped to just these four —
`🔧️norm-fixture-authoring/📜️lint-scoped.py`:

```
📘️en1996       0/22 uncovered · 22 case(s) · 22 wired · 0 error(s) · 176 derived-encoding warning(s)
📘️en1997       0/22 uncovered · 22 case(s) · 22 wired · 0 error(s) · 176 derived-encoding warning(s)
📓️iso16757     0/21 uncovered · 21 case(s) · 21 wired · 0 error(s) · 168 derived-encoding warning(s)
📘️en1995       0/20 uncovered · 20 case(s) · 20 wired · 0 error(s) · 160 derived-encoding warning(s)
SCOPED TOTAL ERRORS: 0
```

The derived-encoding warnings are expected and correct: `.op.semio` / `.spr.semio` / `.dsl.semio` /
`.pack.semio` / `.patch.semio` are produced by `fixtures generate` from these core files (contract
D1/D11) and were deliberately not hand-forged.

Structural checks, all green:

- **425 committed JSON files** (85 cases × 5) parse.
- **85 fixture test files + 4 mutations roots** parse under `rustfmt --edition 2021 --emit stdout`.
- Every `include_str!` target resolves on disk (checked by `📜️lint-scoped.py`).
- Every wired `#[path]` resolves, and the wired-module count equals the on-disk case count per tree.
- Coherence audit over all 85 cases: each `➡️after` differs from its `⬅️before` in **exactly one**
  top-level snapshot key, with identical key sets; each `🔺️diff/🔣️component.json` has **exactly one**
  non-null container, that container is the one that changed, and its value equals the `after` value.
- Snapshot/diff key counts match the Rust structs exactly (22/24, 22/24, 20/22, 8/10).

`cargo` was **not** run — the workspace is mid-sweep and out of this lane's scope. **No test is
claimed to pass.** The claim here is structural: the files exist, parse, wire, and are internally
coherent with the oracles they were transcribed from.

## 🔧️ Authoring tooling (ticket scratch, retained)

`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/COMPOSE-TO-PUZZLE5D-MIGRATION/🔧️norm-fixture-authoring/`

| file | role |
| --- | --- |
| `📜️emit-common.py` | path plumbing, emoji-safe leaf-directory resolution, file writing, test-file skeleton |
| `📜️emit-scalar.py` | the seven test bodies for a `change-<scalar>` leaf |
| `📜️table-en1996.py` | the 22 hand-authored en1996 rows (values, case names, guard prose, assertions) |
| `📜️table-en1997.py` | the 22 hand-authored en1997 rows |
| `📜️table-en1995.py` | the 20 hand-authored en1995 rows |
| `📜️table-iso16757.py` | the base catalogue + the 21 hand-authored iso16757 cases |
| `📜️wire.py` | writes each mutations root's `🧪️FixtureTests` region, resolving every path on disk |
| `📜️lint-scoped.py` | the puzzle lint's rules, scoped to these four trees, plus include_str!/`#[path]` checks |

Every value, name, prose fragment and bespoke assertion in the emitted files comes from these
hand-authored tables — the scripts contain no defaulting, no inference from directory names and no
shared runtime helper that survives into the committed Rust. The emitted tests import nothing from
each other.

## 😮 Surprises

1. **Two of the four dispatch enums are externally tagged.** `En1995Mutation` and `Iso16757Mutation`
   carry no `#[serde(...)]` at all, while `En1996Mutation`/`En1997Mutation` carry
   `#[serde(tag = "mutation", rename_all = "camelCase")]`. Two neighbouring artifacts in the same
   plugin, two different wire formats.
2. **iso16757's payload structs have no `rename_all`** either, so its payload keys are snake_case
   (`new_name`, `property_definition`) while every other tree here is camelCase.
3. **en1995's `📐change-annex` leaf is mounted in `📦️glue.rs` under the stale module name
   `set_snapshot`** — a leftover from the deleted whole-document-replace variant. Harmless for the
   fixtures (which address the leaf by directory), but it is a real naming inconsistency.
4. **`remove-selection-constraint`'s inverse is `AddSelectionConstraint`, which PUSHES.** It is only
   an exact inverse for the trailing index; removing an interior constraint would round-trip to a
   different order. The fixture removes the last constraint and says so explicitly rather than
   silently sitting on the working case.
5. **iso16757's delete verbs cascade nothing.** Deleting a subject, a group or a property definition
   leaves dangling `dictionary_subject_id` / `group_id` / `required_property_ids` references behind.
   Three fixtures assert the dangling pointer survives, so the behaviour is pinned rather than
   discovered later.
