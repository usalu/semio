# Verify — EN 1999 (Wave D, Round 3)

VERDICT: FAIL (8 blocking)

## Round history

| Round | Verdict | Notes |
|-------|---------|-------|
| R1 | FAIL (8 blocking) | Remedy paths, silent alloy, fire k_θ, missing 1-4/1-5, stale DSL, oracle/jsonschema, weak remedy-law, empty field-meta |
| R2 | FAIL (7 blocking) | EN 1990 absent, ρ_u unused, cold-formed discard, shell χ=0.70, single-slope fatigue, remedy-law, ignored leaves, identical en/de |
| R3 | **FAIL (8 blocking)** | R2 formula gaps largely closed; **TS/GraphQL facets still pre-refactor**; perturbation gate weak; governing combination not reported; identical en/de persists; no SLS |

**R3 runner:** `bun nx run @semio-tech/norm-en1999-rs:test --skip-nx-cache -- --no-fail-fast` → **Summary [0.471s] 65 tests run: 65 passed, 0 skipped** (log: `🗑️generated/verify-en1999/test-r3.txt`).

## Round 2 blocking item re-check

| # | Round-2 blocker | R3 | Evidence |
|---|-----------------|-----|----------|
| 1 | EN 1990 action model (no hand-typed member design effects) | **FIXED** (members) / **PARTIAL** (connections) | `part_en1990::governing_member_effects` ULS 6.10a/b (`🧬️schema/🦀️.rs:333–394`); `check_member` uses `gov.n_ed`…`m_z_ed` (`873–875`). Connections: `n_k * 1.35` / `v_k * 1.50` proxy when explicit (`1271–1279`), not full combination engine. |
| 2 | `ρ_u,haz` in HAZ net checks | **FIXED** | `effective_area` applies `rho_u_haz` + `weld_position` (`549–563`); weld resistance uses ρ_u (`1347`); `haz_rho_u_governs_welded_net_section` test (`🧪️tests/⚖️compliance/🦀️.rs:424–431`). |
| 3 | Cold-formed `nEd`/`welded`/`span` discarded | **FIXED** | Axial (`1669–1695`), support/web crippling via `span` (`1697–1729`), N–M (`1731–1762`); no `let _ =` discard. |
| 4 | Shell meridional `χ` hardcoded 0.70 | **FIXED** | `σ_x,Rcr`, `λ̄_x`, `χ_x` from geometry (`1798–1807`); `shell_chi_from_geometry_hand_value` (`🧪️tests/⚖️compliance/🦀️.rs:435–453`). |
| 5 | Fatigue single `slopeM` | **FIXED** | `FatigueDetail.m1`/`m2` (`📸️snapshot/🦀️.rs:188–191`); bi-linear `fatigue_strength_pa` / `damage_ratio` (`729–761`); `fatigue_strength_at_5e5` test. |
| 6 | Remedy-law ≥2 fail→pass | **FIXED** | `remedy_law_writing_required_improves_fail` applies ≥2 remedies, re-evaluates, asserts Pass/u≤1 per targeted id (`196–240`). |
| 7 | Unread editable leaves | **PARTIAL** | `every_editable_leaf_influences_a_check` passes via **ratio gate** (`518–521`), not per-leaf; hardcoded exemptions for `.outerDiameter`, `.bolts.material` (`513–516`). |

## Round 1 item re-check (still relevant)

