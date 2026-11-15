# Consistency pass over the five energy videos (agent `consistency`)

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

## Registry table (generated from `📚️references/🔣️.json`)

| id | designation and edition | status | verified | URL | note and what stayed unverified |
|---|---|---|---|---|---|
| din-4108-2 | DIN 4108-2:2026-05 | current | 2026-11-13 | https://www.dinmedia.de/en/standard/din-4108-2/401017482 | Published 2026-05, replaces DIN 4108-2:2013-02; the GModG paragraph 14 still names 2013-02 (checked on gesetze-im-internet.de). Section 8 is the summer heat protection chapter of both editions; table numbers are not cited because the 2026-05 text was not readable. |
| din-4108-2-2013 | DIN 4108-2:2013-02 | superseded -> din-4108-2 | 2026-11-13 | https://www.gesetze-im-internet.de/geg/__14.html | Superseded by 2026-05, but still the edition the GModG names in paragraph 14 (summer heat protection). |
| din-4108-beiblatt-2 | DIN 4108 Beiblatt 2:2019-06 | current | 2026-11-13 | https://www.dinmedia.de/en/technical-rule/din-4108-beiblatt-2/305990077 | A Berichtigung 1:2025-02 exists; a 2023 date seen on a secondary page could not be confirmed. |
| din-v-4108-6 | DIN V 4108-6:2003-06 | withdrawn -> din-ts-18599 | 2026-11-13 | https://www.dinmedia.de/en/pre-standard/din-v-4108-6/63939447 | Withdrawn; DIN names DIN/TS 18599-12:2021-04 as replacement and recommends the DIN V 18599 series. Kept on screen for the symbol notation (Q_h, Q_l, Q_g, eta) and the heating-period method. Clause numbers could not be checked against the withdrawn text, so only keyword locators are cited. |
| din-v-4701-10 | DIN V 4701-10:2003-08 | withdrawn -> din-ts-18599-5 | 2026-11-13 | https://dinmedia.de/en/pre-standard/din-v-4701-10/64443135 | Withdrawn; DIN recommends DIN V 18599-5, -6, -8 (now DIN/TS 18599 series). |
| din-ts-18599 | DIN/TS 18599:2025-10 | current | 2026-11-13 | https://www.dinmedia.de/en/pre-standard/din-ts-18599-1/389750965 | Series of pre-standards (Vornormen) published 2025-10; DIN states the DIN/TS series replaced the DIN V 18599 series of 2018-09. Series level entry; parts below. |
| din-ts-18599-1 | DIN/TS 18599-1:2025-10 | current | 2026-11-13 | https://www.dinmedia.de/en/pre-standard/din-ts-18599-1/389750965 | Replaces DIN V 18599-1:2018-09 (DIN Media). |
| din-ts-18599-2 | DIN/TS 18599-2:2025-10 | current | 2026-11-13 | https://www.dinmedia.de/en/pre-standard/din-ts-18599-2/389744117 | Replaces DIN V 18599-2:2018-09 (DIN Media). |
| din-ts-18599-5 | DIN/TS 18599-5:2025-10 | current | 2026-11-13 | https://www.dinmedia.de/en/pre-standard/din-ts-18599-5/389786380 | Replaces DIN V 18599-5:2018-09; clause numbers of the 2018 edition are not cited. |
| din-ts-18599-10 | DIN/TS 18599-10:2025-10 | current | 2026-11-13 | https://www.dinmedia.de/en/pre-standard/din-ts-18599-10/389788970 | Replaces DIN V 18599-10:2018-09 (DIN Media); the usage profiles table is cited by keyword only. |
| din-v-18599 | DIN V 18599:2018-09 | superseded -> din-ts-18599 | 2026-11-13 | https://www.gesetze-im-internet.de/geg/__20.html | Superseded by DIN/TS 18599:2025-10, but still the edition GModG paragraph 20 names (Abs. 1: DIN V 18599:2018-09). |
| din-v-18599-1 | DIN V 18599-1:2018-09 | superseded -> din-ts-18599-1 | 2026-11-13 | https://www.gesetze-im-internet.de/geg/__20.html | Superseded by DIN/TS 18599-1:2025-10; GModG paragraph 20 (Abs. 4, 5) still names this edition. |
| din-v-18599-2 | DIN V 18599-2:2018-09 | superseded -> din-ts-18599-2 | 2026-11-13 | https://www.gesetze-im-internet.de/geg/__20.html | Superseded by DIN/TS 18599-2:2025-10; GModG paragraph 20 (Abs. 6) still names this edition. |
| din-v-18599-10 | DIN V 18599-10:2018-09 | superseded -> din-ts-18599-10 | 2026-11-13 | https://www.gesetze-im-internet.de/geg/__20.html | Superseded by DIN/TS 18599-10:2025-10 (DIN Media); GModG paragraph 20 (Abs. 2) still names this edition. |
| din-en-12831-1 | DIN EN 12831-1:2017-09 | current | 2026-11-13 | https://www.dinmedia.de/de/norm/din-en-12831-1/261292587 | Replaces DIN EN 12831:2003-08. National annex: DIN/TS 12831-1:2020-04. |
| din-ts-12831-1 | DIN/TS 12831-1:2020-04 | current | 2026-11-13 | https://www.dinmedia.de/de/vornorm/din-ts-12831-1/316645651 | National addition (Vornorm) to the 2017-09 edition of EN 12831-1; replaces the Beiblätter 1 to 3. |
| din-en-iso-13789 | DIN EN ISO 13789:2018-04 | current | 2026-11-13 | https://www.dinmedia.de/de/norm/din-en-iso-13789/259116312 | Replaces DIN EN ISO 13789:2008-04; a 2017-10 edition does not exist (audit correction). |
| din-en-iso-6946 | DIN EN ISO 6946:2018-03 | current | 2026-11-13 | https://www.dinmedia.de/de/norm/din-en-iso-6946/255315365 | Berichtigung 1:2023-04 exists. Table numbers are not cited (not verifiable); the surface resistances are cited by keyword. |
| din-en-iso-13786 | DIN EN ISO 13786:2018-04 | current | 2026-11-13 | https://www.dinmedia.de/de/norm/din-en-iso-13786/255912584 | Replaces DIN EN ISO 13786:2008-04. |
| din-en-410 | DIN EN 410:2011-04 | current | 2026-11-13 | https://www.dinmedia.de/de/standard/din-en-410/138424295 | Replaces DIN EN 410:1998-12. |
| din-en-673 | DIN EN 673:2025-01 | current | 2026-11-13 | https://www.dinmedia.de/de/norm/din-en-673/385544906 | Replaces DIN EN 673:2011-04 (German version of EN 673:2024); an amendment draft prA1:2026 exists. |
| din-en-14825 | DIN EN 14825:2023-10 | current | 2026-11-13 | https://www.dinmedia.de/de/norm/din-en-14825/343036621 | Replaces DIN EN 14825:2019-07; defines SCOP and SEER. |
| din-en-14511-2 | DIN EN 14511-2:2023-08 | current | 2026-11-13 | https://www.dinmedia.de/de/norm/din-en-14511-2/349069586 | Edition 2023-08 taken from the replacement notes of the DIN pages of parts 1 and 2; part 1 is DIN EN 14511-1:2023-08. |
| din-en-16798-1 | DIN EN 16798-1:2022-03 | current | 2026-11-13 | https://www.dinmedia.de/de/norm/din-en-16798-1/349622591 | Replaces DIN EN 16798-1:2021-04. Annex B (informative) holds the design criteria and categories; table numbers are not cited. |
| din-en-16798-3 | DIN EN 16798-3:2026-10 | current | 2026-11-13 | https://www.dinmedia.de/en/standard/din-en-16798-3/390958579 | Published 2026-10 (German version of EN 16798-3:2025), replaces the withdrawn DIN EN 16798-3:2017-11. |
| din-en-16798-7 | DIN EN 16798-7:2017-11 | current | 2026-11-13 | https://www.dinmedia.de/en/standard/din-en-16798-7/258797864 | Replaces DIN EN 15242:2007-09; the discharge coefficient 0.6 on screen is an assumption, not a clause value. |
| din-en-308 | DIN EN 308:2022-04 | current | 2026-11-13 | https://www.din.de/en/wdc-beuth:din21:353762417 | Edition 2022-04 replaces EN 308:1997; German title taken from the 1997 edition. |
| din-1946-4 | DIN 1946-4:2018-09 | current | 2026-11-13 | https://www.din.de/en/wdc-beuth:din21:294438653 | An amendment A1 (2025-11) was reported but could not be confirmed. |
| din-1946-6 | DIN 1946-6:2019-12 | current | 2026-11-13 | https://www.din.de/de/mitwirken/normenausschuesse/nhrs/veroeffentlichungen/wdc-beuth:din21:383907328/toc-3571455/download | Edition verified indirectly: DIN 1946-6 Beiblatt 1:2025-06 names 2019-12 as its base. A 2022-09 edition claimed on a forum could not be confirmed. |
| din-1946-7 | DIN 1946-7:2022-08 | current | 2026-11-13 | https://www.dinmedia.de/de/norm/din-1946-7/355461283 | Replaces DIN 1946-7:2009-07. |
| din-18032-1 | DIN 18032-1:2014-11 | current | 2026-11-13 | https://www.dinmedia.de/de/norm/din-18032-1/222157679 | Replaces DIN 18032-1:2003-09; a new draft (2026) exists. |
| din-18380 | DIN 18380:2019-09 | current | 2026-11-13 | https://www.dinmedia.de/de/norm/din-18380/310840113 | Latest edition found is 2019-09 (replaces 2016-09); no 2023 edition found. Current status not confirmed on the DIN page itself. |
| din-51603-1 | DIN 51603-1:2024-11 | current | 2026-11-13 | https://www.dinmedia.de/en/standard/din-51603-1/382208921 | Replaces DIN 51603-1:2020-09. |
| vdi-4650-1 | VDI 4650 Blatt 1:2024-02 | current | 2026-11-13 | https://vdi.de/en/home/vdi-standards/details/vdi-4650-blatt-1-calculation-of-the-seasonal-coefficient-of-performance-of-heat-pumps-electric-heat-pumps-for-space-heating-and-domestic-hot-water | The term Jahresarbeitszahl (JAZ) is the guideline title; SCOP belongs to DIN EN 14825. |
| vdi-2078 | VDI 2078:2015-06 | current | 2026-11-13 | https://vdi.de/en/home/vdi-standards/details/vdi-2078-calculation-of-thermal-loads-and-room-temperatures-design-cooling-load-and-annual-simulation | 2015-06 is the current edition; no revision date was found (a claim of a revision in 2027 was removed from the screens). Locators are keywords, not clause numbers. |
| vdi-4645 | VDI 4645:2023-04 | current | 2026-11-13 | https://www.dinmedia.de/de/technische-regel/vdi-4645/364873293 | 2023-04 replaces 2018-03; a newer draft or Blatt 2:2026-01 (training) was seen, no newer main edition confirmed. |
| vdi-6022-1 | VDI 6022 Blatt 1:2018-01 | current | 2026-11-13 | https://vdi.de/richtlinien/details/vdi-6022-blatt-1-ventilation-and-indoor-air-quality-hygiene-requirements-for-ventilation-and-air-conditioning-systems-and-units-vdi-ventilation-code-of-practice-1 | Berichtigung 2023-04 exists. |
| vdi-3807-1 | VDI 3807 Blatt 1:2013-06 | current | 2026-11-13 | https://vdi.de/en/home/vdi-standards/details/vdi-3807-blatt-1-verbrauchskennwerte-fuer-gebaeude-grundlagen | Replaces VDI 3807 Blatt 1:2007-03. The G20/15 reference temperature locator could not be read and is cited as keyword. |
| vdi-2052-1 | VDI 2052 Blatt 1:2017-04 | current | 2026-11-13 | https://vdi.de/richtlinien/details/vdi-2052-blatt-1-air-conditioning-kitchens-vdi-ventilation-code-of-practice | Replaces VDI 2052:2006-04. |
| vdi-2087 | VDI 2087:2006-12 | current | 2026-11-13 | https://vdi.de/en/home/vdi-standards/details/vdi-2087-air-ducts-operating-and-construction-fundamentals | Berichtigung 2008-04 exists; German title translated from the official English title. |
| iso-14644-4 | ISO 14644-4:2022 | current | 2026-11-13 | https://www.dinmedia.de/en/standard/iso-14644-4/362298406 |  |
| iso-7730 | ISO 7730:2025 | current | 2026-11-13 | https://www.iso.org/standard/85803.html | Fourth edition 2025-09, withdraws ISO 7730:2005. |
| iso-8996 | ISO 8996:2021 | current | 2026-11-13 | https://www.iso.org/standard/74443.html | Third edition, December 2021. |
| iso-52016-1 | ISO 52016-1:2017 | current | 2026-11-13 | https://www.iso.org/standard/65696.html | Reviewed and confirmed 2022 (ISO page). Source of the F_sh notation that the videos do not use. |
| gmodg | GModG (Stand 2026-07-29) | current | 2026-11-13 | https://www.gesetze-im-internet.de/geg/BJNR172810020.html | Official source gesetze-im-internet.de (path /geg/) shows the title renamed by Art. 1 Nr. 1 of the act of 2026-07-23, BGBl. 2026 I Nr. 226, in force 2026-07-29. Paragraph headings checked in the table of contents: 14, 15, 16, 20, 22, 60c, 71 (weggefallen), 79 to 82, 84, 86, 87, Anlagen 1, 4, 10. The current text of paragraph 14 still names DIN 4108-2:2013-02 and paragraph 20 still names DIN V 18599:2018-09. |
| bohne-2026 | Bohne/Bohne (2026) | current | 2026-11-13 | https://www.baunetzwissen.de/gebaeudetechnik/tipps/buecher/technischer-ausbau-von-gebaeuden-4347729 | 13th updated edition, 2026 (ISBN of the print edition); title as given by the publisher listing on Baunetzwissen. |
| willems-2024 | Willems/Schild/Stricker (2024) | current | 2026-11-13 | https://www.jpc.de/jpcng/books/detail/-/art/wolfgang-m-willems-formeln-und-tabellen-bauphysik/hnum/11834083 | 8th edition, 2024-08-31. The ISBN 978-3-658-44973-5 belongs to Formeln und Tabellen Bauphysik (print); 978-3-658-44971-1 belongs to Praxisbeispiele Bauphysik. The audit mixed the two books up. |
| maas-1995 | Maas (1995) | current | 2026-11-13 | https://www.uni-kassel.de/fb6/bpy/de/forschung/abgeschlprojekte/pdfs/maas_diss.pdf | Dissertation submitted 1995-06-27. Chapter 8 Luftwechsel und Lüftungswärmeverluste in der Heizzeit, p. 73-81; p. 74 gives 0.56 to 1.38 1/h for a permanently tilted window. |
| pettenkofer-1858 | Pettenkofer (1858) | current | 2026-11-13 | https://lessing.ub.uni-duesseldorf.de/download/pdf/2385693.pdf | Title page: München, Cotta, 1858 (one bibliography says Braunschweig). |
| adhoc-ag-2008 | Ad-hoc-AG Innenraumrichtwerte (2008) | current | 2026-11-13 | https://link.springer.com/content/pdf/10.1007/s00103-008-0707-2.pdf | Published 2008-11-29; the 1000 ppm value is the hygienic guide value of this article. |
| iwu-typologie-2015 | IWU Wohngebäudetypologie (2015) | current | 2026-11-13 | https://iwu.de/fileadmin/publikationen/gebaeudebestand/episcope/2015_IWU_LogaEtAl_Deutsche-Wohngebäudetypologie.pdf | 2nd extended edition of 2015-02-10 (TABULA, EPISCOPE); no later edition found. |
| sunikka-galvin-2012 | Sunikka-Blank/Galvin (2012) | current | 2026-11-13 | https://structurae.net/en/literature/journal-article/introducing-the-prebound-effect-the-gap-between-performance-and-actual-energy-consumption | Volume 40, issue 3, pp. 260-273 (2012), DOI 10.1080/09613218.2012.690952 (structurae and publisher listing). |
| sdes-bilan-2023 | SDES (2023) | current | 2026-11-13 | https://www.statistiques.developpement-durable.gouv.fr/bilan-energetique-de-la-france-pour-2023 | Official SDES page, published 2025-04-10; the primary and final energy figures of the quiz were not re-read against the tables. |
| fraunhofer-ise-pv-fakten | Fraunhofer ISE (2026) | current | 2026-11-13 | https://www.ise.fraunhofer.de/de/veroeffentlichungen/studien/aktuelle-fakten-zur-photovoltaik-in-deutschland.html | Living document, version of 2026-05-05 on the publisher page; the yield value of the quiz was not re-read in the PDF. |
| dwd-try-2017 | DWD TRY (2017) | current | 2026-11-13 | https://www.baulinks.de/webplugin/2017/0983.php4 | Hourly test reference years 2017, funded by BBSR (Zukunft Bau); the page of the DWD itself could not be retrieved. |

