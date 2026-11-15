import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = next(path for path in HERE.parents if (path / "🧰️framework").is_dir())
REGISTRY = ROOT / "🎓️teaching" / "🏛️architecture" / "⚡️energy" / "🏗️authoring" / "📚️references" / "🔣️.json"
entries = json.loads(REGISTRY.read_text(encoding="utf-8"))["entries"]

HEAD = r'''# Consistency pass over the five energy videos (agent `consistency`)

Date 2026-11-13. ENERGY = `🎓️teaching/🏛️architecture/⚡️energy`. TK = ticket folder. Scripts and probes: `TK/consistency/`. Three sub-agents did the bulk edits of items 1, 3, 4, 6 (their full fragments are appended as appendices A to C and carry the exact file:line lists); items 2, 5, 7 and the integration fixes were done by `consistency` itself and are listed here in full.

## 0. Result

| item | status |
|---|---|
| 1 symbols (internal gains `i`, `e_E`, `G_{t,20/15}`, loads, forbidden symbols, indexed Φ) | done, grep proof in appendix B |
| 2 value conflicts | g_⊥ and the whole glazing chain owned by the running example; design day vs example winter day named on screen and in narration; a third conflict (F_S, F_F, F_W, F_C in demand) found and fixed; numbers recomputed (section 2) |
| 3 units (compound units to `\frac`, factors with units, bare brackets, thin spaces) | done, appendix C |
| 4 one reference registry | `🏗️authoring/📚️references/`, 55 entries, schema, tests, scan guard; all on-screen references come from it (section 4, appendix A) |
| 5 parentheses | the Anta `(` `)` reads like `[` `]` at video size: fixed at the font layer (section 5) |
| 6 German wording | `Kühltechnik`, `Grenzwert nach Pettenkofer`, animation clause (appendix B) |
| 7 verification | narration guard clean (all five), 175 family tests plus 50 final tests pass, posters recaptured, 14 scenes looked at (sections 7, 8) |

## 1. Symbols (summary; details in appendix B)

- Internal gains: lower-case `i` everywhere (`Q_i`, `\dot{Q}_{\mathrm{i}}`, parts `i,p`, `i,e`, `i,l`, latent `i,lat`); speech lexicon entries and cases added. The upright `\mathrm{i}` was kept because heating already shipped it.
- `e_E` = end-energy factor `Q_E/Q_h`; `e_P` (primary energy, `e_E · f_P`) is a different quantity and stays. `G_{t,20/15}` unified everywhere, including the heating readout `G_{t20/15}` that the first pass missed (`🔥️heating/🎬️clip/🧮️calculation/🐍️.py:573`, found by looking at the frame) and the plain-text label `G_t,20/15` with a literal underscore (`:640`, now without symbol). `F_{Gt}` is now `0,024 kKh/(Kd) · G_{t,20/15}` in heating (`🧮️calculation/🐍️.py:431-432, 638`), demand and concepts (`📖️concepts/📝️.md:401`); the old `24 h/d · G` was off by the factor 1000 against the unit kKh/a.
- Forbidden symbols removed from shown text, README, concepts. Indexed Φ only for `Φ_k Φ_c Φ_r` (+ the `si`/`se` split) and `Φ_HL` once (demand regulations chapter).

## 2. Value conflicts (all dependent numbers recomputed)

The running example (`🏗️authoring/📏️running-example/🔣️.json` + `🐍️.py`, tests `🧪️tests/🐍️.py`, now 10 tests) owns:

| key | value | used by |
|---|---|---|
| `glazing.gPerp` (0,60 = 0,50 direct + 0,10 secondary inward; 0,20 reflected, 0,20 outward) | 0,60 | physics, heating solar, cooling solar, demand recap |
| `glazing.frameFactorFF` / `incidenceFW` / `shadingFSWinter` | 0,70 / 0,90 / 0,80 | heating solar, demand recap (new) |
| `glazing.sunProtectionFCBlind` / `FCNone` | 0,15 / 1,0 | heating solar, cooling solar |
| window 1,2 m x 1,6 m = 1,92 m²; `winterIrradianceWM2` 400; `clearSkyIrradianceWM2` 500 | | heating solar, physics |
| `winter.outdoorC` (renamed from the misleading `outdoorDesignC`) | 0 °C, Δθ = 20 K | ventilation, heating, demand recap: "Example winter day" |
| `design.outdoorC` + `referenceId` | -12 °C, Δθ = 32 K, `Q̇_H = 6080 W` | demand power: "Design day", DIN/TS 12831-1:2020-04 |

Changes and numbers:

- **g_⊥ in physics** (`🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` Beat5_Strahlung, from about line 2122, the gain panel and the pane): was 0,50 (only the direct-transmission share); now `g_⊥ = 0,60` from the running example. The pane was a 1,25 m x 1,2 m = 1,5 m² window that matched no other video; it is now the example window 1,2 m x 1,6 m = 1,92 m². Recomputed: `g_⊥ · I_s = 0,60 · 500 W/m² = 300 W/m²` (was 250), `Q̇_S = 0,60 · 1,92 m² · 500 W/m² = 576 W` (was 375 W). Concepts line 101 updated. Frame: `shots/physics-gain-de.png`.
- **heating solar-gains** (`🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:117-131, 837`) and **cooling solar-radiation** (`❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py:101`) read every factor from the running example; values unchanged there.
- **Third conflict (missed by the audit)**: the demand recap used `F_S = 0,7` and `F_C = F_F = F_W = 1` while heating uses `F_S = 0,80`, `F_F = 0,70`, `F_W = 0,90`. Demand now uses the heating values (`📊️demand/🎬️clip/🔁️recap/🐍️.py:139-148`; the Beat3 formula row is now `A · I_s · g_⊥ · F_S · F_C · F_F · F_W`; narration keys `fs`, `weitere`, `solar_block`). Numbers: `Q_S` = 590 kWh (was 819), `Q_g` = 813 kWh (1042), `η` = 0,86 (0,80), `Q_h` = 940 kWh (803) in the transition month; January `Q_g` = 459 (551), `η` = 0,98 (0,97), `Q_h` = 2647 (2561); July `Q_g` = 713 (904), `η_c` = 0,53 (0,61), `Q_c` = 153 kWh (256). The narration values of `η` and the clause `Q_S = 590 kWh` were updated; the fixture `🔁️recap/🧪️tests/🧮️balance/🔣️.json` + `🐍️.py` is recomputed from an independent numpy derivation and a new test pins the factors to the running example and the quoted `Q_S` to the computed one.
- **Design basis**: `winter.outdoorC` = 0 °C is the *example winter day*, `design.outdoorC` = -12 °C the *design day*. On screen: ventilation heat-flow chip `Beispiel-Wintertag` / `Example winter day` (was `Winter`; summer chip `Beispiel-Sommertag`; `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:252`), heating fundamentals label `Beispiel-Wintertag / außen 0 °C` (`🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:86, 174`), demand power `Auslegungstag θ_e = -12 °C` with `Norm-Außentemperatur, DIN/TS 12831-1:2020-04` (two lines from `design_day_label` / `design_day_source` of the running example; `⚡️demand-power/🐍️.py` Beat2; constants `THETA_E = _example.THETA_E_DESIGN_C`, `DESIGN_DT = _example.D_THETA_DESIGN_K`). Narration: ventilation heat-flow `delta` and `winter`, handoff `dt` and `winter`, heating fundamentals `setup`, demand recap `januar`, demand power `normtemperatur` (design day) plus a new clause `beispieltag` ("the example winter day ... is a milder case"; the German caption wrapped to 3 lines in one piece and was split). Concepts line 416 names both days.
- **U-values, H_T, H_V**: wall 0,24 / roof 0,20 / window 1,1 / door 1,3 / floor 0,30 W/(m²K), `H_T = 115 W/K`, `H_V = 75 W/K` are used consistently (demand power derives them from the running example; `6,08 kW` follows). The 1960 wall `U = 1,43` and its insulated variant `0,16` in heating are a separate named building case. No contradiction left.
- Checked and consistent: ρ and c_p, `c_Luft = 0,34`, latent 770/810 W, sensible 1500/450 W, `G_{t,20/15} = 3500 Kd/a`.

## 3. Units

Appendix C. Summary: 121 compound units in about 66 display lines are now fractions; 70 `\;` before units became `\,`; bracket units fixed (12 in concepts, not 5); unit-less factors fixed in the ventilation and physics quiz, concepts and the demand README. Narration `$…$` and axis labels keep the inline form on purpose.

## 4. References

One canonical registry: `🏗️authoring/📚️references/` (`🔣️.json`, `🧬️schema/🔣️.schema.json`, `🐍️.py`, tests `🧪️tests/🐍️.py` + `🔣️.cases.json`, registered in `📜️script.ts`). API: `designation`, `short`, `ref(id, locator, language=)`, `chip`, `cite`, `status_chip`, `card`, `bibliography`, `all_forms`. Scene chips, `_din_ref`, bibliography cards and the physics source card are produced by `ref/cite/card`; prose in quiz/README/concepts/narration is rewritten to the exact registry forms and guarded: the scan tests read all string literals of the five videos plus visuals, quiz JSON, README, concepts and fixtures and fail on any designation (DIN/VDI/ISO/GModG...) whose exact text is not one of `all_forms()`, and on entries without edition, URL or verified date. The short form without edition is allowed only inside narration clauses (spoken text; the chip shows the edition) and as successor in the status chip.

Decisions (details in appendix A):

- **GModG**: the official page https://www.gesetze-im-internet.de/geg/BJNR172810020.html confirms the rename (title "... Gebäudemodernisierungsgesetz - GModG", changed by Art. 4 of the act of 2026-07-23, BGBl. 2026 I Nr. 226, heading change effective 2026-07-29). Every shown "GEG" is now "GModG"; "vormals GEG" only in two sentences. § 14 still names DIN 4108-2:2013-02 and § 20 DIN V 18599:2018-09 (opened and read), so both stay as registry entries with status `superseded` that may only appear next to GModG. Paragraph numbers on screen are limited to those whose headings were checked (14, 15, 16, 20, 22, 60c, 71, 79-82, 84, 86, 87, Anlagen 1, 4, 10); "Abs." numbers and "Anlage 4 Nr. ..." were dropped.
- **Willems/Schild/Stricker**: the audit mixed up two books. ISBN 978-3-658-44973-5 (shown) is "Formeln und Tabellen Bauphysik", 8. Aufl. 2024 (Springer Vieweg); 978-3-658-44971-1 is "Praxisbeispiele Bauphysik". The ISBN stays; the registry records the correction. The Bohne title now equals the publisher's title.
- **DIN V 4108-6:2003-06** (withdrawn, kept for the notation as the reviewer wants): chip "DIN V 4108-6:2003-06 (zurückgezogen) – Symbolik; aktuell: DIN/TS 18599" once per video where the balance starts: heating `🧮️calculation/Scene2`, cooling `⚖️heating-comparison/Beat1_WinterGains`, demand `🔁️recap/Beat1_Bilanzgrenze` (physics and ventilation never cite it). Frames: `shots/cool-status-de.png`, `shots/recap-status-de.png`. Unverifiable clause numbers of 4108-6 became keyword locators. DIN V 4701-10 is replaced on screen by DIN/TS 18599-5:2025-10.
- Editions now shown: DIN 4108-2:2026-05 (Abschn. 8 verified, `Tab. 7` dropped), DIN EN ISO 13789:2018-04 (the old on-screen 2017-10 does not exist), DIN EN 16798-3:2026-10, DIN EN 410:2011-04, DIN EN 14825:2023-10, DIN EN ISO 13786:2018-04, VDI 6022 Blatt 1:2018-01, VDI 3807 Blatt 1:2013-06, DIN EN 673:2025-01 and the rest in the table below; "VDI 2078 revision 2027" removed.
- Registry plain-text locators no longer contain symbol letters (`F_C`, `F_Gt`, `e_p`, `R_si` were literal underscores on screen): `🏗️authoring/📚️references/🔣️.json` lines 11, 51, 61, 182.

## 5. Parentheses

Rendered `(seit 7/2026 GModG)`, `(20 °C)`, `(0 °C)`, `[Vergleich]` and `( ) [ ]` in `body_text` at 1920x1080 (`TK/🗑️generated/consistency-out/frames/m/ParenStill-de-1920x1080/i/ParenStill_ManimCE_v0.22.0.png`, 2x crop `paren-zoom.png`, font comparison `paren-fonts.png`). Anta's `(` is a rounded square and differs from `[` only by the corner radius, which disappears at video size: it reads like `[`. Decision: fix at the font layer, not by avoiding parentheses. `🏗️authoring/🔤️fonts/🐍️.py` (`_DISTINCT_PARENTHESES`, `_glyph_fallbacks`): when the requested face is the body face (Anta), `(` and `)` are drawn from the bundled Kelly Slab face (true round arcs, same style family). After: `paren-after.png` shows clearly round parentheses beside clearly square brackets; real scenes: `shots/cool-status-de.png` (`(zurückgezogen)`), the physics poster. Tests: `🔤️fonts/🧪️tests/🔣️.cases.json` `required_symbols` now starts with `()[]` and a new `distinct_parentheses` case; new test `test_parentheses_of_the_body_face_are_the_distinct_round_glyphs` compares the rendered outlines of `(` `)` in a body text run with the Kelly Slab source outlines (14 font tests pass). TeX math keeps its own parentheses.

## 6. German wording

Appendix B (`Kühltechnik`, `Grenzwert nach Pettenkofer`). Kept on purpose: the video title "Kühlung", "freie/sorptionsgestützte Kühlung", cooling as process, the reviewer's sentence in `🌬️ventilation/🎬️clip/❓️need/🐍️.py:237`, roadmap hand-over chips (the roadmap test requires the partner video's name). The `lights up` clause survives only in a historical `previous` block of a caption-regression fixture; the shipped narration already described the physics.

## 7. Verification

- Narration guard: `NARRATION_GUARD_FILTER=<video> bash TK/narration-guard.sh` for physics, ventilation, heating, cooling, demand: OK (3 tests each).
- Family tests (`TK/consistency/family-tests.sh`, 26 modules, profile `final`, language `en`): **175 tests OK**; after the last edits (registry locators, heating calculation, running example) a final run of references, running example, roadmap, narration guard, speech lexicon and demand balance: 50 tests OK. The first run showed errors that were an environment issue of my runner (no `SEMIO_TEACHING_PROFILE`), not code.
- Real bug found by rendering and fixed: four scene files used `_references` without defining it (`🪟️natural`, `⚖️regulations`, `⚡️demand-power`, `🔁️recap`): every render of those chapters would have crashed with `NameError`, no unit test constructs those scenes. The import was added after `_morph` in each file. A caption-width check also caught two over-long clauses (3 lines) that were shortened.
- Spot-check frames (direct Manim stop-frame renders at 1280x720, German, preview profile; `TK/consistency/stopframe.sh`; images in `TK/🗑️generated/consistency-out/shots/`): `recap-weitere-de` (extended solar formula row), `power-day-de` (design day + example winter day clause; the first version clipped at the right edge and was split into two lines), `physics-gain-de` (0,60 / 1,92 m² / 576 W), `heat-driver-de` and `heat-recap-de` (Beispiel-Wintertag label), `vent-delta-de` (chip), `cool-igains-de` (`Q̇_{i,p/e/l}`), `heat-envelope-de` (`ΣΦ`, `Q̇_T`), `vent-second-de` (nested fractions), `heat-gradtag-de` (found the `G_{t20/15}` leftover), `cool-status-de`, `recap-status-de`, `power-sum-de` (fractions in the design-load formula), plus the parentheses stills. Nothing overlaps or leaves the frame in these.

## 8. Posters (item 7)

`script.ts posters --language de --profile preview --audience speaker --video <id>` (via `repo tools exec --light bun`, script `TK/consistency/posters.sh`) for physical_fundamentals, ventilation, heating, cooling, energy_balance: the stale posters were recaptured (ventilation 6, heating 6, cooling 6, energy_balance 7 chapters; physical_fundamentals 1 after one retry, because the first attempt aborted with "Authored sources changed while rendering" while another agent edited). A second run reports all 26 energy posters `current`, none stale. Stored assets are in `<lesson clip folder>/🖼️posters/`; not committed (repo rule).

## 9. Open points

- Style: loads are upright in heating and cooling, italic in the demand scenes (reads correctly, the glyph differs).
- Not re-verified against the new law text: legal values in the quizzes (55 %/70 % limits, Anlage 1 values); see appendix A.
- Unverified: DIN 1946-4 A1:2025-11, DIN 18380 status, VDI 4645 successor; the quiz figures from SDES, Fraunhofer ISE, DIN 1946-4, DIN 18032-1, ISO 14644-4 and DIN 51603-1 have verified editions but unverified numbers; DWD TRY only via a secondary page; IEST-RP-CC012 and DIN 4108-3 were dropped from the screen because their edition could not be verified.
- Layout candidates from appendices B and C that were not rendered: the `t_VL` panel in demand power, the certificate and primary-energy `UNIT` labels, the heating calculation readouts.
- `🔊️speech/🗣️lexicon/🔣️.json` mentions GEG (speech agent's file, outside the registry scan).
- New narration clause `beispieltag` (demand power) and changed clause texts need fresh speech/timing at the next real render; no voice-over was regenerated here.
'''


