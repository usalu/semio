# Beat2 Thinker Layout

Scene: `Beat2_EnergieImAlltag`.

## Changes
1. **Closer person** — clerk foot at `(2.45, -1.7)` height `2.9` (was far-right close-up `5.15 / 6.4`).
2. **Mind text** — slip phrase sits above the head; thought trail uses 5 dots into the crown; cloud pad slightly larger.
3. **Formula clears clutter** — when `1 kW · 1 h = 1 kWh` appears, fade person, mind cloud, `4000 kWh verbraucht`, clock and `1 Stunde` (fly copies feed the panel).
4. **Vacuum handoff** — drops the formula panel with the house/meter (said/clock already gone).

## Verify
Re-run Manim Sideview on `Beat2_EnergieImAlltag`.


## Talking-head close-up (follow-up)

Person is a large upper-frame close-up again (`foot (4.35, -5.55)`, height `7.0`) so head/torso read as speaking to the viewer. Mind-text sits left of the head with a 5-dot trail. Clock nudged to `(6.05, 2.75)`.

## Hide fly sources after formula

After `kW` / `h` fly into `1 kW · 1 h = 1 kWh`, fade and remove the source copies so no stray kWh remains mid-frame.

## Hide mid-frame kWh on formula fly

`_fill_panel(..., hide_sources=True)` fades origin glyphs as copies leave and removes in-flight copies after the morph — no stray kWh stays mid-frame.

## No ghost kWh for the rest of the beat

Dropped the formula fly/morph. Mind `kWh` / clock are faded and removed before the panel appears, so nothing stays mid-frame through vacuum and later beats.
