# Verify — EN 1992 (🏛️) adversarial Wave D

**Round history:** R1 **FAIL (14 blocking)** · R2 **FAIL (10 blocking)**  
**Runner:** `bun nx run @semio-tech/norm-en1992-rs:test --skip-nx-cache -- --no-fail-fast`  
**Summary:** `[   0.505s] 83 tests run: 83 passed, 0 skipped`  
**Impl claim:** `📓️impl-en1992.md` — 83/83, 0 gaps (**rejected**)

**VERDICT: FAIL (10 blocking)**

---

## Check table

| # | Check | Result | Evidence |
|---|-------|--------|----------|
| 1 | Subject completeness | **PARTIAL / FAIL** | Hierarchical subject with characteristic `LoadCaseActions` (`🦀️.rs` L252–270) and EN 1990 combination in `combine_member_actions` (`🧬️schema/🦀️.rs` L859–978). Beam derives from line loads; column/slab/wall use `mK`/`vK`/`vKPunch` external/FEM path (`L836–857`). **Still blocking:** anchors store hand-typed `nEd`/`vEd` design effects (`🦀️.rs` L357–368, `evaluate_anchor` L1693–1737); no prestressed member in any committed example; `title` bound but unused (`L1053`). |
| 2 | Clause coverage | **PARTIAL / FAIL** | 30+ check templates in `evaluate_member`/`evaluate_anchor`. R1 gaps largely closed: c_min,b (`L1024–1026`), SLS §7.2 (`L1496–1558`), anchorage/laps (`L1561–1614`), DE l/d caps (`L1329–1334`), λ_lim DE (`L604–607`, `L1363`), punching DE NA (`L1168–1242`). **Remaining:** accidental ULS uses `AnnexParams::for_choice` only — `for_situation(..., "accidental")` with γ_c=1.3 never called (`L317–325` vs flexure L1057); fire tabulated functions include 5.2b/5.3/5.9/5.11 (`L669–710`) but `required_for` routes column→5.2a only, slab→one-way only (`L713–725`). |
| 3 | Numerics | **PASS** | Hand recompute in `🗑️generated/verify-en1992/hand-numerics.txt`: V_Rd,c,DE=69.91 kN, M_Rd,DE=208.1 kNm, c_nom=30 mm, λ_lim=25, fire a=35 mm — all within 0.5 % of tests (`🧪️tests/⚖️compliance/🦀️.rs` L12–47). DE NA C_Rd,c=0.15/γ_c, α_cc=0.85 sourced in `na_de::AnnexParams` and `shear_v_rd_c_n`. |
| 4 | Applicability | **PASS** | `NotApplicable` for missing ULS (`L1245–1251`), TC0 liquid (`L1657–1662`), torsion gated (`L1144`). No skipped tests. |
| 5 | National annex DE vs EN | **PARTIAL / FAIL** | α_cc, C_Rd,c, cotθ, cover tables tested (`🧪️tests/⚖️compliance/🦀️.rs` L12–38, L154–168). Accidental γ_c=1.3/γ_s=1.0 **defined** (`🧬️schema/🦀️.rs` L317–321) but **not applied** in member resistance checks; ACC-6.11 combinations emitted (`L968–976`) yet never govern (`governing(..., "uls")` L1050 only). |
| 6 | Report quality | **PARTIAL / FAIL** | `[id=…]` paths + resolve test pass (`L123–152`). ≥2 fail→pass remedies tested (`L70–113`). Most checks have distinct en/de (`flexure` L1063–1064, `shear` L1110–1111). **Fail:** punching first `explanation` block still copy-identical en/de (`L1199–1201`) before redundant override (`L1204–1207`); bridge stress checks use identical short strings (`L1625`, `L1641`). |
| 7 | Examples | **PASS** | `compliant_office_frame` → `complies()` (`L50–56`); `failing_under_reinforced` → ≥3 fails (`L58–67`); DSL assets decode. `liquid_retaining_fem_anchor` decodes but has no evaluate verdict test. |
| 7b | Inputs UX | **PASS** | `render_document_editor` + `en1992_field_meta` (`📥️inputs/🦀️.rs` L20). `field_meta_covers_every_editable_leaf_en_de` passes (`L172–201`); choices human en+de (`L190–194`). PrestressSpec meta aligned (`🏷️field-meta/🦀️.rs` L167–171). |
| 8 | Mutations & schema | **FAIL** | Top-level mutation GraphQL regenerated (`🧬️mutations/🔗️.graphql` L1–48). **28 per-mutation sub-facets still stubs:** `_placeholder: Boolean` in each `🧬️mutations/*/🧬️schema/🔗️.graphql` and `Record<string, unknown>` in matching `🟦️.ts` (count in `ignored-fields-audit.txt`). Snapshot GraphQL/ Rust aligned on PrestressSpec (`📸️snapshot/🔗️.graphql` L46). |
| 9 | Tests | **PASS (run) / FAIL (13:43)** | 83 executed, 0 skipped (`test-r2.txt`). Oracle ≥10 checks ±0.5 % (`L265–270`), jsonschema third-party (`L274–295`), remedy law (`L70–113`). **No scope-aware perturbation test** anywhere in family (`rg perturb` → 0 matches). |
| 10 | Stubs | **PARTIAL / FAIL** | No `todo!`/`unimplemented!` in evaluate. Stubs remain: 28 mutation sub-schema placeholders; dead `_ = (...)` reads pretending field use (`L1615`: `reinf.k`, `eps_uk`, `concrete.eps_cu2`, `n_parabola`); catalogue panel delegates to generic renderer (`📚️catalogue/🦀️.rs` L82–84) though markdown catalogue exists (`L48–73`). |

