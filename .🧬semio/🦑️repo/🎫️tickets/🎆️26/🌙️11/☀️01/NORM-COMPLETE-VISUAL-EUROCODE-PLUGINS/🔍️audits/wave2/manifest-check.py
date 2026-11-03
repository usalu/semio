import fnmatch, json, os, re, sys, tomllib
ROOT = "/home/deck/semio"
NORM = os.path.join(ROOT, "✏️s/🔌️plugins/📕️norm")
SKIP = {"node_modules", "target", ".cache", ".git"}

def walk(base, name):
    for dp, dn, fn in os.walk(base):
        dn[:] = [d for d in dn if d not in SKIP]
        for f in fn:
            if f == name:
                yield os.path.join(dp, f)

def lines_of(p):
    with open(p, encoding="utf-8", errors="replace") as h:
        return h.read().split("\n")

def find_line(p, needle):
    for i, l in enumerate(lines_of(p), 1):
        if needle in l:
            return i
    return 0

def rel(p):
    return os.path.relpath(p, ROOT)

def dep_paths(tbl, out, prefix):
    if not isinstance(tbl, dict):
        return
    for k, v in tbl.items():
        if isinstance(v, dict):
            if "path" in v and isinstance(v["path"], str):
                out.append((prefix + k, v["path"]))
            else:
                dep_paths(v, out, prefix)

def collect_path_deps(m):
    out = []
    for sec in ("dependencies", "dev-dependencies", "build-dependencies"):
        dep_paths(m.get(sec, {}), out, sec + ":")
    for t, tv in m.get("target", {}).items():
        for sec in ("dependencies", "dev-dependencies", "build-dependencies"):
            dep_paths(tv.get(sec, {}), out, f"target.{t}.{sec}:")
    dep_paths(m.get("workspace", {}).get("dependencies", {}), out, "workspace:")
    return out

manifests = sorted(walk(NORM, "Cargo.toml"))
report = {"crates": [], "problems": [], "path_attrs": [], "owners": [], "deps_json": []}
P = report["problems"]
parsed = {}
for mp in manifests:
    try:
        with open(mp, "rb") as h:
            parsed[mp] = tomllib.load(h)
    except Exception as e:
        P.append({"kind": "toml-parse", "where": f"{rel(mp)}:1", "msg": str(e)})

def nearest_workspace(d):
    cur = d
    while True:
        parent = os.path.dirname(cur)
        if parent == cur or not parent.startswith(ROOT):
            return None
        cur = parent
        cand = os.path.join(cur, "Cargo.toml")
        if cand in parsed and "workspace" in parsed[cand]:
            return cand

def is_member(ws_path, crate_dir):
    ws = parsed[ws_path]["workspace"]
    wsdir = os.path.dirname(ws_path)
    r = os.path.relpath(crate_dir, wsdir).replace(os.sep, "/")
    members = ws.get("members", [])
    excl = ws.get("exclude", [])
    m = any(r == x or fnmatch.fnmatch(r, x) for x in members)
    e = any(r == x or fnmatch.fnmatch(r, x) for x in excl)
    return m and not e, r

for mp, m in parsed.items():
    d = os.path.dirname(mp)
    if "package" not in m:
        continue
    name = m["package"].get("name")
    row = {"manifest": rel(mp), "name": name, "dir": rel(d)}
    row["has_project_json"] = os.path.isfile(os.path.join(d, "📋️project.json"))
    row["has_script_ts"] = os.path.isfile(os.path.join(d, "📜️script.ts"))
    ws = nearest_workspace(d)
    row["workspace"] = rel(ws) if ws else None
    if ws:
        ok, r = is_member(ws, d)
        row["member"] = ok
        if not ok:
            P.append({"kind": "not-workspace-member", "where": f"{rel(mp)}:1", "msg": f"dir '{r}' not a member/or excluded in {rel(ws)}"})
    else:
        row["member"] = None
        P.append({"kind": "no-workspace", "where": f"{rel(mp)}:1", "msg": "no ancestor workspace manifest"})
    for sec, p in collect_path_deps(m):
        target = os.path.normpath(os.path.join(d, p))
        tm = os.path.join(target, "Cargo.toml")
        if not os.path.isfile(tm):
            ln = find_line(mp, p)
            P.append({"kind": "missing-path-dep", "where": f"{rel(mp)}:{ln}", "msg": f"{sec} '{p}' -> no {rel(tm)}"})
    report["crates"].append(row)

