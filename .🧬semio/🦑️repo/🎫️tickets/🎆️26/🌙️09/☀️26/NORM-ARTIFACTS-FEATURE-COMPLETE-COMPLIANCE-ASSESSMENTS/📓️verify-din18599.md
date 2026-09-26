# Wave D Verification — DIN V 18599 (`⚡️din18599`) — Round 4

**VERDICT: FAIL (1 blocking)**

Reviewer: adversarial Wave D read-only, round 4. Round 1: **FAIL (9 blocking)**. Round 2: **FAIL (5 blocking)**. Round 3: **FAIL (2 blocking)**. Implementer claim: 102/102 (`📓️impl-din18599.md` Round 3 section). Evidence date: 2026-09-26.

Family root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

---

## Round history (short)

| Round | Verdict | Main gaps |
|-------|---------|-----------|
| R1 | FAIL (9) | id-only paths, stub DSL, hard-coded setActiveExample, missing field-meta choices, single remedy test, no jsonschema/oracle, heuristic limits, trivial example tests |
| R2 | FAIL (5) | `deltaUWbWM2K` path typo, facet drift, no path-resolve test, η_WRG/tabular magic limits, weak `report_out`, identical en/de cooling copy |
| R3 | FAIL (2) | No scope-aware perturbation test; five editable leaves never read by `evaluate()` |
| R4 | **FAIL (1)** | Both R3 code gaps closed; perturbation test runs on **one** subject only — does not loop all committed examples (two-zone + cooled + detached) per CORRECTION 13:43 / R4 step 4 |

---

## Test run (mandated runner)

```
bun nx run @semio-tech/norm-din18599-rs:test --skip-nx-cache -- --no-fail-fast
Summary [   1.580s] 103 tests run: 103 passed, 0 skipped
```

Log: `🗑️generated/verify-din18599/test-r4.txt`.

---

## Check summary (brief §1–10)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **PASS** | Hierarchical snapshot (`📸️snapshot/🔣️.json`). Static audit: 44/44 editable leaves read in `zone_balances`/`balance_for`/`evaluate` (`🗑️generated/verify-din18599/ignored-fields-audit.txt`). `heatedVolumeM3` in H_V infiltration + A/V_e + plausibility (schema L425, L774, L907–951). No building `lighting.powerDensityWM2`. |
| 2 | Clause coverage | **PASS** | Checks: `din18599.1.heated-volume`, `din18599.1.net-floor-area`, `din18599.1.element-zone.*`, `din18599.geg.ht-prime`, `din18599.geg.mean-u.*`, `din18599.2.heating-demand`, `din18599.4.lighting-power.{zoneId}`, `din18599.5.heating-efficiency`, `din18599.6.heat-recovery`, `din18599.7.cooling`, `din18599.8.dhw`, `din18599.geg.qp`, `din18599.11.automation`, `din18599.12.tabular`. Limits from `#region 📜️NormTables`; `norm_table_rows_match_cited_sources` (compliance L294–331). No tautologies found. |
| 3 | Numerics | **PASS** | Hand: H_T=**112.893 W/K**, H′T=**0.239 W/(m²·K)**; zone H_V=**46.648 W/K**; Q_l=**1142.4 kWh/a**; V_e plausibility ΣV/V_e=1.00. Oracle ±0.5 % on compliant/two_zone/cooled/noncompliant. Details: `🗑️generated/verify-din18599/hand-numerics.txt`. |
| 4 | Applicability | **PASS** | Cooling N/A when `plant: None` (schema L1378–1389). Tabular N/A for `DetailedMonthly`. Mean Ū N/A per kind when no elements. |
| 5 | National annex | **PASS** | DE-only family; all checks `AnnexChoice::De` (schema L904). |
| 6 | Report quality | **PASS** | en≠de (cooling L1362, η_sys L1265–1267, DHW L1299–1307). `[id=…]` paths (L751–757, L984–1014, L1223). `every_emitted_path_resolves_via_get_value_at_path` (compliance L355–389). ≥5 remedy-apply tests (L153–351). |
| 7 | Examples | **PASS** | DSL assets 16 lines with zones + elements (`🖼️assets/✅️compliant-detached/🗣️.dsl.semio`). `setActiveExample` loads PRIMARY_TEXT (editor `🎨️set-active-example/🦀️.rs` L19–25). `example_assets_decode_and_match_claimed_verdict` (compliance L257–272). Subjects: `compliant_two_zone_house`, `cooled_office_building` in tests. |
| 7b | Inputs UX | **PASS** | `🏷️field-meta/🦀️.rs`: en+de labels, SI units, `NormFieldChoice` for carriers/profiles/kinds. `every_default_leaf_has_en_de_label` (L139–145). Structured editor `render_document_editor` (`📥️inputs/🦀️.rs` L25–30). |
| 8 | Mutations & schema | **PASS** | Semantic verbs incl. `specify-heating-system` / `specify-dhw-system` (mutations L67–68). Facets aligned on `deltaUWbWM2k`. Snapshot TS: no `Record<string, unknown>` (guard-only in inference text TS). |
| 9 | Tests | **FAIL** | 103/103 executed, 0 skipped. Oracle + jsonschema + remedy-law + path-resolve present. **Gap:** `every_editable_leaf_influences_a_check` uses only `cooled_office_building()` (compliance L438–442) — does not perturb leaves in `compliant_two_zone_house()` or `compliant_detached_house()` per R4 step 4 / CORRECTION 13:43. |
| 10 | Stubs | **PASS** | No `todo!`/`unimplemented!` in family `.rs`. Comment-only “placeholder” in mutate feature prose (`🧪️tests/⚡️mutate-din18599-1/🦀️.rs` L119). |