---

## Appendix A: references agent fragment

# Fragment: one reference registry for the five energy videos (agent `consistency-refs`)

Date of all checks: 2026-11-13. ENERGY = `🎓️teaching/🏛️architecture/⚡️energy`.

## 1. What was built

Module `ENERGY/🏗️authoring/📚️references/` (layout of `📏️running-example`):

| File | Role |
|---|---|
| `🔣️.json` | data: 55 entries (standards, guidelines, law, books, article, thesis, data sets) with id, kind, designation, edition/version/year, title de+en, status, successor, `url`, `verified`, per-use `locators` |
| `🧬️schema/🔣️.schema.json` | JSON Schema 2020-12 (kind-dependent required fields; withdrawn/superseded require a successor) |
| `🐍️.py` | pure-stdlib API: `designation(id)` ("DIN 4108-2:2026-05"), `short(id)`, `ref(id, locator, *, language)`, `chip(*parts, language)` (parts: id, `(id, locator)`, `!id` = status chip; wraps to two lines above 72 characters), `cite(*parts)` (language from `SEMIO_TEACHING_LANGUAGE`, same contract as `visuals.language`), `status_chip(id, language)`, `citation`, `bibliography(id, language)`, `card(id, language)` (lines of a bibliography card), `all_forms()` |
| `🧪️tests/🐍️.py` + `🧪️tests/🔣️.cases.json` | tests; registered in `🏗️authoring/📜️script.ts` `test` list |

