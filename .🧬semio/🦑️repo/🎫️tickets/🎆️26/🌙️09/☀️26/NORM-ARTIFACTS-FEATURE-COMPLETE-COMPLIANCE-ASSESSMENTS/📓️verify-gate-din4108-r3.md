# Verify Gate — DIN 4108 U′ Remedy Fix (Round 3)

**Verifier:** read-only re-audit after fixer claim (`📓️fix-din4108-u-prime-length-remedy.md`, `📓️impl-gate-din4108.md`)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/` (located via `os.walk`, basename `🧱️din4108`)  
**Prior verifier:** [Reverify DIN 4108 remedies](e17e80fb-86da-452b-b1f8-9ece569f2508) — FAIL (gate panic `Fail checks must carry at least one remedy: id=din4108-6.u-prime.wall-north`)  
**Fixer:** [Fix DIN 4108 bare U-prime](87ff83f2-d781-4d38-a2be-720bf87fd94e) — claimed ulp-nudged bounds always emitted + regression `zone_ht_flip_leaves_u_prime_and_targets_with_clearing_remedies`

---

## VERDICT: FAIL

**Summary line:** `1 test run: 0 passed, 1 failed, 0 skipped`

---

## Compliance gate (authoritative)

Command:

`NX_DAEMON=false bun nx run @semio-tech/norm-plugin:test --skip-nx-cache -- quick --test compliance_gate`

Log: `🗑️generated/compliance-gate-r3.txt`

### Fleet stderr (excerpt)

```
[DEBUG] compliance-gate fleet:
din4108: FAIL — din4108-6.u-prime.wall-north: no applicable remedy cleared the Fail (remedy[0] only lowered utilization 3.2819→1.0000)
…
en1994: FAIL — DE vs EN annex produced identical limit/computed values for all shared check ids
…
2/15 families failed compliance gate
```

### din4108 — panic resolved, remedy-clear still blocking

| R2 | R3 |
|----|-----|
| Panic: `Fail checks must carry at least one remedy: id=din4108-6.u-prime.wall-north` during remedy-flip | **No panic** — bare-Fail / zero-remedy path fixed |
| Gate aborts before fleet summary | Gate completes; din4108 reported as FAIL |

**Mechanism (R3):** On `failing_thin_insulation`, `din4108-6.u-prime.wall-north` is Fail with utilization ≈ 3.28. `assert_remedies_flip_fails` applies remedy[0] option-0 on the **initial** document; re-evaluate drops utilization to **1.0000** but status remains **Fail**. That violates the gate contract (remedy must leave status not Fail). Per audit rules: a remedy that leaves status Fail after option-0 apply is **blocking**, even if utilization moved.

Fixer ulp-nudge / guarantee blocks in `check_u_prime` (~1561–1812) prevent the R2 panic but **do not** produce a remedy[0] that clears wall-north on the gate's initial-doc flip path.

### en1994 (other family — not edited)

`en1994: FAIL — DE vs EN annex produced identical limit/computed values for all shared check ids`

Listed per instructions; out of scope for din4108 fix.

---

## Family nx test

`NX_DAEMON=false bun nx run @semio-tech/norm-din4108-rs:test --skip-nx-cache`  
→ **`Summary [ 0.641s] 96 tests run: 96 passed, 0 skipped`** (log `🗑️generated/din4108-family-r3.txt`)

Includes `zone_ht_flip_leaves_u_prime_and_targets_with_clearing_remedies` (post zone-ht option-0 + sequential applicables). Family green **does not** substitute for cross-family compliance gate.

---

## Five target checks — gate vs family

Gate `assert_remedies_flip_fails` probes each Fail on the **initial** example document (not cross-applied zone-ht first). Family regression applies zone-ht then asserts clearing on all five.

| Check | Gate (initial-doc flip) | Family (post zone-ht flip) |
|-------|-------------------------|----------------------------|
| `din4108-6.u-prime.wall-north` | **FAIL** — remedy[0] util 3.28→1.00, still Fail | Regression passes (different probe order / snapshot) |
| `din4108-6.u-prime.roof` | Not reported (gate did not fail this id) | Regression passes |
| `din4108-2.summer.zone-living` | Not reported | Regression passes |
| `din4108-2.zone-ht.zone-living` | Not reported | Regression passes |
| `din4108-10.app.wall-north.thin-eps` | Not reported | Regression passes |

**Contract spot-check:** remedy[0] on wall-north lowers displayed utilization but leaves Fail — consistent with a non-clearing or ulp-mismatched bound; **blocking** on the authoritative gate path.

---

## Fixer claim vs this audit

| Fixer claim | This audit |
|-------------|------------|
| ulp-nudged bounds always emitted; no bare Fail panic | **Confirmed** — no panic |
| Five gate Fails cleared end-to-end | **Not confirmed** — gate still fails wall-north on initial-doc flip |
| `zone_ht_flip_leaves_u_prime_and_targets_with_clearing_remedies` | **Confirmed** — family 96/96 |
| Compliance gate PASS | **FAIL** — Summary `0 passed, 1 failed` |

---

## Blocking list

- `din4108-6.u-prime.wall-north` — compliance gate: remedy[0] only lowered utilization 3.2819→1.0000; status still Fail after option-0 apply on `failing_thin_insulation`.
- `en1994` (fleet) — DE vs EN annex identical limit/computed values for all shared check ids (separate family; not din4108 scope).

---

## Non-blocking (din4108)

- R2 panic (`Fail checks must carry at least one remedy`) — **resolved**.
- Family suite 96/96 including zone-ht flip regression.
- `din4108-6.u-prime.roof`, `din4108-2.summer.zone-living`, `din4108-2.zone-ht.zone-living`, `din4108-10.app.wall-north.thin-eps` — not reported as gate failures on initial-doc flip; family post-flip regression green. Not gate-certified for fleet PASS because gate aborts on wall-north + en1994 before any PASS summary.

---

## Generated logs (this audit)

- `🗑️generated/compliance-gate-r3.txt`
- `🗑️generated/din4108-family-r3.txt`
