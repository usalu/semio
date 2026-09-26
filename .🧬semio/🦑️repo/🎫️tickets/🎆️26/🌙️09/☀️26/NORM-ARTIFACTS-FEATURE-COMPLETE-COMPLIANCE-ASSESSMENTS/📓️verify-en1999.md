# Verify — EN 1999 (Wave D, Round 6)

**VERDICT: PASS (0 blocking)**

**Verifier:** Wave D Round 6 (read-only; tests run)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** fixer `d469004d` — hierarchical diff/mutation facets regenerated, inference guard typed, three referential-integrity tests, 77/77 + 51/51  
**Test runs (verifier):**

| Command | Result | Log |
|---------|--------|-----|
| `bun nx run @semio-tech/norm-en1999-rs:test --skip-nx-cache -- --no-fail-fast` | **Summary [0.633s] 77 passed, 0 skipped** | `🗑️generated/verify-en1999/test-r6.txt` |
| `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` | **Summary [0.273s] 51 passed, 0 skipped** | `🗑️generated/verify-en1999/contract-test-r6.txt` |

## Round 6 — R5 blocker re-check

| # | R5 blocker | R6 | Evidence |
|---|------------|-----|----------|
| 1 | Diff facets match hierarchical `En1999Diff`; no scalar `nEdKn`/`chi` | **PASS** | `🔺️diff/🟦️.ts:16–37` (`artifact`…`shells` only); `🔺️diff/🔗️.graphql:2–13`; `🔺️diff/🛰️.proto:4–25`; `facet_diff_matches_rust` (`🧪️tests/⚖️compliance/🦀️.rs:456–472`) asserts fields + absence of stale scalars |
| 2 | Mutation facets match 18 semantic `KINDS`; scalar union gone | **PASS** | `🧬️mutations/🦀️.rs:52–71` (`KINDS`); `🧬️mutations/🟦️.ts:101–143` (`EN1999_MUTATION_KINDS` + discriminated union); `🧬️mutations/🔗️.graphql:3–23`; `facet_mutations_match_kinds` (`:475–491`) |
| 3 | `normEn1999InferenceGuardObject` → `Readonly<En1999Inference>` | **PASS** | `💡️inferences/🟦️.ts:30–31`; `rg Record<string, unknown>` over family → **0** hits |
| 4 | Referential-integrity compliance tests (duplicate + dangling) | **PASS** | `duplicate_material_id_fails_with_oneof_remedy` (`:783–801`); `dangling_connection_material_id_fails_with_oneof_existing_materials` (`:804–825`); `dangling_connection_member_id_fails_with_oneof_existing_members` (`:828–849`) — each asserts `CheckStatus::Fail`, en≠de explanations, `RemedyBound::OneOf` with existing ids (not `exactly`/dummy utilization) |

**New tests since R5 (72→77):** `facet_diff_matches_rust`, `facet_mutations_match_kinds`, `duplicate_material_id_fails_with_oneof_remedy`, `dangling_connection_material_id_fails_with_oneof_existing_materials`, `dangling_connection_member_id_fails_with_oneof_existing_members`.

### Evaluate-path spot-check (no regression)

| Item | R6 | Evidence |
|------|-----|----------|
| No fingerprint / `en1999.en1990.psi` / `id_score` / `tag_fp` / `1e-9 *` gaming | **PASS** | `rg fingerprint\|en1990\.psi\|id_score\|tag_fp\|1e-9 \*` over family `*.rs` evaluate path → **0** gaming hits; `.max(1e-9)` divide guards only (`🧬️schema/🦀️.rs:961,1284,…`) |
| Perturbation signature `(id,status,computed,limit,utilization)` | **PASS** | `every_editable_leaf_influences_a_check` (`:665–748`) — `to_bits()` on computed/limit/utilization; explanation excluded |
| Catalogue shares evaluate consts | **PASS** | `CATALOGUE_ALLOY_ROWS` (`🧬️schema/🦀️.rs:616–641`); catalogue panel imports same (`✏️editor/📌️panels/📚️catalogue/🦀️.rs:25–47`); `alloy_table_3_2_6082_t6` (`:17–34`) |
| SLS frequent + shell `tau_rcr` | **PASS** | `en1999.7.2.sls-freq.*` (`🧬️schema/🦀️.rs:1652`); `en1999.8.sls-freq.*` (`:1902`); shell shear `tau_rcr` (`:2378–2480`) |