---

## Round-3 blocking items (re-check)

| R3 # | Item | Round 4 | Evidence |
|------|------|---------|----------|
| 1 | Wire ignored editable fields into `balance_for`/`evaluate()` | **FIXED** | `heatedVolumeM3` L425,L774,L907+; `zones[].usageProfile` L421,L444,L760–762; `zones[].volumeM3` L423,L438,L760; `zones[].lightingPowerWM2` L469,L1217–1245; `elements[].zoneId` L398–399,L431–478,L984+; static audit 0 unread |
| 2 | Scope-aware `every_editable_leaf_influences_a_check` | **PARTIAL** | Test exists and passes on `cooled_office_building()` (L438–512) — covers cooling.plant.* and Office profile. **Not** run on `compliant_two_zone_house()` (second zone JSON paths) or `compliant_detached_house()` (WFH residential scope) |

---

## Coordinator decisions A–F (re-check)

| Dec | Requirement | Round 4 | Evidence |
|-----|-------------|---------|----------|
| A | Zone monthly balance; `elements[].zoneId`; dangling → Fail + one_of; field meta zone choices | **PARTIAL** | `zone_balances` L431–478; zoneId check L984–1017 with `Remedy::one_of` L1007–1014. **`elements[].zoneId` field-meta `choices: None`** (field-meta L104) — zone ids not exposed as editor choices |
| B | `heatedVolumeM3` = V_e; Σ net ≤ V_e; net/gross ≥ 0.80 | **FIXED** | `DIN_V_18599_1_NET_TO_GROSS_VOLUME_RATIO` L709; check `din18599.1.heated-volume` L907–951; infiltration on V_e L425 |
| C | Per-zone lighting; remove building `powerDensityWM2` | **FIXED** | `LightingSystem` control-only (root L217–219); per-zone checks L1217–1245; no `powerDensityWM2` in schema/DSL |
| D | Cooling discriminated `Option<CoolingPlant>`; cooled example; perturb cooling leaves | **FIXED** | `CoolingSystem.plant: Option<CoolingPlant>` (root L191–210); `cooled_office_building` L484–498; perturbation base uses cooled (compliance L442) |
| E | Rename `update-heating`/`update-dhw` → semantic verbs | **FIXED** | `specify-heating-system` / `specify-dhw-system` (mutations/🔥specify-heating-system, 🚿specify-dhw-system) |
| F | Two-zone / profile / oracle / jsonschema | **FIXED** | `two_zone_moving_element_changes_zone_balances` L392–423; `usage_profile_change_changes_zone_hours_and_gains` L426–435; oracle + jsonschema on compliant/noncompliant/two_zone/cooled (`🔬️oracle/🦀️.rs` L18–70) |

---

## Round-2 / Round-1 still-relevant items

All nine R1 and five R2 blockers remain **FIXED** (unchanged from R3 re-check; spot-verified: `deltaUWbWM2k` L826+, path-resolve L355+, table asserts L294+, `report_out` editor L231–241).

---

## CORRECTION 13:27 — 12 recurring causes

| # | Cause | Result | Evidence |
|---|--------|--------|----------|
| 1 | Human en+de `NormFieldChoice` labels | **PASS** | `ENERGY_CARRIER`, `USAGE_PROFILE`, `BOOL_YES_NO` (field-meta L48–60) |
| 2 | Every editable leaf has meta + test | **PASS** | `every_default_leaf_has_en_de_label` (field-meta L139–145) |
| 3 | Structured Inputs, not JSON dump | **PASS** | `render_document_editor` + `field_meta` (inputs L25–30) |
| 4 | Entity paths `[id=…]` + path resolution test | **PASS** | compliance L355–389; zone/element paths in evaluate |
| 5 | ≥2 remedy-apply tests; applicable writable targets | **PASS** | Five+ apply tests (compliance L153–351) |
| 6 | Example tests decode DSL + verdict | **PASS** | compliance L257–272 |
| 7 | Python oracle + third-party jsonschema | **PASS** | oracle/🦀️.rs L18–70; no skip hatch |
| 8 | Facets regenerated, no drift | **PASS** | `deltaUWbWM2k` lockstep; snapshot TS clean |
| 9 | No tautologies / ignored editable fields | **PASS** | Static audit 0 unread (`ignored-fields-audit.txt`) |
| 10 | No trivially-true tests | **PASS** | `report_out` asserts complies + ids + numerics (editor L231–241) |
| 11 | Semantic mutation verbs | **PASS** | `specify-heating-system`, `replace-elements`, … |
| 12 | Dynamic issue text localized | **PASS** | Cooling L1362 en/de differ; η_sys L1265–1267 |

