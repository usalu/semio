# 📓️ Content Audit — Architecture Energy Quizzes (quizze.architektur-und-technologie.de)

Read-only audit of the catalog, the four energy quizzes and the two teaching READMEs, against `📓️design.md` §6
(scoring), `📓️content-report.md` and `📓️research-quiz-content-data.md` in this ticket. Method: recomputed every
formula the READMEs and item explanations claim (unit conversions, U-value-derived loads, ventilation-loss and
final-energy formulas, classification profile distances/`d_max`), cross-checked values for the same building across
quizzes, checked badge rules against the stated requirement, and verified two physically-load-bearing claims against
current sources (web search, cited below). No repo file other than this report was modified.

Files audited: `🎓️teaching/README.md`, `🎓️teaching/🏛️architecture/README.md`,
`🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` (catalog), and the four quiz files under
`🎓️teaching/🏛️architecture/⚡️energy/{🧲️physics,🔥️heating,❄️cooling,📊️demand}/❓️quiz/🔣️.json`.

## Summary

The overwhelming majority of the content re-checks correctly: every unit conversion, every U-value/ventilation/
final-energy/cost formula, and the classification `d_max`/closest-pair distance recompute to the stated values
(see “Confirmed correct” below). One factual/labelling error was found (finding 1, high severity) and three minor
issues (findings 2–4, low/informational). Badge rules exactly match the stated requirement (finding 5, confirms
correct — no defect).

## 1. Physical correctness

### Finding 1 — “GEG 2024 minimum” item actually represents the older, looser EnEV 2014/GEG 2020 rule (High)

- **File**: `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/❓️quiz/🔣️.json`
- **Pointer**: `/tasks/1/items/4` (task `heating-load-and-demand`, item id `geg-2024`)
- **Current text**: label `{"en": "New single-family house built to the GEG 2024 minimum", "de": "Neues
  Einfamilienhaus nach GEG-2024-Mindeststandard"}`, `values: {"heating-load": 40, "heating-demand": 50}`,
  explanation: “reference U-values of GEG Annex 1 (wall 0.28, window 1.3 W/(m²·K))… IWU calculates 43.7 kWh/(m²·a)
  per usable floor area A_N…, about 50 per m² of living area”.
- **Problem**: the item computes the performance of a building built **exactly to the reference building’s own
  envelope U-values** (i.e. 100 % of the Referenzgebäude). That is not the current legal minimum for new
  residential construction. Since the GEG amendment in force since 2023 (§ 15 GEG/GModG), a new residential
  building’s annual primary energy demand must be at most **55 %** of its reference building’s value — tightened
  from 75 % under GEG 2020 (source: GEG‑Infoportal, `gmodg.bund.de/GEGPortal/DE/GEGRegelungen/Neubau/Wohngebaeude`;
  `geg-info.de/geg_2024/018_§_gesamtenergiebedarf.htm`, § 18). A building that only reaches 100 % of the reference
  (this item’s value) would not be permitted today; the real “GEG 2024 minimum” is much closer to the **kfw-55**
  row in the very same task (`/tasks/1/items/3`: 25 W/m², 35 kWh/(m²·a)) than to this one (40 W/m², 50 kWh/(m²·a)).
  Supporting evidence inside the same catalog: the demand quiz uses the identical “100 %-of-reference” calculation
  but labels it correctly as the **older** standard — `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/❓️quiz/🔣️.json`
  `/tasks/0/categories/5` (`profile-f`) and `/tasks/0/items/2` (item `enev-2014`, label “New build to EnEV 2014 /
  GEG 2020”, heating value 55) — i.e. the catalog itself already knows that this performance level is the
  pre‑2023 rule, not “GEG 2024”.
- **Proposed correction**: rename the heating-quiz item to match the demand quiz’s correct term, e.g. “New
  single-family house to EnEV 2014 / GEG 2020” / „Einfamilienhaus nach EnEV 2014 / GEG 2020“, and reconcile its
  value with the demand quiz’s twin (55, not 50 — see finding 1b). If a genuine “built to today’s GEG minimum”
  row is wanted, add one near 25–30 W/m² / 30–35 kWh/(m²·a) (55 % of a comparably scaled reference), distinct from
  `kfw-55`.
