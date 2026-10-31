import os, re, sys, tomllib
ROOT = "/Users/ueli/Documents/semio"
G = sys.argv[1]
rows = []
for line in open(os.path.join(G, "taxaudit-status-manifests.txt"), encoding="utf-8"):
    line = line.rstrip("\n")
    if line.strip():
        rows.append((line[:2].strip() or "?", line[3:]))

def load(p):
    fp = os.path.join(ROOT, p)
    if not os.path.isfile(fp):
        return None
    with open(fp, "rb") as f:
        return tomllib.load(f)

def glob_re(pat):
    out, i = "", 0
    while i < len(pat):
        if pat[i:i+3] == "**/":
            out += "(?:.*/)?"; i += 3
        elif pat[i:i+2] == "**":
            out += ".*"; i += 2
        elif pat[i] == "*":
            out += "[^/]*"; i += 1
        elif pat[i] == "?":
            out += "[^/]"; i += 1
        else:
            out += re.escape(pat[i]); i += 1
    return re.compile("^" + out + "$")

def ws_info(wsdir):
    d = load(os.path.join(wsdir, "Cargo.toml") if wsdir else "Cargo.toml")
    return d.get("workspace") if d and "workspace" in d else None

def member_state(wsdir, pkgdir):
    w = ws_info(wsdir)
    if w is None:
        return "NOWS"
    rel = os.path.relpath(pkgdir, wsdir) if wsdir else pkgdir
    m = any(glob_re(g).match(rel) for g in w.get("members", []))
    e = any(glob_re(g).match(rel) for g in w.get("exclude", []))
    if m and not e: return "MEMBER"
    if e: return "EXCLUDED"
    return "NOT-LISTED"

def nearest_ws(pkgdir):
    parts = pkgdir.split("/") if pkgdir else []
    for i in range(len(parts) - 1, -1, -1):
        cand = "/".join(parts[:i])
        if ws_info(cand) is not None:
            return cand
    if ws_info("") is not None:
        return ""
    return None

def effective_ws(pkgdir):
    parts = pkgdir.split("/") if pkgdir else []
    cands = []
    for i in range(len(parts), -1, -1):
        cand = "/".join(parts[:i])
        if ws_info(cand) is not None:
            cands.append(cand)
    for cand in cands:
        if member_state(cand, pkgdir) == "MEMBER":
            return cand, "MEMBER"
    if cands:
        return cands[0], "ORPHAN-OR-EXCLUDED-ALL:" + ",".join(f"{c or '.'}={member_state(c, pkgdir)}" for c in cands)
    return None, "NO-WORKSPACE-FOUND"

def lock_names(lockdir):
    fp = os.path.join(ROOT, lockdir, "Cargo.lock") if lockdir else os.path.join(ROOT, "Cargo.lock")
    if not os.path.isfile(fp):
        return None
    with open(fp, "rb") as f:
        L = tomllib.load(f)
    return {(p["name"], p.get("version"), p.get("source", "path")) for p in L.get("package", [])}, {p["name"] for p in L.get("package", [])}

for code, p in rows:
    pkgdir = os.path.dirname(p)
    t = load(p)
    own_ws = bool(t and "workspace" in t)
    pkg = (t or {}).get("package", {}).get("name")
    tests = []
    if t:
        for tt in t.get("test", []) or []:
            nm = tt.get("name", ""); pa = tt.get("path", "")
            tests.append(f"{nm}{'[NONASCII]' if not nm.isascii() else ''}->{pa}{'' if (not pa or os.path.isfile(os.path.join(ROOT, pkgdir, pa))) else '[MISSING]'}")
    if code == "D":
        print(f"{code}\t{pkg}\t{p}\tDELETED\t-\t-\t-\t-"); continue
    if own_ws:
        ws_root = pkgdir; state = "OWN-WORKSPACE"
    else:
        ws_root, state = effective_ws(pkgdir)
    lk = lock_names(ws_root) if ws_root is not None else None
    inlock = "NOLOCK"
    if lk is not None:
        names = lk[1]
        inlock = "IN-LOCK" if pkg in names else "MISSING-IN-LOCK"
    print("\t".join(map(str, [code, pkg, p, f"ws={ws_root!r}", state, f"lock={inlock}", f"len={len(p)}/{len(p.encode('utf-16-le'))//2}", ";".join(tests) or "-"])))
