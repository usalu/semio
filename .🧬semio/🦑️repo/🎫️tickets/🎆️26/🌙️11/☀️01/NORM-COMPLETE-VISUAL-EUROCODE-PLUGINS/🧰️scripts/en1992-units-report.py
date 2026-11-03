"""Writes the change table of audits/wave2/units-review-en1992.md body from the old vocabulary json (argv[1]) and the rows of catalogue-bridge-en1992-units.py. Run with python3 -I."""
import json, sys, pathlib, importlib.util
HERE = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("units", HERE / "catalogue-bridge-en1992-units.py")
U = importlib.util.module_from_spec(spec); spec.loader.exec_module(U)
old = json.load(open(sys.argv[1], encoding="utf-8"))["inputs"]
def old_row(key, scope):
    exact = [r for r in old if r["key"] == key and r["scope"] == scope]
    if exact: return exact[0]
    un = [r for r in old if r["key"] == key and not r["scope"]]
    return un[0] if un else None
def desc(r):
    if r is None: return "(none)"
    u = r.get("unit", "-")
    return f'{r["kind"]} {u}'
lines = ["| key | scope | old kind unit (old label) | new kind unit | new EN label | status | evidence |", "|---|---|---|---|---|---|---|"]
changed = same = 0
for r in U.ROWS:
    o = old_row(r["key"], r["scope"])
    symbol = {"mm2": "mm²", "mm3": "mm³", "mm4": "mm⁴", "deg": "°", "MPa": "N/mm²", "Nmm": "N·mm", "pct": "%", "per_m": "1/m", "mm2_m": "mm²/m", "N_mm": "N/mm", "C": "°C"}.get(r["unit"], r["unit"])
    kind = {"n": "number", "v": "vector", "c": "choice", "x": "text"}[r["kind"]]
    new = f'{kind} {symbol}' if kind in ("number", "vector") else kind
    olddesc = desc(o) + (f' ({o["en"]})' if o else "")
    if o is None: status = "added"
    elif desc(o) != new: status = "unit/kind"
    else: status = "label"
    if o is not None and desc(o) == new: same += 1
    else: changed += 1
    scope = r["scope"] or "all"
    ex = "" if r["ex"] is None else f' ex {r["ex"]}'
    lines.append(f'| `{r["key"]}` | {scope} | {olddesc} | {new}{ex} | {r["en"]} | {status} | {U.resolve(r["ev"]).replace("|", "/")} |')
print("\n".join(lines))
print(f"\n<!-- unit/kind changed or added: {changed}; same unit and kind, labels only: {same} -->", file=sys.stderr)
