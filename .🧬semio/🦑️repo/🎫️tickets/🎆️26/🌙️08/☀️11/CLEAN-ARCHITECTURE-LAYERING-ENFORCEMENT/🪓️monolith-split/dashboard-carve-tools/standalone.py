"""🔎 Standalone resolution check: each crate compiled as its own root; every error that is not about a missing external or dependency crate is listed."""
import collections, json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import names as n
import oracle as o
import spec

dashboard_dir, tag = sys.argv[1], sys.argv[2]
dest = f"{o.SCR}/D-{tag}"
o.make_sandbox(dest, dashboard_dir)
o.patch_tree(dest)
base = os.path.join(dest, o.DASH)
externs = {"serde", "serde_json", "ui_locale", "ui_tui", "ui_styling"} | {k for k in spec.CRATES} | {n_.replace("-", "_") for n_ in spec.REPO_SIBLINGS}
roots = {key: os.path.join(base, *spec.CRATES[key][0], n.RS) for key in spec.CRATES}
roots["facade"] = os.path.join(base, n.PKG, n.RUST, n.RS)
for key, root in roots.items():
    for cfg, extra in (("native", []), ("test", ["--test"])):
        cmd = ["rustc", o.TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", "probe", "--emit=metadata", "-o", "/dev/null", "--error-format=json", *extra, root]
        proc = subprocess.run(cmd, cwd=os.path.dirname(root), capture_output=True, text=True)
        rows = collections.Counter()
        for line in proc.stderr.splitlines():
            try: d = json.loads(line)
            except Exception: continue
            if d.get("level") != "error" or d["message"].startswith("aborting"): continue
            msg = d["message"]
            m = re.search(r"`(\w+)`", msg)
            code = (d.get("code") or {}).get("code") or ""
            if code in ("E0432", "E0433") and m and m.group(1) in externs: continue
            if msg.startswith("cannot find attribute `serde`"): continue
            span = next((s for s in d.get("spans", []) if s.get("is_primary")), None)
            where = os.path.relpath(span["file_name"], base) if span else ""
            rows[(code, msg[:100], where.split("/")[-2] if where else "")] += 1
        print(f"{key:24s} {cfg:6s} non-extern errors: {sum(rows.values())}")
        for (code, msg, where), count in sorted(rows.items()):
            print("    ", count, code, msg, where)