---

## CORRECTION 13:43 — perturbation test + ignored-fields audit

### Perturbation test

| Requirement | Result | Evidence |
|-------------|--------|----------|
| Test exists and perturbs editable leaves | **PASS** | `every_editable_leaf_influences_a_check` (compliance L438–512); asserts `unchanged.is_empty()` |
| Scope-aware: cooling leaves in cooled example | **PASS** | Base = `cooled_office_building()` (L442); `cooling.plant.eer` / `energyCarrier` present |
| **Complete across all committed examples (two-zone + cooled + detached)** | **FAIL** | Test runs **only** on `cooled_office_building()`. Does not loop `compliant_two_zone_house()` (zones[1].*, multi-zone zoneId splits) or `compliant_detached_house()` (WFH residential). Dedicated integration tests (L392–435) do not substitute per-leaf perturbation |
| Exempt only descriptive labels | **PASS** | Exempts `labelEn`/`labelDe`/`id`/climate composition (L456–465) |
| Assert ≥1 check status/utilization change | **PASS** | Signature compare on `(id, status, utilization)` (L504–510) |

### Static audit — editable leaves vs `evaluate()` reads

Full audit: `🗑️generated/verify-din18599/ignored-fields-audit.txt`.

| Path group | Read by checks? | Notes |
|------------|-----------------|-------|
| `heatedVolumeM3` | **YES** | Infiltration L425; A/V_e L774; plausibility L907–951 |
| `zones[].usageProfile` | **YES** | `usage_profile_row` L421,L444; fan weighting L760–762 |
| `zones[].volumeM3` | **YES** | H_V profile term L423; volume share L438 |
| `zones[].lightingPowerWM2` | **YES** | Q_l L469; part-4 checks L1217–1245 |
| `elements[].zoneId` | **YES** | `elements_for_zone` L398–399; zone balances L431–478; linkage checks L984+ |
| `zones[].labelEn/labelDe`, `elements[].labelEn/labelDe` | EXEMPT | Descriptive entity labels |
| `zones[].id`, `elements[].id` | EXEMPT | Entity identifiers for `[id=…]` paths |
| `climate.childId`, `climate.target`, … | EXEMPT | Composition handles; climate via `din18599_climate()` |

**R3 five-leaf gap: FIXED** (0 potentially unread leaves).

---

## Numeric derivations (≥3)

See `🗑️generated/verify-din18599/hand-numerics.txt`. Summary:

1. **H_T / H′T:** 112.893 W/K → 0.239 W/(m²·K) vs limit 0.40.
2. **H_V (zone + WFH profile):** 46.648 W/K with η=0.80 (formula at schema L420–428).
3. **Q_l (DIN V 18599-4/-10):** 1142.4 kWh/a from zone LPD × t_L(WFH)=1700 h/a × control 0.8.
4. **V_e plausibility:** ΣV_zone/V_e = 1.00 ≥ 0.80.

18599-10 profile constants sourced in `#region 📜️NormTables` (L695–706) and asserted L311–313.

---

## Blocking fix list

1. **`…/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — Extend `every_editable_leaf_influences_a_check` to loop all committed subject fixtures** (do not delete fields or narrow scope):
   - Run the existing perturbation loop on **`cooled_office_building()`** (cooling.plant.* scope), **`compliant_two_zone_house()`** (second zone + multi-zone zoneId scope), and **`compliant_detached_house()`** (WFH residential scope).
   - For each fixture: walk JSON leaves, apply the same exempt list (descriptive `labelEn`/`labelDe`/`id` + climate composition handles only), perturb, assert ≥1 check `(id, status, utilization)` change vs that fixture’s baseline.
   - Must fail until every leaf in every fixture influences a check (same strictness as current `unchanged.is_empty()` per fixture, or document-equivalent union coverage with explicit per-fixture assert messages).

---

## Non-blocking observations

- R3 core evaluate wiring is credibly closed; test count 99→103 with zone/volume/plausibility coverage.
- Decision A remedy `one_of` for dangling `zoneId` works (schema L1007–1014); static `NormFieldMeta` still lacks dynamic zone-id choices for the inputs editor (field-meta L104) — consider B2 dynamic-choice hook.
- Dead helpers `fan_operating_hours_a(use_class)` / `lighting_power_density_limit_w_m2(use_class)` (schema L716–730) unused by evaluate path; fan/lighting now zone-profile-driven.
- `two_zone` / `cooled` subjects exist in tests/oracle but are not DSL `setActiveExample` entries (only compliant/noncompliant/demo) — UX gap only.
- Impl md “102/102 no gaps” overstated until perturbation loops all committed examples.

---

*Auditor: adversarial Wave D read-only · round 4 · 2026-09-26 · Family `⚡️din18599`*
