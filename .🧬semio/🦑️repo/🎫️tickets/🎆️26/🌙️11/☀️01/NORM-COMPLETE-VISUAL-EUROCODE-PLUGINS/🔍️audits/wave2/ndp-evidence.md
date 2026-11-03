# NDP evidence (wave 2)

Script: `🧰️ndp/ndp_evidence.py` (idempotent), then `codegen.py` and `--check` (ok, 2937 entries), registry test `🧪️tests/🔖️ndp-registry/🐍️.py` (ok, 2937 entries, 2679 unverified). No cargo, no git write. "ready for rebuild: ndp evidence" appended to `integrate` and `build-en1995` `latest.md`. en1995 v7 owner notified in `📌️important/requests.md`.

## 1. Independent source search
Searched `.venv`: blue-prints 0.8.0 (`eurocode/en_1995_1_1_2004|2023|2025`: only formulas, chapter 7 and annex E; `en_338_2016`: strength tables only), structuralcodes 0.7.3, groundhog 0.15.0, plus a grep for kmod/k_def tables over the venv and the repo. None holds EN 1995-1-1 Table 3.1 (k_mod) or Table 3.2 (k_def). No row could be kept verified.

## 2. Downgrades
- 117 k_mod/k_def rows (`ndp.en1995.kmod.*`, `ndp.en1995.kdef.*`): unverified, evidence states search and needed print.
- Found by the new rule: 21 more verified rows without any source (EN 14080 GL24h/GL28h/GL32h, only "flagged verified in the code"): unverified. The other 15 GL rows cite named public sources (non-authoritative), kept verified with `source.edition` "EN 14080:2013" (was a placeholder).
- Registry: 279 verified before, 258 after (240 EN 338:2016 C/D, 3 steel, 15 GL public); en1995 255 verified / 480 unverified.

## 3. Registry test rule
`evidence_invariants` in the registry test: verified implies non-empty evidence, no "CARRIED-OVER" or "no independent source", and a real `source.edition` (not the "to be recorded" placeholder). Unverified rows keep the placeholder.

## 4. Dependent verdicts, recomputed from the oracle
- Oracle `🔮️oracles/🚦️check-outcome/🐍️.py` counts k_mod/k_def reads (compliance rows by id family, design-case rows dynamically); adapter `🧪️tests/🚦️check-outcome-en1995-1/🐍️.py` maps a pass with reads to `unverifiedBasis`, a fail stays fail.
- Feature rows regenerated from that oracle (script in scratchpad, not hand-edited): pass-01, 04, 08 to 20 became `unverifiedBasis` (14 rows); pass-02, 03, 09, 21 and all fail rows unchanged; utilisations all unchanged.
- Rust tests (not run): `compliance` tests: compliant example asserts no failing check and complies iff no unverified; johansen checks expect `UnverifiedBasis` with a k_mod ndp; added assertion that kmod/kdef rows are unverified. `compliance-report` and the two glulam examples: `complies()` replaced by "no failing check" because `complies()` is false for UnverifiedBasis.

## Caveat
The oracle's read attribution (which design-case rows read the table when `kmod` is explicit, fire rows) is inferred from the oracle code, not from the Rust; the rebuild verifies it. Any mismatch is a feature row to recompute, not a value to edit.
