import sys, importlib, json, subprocess
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/cargo-bike")
sc = int(sys.argv[1]); bones = set(sys.argv[2].split(","))
out = subprocess.run(["./vehicles-run.sh", "extj", "cargo-bike", str(sc)], capture_output=True, text=True).stdout
ext = json.loads(out[out.index("{"):])
for it in d.at("asset", "representations", str(sc), "geometry", "elevation")["items"]:
    o = json.loads(d.t[it["s"]:it["e"]])
    if o.get("bone") not in bones or "solid" in o: continue
    ids = [k for k in ext if k == o["id"] or k.startswith(o["id"] + "-") or o["id"].startswith(k)]
    z0 = min((ext[k][0][2] for k in ids), default=-0.03); z1 = max((ext[k][1][2] for k in ids), default=0.03)
    hw = round(max(abs(z0), abs(z1)), 4)
    d.add_key_after(it, "closed" if o["kind"] == "poly" else "d", "solid", {"kind": "extruded", "sweep": "w", "z0": -hw, "z1": hw})
    print(sc, o["id"], o["kind"], hw)
d.commit()