# Also check the root workspace path deps (they resolve relative to the workspace dir)
for mp, m in parsed.items():
    if "workspace" in m and "package" not in m:
        d = os.path.dirname(mp)
        for sec, p in collect_path_deps(m):
            if sec.startswith("workspace:") or True:
                target = os.path.normpath(os.path.join(d, p))
                if not os.path.isfile(os.path.join(target, "Cargo.toml")):
                    ln = find_line(mp, p)
                    if not any(x["where"] == f"{rel(mp)}:{ln}" for x in P):
                        P.append({"kind": "missing-path-dep", "where": f"{rel(mp)}:{ln}", "msg": f"{sec} '{p}' -> no Cargo.toml"})

# workspace members that do not exist
for mp, m in parsed.items():
    if "workspace" in m:
        d = os.path.dirname(mp)
        for mem in m["workspace"].get("members", []):
            if not os.path.isfile(os.path.join(d, mem, "Cargo.toml")):
                P.append({"kind": "missing-member", "where": f"{rel(mp)}:{find_line(mp, mem)}", "msg": f"member '{mem}' has no Cargo.toml"})

# #[path] attributes, tracking inline-module base directories
PATH_RE = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]')
MOD_INLINE = re.compile(r'^\s*(pub(\([^)]*\))?\s+)?mod\s+\w+\s*\{')
MOD_FILE = re.compile(r'^\s*(pub(\([^)]*\))?\s+)?mod\s+\w+\s*;')
def strip(l):
    l = re.sub(r'"(\\.|[^"\\])*"', '""', l)
    return l.split("//")[0]
for dp, dn, fn in os.walk(NORM):
    dn[:] = [x for x in dn if x not in SKIP]
    for f in fn:
        if not f.endswith(".rs"):
            continue
        fp = os.path.join(dp, f)
        lines = lines_of(fp)
        stack = []  # (depth_when_opened, base_dir)
        depth = 0
        base = dp
        pending = None  # (pathstr, lineno)
        for i, l in enumerate(lines, 1):
            m = PATH_RE.search(l)
            if m:
                pending = (m.group(1), i)
                continue
            s = strip(l)
            if pending and s.strip() and not s.strip().startswith("#"):
                p, pl = pending
                pending = None
                if MOD_INLINE.match(s):
                    tgt = os.path.normpath(os.path.join(base, p))
                    ok = os.path.isdir(tgt)
                    report["path_attrs"].append({"where": f"{rel(fp)}:{pl}", "target": p, "kind": "inline-dir", "ok": ok})
                    if not ok:
                        P.append({"kind": "missing-path-attr", "where": f"{rel(fp)}:{pl}", "msg": f"inline #[path = \"{p}\"] -> dir missing"})
                    stack.append((depth, base))
                    base = tgt
                elif MOD_FILE.match(s):
                    tgt = os.path.normpath(os.path.join(base, p))
                    ok = os.path.isfile(tgt)
                    report["path_attrs"].append({"where": f"{rel(fp)}:{pl}", "target": p, "kind": "file", "ok": ok})
                    if not ok:
                        P.append({"kind": "missing-path-attr", "where": f"{rel(fp)}:{pl}", "msg": f"#[path = \"{p}\"] -> file missing"})
            o_ = s.count("{"); c_ = s.count("}")
            depth += o_ - c_
            while stack and depth <= stack[-1][0]:
                base = stack.pop()[1]

