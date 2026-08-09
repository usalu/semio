# Fix Heating Solar Scale\nCONTENT_SCALE 0.62 made diagrams+labels unreadably small.\nRaise scale and tighten vertical gap so stage fills the title/caption band.\n

### Scale fix
- Dropped CONTENT_SCALE 0.62 crush; `_fit_stage` fills title–caption band using a diagram `focus` core
- Diagram callouts use BODY_FONT_SIZE (was LABEL)
- Smoke: final_b*.png
