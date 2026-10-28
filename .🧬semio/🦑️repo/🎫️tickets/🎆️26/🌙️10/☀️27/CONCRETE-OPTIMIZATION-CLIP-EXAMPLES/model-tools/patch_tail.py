import sys
from pathlib import Path
tail = Path(sys.argv[1]); model = Path(sys.argv[2])
t = tail.read_text(encoding="utf-8")
a = t.index("def weighted_pick(weight):"); b = t.index("def _clip(value, bounds):")
t = t[:a] + '''def _pick(front, weight):
    """🎯 Front id minimizing the weighted sum of min–max normalized cost and electricity."""
    ranges = {key: (min(item[key] for item in front), max(item[key] for item in front)) for key in ("cost", "electricity")}
    normal = lambda item, key: (item[key] - ranges[key][0]) / (ranges[key][1] - ranges[key][0])
    return min(front, key=lambda item: (1 - weight) * normal(item, "cost") + weight * normal(item, "electricity"))["id"]


def weighted_pick(weight):
    """⚖️ Front id for a weighted sum of normalized cost and electricity; weight is the share of electricity."""
    if not 0 <= weight <= 1:
        raise ValueError("The electricity weight must lie between 0 and 1")
    data = _pareto()
    lookup = {item["id"]: item for item in data["designs"]}
    return _pick([lookup[identifier] for identifier in data["front"]], weight)


''' + t[b:]
t = t.replace('''"picks": [{"weight": weight, "id": weighted_pick(weight)} for weight in setup["weights"]]}''', '''"picks": [{"weight": weight, "id": _pick(front, weight)} for weight in setup["weights"]]}''')
t = t.replace('''    return {"designs": designs, "front": [item["id"] for item in front], "dominated_example"''', '''    front = [lookup[item["id"]] for item in front]
    return {"designs": designs, "front": [item["id"] for item in front], "dominated_example"''')
t = t.replace('''"pattern": [_gradient_path()[0]["electricity"] if False else _pattern_path()[0]["electricity"],''', '''"pattern": [_pattern_path()[0]["electricity"],''')
t = t.replace('''_contrast(y, [a for a, _ in (run["coded"] for run in runs)])''', '''_contrast(y, [run["coded"][0] for run in runs])''')
m = model.read_text(encoding="utf-8")
i = m.index("def _sample(wwr, overhang):")
m = m[:i] + t
m = m.replace('''"tariff": tariff,''', '''"tariff": tariff,''').replace('values["prices"]["tariff"]', 'values["economics"]["tariff"]').replace('"sun": settings["sun"]', '"sun": study["sun"]')
model.write_text(m, encoding="utf-8")
