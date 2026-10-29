import json, os, re, sys, glob, fnmatch
try:
    import tomllib
except ImportError:
    import pip._vendor.tomli as tomllib

ROOT = "/Users/ueli/Documents/semio"
CASES = os.path.join(ROOT, "♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json")
CATALOG = os.path.join(ROOT, "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🧩️plugins/🟦️.ts")
OWNERS = os.path.join(ROOT, "📋️build-owners.json")

def load_toml(p):
    with open(p, "r", encoding="utf-8") as f:
        return tomllib.load(f)

def catalog_entries():
    src = open(CATALOG, encoding="utf-8").read()
    out = {}
    for m in re.finditer(r"\{\s*pluginId:\s*\"([^\"]+)\"(.*?)\}", src, re.S):
        body = m.group(2)
        cp = re.search(r"cratePath:\s*\"([^\"]+)\"", body)
        pn = re.search(r"packageName:\s*\"([^\"]+)\"", body)
        role = re.search(r"role:\s*\"([^\"]+)\"", body)
        if cp:
            out[m.group(1)] = dict(crate=cp.group(1), packageName=pn.group(1), role=role.group(1))
    return out

def find_ws_root(crate_dir):
    d = crate_dir
    while True:
        p = os.path.join(d, "Cargo.toml")
        if os.path.isfile(p) and p != os.path.join(crate_dir, "Cargo.toml"):
            t = load_toml(p)
            if "workspace" in t:
                return d, t
        parent = os.path.dirname(d)
        if parent == d:
            return None, None
        d = parent

def ws_member_status(ws_dir, ws_toml, target_dir):
    rel = os.path.relpath(target_dir, ws_dir)
    ws = ws_toml["workspace"]
    members = ws.get("members", [])
    excludes = ws.get("exclude", [])
    def match(patterns):
        for pat in patterns:
            if pat == rel or fnmatch.fnmatch(rel, pat):
                return True
            if pat.endswith("/*") and os.path.dirname(rel) == pat[:-2]:
                return True
        return False
    m = match(members)
    e = match(excludes)
    if e:
        return "excluded"
    if m:
        return "member"
    return "NOT-MEMBER"

def check_features(target_toml, feats):
    known = set((target_toml.get("features") or {}).keys())
    deps = {}
    for sec in ("dependencies", "build-dependencies"):
        for k, v in (target_toml.get(sec) or {}).items():
            if isinstance(v, dict) and v.get("optional"):
                deps[k] = True
    missing = []
    for f in feats:
        if f.startswith("dep:"):
            if f[4:] not in deps and f[4:] not in known:
                missing.append(f)
            continue
        base = f.split("/")[0].rstrip("?")
        if "/" in f:
            continue
        if f not in known and f not in deps:
            missing.append(f)
    return missing

def iter_dep_tables(t, ws_deps):
    for sec in ("dependencies", "dev-dependencies", "build-dependencies"):
        for k, v in (t.get(sec) or {}).items():
            yield sec, k, v
    for cfg, ct in (t.get("target") or {}).items():
        for sec in ("dependencies", "dev-dependencies", "build-dependencies"):
            for k, v in (ct.get(sec) or {}).items():
                yield f"target.{cfg}.{sec}", k, v

def owner_for(crate_rel, owners):
    best = None
    for o in owners:
        r = o["root"]
        if not o["manifest"].endswith("project.json"):
            continue
        if crate_rel == r or crate_rel.startswith(r + "/"):
            if best is None or len(r) > len(best["root"]):
                best = o
    return best

