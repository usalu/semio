import sys
api, rep = sys.argv[1], sys.argv[2]
s = open(api, encoding="utf-8").read()
s = s.replace("# .recalculate(new_workbook, values=None, **kw) -> Animation   # every changed cell morphs to its new text (engine.apply result); bound cells stay live", "# .recalculate(new_workbook, values=None, **kw) -> Glide   # every changed cell is retyped in place (old text fades out in the first half, new text fades in in the second, one green flash; glyphs never leave their cell; `.retexts`, `.animations == [self]`); bound cells stay live")
open(api, "w", encoding="utf-8").write(s)
r = open(rep, encoding="utf-8").read()
r = r.replace("## Notes for scene authors\n", "## Notes for scene authors\n\n- `window.recalculate(wb)` retypes changed cells in place (fade out, fade in, brief green flash) and never moves glyphs between cells; a Glide with `.retexts`. Test `test_recalculate_retypes_cells_in_place` checks every glyph stays in its own cell box at alpha .25/.5/.75/1.", 1)
open(rep, "w", encoding="utf-8").write(r)