- **Severity**: High — this is the kind of factual claim (“what must a new German house achieve today by law”) the
  quiz exists to teach correctly, and it currently understates the legal minimum’s stringency by roughly 30 %.

### Finding 1b — same standard, two different heating-demand values across quizzes (Medium, folds into 1)

- **Files / pointers**:
  `…/🔥️heating/❓️quiz/🔣️.json` `/tasks/1/items/4/values/heating-demand` = **50**
  `…/📊️demand/❓️quiz/🔣️.json` `/tasks/0/categories/5/profile/heating` (and `/tasks/0/items/2`) = **55**
- **Cross-check that shows these are meant to be the same building**: the *cooling* value for the same standard
  matches exactly between quizzes — `…/❄️cooling/❓️quiz/🔣️.json` `/tasks/1/items/1/values/cooling-demand`
  (item `new-home-geg`, “New home to GEG with external blinds”) = **5**, identical to the demand quiz’s
  `/tasks/0/categories/5/profile/cooling` = **5**. Only the heating axis disagrees (50 vs 55), a ~10 % gap.
- **Proposed correction**: once finding 1’s relabelling is applied, set the heating quiz’s value to 55 kWh/(m²·a)
  (matching demand/cooling) or explain in both explanations why they differ (e.g. different assumed floor-area
  basis). Source: internal consistency, no external source needed.
- **Severity**: Medium (folds into finding 1 as the same underlying labelling problem).

### Finding 2 — `sfh-2000s` heating load does not reproduce from the README’s own formula (Low)

- **File**: `…/🔥️heating/❓️quiz/🔣️.json`
- **Pointer**: `/tasks/1/items/5/values/heating-load` = **50** W/m² (item id `sfh-2000s`, IWU type EFH_J)
- **Recomputation**: `🎓️teaching/🏛️architecture/README.md` “Conventions for derived values” gives Load =
  (H_T/A + 0.34 × 0.6 × 2.5) × 32 K. `📓️content-report.md` §2.2 states H_T/A = 1.14 for EFH_J. That gives
  (1.14 + 0.51) × 32 = **52.8** W/m², a ~5.6 % gap — larger than the ≤ 2 % rounding seen when the same formula is
  applied to every other existing-building row (EFH_E 160.3→160, EFH_F 116.5→115, EFH_I 73.3→73, NBL_GMH_F
  60.2→60, MFH_B 94.4→95, all reproduced almost exactly).
- **Proposed correction**: most likely the H_T/A printed in the content report (1.14) is itself coarsely rounded
  (a true value near 1.06 reproduces 50 exactly); recommend a quick re-check of the IWU 2015 appendix C.3 cell for
  EFH_J before treating the encoded value as confirmed. Not flagged with confidence as wrong, only as unverified.
- **Severity**: Low — no external source consulted to resolve it either way; flagged for a primary-source spot
  check.

### Finding 3 — minor Wh rounding on `chocolate-bar` (very low)

- **File**: `…/🧲️physics/❓️quiz/🔣️.json`
- **Pointers**: `/tasks/0/items/12` (classification, “≈ 0.63 kWh”) and `/tasks/2/items/2/value` = **630** (sorting
  `energies`, item `chocolate-bar`)
- **Current text**: “540 kcal ≈ 630 Wh (1 kcal = 1.163 Wh)”
- **Check**: 540 × 1.163 = 628.02 Wh, not 630. Difference is 0.3 %, invisible to the task (draw is random, next
  neighbour `daily-food` is at 2,700 Wh, factor 4.3×) and well inside the “representative, rounded” convention
  stated in the README’s Sources section.
- **Proposed correction**: optional only — either state 628 Wh or drop the explicit “1 kcal = 1.163 Wh” factor
  since it doesn’t quite reproduce the printed value. Source: standard 1 kcal = 4.184 kJ = 1.163 Wh conversion.
- **Severity**: Very low / cosmetic.

## 2. Classification correctness

No defects found. `power-or-energy` (physics) unambiguously assigns each item to power (W, a rate) or energy (Wh/
kWh/kcal, an amount); the prompt explicitly resolves the one genuinely tricky case (“kWh/a … counts as energy
demand”) before the learner meets `energy-certificate`. `standard-profiles` (demand) uses neutral category labels
“Profile A”…“Profile F” with **no monotonic relationship to the true ranking** — spot-checked: heating values in
letter order are 26, 303, 20, 135, 15, 55 (A…F), i.e. neither ascending nor alphabetically matching age/quality, so
the label order does not leak the answer. Confirmed by recomputation: closest pair (`profile-a`/kfw-40 vs
`profile-e`/passive-house) Euclidean distance over the four normalised axes = 0.206, `d_max` (profile-b vs
profile-e) = 1.496 — both reproduce `📓️content-report.md` §2.4 exactly.