## Round 6 — Brief checks (1–10 + 7b)

| Check | Result | Evidence |
|-------|--------|----------|
| 1 Subject completeness | **PASS** | Hierarchical SI snapshot; characteristic `MemberAction`; EN 1990 ULS+SLS governing |
| 2 Clause coverage | **PASS** | Parts 1-1…1-5 + §7.2 SLS + §8 connections; shell τ; bi-linear fatigue |
| 3 Numerics | **PASS** | Hand/oracle tests green; catalogue cell = evaluated `N_Rd` limit |
| 4 Applicability | **PASS** | Empty-list N/A gates |
| 5 National annex | **PASS** | γ_Mf DE vs EN on fatigue |
| 6 Report quality | **PASS** | Governing combo + action_id on ULS/SLS; `[id=…]` remedies resolve |
| 7 Examples | **PASS** | Compliant + multi-fail decode/evaluate |
| 7b Inputs UX | **PASS** | `field_meta_covers_every_editable_leaf_en_de` |
| 8 Mutations & schema | **PASS** | Snapshot + diff + mutations + proto + inference guards aligned |
| 9 Tests | **PASS** | 77/77 + 51/51; facet parity + referential integrity + perturbation + catalogue cell |
| 10 Stubs | **PASS** | No `todo!`/`#[ignore]` in evaluate path |

## Round 6 — Blocking fix list

None

## Round 6 — Non-blocking observations

- R5 doc cited “19 semantic `KINDS`”; Rust/TS/GQL/proto and `facet_mutations_match_kinds` correctly assert **18** (`🧬️mutations/🦀️.rs:52–71`, `:480`).
- Referential-integrity tests assert `status` + remedy law but not `computed`/`limit`/`utilization` — acceptable for pure reference Fails (no normative quantity emitted).
- `actions[].id` duplicates within one owner still not scanned by `push_duplicate_id_fails` (only top-level entity lists).

---

# Verify — EN 1999 (Wave D, Round 5)

**VERDICT: FAIL (4 blocking)**

**Verifier:** Wave D Round 5 (read-only; tests run)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** `📓️impl-en1999.md` — R4 rework (fingerprint removal + normative connection `sls-freq`), 72/72  
**Test runs (verifier):**

| Command | Result | Log |
|---------|--------|-----|
| `bun nx run @semio-tech/norm-en1999-rs:test --skip-nx-cache -- --no-fail-fast` | **Summary [0.801s] 72 passed, 0 skipped** | `🗑️generated/verify-en1999/test-r5.txt` |
| `bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache` | **Summary [0.277s] 51 passed, 0 skipped** | `🗑️generated/verify-en1999/contract-test-r5.txt` |

## Round history

| Round | Verdict | Notes |
|-------|---------|-------|
| R1 | FAIL (8 blocking) | Remedy paths, silent alloy, fire k_θ, missing 1-4/1-5, stale DSL, oracle/jsonschema, weak remedy-law, empty field-meta |
| R2 | FAIL (7 blocking) | EN 1990 absent, ρ_u unused, cold-formed discard, shell χ=0.70, single-slope fatigue, remedy-law, ignored leaves |
| R3 | FAIL (8 blocking) | Facets pre-refactor; perturbation gate weak; governing combo not reported; identical en/de; no SLS |
| R4 | FAIL (9 blocking) | Snapshot facets fixed; signature gaming + dummy binds + partial gov combo + hand-typed cold/shell + catalogue dup + dangling materialId + no duplicate scan + stale diff/mutation guards |
| R4 rework | Coordinator rejected `en1999.en1990.psi.*` fingerprints (CORRECTION 14:37); normative `en1999.8.sls-freq.*` wired instead (`📓️fix-en1999-r4-no-fingerprint-gaming.md`) |
| **R5** | **FAIL (4 blocking)** | **All nine R4 evaluate-path blockers closed; diff/mutation/proto facets still pre-refactor; inference guard loose; referential Fails untested** |