Tests (10): (a) schema validation with third-party `jsonschema`; (b) every entry has edition / year / version and `url` and `verified`; unique ids and designations; (c) every withdrawn/superseded entry names its status word and a current successor in the status chip (de + en), required withdrawn/current ids listed in the cases JSON; (d) language-agnostic API cases (23 cases in the cases JSON: designation, short, ref de+en, chip, status chip de+en, bibliography, citation, card); (e) scan rules with must-flag and must-pass strings in the cases JSON; (f) the source scan `Screens.test_every_shown_designation_is_a_registry_form`: every string literal (AST, docstrings and comments excluded) of all `.py` under the five videos, `🏗️authoring/{🎨️visuals,🎬️clip,📝️narration,🗺️roadmap,📏️running-example,🔤️fonts,🎨️theme}`, every JSON string (quiz, fixtures, manifest) and every line of README/concepts md is matched with the designation regex (DIN, DIN EN, DIN EN ISO, DIN V, DIN/TS, DIN SPEC, VDI, ISO, EN, IEST-RP-CC) and fails when the text is not in `all_forms()`, when a designation has no edition, when a withdrawn/superseded edition is shown without a status word or `GModG` context, when `GEG` appears outside "vormals GEG"/"formerly GEG", when an author name (Willems, Bohne, Maas, Pettenkofer, Bundesgesundheitsbl, IWU typology) appears outside its registry citation, or when a dropped claim (`Tab. 7`, `Tab. B.2`, `Anh. B.4`, "revision 2027") survives next to a designation. `🔊️speech/**` is not scanned (not mine to edit).