## 3. Didactic quality

### Finding 4 — asymmetric magnitude-span framing between the two sorting tasks (Very low)

- **File**: `…/🧲️physics/❓️quiz/🔣️.json`
- **Pointers**: `/tasks/1/prompt` (`powers`) vs `/tasks/2/prompt` (`energies`)
- **Current text**: `powers` prompt: “…the values span about 25 orders of magnitude.” `energies` prompt has no
  equivalent sentence, although its own span (15 Wh to 1.644 × 10¹⁷ Wh) is also large (≈ 16 orders of magnitude,
  matching `📓️content-report.md`’s “16.0 decades”).
  Proposed correction: add a matching clause to the `energies` prompt, e.g. “…the values span about 16 orders of
  magnitude.” Severity: very low, cosmetic consistency only.

Everything else checked as strong: every matching/sorting prompt states the unit and the sort direction explicitly
(“ascending order, smallest first”); the `heating`/`cooling` matching prompts define both dimensions physically
(“the heat flow needed on the coldest design day” / “the heat to remove on a hot design day”) rather than just
naming them; the `final-energy` prompt explicitly excludes primary-energy factors and PV credits from scope,
pre-empting a very natural confusion with the (unrelated) primary-energy figures elsewhere in the research file.
German text is idiomatic and consistently gender-fair — no generic-masculine agent nouns were found anywhere in the
four quiz files or the two READMEs (`Sporttreibende(n)`, `Person`, `Schulkinder`, `Gast` are all used instead of
e.g. `Sportler`, `Schüler`); en/de pairs say the same thing in every item spot-checked. One very minor English
idiom nit: the catalog introduction’s “partly right answers earn partial points” (`🔣️.json` `/introduction/
paragraphs/2/en`) reads slightly informally; “partially correct answers” would be more idiomatic — cosmetic only,
not listed as a numbered finding.

## 4. Distinguishability for matching/sorting

No new defects; one already-accepted trade-off is worth restating here since it is exactly what this criterion
asks about:

- **`u-values` (heating)** — `…/🔥️heating/❓️quiz/🔣️.json` `/tasks/0/items`: 5 of 16 neighbour pairs fall below the
  design’s 1.25 soft guideline (down to a 1.15 factor between `window-geg` 1.3 and `half-timbered-wall` 1.5
  W/(m²·K); also 0.5↔0.6, 1.5↔1.8, 1.8↔2.1, 3.6↔4.3). `📓️design.md` and `📓️content-report.md` §4.1 already flag
  and knowingly accept these as warnings (not errors) because they are real BAnz/GEG table cells, not invented
  numbers, and the logarithmic pair-concordance scoring (design §6) already charges very little for swapping such
  close neighbours. Recommendation: no change needed; confirmed as an intentional, reasonable trade-off rather
  than an oversight.
- All other matching tasks clear the 1.25 guideline everywhere except two boundary cases at exactly 1.25
  (`air-change-rates` 20↔25 1/h; `cooling-load-and-demand` load 40↔50 and demand 8↔10/20↔25) — acceptable, not
  below the threshold.
- Sorting tasks (`powers`, `energies`) both clear 1.25 comfortably (smallest factors 1.60 and 3.70).
- Draw sizes are all 59–100 % of the item pool (never a tiny fraction of a large pool), giving meaningful
  run-to-run variety without hiding most of the content in any single run — no finding.

## 5. Badge definitions

**Confirmed correct — no defect.** Checked `🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` `/badges` against the
stated requirement and design §7:

