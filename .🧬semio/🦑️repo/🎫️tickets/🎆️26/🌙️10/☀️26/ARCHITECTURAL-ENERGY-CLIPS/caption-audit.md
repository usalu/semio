# Canonical Caption Audit

Checked 1652 language-specific clauses across 110/110 canonical sources in 63.1 s.

Current overflowing clauses: 0. Actual shared caption_bar and bundled Pango typography were used for every unique clause. The ticket-only boundary search verifies greedy line endpoints with the original body_text measurements and first checks equivalence against the original wrapping implementation.



## Final Result

All 1,652 EN/DE narration clauses across 110 canonical sources fit the two-line caption limit. Five intro classes have no NARRATION field. The audit found 43 overflows and each was shortened in its existing narration field, preserving numerical values, units, equation relationships, and the existing instructional claims. Before that sweep, the Physics heat-pump comparison had already been repaired consistently in English and German.

The 39 language-neutral canonical source/scene/key cases now pass through the unmodified real caption_bar implementation in both languages. Each case also reproduces its previous three- or four-line overflow. The first batch passed in 278.390 seconds; the remaining 15 cases passed in 152.254 seconds. The full post-repair corpus check reported zero overflows in 63.1 seconds by reusing unchanged clauses that had already passed actual font/layout measurement.

The ticket audit accelerates greedy word wrapping by deriving a candidate boundary from one real shaped paragraph, verifying the chosen line and its next-word candidate with the existing body_text interface, and falling back to the original wrap routine for non-monotonic glyph positions. Equivalence checks cover short, long, and fallback-glyph examples before use. The production wrapping implementation is unchanged. All changed clauses were independently checked by the original production caption_bar regression.

Narration edits are complete and source stability has been reported to the parent and render-cache agent. The comprehensive actual-render pipeline owns final video verification; this caption audit does not claim that the complete videos have finished rendering.

## Changed Clause Cases

| Canonical Source | Scene / Key | Changed Language(s) |
|---|---|---|
| `🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | `Beat4_Waermepumpe` / `compare2` | DE |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | `Beat3_LatenteLast` / `dx` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | `Beat3_LatenteLast` / `r` | DE |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat1_Fensterregeln` / `winter` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat1_Fensterregeln` / `rule` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat2_StossOderKipp` / `summer` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat3_Querlueftung` / `limit` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat3_Querlueftung` / `winter` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat4_Auftrieb` / `rho` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat4_Auftrieb` / `winter` | EN, DE |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat5_Nachtlueftung` / `night` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat5_Nachtlueftung` / `winter` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat6_GrenzenDerFreienLueftung` / `intro` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat6_GrenzenDerFreienLueftung` / `control` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat6_GrenzenDerFreienLueftung` / `recovery` | EN, DE |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | `Beat6_GrenzenDerFreienLueftung` / `handover` | EN, DE |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` | `Beat1_Grundtypen` / `exhaust` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` | `Beat1_Grundtypen` / `verdict` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` | `Beat2_Waermerueckgewinnung` / `formula` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` | `Beat2_Waermerueckgewinnung` / `summer` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` | `Beat3_Anlagenvergleich` / `outro` | EN, DE |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` | `Beat3_Anlagenvergleich` / `passivhaus` | EN |
| `🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` | `Beat4_Kanalquerschnitt` / `area` | EN |
| `🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | `Beat7_Waermedurchlasswiderstand` / `gradient` | EN |
| `🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | `Beat9_WaermestromFormel` / `power` | DE |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py` | `Beat3_LueftungslastImSommer` / `intro` | EN |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py` | `Beat3_LueftungslastImSommer` / `sens` | EN |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` | `Beat1_SolarIrradiance` / `chart` | EN |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` | `Beat5_SolarCoolingLoad` / `intro` | EN |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | `Beat2_VolumeFlowEquation` / `formula` | EN |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | `Beat3_IsolateAirflow` / `substitute` | EN |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | `Beat3_IsolateAirflow` / `qvr` | EN |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | `Beat3_IsolateAirflow` / `load` | EN |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | `Beat3_IsolateAirflow` / `compare` | DE |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` | `Beat1_LastZuerstSenken` / `reserve` | EN |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` | `Beat2_SorptionsKuehlung` / `intro` | DE |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` | `Beat2_SorptionsKuehlung` / `nachteil` | EN, DE |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` | `Beat3_KomfortStrategie` / `yes` | EN |
| `🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` | `Beat3_KomfortStrategie` / `reserve` | EN |

The Physics comparison's English wording was also shortened for consistency. Full reviewed replacements are retained in the portable test contract and ticket `caption-repairs.json`; no generated logs are needed to understand the changes.

## Inputs and Verification Files

- `caption-audit.py`: canonical AST extraction and actual caption layout audit.
- `caption-edit.py`, `caption-repairs.json`: reviewed narration-only byte-range edits; the helper refuses to overwrite a concurrently changed source.
- `caption-regression.py`, `caption-validated.json`, `caption-regression.md`: content-addressed, original-layout regression verification.
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️integration.py` and `🔣️integration.json`: portable production regression cases and real third-party Manim/Pango checks.

All temporary SVGs and JSON audit output live under this ticket's generated folder. The root task will remove generated output after the final render checks while retaining these audit inputs and reports.

## Caption Count Reconciliation

The portable contract contains **39 canonical scene/key cases and 44 previous-language strings**; five keys have both EN and DE previous strings. This is **43 overflows from the all-source caption sweep plus one earlier Physics comparison failure**. It is not a claim that the sweep found 44 failures.

The retained records establish the partition: `caption-repairs.json` contains 42 unique language-specific sweep repairs; the separately registered ventilation `Beat3_LatenteLast` / `dx` / EN case is the 43rd. The remaining previous string is Physics `Beat4_Waermepumpe` / `compare2` / DE, whose earlier three-line failure is still recorded in `🗑️generated/smoke/results.json`. Every retained repair's previous text matches its portable case. The original caption oracle has validated all 44 previous strings across the 39 keys.

The later **Opaque envelope** qualifier restoration changes one English narration datum and its matching portable case/repair input. It adds no new overflow occurrence or canonical key. The targeted original caption oracle check establishes the restored wording still fits.
