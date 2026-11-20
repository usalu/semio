# Ventilation review report

Timestamps were mapped by content (silent estimate: Ventilation starts at about 16:00). Screenshots: `🖼️screenshots/V<letter>_<slug>_before|after.png` (German, spectator render, 480p15, clause-end frame). Scene files live under `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/`.

Note: the working tree already contained uncommitted layout work by others (unit blocks "Gebäudebezogene Einheiten / SI-Einheiten", sun/shading in the night beat, the Δρ/v chain). BEFORE = tree as found, so for F, L, M the before frame already shows that earlier work.

| Req | Scene / file | Change | Screenshots | LAYOUT_CHECK |
|---|---|---|---|---|
| A | ❓️need `Beat1_Lueftungsbedarf` | Gauge tag was only "Grenzwert"; now "Grenzwert nach Pettenkofer" (DE, two lines) / "Pettenkofer limit" (EN). Narration and references locator already said so. | VA_pettenkofer-label | clean |
| B | ❓️need `Beat1_Lueftungsbedarf`, clause `jahreszeit` | Narration "Lüftung ist neben Heizen und Kühlen immer notwendig" (EN equivalent) plus a yellow chip on screen. | VB_always-needed | clean |
| C1 | curriculum `🎬️clip/🔣️.json` | No change: all five videos already open with their own `Beat1_Fahrplan`. A course-level overview before the Fundamentals video does not exist (Fundamentals scope, other agent). Chapters 2-6 rendered alone via `--chapter` have no Fahrplan (see O). | - | - |
| C2 | `🗺️roadmap`, ❓️need `Beat2_Lueftungsweg`, README | All Φ removed from Ventilation: card symbol, roadmap narration and path narration use `Q̇_V` (V = Lüftung). Φ remains only in Fundamentals/Heating (outside my scope; Heating roadmap and heating fundamentals still use Φ and need the same decision). | VC_heat-flow-symbol, VC_roadmap-symbol | clean |
| C3 | Fundamentals | The "2 Heat flow" slide is in Fundamentals (other agent); Ventilation no longer uses Φ. | - | - |
| C4 | 🌡️heat-flow `Beat3_LatenteLast`, clause `dx` | Δx first appears in chapter 3 only. Now justified in narration: the humidity difference drives the latent flow like Δθ the sensible one. No Δx exists at the timestamp in Ventilation; the professor probably meant Δθ or the Fundamentals sensible/latent beat. | VC_delta-x-latent | clean |
| D | 📦️volume `Beat1_Innenvolumen` | The blue outdoor-air streams that faded out at "focus" are removed (with the unused `_fade_flow`). | VD_outside-air-fades | pre-existing fill-polygon note only |
| E | 🌡️heat-flow `Beat1_Luftwaerme`, `🧪️architecture/🔣️.numbers.json`, `🐍️numbers.py` | Scene already states 1.2 kg/m³ at 20 °C and 1.27 kg/m³ at 5 °C; the 1.27 label is enlarged and rows spaced. New language-agnostic test checks ideal-gas values (1.204 at 20, 1.269 at 5, 1.292 at 0 °C) and that the scene quotes each value with its temperature. | VE_air-density | morph overlaps at t=43-44 s are transition frames (pre-existing) |
| F | 🌡️heat-flow `Beat1_Luftwaerme` | Already in tree: "Gebäudebezogene Einheiten" / "SI-Einheiten" and "Einheiten variieren, nicht nur in der DIN". No edit. | VF_unit-sets | clean |
| G, I | all Ventilation scenes | Scanned all `label_number`/formula strings: every quantity carries a unit except dimensionless ones (η, 0.76, ×). No `[W]`-style bracket units exist; readouts use "in W". No edit needed. | VG_units-missing, VI_units-brackets | Sensible: 0.01 touch Temperaturdifferenz/Δθ (pre-existing) |
| H | narration/lexicon | Ventilation text already writes `$\Delta\theta$` after "Temperaturdifferenz"; lexicon rule `greek-name` rejects spelled-out "Delta-Theta"; lexicon tests pass. TTS speaks the symbol as "Delta Theta" by lexicon design. | VH_delta-theta | - |
| J | 🪟️natural `Beat2_StossOderKipp` | "8 %" is not a published figure: it is 5 min / 60 min at n = 1 h⁻¹. Source for n: Maas (1995), registry id `maas-1995` (ch. 8.2, p. 74: 0.56 to 1.38 h⁻¹), already searchable via the cases (`"Maas": "maas-1995"`) and shown top right. I could not verify any source giving 8 % itself and did not invent one. References JSON untouched. | VJ_eight-percent | pre-existing `weit auf` overlap with the 5min/60min chip at t=30.7 s (not touched) |
| K | 🪟️natural `Beat4_Auftrieb` | Outlet is now a visible chimney above a lower room (shaft 0.9 above the roof, rises in the "taller" step); inlet stays a plain wall opening; stack paths, h arrows and tokens follow the real outlet position. | VK_stack-openings | clean |
| L | 🪟️natural `Beat4_Auftrieb` | `Δρ(θ)` and `v(Δp)` named in narration (DE+EN) and as formula left-hand sides. | VL_drho-v-dp | clean |
| M | 🪟️natural `Beat5_Nachtlueftung` | Sun, rays and shading louvre existed but tiny: room, window, sun and louvre enlarged; label moved clear of curves. | VM_sun-shading | label near-touches wall bounding box (note only) |
| N | 🪟️natural `Beat6_GrenzenDerFreienLueftung` | Single-sided round loops through one opening looked like a short circuit. Now intake low in the left wall, exhaust opening in the ceiling, one through-flow, "ungenutzt η = 0" at the exhaust. Mechanical `Beat1_Grundtypen` checked: already low intake and high exhaust. | VN_short-circuit | clean |

## Tests
- `🧪️architecture` (ventilation): `🐍️.py` ok, `🐍️numbers.py` new test ok (run with `unittest`, no pytest in the venv, env `SEMIO_TEACHING_*` and PYTHONPATH set).
- Failing, not caused by me: `test_literal_narration_quotes_the_computed_numbers` (expects "0.069" m³/s in Beat4 narration), `🐍️transitions.py` (4, cross-ventilation transitions in Beat3_Querlueftung, concurrent edit), volume morph test `test_authored_show_use...`, references test (26 physics-quiz designations without edition).

## O: chapters as separate videos
- Declared in `🎬️clip/🔣️.json`: `videos[].chapters[]` (ordered segments of scene and clauses). `--chapter <id>` renders one chapter inside the virtual window; the video `script` (entry `🎬️clip/🐍️.py`, fixed `VIDEO_ID`) and `output` give one master per video; each chapter already has a poster and export.
- Every Kapitel already renders alone (`--chapter Ventilation_0N_...`). Real separate videos need one entry script per video (schema pattern allows one `🎬️clip/🐍️.py` per folder, `id` maps to a constant) and presentation-hierarchy changes, so it is not a small schema edit. Not implemented.
- Recommendation: let curriculum video entries share the folder entry script with `VIDEO_ID` passed in, and give the shared roadmap builder a "you are here" step so every chapter video opens with the Fahrplan (also resolves C1 for single chapters). Needs the user's decision.

## Cleanup
Scratchpad render outputs (`scratchpad/out`, `ot`, sheet PNGs) were not deleted because the safety check blocked `rm -rf` with a variable path; they live only in the session scratchpad.
