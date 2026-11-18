import sys, importlib, json, subprocess
sys.path.insert(0, ".")
m = importlib.import_module("vehicles-lib")
d = m.V("🚜️vehicles/cargo-bike")
BONE = {k: tuple(v) for k, v in json.loads(subprocess.run(["./vehicles-run.sh", "bones", "cargo-bike", "50"], capture_output=True, text=True).stdout.strip().splitlines()[-1]).items()}
tol = {50: "0.004", 100: "0.006", 200: "0.005", 500: "0.012"}
skip = tuple(f"{p}-{s}" for p in ("tyre", "rim", "hub", "rotor", "rotor-carrier", "flange", "axle", "spoke") for s in ("r", "f", "r-a", "r-b", "f-a", "f-b", "r-1", "r-2", "r-3", "r-4", "f-1", "f-2", "f-3", "f-4"))
sc = int(sys.argv[1])
for v in sys.argv[2].split(","):
    print(sc, v, d.add_missing(sc, v, "frame", tol[sc], skip=skip, xf=BONE["frame"]))
d.commit()