---

## Round-1 blocking re-check (14 items)

| # | R1 blocker | R2 | Evidence |
|---|------------|-----|----------|
| 1 | Characteristic actions + EN 1990 combinations | **FIXED** | `LoadCaseActions` uses `mK`/`vK`/line loads (`🦀️.rs` L257–269); `combine_member_actions` ULS 6.10a/b + SLS char/freq/qp (`🧬️schema/🦀️.rs` L859–967); governing combo in explanations (`L1063–1064`). |
| 2 | `designWorkingLifeYears` / `cementType` in cover | **FIXED** | `c_min_dur_adjusted_m(..., doc.design_working_life_years, &doc.cement_type, ...)` (`L1025–1026`); `structural_class_delta` (`L548–558`). |
| 3 | `c_min,b` check | **FIXED** | `c_min_b_m` + `c_nom_m` in cover check (`L1024–1026`, explanation L1032–1033). |
| 4 | Full materials + accidental γ | **PARTIAL / NOT FIXED** | C12–C100 + B500A/B catalogue (`📚️catalogue/🦀️.rs` L19–46); `AnnexParams::accidental` γ_c=1.3 (`🧬️schema/🦀️.rs` L317–321). **Not wired:** `for_situation` never used; flexure/shear always `for_choice(annex)` (`L1057`, L1098). |
| 5 | SLS §7.2 stress limits | **FIXED** | σ_s, σ_c char + 0.45 f_ck qp checks (`L1496–1558`). |
| 6 | Anchorage §8.4 / laps §8.7 | **FIXED** | f_bd, l_bd, l_0 with remedies (`L1561–1614`). |
| 7 | DE-NA deflection K·35 / K²·150/l | **FIXED** | `de_ld_caps` + min with Table 7.4N (`L1329–1341`). |
| 8 | λ_lim DE + second-order | **FIXED** | `lambda_lim_de(n)` (`L604–607`); `second_order_moment_nm` when exceeded (`L1364–1371`). |
| 9 | `useFem` / `udl` evaluated | **FIXED** | `characteristic_effects` reads both (`L837–857`); liquid example sets `use_fem: true`, `udl: 24000` (`📸️snapshot/🦀️.rs` L261–262). |
| 10 | PrestressSpec facet parity | **FIXED** | Rust `force/area/eccentricity/lossRatio` (`🦀️.rs` L234–238); GraphQL/proto/field-meta match (`📸️snapshot/🔗️.graphql` L46, `🏷️field-meta/🦀️.rs` L167–171). |
| 11 | Regenerate mutation/outline facets | **NOT FIXED** | Top-level hierarchical mutations OK; **28** per-mutation `🧬️schema/{🔗️.graphql,🟦️.ts}` still `_placeholder` / `Record<string, unknown>`. |
| 12 | Localized explanations (no copy x,x) | **PARTIAL / NOT FIXED** | Most checks distinct en/de. **Remaining identical blocks:** punching L1199–1201; bridge L1625, L1641. |
| 13 | Punching DE NA β, u₀/u₁, C_Rd,c | **FIXED** | `punching_beta`, `punching_perimeters`, DE C_Rd,c + u₀/d reduction (`L1171–1187`). |
| 14 | Fire Tables 5.2a–5.11 | **PARTIAL / NOT FIXED** | Tabulated data for 5.2a–5.11 implemented (`L651–710`). **`required_for` only maps subset:** column→5.2a, slab→one-way 5.8; 5.2b, 5.3, 5.9 two-way, 5.11 ribbed unused (`L713–725`). |