| # | Round-1 blocker | R3 | Evidence |
|---|-----------------|-----|----------|
| 1 | `[id=…]` remedy paths | **PASS** | `remedy_paths_use_id_selectors_and_resolve_in_snapshot` (`173–193`). |
| 2 | Unknown alloy silent default | **PASS** | `resolve_alloy` → `None`; `unknown_alloy_emits_fail_with_oneof_catalogue`. |
| 3 | Fire without `k_θ` | **PASS** | `k_theta` on `N_fi`/`M_fi` (`1441–1447`); fire test. |
| 4 | Missing `coldFormed[]`/`shells[]` | **PASS** | Snapshot + evaluate + entity tests. |
| 5 | Stale DSL/pack | **PASS** | `bundled_example_assets_match_regenerated_dsl_and_pack`. |
| 6 | Oracle ±0.5% + jsonschema | **PASS** | Both tests run; oracle overlap limited to 4 id patterns (`294–297`). |
| 7 | Remedy-law ≥2 | **PASS** | See R2 #6. |
| 8 | `empty_field_meta` | **PASS** | `field_meta_covers_every_editable_leaf_en_de`; stale orphan rows remain (see 13:43). |

## Brief checks (1–10 + 7b)

| Check | Result | Evidence |
|-------|--------|----------|
| 1 Subject completeness | **PARTIAL** | Members: characteristic `MemberAction` + EN 1990 ULS (`📸️snapshot/🦀️.rs:57–87`, `governing_member_effects`). **Governing combination computed but discarded** (`876: let _ = gov.combination`). Connections use γ·N_k proxy (`1271–1279`). Cold-formed/shells still carry hand-typed design scalars (`m_ed`/`n_ed`, `sigma_x_ed`/`sigma_theta_ed`). **No SLS** combinations anywhere (`rg SLS` → none). |
| 2 Clause coverage | **PASS** (formulas) | Parts 1-1…1-5 reached with real formulas; no `χ=0.70` hardcode; bi-linear fatigue. |
| 3 Numerics | **PASS** | Hand recompute in `🗑️generated/verify-en1999/hand-numerics.txt`: χ(λ̄=1)=0.656, aw6082 ρ_o/ρ_u, Δσ_R@5×10⁵≈98.01 MPa, shell χ_x from geometry, M_c,Rd≈5.67 kNm — all within ±0.5 %. |
| 4 Applicability | **PASS** | Empty-list N/A gates; cold/shell NA when absent. |
| 5 National annex | **PASS** | γ_Mf DE 1.35 vs EN 1.0 (`de_and_en_gamma_identical` skips fatigue); aluminium γ_M1/γ_M2 identical EN/DE documented. |
| 6 Report quality | **FAIL** | Paths/id grammar OK; remedies on fails. **≥7 check explanations use identical en/de format strings** (`923–926`, `958–961`, `994–997`, `1126`, `1160`, `1202`, `1338–1341`, `1847–1848`, `1874–1875`). **Governing ULS combination not named** in report (`876`). |
| 7 Examples | **PASS** | Compliant + multi-fail decode/evaluate; DSL/pack drift green. |
| 7b Inputs UX | **PARTIAL** | Structured editor test green; field-meta covers snapshot leaves. **Stale meta rows** `members[].actions[].vYEd`/`vZEd`/`mZEd` (design labels) do not match snapshot `vYK`/`vZK`/`mZK` (`🏷️field-meta/🦀️.rs:95–98`). `CONN_KINDS` omits `combined` though examples use it (`📸️snapshot/🦀️.rs:377`). |
| 8 Mutations & schema | **FAIL** | Semantic mutations OK. **JSON Schema (`📸️snapshot/🔣️.json`) matches Rust** (nK, m1/m2, coldFormed, shells). **TS + GraphQL facets are stale pre-refactor**: `MemberAction` still `nEd/vYEd/mYEd` (`📸️snapshot/🟦️.ts:13`, `🔗️.graphql:13`); `FatigueDetail.slopeM` (`🟦️.ts:19`); `En1999Snapshot` missing `coldFormed`/`shells` (`🟦️.ts:1–9`); connections still `nEd`/`vEd`. Guard helpers use `Record<string, unknown>` (`📸️snapshot/📝️text/🟦️.ts:20–21`). |
| 9 Tests | **PASS** | 65 executed, 0 skipped; numeric/oracle/jsonschema/remedy tests assert values. |
| 10 Stubs | **PASS** | No `todo!`/`#[ignore]` in evaluate path; catalogue panel placeholder only (`✏️editor/📌️panels/📚️catalogue/🦀️.rs:3`). |

