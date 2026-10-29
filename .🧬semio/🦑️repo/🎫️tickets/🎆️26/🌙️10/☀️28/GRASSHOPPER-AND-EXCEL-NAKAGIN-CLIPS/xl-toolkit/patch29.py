import sys
api, rep = sys.argv[1], sys.argv[2]
add = " Draw order inside a window is fill, selection tint, gridlines, cell text, headers, reference frames, active border, overlays, chrome, and it survives animation: `select` and `recalculate` return a `Glide` animation whose mobject is the window itself, so the scene never lifts the tint or the texts out of the window (the old AnimationGroup did, which greyed values under the tint)."
s = open(api, encoding="utf-8").read()
key = [l for l in s.split("\n") if l.startswith("Notes: a window")][0]
open(api, "w", encoding="utf-8").write(s.replace(key, key + add))
r = open(rep, encoding="utf-8").read()
open(rep, "w", encoding="utf-8").write(r.replace("## Notes for scene authors\n", "## Notes for scene authors\n\n- Z-order is fixed (fill, tint, gridlines, text, frames, border) and kept by `select` / `recalculate` (`Glide` animates in place); the range tint can no longer grey the values. Test `test_selection_tint_stays_under_text` plays both animations and checks the order."))