**R1 score:** 10 FIXED · 2 PARTIAL/NOT FIXED · 0 unchanged FAIL (structural actions fixed; facets/explanations/fire routing still open)

---

## CORRECTION 13:27 — explicit 12-cause audit

| # | Cause | Result | Evidence |
|---|-------|--------|----------|
| 1 | `NormFieldChoice` human en+de | **PASS** | `🏷️field-meta/🦀️.rs` L5–95; test L190–194 |
| 2 | Every editable leaf meta + en/de test | **PARTIAL** | Test passes via prefix fallback (`lookup_norm_field_meta` longest-prefix); concrete snapshot exposes `fCkCube`/`epsCu2`/… in GraphQL (`📸️snapshot/🔗️.graphql` L14–22) but no dedicated meta entries — only parent `concreteGrades` label |
| 3 | Structured editor, not JSON dump | **PASS** | `📥️inputs/🦀️.rs` L20 |
| 4 | `[id=…]` paths + resolve test | **PASS** | `🧪️tests/⚖️compliance/🦀️.rs` L123–152 |
| 5 | ≥2 distinct fail→pass remedy tests | **PASS** | `remedy_law_cover_as_and_stirrups_flip_to_pass` L70–113 |
| 6 | Example DSL decode + verdict asserts | **PASS** | Compliant/failing example tests L50–67; liquid example decode only |
| 7 | Python oracle ±0.5 % + jsonschema | **PASS** | L228–295; oracle requires ≥10 overlapping checks |
| 8 | All facets match Rust snapshot | **FAIL** | 28 per-mutation sub-schema placeholders (audit file) |
| 9 | No tautologies / hardcodes / ignored fields | **FAIL** | Dead `_ = (...)` at L1615; anchor `nEd`/`vEd` design scalars; accidental γ unused; no perturbation proof |
| 10 | No trivially-true tests | **PASS** | Numeric tolerances, fail counts, oracle parity |
| 11 | Semantic mutation verbs | **PASS** | `change-member-cover`, `insert-member`, etc. (`🧬️mutations/🦀️.rs` L69–98). Legacy names `change-action-m-ed` mutate `mK` (`⤴️change-action-m-ed/🔺️diff/🦀️.rs` L16) — misleading label only |
| 12 | Dynamic issue text localized | **PARTIAL** | Most checks distinct. Punching L1199–1201; bridge L1625/L1641 still copy-identical en/de |

---

## ADDENDUM 13:43 — structural actions & field-read law

| Requirement | Result | Evidence |
|-------------|--------|----------|
| Members + characteristic actions combined per EN 1990 (+ DE NA) in `evaluate()` | **PASS (members)** | `combine_member_actions` + `governing` (`L859–984`, L1050) |
| Hand-typed design effects as only action input | **FAIL (anchors)** | `Anchor { n_ed, v_ed }` edited directly (`🦀️.rs` L357–368); field-meta labels “Tension N_Ed” / “Shear V_Ed” (`🏷️field-meta/🦀️.rs` L208–209); checks use values without combination (`evaluate_anchor` L1693+) |
| FEM/external path not a design-effect backdoor | **PASS (members)** | External path stores **characteristic** `mK`/`vK`/`vKPunch`, combined before ULS (`L836–838`, L859+) |
| Scope-aware perturbation test (every applicable leaf) | **FAIL** | **No test** (`rg perturb` → 0). Required examples missing: **prestressed member** (all `prestress.*` / `prestressSteelId` N/A in default/failing); **pointForce** / **tK** never non-zero in committed examples |
| Static audit: editable leaves vs evaluate reads | **FAIL** | `🗑️generated/verify-en1992/ignored-fields-audit.txt` — fields with token presence but **no output effect:** `reinf.k`, `reinf.eps_uk`, `concrete.eps_cu2`, `n_parabola` (dead bind L1615); `prestressSteels[].fP01k` never read; `title` dead bind L1053; snapshot `fCkCube`/strain fields editable via schema but not consumed by checks |

---

## Implemented check catalogue (as shipped)

