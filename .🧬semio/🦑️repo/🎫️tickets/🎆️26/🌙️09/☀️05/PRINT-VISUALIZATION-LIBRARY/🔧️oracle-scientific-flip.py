"""🔮️ Aligns the print oracle manifest with the scientific cases: registers the surveyed capability
of every flipped oracle, retires the four library-named no-oracle decisions the flips discharged,
and states the two remaining decisions in the schema's own substitute vocabulary."""
import json, os, io

os.chdir(os.path.join(os.environ["SEMIO_ROOT"], "🧰️framework/🛍️products/📓️print/🔮️oracle"))
with io.open("🔣️.json", encoding="utf-8") as handle:
    doc = json.load(handle)

caps = {
    "mathjs": ["viz-scientific-function-sampling"],
    "jstat": ["viz-scientific-distributions"],
    "fft-js": ["viz-scientific-signal"],
    "gl-matrix": ["viz-scientific-3d"],
}
for entry in doc["oracles"]:
    if entry["id"] in caps:
        entry["capabilities"] = caps[entry["id"]]

retired = {"mathjs", "jstat", "fft-js", "gl-matrix"}
kept = [entry for entry in doc["noOracleDecisions"] if entry["id"] not in retired]
for entry in kept:
    entry["substitutes"] = ["independent-implementations" if value == "independent-second-implementation" else value for value in entry["substitutes"]]
kept.append({
    "id": "projectile-integration",
    "capabilities": ["viz-scientific-physics"],
    "rationale": "No package in the registry integrates an ordinary differential equation: mathjs evaluates expressions and jStat samples distributions, and d3-force steps a damped velocity-Verlet simulation of a different problem. The velocity-Verlet projectile and the fourth-order Runge-Kutta oscillator are therefore adjudicated by an independent second implementation written in the adapter over the same initial conditions, step size and step count, sharing only the published formula with the l3fp subject, and by the closed-form parabola and analytic circle the same scenarios assert. Covers no runtime mutation.",
    "substitutes": ["independent-implementations", "specification-vectors"],
})
kept.append({
    "id": "survival-product-limit",
    "capabilities": ["viz-scientific-survival"],
    "rationale": "jStat carries distributions, not survival analysis: it has no product-limit estimator, no censoring vocabulary and no cumulative hazard, so it cannot adjudicate a Kaplan-Meier curve even though it adjudicates the distribution case. The estimator, its censoring rule and the Nelson-Aalen hazard are therefore checked against an independent second implementation written in the adapter from the published product-limit definition over the same event records. Covers no runtime mutation.",
    "substitutes": ["independent-implementations", "specification-vectors"],
})
doc["noOracleDecisions"] = sorted(kept, key=lambda entry: entry["id"])

with io.open("tmp-oracle.json", "w", encoding="utf-8", newline="\n") as handle:
    json.dump(doc, handle, ensure_ascii=False, indent=2)
    handle.write("\n")
os.replace("tmp-oracle.json", "🔣️.json")
print("noOracleDecisions", len(doc["noOracleDecisions"]))
