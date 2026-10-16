"""🔎 Per-crate dependency completeness: each crate compiled as its own root, the set of unresolved external crates equals the declared dependencies (analysis only)."""
import json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import names as n
import oracle as o
import spec

dashboard_dir, tag = sys.argv[1], sys.argv[2]
dest = f"{o.SCR}/D-{tag}"
o.make_sandbox(dest, dashboard_dir)
o.patch_tree(dest)
base = os.path.join(dest, o.DASH)

def unresolved(root, extra):
    cmd = ["rustc", o.TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", "probe", "--emit=metadata", "-o", "/dev/null", "--error-format=json", *extra, root]
    proc = subprocess.run(cmd, cwd=os.path.dirname(root), capture_output=True, text=True)
    names, others = set(), []
    for line in proc.stderr.splitlines():
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("level") != "error":
            continue
        message = d["message"]
        m = re.search(r"cannot find module or crate `(\w+)`", message) or re.search(r"unresolved import `(\w+)`", message) or re.search(r"cannot find attribute `(\w+)`", message)
        if m:
            names.add(m.group(1))
        elif not message.startswith("aborting"):
            others.append(message[:140])
    return names, others

def declared(key):
    normal, dev = spec.DEPS[key]
    conv = lambda name: name.replace("+testkit", "").replace("-", "_")
    return {conv(x) for x in normal}, {conv(x) for x in dev}

roots = {key: os.path.join(base, *spec.CRATES[key][0], n.RS) for key in spec.CRATES}
roots["facade"] = os.path.join(base, n.PKG, n.RUST, n.RS)
bad = 0
for key, root in roots.items():
    nat, nat_other = unresolved(root, [])
    tst, tst_other = unresolved(root, ["--test"])
    if key == "facade":
        normal, dev = {x.replace("-", "_") for x in spec.FACADE_DEPS}, set()
    else:
        normal, dev = declared(key)
    missing_native = nat - normal
    missing_test = tst - normal - dev
    unused = (normal | dev) - nat - tst
    unused_dev_only = dev - tst
    status = "ok" if not (missing_native or missing_test or unused or unused_dev_only) else "DIFF"
    if status != "ok":
        bad += 1
    print(f"{key:24s} {status} native={sorted(nat)} test-extra={sorted(tst - nat)}")
    if missing_native: print("   MISSING (native):", sorted(missing_native))
    if missing_test: print("   MISSING (test):", sorted(missing_test))
    if unused: print("   UNUSED declared:", sorted(unused))
    if unused_dev_only: print("   dev never used in tests:", sorted(unused_dev_only))
    for other in sorted(set(nat_other + tst_other))[:8]:
        print("   other error:", other)
print("DIFFS:", bad)
