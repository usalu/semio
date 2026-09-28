# Verify Gate — DIN 4108 U′ Remedy Fix (Round 2)

**Verifier:** read-only re-audit after fixer claim (`📓️fix-din4108-u-prime-length-remedy.md`, `📓️impl-gate-din4108.md`)  
**Family root:** `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/` (located via `os.walk`, basename `🧱️din4108`)  
**Prior verifier:** [Verify DIN 4108 remedies](ba531f88-4625-4c35-b9f2-fbc1ceab1a0c) — FAIL (same panic)  
**Fixer:** [Fix DIN 4108 U-prime remedy](cfa5b7f0-d439-4fca-94eb-aa41aaa9b2aa) — claimed gate unblocked via clearing `lengthM` / area / forced-insulation fallbacks in `check_u_prime`

---

## VERDICT: FAIL

---

## Compliance gate (authoritative)

Command (reproduced twice):

`NX_DAEMON=false bun nx run @semio-tech/norm-plugin:test --skip-nx-cache -- quick --test compliance_gate`

Result: **`1 test run: 0 passed, 1 failed`** — `compliance_gate_all_families` **panics** on din4108 before any fleet PASS line:

```
Fail checks must carry at least one remedy: id=din4108-6.u-prime.wall-north
```

Backtrace (`RUST_BACKTRACE=1`, log `🗑️generated/compliance-gate-r2-backtrace.txt`):

`assert_remedies_flip_fails` → `Din4108Family::evaluate` → `check_u_prime` → `CheckBuilder::build` (`⚖️compliance/🦀️.rs:361`).

**Mechanism (unchanged from round 1):** Initial `failing_thin_insulation` DSL evaluate emits remedies on every Fail (`failing_example_has_failures_and_remedies` passes). During gate **remedy-flip** re-evaluation (triggered while clearing another Fail — notably `din4108-2.zone-ht.zone-living` thickness on `thin-eps`), `check_u_prime` for `wall-north` is still **Fail** while **all** branches in the new fallback chain can skip together:

- insulation: `d_req ≤ current thickness` after zone-ht thickening
- ψ: `psi_req + 1e-12 ≥ bridge.psi` for `tb-bad` (ψ = 0.4)
- `lengthM`: guard/emit pair at `🦀️.rs` ~1628–1635 can still refuse when `length_req` is driven to 0 but `u + Σ(ψ·l)/A_opaque` remains above `u_max` under the loop’s denominator (post-flip `u ≈ u_max` edge)
- area / forced-insulation last resorts: also skip when `u ≥ u_max − ε` and `d_req ≤ current`

That violates the contract (`debug_assert` in `CheckBuilder::build`) and aborts the gate **before** din4108 can PASS and before any of the five target checks are gate-certified for remedy-clear.

The fixer’s “always emit clearing `lengthM`” block (~1612–1657) is present in source but **does not** prevent this panic at runtime.

---

## Family nx test

`NX_DAEMON=false bun nx run @semio-tech/norm-din4108-rs:test --skip-nx-cache`  
→ `Summary [ 1.089s] 95 tests run: 95 passed, 0 skipped` (log `🗑️generated/din4108-nx-test-r2.txt`).

Family tests **do not** exercise cross-check remedy-flip re-evaluation; they do **not** contradict the gate failure.

---

## Test count (96 → 95)

| Source | Count |
|--------|------:|
| Round 4 verify note (`📓️verify-din4108.md`) | 96 |
| This audit `cargo test -p semio-s-artifact-norm-din4108 -- --list` | 95 |
| `HEAD` same command | 95 |

**Deleted or ignored test?** **No named test removed.** Git tree and fixer diff vs `HEAD` show no deleted `async fn`; compliance suite still has **27** `async fn` tests. The single missing slot vs Round 4 log was **not** identified by name — treat as unexplained drift, **not** as confirmed test deletion.

---

## Round gate blockers — re-check (five target checks)