| Requirement | Catalog rule (pointer) | Verified match |
|---|---|---|
| “Heating expert” = perfect score on the heating quiz | `/badges/1/rule` = `{"kind":"perfect-quiz","quiz":"heating"}` | Exact — some result of quiz `heating` scores 1. |
| “Numerical Brain” = all numeric sortings flawless | `/badges/4/rule` = `{"kind":"perfect-tasks","taskKind":"sorting"}` (no `quiz` filter) | Selector resolves to exactly the catalog’s 2 sorting tasks (`powers`, `energies`, both in `physics` — the only quiz with `sorting` tasks), matching content-report §4.1 “numerical-brain 2 sorting tasks”. Catalog-wide selector (not quiz-scoped) is correct future-proofing: a later quiz with a sorting task would automatically be included. |
| “Pattern seer” = perfect score on all classification tasks | `/badges/5/rule` = `{"kind":"perfect-tasks","taskKind":"classification"}` | Selector resolves to exactly the catalog’s 2 classification tasks (`power-or-energy` in `physics`, `standard-profiles` in `demand`), matching content-report §4.1 “pattern-seer 2 classification tasks”. |

Labels/descriptions in the catalog and in `🎓️teaching/🏛️architecture/README.md`’s badge table agree with each
other and with the rules; evaluation order in the catalog (`physics-expert, heating-expert, cooling-expert,
demand-expert, numerical-brain, pattern-seer, completionist`) matches `📓️content-report.md` §2.5.

## Confirmed correct (representative sample of what was re-derived, not just re-read)

- Unit conversions: 540 kcal → 628 Wh (vs. printed 630, finding 3); 10,529 PJ → 2.925 × 10¹⁵ Wh; 592 EJ → 1.644 ×
  10¹⁷ Wh and → 1.87 × 10¹³ W (self-consistent power/energy pair for the same 592 EJ/a figure, 8760 h basis);
  527 TWh ÷ 8,784 h → 6.0 × 10¹⁰ W; 1,361 W/m² × Earth cross-section → 1.74 × 10¹⁷ W.
- Heating loads from H_T/A via the README formula reproduce the quiz values for 5 of 6 existing-building rows
  (EFH_E/F/I, NBL_GMH_F, MFH_B) to within rounding; the sixth is finding 2.
- Ventilation-heat-loss formula (README “Conventions”) reproduces all six `standard-profiles` ventilation axis
  values exactly (61, 43, 32, 9, 6.5, 8 kWh/(m²·a) for n50/heat-recovery pairs 8/0 %, 4/0 %, 1.5/0 %, 0.8/80 %,
  0.6/85 %, 0.6/80 %).
- `final-energy` (demand quiz) formula (heating ÷ system efficiency + hot water 12.5 ÷ its efficiency + auxiliary)
  reproduces all 9 encoded values exactly (14, 23, 30, 48, 80, 122, 182, 274, 409).
- Cost axis of `standard-profiles`, including the plus-energy house’s negative net cost (−2 €/(m²·a)), reproduces
  from the README’s price conventions once PV self-consumption is credited at the avoided purchase price (30 ct)
  and only the exported surplus at the feed-in tariff (8 ct).
- Cross-quiz building identity checks beyond finding 1b all agree: 1960s SFH (IWU EFH_E) heating load/demand
  160 W/m² / 303 kWh/(m²·a) is identical across `physics` (items `heating-load`/`heating-load-old-house`/
  `heating-demand-house`) and `heating` (`sfh-1960s`); WSchVO 1995 (EFH_I) 135 kWh/(m²·a) identical across
  `heating` (`sfh-1990s`), `demand` classification (`profile-d`) and `demand` matching (`wschvo-1995-gas`); KfW 40
  26 kWh/(m²·a) identical across `heating` (`kfw-40`) and `demand` (`profile-a`, `kfw-40-heat-pump`’s 26 input);
  passive house 15 kWh/(m²·a) identical across `heating`, `physics`-adjacent PHI citation and `demand`
  (`profile-e`); passive-house cooling demand 2 kWh/(m²·a) identical across `cooling` (`passive-house-home`) and
  `demand` (`profile-e`).

## Sources consulted (web)

- GEG‑Infoportal (Bund), Neubau Wohngebäude requirements: `gmodg.bund.de/GEGPortal/DE/GEGRegelungen/Neubau/
  Wohngebaeude/Wohngebaeude.html` — 55 % of reference building since the 2023 GEG amendment (§ 15), down from
  75 % under GEG 2020.
- `geg-info.de/geg_2024/018_§_gesamtenergiebedarf.htm` — GEG 2024 § 18, confirms the 0.55× reference-building
  factor.

No repo file other than this report was modified during the audit.
