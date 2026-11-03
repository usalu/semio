"""Traffic groups layer F: python oracle (table-driven groups, row coefficient) and the two check-outcome fixtures plus feature rows."""
from tg_lib import *
import copy
OR = SUB + "/🔮️oracles/🚦️check-outcome/🐍️.py"
def oracle(t):
    if "GROUP_COEFFICIENT" in t:
        return t
    t = rep(t, "ALPHA_CR_FIRST_ORDER = 10.0\n", '''ALPHA_CR_FIRST_ORDER = 10.0
GROUP_COEFFICIENT = {
    ("gr1a", "road_traffic"): 1.0, ("gr1a", "road_udl"): 1.0, ("gr1a", "road_pedestrian"): 1.0, ("gr1b", "road_single_axle"): 1.0,
    ("gr2", "road_horizontal"): 1.0, ("gr3", "road_gr3_pedestrian"): 1.0, ("gr4", "road_gr4_crowd"): 1.0, ("gr5", "road_special"): 1.0,
}


def effective_group(doc, variable):
    """🚦️ The exclusive group of a variable: its trafficGroups row (lower-cased key) wins over the free group text."""
    row = next((r for r in doc.get("trafficGroups", []) if r["variableId"] == variable["id"]), None)
    return row["group"].lower() if row else variable.get("group", "")
''')
    t = rep(t, '''                return q.get("group", "") == "" or any(
                    o is not q and ((o.get("group", "") == q.get("group", "") and og != g) or (og == g and o.get("group", "") != q.get("group", "")))
                    for o, og in traffic
                )''', '''                return effective_group(doc, q) == "" or any(
                    o is not q and ((effective_group(doc, o) == effective_group(doc, q) and og != g) or (og == g and effective_group(doc, o) != effective_group(doc, q)))
                    for o, og in traffic
                )''')
    t = rep(t, '            put("en1990.a2.traffic-groups", 1.0 + sum(violates(q, g) for q, g in traffic) / len(traffic))\n', '''            put("en1990.a2.traffic-groups", 1.0 + sum(violates(q, g) for q, g in traffic) / len(traffic))
        for row in doc.get("trafficGroups", []):
            variable = next((q for q in doc["variables"] if q["id"] == row["variableId"]), None)
            coefficient = GROUP_COEFFICIENT.get((row["group"].lower(), variable["category"])) if variable else None
            if coefficient is not None and TRAFFIC_GROUP.get(variable["category"]) == row["group"].lower():
                put(f"en1990.a2.traffic-group.{row['id']}", coefficient)
''')
    return t
edit(OR, oracle)
FX = SUB + "/🧪️tests/🧫️fixtures/🚦️check-outcome"
base = json.loads(rd(FX + "/traffic-groups-pass/🔣️.json"))
def build(rows):
    d = copy.deepcopy(base)
    d["projectId"] = "traffic-group-table"
    for q in d["variables"]:
        q["group"] = ""
    d["trafficGroups"] = [{"id": i, "group": g, "variableId": v} for i, g, v in rows]
    return d
for name, rows in (("traffic-group-table-pass", [("TG-ts", "GR1A", "Q-ts"), ("TG-udl", "GR1A", "Q-udl"), ("TG-sa", "GR1B", "Q-sa")]), ("traffic-group-table-fail", [("TG-ts", "GR1A", "Q-ts"), ("TG-udl", "GR1A", "Q-udl"), ("TG-sa", "GR1A", "Q-sa")])):
    os.makedirs(FX + "/" + name, exist_ok=True)
    wr(FX + "/" + name + "/🔣️.json", json.dumps(build(rows), indent=2, ensure_ascii=False) + "\n")
    print("  wrote", name)
FE = SUB + "/🧪️tests/🚦️check-outcome-en1990-1/🥒️.feature"
rows = ("      | pass-a2-09 | shared://🚦️check-outcome/traffic-group-table-pass/🔣️.json | en1990.a2.traffic-groups | pass | 1 | 1e-6 |\n"
        "      | pass-a2-10 | shared://🚦️check-outcome/traffic-group-table-pass/🔣️.json | en1990.a2.traffic-group.TG-ts | pass | 1 | 1e-6 |\n"
        "      | pass-a2-11 | shared://🚦️check-outcome/traffic-group-table-pass/🔣️.json | en1990.a2.traffic-group.TG-sa | pass | 1 | 1e-6 |\n"
        "      | fail-a2-12 | shared://🚦️check-outcome/traffic-group-table-fail/🔣️.json | en1990.a2.traffic-groups | fail | 2 | 1e-6 |\n")
anchor = "      | fail-a2-08 | shared://🚦️check-outcome/traffic-groups-fail/🔣️.json | en1990.a2.traffic-groups | fail | 2 | 1e-6 |\n"
edit(FE, lambda t: t if "pass-a2-09" in t else rep(t, anchor, anchor + rows))
