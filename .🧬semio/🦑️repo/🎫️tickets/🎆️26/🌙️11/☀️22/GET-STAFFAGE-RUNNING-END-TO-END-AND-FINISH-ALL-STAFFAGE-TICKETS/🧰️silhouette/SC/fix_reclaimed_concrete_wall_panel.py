"""🧱️ Repair of `reclaimed-concrete-wall-panel`: the six cut rebar stubs (16 mm, 60 mm long, three at each end) make the declared 6.12 m. (1) The plan of 1:50 and 1:100 drew only the topmost stub of each end; the panel leans, so all three are seen from above beyond the panel ends (60 mm from the next drawn edge): the lower two of each end are drawn like the third. (2) The bodies of 1:200 and 1:500 had no stubs and were 6.00 m long against the declared 6.12 m: 1:200 gets the stub solids of 1:100, 1:500 one 75 mm cube per stub (thickened to its smallest feature like its lifting loops: the 60 mm stub with 15 mm of the bar inside the panel); the side elevation of 1:200 draws all of them and its plan the lowest and the topmost of each end (the middle one would leave a gap of 0.1 mm on paper and lies within the limit of the two); at 1:500 they stay undrawn, within the limit. In every plan the lower stubs lie one layer (`z`) below the next higher one, as they do in space: hoisted upright the three are one above the other. (3) In the plan of 1:100 the three stubs of an end are 38 mm apart: they are outlined with the detail pen (0.13 mm) so that 0.25 mm of paper stays between them. Idempotent: python3 fix_reclaimed_concrete_wall_panel.py"""
import copy, sys
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import sc

ID = "reclaimed-concrete-wall-panel"
text, doc = sc.load(ID)
ROWS = {1: -0.9, 2: 0.0, 3: 0.9}
for scale in (50, 100):
    plan = sc.shapes(doc, scale, "plan")
    for side, x0 in (("left", -0.06), ("right", 6.0)):
        third = sc.shape(doc, scale, "plan", f"rebar-stub-{side}-3")
        for row in (2, 1):
            stub = copy.deepcopy(third)
            stub["id"] = f"rebar-stub-{side}-{row}"
            stub["z"] = sc.r6(third["z"] - 0.1 * (3 - row))
            stub["points"] = [[x0, sc.r6(ROWS[row] - 0.008)], [sc.r6(x0 + 0.06), sc.r6(ROWS[row] - 0.008)], [sc.r6(x0 + 0.06), sc.r6(ROWS[row] + 0.008)], [x0, sc.r6(ROWS[row] + 0.008)]]
            sc.put(plan, stub, before=f"rebar-stub-{side}-3")
        if scale == 100:
            for row in (1, 2, 3):
                sc.shape(doc, scale, "plan", f"rebar-stub-{side}-{row}")["penMm"] = 0.13
for scale in (200, 500):
    for side in ("left", "right"):
        rods = copy.deepcopy(sc.solid(doc, 100, f"rebar-stubs-{side}"))
        if scale == 500:
            x, y, z = rods["origin"]
            rods = {"id": rods["id"], "kind": "box", "bone": rods["bone"], "material": rods["material"], "role": rods["role"], "tone": rods["tone"], "size": [0.075, 0.075, 0.075], "transform": {"translation": [sc.r6(x + 0.0375 if x < 0 else x + 0.0225), y, z]}, "repeat": rods["repeat"]}
        sc.put(sc.solids(doc, scale), rods)
for view in ("elevation", "plan"):
    for stub in [s for s in list(sc.shapes(doc, 100, view)) if s["id"].startswith("rebar-stub-")]:
        if view == "plan" and stub["id"].endswith("-2"):
            sc.drop(sc.shapes(doc, 200, view), stub["id"])
            continue
        drawn = copy.deepcopy(stub)
        drawn.pop("penMm", None)
        sc.put(sc.shapes(doc, 200, view), drawn)
sc.save(ID, text, doc)
