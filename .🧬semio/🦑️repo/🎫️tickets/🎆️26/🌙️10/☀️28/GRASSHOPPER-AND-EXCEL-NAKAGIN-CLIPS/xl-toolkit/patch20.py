import sys
api, rep = sys.argv[1], sys.argv[2]
s = open(api, encoding="utf-8").read()
old = [l for l in s.split("\n") if l.startswith("xl_tabs(")][0]
s = s.replace(old, 'xl_tabs(sheets, active, *, at=(0,0,0), width=None, titles=None) -> VGroup   # .tabs[canonical sheet name] .plus ; titles {name: tab text} e.g. engine.sheet_titles(language, wb) ; compact 16.2 tabs, all 7 Nakagin sheets fit at 12.4 ; active tab white + green underline ; tabs that do not fit are dropped from the left')
note = [l for l in s.split("\n") if l.startswith("Notes: a window")][0]
extra = (" Window tabs are localized automatically through the engine's `tab_name` (`window.tabs` stays keyed by canonical sheet name; no wrapper view needed). `Window.scene` is a weak-reference property, so `FadeIn`/`FadeOut`/`copy.deepcopy` of a window work without scene overrides."
         " Text placement: runs without letters or digits (`-`, `*`, `=`, `,`) are placed by the real baseline, never by their ink bottom (the old `-` → `_` bug); `(` and `)` are drawn with the theme serif face because the body face renders them as square brackets."
         " Overflow: left-aligned text and centred text (error codes such as `#BEZUG!`, `#ÜBERLAUF!`) spread over empty neighbour cells (centred: equally on both sides, on an opaque cell-coloured backing so gridlines do not cross it); centred text that still does not fit is scaled down, never cut; left text that does not fit ends in an ellipsis; numbers show `###`.")
s = s.replace(note, note + extra)
open(api, "w", encoding="utf-8").write(s)
r = open(rep, encoding="utf-8").read()
r = r.replace("## Notes for scene authors\n", "## Notes for scene authors\n\n- Tabs: localized by the engine's `tab_name` automatically; `xl_tabs(..., titles=sheet_titles(language, wb))` for standalone strips; compact tabs, all seven sheets fit at 12.4 units.\n- `window.scene` is weak: standard `FadeIn`, `FadeOut`, `copy.deepcopy` work; drop scene `__deepcopy__` overrides.\n- Glyphs: `-`, `*`, `=`, `,` sit on the baseline; `(` `)` use the theme serif face (round, the body face draws brackets). Test `test_formula_glyphs` renders `=SUM(B2:B4)-B5`.\n- Overflow: left text and centred errors overflow into empty neighbours (opaque backing), centred text is scaled down if needed, left text gets an ellipsis, numbers `###`; error codes are never truncated.")
open(rep, "w", encoding="utf-8").write(r)
