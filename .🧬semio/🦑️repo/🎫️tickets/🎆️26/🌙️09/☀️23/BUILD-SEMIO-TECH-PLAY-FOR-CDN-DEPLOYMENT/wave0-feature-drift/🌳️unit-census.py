"""🌳️ Per-package `cargo build --unit-graph` census of the hub prebuild plan: for every compile unit key
(package, target, mode, platform; a build-script compile unit takes the platform of the run that executes it) it counts
the distinct feature sets and the distinct closure signatures (features + dependency signatures, recursively) across
the planned packages, each resolved alone as the prebuild's `CARGO_RESOLVER_FEATURE_UNIFICATION=package` and every
narrow `cargo build -p` does. No compilation.
usage: python3 🌳️unit-census.py <plan.json> <profile> <out.json>"""
import collections, hashlib, json, os, subprocess, sys
plan, profile, out = sys.argv[1], sys.argv[2], sys.argv[3]
ws = json.load(open(plan))["workspaces"][0]
env = dict(os.environ, CARGO_UNSTABLE_FEATURE_UNIFICATION="true", CARGO_RESOLVER_FEATURE_UNIFICATION="package")
features = collections.defaultdict(lambda: collections.defaultdict(set))
signatures, packages = collections.defaultdict(set), collections.defaultdict(set)
for pkg in ws["packages"]:
    run = subprocess.run(["cargo", "build", "--locked", "--offline", "--manifest-path", ws["manifest"], "-p", pkg, "--lib", "--target", "wasm32-wasip2", "--profile", profile, "-Z", "unstable-options", "--unit-graph"], env=env, capture_output=True, text=True)
    if run.returncode: sys.exit(f"{pkg}: {run.stderr[-2000:]}")
    units = json.loads(run.stdout)["units"]
    runs = collections.defaultdict(set)
    for u in units:
        if u["mode"] == "run-custom-build":
            for d in u["dependencies"]: runs[d["index"]].add(u["platform"] or "host")
    memo = {}
    def sig(i):
        if i not in memo:
            u = units[i]
            memo[i] = hashlib.sha256(json.dumps([u["pkg_id"], u["target"]["name"], u["mode"], u["platform"], u["features"], sorted(sig(d["index"]) for d in u["dependencies"])]).encode()).hexdigest()[:16]
        return memo[i]
    for i, u in enumerate(units):
        for platform in runs.get(i) or [u["platform"] or "host"]:
            key = f'{u["pkg_id"].split("#")[-1]} {u["target"]["name"]} {u["mode"]} {platform}'
            features[key][",".join(u["features"])].add(pkg); signatures[key].add(sig(i)); packages[key].add(pkg)
rows = {k: {"featureSets": {f: sorted(ps) for f, ps in sorted(features[k].items())}, "signatures": len(signatures[k]), "packages": len(packages[k])} for k in sorted(features)}
json.dump(rows, open(out, "w"), indent=1)
drift = {k: v for k, v in rows.items() if len(v["featureSets"]) > 1}
print(f"units={len(rows)} featureDrift={len(drift)} multiSignature={sum(1 for v in rows.values() if v['signatures'] > 1)} unitTotal={sum(v['signatures'] for v in rows.values())}")
for k, v in sorted(drift.items(), key=lambda kv: (-len(kv[1]["featureSets"]), kv[0])):
    print(f"{len(v['featureSets'])} sets {v['signatures']} sigs  {k}")
