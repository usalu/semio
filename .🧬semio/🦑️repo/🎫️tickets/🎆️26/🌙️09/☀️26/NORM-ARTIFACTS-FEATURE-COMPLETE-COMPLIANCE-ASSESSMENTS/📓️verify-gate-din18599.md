# Verify — DIN 18599 heating-demand remedy fix

**VERDICT: PASS**

**Blocking list:** None

**Reviewer:** read-only verifier (heating-demand gate fix, fixer `09bbac83`)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/`  
**Impl claim:** `📓️impl-gate-din18599.md` — stack `deltaUWbWM2k` + window `gValue`/`fc` with per-element U remedies; family `113 passed, 0 skipped`  
**Test rerun:** not executed (active `cargo`/`rustc` build in workspace; judged from source + independent Python oracle aligned to `🔮️oracles/⚖️compliance/🐍️.py`)  
**Cross-family compliance gate:** not rerun (per audit scope)

---

## Gate criterion

A `Fail` clears only when `apply_remedy_edit` (option index 0) or sequential application of every applicable remedy on one copy leaves `status != Fail`. Lower utilization that stays `Fail` does not count.

---

## Check table

| # | Check | Result | Evidence |
|---|--------|--------|----------|
| 1 | Sequential applicables change U, `deltaUWbWM2k`, and (when offered) `gValue`/`fc` enough that `din18599.2.heating-demand` is not `Fail` | **PASS** | Remedy block (`🧬️schema/🦀️.rs:1220–1269`) emits independent remedies: per-element `AtMost` U → `reference_u(kind)`; for windows, `AtLeast` `gValue` → `GEG_ANLAGE1_WINDOW_G_REF` when below ref, `AtLeast` `fc` → `1.0` when below ref; `AtMost` `deltaUWbWM2k` → `GEG_ANLAGE1_DELTA_U_WB_REF` (0.05) when above ref — same stacking pattern as `din18599.geg.ht-prime` (`1137–1146`). No `remedied` / exclusive fallback remains in family sources. Independent oracle on `noncompliant-detached` geometry/climate: baseline `Q_H,nd/Q_H,nd,Ref = 2.4544`; U-only remedies → `1.1936` (still Fail); U + `deltaUWbWM2k→0.05` → `0.984` (Pass, `Q_H,nd ≤ Q_H,nd,Ref`). |
| 2 | Remedy bounds are fields the heating-demand check reads, not explanation-only | **PASS** | Check uses `.utilization(energy(q_h), energy(q_h_ref))` with `q_h = derived.q_h_nd_kwh` and `q_h_ref` from `derive_balance` reference run (`1205–1214`, `817–820`). Reference run sets window `g_value`/`fc` and `delta_u_wb_w_m2k` (`808–817`). Remedy targets: `elements[id=…].uValueWM2k`, `.gValue`, `.fc`, `deltaUWbWM2k` with `required` SI values (`1225–1267`). Solar gains in zone balance use `g_value` and `fc` (`454–455`). |
| 3 | EN and DE copy are not identical prose | **PASS** | Heating-demand explanation: “vs reference building” / “gegenüber Referenzgebäude” (`1216–1218`). Remedy actions differ per language (e.g. U: “Improve envelope…” / “Hülle verbessern…”; g: “Raise g-value…” / “g-Wert … anheben”; fc: “Raise shading factor F_c…” / “Abminderungsfaktor F_c …”; ΔU_WB: “Lower ΔU_WB to 0.05…” / “ΔU_WB auf 0.05 … senken”). Family test `no_identical_en_de_explanations_in_committed_examples` guards committed examples (`⚖️compliance/🦀️.rs:688–716`). |
| 4 | No fingerprint, epsilon fold, `id_score`, `tag_fp`, or dummy utilization | **PASS** | Grep over `🧬️schema/` evaluate path: no `field_fingerprint`, `id_score`, `tag_fp`, or `1e-9 *` field folds. Utilization is `computed.value / limit.value` from real `Q_H,nd` and `Q_H,nd,Ref` (`⚖️compliance/🦀️.rs:294–303`). No hardcoded pass utilization on this check. |

---

## Notes

- On `noncompliant-detached`, window `gValue=0.70` (above GEG ref 0.60) and `fc=1.0`, so g/fc remedies are correctly **not** offered; the gate clear is delivered by nine U remedies plus `deltaUWbWM2k`, matching the prior failure mode (`u=1.1936` with U-only).
- g/fc remedy axes are wired for cases below the reference building (e.g. compliant subject south window `g=0.55`, `fc=0.7`); oracle confirms raising them with reference U and ΔU_WB reaches `u=1.0`.
- Family lacks a dedicated `remedy_law_*heating-demand*` unit test (unlike `ht-prime` / `dhw`); fix is otherwise consistent with shared compliance-gate `assert_remedies_flip_fails` logic. Fixer-reported `113/113` not independently rerun here.
