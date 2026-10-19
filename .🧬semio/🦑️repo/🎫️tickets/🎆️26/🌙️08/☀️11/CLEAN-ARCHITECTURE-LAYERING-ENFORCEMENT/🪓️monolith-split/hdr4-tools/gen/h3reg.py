import json, os, re, shutil, sys
R = "/Users/ueli/Documents/semio"
PKG = "📦️packages/🦀️rust"
F = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
BK = F + "/hdr4-tools/orig"

def backup(path):
    os.makedirs(BK, exist_ok=True)
    dst = BK + "/" + os.path.relpath(path, R).replace("/", "__")
    if not os.path.exists(dst) and os.path.exists(path):
        shutil.copy(path, dst)

def rd(p):
    return open(p, encoding="utf8").read()

def wr(p, t):
    backup(p)
    open(p, "w", encoding="utf8").write(t)

def census_manifest(name):
    for l in open(F + "/🗑️generated/hdr4-crate-census.csv", encoding="utf8").read().split("\n"):
        if l.startswith(name + ","):
            return R + "/" + l.split(",")[1]
    raise KeyError(name)

def pkg_name(man):
    return re.search(r'^name = "([^"]+)"', rd(man), re.M).group(1)

def ws_dir(man):
    rel = re.search(r'^workspace = "([^"]+)"', rd(man), re.M).group(1)
    return os.path.normpath(os.path.join(os.path.dirname(man), rel))

def lib_root(man):
    mt = rd(man)
    m = re.search(r'\[lib\][^\[]*?path = "([^"]+)"', mt)
    return os.path.normpath(os.path.join(os.path.dirname(man), m.group(1) if m else "src/lib.rs"))

def add_member(wsd, newpkgdir, afterpkgdir):
    p = wsd + "/Cargo.toml"
    t = rd(p)
    mem = os.path.relpath(newpkgdir, wsd)
    old = os.path.relpath(afterpkgdir, wsd)
    m = re.search(r'(?m)^([ \t]*)"%s",\n' % re.escape(old), t)
    assert m, ("member anchor", old, p)
    assert ('"%s",' % mem) not in t, "member exists"
    ind = m.group(1)
    t = t[:m.end()] + '%s"%s",\n' % (ind, mem) + t[m.end():]
    wr(p, t)

def add_ws_dep(wsd, newpkg, newpkgdir, afterpkg):
    p = wsd + "/Cargo.toml"
    t = rd(p)
    rel = os.path.relpath(newpkgdir, wsd)
    m = re.search(r'(?m)^([ \t]*)\[workspace\.dependencies\.%s\]\n[ \t]*path = "([^"]+)"\n' % re.escape(afterpkg), t)
    if m:
        ind = m.group(1)
        add = '\n%s[workspace.dependencies.%s]\n%spath = "%s"\n' % (ind, newpkg, ind, rel)
        assert "[workspace.dependencies.%s]" % newpkg not in t
        t = t[:m.end()] + add + t[m.end():]
        wr(p, t)
        return "table"
    m = re.search(r'(?m)^%s = \{ path = "([^"]+)"[^\n]*\}\n' % re.escape(afterpkg), t)
    if m:
        assert ("\n%s = " % newpkg) not in t
        t = t[:m.end()] + '%s = { path = "%s" }\n' % (newpkg, rel) + t[m.end():]
        wr(p, t)
        return "inline"
    return None

def add_owner(name, pkgdir, project=None):
    op = R + "/📋️build-owners.json"
    o = rd(op)
    rrel = os.path.relpath(pkgdir, R)
    nm = project or name
    man = rrel + ("/📋️project.json" if project else "/Cargo.toml")
    row = '    {\n      "name": "%s",\n      "root": "%s",\n      "manifest": "%s",\n      "packages": []\n    },\n' % (nm, rrel, man)
    names = [(x.start(), x.group(1)) for x in re.finditer(r'    \{\n      "name": "([^"]+)"', o)]
    assert '"%s"' % nm not in o, "owner row exists"
    p = next((p for p, n in names if n > nm), None)
    assert p is not None
    o = o[:p] + row + o[p:]
    json.loads(o)
    wr(op, o)

def project_json(pkgdir, ident):
    open(pkgdir + "/📋️project.json", "w", encoding="utf8").write('{\n  "name": "@semio-tech/%s-rs",\n  "tags": [\n    "scope:framework",\n    "language:rust"\n  ]\n}\n' % ident)

def routes(name, ident, k, after_name):
    lp = R + "/.vscode/launch.json"
    t = rd(lp)
    def route(label, target, conf, group, order):
        return ('    {\n      "name": "🦀️ %s %s",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "bun repo run @semio-tech/%s-rs:%s --configuration=%s --parallel=1 -- --lib --offline",\n      "cwd": "${workspaceFolder}",\n      "presentation": {\n        "group": "%s",\n        "order": %s\n      }\n    },\n' % (name, label, ident, target, conf, group, order))
    a, b = 2 * k - 1, 2 * k
    rs = "".join([route("🏗️build 🛠️prototype", "build", "prototype", "4_build", "900.96%03d" % a), route("🏗️build 🚀️release", "build", "release", "4_build", "900.96%03d" % b),
        route("🔎️check 🛠️prototype", "check", "prototype", "4_gate", "922.96%03d" % a), route("🔎️check 🚀️release", "check", "release", "4_gate", "922.96%03d" % b),
        route("🧪️test 🛠️prototype", "test", "prototype", "4_test", "903.96%03d" % a), route("🧪️test 🚀️release", "test", "release", "4_test", "903.96%03d" % b)])
    for o in ("900.96%03d" % a, "922.96%03d" % a, "903.96%03d" % a, "900.96%03d" % b):
        assert ('"order": %s\n' % o) not in t, ("order used", o)
    assert ('"🦀️ %s ' % name) not in t
    last = [m.end() for m in re.finditer(r'"name": "🦀️ %s [^"]+",\n(?:.*\n)*?    \},\n' % re.escape(after_name), t)]
    assert last, "anchor routes"
    t = t[:last[-1]] + rs + t[last[-1]:]
    wr(lp, t)

def next_route_index():
    t = rd(R + "/.vscode/launch.json")
    used = [int(x) for x in re.findall(r'"order": 900\.96(\d{3})\n', t)]
    return (max(used) + 1) // 2 + 1 if used else 1
