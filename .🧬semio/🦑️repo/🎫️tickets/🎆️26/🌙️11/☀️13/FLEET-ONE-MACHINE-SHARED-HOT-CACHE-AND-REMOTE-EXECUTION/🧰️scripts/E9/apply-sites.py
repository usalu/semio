import json, subprocess, os
H = os.path.dirname(os.path.abspath(__file__))
BUN = "/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
fx = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests/🧫️fixtures/🧭️sites.json"
live = json.loads(subprocess.run([BUN, "run", H + "/census.ts"], capture_output=True, text=True, errors="replace").stdout)
d = json.load(open(fx))
d["sites"] = live["sites"]; d["dynamic"]["files"] = live["dyn"]; d["unclassified"]["names"] = live["unclassified"]
open(fx, "w").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")
print(live["sites"], len(live["dyn"]), live["unclassified"])
