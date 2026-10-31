import os, re, sys, tomllib
ROOT = "/Users/ueli/Documents/semio"
G = sys.argv[1]
def load(p):
    fp = os.path.join(ROOT, p)
    return tomllib.load(open(fp, "rb")) if os.path.isfile(fp) else None
def gre(pat):
    out, i = "", 0
    while i < len(pat):
        if pat[i:i+3] == "**/": out += "(?:.*/)?"; i += 3
        elif pat[i:i+2] == "**": out += ".*"; i += 2
        elif pat[i] == "*": out += "[^/]*"; i += 1
        elif pat[i] == "?": out += "[^/]"; i += 1
        else: out += re.escape(pat[i]); i += 1
    return re.compile("^" + out + "$")
def ws_file(cand):
    return os.path.join(cand, "Cargo.toml") if cand else "Cargo.toml"
def is_member(cand, d, pkgdir):
    rel = os.path.relpath(pkgdir, cand) if cand else pkgdir
    w = d["workspace"]
    return any(gre(g).match(rel) for g in w.get("members", [])) and not any(gre(g).match(rel) for g in w.get("exclude", []))
def effective(pkgdir, t):
    if "workspace" in t:
        return pkgdir, t
    parts = pkgdir.split("/") if pkgdir else []
    for i in range(len(parts) - 1, -1, -1):
        cand = "/".join(parts[:i])
        d = load(ws_file(cand))
        if d and "workspace" in d and is_member(cand, d, pkgdir):
            return cand, d
    return None, None
rows = [l.rstrip("\n") for l in open(os.path.join(G, "taxaudit-status-manifests.txt"), encoding="utf-8") if l.strip()]
for r in rows:
    code, p = r[:2].strip(), r[3:]
    if code == "D": continue
    pkgdir = os.path.dirname(p)
    t = load(p)
    if not t or "package" not in t: continue
    wsd, wsdata = effective(pkgdir, t)
    wsdeps = (wsdata or {}).get("workspace", {}).get("dependencies", {}) if wsdata else {}
    issues = []
    for sect in ("lib", "bin", "test", "bench", "example"):
        vals = t.get(sect)
        if vals is None: continue
        for v in (vals if isinstance(vals, list) else [vals]):
            pa = v.get("path")
            if pa and not os.path.isfile(os.path.normpath(os.path.join(ROOT, pkgdir, pa))):
                issues.append(f"[{sect}] path missing: {pa}")
            nm = v.get("name", "")
            if sect in ("test", "bin", "bench", "example") and nm and not nm.isascii():
                issues.append(f"[{sect}] NON-ASCII name {nm}")
    for sect in ("dependencies", "dev-dependencies", "build-dependencies"):
        for name, spec in (t.get(sect) or {}).items():
            if not isinstance(spec, dict): continue
            if spec.get("workspace") is True and name not in wsdeps:
                issues.append(f"[{sect}] '{name}' workspace=true but absent from workspace.dependencies of {wsd!r}")
            if "path" in spec:
                dp = os.path.normpath(os.path.join(ROOT, pkgdir, spec["path"]))
                if not os.path.isfile(os.path.join(dp, "Cargo.toml")):
                    issues.append(f"[{sect}] path dep '{name}' -> {spec['path']} has no Cargo.toml")
    print(f"{code}\t{t['package']['name']}\t{p}\tws={wsd!r}\t{' | '.join(issues) if issues else 'OK'}")