def audit_component(pid, entry, owners):
    problems = []
    crate_rel = entry["crate"]
    crate_dir = os.path.join(ROOT, crate_rel)
    row = dict(pid=pid, role=entry["role"], crate=crate_rel, manifest="", ws="", lock="", deps_checked=0, problems=problems)
    manifest = os.path.join(crate_dir, "Cargo.toml")
    if not os.path.isdir(crate_dir):
        problems.append(f"crate dir missing: {crate_rel}")
        return row
    if not os.path.isfile(manifest):
        problems.append("Cargo.toml missing")
        return row
    row["manifest"] = os.path.relpath(manifest, ROOT)
    t = load_toml(manifest)
    pkg = t.get("package", {})
    if pkg.get("name") != entry["packageName"]:
        problems.append(f"package name {pkg.get('name')} != catalog packageName {entry['packageName']}")
    # workspace resolution
    if "workspace" in pkg:
        ws_dir = os.path.normpath(os.path.join(crate_dir, pkg["workspace"]))
        ws_toml_path = os.path.join(ws_dir, "Cargo.toml")
        ws_toml = load_toml(ws_toml_path) if os.path.isfile(ws_toml_path) else None
        if ws_toml is None or "workspace" not in ws_toml:
            problems.append(f"package.workspace {pkg['workspace']} has no [workspace]")
            ws_dir = None
    else:
        ws_dir, ws_toml = find_ws_root(crate_dir)
    if ws_dir is None:
        problems.append("no enclosing workspace found")
    else:
        row["ws"] = os.path.relpath(ws_dir, ROOT)
        st = ws_member_status(ws_dir, ws_toml, crate_dir)
        if st == "NOT-MEMBER":
            problems.append(f"crate not a member of workspace {row['ws']}")
        elif st == "excluded":
            problems.append(f"crate excluded from workspace {row['ws']}")
        lock = os.path.join(ws_dir, "Cargo.lock")
        if os.path.isfile(lock):
            row["lock"] = os.path.relpath(lock, ROOT)
            lk = open(lock, encoding="utf-8").read()
            if f'name = "{pkg.get("name")}"' not in lk:
                problems.append(f"package {pkg.get('name')} absent from {row['lock']}")
        else:
            problems.append(f"no Cargo.lock at workspace {row['ws']}")
    ws_deps = (ws_toml or {}).get("workspace", {}).get("dependencies", {}) if ws_toml else {}
    # [lib]/[[bin]] paths
    lib = t.get("lib", {})
    if "path" in lib and not os.path.isfile(os.path.normpath(os.path.join(crate_dir, lib["path"]))):
        problems.append(f"[lib] path missing: {lib['path']}")
    for b in t.get("bin", []) or []:
        if "path" in b and not os.path.isfile(os.path.normpath(os.path.join(crate_dir, b["path"]))):
            problems.append(f"[[bin]] path missing: {b['path']}")
    # dependencies one level deep
    for sec, key, spec in iter_dep_tables(t, ws_deps):
        if isinstance(spec, str):
            continue
        row["deps_checked"] += 1
        dep_name = spec.get("package", key)
        dep_path = None
        target_toml = None
        if spec.get("workspace"):
            wspec = ws_deps.get(key)
            if wspec is None:
                problems.append(f"{sec}.{key}: workspace=true but not in [workspace.dependencies]")
                continue
            if isinstance(wspec, str):
                continue
            dep_name = wspec.get("package", dep_name)
            if "path" in wspec:
                dep_path = os.path.normpath(os.path.join(ws_dir, wspec["path"]))
            feats = list(spec.get("features", []))
            spec_eff = dict(wspec); spec_eff["features"] = list(wspec.get("features", [])) + feats
            spec = spec_eff
        elif "path" in spec:
            dep_path = os.path.normpath(os.path.join(crate_dir, spec["path"]))
        if dep_path is None:
            continue
        mf = os.path.join(dep_path, "Cargo.toml")
        if not os.path.isfile(mf):
            problems.append(f"{sec}.{key}: path missing {os.path.relpath(dep_path, ROOT)}/Cargo.toml")
            continue
        target_toml = load_toml(mf)
        tname = target_toml.get("package", {}).get("name")
        if tname != dep_name:
            problems.append(f"{sec}.{key}: package name {tname} != expected {dep_name}")
        if ws_dir is not None:
            st = ws_member_status(ws_dir, ws_toml, dep_path)
            inside = dep_path == ws_dir or dep_path.startswith(ws_dir + os.sep)
            if st == "NOT-MEMBER" and inside:
                problems.append(f"{sec}.{key}: path dep {os.path.relpath(dep_path, ROOT)} inside workspace {row['ws']} but not a member")
            if os.path.isfile(os.path.join(ws_dir or "", "Cargo.lock")) and f'name = "{tname}"' not in open(os.path.join(ws_dir, "Cargo.lock"), encoding="utf-8").read():
                problems.append(f"{sec}.{key}: dep {tname} absent from {row.get('lock','')}")
        feats = spec.get("features", []) if isinstance(spec, dict) else []
        if feats:
            miss = check_features(target_toml, feats)
            if miss:
                problems.append(f"{sec}.{key}: features not in {tname}: {', '.join(miss)}")
    # project.json component target
    row["projectJsons"] = []
    d = crate_dir
    for _ in range(6):
        for cand in glob.glob(os.path.join(d, "📋️project.json")):
            row["projectJsons"].append(os.path.relpath(cand, ROOT))
        d = os.path.dirname(d)
    o = owner_for(crate_rel, owners)
    row["owner"] = o["name"] if o else ""
    if o is None:
        problems.append("no project.json build owner ancestor in 📋️build-owners.json")
    else:
        mf = os.path.join(ROOT, o["manifest"])
        row["projectJson"] = o["manifest"]
        if not os.path.isfile(mf):
            problems.append(f"owner manifest missing: {o['manifest']}")
        else:
            try:
                pj = json.load(open(mf, encoding="utf-8"))
            except Exception as ex:
                problems.append(f"owner project.json unparseable: {o['manifest']} ({ex.msg})")
                return row
            tg = (pj.get("targets") or {})
            row["targets"] = sorted(tg.keys())
            comp = tg.get("component")
            if comp is None:
                problems.append("owner project.json has no `component` target")
            else:
                opts = comp.get("options", {})
                refs = []
                for k in ("command", "cwd", "main", "entry", "file", "path", "input", "output"):
                    if isinstance(opts.get(k), str):
                        refs.append(opts[k])
                for k in ("outputs",):
                    refs += [x for x in comp.get(k, []) if isinstance(x, str)]
                for r in refs:
                    if "{" in r or "*" in r or "/" not in r and "." not in r:
                        continue
                    cand = os.path.normpath(os.path.join(os.path.dirname(mf), r))
                    if not os.path.exists(cand):
                        problems.append(f"component target ref missing: {r}")
    return row

def main():
    cases = json.load(open(CASES, encoding="utf-8"))
    ids = [("plugin", x) for x in cases["pluginIds"]] + [("extension", x) for x in cases["extensionIds"]]
    cat = catalog_entries()
    owners = json.load(open(OWNERS, encoding="utf-8"))["owners"]
    rows = []
    for kind, pid in ids:
        if pid not in cat:
            rows.append(dict(pid=pid, role=kind, crate="", manifest="", problems=["NOT IN CATALOG"]))
            continue
        rows.append(audit_component(pid, cat[pid], owners))
    out = os.path.join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1028/manifestaudit-rows.json")
    json.dump(rows, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    for r in rows:
        print(r["pid"], "|", r.get("crate",""), "|", r.get("manifest",""), "|", r.get("ws",""), "| deps", r.get("deps_checked",0), "| owner", r.get("owner",""), "|", "; ".join(r["problems"]) or "OK")

main()
