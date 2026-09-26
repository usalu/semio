# Verify — DIN 4108 (`din4108` / 🧱️) adversarial family verification

**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/`

---

## Round history

| Round | Verdict | Blocking |
|-------|---------|----------|
| **Round 1** | **FAIL** | 13 |
| **Round 2** | **FAIL** | 6 |
| **Round 3** | **FAIL** | 5 |
| **Round 4** | **PASS** | 0 |

---

## Round 4

**Verifier:** read-only Wave D Round 4  
**Impl claim:** `📓️impl-din4108.md` — Round 3 section claims all 5 blockers cleared, 96/96 passed  
**Test runs (this round):**

| Target | Command | Summary |
|--------|---------|---------|
| `@semio-tech/norm-din4108-rs:test` | `bun nx run … --skip-nx-cache -- --no-fail-fast` | `96 tests run: 96 passed, 0 skipped` |
| `@semio-tech/norm-artifact-contract-rs:test` | `bun nx run … --skip-nx-cache` | `51 tests run: 51 passed, 0 skipped` |

**Logs:** `🗑️generated/verify-din4108/test-r4-din4108.txt`, `🗑️generated/verify-din4108/test-r4-contract.txt`

**VERDICT: PASS (0 blocking)**

Round 4 re-checks every Round-3 blocker and ADDENDA 14:54/14:42/14:37. All five Round-3 blockers are cleared with file:line evidence; perturbation signature is exactly `(id, status, computed, limit, utilization)`; DIN 4108-10 Table 1 classes, Glaser climate BC, referential integrity, zone H_T aggregation, and catalogue tables are normatively wired and tested. No `let _ =` gaming in evaluate/inference; no fingerprint/epsilon field folds; 96/96 + 51/51 executed with zero skipped.

---

### Round-3 blocking items — re-check

| # | Round-3 item | Round 4 | Evidence |
|---|--------------|---------|----------|
| **1** | Perturbation signature `(id, status, computed, limit, utilization)` only; no explanation-only / ratio slack / over-exemptions | **PASS** | `check_signature` 5-tuple — no `explanation.en` (`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:428–443`). `every_editable_leaf_perturbation_changes_some_check_on_default_snapshot` green on default + timber (`:504–543`). Entity-`id` exemptions only (`:446–450`, `:506–508`). `rg "let _ ="` under `💡️inferences` → 0; evaluate path clean. `rg "fingerprint\|1e-9 \*\|1e-12 \*"` under family evaluate code → 0. |
| **2** | DIN 4108-10 water/tensile/acoustic classes normative (Table 1) | **PASS** | `part_10::{min_water_for,min_tensile_for,min_acoustic_for,check_application}` — `ok_w`/`ok_t`/`ok_a` in `pass` logic and class ranks in `minimum(score, need)` utilization (`🦀️.rs:1894–1925`). Test `din4108_10_property_classes_affect_status_or_utilization` (`compliance/🦀️.rs:627–648`) — wk/tk/sh perturbations change signature. |
| **3** | Glaser `climate` selects boundary conditions / numerics | **PASS** | `part_3::{glaser_winter_t_ext_c,glaser_summer_t_ext_c}` read `ClimateZoneDe` design temps (`🦀️.rs:1116–1127`); `check_glaser` uses them in `t_ext_w`/`t_ext_s` condensate math (`:1155–1160`). Test `glaser_climate_changes_condensation_or_limit` Zone2→Zone1 (`compliance/🦀️.rs:701–710`). |
| **4** | Dangling `materialId`/`zoneId` → Fail + en/de + `one_of`; duplicate ids → Fail | **PASS** | `push_referential_integrity` before clause checks (`💡️inferences/🦀️.rs:157–239`); `push_dangling_ref` Fail + distinct en/de explanation + `Remedy::one_of` (`:127–151`). Tests `dangling_material_id_fails_referential_integrity`, `dangling_zone_id_fails_referential_integrity`, `duplicate_element_id_fails_integrity` (`compliance/🦀️.rs:652–684`). Dangling ids no longer silently NotApplicable — integrity Fail emitted first. |
| **5** | Opaque `elements[].zoneId` in normative computation | **PASS** | `part_2::{zone_transmission_ht_wk,check_zone_transmission_loss}` filter opaque elements by `zone_id` (`🦀️.rs:632–669`); wired in `evaluate` (`💡️inferences/🦀️.rs:244`). Default snapshot has `zone-living` + `zone-utility` (`📸️snapshot/🦀️.rs:59–78`). Test `moving_opaque_zone_id_changes_zone_transmission_loss` (`compliance/🦀️.rs:687–697`). |
| **6** | `reference_tables()` non-empty; shared const; evaluated limit = cell | **PASS** | `✏️editor/📌️panels/📚️catalogue/🦀️.rs:20–101` — 3 tables from `part_4::DESIGN_LAMBDA_ROWS`, `part_2::TABLE3_R_MIN_ROWS`, `part_10::CATALOGUE_APPLICATION_TYPES` with clause ids, distinct en/de titles, units. Tests `design_lambda_table_matches_part4_const`, `catalogue_design_lambda_matches_evaluated_limit` (`catalogue/🧪️tests/🔬️unit/🦀️.rs:34–43`; `compliance/🦀️.rs:714–722`). |

---

### ADDENDA 14:54 / 14:42 / 14:37 — re-check

| Requirement | Result | Evidence |
|-------------|--------|----------|
| **14:54** Catalogue tables non-empty, shared numbers | **PASS** | See blocker #6 above. |
| **14:42** No `let _ =` in evaluate/inference | **PASS** | `rg "let _ ="` under `🧬️schema/💡️inferences` → 0. Mutation inverse stubs excluded per convention. |
| **14:42** Signature excludes explanation | **PASS** | `check_signature` 5-tuple only (`compliance/🦀️.rs:428–443`). |
| **14:42** Dangling `*Id` → Fail + `one_of` | **PASS** | `push_dangling_ref` (`💡️inferences/🦀️.rs:127–151`). |
| **14:42** Duplicate ids → Fail | **PASS** | `push_duplicate_ids` (`💡️inferences/🦀️.rs:88–124`). |
| **14:42** Identical en/de prose | **PASS** | Spot-check: integrity (`'…does not reference…'` / `'…verweist auf kein gültiges Ziel.'`), Glaser (`Climate` / `Klima`), zone H_T (`Zone transmission` / `Zonaler Transmissionswärmeverlust`). |
| **14:37** No fingerprint / field epsilon gaming | **PASS** | `rg` clean on evaluate path; Glaser `1e-9` dry-out tolerance is inside normative pass logic (`🦀️.rs:1166`) — acceptable. |
| **14:37** Perturbation asserts status/computed/limit/utilization | **PASS** | Signature uses all four numeric fields via `to_bits()`; test compares full signature. |

---

### `📓️audit-perturbation-gaming.md` 🧱️ din4108 — re-check

| Audit claim | Round 4 |
|-------------|---------|
| **CLEAN (0 instances)** | **CONFIRMED** — prior Round-3 overturn resolved. |
| Signature `(id, status, computed.to_bits())` | **Stale in audit doc** — actual signature is 5-tuple incl. limit + utilization (`compliance/🦀️.rs:428–439`). Implementation correct; audit text not updated (non-blocking). |
| Spot-check leaves normative | **Complete** — water/tensile/acoustic, climate, zoneId, materialId all normative or integrity-gated. |

---

### CORRECTION 13:27 — recurring causes (12)

| # | Self-check | Result | Evidence |
|---|------------|--------|----------|
| 1 | Human en+de choice labels | **PASS** | `field-meta/🦀️.rs` (`NormFieldChoice`). |
| 2 | Every editable leaf has meta + iteration test | **PASS** | `field_meta_covers_every_editable_leaf_on_default_snapshot` (`compliance/🦀️.rs:237–287`). |
| 3 | Structured editor, not JSON dump | **PASS** | `📥️inputs/🦀️.rs` + `din4108_field_meta`. |
| 4 | `[id=…]` paths + resolve test | **PASS** | `compliance/🦀️.rs:290–309`. |
| 5 | ≥2 fail→pass remedy tests | **PASS** | Thickness, n50, summer, DIN 4108-10 (`compliance/🦀️.rs:319–367`, `:576–605`). |
| 6 | DSL decode + complies / fail_count ≥ 2 | **PASS** | Example tests (Round 2). |
| 7 | Python oracle ±0.5 % + jsonschema | **PASS** | `compliance/🦀️.rs:371–423`. |
| 8 | Facets regenerated, real diff/inverse | **PASS** | 43 mutation kinds; mutate suite (`🧪️tests/🧱️mutate-din4108-1/🦀️.rs`). |
| 9 | No tautologies / hardcodes / ignored fields | **PASS** | Round-3 ignored-field gaps closed (blockers #2–#5). |
| 10 | No trivial tests | **PASS** | Perturbation, integrity, catalogue-lock tests assert concrete values. |
| 11 | Semantic mutation verbs | **PASS** | 43 domain verbs (`🧬️mutations/🦀️.rs:98–142`). |
| 12 | Dynamic localized issue text | **PASS** | Distinct en/de with interpolated numbers. |

---

### Wave-D check table (Round 4)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **PASS** | Hierarchical envelope; all Round-3 leaf/integrity gaps closed. |
| 2 | Clause coverage | **PASS** | 4108-2/3/4/7, ISO 6946, Bbl.2, 4108-10 + integrity checks in `evaluate`. |
| 3 | Numerics | **PASS** | Round 2 hand derivations still valid (U, S, R, f_Rsi, timber R); oracle ±0.5 % test green. |
| 4 | Applicability | **PASS** | Transparent/window → N/A with reason; empty lists → explicit Fail. |
| 5 | National annex | **PASS** (N/A) | DIN national; all checks `AnnexChoice::De`. |
| 6 | Report quality | **PASS** | `[id=…]` paths; remedy-flip tests; localized en/de. |
| 7 | Examples | **PASS** | Compliant + failing DSL decode tests. |
| 7b | Inputs UX | **PASS** | Field meta + structured editor. |
| 8 | Mutations & schema | **PASS** | 43 kinds; facets aligned; oracle lock. |
| 9 | Tests | **PASS** | 96/96 + 51/51 executed, 0 skipped. |
| 10 | Stubs | **PASS** | No `todo!`/`stub`/`placeholder` in compliance evaluate path. |

---

## Blocking fix list

None.

---

## Non-blocking observations

- `📓️audit-perturbation-gaming.md` 🧱️ din4108 section still documents a 3-field signature and predates Round-3/4 fixes — cosmetic doc drift only.
- `🔮️oracles/🔣️.json` rationale text may still say “33 kinds” in places while `vectors` and `KINDS` are 43 — lock test passes.
- `check_design_lambda` still emits `NotApplicable` for unknown `materialId` (`🦀️.rs:1277–1290`) when integrity is bypassed; integrity Fail precedes it in `evaluate` so dangling ids are covered.
- `elements[].zoneId` still echoed in Table 3 / U explanations (`🦀️.rs:807`, `:1445`) in addition to normative zone H_T — redundant prose, not gaming.
- Inputs render test remains weak (`contains(':')`) — editor wiring is real.
- Glaser `1e-9` dry-out comparison (`🦀️.rs:1166`) is numerical tolerance inside normative pass logic — acceptable.

---

## Prior rounds (archived)

### Round 3 summary

**VERDICT: FAIL (5 blocking)** — perturbation signature included `explanation.en`; water/tensile/acoustic/Glaser climate/zoneId explanation-only; missing referential integrity; empty catalogue tables.

### Round 2 summary

**VERDICT: FAIL (6 blocking)** — perturbation absent, ignored leaves, mutate stub, stale oracle vectors, DIN 4108-10 scope gap.

### Round 1 summary

**VERDICT: FAIL (13 blocking)** — initial Wave D audit.
