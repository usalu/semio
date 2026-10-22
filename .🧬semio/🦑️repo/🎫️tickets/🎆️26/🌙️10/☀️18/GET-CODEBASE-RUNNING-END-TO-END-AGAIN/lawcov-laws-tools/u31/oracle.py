CLOSURES = "🧰framework/🛍products/🦑repo/🔨modules/📚library/🏗build/📥source/🛗zero-touch/🗂closures/🔣.json"
BUILTINS = {"fs", "path", "os", "crypto", "util", "url", "child_process", "events", "stream", "http", "zlib", "assert", "buffer", "readline", "module", "worker_threads"}
EXTS = [".ts", ".tsx", ".mts", ".js", ".mjs"]
CODE = re.compile(r"\.(?:[cm]?[jt]s|tsx)$")


def specifiers(text):
    found = {}
    for m in re.finditer(r"""(?:^|[\s;{}(])(?:import|export)\b[^'"`;]*?\bfrom\s*['"]([^'"]+)['"]""", text, re.M):
        found[("static", m.group(1))] = 1
    for m in re.finditer(r"""(?:^|[\s;{}(])import\s*['"]([^'"]+)['"]""", text, re.M):
        found[("static", m.group(1))] = 1
    for m in re.finditer(r"""\bimport\(\s*['"]([^'"]+)['"]\s*\)""", text):
        found[("dynamic", m.group(1))] = 1
    for m in re.finditer(r"""\brequire\(\s*['"]([^'"]+)['"]\s*\)""", text):
        found[("require", m.group(1))] = 1
    return sorted(found, key=lambda kv: (kv[1].encode(), kv[0].encode()))


def relative_target(t, frm, spec):
    base = posixpath.normpath(posixpath.join(posixpath.dirname(frm), spec.split("?")[0]))
    for cand in [base] + [base + e for e in EXTS]:
        if CODE.search(cand) and cand in t.files:
            return cand
    return None


def check(t):
    roster = json.loads(t.files[CLOSURES])
    pkg = json.loads(t.files["package.json"])
    packages = {}
    for d in pkg.get("workspaces", []):
        p = d + "/package.json"
        if p in t.files:
            j = json.loads(t.files[p])
            if isinstance(j.get("name"), str) and "exports" in j:
                packages[j["name"]] = (d, {".": j["exports"]} if isinstance(j["exports"], str) else j["exports"])
    seams = {s["module"] for s in roster["seams"]}
    out = []

    def add(closure, kind, file, spec):
        out.append(["zero-touch-%s-%s-%s-%s" % (closure, kind, file, spec), "repo/zero-touch-closure", "high", False, file])

    for s in roster["seams"]:
        if s["module"] not in t.files:
            add("seams", "missing-entry", s["module"], s["module"])
    for c in roster["closures"]:
        seen, pending = set(), [c["entry"]]
        if c["entry"] not in t.files:
            add(c["id"], "missing-entry", c["entry"], c["entry"])
        while pending:
            f = pending.pop()
            if f in seen:
                continue
            seen.add(f)
            if f not in t.files:
                continue
            for kind, spec in specifiers(t.files[f]):
                if c["mode"] == "static" and kind != "static":
                    continue
                if spec.startswith("."):
                    target = relative_target(t, f, spec)
                    if target is None:
                        continue
                    if target not in seams:
                        pending.append(target)
                    elif kind == "static":
                        add(c["id"], "seam-static", f, spec)
                elif spec.startswith(("node:", "bun:")) or spec == "bun" or spec in BUILTINS:
                    continue
                elif any(spec == n or spec.startswith(n + "/") for n in packages):
                    add(c["id"], "workspace-specifier", f, spec)
                else:
                    add(c["id"], "external-package", f, spec)
    return out


def run(c):
    return sorted([[r[0], r[1], r[2], r[3], r[4]] for r in check(Tree(expand(c)))], key=lambda r: json.dumps(r, ensure_ascii=False, separators=(",", ":")).encode("utf-16-be"))


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})
