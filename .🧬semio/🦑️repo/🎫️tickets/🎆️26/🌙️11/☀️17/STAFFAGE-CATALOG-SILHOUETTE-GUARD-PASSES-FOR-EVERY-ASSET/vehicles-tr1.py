import sys, importlib, json
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/tram")
tol = {500: "0.03", 200: "0.012", 100: "0.008", 50: "0.005"}
def ids_of(sc, bone):
    return ",".join("=" + json.loads(d.t[v["s"]:v["e"]]) for it in d.at("asset", "volumes", str(sc), "body")["items"]
                    if json.loads(d.t[sp_child(it, "bone")["s"]:sp_child(it, "bone")["e"]]) == bone for k, v in it["pairs"] if k == "id")
sp = importlib.import_module("sil-span")
sp_child = sp.child
for sc in [int(a) for a in sys.argv[1:]]:
    end, mid = ids_of(sc, "body-end"), ids_of(sc, "body-mid")
    d.to_poly(sc, "elevation", "shell-end", end, tol[sc])
    d.to_poly(sc, "elevation", "shell-mid", mid, tol[sc], xf=(0, 0.3, 180))
    d.to_poly(sc, "plan", "shell-end", end, tol[sc])
    d.to_poly(sc, "plan", "shell-mid", mid, tol[sc])
d.commit()
