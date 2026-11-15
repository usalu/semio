# vent-b report (ventilation chapters 4–6), recorded by the coordinator from the agent's reply

Status at 06:14: code, narration (en/de), README and tests done; NOT yet visually verified
(render of `Ventilation_04_NatuerlicheLueftung` was queued behind the memory gate;
mechanical and handoff not rendered). Risks: `$…$` captions over two lines; new layouts
(handoff nine-card row, mechanical rooms) unchecked.

## Items
- F12: "8 %" = 5 min / 60 min at a tilted-window air change of about 1 h⁻¹. Source: Maas (1995),
  Diss. Univ. Kassel, ch. 8 (0,56–1,38 h⁻¹ for a permanently tilted window); page numbers
  (~74–75) estimated from extracted text, unverified. Source line + chip on screen; purge-energy
  and summer equations carry units on every number, fraction bars.
- F13/F14: stack effect has a low inlet opening and a high outlet opening on a shaft, no window
  glyph. Introduces Δρ(θ), Δp = Δρ·g·h, v = √(2Δp/ρ), V̇ = C_d·A·v before the result. C_d = 0,6
  (typical, not pinned to a clause), A = 0,1 m²; cites DIN EN 16798-7:2017-11.
- F15: night ventilation shows sun, rays, external shading device, leak ray into the thermal
  mass; shading off at night, back by day.
- F16: mechanical Beat1 redrawn in all three rooms (inlet low one side, outlet high opposite
  side, occupant); Beat3 central-unit room supply low / extract high.
- F17/F18: the "third symbol" was `r` without a card. Now nine cards incl. ρ and r₀. Winter
  latent 770 W replaces the dash; sum row Q̇_V (2270 W winter, 1260 W summer) with and without
  recovery; heating-load convention note; Beat2 latent display. "Kühlung" → "Kühltechnik".
- Symbols: no Φ_V / Q̇_L left; narration uses `$…$`.

## Visual verification (added later)
All three chapters rendered through the pipeline (`repo tools exec --light`, preview, German,
one chapter at a time); frames in `🗑️generated/vent-b-out/frames` (`b2`, `b4`, `b5`, `m1`, `m2`,
`h1`, `h2`). Narration guard clean (filter `ventilation`).
- Six clauses wrapped to three lines → shortened; no caption over two lines; `$…$` renders.
- Beat2_StossOderKipp: source line, 8 % chip, equation fit; "Laibung 190 Wh" bar label scaled
  after the frame was taken (not seen rendered).
- Beat4_Auftrieb: Δρ → Δp → v → V̇ step by step; v panel inline `Δp/ρ`, legend moved; winter and
  taller-shaft frames not checked in the final render.
- Beat5_Nachtlueftung: sun, rays, shading visible; "Day: close + shade" frame not extracted.
- Mechanical Beat1: rooms narrowed (width 3,5, centres ±4,35), fits. Beat2 crashed on `\mid`,
  fixed; latent note (winter 770 W, summer 810 W) renders.
- Handoff Beat1: nine cards, 770 W instead of the dash, sum row render cleanly; card names small
  at 480p. Beat2: chips shortened (Q̇_V, Q̇_V,sens); winter note "nicht in der Heizlast".
- Computed narration in Beat4_Auftrieb / mechanical Beat2 turned into literal strings + test
  checking quoted against computed numbers.
- Failing, not from these files: `test_become_only_at_listed_analytical_or_static_sites`
  (expects `❓️need` / `icon`, removed by vent-a's changes).

## Shared value changed
Running example: `outdoorHumidityGKg` 3,0, `indoorHumidityGKg` 7,3, `D_X_WINTER_G_KG`,
`Q_LAT_WINTER_W` + test. vent-a's heat-flow latent beat may show winter Δx too.

## Renames
Chapter 6 title "Übergabe an Heizung und Kühltechnik" (scene title + README; manifest chapter
name unchanged). Quiz unchanged.

## Tests
running-example, transitions, numbers pass. New `🧪️architecture/🐍️numbers.py` +
`🔣️.numbers.json`; handoff transition count 11 (`lat-rho`, `lat-r0`).

## Files (under `🎓️teaching/🏛️architecture/⚡️energy/`)
- `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py`, `⚙️mechanical/🐍️.py`, `🤝️handoff/🐍️.py`
- `🌬️ventilation/🎬️clip/🤝️handoff/🔀️transitions/🔣️.json`
- `🌬️ventilation/🎬️clip/🧪️architecture/🔣️.transitions.json`, `🐍️numbers.py`, `🔣️.numbers.json`
- `🌬️ventilation/🎬️clip/README.md`
- `🏗️authoring/📏️running-example/🔣️.json`, `🐍️.py`, `🧪️tests/🐍️.py`
