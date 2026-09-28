# DIN 4108 Browser Walk

Editor: `http://127.0.0.1:6091/` (`dev-din4108-react-dev`, Vite ready). Catalogue example in the UI is **Compliant ETICS dwelling** (German title **Regelgerechtes WDVS-Wohnhaus**).

## English

1. Evaluate on the open example: DIN 4108-7 complies, `[Pass] Airtightness n50` 1.500 1/h vs 1.500 1/h, u=1.00.
2. Airtightness n₅₀ set to 4.5, then Evaluate: `DIN 4108-7 — does not comply (✓0 ⚠0 ✗1 · u=3.00)`.
3. Expanded check: `[Fail] Airtightness n50 — Building airtightness @ airtightnessN50 — 4.500 1/h vs 1.500 1/h u=3.00 — DIN 4108-7 §4 §4.2`.
4. Explanation: `n50 = 4.50 h⁻¹ vs limit 1.5 h⁻¹ (mechanical ventilation = true).`
5. Apply: `Remedy: Achieve n50 ≤ 1.5 h⁻¹ (currently 4.50 h⁻¹) by tightening the envelope. 4.500 1/h → 1.500 1/h`.
6. Field returned to 1.5 1/h. Evaluate again: `DIN 4108-7 — complies (✓1 ⚠0 ✗0 · u=1.00)`, `[Pass]` at 1.500 vs 1.500.

## Deutsch

Settings → Language → Deutsch. The same fail, after setting n₅₀ back to 4.5:

- `DIN 4108-7 — nicht erfüllt (✓0 ⚠0 ✗1 · u=3.00)`
- `[Nicht bestanden] Luftdichtheit n50 — Gebäudedichtheit @ airtightnessN50 — 4.500 1/h vs 1.500 1/h`
- `Erläuterung n50 = 4.50 h⁻¹ gegenüber Grenzwert 1.5 h⁻¹ (mechanische Lüftung = true).`
- `Anwenden — Abhilfe: n50 ≤ 1.5 h⁻¹ erreichen (aktuell 4.50 h⁻¹) durch dichtere Gebäudehülle.`

Anwenden restored `DIN 4108-7 — erfüllt` and `[Bestanden] Luftdichtheit n50` at 1.500 vs 1.500, u=1.00.

Chrome verbs in Deutsch: Auswerten, Abhilfe anwenden, Eingaben, Ergebnisse.
