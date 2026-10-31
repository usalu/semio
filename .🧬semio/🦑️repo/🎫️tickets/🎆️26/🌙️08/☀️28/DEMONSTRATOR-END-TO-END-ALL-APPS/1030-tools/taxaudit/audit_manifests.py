import os, re, sys, tomllib, json, fnmatch
ROOT = "/Users/ueli/Documents/semio"
G = sys.argv[1]
rows = []
for line in open(os.path.join(G, "taxaudit-status-manifests.txt"), encoding="utf-8"):
    line = line.rstrip("\n")
    if not line.strip():
        continue
    code, path = line[:2], line[3:]
    rows.append((code.strip() or "?", path))

def load(p):
    try:
        with open(os.path.join(ROOT, p), "rb") as f:
            return tomllib.load(f)
    except FileNotFoundError:
        return None

def glob_re(pat):
    out = ""
    i = 0
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

# discover workspace roots among tracked, non-ticket manifests via git ls-files list
ls = [l.rstrip("\n") for l in open(os.path.join(G, "taxaudit-ls-nontickets.txt"), encoding="utf-8")]
manifests_all = [p for p in ls if p.endswith("Cargo.toml")]
ws = {}
for p in ["Cargo.toml", "✏️s/Cargo.toml", "🧰️framework/Cargo.toml"]:
    d = load(p)
    ws[os.path.dirname(p)] = d
def ancestors(p):
    parts = os.path.dirname(p).split("/") if os.path.dirname(p) else []
    out = []
    for i in range(len(parts), -1, -1):
        out.append("/".join(parts[:i]))
    return out

def registrations(manifest_dir):
    res = []
    for wsdir, d in ws.items():
        if d is None or "workspace" not in d:
            continue
        if not (manifest_dir == wsdir or manifest_dir.startswith(wsdir + "/") or wsdir == ""):
            continue
        rel = manifest_dir[len(wsdir)+1:] if wsdir and manifest_dir.startswith(wsdir + "/") else manifest_dir
        if wsdir == "" and rel == "":
            continue
        members = d["workspace"].get("members", [])
        excl = d["workspace"].get("exclude", [])
        m = any(glob_re(g).match(rel) for g in members)
        e = any(glob_re(g).match(rel) for g in excl)
        res.append((wsdir or ".", m and not e, m, e))
    return res

print("STATUS\tPKG\tDIR\tWS(dir:member_ok)\tTESTS\tLEN_CHARS\tLEN_UTF16\tMAXFILE_CHARS\tPROJECT_JSON")
for code, p in rows:
    d = os.path.dirname(p)
    t = load(p) if code != "D" else None
    pkg = (t or {}).get("package", {}).get("name") if t else None
    wsdata = registrations(d) if d else []
    regs = ";".join(f"{a}:{'Y' if b else ('EXCL' if e else ('N' if m is False else '?'))}" for a, b, m, e in wsdata)
    tests = []
    if t:
        for tt in t.get("test", []) or []:
            nm = tt.get("name", "")
            pa = tt.get("path", "")
            ascii_ok = all(ord(c) < 128 for c in nm)
            exists = os.path.exists(os.path.join(ROOT, d, pa)) if pa else None
            tests.append(f"{nm}{'' if ascii_ok else '[NONASCII]'}->{pa}{'' if exists in (True, None) else '[MISSING]'}")
    maxlen = 0; maxfile = ""
    if code != "D" and d and os.path.isdir(os.path.join(ROOT, d)):
        for base, dirs, files in os.walk(os.path.join(ROOT, d)):
            dirs[:] = [x for x in dirs if x not in ("target", "node_modules", "dist", ".git")]
            for f in files:
                fp = os.path.relpath(os.path.join(base, f), ROOT)
                if len(fp) > maxlen:
                    maxlen, maxfile = len(fp), fp
    pj = os.path.exists(os.path.join(ROOT, d, "📋️project.json")) if d else None
    print("\t".join(map(str, [code, pkg, p, regs or "NO-WS", " | ".join(tests) or "-", len(p), len(p.encode('utf-16-le'))//2, f"{maxlen}", pj])))
