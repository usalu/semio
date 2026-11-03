import json, re
p = r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy\🏗️authoring\🎨️visuals\🧍️staffage\🔣️.json"
lines = open(p, encoding="utf-8", newline="").read().split("\n")
out, n = [], 0
for line in lines:
    if '"view": "thermal"' in line and '"asset"' in line and "thermalSpanK" not in line:
        m = re.match(r"^(\s*\"[^\"]+\": \{.*?)( \}),?$", line)
        assert m, line
        comma = "," if line.rstrip().endswith("},") else ""
        line = m.group(1) + ', "thermalSpanK": 25 }' + comma
        n += 1
    out.append(line)
open(p, "w", encoding="utf-8", newline="").write("\n".join(out))
d = json.load(open(p, encoding="utf-8"))
print(n, sum(1 for v in d["casts"].values() if v["view"] == "thermal"), sum(1 for v in d["casts"].values() if "thermalSpanK" in v))
