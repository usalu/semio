"""🧪️ Runs the measurement and every audit of the BIM IFC oracle for each case and writes everything as JSON, so a split can be compared byte for byte.

usage: pycompare.py <oracle 🐍️.py> <fixtures/🏗️ifc dir> <out.json>
Per case: the `measure` table, the `audit` problems (open_case, measure, audit as `main` calls them), the EXPRESS pass is skipped (the system Python
lacks `_pytest`); for the psets case also `library_table` and `library_problems`. A case that raises records the exception type and message.
"""
import json, runpy, sys, traceback
import ifcopenshell

oracle, fixtures, out = sys.argv[1:4]
g = runpy.run_path(oracle, run_name="oracle")
result = {}
for name in list(g["CASES"]):
    entry = {}
    try:
        case, snapshot, model, committed = g["open_case"](fixtures, name)
        table = g["measure"](model, snapshot)
        entry["table"] = table
        entry["problems"] = g["audit"](model, snapshot, table, committed)
        if name == g["PSETS"]:
            lib = ifcopenshell.open(str(case / g["LIBRARY"]))
            lt = g["library_table"](lib)
            entry["library_table"] = lt
            entry["library_problems"] = g["library_problems"](lib, snapshot, lt)
    except Exception as e:
        entry["error"] = "%s: %s" % (type(e).__name__, str(e).split("\n")[0][-160:])
    result[name] = entry
json.dump(result, open(out, "w", encoding="utf-8"), indent=1, sort_keys=True, ensure_ascii=False, default=str)
print({k: ("error" if "error" in v else "ok") for k, v in result.items()})
