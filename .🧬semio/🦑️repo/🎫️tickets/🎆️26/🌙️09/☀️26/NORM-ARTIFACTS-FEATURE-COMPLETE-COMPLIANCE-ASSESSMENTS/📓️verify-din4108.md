# Verify — DIN 4108 (`din4108` / 🧱️) adversarial family verification

**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/`

---

## Round history

| Round | Verdict | Blocking |
|-------|---------|----------|
| **Round 1** | **FAIL** | 13 |
| **Round 2** | **FAIL** | 6 |
| **Round 3** | **FAIL** | **5** |

---

## Round 3

**Verifier:** read-only Wave D Round 3  
**Impl claim:** `📓️impl-din4108.md` — Round 2 section claims all 6 blockers cleared, 88/88 passed  
**Test runs (this round):**

| Target | Command | Summary |
|--------|---------|---------|
| `@semio-tech/norm-din4108-rs:test` | `bun nx run … --skip-nx-cache -- --no-fail-fast` | `88 tests run: 88 passed, 0 skipped` |
| `@semio-tech/norm-artifact-contract-rs:test` | `bun nx run … --skip-nx-cache` | `51 tests run: 51 passed, 0 skipped` |

**Logs:** `🗑️generated/verify-din4108/test-r3-din4108.txt` (summary tail; full nx output on runner)

**VERDICT: FAIL (5 blocking)**

Round 3 clears **5 of 6** Round-2 blockers (bb2Type, segments, mutate suite, oracle vectors, DIN 4108-10). The perturbation harness exists and runs green, but **CORRECTION 14:42** re-audit finds the signature still folds in `explanation.en`, several editable leaves are read only into explanation text, and reference ids are silently absorbed (NotApplicable / no check) instead of failing referential-integrity with remedies. The prior `📓️audit-perturbation-gaming.md` **CLEAN** verdict for 🧱️ din4108 is **overturned**.

---

### Round-2 blocking items — re-check

| # | Round-2 item | Round 3 | Evidence |
|---|--------------|---------|----------|
| **1** | Scope-aware perturbation test on `compliant_etics_dwelling()` + timber; signature `(id, status, computed, limit, utilization)`; no explanation-only / ratio slack / over-exemptions | **FAIL** | Test exists: `every_editable_leaf_perturbation_changes_some_check_on_default_snapshot` (`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs:506–544`). **`check_signature` includes `c.explanation.en` as 6th tuple field** (`:428–439`) — violates CORRECTION 14:42; explanation-only edits satisfy the test. Spot-check ignored leaves on default opaque walls: `elements[].layers[].waterClass` / `tensileClass` / `acousticClass` echoed only in `part_10::check_application` explanations (`🦀️.rs:1634–1641`) — **not** in `pass` logic (`:1617–1621` uses only `applicationType` + `compressiveClass`). `elements[].zoneId` on opaque elements only in Table 3 / U explanations (`:641`, `:1214`), not in R/U math. Walk uses `collect_leaf_paths` with `[id=…]` for all array items (`:179–215`) — OK. No ratio slack. Entity-`id` exemptions only (`:447–450`, `:507–508`) — OK. |
| **2** | `thermalBridges[].bb2Type` read normatively (`conform` vs `custom`/`detailed`) | **PASS** | `bb_2::bb2_category` (`🦀️.rs:1371–1377`); `delta_u_wb_limit` reads categories (`:1384–1396`); per-bridge `check_bridge_category` (`:1494–1544`); wired in `evaluate` (`💡️inferences/🦀️.rs:181–184`). Test `flipping_bb2_type_on_default_bridge_changes_report` (`compliance/🦀️.rs:547–555`). |
| **3** | Segment `lambda` / `fraction` / `sd_m` (Glaser) / `materialId` when `segments[]` non-empty; timber test | **PASS** | `layer_resistance_lower` §6.7 (`🦀️.rs:305–360`); `layer_mu_eq` → `sd_m` (`:868–879`, `:905`); `part_4::check_segment_design_lambda` (`:1092–1143`); `layer_surface_mass_kg_m2` (`:429–434`). Test `timber_segment_lambda_and_density_affect_checks` (`compliance/🦀️.rs:558–574`). |
| **4** | No empty mutate stub; cucumber kind count = enum | **PASS** | `mutate_suite_every_kind_fixture_changes_leaf_and_inverse_restores` (`🧪️tests/🧱️mutate-din4108-1/🦀️.rs:24–65`) — applies fixture, asserts inverse restore, covers all `KINDS`. `🥒️.feature:12` “43 kinds”. 43 fixture directories under `🎫️fixtures/🧬️mutations/`. |
| **5** | Oracle `fixtureCoverage.vectors` = `KINDS.len()` | **PASS** | `🔮️oracles/🔣️.json:52` `vectors: 43`; `KINDS` has 43 entries (`🧬️mutations/🦀️.rs:98–142`). Lock test `oracle_manifest_mutation_vectors_match_kinds_len` (`compliance/🦀️.rs:609–624`). |
| **6** | DIN 4108-10 application-class check implemented + tested | **PASS** | `part_10::{APPLICATION_TYPES,required_application_types,check_application}` (`🦀️.rs:1549–1659`); emitted in `evaluate` (`💡️inferences/🦀️.rs:138`). Test `din4108_10_wrong_application_fails_and_remedy_passes` (`compliance/🦀️.rs:577–606`) — DAD on wall eps → Fail + `one_of` WAP/WAB/WAA/WH remedy flips pass. |

---

### CORRECTION 14:42 — additional blocking (this round)

| Requirement | Result | Evidence |
|-------------|--------|----------|
| No `let _ =` dummy bindings in evaluate/inference | **PASS** | `rg "let _ ="` under `🧬️schema/💡️inferences` → 0; evaluate path clean. (Mutation inverse `let _ = base` excluded per audit convention.) |
| Perturbation signature excludes explanation | **FAIL** | `check_signature` 6-tuple ends with `c.explanation.en` (`compliance/🦀️.rs:428–439`). |
| No epsilon / fingerprint gaming in computed values | **PASS** | `rg "fingerprint\|1e-9 \*\|1e-12 \*"` under family evaluate code → 0. |
| Dangling `*Id` → Fail referential-integrity + `one_of` remedy | **FAIL** | Unknown `materialId` → `NotApplicable` (`part_4::check_design_lambda` `:1046–1059`; segment `:1100–1113`) — no Fail, no `one_of` over catalogue ids. Dangling `elements[].zoneId` silently drops window/door contribution in `s_vorhanden_with_elements` filter (`part_2` `:598`) with no integrity check. |
| Duplicate entity ids → Fail | **FAIL** | `evaluate` (`💡️inferences/🦀️.rs:86–186`) emits no duplicate-id scan across `zones` / `elements` / `layers` / `windows` / `bridges` / `segments`. |
| Identical en/de prose | **PASS** (spot) | Explanations use distinct German (`Klima`, `Grenze`, `Wärmebrücke`, …) vs English (`Climate`, `limit`, `Bridge`, …) — e.g. Glaser `:976–983`, bb2 `:1515–1516`. |
| Facet parity (no `unknown` / `Record<string, unknown>` on snapshot) | **PASS** | `📸️snapshot/🟦️.ts` fully typed interfaces; `📸️snapshot/🔣️.json` — no `unknown` / `_placeholder`. Parser-guard `unknown` in `📝️text/🟦️.ts` is codec infrastructure, not snapshot schema. |
| No skip hatches in python/jsonschema tests | **PASS** | `validate_snapshot.py` — no `skip`/`pytest.mark`; `jsonschema_validates_default_and_failing_snapshots` asserts concrete field values (`compliance/🦀️.rs:415–422`). |

---

### `📓️audit-perturbation-gaming.md` 🧱️ din4108 — re-check

| Audit claim | Round 3 |
|-------------|---------|
| **CLEAN (0 instances)** | **OVERTURNED → GAMING (4+)** |
| Signature `(id, status, computed.to_bits())` | **Stale** — actual signature adds `limit`, `utilization`, and **`explanation.en`** (`compliance/🦀️.rs:428–439`). |
| Spot-check leaves normative | **Incomplete** — `waterClass` / `tensileClass` / `acousticClass` / Glaser `climate` label / opaque `zoneId` are explanation-only on default subject. |

**Gaming instances (evaluate path):**

| File:line | Pattern |
|-----------|---------|
| `🦀️.rs:934`, `:976–983` | `climate` → `climate_label` in Glaser explanation only; BC/computed unchanged |
| `🦀️.rs:1634–1641` | `water_class` / `tensile_class` / `acoustic_class` in explanation only; pass uses `application_type` + `compressive_class` only (`:1617–1621`) |
| `🦀️.rs:641`, `:1214` | `element.zone_id` echoed in Table 3 / U explanations for opaque walls — not in R/U limits |
| `compliance/🦀️.rs:428–439` | Perturbation signature accepts explanation-only diffs |

---

### CORRECTION 13:27 — recurring causes (12)

| # | Self-check | Result | Evidence |
|---|------------|--------|----------|
| 1 | Human en+de choice labels | **PASS** | `field-meta/🦀️.rs:9–58` (`NormFieldChoice`). |
| 2 | Every editable leaf has meta + iteration test | **PASS** | `field_meta_covers_every_editable_leaf_on_default_snapshot` (`compliance/🦀️.rs:237–287`). |
| 3 | Structured editor, not JSON dump | **PASS** | `📥️inputs/🦀️.rs` + `din4108_field_meta`. |
| 4 | `[id=…]` paths + resolve test | **PASS** | `compliance/🦀️.rs:290–309`. |
| 5 | ≥2 fail→pass remedy tests | **PASS** | Thickness, n50, summer (`compliance/🦀️.rs:319–367`). |
| 6 | DSL decode + complies / fail_count ≥ 2 | **PASS** | Example tests (Round 2 check 7). |
| 7 | Python oracle ±0.5 % + jsonschema | **PASS** | `compliance/🦀️.rs:371–423`. |
| 8 | Facets regenerated, real diff/inverse | **PASS** | Typed snapshot TS; 43 mutation fixture triads. |
| 9 | No tautologies / hardcodes / ignored fields | **FAIL** | `waterClass` / `tensileClass` / `acousticClass` ignored normatively; Glaser `climate` echo; dangling `materialId`/`zoneId` absorbed. |
| 10 | No trivial tests | **PASS** | Mutate suite + perturbation are substantive (signature flaw is implementation, not empty test). |
| 11 | Semantic mutation verbs | **PASS** | 43 domain verbs (`🧬️mutations/🦀️.rs:98–142`). |
| 12 | Dynamic localized issue text | **PASS** | Distinct en/de with interpolated numbers. |

---

### Wave-D check table (Round 3)

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Subject completeness | **FAIL** | Perturbation + integrity gaps above; `water`/`tensile`/`acoustic` classes editable but not read for pass/fail. |
| 2 | Clause coverage | **PASS** | 4108-2/3/4/7, ISO 6946, Bbl.2, **4108-10** all reached in `evaluate`. |
| 3 | Numerics | **PASS** | Round 2 hand derivations still valid (U, S, R, f_Rsi, timber R). |
| 4 | Applicability | **PASS** | Transparent/window → N/A; empty lists → explicit Fail. |
| 5 | National annex | **PASS** (N/A) | DIN national; all checks `AnnexChoice::De`. |
| 6 | Report quality | **PASS** | `[id=…]` paths; remedy-flip tests; localized en/de. |
| 7 | Examples | **PASS** | Compliant + failing DSL decode tests. |
| 7b | Inputs UX | **PASS** | Field meta + structured editor. |
| 8 | Mutations & schema | **PASS** | 43 kinds; facets aligned. |
| 9 | Tests | **PASS** (runner) / **FAIL** (14:42 harness) | 88/88 executed; perturbation signature + integrity gaps. |
| 10 | Stubs | **PASS** | No empty mutate stub; no `todo!` in compliance path. |

---

## Blocking fix list

1. **`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` — `check_signature`** — Remove `c.explanation.en` from the comparison tuple; signature must be exactly `(check_id, status, computed.to_bits(), limit.to_bits(), utilization.to_bits())` sorted by id. Re-run perturbation test; fix any leaves that then fail.

2. **`🧬️schema/🦀️.rs` — `part_10::check_application`** — Wire `water_class`, `tensile_class`, and `acoustic_class` into normative pass/fail per DIN 4108-10 Table 1 property-class rules (not only explanation echo), **or** remove them from the editable snapshot if out of scope. Add unit test proving perturbing each class changes `status` or `utilization`.

3. **`🧬️schema/🦀️.rs` — `part_3::check_glaser`** — Either use `climate` in boundary selection / limit logic normatively, **or** stop binding `climate_label` into explanation (`:934`, `:976–983`) and drop `climate` parameter if Annex A BC are fixed.

4. **`🧬️schema/💡️inferences/🦀️.rs` + `🦀️.rs` — referential integrity** — Before clause checks, emit explicit **Fail** checks (en+de explanation, `one_of` remedy over valid target ids) when: (a) `layers[].materialId` or `segments[].materialId` is not in `part_4::design_lambda` catalogue; (b) `elements[].zoneId` does not match any `zones[].id`; (c) duplicate `id` within any entity list. Dangling perturbation must change **status/computed/limit/utilization**, not silently flip to NotApplicable. Add tests: dangling `materialId`, dangling `zoneId`, duplicate `elements[].id`.

5. **`🧬️schema/🦀️.rs` — opaque `elements[].zoneId`** — If zone linkage is in scope, use `zone_id` in at least one normative computation for opaque elements (e.g. aggregate reporting), not only explanation strings on Table 3 / U checks (`:641`, `:1214`).

---

## Non-blocking observations

- Round-2 functional landings (bb2Type categories, ISO 6946 §6.7 segments, 43-kind mutate suite, oracle vector lock, DIN 4108-10 check + remedy) are solid and tested.
- `oracle manifest` rationale text still says “33 kinds” in places (`🔮️oracles/🔣️.json:17`) while `vectors` and `KINDS` are 43 — cosmetic doc drift only (lock test passes).
- `softwood` segment `materialId` is absent from `design_lambda` catalogue (`🦀️.rs:1020–1035`) — currently NotApplicable; referential fix (#4) should clarify catalogue vs Fail.
- Glaser `1e-9` in dry-out comparison (`:965`) is a numerical tolerance inside normative pass logic — acceptable.
- Inputs render test remains weak (`contains(':')`) — editor wiring is real.

---

## Prior rounds (archived)

### Round 2 summary

**VERDICT: FAIL (6 blocking)** — see git history / prior content. Cleared 10/13 Round-1 items; remaining: perturbation absent, ignored leaves, mutate stub, stale oracle vectors, DIN 4108-10 scope gap.

### Round 1 summary

**VERDICT: FAIL (13 blocking)** — initial Wave D audit.
