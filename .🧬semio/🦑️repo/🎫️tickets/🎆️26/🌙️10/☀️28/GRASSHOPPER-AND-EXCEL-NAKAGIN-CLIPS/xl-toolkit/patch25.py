import sys
api, rep = sys.argv[1], sys.argv[2]
add = " Formula bar: text that does not fit one line at 19.2 shrinks to 16.2, and if it still does not fit it wraps at operators and separators (never inside a reference) into an expanded bar that grows downward over the grid with Excel's chevron (`up`); coloured tokens span the lines, nothing leaves the window (`window.formula_bar[1].current.lines` lists the lines). A formula edited in a cell that is too long for the window is shown only in the bar."
s = open(api, encoding="utf-8").read()
key = [l for l in s.split("\n") if l.startswith("Notes: a window")][0]
s = s.replace(key, key + add)
open(api, "w", encoding="utf-8").write(s)
r = open(rep, encoding="utf-8").read()
r = r.replace("## Notes for scene authors\n", "## Notes for scene authors\n\n- Long formulas: the formula bar shrinks to 16.2, then wraps (operators, separators, never inside a reference) into an expanded panel over the grid with a chevron; colours stay per reference; in-cell edit text is dropped when it cannot fit. Test: `test_long_formula_wraps_inside_the_window` (120+ character German formula); still `long-{en,de}.png`.")
open(rep, "w", encoding="utf-8").write(r)
