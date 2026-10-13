"""🕸️ Counts compile units of a prebuild plan in ONE `cargo build --unit-graph` per feature-unification mode (no compilation):
total units, distinct (package, target, mode, platform) keys, duplicated keys, first-party units and library units that also
link a cdylib. usage: python3 🕸️unit-graph-census.py <plan.json> <profile>"""
import collections, json, os, subprocess, sys
plan, profile = sys.argv[1], sys.argv[2]
ws = json.load(open(plan))["workspaces"][0]
args = ["cargo", "build", "--locked", "--offline", "--manifest-path", ws["manifest"]] + sum([["-p", p] for p in ws["packages"]], []) + ["--lib", "--target", "wasm32-wasip2", "--profile", profile, "-Z", "unstable-options", "--unit-graph"]
for mode in ["package", "workspace"]:
    env = dict(os.environ, CARGO_UNSTABLE_FEATURE_UNIFICATION="true", CARGO_RESOLVER_FEATURE_UNIFICATION=mode)
    run = subprocess.run(args, env=env, capture_output=True, text=True)
    if run.returncode: print(mode, "error", run.stderr[-800:]); continue
    units = json.loads(run.stdout)["units"]
    keys = collections.Counter((u["pkg_id"].split("#")[-1], u["target"]["name"], u["mode"], u["platform"]) for u in units)
    dup = sorted(((k[0], n) for k, n in keys.items() if n > 1), key=lambda kv: -kv[1])
    cdylib = sorted(u["pkg_id"].split("#")[-1].split("@")[0] for u in units if "cdylib" in u["target"]["crate_types"] and u["pkg_id"].split("#")[-1].split("@")[0] not in ws["packages"])
    print(f"{mode}: units={len(units)} keys={len(keys)} firstParty={sum(1 for u in units if u['pkg_id'].startswith('path+'))} duplicated={dup} libraryCdylib={cdylib}")