# project.json files under norm and their script/cwd
pj_names = {}
for pj in walk(NORM, "📋️project.json"):
    try:
        j = json.load(open(pj, encoding="utf-8"))
    except Exception as e:
        P.append({"kind": "project-json-parse", "where": f"{rel(pj)}:1", "msg": str(e)}); continue
    pj_names[j.get("name")] = pj
    for tname, t in j.get("targets", {}).items():
        cwd = t.get("options", {}).get("cwd")
        if cwd and not os.path.isdir(os.path.join(ROOT, cwd)):
            P.append({"kind": "project-cwd-missing", "where": f"{rel(pj)}:1", "msg": f"target {tname} cwd '{cwd}' missing"})
        cmd = t.get("options", {}).get("command", "")
        mm = re.search(r"repo \./(\S+)", cmd)
        if mm and cwd and not os.path.isfile(os.path.join(ROOT, cwd, mm.group(1))):
            P.append({"kind": "project-script-missing", "where": f"{rel(pj)}:1", "msg": f"target {tname} script '{mm.group(1)}' missing in cwd"})

# crate-level: crates with project.json vs script.ts / build-owners
bo = json.load(open(os.path.join(ROOT, "📋️build-owners.json"), encoding="utf-8"))
owner_manifests = {o["manifest"]: o for o in bo["owners"]}
for o in bo["owners"]:
    if "📕️norm" not in o["root"]:
        continue
    rr = {"name": o["name"], "root": o["root"], "root_ok": os.path.isdir(os.path.join(ROOT, o["root"])), "manifest_ok": os.path.isfile(os.path.join(ROOT, o["manifest"]))}
    report["owners"].append(rr)
    if not rr["root_ok"]:
        P.append({"kind": "owner-root-missing", "where": "📋️build-owners.json", "msg": f"owner {o['name']} root '{o['root']}' missing"})
    if not rr["manifest_ok"]:
        P.append({"kind": "owner-manifest-missing", "where": "📋️build-owners.json", "msg": f"owner {o['name']} manifest '{o['manifest']}' missing"})
    if rr["manifest_ok"] and o["manifest"].endswith(".json"):
        try:
            j = json.load(open(os.path.join(ROOT, o["manifest"]), encoding="utf-8"))
            if j.get("name") != o["name"]:
                P.append({"kind": "owner-name-mismatch", "where": "📋️build-owners.json", "msg": f"owner name '{o['name']}' != project.json name '{j.get('name')}' at {o['manifest']}"})
        except Exception as e:
            P.append({"kind": "owner-manifest-parse", "where": "📋️build-owners.json", "msg": str(e)})
for r in report["crates"]:
    if r["has_project_json"]:
        man = os.path.join(r["dir"], "📋️project.json")
        if man not in owner_manifests and os.path.join(ROOT, man) not in owner_manifests:
            report.setdefault("crates_with_project_not_owned", []).append(r["manifest"])

# dependencies.json
dj = json.load(open(os.path.join(ROOT, "🔒️dependencies.json"), encoding="utf-8"))
for e in dj["entries"]:
    txt = json.dumps(e, ensure_ascii=False)
    if "📕️norm" in txt or (e.get("name", "").startswith("semio-s-") or e.get("name", "").startswith("@semio-tech/norm") or e.get("name", "").startswith("@semio-s/plugin-norm")):
        for u in e.get("users", []):
            if "📕️norm" in u and not os.path.exists(os.path.join(ROOT, u)):
                P.append({"kind": "deps-user-missing", "where": "🔒️dependencies.json", "msg": f"{e['name']} user '{u}' missing"})
        report["deps_json"].append({"name": e.get("name"), "ecosystem": e.get("ecosystem"), "users": e.get("users")})
for u in []:
    pass
# also check dependencies that name norm crates in users
json.dump(report, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print("crates", len(report["crates"]), "members-ok", sum(1 for c in report["crates"] if c["member"]), "project", sum(c["has_project_json"] for c in report["crates"]), "script", sum(c["has_script_ts"] for c in report["crates"]))
print("path attrs", len(report["path_attrs"]), "missing", sum(1 for a in report["path_attrs"] if not a["ok"]))
print("owners norm", len(report["owners"]))
print("deps_json norm-related", len(report["deps_json"]))
print("problems", len(P))
from collections import Counter
print(Counter(p["kind"] for p in P))
for p in P[:80]:
    print(p["kind"], p["where"], p["msg"])