def edition(entry: dict) -> str:
    if entry["kind"] in ("standard", "guideline"):
        return f"{entry['designation']}:{entry['edition']}"
    if entry["kind"] == "law":
        return f"{entry['designation']} (Stand {entry['version']})"
    return f"{entry['designation']} ({entry.get('year', '')})"


rows = ["| id | designation and edition | status | verified | URL | note and what stayed unverified |", "|---|---|---|---|---|---|"]
for entry in entries:
    note = str(entry.get("note", "")).replace("|", "/")
    successor = " -> " + entry["successor"] if entry.get("successor") else ""
    rows.append(f"| {entry['id']} | {edition(entry)} | {entry['status']}{successor} | {entry.get('verified', '')} | {entry.get('url', '')} | {note} |")

parts = [HEAD, "\n## Registry table (generated from `📚️references/🔣️.json`)\n\n" + "\n".join(rows) + "\n"]
for name, title in (("refs", "Appendix A: references agent fragment"), ("symbols", "Appendix B: symbols and wording fragment"), ("units", "Appendix C: units fragment")):
    parts.append(f"\n---\n\n## {title}\n\n" + (HERE / name / "FRAGMENT.md").read_text(encoding="utf-8").replace("\n# ", "\n### "))
(HERE / "REPORT.md").write_text("".join(parts), encoding="utf-8")
print("written", len(entries), "entries")
