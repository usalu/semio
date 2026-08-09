## Polish Heating Modul 5 scene_5
Same skill polish as Modul 2–4:
- Topic: Modul 5: Solarer Wärmegewinn
- Fonts: only manim_fonts type scale (no literal sizes)
- Formulas: G·A·F_f·g·F_sh [W]; Q_speicher=m·c·ΔT [J]; callouts with SI units
- Screen: compact stages + _fit_stage; FORMULA_EDGE_BUFF=1.2
- Captions: swap_caption + hold_for synced; narration mentions units
- Animations preserved (reposition / timing only)

Smoke: all 8 beats Rendered at -ql

### Title/subtitle sync
- Shared `beat_subtitle` + `BEAT_SUBTITLE_FADE` in manim_fonts
- Heating Modul 1–5 all use it (BODY_FONT, set_x(0), buff=0.22)
- Modul 5 beat titles renamed to Modul 2–4 style

### Soft sun opacities
- Never `FadeIn` glow/rings as a group after `set_opacity(1)` (flattens soft fills).
- Reveal pattern: set core fill to 1.0 (FadeIn target), animate glow/rings to original soft opacities (glow 0.2–0.4, rings 0.25–0.8).
- Do not FadeIn a mobject that still has fill opacity 0 (target stays invisible).
- Beats 1,2,4,5,6 updated; smoke frames `frames/sun_b*f.png`.
