#!/usr/bin/env python3
"""📦️ Wave 3 (split plan 3d): every consumer manifest depends on exactly the semio crates its sources name; every workspace
that defines the aggregator defines those crates too."""
import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from ownership import CRATE, crate_roots
from mount_graph import walk
from use_rewrite import _mask

roots = crate_roots()
PKG = {name: (os.path.dirname(roots[k]) + "/📦️packages/🦀️rust" if k != "agg" else SEMIO + "/📦️packages/🦀️rust") for k, name in CRATE.items()}
IDENT = {name.replace("-", "_"): name for name in CRATE.values()}
SEMIO_WS = SEMIO + "/Cargo.toml"
DEP_RX = re.compile(r'(?m)^semio-s-artifact-stdio-semio = (\{[^}\n]*\})[ \t]*$')
manifests = [p for p in subprocess.run(["rg", "-l", r"semio-s-artifact-stdio-semio\b", "-g", "Cargo.toml", "-g", "!.🧬semio/**", "-g", "!**/target/**", REPO], capture_output=True, text=True).stdout.split()]
manifests = [p for p in manifests if not (p.startswith(SEMIO + "/") and os.path.dirname(p) in PKG.values()) and p != SEMIO_WS and not p.startswith(REPO + "/🌎️hub/🧩️compositions/🗄️stdio/")]
DONE = set(json.load(open(__file__.rsplit("/", 1)[0] + "/consumer-manifests.json"))) if os.path.exists(__file__.rsplit("/", 1)[0] + "/consumer-manifests.json") else set()


def sources(manifest):
    d = os.path.dirname(manifest)
    data = toml_loads(read(manifest))
    entries = []
    lib = data.get("lib", {})
    if "package" in data:
        entries.append(os.path.join(d, lib.get("path", "src/lib.rs")))
        for kind in ("bin", "test", "example", "bench"):
            for t in data.get(kind, []):
                if "path" in t:
                    entries.append(os.path.join(d, t["path"]))
        for default in ("src/main.rs",):
            entries.append(os.path.join(d, default))
    files = set()
    for e in entries:
        e = os.path.normpath(e)
        if os.path.exists(e):
            files.update(f for f in walk(e) if os.path.exists(f))
    return files, data


def used(files):
    out = set()
    for f in files:
        m = _mask(read(f))
        for ident in re.findall(r"(?<![\w:])(semio_s_artifact_stdio_semio(?:_[a-z_]+)?)(?=::|!|;)", m):
            if ident in IDENT:
                out.add(IDENT[ident])
    return out


def workspace_root(manifest, data):
    rel = data.get("package", {}).get("workspace")
    if rel:
        return os.path.normpath(os.path.join(os.path.dirname(manifest), rel, "Cargo.toml"))
    d = os.path.dirname(manifest)
    while d.startswith(REPO):
        c = os.path.join(d, "Cargo.toml")
        if os.path.exists(c) and re.search(r"(?m)^\[workspace\]", read(c)):
            return c
        d = os.path.dirname(d)
    return None


ws_needs = {}
report = {}
for m in manifests:
    text = read(m)
    sec = None
    package_lines = []
    for l in text.split("\n"):
        if l.startswith("["):
            sec = l
        if DEP_RX.match(l) and not (sec or "").startswith("[workspace"):
            package_lines.append(l)
    if not package_lines:
        continue
    files, data = sources(m)
    need = sorted(used(files))
    if not need:
        need = []
    def repl(match):
        spec = match.group(1)
        if "workspace = true" in spec:
            opt = ", optional = true" if "optional = true" in spec else ""
            return "\n".join("%s = { workspace = true%s }" % (n, opt) for n in need)
        path = re.search(r'path = "([^"]+)"', spec).group(1)
        base = os.path.normpath(os.path.join(os.path.dirname(m), path))
        return "\n".join('%s = { path = "%s" }' % (n, os.path.relpath(PKG[n], os.path.dirname(m))) for n in need)
    lines = text.split("\n")
    out, section, agg_sections = [], None, set()
    for l in lines:
        if l.startswith("["):
            section = l
        if DEP_RX.match(l) and not (section or "").startswith("[workspace"):
            agg_sections.add(section)
    section = None
    for l in lines:
        if l.startswith("["):
            section = l
        if section in agg_sections and re.match(r"^semio-s-artifact-stdio-semio-[a-z0-9-]+ = ", l) and l.split(" =")[0] in CRATE.values():
            continue
        out.append(l)
    final, section = [], None
    for l in out:
        if l.startswith("["):
            section = l
        if DEP_RX.match(l) and not (section or "").startswith("[workspace"):
            rendered = repl(DEP_RX.match(l))
            if rendered:
                final.append(rendered)
            continue
        final.append(l)
    new = "\n".join(final)
    new = new.replace('"dep:semio-s-artifact-stdio-semio"', ", ".join('"dep:%s"' % n for n in need))
    write(m, new)
    report[os.path.relpath(m, REPO)] = need
    json.dump(report, open(__file__.rsplit("/", 1)[0] + "/consumer-manifests.json", "w"), ensure_ascii=False, indent=1)
    root = workspace_root(m, data)
    if any("workspace = true" in l for l in package_lines):
        ws_needs.setdefault(root, set()).update(need)
    report[os.path.relpath(m, REPO)] = need

for ws, need in ws_needs.items():
    text = read(ws)
    table = re.search(r'(?m)^\[workspace\.dependencies\.semio-s-artifact-stdio-semio\]\npath = "([^"]+)"\n', text)
    inline = re.search(r'(?m)^semio-s-artifact-stdio-semio = \{ ?path = "([^"]+)"[^}\n]*\}[ \t]*\n', text)
    if not (table or inline):
        raise SystemExit("workspace without aggregator definition: " + ws)
    agg_path = (table or inline).group(1)
    semio_dir = os.path.normpath(os.path.join(os.path.dirname(ws), agg_path, "../.."))
    assert semio_dir == SEMIO, (ws, semio_dir)
    add = [n for n in sorted(need) if n != CRATE["agg"] and not re.search(r"(?m)^(\[workspace\.dependencies\.%s\]|%s = )" % (re.escape(n), re.escape(n)), text)]
    rel = lambda n: os.path.relpath(PKG[n], os.path.dirname(ws))
    if table:
        block = "".join('[workspace.dependencies.%s]\npath = "%s"\n\n' % (n, rel(n)) for n in add)
        text = text[:table.end()] + ("\n" + block.rstrip("\n") + "\n" if block else "") + text[table.end():]
    else:
        block = "".join('%s = { path = "%s" }\n' % (n, rel(n)) for n in add)
        text = text[:inline.end()] + block + text[inline.end():]
    if CRATE["agg"] not in need and not re.search(r"semio-s-artifact-stdio-semio\s*=\s*\{\s*workspace", "".join(read(p) for p in manifests if workspace_root(p, toml_loads(read(p))) == ws)):
        pass
    write(ws, text)
json.dump(report, open(__file__.rsplit("/", 1)[0] + "/consumer-manifests.json", "w"), ensure_ascii=False, indent=1)
print(len(report), "package manifests;", len(ws_needs), "workspaces")