Declared allow rule (cases JSON `shortAllowedIn` + `shortRuleJustification`, kept minimal): the short form without edition is allowed only inside narration clauses `(key, en, de)` (spoken and subtitled; editions are not read aloud and the beat's chip shows them), plus the registry's own status chip, which names the successor by short form. Nothing else.

## 2. Registry table

Verified = page opened or search result of the named URL on 2026-11-13. "indirect" = edition read from a DIN page that replaces/names it.

| id | designation + edition | status | URL (host) | verified how / not verified, decision |
|---|---|---|---|---|
| din-4108-2 | DIN 4108-2:2026-05 | current | dinmedia.de/en/standard/din-4108-2/401017482 | page opened: published 2026-05, replaces 2013-02. Only "Abschn. 8" kept; "Tab. 7" dropped (2026-05 text unreadable) |
| din-4108-2-2013 | DIN 4108-2:2013-02 | superseded -> 2026-05 | gesetze-im-internet.de/geg/__14.html | GModG § 14 still names "DIN 4108-2: 2013-02" (Abs. 2: Abschn. 8, Abs. 3: 8.4, Abs. 5: 8.2.2). Allowed on screen only with GModG context |
| din-4108-beiblatt-2 | DIN 4108 Beiblatt 2:2019-06 | current | dinmedia.de/en/technical-rule/din-4108-beiblatt-2/305990077 | search result; Berichtigung 1:2025-02; a "2023" date on a secondary page not confirmed |
| din-v-4108-6 | DIN V 4108-6:2003-06 | withdrawn -> DIN/TS 18599 | dinmedia.de/en/pre-standard/din-v-4108-6/63939447 | page opened: "Document withdrawn", replaced by DIN/TS 18599-12:2021-04, DIN recommends the DIN V 18599 series. Kept (`shownWithoutStatus`) for the symbol notation by the reviewer's wish. **All clause numbers (3.2, 5.5, 6.1-6.5, Anh. A, B.4, D.2, D.5) dropped**, replaced by keyword locators (symbols, losses, gains, utilisation factor, climate data, heating-period method) |
| din-v-4701-10 | DIN V 4701-10:2003-08 | withdrawn -> DIN/TS 18599-5 | dinmedia.de/en/pre-standard/din-v-4701-10/64443135 | withdrawn; replaced on screen by DIN/TS 18599-5 (+ -1 Endenergie) |
| din-ts-18599 | DIN/TS 18599:2025-10 (series) | current | dinmedia.de/en/pre-standard/din-ts-18599-1/389750965 | DIN states the 2025-10 series replaced the DIN V series |
| din-ts-18599-1 / -2 / -5 / -10 | DIN/TS 18599-1/-2/-5/-10:2025-10 | current (Vornorm) | dinmedia.de (389750965, 389744117, 389786380, 389788970) | search result + page for -10: each replaces DIN V 18599-n:2018-09. Part -6 and -8 (named in old READMEs) not on screen, not registered. 2018 clause numbers (5.5.3, 6.2 ...) dropped |
| din-v-18599, -1, -2, -10 | DIN V 18599(-n):2018-09 | superseded -> TS | gesetze-im-internet.de/geg/__20.html | GModG § 20 still names DIN V 18599:2018-09 (Abs. 1), -10 (Abs. 2), -1 (Abs. 4, 5), -2 (Abs. 6). Shown only with GModG context |
| din-en-12831-1 | DIN EN 12831-1:2017-09 | current | dinmedia.de/de/norm/din-en-12831-1/261292587 | page opened; "Abschn. 6 = detailed method" verified indirectly through the DIN/TS 12831-1 description |
| din-ts-12831-1 | DIN/TS 12831-1:2020-04 | current (national addition) | dinmedia.de/de/vornorm/din-ts-12831-1/316645651 | search result of the DIN page |
| din-en-iso-13789 | DIN EN ISO 13789:2018-04 | current | dinmedia.de/de/norm/din-en-iso-13789/259116312 | search: replaces 2008-04; "2017-10" of the old scene does not exist (corrected) |
| din-en-iso-6946 | DIN EN ISO 6946:2018-03 | current | dinmedia.de/de/norm/din-en-iso-6946/255315365 | page opened; Berichtigung 2023-04; "Tab. 7" dropped, keyword locator for R_si/R_se |
| din-en-iso-13786 | DIN EN ISO 13786:2018-04 | current | dinmedia.de/de/norm/din-en-iso-13786/255912584 | page opened |
| din-en-410 | DIN EN 410:2011-04 | current | dinmedia.de/de/standard/din-en-410/138424295 | page opened |
| din-en-673 | DIN EN 673:2025-01 | current | dinmedia.de/de/norm/din-en-673/385544906 | search result (German version of EN 673:2024, replaces 2011-04); was "EN 673" in a heating quiz |
| din-en-14825 | DIN EN 14825:2023-10 | current | dinmedia.de/de/norm/din-en-14825/343036621 | page opened |
| din-en-14511-2 | DIN EN 14511-2:2023-08 | current | dinmedia.de/de/norm/din-en-14511-2/349069586 | part 1 page opened (2023-08); part 2 edition from replacement notes. Bare "DIN EN 14511" (COP) now cites part 2 (test conditions) |
| din-en-16798-1 | DIN EN 16798-1:2022-03 | current | dinmedia.de/de/norm/din-en-16798-1/349622591 | search result; "Tab. B.2" dropped, Annex B kept |
| din-en-16798-3 | DIN EN 16798-3:2026-10 | current | dinmedia.de/en/standard/din-en-16798-3/390958579 | page opened: published 2026-10, 2017-11 withdrawn |
| din-en-16798-7 | DIN EN 16798-7:2017-11 | current | dinmedia.de/en/standard/din-en-16798-7/258797864 | page opened. C_d = 0.6 on screen is an assumption, not a clause |
| din-en-308 | DIN EN 308:2022-04 | current | din.de/en/wdc-beuth:din21:353762417 | search result (2022-04 replaces 1997); German title taken from 1997 |
| din-1946-4 | DIN 1946-4:2018-09 | current | din.de/en/wdc-beuth:din21:294438653 | audit URL; A1:2025-11 reported, not confirmable |
| din-1946-6 | DIN 1946-6:2019-12 | current | din.de (toc of DIN 1946-6 Beiblatt 1:2025-06) | indirect: Beiblatt 1:2025-06 names 2019-12 as base; forum claim "2022-09" not confirmed |
| din-1946-7 | DIN 1946-7:2022-08 | current | dinmedia.de/de/norm/din-1946-7/355461283 | search result: replaces 2009-07 |
| din-18032-1 | DIN 18032-1:2014-11 | current | dinmedia.de/de/norm/din-18032-1/222157679 | search: DIN lists 2014-11 as current (replaces 2003-09); a 2026 draft exists |
| din-18380 | DIN 18380:2019-09 | current (latest found) | dinmedia.de/de/norm/din-18380/310840113 | latest edition found; no 2023 edition; status not explicitly confirmed |
| din-51603-1 | DIN 51603-1:2024-11 | current | dinmedia.de/en/standard/din-51603-1/382208921 | search result: replaces 2020-09 |
| vdi-4650-1 | VDI 4650 Blatt 1:2024-02 | current | vdi.de (detail page) | audit-verified; JAZ is the title term, SCOP belongs to DIN EN 14825 |
| vdi-2078 | VDI 2078:2015-06 | current | vdi.de (detail page) | audit-verified; "revision announced for 2027" removed everywhere (no source) |
| vdi-4645 | VDI 4645:2023-04 | current | dinmedia.de/de/technische-regel/vdi-4645/364873293 | search: 2023-04 current (replaces 2018-03); a newer draft / Blatt 2:2026-01 (training) seen, no new main edition confirmed |
| vdi-6022-1 | VDI 6022 Blatt 1:2018-01 | current | vdi.de (detail page) | audit-verified; Berichtigung 2023-04 |
| vdi-3807-1 | VDI 3807 Blatt 1:2013-06 | current | vdi.de (detail page) | audit-verified; G20/15 locator kept as keyword |
| vdi-2052-1 | VDI 2052 Blatt 1:2017-04 | current | vdi.de (detail page) | search result (replaces 2006-04); bare "VDI 2052" now cites Blatt 1 |
| vdi-2087 | VDI 2087:2006-12 | current | vdi.de (detail page) | search result; Berichtigung 2008-04 |
| iso-14644-4 | ISO 14644-4:2022 | current | dinmedia.de/en/standard/iso-14644-4/362298406 | search result (2nd edition 2022); claim "0.2-0.5 m/s" in the quiz not verified in the text |
| iso-7730 | ISO 7730:2025 | current | iso.org/standard/85803.html | search: 4th edition 2025-09 withdraws 2005 |
| iso-8996 | ISO 8996:2021 | current | iso.org/standard/74443.html | search: 3rd edition 2021-12 |
| iso-52016-1 | ISO 52016-1:2017 | current | iso.org/standard/65696.html | search: confirmed 2022 |
| gmodg | GModG (Stand 2026-07-29) | current law | gesetze-im-internet.de/geg/BJNR172810020.html | OFFICIAL source opened: title "Gesetz zur Einsparung von Energie und zur Modernisierung der Wärmeversorgung in Gebäuden (Gebäudemodernisierungsgesetz - GModG)", changed by Art. 4 of the act of 2026-07-23, BGBl. 2026 I Nr. 226, heading changed with effect from 2026-07-29. Paragraph headings checked in the table of contents: 14, 15, 16, 20, 22, 60c, 71 (weggefallen, 71-73), 79-82, 84, 86, 87, Anlagen 1, 4, 10. §§ 14, 15, 20, 86 text opened. "Abs." numbers and "Anlage 4 Nr. 2, 8, 9, 12" **dropped** (not verified) |
| bohne-2026 | Bohne, Bohne (2026), 13. Aufl. | book | baunetzwissen.de/.../technischer-ausbau-von-gebaeuden-4347729 | page opened: title "Gebäudetechnik - Technischer Ausbau und nachhaltige Gebäudesysteme", authors Dirk and Marius Bohne, Springer Vieweg Heidelberg, ISBN 978-3-658-51570-6 (e-book -51571-3) |
| willems-2024 | Willems, Schild, Stricker (2024), 8. Aufl. | book | jpc.de/.../formeln-und-tabellen-bauphysik/hnum/11834083 | see decisions |
| maas-1995 | Maas (1995), Diss. Univ. Kassel, Kap. 8.2, S. 74 | thesis | uni-kassel.de/.../maas_diss.pdf | audit-verified (S. 74: 0.56-1.38 1/h) |
| pettenkofer-1858 | Pettenkofer (1858), München: Cotta | book | lessing.ub.uni-duesseldorf.de/.../2385693.pdf | audit-verified title page |
| adhoc-ag-2008 | Bundesgesundheitsbl. 51 (2008) 1358-1369 | article | link.springer.com/.../10.1007/s00103-008-0707-2.pdf | audit-verified, DOI |
| iwu-typologie-2015 | IWU Wohngebäudetypologie (2015), 2. Aufl. | data set | iwu.de/.../2015_IWU_LogaEtAl_Deutsche-Wohngebäudetypologie.pdf | search: 2nd extended edition 2015-02-10, ISBN 978-3-941140-47-9, no later edition found. "Tabelle 26" dropped |
| sunikka-galvin-2012 | Sunikka-Blank, Galvin (2012), Build. Res. Inf. 40(3) | article | structurae.net/... | search: 260-273, DOI 10.1080/09613218.2012.690952 |
| sdes-bilan-2023 | SDES, Bilan énergétique de la France pour 2023 | data set | statistiques.developpement-durable.gouv.fr | page found, figures of the quiz not re-read |
| fraunhofer-ise-pv-fakten | Fraunhofer ISE, Aktuelle Fakten zur Photovoltaik in Deutschland | data set | ise.fraunhofer.de | living document 2026-05-05, quiz value not re-read |
| dwd-try-2017 | DWD TRY 2017 | data set | baulinks.de/webplugin/2017/0983.php4 | DWD page not retrievable; secondary page |

**Not registered, claim dropped from screen:** IEST-RP-CC012 (current revision unverifiable; ventilation quiz items now say "typical guideline value" without a source), DIN 4108-3 (edition unverified; concepts dew point row loses the attribution and the "-3" in the physics row), the λ attribution "from DIN 4108" in the conduction scene (DIN 4108-4 would be the source, not verified), the label "(DIN 4108)" at the insulation layer, "Fraunhofer ISE heat pump field test" (no concrete source in the quiz, left as text), DIN V 18599-6/-8 (not on screen).

## 3. Decisions

- **GModG.** Confirmed on the official site: the law was renamed, the abbreviation is GModG, effective 2026-07-29 (BGBl. 2026 I Nr. 226). Every shown "GEG" is now "GModG" (quiz, README, scene labels); "GEG" survives only as "vormals GEG"/"formerly GEG" (regulations law card, primary-energy law note) and is enforced by the scan. "GModG 2024" in the quizzes (anachronism) became "GModG". The statutory text still names DIN 4108-2:2013-02 (§ 14) and DIN V 18599:2018-09 (§ 20), so the registry keeps both editions as `superseded` entries usable only with GModG context; scenes show the current editions (2026-05, DIN/TS 18599 2025-10) and the primary-energy scene says "GModG § 20 still cites DIN V 18599-1:2018-09". The content of the heating quiz about the legal requirements (e.g. 55 %/70 % H'_T values, Anlage 1 values) was renamed but **not re-verified against the new text** (open point).
- **Willems.** The audit mixed two books: ISBN 978-3-658-44973-5 (as on screen) belongs to "Formeln und Tabellen Bauphysik", 8. Aufl. 2024 (Springer Vieweg, 2024-08-31, e-book/DOI 10.1007/978-3-658-44974-2; confirmed by jpc listing and the DOI the scene already showed); 978-3-658-44971-1 is "Praxisbeispiele Bauphysik". The scene cites the former, so the ISBN stays and the registry records the correction. Original German titles are used in both languages for all German works (findability).
- **Bohne.** Title on screen is now exactly the publisher title "Gebäudetechnik - Technischer Ausbau und nachhaltige Gebäudesysteme" (card via `card("bohne-2026")`).
- **Withdrawn handling.** DIN V 4108-6:2003-06 stays for the notation; chip text "DIN V 4108-6:2003-06 (zurückgezogen) – Symbolik; aktuell: DIN/TS 18599" (en: "(withdrawn) – notation; current: DIN/TS 18599") is shown ONCE per video: heating `🧮️calculation/Scene2` (replaces the source line "Heating-period method of DIN V 4108-6 (Annex D.2)", two lines with VDI 3807 Blatt 1:2013-06); cooling `⚖️heating-comparison/Beat1_WinterGains` (second line below the DIN/TS 18599-2 chip); demand `🔁️recap/Beat1_Bilanzgrenze` (replaces the old `3.2 · 5.5` chip). Physics: no balance, none. Ventilation: no DIN V 4108-6 citation at all, none. DIN V 4701-10:2003-08 is replaced on screen by DIN/TS 18599-5:2025-10 (+ -1 Endenergie) in `♨️system-losses/Beat4`; the withdrawn marking lives in the registry and README.
- **Clause numbers.** Every unverified clause/table number was replaced by a verifiable keyword locator or dropped (see table). Verified as is: DIN 4108-2 Abschn. 8 (also named by GModG § 14), DIN EN 12831-1 Abschn. 6, Maas Kap. 8.2 S. 74, GModG paragraph headings.
- **Layout.** Chips longer than 72 characters wrap to a second line (`chip`); two-part chips such as GModG + DIN/TS fit. The visual check of every chip stays with the parent.

## 4. Changed files (file:line groups)

Chips now `_din_ref(_references.cite(...))`, `import` of `_references` added after `tr = _manim_module.tr` in each file (ENERGY-relative):
see `🗑️generated/consistency-out/refs/usage-lines.txt` for the exact lines; groups:
cooling: `☀️solar-radiation` 202,433; `⚖️heating-comparison` 274 (status chip),342,412; `🌤️summer-strategy` 172,305,502; `👥️internal-gains` 202,284,585,792,910; `💧️transmission-humidity` 233,384; `📐️system-sizing` 230,403.
ventilation: `⚙️mechanical` 280,454,736,943; `❓️need` 247 (Pettenkofer + Bundesgesundheitsbl. instead of DIN 1946-6),284 (label with "(1858)"),438; `🌡️heat-flow` 304,479,623,786; `📦️volume` 487,733,928; `🤝️handoff` 279,455; `🪟️natural` 346,591,592 (Maas, Kap. 8.2 S. 74),820,1032,1321,1558.
demand: `♨️system-losses` 586,774,1003; `⚖️regulations` 569 (law card),586,719,953,1140; `⚡️demand-power` 434,630,891,1087; `🏭️primary-energy` 416,421,422,627,955,1188; `🏷️certificate` 341,544,785; `🔁️recap` 468 (status chip),659,989,1273; `🧪️practice` 313,570,707,1069.
heating: `☀️solar-gains` 393,517,643,757,892,1122; `🌬️convection` 172; `👥️internal-gains` 336,367,438,542; `🧮️calculation` 640 (status chip); `🧱️conduction` 387,579,697,882,1044,1240 (+ narration "DIN 4108-2" at the minimum-resistance clause, "DIN 4108 Beiblatt 2", label "Insulation layer", λ clause without false attribution); `🧲️fundamentals` 634,744,921,1078,1158,1273,1502,1663.
physics `🧲️fundamentals`: 1920 (JAZ/SCOP label), 2137 (EN 410), 2266 (4108-2), 2600 (13786), 3183-3190 (bibliography cards via `card()`); narration `jaz_ref` "VDI 4650 Blatt 1" (the English clause said "part 1").
Prose strings (labels, narration, quiz, README, concepts, fixtures) rewritten by script to exact registry forms (non-narration: edition appended; narration: short form; DIN V 18599(-n) -> DIN/TS 18599(-n); DIN EN 12831 -> DIN EN 12831-1; EN 16798-1 -> DIN EN 16798-1:2022-03; VDI 4650-1 -> VDI 4650 Blatt 1; Bbl. 2 -> Beiblatt 2; clause numbers after DIN V 4108-6 removed; GEG -> GModG; IWU citation -> "IWU Wohngebäudetypologie (2015)", "Tabelle 26" removed): quiz JSON of all five videos, `📖️concepts/📝️.md`, the five `🎬️clip/README.md` (cooling, demand and ventilation README paragraphs rewritten by hand: references on screen, 2027 revision claim removed, Pettenkofer/Maas/Bundesgesundheitsbl. lines), `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json` (DIN 1946-6:2019-12), `🏗️authoring/🔤️fonts/🧪️tests/📑️heading/🔣️.json`, `📊️demand/🎬️clip/🔁️recap/🧪️tests/🧮️balance/🔣️.json`, `🌬️ventilation/🎬️clip/🧪️architecture/🔣️.numbers.json` (Maas source), narration in `📦️volume`, `👥️internal-gains` (heating), `⚖️heating-comparison`, `🧮️calculation`, `♨️system-losses`, `⚖️regulations`, `🏭️primary-energy`, `🧪️practice`.
New: `🏗️authoring/📚️references/**`; one line in `🏗️authoring/📜️script.ts` (test registration).
Scripts of this task: `TK/consistency/refs/{inventory,grep_lines,manifest_order,apply_chips,apply_text,fix_law_names}.py`, `run-tests.sh`, `family-tests.sh`.

## 5. Verification run

Narration guard clean. Passing (SEMIO_TEACHING_PROFILE=preview): references (10), visuals `scenes` (15), `labels` (5), fonts heading (3), demand recap balance / chain / primary-energy chain+scale, ventilation volume + architecture (numbers, transitions), physics fundamentals (13), heating conduction (2), cooling system-sizing (2). Visuals `integration`: 8 of 9 pass; `test_physics_scene_constructor_bounds` fails on the radiator staffage height (0.681 vs 0.738), unrelated to references (staffage geometry of another agent). No renders were run.

## 6. Open points

- The symbol agent may want to render `F_C` in the registry locator text `Abschn. 8 – Sonnenschutzvorrichtung F_C` (plain text, underscore shown literally); same for `R_si, R_se`, `e_p`.
- Legal facts quoted in the heating/cooling/demand quizzes (55 %/70 % H'_T limits, Anlage 1 values, "65-%-rule repealed", efficiency classes) were renamed to GModG but not re-read against the new text; GModG § 14/§ 20 text confirms only the DIN editions.
- Unverified claims left on screen as text: Fraunhofer ISE heat pump field test, SDES and Fraunhofer ISE quiz numbers (sources registered, figures not re-read), DIN 1946-4 values (60 m³/(h·m²), 20 1/h), DIN 18032-1 "60 m³/h per athlete", ISO 14644-4 "0.2-0.5 m/s", DIN 51603-1 calorific value; editions are verified, the numbers are not.
- DIN 1946-4:2018-09/A1:2025-11, DIN 18380 current status and VDI 4645 successor not confirmable.
- Long chips (law + standard) now wrap to two lines; visual check of overlap with the heading and the caption is for the parent.
- Not checked: `🔊️speech/🗣️lexicon/🔣️.json` mentions GEG (speech agent's file, outside the scan).

---

## Appendix B: symbols and wording fragment

# Fragment: symbols and German wording (agent consistency-symbols)

ENERGY = `🎓️teaching/🏛️architecture/⚡️energy`. All edits are in place (no copies); scripts and probes live in `consistency/symbols/` (`apply_I.py`, `apply_I2.py`, `apply_G.py`, `apply_concepts.py`, `scan.py`, `proof.py`, `speak.py`).

## A. Symbols

### A1. Internal gains: lower-case `i`, no `\mathrm{I}`
Rendering decision: the loads keep the family style of heating (already shipped as `\dot{Q}_{\mathrm{i}}`, `\dot{Q}_{\mathrm{i,p}}`): upright lower-case i inside `\mathrm{}`, so cooling, heating and ventilation draw the identical glyph and sit beside the upright `T V S C` indices. Parts: `i,p` persons, `i,e` equipment, `i,l` lighting, `i,lat` latent internal. README text uses the plain form `Q̇_i`.

| file | change |
|---|---|
| `❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py` 403, 404, 460 | `\dot{Q}_{\mathrm{I}}` -> `\dot{Q}_{\mathrm{i}}` |
| `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py` 265-266, 437, 517, 555-556, 567-568, 629, 723, 771-786, 822-824, 849-857, 901-902 | `\mathrm{I}`, `\mathrm{I,p}`, `\mathrm{I,e}`, `\mathrm{I,l}` -> `\mathrm{i}`, `\mathrm{i,p}`, `\mathrm{i,e}`, `\mathrm{i,l}` (also the f-string `{{ }}` forms) |
| `❄️cooling/🎬️clip/🗺️roadmap/🐍️.py` 38, 39, 57 | `\dot{Q}_{\mathrm{I}}` -> `\dot{Q}_{\mathrm{i}}` |
| `🌬️ventilation/🎬️clip/❓️need/🐍️.py` 286 | `\dot{Q}_{\mathrm{I,lat}}` -> `\dot{Q}_{\mathrm{i,lat}}` |
| `❄️cooling/🎬️clip/README.md` 42, 46 | `Q̇_I` -> `Q̇_i`; `\dot{Q}_\mathrm{I}` -> `\dot{Q}_i`; `I,p / I,e / I,l` -> `i,p / i,e / i,l` plus `i,lat` |
| `🔥️heating/🎬️clip/README.md` 47 | `Q̇_I` -> `Q̇_i` |
| `🔥️heating/🎬️clip/🗺️roadmap/🐍️.py` 44, 45, 60 | `\mathrm{I}` -> `\mathrm{i}` |
| `📊️demand/🎬️clip/🔁️recap/🐍️.py` | already `Q_i` / `Q_{i}` when this pass started (another agent), verified |
| concepts `📖️concepts/📝️.md` ~456-459 | `Q̇_Pers`, `Q̇_Geräte`, `Q̇_Licht` -> `Q̇_i,p`, `Q̇_i,e`, `Q̇_i,l` |

Not touched on purpose: Python identifiers (`THETA_I`, `Q_I_SPEC`), IWU type names `EFH_I` in quiz answers (a building type, not a symbol), irradiance `I_s`.
Open style point (not decided here): heating and cooling write all load/energy indices upright (`\mathrm{}`), the demand scenes write `Q_{h}`, `Q_{i}`, `\dot{Q}_H`, `\dot{Q}_C` italic; both read correctly, the glyph differs between families.

### A2. Speech lexicon
`🏗️authoring/🔊️speech/🗣️lexicon/🔣️.json`: new whole-symbol entry `G_{t,20/15}` -> "G t 20 15" (otherwise spoken "G t 20 durch 15", i.e. "20 divided by 15").
`🔊️speech/🧪️tests/🗣️lexicon/🔣️.cases.json` new cases: `\dot{Q}_{\mathrm{i}}`, `\dot{Q}_{\mathrm{i,p}}`, `\dot{Q}_{\mathrm{i,e}}`, `\dot{Q}_{\mathrm{i,l}}`, `\dot{Q}_{\mathrm{i,lat}}` ("Q-Punkt i p", "Q dot i latent", ...), `\Phi_{\mathrm{c,si}}` ("Phi c s i"), `G_{t,20/15}`, `e_{E}` ("e E"). Everything else (`\dot{Q}_{\mathrm{H}}`, `\dot{Q}_{\mathrm{C}}`, `F_{Gt}`, `e_{ce}`, `\Sigma\,\Phi`, `I_{s,max}`, `\Delta\theta_{eq}`) was already spoken by the generic composition (probe: `speak.py` + `speak-in.txt`, all fourteen probes speak in de and en).

### A3. Anlagenaufwandszahl `e_E`
Quantity check: `e_E = e_ce · e_d · e_s · e_g = Q_E / Q_h` (final energy per useful energy, DIN V 4701-10 chain; gas boiler 1,40, heat pump 0,31). `e_P` in `📊️demand/🎬️clip/🏭️primary-energy` is a different quantity, `Q_P / Q_h = e_E · f_P` (1,54 / 0,56), so it stays `e_P` but is no longer called "the plant expenditure figure": primary-energy `🐍️.py` 941-942 now "primary-energy expenditure figure / primärenergetische Aufwandszahl `e_P = e_E · f_P = Q_P/Q_h`". `e_g` is the generator stage factor (one of the four stage factors `e_ce e_d e_s e_g`), not the chain figure: kept.
Unified to `e_{E}`: `📊️demand/🎬️clip/🗺️roadmap/🐍️.py` 44, 45, 64 (`e_{\mathrm{E}}` -> `e_{E}`; the English roadmap line now says "plant expenditure figure"); comment in `♨️system-losses/🐍️.py` 108 (`e_p` -> `e_E`); test fixture key `e_p` -> `e_E` in `📊️demand/🎬️clip/🧪️tests/🔗️chain/🔣️.json` and `🐍️.py` 40-41. No `e_ges`/`e_tot`/`e_{\mathrm{E}}` variants remain.

### A4. Degree days `G_{t,20/15}` and `F_{Gt}`
`📊️demand/🎬️clip/⚖️regulations/🐍️.py` 97, 144, 1107-1108, 1131-1132, 1321; `📊️demand/🎬️clip/🔁️recap/🧪️tests/🧮️balance/🐍️.py` 73; `📊️demand/🎬️clip/🗺️roadmap/🐍️.py` 38, 39, 62 (`G_{\mathrm{t}}` -> `G_{t,20/15}`); `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` 196, 424, 425, 430, 431, 487, 489, 566, 597, 639, 640 (`G_{\mathrm{t20/15}}`, `G_{\mathrm{t}}`, `Gt20/15` -> `G_{t,20/15}`; the plain-text source label reads `G_t,20/15`); `F_{\mathrm{Gt}}` -> `F_{Gt}` in the same file (430, 431, 636); `📊️demand/🎬️clip/README.md` 28, 61, 90, 101; concepts 400, 401.
Needs visual check: heating calculation 566 (`gt_def`), 597 (formula bar with the longer `G_{t,20/15}`), 636 (`f_gt`), 487/489 (formula strip).

### A5. Loads
`\dot{Q}_{\mathrm{C}}` cooling load, `\dot{Q}_{\mathrm{H}}` heating load, `Q_c` cooling demand: no `Q̇_K`, `Q_K`, `Q̇_L` anywhere (grep proof). Demand scenes write the same symbols italic (`\dot{Q}_H`); see style point in A1.

### A6. Forbidden symbols removed from shown text, README, concepts, docstrings
- `📖️concepts/📝️.md`: `ΔT`/`ΔT_eq` -> `Δθ`/`Δθ_eq` (65, 67, 84, 95, 492, 498, 505, 612; table row 612 now just `Δθ`); `ϑ` -> `θ` (66); `Φ_V` -> `Q̇_V`, `Φ_T` -> `Q̇_T` (197, 276, 302, 308, 309, 313); `Q̇_L` -> `Q̇_V` (198, 499); `R_ges` -> `R_T` (236, 237, 274); `F_V` -> `F_C` and `I_S,max` -> `I_s,max`, `I_reduziert` -> `I_s,red` (527-538; the `F_C` row now reads "Sonnenschutzfaktor"); mechanism fluxes `Q̇_k/Q̇_c/Q̇_r` -> `Φ_k/Φ_c/Φ_r` (222-224), surface chain `Φ_c,si + Φ_r,si = Φ_k = Φ_c,se + Φ_r,se` (234), `Φ = U · A · Δθ` (238, 290, 216). Unicode tables were not rewritten into `$…$` (the concepts file is a plain reference, not shown on screen); only the lines carrying forbidden or changed symbols were fixed.
- `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py` docstring lines 1, 5, 410: `Φ_V` -> `Q̇_V` and Kühlung -> Kühltechnik; same in its `🔊️speech/🐍️.py` docstring.
- `❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` 179 region comment `I_S,max` -> `I_s,max`; `🏗️authoring/🎨️visuals/🧪️tests/🔣️scenes.json` 104 layout fixture `I_{S,max} = 800` -> `I_{s,max} = 800`.
- Kept deliberately: `ϑ`/`\vartheta` in the font tests (`🔤️fonts/🧪️tests`), the TeX map `📝️narration/🧮️tex/🔣️.json`; `F_sh` named as "never F_sh" in `❄️cooling/🎬️clip/README.md` and as ISO 52016-1 notation in `📚️references/🔣️.json:401`.

### A7. Indexed Φ
Allowed set after this pass: `\Phi_{\mathrm{k}}`, `\Phi_{\mathrm{c}}`, `\Phi_{\mathrm{r}}` (defined in heating fundamentals 624/734/911) and the surface split `\Phi_{\mathrm{c,si}}`, `\Phi_{\mathrm{r,si}}`, `\Phi_{\mathrm{c,se}}`, `\Phi_{\mathrm{r,se}}` (fundamentals 1055-1057, was `c,i r,i c,e r,e`), plus `\Phi_{HL}` once.
- `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` 1024-1025 narration: `\Phi_i = U_i A_i Δθ_i` -> `\Phi = U · A · Δθ` per part, "their sum `\Sigma\,\Phi` is the transmission load `\dot{Q}_{\mathrm{T}}`" (en + de); 1091 `\Sigma\,\Phi_{i}` -> `\Sigma\,\Phi`; 1109 table header `\Phi_{i}` -> `\Phi`.
- `\Phi_{HL}` (demand video): only the regulations scene (chapter 2, `⚖️regulations/🐍️.py` 549-550, "dort heißt sie Φ_HL" / "which calls it Φ_HL") keeps it. `⚡️demand-power/🐍️.py` 587-588 narration now says "eine eigene Norm, DIN EN 12831-1, und eigene Regeln" and the `Symbol:` fact row (650) shows only `\dot{Q}_{H}`. README mentions of "Norm: Φ_HL" stay (documentation).
- Internal-gain parts are never `Φ_l`-style (grep proof).
Needs visual check: fundamentals 1055-1057 (longer `c,si` subscripts in the surface-chain formula), conduction 1091 and 1109 (shorter sum, header).

## B. German wording
- Kühlung -> Kühltechnik (system meant): `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` 777 (narration), 860 (chip `→ Kühltechnik:`); `📦️volume/🐍️.py` 912 (narration), 1025 (card `Kühltechnik · Sommer`), 1093 (chip); `🗺️roadmap/🐍️.py` 51 (narration), 66 (chip); `🤝️handoff` docstrings; `🌬️ventilation/🎬️clip/README.md` 100; `📊️demand/🎬️clip/🗺️roadmap/🐍️.py` 61 (chip `← Kühltechnik: Q_c`); `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` 3134; quiz `🌬️ventilation/❓️quiz/🔣️.json` 647, 651 and `❄️cooling/❓️quiz/🔣️.json` 859; `🏗️authoring/🎬️clip/🪟️presentation/📚️collection/🔣️.json` 91 ("Auslegung der Kühltechnik"); concepts 18, 157, 168, 176, 202, 204. Already changed by others before this pass: `integration.json` 213 and `🪟️natural` 1551, `README.md` 10 and 121, `handoff` `TITLE_DE`.
- Kept: video title "Kühlung" (concepts headings, `Cooling_*` titles, roadmap handover names in `🗺️roadmap/🧪️tests`), "freie Kühlung", "sorptionsgestützte Kühlung", process uses (physics 2743, 3058 "Kühlung entfeuchtet", concepts 481), and the reviewer's own sentence in `🌬️ventilation/🎬️clip/❓️need/🐍️.py:237` ("…bedarf es immer der Lüftung neben Heizung und Kühlung", F05 quotes it verbatim; the integration fixture mirrors the English/Kühltechnik handover instead, not this line).
- Pettenkoferwert -> "1 000 ppm Grenzwert nach Pettenkofer" (concepts 163); grep: 0 remaining.
- "each facade lights up in turn": the shipped narration was already physics wording ("Facade power peaks east in the morning, south at noon, west in the afternoon"); the phrase survives only in the `previous` block of `integration.json:349`, which is the long-caption regression fixture (`previous` texts must still wrap to 3+ lines, `🐍️integration.py:191`), so it is left as historical input. Narration grep for `leuchtet`, `erscheint`, `verblasst`, `blendet`, `lights up`: 0 hits in clauses (docstrings only).

## C. Verification

Final grep proof (`python consistency/symbols/proof.py`, whole ENERGY tree, py/md/json/ts):

| token | remaining |
|---|---|
| `\mathrm{I}` index, `Q_I`, `Q̇_I`, `I,p/e/l` | 0 |
| `Φ_V`, `Φ_T` | 0 |
| `Q̇_L`, `Q̇_K` | 0 |
| `F_V`, `F_f` | 0 |
| `F_sh` | 2, both deliberate negative mentions (README "never F_sh", references note) |
| `R_ges` | 0 |
| `ΔT` / `\Delta T` | 0 |
| `ϑ` / `\vartheta` | 12, all font tests and the TeX letter map (deliberate fixtures) |
| `G_t20/15`, `G_{\mathrm{t…}}`, bare `G_t` | 0 |
| `e_{\mathrm{E}}`, `F_{\mathrm{Gt}}` | 0 |
| `e_P` | 8, all the primary-energy expenditure figure and the norm symbol list (README), a different quantity |
| `Pettenkoferwert`, `I_S,max`, `Q̇_Licht/Pers/Geräte` | 0 |

Roadmap chips (`🗺️roadmap` ventilation 66 `→ Kühlung:`, demand 61 `← Kühlung:`) stay "Kühlung": the roadmap contract test requires the chip to name the partner VIDEO ("Kühlung"); I had changed them and reverted. Narration at ventilation roadmap 51 stays "die Kühltechnik".

Tests (private output root, `SEMIO_TEACHING_PROFILE=preview`, language env set where a test needs it; `run-tests.sh` runs each module alone):
- Narration guard on the five energy videos: clean (907 clauses). Only 2 offenders, both in `🏗️excel/🎬️clip/⚡️energy` (Φ in `load`), not ours.
- Pass: speech lexicon (12, incl. the new cases), cooling (internal-gains, system-sizing, tests), ventilation (volume, architecture, numbers, transitions), visuals (captions, scenes, sections, section_consumers, icons, labels, readout, transforms, semio-style needs the language env only), presentation hierarchy + chapters + pointer, running-example, narration (guard, tex, caption-layout), roadmap (11), demand (regulations sun, primary-energy chain/scale, recap balance, chain, guard), heating conduction, physics fundamentals.
- Not green and not caused by this pass: `visuals.🧪️tests.🐍️integration` `test_physics_scene_constructor_bounds` (radiator height 0.681 vs 0.738, staffage geometry); `visuals.🧪️tests.🐍️staffage_only` timed out at 900 s in the combined run.
- Fixture `integration.json` `previous` blocks (long-caption regression inputs, e.g. "lights up", "Kühlung") are intentionally unchanged.

## D. Needs visual check
- heating calculation 566, 597, 636, 487/489 (longer `G_{t,20/15}` / `F_{Gt}`)
- heating fundamentals 1055-1057 (`c,si` subscripts)
- heating conduction 1091, 1109 (`\Sigma\,\Phi`, header `\Phi`)
- demand-power ~650 (Symbol row shows only `\dot{Q}_{H}`, three fact rows still stacked), demand roadmap symbols chip `G_{t,20/15}`
- ventilation heat-flow 860, volume 1025 and 1093 (longer `Kühltechnik` chips/cards)
- cooling internal-gains 849-857 (`\mathrm{i,p}` formula rows), unchanged width expected

---

## Appendix C: units fragment

# Units consistency fragment (agent `consistency-units`)

Scripts: `consistency/units/{scan,display,frac,thin,bare,bare_text,balance,quiz_fix}.py`, `run-tests.sh`. Tool output: `🗑️generated/consistency-out/units/`.
ENERGY = `🎓️teaching/🏛️architecture/⚡️energy`; scene paths are relative to it.

## 1. Compound units in display math -> `\frac{\mathrm{A}}{\mathrm{B}}` (121 unit occurrences, 66 lines, `frac.py`)

Rule applied: every `\mathrm{A/B}` outside `$…$` (display MathTex/label/readout strings, f-string `{{ }}` variants included) becomes `\frac{\mathrm{A}}{\mathrm{B}}` (parentheses of the denominator dropped: `W/(m\,K)` -> `\frac{\mathrm{W}}{\mathrm{m\,K}}`). Re-greped: the audit rows 1-61 had drifted; the generator found all current ones (also `kg/m^3`, `J/(kg\,K)`, `g/kg`, `kJ/kg`, `s/h`, `min/h`, `l/s`, `g/h`, `Kd/a`, `h/a`, `kWh/(m^2\,a)` that the audit missed).

| file | lines (before -> after, all `\mathrm{X/Y}` -> `\frac{\mathrm{X}}{\mathrm{Y}}`) |
|---|---|
| `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | 886 (kWh/a, en+de), 2895 (g/kg), 2999 (g/h x2), 3063, 3068 (g/kg, en+de) |
| `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | 1395-1403 (kWh/a, W/m, kWh/d), 1463 |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 632 (W/(m K)), 1328 (W/K x2), 1340, 1359, 1469, 1113 (header `in W/(m^2 K)`, by hand) |
| `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | 1350 (W/(m K)) |
| `📊️demand/🎬️clip/♨️system-losses/🐍️.py` | 1020, 1022, 1117 (kWh/(m^2 a), also constant `unit_tex`) |
| `📊️demand/🎬️clip/⚖️regulations/🐍️.py` | 1207, 1236, 1238 (Kd/a) |
| `📊️demand/🎬️clip/⚡️demand-power/🐍️.py` | 522, 988, 1019, 1225 (kWh/a), 1021 (h/a) |
| `📊️demand/🎬️clip/🏭️primary-energy/🐍️.py` | 113 (`UNIT` constant kWh/(m^2 a)), 1331 (W/(m^2 K) x2) |
| `📊️demand/🎬️clip/🏷️certificate/🐍️.py` | 82 (`UNIT` constant) |
| `📊️demand/🎬️clip/🧪️practice/🐍️.py` | 104 (`UNIT_INLINE` constant) |
| `📊️demand/🎬️clip/🔁️recap/🐍️.py` | 811 (W/(m^2 K)) |
| `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` | 990, 992, 1025 (m^3/s, m/s, x2 each) |
| `🌬️ventilation/🎬️clip/❓️need/🐍️.py` | 360, 364, 373, 374, 384 (m^3/h) |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | 317, 319, 321, 431 (kg/m^3), 340, 431 (J/(kg K)), 371, 431 (s/h), 373, 375, 554, 543, 723, 667, 669, 682, 713, 831, 853 |
| `🌬️ventilation/🎬️clip/📦️volume/🐍️.py` | 797, 884 (min/h), 960, 969, 1003 (m^3/h, s/h, m^3/s), 1004 (l/s), 1024-1058, 1081-1086, 816 (header, by hand) |
| `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py` | 208-215, 321, 512 |
| `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | 1111, 1113, 1140, 1149 |
| `❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` | 370 (W/m^2) |
| `❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py` | 434 (W/K) |
| `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | 304, 308 (kg/m^3, kJ/(kg K)), 526, 528, 542, 573 (m^3/s, m/s), 316, 445 (headers `q_v,R in m^3/s`, by hand) |

Restructured by hand: `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:1140` `\sqrt{2 \cdot Δp\,Pa\,/\,ρ\,kg/m^3}` -> `\sqrt{\frac{2 \cdot Δp\,\mathrm{Pa}}{ρ\,\frac{\mathrm{kg}}{\mathrm{m^3}}}}`.

Kept inline on purpose (judgement):
- chart axis names (`θ in °C`, `x in g/kg` `🧲️physics…fundamentals:2850`, `Q in kWh/Monat` `📊️demand…regulations:728`, `I_s in kWh/m^2` `…regulations:964`, `Irradiance in W/m^2` `❄️cooling…solar-radiation:237`): axis labels, as the brief allows.
- `$…$` in narration (caption layer), the lexicon/tex fixtures and the chain fixture `narration_values` (pinned inline narration strings).
- `\mathrm{t20/15}` is an index of `G_t20/15`, not a unit.

## 2. Thin space / upright (`thin.py`, 70 places, all non-narration TeX)

`number\;\mathrm{unit}` / `number\;\frac{…\mathrm{…}}` / `number\;^{\circ}\mathrm{C}` -> `\,` in `📊️demand/🎬️clip/{⚖️regulations,⚡️demand-power,🔁️recap,🧪️practice}/🐍️.py` (regulations 1158, 1183, 1236, 1238, 1254-1256; demand-power 461-1225 value rows; recap 717-1352 value rows, 875/876 rho note; practice 435). Narration `$…$` untouched (prose). Greps afterwards: digit glued to `\mathrm`/`\text` = 0; `number\;unit` in display = 0. All units are `\mathrm`.

## 3. Numeric factors without units (2c)

| item | result |
|---|---|
| ventilation quiz 898/899 | `28 × 25 m³/h` -> en `28 persons × 25.2 m³/(h·person) + 60 m² × 2.52 m³/(h·m²) ≈ 860 m³/h`, de `28 Pers. × 25,2 m³/(h·Pers.) + 60 m² × 2,52 m³/(h·m²) ≈ 860 m³/h` (exact EN 16798-1 cat. II values: 7 l/s = 25.2 m³/h, 0.7 l/(s·m²) = 2.52 m³/(h·m²); 705.6 + 151.2 = 856.8 ≈ 860, 4.8 1/h unchanged). Sports hall 816/817: `× 30` -> `× 30 athletes` / `× 30 Personen` |
| `integration.json:401` | already corrected by another agent (`30 m² · 0.70 · 0.15 · 0.60 · 800 W/m² ≈ 1510 W`, mirrors `❄️cooling…system-sizing:387/388`, test green); the `previous` entry intentionally keeps the old 0.49 text (it must still wrap to 3+ lines) |
| concepts 187 | `1,2 kg/m³ · 1 005 J/(kg·K) / 3 600 J/Wh ≈ 0,34 Wh/(m³·K)` |
| concepts 188 | `440 m³ · 0,5 h⁻¹ · 0,34 Wh/(m³·K) · 20 K ≈ 1 500 W` (220 m³/h · 0,34 · 20 = 1 496 W) |
| concepts 189 | `1,2 kg/m³ · 2 450 kJ/kg · 4,5 g/kg · 0,061 m³/s ≈ 810 W` (recomputed 807 W) |
| concepts 190 | `1 500 W → 300 W, 450 W → 90 W` |
| concepts 308 | same as 188 |
| concepts 101 | unchanged: `g` is dimensionless |
| physics quiz 2270/2271 | `1,496 TWh × 0.27 / 8,760 h ≈ 0.046 TW = 46 GW` (de `1.496 TWh × 0,27 / 8.760 h ≈ 0,046 TW = 46 GW`) |
| `📊️demand/🎬️clip/README.md:43/44` | `Q_E`, `Q_P` now carry `kWh/(m²·a)` |
| scenes | re-grep (`bare.py`: every `\cdot` chain with a number beside it) found no further substitution lacking a unit; remaining bare factors are dimensionless (η, g, F_*, ratio, counts) |

## 4. Bare bracketed units (concepts, `📖️concepts/📝️.md`)

Unit text only, form `X, in unit` / `U in W/(m²·K)`: 175 (`h⁻¹`), 186 (`m³/h`), 226 (`m²·K/W`), 227, 228, 238, 266, 267, 275, 276, 282 (table header), 456. Audit rows 238/267/275/276/456 all done, plus 175/186/226/227/228/266/282 which the audit had not listed.

## 5. Re-grep proof

- inline compound `\mathrm{A/B}` outside `$…$` in scene/README/quiz/concepts: 0 except the 4 kept axis-label lines, the index `t20/15`, and test fixtures (above) -> display math = 0.
- `\mathrm{W}/\mathrm{K}`, `\,/\,\mathrm{…}` variants: 0.
- bracket units (`[W]`, `[kWh]`, `\left[`, `\,[\mathrm`, `[W/(m²·K)]`, `[m²·K/W]`, `[h⁻¹]`, `[m³/h]`, …) in concepts/README/quiz/scenes: 0 (the cooling README line 46 only describes the rule).
- Narration guard: no offender in the five energy videos; 16 offenders remain in OTHER families (`🏛️architecture/🎯️optimization/…/📉️extrema` θ ×14 and `📊️excel/…/⚡️energy` Φ ×2), not touched.
- Brace balance of all `\frac` strings (`balance.py`): ok.

## 6. Tests

Ran green: cooling (internal-gains, system-sizing, clip tests), ventilation (volume, architecture, numbers, transitions), heating conduction, physics fundamentals, demand (regulations sun, primary-energy chain+scale, recap balance, practice, practice speech/composition, tests chain+guard), visuals (labels, captions, scenes, readout, `🐍️.py`), narration tex, running-example. One pre-existing unrelated failure: `🏗️authoring/🎨️visuals/🧪️tests/🐍️integration.py` `test_physics_scene_constructor_bounds` (radiator height 0.6812 vs 0.7384, a radiator geometry case; no unit string involved).

## 7. Needs visual check (fractions are taller than inline units)

- `📊️demand/🎬️clip/⚡️demand-power/🐍️.py:1019,1021` (kWh/a and h/a fractions inside the `t_VL = Q_h/Q̇_H` panel: fraction in fraction numerator) and `:522,988,1225` (labels), `:779-817` (H_T, H_V rows, were `\;` now `\,`)
- `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:431` (nested: `kg/m³ · J/(kg K)` over `s/h`), `:371`, `:340`, `:667-713` (readouts), `:317-321` (m = ρ·V rows)
- `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1003` (`V̇/3600 s/h = … m³/s` nested), `:884`, `:816` (header `V̇ in m³/h`), `:1024-1086` (tight table rows)
- `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:1025` (`0,061 m³/s / 2,5 m/s` fraction with fraction units, numerator and denominator)
- `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:1140` (new `\sqrt{\frac{…}{…\frac{kg}{m³}}}`), `:1149`, `:1133`
- `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:214` (`4,5 / 11,2 g/kg` pair), `:208-215` (card values)
- `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1395-1403` (readout column, fraction units in each row)
- `🔥️heating/🎬️clip/🧱️conduction/🐍️.py:632` (`R = d/λ` fraction whose denominator holds a fraction unit), `:1113` (header), `:1328-1359` (ψ fractions with W/K numerators)
- `📊️demand/🎬️clip/🏷️certificate/🐍️.py` (`UNIT` constant used in the class-scale labels, lines 558-839) and `📊️demand/🎬️clip/🏭️primary-energy/🐍️.py` (113, 793, 963, 1104-1258) and `🧪️practice/🐍️.py:1247`, `♨️system-losses/🐍️.py:1117-1178`: the unit constant is now a fraction wherever `Q in UNIT` / `Q = 140 UNIT` is shown
- `📊️demand/🎬️clip/⚖️regulations/🐍️.py:1207,1236-1256` (Kd/a, kKh/a fractions)
- `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py:316,445` (headers), `:304,308`, `:526-573`
- `🔁️recap:811` (`du_unit` under the ΔU meter)
