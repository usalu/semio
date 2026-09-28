# Verify Gate — EN 1997 Remedy Clearance (`🌍️en1997`)

**PASS**

**Auditor:** read-only subagent (remedy-clear gate)  
**Family:** `en1997` — located via `os.walk` under `✏️s/🔌️plugins/📕️norm/🗿️artifacts` → `🌍️en1997`  
**Fixer claim:** `📓️impl-gate-en1997.md` — `Summary [1.432s] 97 tests run: 97 passed, 0 skipped`  
**Gate rule:** each `Fail` must leave `Fail` after `apply_remedy_edit(..., option_index 0)` + `evaluate()` on at least one applicable remedy, or after sequential application of all applicables on one copy (`🚦️compliance-gate/🦀️.rs:116–228`).

**Runners (this audit):**
- Did **not** rerun the cross-family compliance gate (per instructions).
- Targeted family tests: `remedy_law_footing_width_clears_bearing_or_improves`, `noncompliant_demo_has_failures_with_remedies`, `settlement_remedy_targets_governing_layer_id_path` — all passed via `bun nx run @semio-tech/norm-en1997-rs:test --skip-nx-cache`.
- Ephemeral probe harness on `noncompliant_demo()` for the four gate ids — log: `🗑️generated/verify-en1997-gate/probe-output.txt`.

---

## Per-check audit (noncompliant demo)

| Check id | Before | Remedy[0] (solo) | Sequential (if needed) | Field read by evaluate | en/de distinct | Notes |
|----------|--------|------------------|------------------------|------------------------|----------------|-------|
| `en1997.6.5.bearing.footing-F1.lc-bsP` | Fail u=7.18 | **Pass** u=0.925 — `footings[id=footing-F1].width` → 5.283 m | n/a | `footing.width` in `design_bearing_resistance_with_params` / `find_required_width` | yes | `find_required_width` holds `length` fixed (`🦀️.rs:1083`), uses characteristic `phi_use` (`:1532`), 2% margin (`:1096`). No L∝B growth. |
| `en1997.6.6.settlement.footing-F1` | Fail u=5.17 | **Pass** u=0.982 — `layers[id=layer-sand].oedometricModulus` scaled to governing contribution (`🦀️.rs:1752–1759`) | n/a (solo[0] clears) | `layer.oedometric_modulus` in `settlement_oedometric_with_gwl` | yes | No width remedy on settlement. Second remedy raises `settlementLimit` to computed `s` (`:1760–1765`); solo[1] → u=1.000 (**Pass** at equality, `CheckBuilder::utilization` `:302`). |
| `en1997.7.6.3.tension.pile-P1` | Fail u=1.33 | **Pass** u=0.664 — `piles[id=pile-P1].count` → 2 | count alone sufficient | `pile.count` in `r_t_d` (`:1914`) | yes | Test-shaft governs (`test_profiles`); **no length remedy**. `tensionVariable` at_most solo still **Fail** u=1.04 (not the only applicable). `tensionPermanent` not emitted when count increase suffices (`n_req > pile.count`, `:1957`). |
| `en1997.9.sliding.wall-W1` | Fail u=2.16 | solo[0] width still Fail u=2.10; solo[1] embed u=1.39; solo[2] height u=1.24 | **Pass** u=0.784 — width + embed + height in order | `baseWidth`, `embedment`, `height` in sliding resistance / active pressure (`:2048–2097`) | yes | Width from `(H−½Ep)/R_base` margin (`:2067–2071`); embed `√(Ep,need/Ep)` (`:2073–2077`); height `√(R/H)` margin (`:2080–2081`). Sequential set required; no single remedy raises u as the only applicable path. |

---

## Compliance criteria

| # | Criterion | Result | Evidence |
|---|-----------|--------|----------|
| 1 | Remedy changes a field the check reads by enough that status ≠ Fail | **PASS** | Probe harness above; `remedy_law_footing_width_clears_bearing_or_improves` asserts `CheckStatus::Pass` after width apply. |
| 2 | No remedy that raises utilization is the **only** applicable one | **PASS** | `tensionVariable` solo worsens (u 1.33→1.04) but `count` solo clears. Wall width solo lowers u but fails; embed/height also applicable; sequential clears. |
| 3 | en and de copy not identical | **PASS** | All four checks: `en==de: false` in probe; distinct EN/DE action strings in `🦀️.rs` remedy constructors. |
| 4 | No fingerprint / epsilon fold / id_score / tag_fp / dummy 0/1 utilization on these checks | **PASS** | `rg` over evaluate schema: no `fingerprint`, `tag_fp`, `id_score`, `field_fingerprint`. Four audited checks use real force/length utilizations. Incidental `1.0/1.0` only on GK pass branch (`:1345`); `0.0/1.0` only on missing `governingLayerId` Bishop fail with explicit `.status(Fail)` (`:2261–2262`) — not on the four gate ids. |

---

## Source anchors (fixer claims vs code)

| Fixer claim | Verified |
|-------------|----------|
| Bearing: fixed L + characteristic φ′, 2% margin | `find_required_width` `:1078–1103`; remedy at `:1578–1583` |
| Settlement: E scaled to governing layer; limit raised to `s`; no width remedy | `:1751–1765` |
| Tension: count / tensionVariable / tensionPermanent; length inert when test shaft governs | `:1930–1966` (no length remedy in tension fail branch) |
| Sliding wall: width / embed / height formulas | `:2064–2097` |

---

## Blocking fix list

None

---

## Non-blocking observations

- `tensionVariable` at_most remains applicable but does not clear alone; gate passes because `count` (remedy[0]) clears individually — acceptable per gate logic.
- `en1997.9.sliding.wall-W1` requires sequential applicables; solo remedies lower u but remain Fail until all three apply — matches gate sequential path (`compliance-gate` `:193–220`).
- Implementer 97/97 claim not independently re-run in full (`--no-fail-fast`); targeted tests + probe suffice for this gate audit.

---

*Verifier: read-only subagent · ticket `26/09/26/NORM-ARTIFACTS-FEATURE-COMPLETE-COMPLIANCE-ASSESSMENTS` · probe `🗑️generated/verify-en1997-gate/`*
