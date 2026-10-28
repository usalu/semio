import json, math
b = json.load(open("🗑️generated/model-r3/before.json", encoding="utf-8"))["designs"]
a = json.load(open("🗑️generated/model-r3/after.json", encoding="utf-8"))["designs"]
rows = {}
for k, v in b.items():
    n = a[k]
    theta = float(k.split()[0])
    dh, dc, de = n["heating"] - v["heating"], n["cooling"] - v["cooling"], n["electricity"] - v["electricity"]
    if abs(de) > 1e-9 or abs(n["shading_summer"] - v["shading_summer"]) > 1e-12 or abs(n["shading_winter"] - v["shading_winter"]) > 1e-12:
        r = rows.setdefault(theta % 360, [0, 0, 0, 0, 0, 0])
        r[0] += 1
        r[1] = max(r[1], abs(dh)); r[2] = max(r[2], abs(dc)); r[3] = max(r[3], abs(de))
        r[4] = max(r[4], abs(n["shading_summer"] - v["shading_summer"])); r[5] = max(r[5], abs(n["shading_winter"] - v["shading_winter"]))
print("bearing(mod 360) | designs | max|dHeating| | max|dCooling| | max|dElectricity| | max|dShadeSummer| | max|dShadeWinter|")
for t in sorted(rows):
    r = rows[t]
    print(f"{t:g} | {r[0]} | {r[1]:.2f} | {r[2]:.2f} | {r[3]:.2f} | {r[4]:.4f} | {r[5]:.4f}")
print()
print("probe wwr 45 %, d 0.6 m, t 16 cm: bearing: before -> after (shading_summer, cooling, electricity)")
for t in ("340", "345", "350", "353", "354", "354.5", "355", "356", "360"):
    key = f"{t} 0.45 0.6 0.16"
    if key in b:
        v, n = b[key], a[key]
        print(t, f"{v['shading_summer']:.4f}->{n['shading_summer']:.4f}", f"{v['cooling']:.1f}->{n['cooling']:.1f}", f"{v['electricity']:.1f}->{n['electricity']:.1f}")