Per member (when ULS present): cover §4.4 (c_min,b + c_min,dur + life/cement); flexure §6.1; shear §6.2; torsion §6.3; punching §6.4/6.4.5; crack §7.3.4; min/max As §9.2.1; l/d §7.4.2 + DE caps; slenderness §5.8.3 + §5.8.8; prestress §5.10.2 (if `prestress` Some); fire §5.6 (tabulated subset); SLS §7.2 σ_s/σ_c/qp; anchorage §8.4; laps §8.7; bridge §7.2/§6.8.4; liquid §7.3. Per anchor: steel §7.2.1.4; cone §7.2.1.5; edge §7.2.2.5.

---

## Blocking fix list

1. **`🧬️schema/🧪️tests/⚖️compliance/🦀️.rs` (+ examples)** — Add scope-aware perturbation test: for each editable leaf, perturb in a committed example where it applies (beam B1, col C1, slab S1, liquid wall, **new prestressed member example**, anchor) and assert ≥1 check `utilization`/`status`/`computed` changes. Must cover `pointForce`, `tK`, `prestress.*`, `prestressSteelId`, catalogue strain fields, not only default office frame.

2. **`📸️snapshot/🦀️.rs`** — Add committed **prestressed beam/column example** (non-empty `prestress`, linked `prestressSteelId`) selectable alongside compliant/failing; use in perturbation test.

3. **`🧬️schema/🧬️mutations/*/🧬️schema/{🔗️.graphql,🟦️.ts,🛰️.proto}` (28 leaves)** — Regenerate per-mutation schema facets from Rust `Change*` structs; remove `_placeholder: Boolean` and `Record<string, unknown>`.

4. **`🧬️schema/🦀️.rs` (`evaluate_member` flexure/shear/…)** — When accidental load cases exist, evaluate governing **ACC-6.11** combination with `AnnexParams::for_situation(annex, "accidental")` (γ_c=1.3 DE) and report/pass-fail alongside ULS; prove with DE-vs-accidental test.

5. **`🦀️.rs` (`Anchor`) + `evaluate_anchor`** — Replace hand-typed `nEd`/`vEd` with characteristic anchor actions + EN 1990 combination (same pattern as members) **or** derive design values inside `evaluate()` from declared actions; remove design-effect-only anchor inputs.

6. **`🧬️schema/🦀️.rs` L1615 + material checks** — Remove dead `_ = (...)` binds; **read** `reinf.k`, `reinf.eps_uk`, `concrete.eps_cu2`, `concrete.n_parabola`, `prestressSteels[].fP01k`, and snapshot `fCkCube`/ε fields in the checks that depend on ductility, constitutive law, or prestress limits; prove via perturbation.

7. **`🧬️schema/part_1_2_fire` (`required_for`)** — Route member kind/support to correct table: column method B option, tension member 5.3, two-way slab 5.9, ribbed 5.11; add tests per rating.

8. **`🧬️schema/🦀️.rs` L1199–1201, L1625, L1641** — Replace identical en/de `explanation` strings with distinct German engineering text (remove redundant duplicate `punch.explanation` call at L1204–1207 after fix).

9. **`🧬️schema/🦀️.rs` L1053** — Either remove editable `title` from artifact state or feed it into report grouping/header so perturbation changes observable output; if kept as non-normative metadata, document exemption in spec — ADDENDUM currently forbids unread editable leaves.

10. **`📸️snapshot/🦀️.rs` examples** — Ensure at least one committed member uses non-zero `pointForce` and non-zero `tK` so scope-aware perturbation can reach torsion and point-load derivation paths (`characteristic_effects` L843–845, torsion check L1144+).

---

## Non-blocking observations

- Major R1 progress: characteristic actions, EN 1990 engine, c_min,b, SLS §7.2, anchorage/laps, DE l/d, λ_lim, punching DE NA, prestress facet parity, oracle expanded to ≥10 checks.
- `change-action-m-ed` mutation correctly writes `m_k` but verb label still says `m_ed` — rename for clarity.
- Liquid-retaining example decodes but lacks evaluate/compliance assertion test.
- Catalogue panel renders generic example picker; markdown Table 3.1 exists in code (`catalogue_markdown`) but is not surfaced in UI (`📚️catalogue/🦀️.rs` L82).
- Prefix-based field-meta fallback masks missing leaf labels for concrete constitutive parameters.

---

## Test log

Full output: `🗑️generated/verify-en1992/test-r2.txt`  
Hand numerics: `🗑️generated/verify-en1992/hand-numerics.txt`  
Ignored-fields audit: `🗑️generated/verify-en1992/ignored-fields-audit.txt`