| Check | Gate remedy-clear? | Evidence |
|-------|-------------------|----------|
| `din4108-6.u-prime.wall-north` | **FAIL** | Gate panic above — **Fail with zero remedies** on remedy-flip re-evaluate. `lengthM` is a real field (`bridge.length_m` in `psi_l_sum` / `u_with_bridges`); not a dummy utilization. |
| `din4108-6.u-prime.roof` | **Not certified** (gate aborts) | Same `check_u_prime` logic (~1554–1741); same cross-remedy risk; gate never finishes din4108. |
| `din4108-2.summer.zone-living` | **Not certified** (gate aborts) | Source: proportional `scale`, floor `at_least`, window `shadingFc` / `areaM2` (~1003–1073). Family `remedy_law_shading_fc_fixes_summer` passes (applies all applicables once). **Not** gate-proven end-to-end because gate aborts on U′ panic first. |
| `din4108-2.zone-ht.zone-living` | **Not certified** (gate aborts) | Source: per-opaque thickness + area-scale (~691–738). **Likely trigger** for zone-ht thickness flip that re-evaluates bare U′ Fail. |
| `din4108-10.app.wall-north.thin-eps` | **Not certified** (gate aborts) | Source: `one_of` application/class options from required ranks (~2157+). Option 0 should be suitable `WAP` + class ladder when sequential applicables run; **not** gate-verified because gate never completes din4108. |

**Contract:** A remedy that exists on initial evaluate but leaves status **Fail** after option-0 / sequential applicables (or triggers bare Fail on sibling re-evaluate) is **blocking**. The U′ panic is blocking even though initial evaluate had remedies.

---

## Contract / gaming spot-check

| Rule | Result |
|------|--------|
| Remedies edit fields checks read (`lengthM`, `thicknessM`, `psi`, `areaM2`, `shadingFc`, …) | **PASS** |
| en/de copy differ on new U′ / length remedy text | **PASS** (e.g. “Shorten thermal bridge …” vs “Wärmebrücke … verkürzen …”) |
| No fingerprint / epsilon fold / id_score / tag_fp gaming in evaluate path | **PASS** (spot-check; gate `scan_evaluate_gaming` did not fire before panic) |
| Fail must have ≥1 remedy on every evaluate (including post-flip) | **FAIL** (`din4108-6.u-prime.wall-north`) |
| Compliance gate unchanged | **PASS** (read-only audit; gate source not edited) |

---

## Fixer claim vs this audit

| Fixer claim | This audit |
|-------------|------------|
| `check_u_prime` always carries clearing `lengthM` / area / forced insulation | Source blocks present; **gate still panics** on flip re-evaluate |
| Five gate Fails cleared | **Not observed** — gate aborts on din4108 |
| 95 tests passed | **Confirmed** |
| No test deletion | **Confirmed** (no named test vs `HEAD`) |

---

## Blocking list

- `din4108-6.u-prime.wall-north` — compliance gate panic: Fail with **no remedies** during `assert_remedies_flip_fails` re-evaluation after zone-ht / other remedies thicken `thin-eps`; fixer `lengthM` fallback insufficient at runtime.
- `din4108-6.u-prime.roof` — not gate-cleared (gate aborts; same `check_u_prime` path).
- `din4108-2.summer.zone-living` — not gate-cleared (gate aborts before din4108 PASS).
- `din4108-2.zone-ht.zone-living` — not gate-cleared (gate aborts; flip interaction triggers U′ bare Fail).
- `din4108-10.app.wall-north.thin-eps` — not gate-cleared (gate aborts before remedy-flip completes).

---

## Non-blocking

- Test count 96→95 without a named deleted/ignored test function (see above).
- Family unit/regression suite green (95/95) — does not substitute for compliance gate.

---

## Generated logs (this audit)

- `🗑️generated/compliance-gate-r2.txt`
- `🗑️generated/compliance-gate-r2-backtrace.txt`
- `🗑️generated/din4108-nx-test-r2.txt`
- `🗑️generated/u-prime-probe-r2.txt` (Python branch simulation; illustrative only)