---

## Round 5 — R4 blocker re-check (+ fingerprint rejection)

| # | R4 blocker | R5 | Evidence |
|---|------------|-----|----------|
| 1 | Perturbation signature `(id,status,computed,limit,utilization)`; no explanation | **PASS** | `every_editable_leaf_influences_a_check` (`🧪️tests/⚖️compliance/🦀️.rs:627–709`) compares `to_bits()` on computed/limit/utilization; `unchanged.is_empty()` per leaf; only `.id` exempt |
| 2 | `effective_wel_y` uses ρ_u,haz; no `let _ = rho_u_haz` | **PASS** | `rho_o_haz.min(rho_u_haz)` in `effective_wel_y` (`🧬️schema/🦀️.rs:756–764`); `rg let _ = (rho_u_haz|rho_o_haz)` → **0** hits in family evaluate path |
| 3 | Shell τ check uses `tau_rcr` | **PASS** | `en1999.1-5.shear.{id}` (`🧬️schema/🦀️.rs:2470–2480`); `tau_rcr` in explanation |
| 4 | Every member ULS explanation names governing combo + action_id | **PASS** | All `6.2.*`/`6.3.*` checks use `(ULS {combo}, lead {action_id})` / `(GZT {combo}, führend {action_id})` (e.g. `:1247–1248`, `:1313–1314`, `:1382+`); test `governing_uls_combination_named_in_member_explanations` (`:456–475`) filters all `en1999.6.2.*`/`6.3.*` |
| 5 | SLS frequent + cold/shell characteristic `actions[]` | **PASS** | Member `en1999.7.2.sls-freq.*` (`:1651–1664`); connection `en1999.8.sls-freq.*` (`:1866–1907`); `ColdFormedSheet`/`AluminiumShell` carry `actions: Vec<MemberAction>` (`📸️snapshot/🦀️.rs:218,236`); no `mEd`/`nEd`/`sigma*Ed` in snapshot JSON leaves |
| 6 | `CATALOGUE_ALLOY_ROWS` shared; evaluated limit = catalogue cell | **PASS** | `part_1_1::CATALOGUE_ALLOY_ROWS` (`🧬️schema/🦀️.rs:616–641`); catalogue panel imports same const (`✏️editor/📌️panels/📚️catalogue/🦀️.rs:25–47`); test `alloy_table_3_2_6082_t6` asserts `N_Rd` limit vs `A_eff·f_o/γ_M1` (`🧪️tests/⚖️compliance/🦀️.rs:17–34`) |
| 7 | Dangling `connections[].materialId` → Fail + one_of | **PASS** | `evaluate_structure` (`🧬️schema/🦀️.rs:2700–2720`) |
| 8 | Duplicate entity ids → Fail | **PASS (code)** | `push_duplicate_id_fails` on all eight lists (`:2543–2550`) — **no compliance test** (see blocking #4) |
| 9 | Typed text guards (inference/diff/mutations) | **PARTIAL** | `📝️text/🟦️.ts` for snapshot/diff/mutations/inferences return `Readonly<En1999*>`; **`💡️inferences/🟦️.ts:30–31` still `Record<string, unknown>`** |

### Fingerprint gaming (CORRECTION 14:37)

| Item | R5 | Evidence |
|------|-----|----------|
| No `en1999.en1990.psi.*` / `fingerprint` / `1e-9 *` field folds in evaluate | **PASS** | `rg fingerprint|en1990\.psi|1e-9 \*` over `🧬️schema/🦀️.rs` evaluate path → **0** gaming hits; `1e-9` only in `.max(1e-9)` denominators |
| Connection `actions[].category` moves normative SLS without fingerprints | **PASS** | `sls_connection_effects` + `en1999.8.sls-freq.{id}` (`🧬️schema/🦀️.rs:1866–1907`) |

---

## Round 5 — ADDENDA 14:54 / 14:42 / CORRECTION 13:27

| Item | R5 | Evidence |
|------|-----|----------|
| `reference_tables()` populated from evaluate consts | **PASS** | `reference_tables()` → `alloy_table_3_2()` from `CATALOGUE_ALLOY_ROWS` (`✏️editor/📌️panels/📚️catalogue/🦀️.rs:20–48`) |
| Dangling refs Fail (en+de, remedy `one_of`) | **PASS (code)** / **FAIL (tests)** | Members (`:2657–2676`), connections materialId (`:2700–2720`), memberId (`check_connection` `:1712–1729`), fire/fatigue/cold/shell material refs — **no `*dangling*` / `*duplicate*` compliance tests** |
| Duplicate entity ids Fail | **PASS (code)** / **FAIL (tests)** | `push_duplicate_id_fails` — not exercised by tests |
| Facets regenerated (snapshot + diff + mutations + proto) | **FAIL** | Snapshot TS/GQL/proto current (`facet_field_names_match_snapshot_json_schema` `:436–452`). **Diff + mutations facets still flat scalar pre-refactor** (below) |
| No `Record<string, unknown>` on subject/inference guards | **FAIL** | `normEn1999InferenceGuardObject` → `Record<string, unknown>` (`💡️inferences/🟦️.ts:30–31`) |
| Perturbation: full nested walk, signature without explanation | **PASS** | `walk_leaves` + per-leaf assert (`:627–709`); two fixtures (compliant + noncompliant) |

---

## Round 5 — Facet drift (blocking)

Rust source of truth:

- `En1999Diff` (`🔺️diff/🦀️.rs:11–32`): `artifact`, `annex`, `materials`, `sections`, `members`, `connections`, `fireScenarios`, `fatigueDetails`, `coldFormed`, `shells` only.
- `En1999Mutation` (`🧬️mutations/🦀️.rs:31–71`): 19 semantic kinds (`change-materials`, `change-members`, `add-member`, …).

Stale generated facets still expose the **old 26-scalar** Wave-A demo:

| Facet | Stale fields (examples) | Rust expects |
|-------|-------------------------|--------------|
| `🔺️diff/🟦️.ts` | `nEdKn`, `mEdKnm`, `chi`, `sheetMEdKnm`, `sigmaEdShellMpa`, … | entity list optionals only |
| `🔺️diff/🔗️.graphql` | same scalar set | hierarchical diff |
| `🔺️diff/🛰️.proto` | `n_ed_kn` … `sigma_ed_shell_mpa` (fields 2–27) | `materials`/`members`/… messages |
| `🧬️mutations/🟦️.ts` | `changeNEdKn`, `changeChi`, `changeSheetMEdKnm`, … (26 variants) | `changeMembers`, `changeConnections`, … |
| `🧬️mutations/🔗️.graphql` | flat `nEdKn`…`annex` on `En1999Mutation` | discriminated semantic mutations |
| `🧬️mutations/🛰️.proto` | scalar `n_ed_kn`… | semantic mutation union |

`facet_field_names_match_snapshot_json_schema` reads `mut_ts` but **never asserts** on diff/mutation facet parity — drift survives 72/72 green.

---

## Round 5 — Brief checks (1–10 + 7b)

| Check | Result | Evidence |
|-------|--------|----------|
| 1 Subject completeness | **PASS** | Hierarchical SI snapshot; characteristic `MemberAction` on members/connections/cold/shell; EN 1990 ULS+SLS governing |
| 2 Clause coverage | **PASS** | Parts 1-1…1-5 + §7.2 SLS + §8 connections; shell τ; bi-linear fatigue |
| 3 Numerics | **PASS** | Hand/oracle tests green; `alloy_table_3_2_6082_t6` ties catalogue to `N_Rd` |
| 4 Applicability | **PASS** | Empty-list N/A gates for members/connections/fire/fatigue/cold/shell |
| 5 National annex | **PASS** | γ_Mf DE vs EN on fatigue; documented identical γ_M1/γ_M2 |
| 6 Report quality | **PASS** | Governing combo + action_id on ULS/SLS checks; `[id=…]` remedies resolve |
| 7 Examples | **PASS** | Compliant + multi-fail decode/evaluate; DSL/pack drift test |
| 7b Inputs UX | **PASS** | `field_meta_covers_every_editable_leaf_en_de`; structured editor test |
| 8 Mutations & schema | **FAIL** | Snapshot + artifact JSON aligned; **diff/mutation TS/GQL/proto stale**; inference guard loose |
| 9 Tests | **PARTIAL** | 72/72 pass; perturbation + catalogue cell test strong; **missing referential-integrity + facet parity beyond snapshot** |
| 10 Stubs | **PASS** | No `todo!`/`#[ignore]` in evaluate path |

## Round 5 — CORRECTION 13:27 (twelve causes)

| # | Cause | R5 |
|---|-------|-----|
| 1 | Choice labels en+de | **PASS** |
| 2 | Every editable leaf meta | **PASS** |
| 3 | Structured editor | **PASS** |
| 4 | `[id=…]` paths resolve | **PASS** |
| 5 | ≥2 fail→pass remedy apply | **PASS** |
| 6 | Example decode + verdicts | **PASS** |
| 7 | Oracle + jsonschema | **PASS** |
| 8 | Facets regenerated | **FAIL** — diff + mutations + proto; inference `Record` guard |
| 9 | No tautologies / ignored inputs | **PASS** — gaming audit clean post fingerprint removal |
| 10 | No trivially-true tests | **PASS** |
| 11 | Semantic mutation names (Rust) | **PASS** — TS/GQL facets still scalar names |
| 12 | Localized dynamic text | **PASS** — `explanations_en_de_not_identical_except_numbers` |

---

## Round 5 — Blocking fix list

1. **`🔺️diff/🟦️.ts`, `🔺️diff/🔗️.graphql`, `🔺️diff/🛰️.proto`** — Regenerate from `🔺️diff/🦀️.rs` (`artifact`, `annex`, `materials`…`shells`); remove all scalar `nEdKn`/`chi`/… fields. Extend `facet_field_names_match_snapshot_json_schema` (or add `facet_diff_matches_rust`) to fail on stale diff field names.

2. **`🧬️mutations/🟦️.ts`, `🧬️mutations/🔗️.graphql`, `🧬️mutations/🛰️.proto`** — Regenerate from `🧬️mutations/🦀️.rs` `KINDS` (19 semantic mutations); delete scalar `changeNEdKn`/`changeChi`/… union. Assert TS/GQL/proto list matches `KINDS`.

3. **`💡️inferences/🟦️.ts:30–31`** — Change `normEn1999InferenceGuardObject` to return `Readonly<En1999Inference>` (mirror `💡️inferences/📝️text/🟦️.ts:21–22`); remove `Record<string, unknown>`.

4. **`🧪️tests/⚖️compliance/🦀️.rs`** — Add tests: (a) duplicate `materials[].id` (or `members[].id`) → `en1999.ref.duplicate.*` Fail with `one_of` remedy; (b) dangling `connections[].materialId` → `en1999.ref.conn.material.*` Fail; (c) dangling `connections[].memberId` → `en1999.8.ref.*` Fail. Mirror en1990/en1997 fleet pattern (ADDENDUM 14:42).

---

## Round 5 — Non-blocking observations

- R5 closes **all nine R4 evaluate-path blockers** and passes the post-rejection gaming audit; the family is materially feature-complete on the Rust evaluation surface.
- `push_action_field_fails` adds valuable EN 1990 referential validation on `kind`/`category`/`source` without fingerprints.
- Oracle overlap threshold remains `compared >= 2` per example (`:343`) — consider raising once facet drift is fixed.
- `actions[].id` duplicates within one owner are not scanned by `push_duplicate_id_fails` (only top-level entity lists).
- Connection lever arm `0.50` m for moment→force conversion is hardcoded (`check_connection` `:1706–1708`) — document or expose if subject-specific geometry is required later.