## CORRECTION 13:27 (12 causes)

| # | Cause | R3 |
|---|-------|-----|
| 1 | Choice labels en+de | **PASS** — `🏷️field-meta/🦀️.rs:9–49` |
| 2 | Every editable leaf meta | **PASS** — walk test on both examples |
| 3 | Structured editor | **PASS** — `renders_the_structured_document_editor` |
| 4 | `[id=…]` paths resolve | **PASS** |
| 5 | ≥2 fail→pass remedy apply | **PASS** |
| 6 | Example decode + verdicts | **PASS** |
| 7 | Oracle + jsonschema | **PASS** |
| 8 | Facets regenerated | **FAIL** — TS/GraphQL/proto out of sync with Rust/JSON (see check 8) |
| 9 | No tautologies / ignored inputs | **PARTIAL** — perturbation uses 50% allowance; `rho_o_haz` explicitly discarded in `effective_area` (`551: let _ = rho_o_haz`) though ρ_o used in `effective_wel_y` |
| 10 | No trivially-true tests | **PASS** |
| 11 | Semantic mutation names | **PASS** |
| 12 | Localized dynamic text | **FAIL** — identical en/de formula copies (see check 6) |

## CORRECTION 13:43 (perturbation + structural actions)

| Item | Result | Evidence |
|------|--------|----------|
| Scope-aware perturbation | **FAIL** | `every_editable_leaf_influences_a_check` runs **only** on `compliant_roof_purlin()` (`456–457`); gate `unchanged.len() * 2 < active_leaves` allows ~half unchanged (`518–521`); exemptions `.outerDiameter`, `.bolts.material` (`513–516`) beyond id labels. Does not perturb cold-formed/fire/HAZ leaves in a dedicated example per ADDENDUM. |
| N/A-in-default not exempt | **FAIL** | `outerDiameter` exempt though on schema; `gKLine`/`qKLine` on `source=external` actions likely no-op in default example (see `🗑️generated/verify-en1999/ignored-fields-audit.txt`). |
| Leaf set vs field-meta | **PARTIAL** | Snapshot leaves covered by meta walk test. **Orphan meta paths** `vYEd`/`vZEd`/`mZEd` never appear in JSON snapshot. |
| Static ignored-fields audit | **PASS** (reads) | `ignored-fields-audit.txt`: no leaf wholly absent from `schema.rs` string search; perturbation gate is the gap. |
| Governing combination reported | **FAIL** | `gov.combination` discarded (`876`). |
| SLS + fatigue spectra | **FAIL** | ULS + fire ψ₂ only; no SLS checks; fatigue is single-block Δσ_Ed not multi-range spectrum. |

**Explicit perturbation exemptions (allowed: descriptive `id` only):**
- Test exempts: `sections[].outerDiameter`, `connections[].bolts.material`
- Test allows: any leaf failing ratio gate (up to ~50% of active leaves)

## EN 1999 scope spot-check

| Scope item | R3 | Notes |
|------------|-----|-------|
| 1-1 alloys Table 3.2, HAZ ρ_o/ρ_u | **OK** | Catalogue from Table 3.2 (`436–454`); ρ_u in net section |
| EN 1990 ULS combinations | **OK** (members) | 6.10a/b + governing pick |
| EN 1990 governing combination in report | **FAIL** | Not emitted |
| SLS combinations | **FAIL** | Not implemented |
| 1-2 fire k_θ, durationS | **OK** | `theta_eff` from `duration_s` (`1438–1440`) |
| 1-3 bi-linear fatigue m1/m2 | **OK** | |
| 1-4 cold-formed | **OK** | nEd, welded, span wired |
| 1-5 shell χ | **OK** | Computed; `length` in σ_θ,Rcr (`1799`) |
| Facets parity Rust↔TS↔GraphQL | **FAIL** | TS/GraphQL stale |

