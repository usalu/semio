"""👯️ Registers the TypeScript twin as a supplemental `cross-semio-implementation` reference, the
oracle of `👯️kernel-twin-parity`. The registry's own definition says such an entry can never
discharge a mutation's external-oracle requirement, which is exactly right: both kernels were written
here, from one specification, and every algorithm the case covers is separately adjudicated by d3."""
import io, json, os

os.chdir(os.path.join(os.environ["SEMIO_ROOT"], "🧰️framework/🛍️products/📓️print/🔮️oracle"))
doc = json.load(io.open("🔣️.json", encoding="utf-8"))
entry = {
    "id": "print-viz-kernel-twin",
    "kind": "cross-semio-implementation",
    "ecosystem": "javascript",
    "package": "@semio-tech/print-viz-kernel",
    "version": "workspace",
    "capabilities": ["viz-kernel-twin-parity"],
    "comparisonProfiles": ["viz-probe-v1", "viz-probe-coarse-v1", "viz-probe-exact-v1"],
    "license": "LGPL-3.0-or-later",
    "testOnly": True,
    "homepage": "https://github.com/usalu/semio",
    "rationale": "The second implementation of the semio visualization kernel, written in TypeScript against the same specification as the LaTeX packages and adjudicated module by module against d3 in its own harness. It is registered as a SUPPLEMENT, never as independent evidence: it shares this repository, this specification and these authors with the subject it is compared to. What it buys is the one comparison d3 cannot make - the two copies of the kernel drifting apart, including where d3 has no opinion.",
    "hostPath": "\u00a0",
    "engine": {"family": "semio", "implementation": "@semio-tech/print-viz-kernel", "version": "workspace"},
    "productionReachable": False,
    "networkDuringExecution": False,
}
del entry["hostPath"]
doc["oracles"] = [o for o in doc["oracles"] if o["id"] != entry["id"]] + [entry]
doc["oracles"].sort(key=lambda o: o["id"])
io.open("tmp.json", "w", encoding="utf-8", newline="\n").write(json.dumps(doc, ensure_ascii=False, indent=2) + "\n")
os.replace("tmp.json", "🔣️.json")
print("oracles", len(doc["oracles"]))
