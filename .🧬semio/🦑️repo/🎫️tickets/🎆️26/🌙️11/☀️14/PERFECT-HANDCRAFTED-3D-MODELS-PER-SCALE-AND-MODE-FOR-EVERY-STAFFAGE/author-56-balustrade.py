import os, json
exec(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "author-17-lib.py")).read())
p = os.path.join(T, "volumes", "components", "glass-balustrade.json")
cur = json.load(open(p))
v = cur["volumes"]["200"]
v["structure"] = [{"id": "handrail-member", "of": "handrail", "member": "panel", "part": "member"} if d["id"] == "handrail-member" else d for d in v["structure"]]
for d in v["decisions"]:
    if d["en"].startswith("Thermal:") and "octagonal prism itself" not in d["en"]:
        d["en"] = d["en"].replace("a square 53 mm handrail member (the octagon's hull)", "the octagonal handrail prism itself as the handrail member (a square hull member would bury the handrail's surface in the exposed-area test)")
        d["de"] = d["de"].replace("ein quadratischer Handlaufstab von 53 mm (die Hülle des Achtecks)", "das achteckige Handlaufprisma selbst als Handlaufstab (ein quadratischer Hüllstab begrübe die Handlauffläche im Flächentest)")
json.dump(cur, open(p, "w"), ensure_ascii=False, indent=1)
print([d["en"][:160] for d in v["decisions"] if d["en"].startswith("Thermal")])