## Blocking fix list

1. **`🧬️schema/📸️snapshot/🟦️.ts`, `🧬️schema/🔗️.graphql`, `🧬️schema/🟦️.ts`, proto leaves** — Regenerate all non-JSON facets from current Rust snapshot: `MemberAction` must use `id/kind/category/source/gKLine/qKLine/nK/vYK/vZK/mYK/mZK` (not `nEd/vYEd/…`); `AluminiumConnection` → `nK`/`vK`; `FatigueDetail` → `detailCategory/m1/m2` (remove `slopeM`); `En1999Snapshot` must include `coldFormed` and `shells`. Add a parity test that fails when TS/GraphQL field names diverge from `📸️snapshot/🔣️.json`.

2. **`🧪️tests/⚖️compliance/🦀️.rs:456–521` — scope-aware perturbation** — Replace ratio gate with **per-leaf** assert: for each editable leaf in the committed example where the field is applicable, perturbation must change ≥1 check utilization/status/explanation. Run on `compliant_roof_purlin` **and** ensure part-specific leaves (cold-formed, shell, fire HAZ, fatigue) are perturbed in that example (already present). Remove `.outerDiameter`/`.bolts.material` exemptions unless proven non-applicable with a localized N/A reason; wire `outerDiameter` into tube checks or move to discriminated `tube` section kind only.

3. **`🧬️schema/🦀️.rs:876` + member report text** — Emit `gov.combination` and governing `action_id` in every member ULS check `explanation` (en + de, distinct German wording). Add test asserting explanation contains `uls-610a` or `uls-610b` for multi-variable load case.

4. **`🧬️schema/🦀️.rs` — identical en/de explanations** — Localize all formula explanation pairs currently using the same `format!` template for en and de (minimum: `923–926`, `958–961`, `994–997`, `1126`, `1160`, `1202`, `1338–1341`, `1847–1848`, `1874–1875`). German must use engineering terms (e.g. „Kehlnahtdicke“, „Biegedrillknicken“, not English unit strings only).

5. **`🧬️schema/🦀️.rs` + `part_en1990`** — Implement EN 1990 **SLS** combination family (characteristic / frequent / quasi-permanent per DE NA) for member deflection/stress service checks, or dedicated SLS check ids gated when service limits are modelled; structural-family rule requires SLS alongside ULS.

6. **`✏️editor/🏷️field-meta/🦀️.rs:95–98`** — Remove stale `vYEd`/`vZEd`/`mZEd` rows; ensure `vYK`/`vZK`/`mZK` labels say “characteristic” (not “design”). Add `combined` to `CONN_KINDS` (`57–60`) to match evaluate + examples.

7. **`🧬️schema/🦀️.rs:1271–1279`** — Connection design effects must come from the same EN 1990 governing engine (or explicit characteristic connection load cases combined per 6.10), not `n_k * 1.35` / `v_k * 1.50` scalars when `n_k`/`v_k` are characteristic inputs.

8. **Generated TS guard helpers (`📝️text/🟦️.ts` et al.)** — Regenerate without `Record<string, unknown>` bare-object guards per CORRECTION 13:27 #8; use typed snapshot interfaces.

## Non-blocking observations

- Round 3 closes all seven Round-2 **formula** blockers; evaluate path is materially complete for ULS member/shell/cold-formed/fatigue/fire.
- JSON Schema anchor (`📸️snapshot/🔣️.json`) is current; jsonschema test validates against it while TS consumers would deserialize wrong shapes.
- Oracle compares only `.6.2.3.n.`, `.6.2.5.m.`, `1-2.fire.`, `1-3.fat.` ids — extend after facet fix.
- `effective_area` ignores `rho_o_haz` parameter (`551`) while bending path uses ρ_o in `effective_wel_y` — intentional split but parameter should be used or removed from signature.
- Catalogue panel remains headline placeholder (outside Wave D gate).
