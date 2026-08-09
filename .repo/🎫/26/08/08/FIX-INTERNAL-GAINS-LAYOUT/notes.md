## Redo last 2 prompts
1. Shift/rescale whole animated stage under titles via `_fit_stage(mob, below=subtitle)`
2. Beat3: person seated at desk with laptop + server (not far-left orphan)
Constants: CONTENT_SCALE=0.70, CONTENT_GAP_BELOW_TITLE=0.55
Smoke: all 5 beats Rendered

### Beat2 fix
- Soft glow fill opacities 0.03/0.06/0.12 (no set_opacity(1))
- Watt badge next_to(house, RIGHT) outside walls

### Beat3 human + cord
- Reused `_seated_person_with_chair()` from Beat2 at desk (LEFT*0.85)
- Cord: stroke-only ArcBetweenPoints; hide/show via set_stroke(opacity) — never set_opacity (fills arc sector)
- Smoke: Beat3_GeraetePhiE Rendered; frames beat3_person.png beat3_cord.png

### Formula lower
- FORMULA_EDGE_BUFF=1.2 (was default 1.7) on all Beat2–5 panels
