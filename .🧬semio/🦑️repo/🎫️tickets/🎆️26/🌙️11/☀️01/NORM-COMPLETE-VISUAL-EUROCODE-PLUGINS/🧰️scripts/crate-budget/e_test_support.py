"""🧪 Applies the `test-support` serde feature to every norm crate whose lib closure derives serde under `cfg_attr(test, ..)` (mutation, diff, schema, inference payloads) and enables it in the dev-dependencies of every crate whose tests call serde_json on types of such crates. Usage: e_test_support.py [--check]. Idempotent."""
import os, re, subprocess, sys, tomllib
sys.path.insert(0, os.path.dirname(__file__))
from lib import *

MOD = re.compile(r'((?:#\[path\s*=\s*"([^"]+)"\]\s*)?)(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;')
INC = re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
USE = re.compile(r'serde_json::(?:to_value|to_string|from_value|from_str|to_vec|from_slice|to_string_pretty|to_writer|from_reader|to_writer_pretty|value::to_value|value::from_value)\b|serde_json::\{[^}]*\b(?:to_value|to_string|from_value|from_str|to_vec|from_slice|to_string_pretty)\b')
OLD = "cfg_attr(test, "
NEW = 'cfg_attr(any(test, feature = "test-support"), '
FEAT = "test-support"
CHECK = "--check" in sys.argv


def closure(entry):
    seen, st = set(), [entry]
    while st:
        p = os.path.normpath(st.pop())
        if p in seen or not os.path.isfile(p):
            continue
        seen.add(p)
        if not p.endswith(".rs"):
            continue
        t = rd(p)
        d = os.path.dirname(p)
        for m in MOD.finditer(t):
            if m.group(2):
                st.append(os.path.join(d, m.group(2)))
            else:
                b = os.path.splitext(os.path.basename(p))[0]
                st += [os.path.join(d, m.group(3) + ".rs"), os.path.join(d, m.group(3), "mod.rs")] if b in ("lib", "main", "mod") else [os.path.join(d, b, m.group(3) + ".rs")]
        for m in INC.finditer(t):
            st.append(os.path.join(d, m.group(1)))
    return seen


pk = {}
for dp, dn, fn in os.walk(NORM):
    dn[:] = [d for d in dn if d not in ("target", "node_modules", "🎫️tickets")]
    if "Cargo.toml" in fn:
        try:
            c = tomllib.loads(rd(dp + "/Cargo.toml"))
        except Exception:
            continue
        if "package" in c:
            pk[c["package"]["name"]] = (dp, c)

libfiles, gated = {}, set()
for n, (dp, c) in pk.items():
    libfiles[n] = closure(os.path.join(dp, c.get("lib", {}).get("path", "src/lib.rs")))
    if any(f.endswith(".rs") and (OLD in rd(f) or NEW in rd(f)) for f in libfiles[n]):
        gated.add(n)


def direct(c, dev):
    return set(c.get("dependencies", {})) | (set(c.get("dev-dependencies", {})) if dev else set())


forward = {n: sorted(d for d in pk[n][1].get("dependencies", {}) if d in gated and d != n) for n in gated}
edges = {}


def add_edge(n, *deps):
    edges.setdefault(n, set()).update(deps)


def toml_set_features(t, name, fwd, optional=True):
    items = (['"dep:serde"'] if optional else []) + ['"%s/%s"' % (d, FEAT) for d in fwd]
    line = "%s = [%s]\n" % (FEAT, ", ".join(items))
    if re.search(r"(?m)^\[features\]", t):
        if re.search(r"(?m)^%s = " % FEAT, t):
            return re.sub(r"(?m)^%s = \[.*?\]\n" % FEAT, lambda m: line, t, count=1, flags=re.S)
        return t.replace("[features]\n", "[features]\n" + line, 1)
    if "\n[dev-dependencies]" in t:
        return t.replace("\n[dev-dependencies]", "\n[features]\n" + line + "\n[dev-dependencies]", 1)
    return t.rstrip("\n") + "\n\n[features]\n" + line


changed = 0
for n in sorted(gated):
    dp, c = pk[n]
    for f in libfiles[n]:
        if f.endswith(".rs"):
            s = rd(f)
            if OLD in s:
                if not CHECK:
                    wr(f, s.replace(OLD, NEW))
                changed += 1
    p = dp + "/Cargo.toml"
    t = rd(p)
    sd = c.get("dependencies", {}).get("serde")
    if sd is None:
        t = t.replace("[dependencies]\n", "[dependencies]\nserde = { workspace = true, optional = true }\n", 1)
        assert "optional = true" in t, p
    t2 = toml_set_features(t, n, forward[n], sd is None or bool(sd.get("optional")))
    if not CHECK and t2 != rd(p):
        wr(p, t2)
    add_edge(n, "serde", *forward[n])
print("gated crates", len(gated), "files rewritten", changed)


def reach(n):
    seen, st, first = set(), [n], True
    while st:
        x = st.pop()
        if x in seen or x not in pk:
            continue
        seen.add(x)
        st += list(direct(pk[x][1], first))
        first = False
    return seen


def enabled_by(deps):
    seen, st = set(), list(deps)
    while st:
        x = st.pop()
        if x in seen:
            continue
        seen.add(x)
        st += forward.get(x, [])
    return seen


def dev_line(t, name, c):
    sec = c.get("dev-dependencies", {}).get(name)
    src = c.get("dependencies", {}).get(name)
    spec = sec if sec is not None else src
    m = re.search(r"(?m)^%s = (\{.*\})$" % re.escape(name), t)
    return spec, m


def ws_names(art_root):
    return set(re.findall(r"\[workspace\.dependencies\.([\w-]+)\]", rd(art_root + "/Cargo.toml")) + re.findall(r"(?m)^([\w-]+) = \{", rd(art_root + "/Cargo.toml").split("[workspace.dependencies]")[-1] if "[workspace.dependencies]" in rd(art_root + "/Cargo.toml") else ""))


def art_root_of(dp):
    x = dp
    while x != "/":
        if os.path.exists(x + "/Cargo.toml") and "[workspace.dependencies" in rd(x + "/Cargo.toml"):
            return x
        x = os.path.dirname(x)


report, unresolved = [], []
for y, (dp, c) in sorted(pk.items()):
    ent = [os.path.join(dp, c.get("lib", {}).get("path", "src/lib.rs"))] + [os.path.join(dp, x["path"]) for x in c.get("test", []) if "path" in x]
    files = set()
    for e in ent:
        files |= closure(e)
    if not any(f.endswith(".rs") and USE.search(rd(f)) for f in files):
        continue
    dirs = {x for x in direct(c, True) if x in gated and x != y}
    cov = enabled_by(dirs)
    need = {x for x in reach(y) if x in gated and x != y} - cov
    art = art_root_of(dp)
    wsn = ws_names(art) if art else set()
    p = dp + "/Cargo.toml"
    t = rd(p)
    ok = set(dirs)
    for x in sorted(need):
        if x in wsn:
            ok.add(x)
        else:
            unresolved.append((y, x))
    for x in sorted(ok):
        sec = c.get("dev-dependencies", {}).get(x)
        if isinstance(sec, dict) and FEAT in sec.get("features", []):
            continue
        m = re.search(r"(?m)^%s = (\{.*\})\n" % re.escape(x), t[t.find("[dev-dependencies]"):]) if "[dev-dependencies]" in t and sec is not None else None
        if m:
            off = t.find("[dev-dependencies]")
            body = m.group(1)
            if "features = [" in body:
                body = re.sub(r"features = \[", 'features = ["%s", ' % FEAT, body, count=1)
            else:
                body = body[:-1].rstrip() + ', features = ["%s"] }' % FEAT
            t = t[: off + m.start()] + "%s = %s\n" % (x, body) + t[off + m.end():]
        else:
            src = c.get("dependencies", {}).get(x)
            if isinstance(src, dict) and "path" in src:
                line = '%s = { path = "%s", features = ["%s"] }\n' % (x, src["path"], FEAT)
            else:
                line = '%s = { workspace = true, features = ["%s"] }\n' % (x, FEAT)
            if "[dev-dependencies]" in t:
                t = t.replace("[dev-dependencies]\n", "[dev-dependencies]\n" + line, 1)
            else:
                t = t.rstrip("\n") + "\n\n[dev-dependencies]\n" + line
        add_edge(y, x)
        report.append((y, x))
    if not CHECK and t != rd(p):
        wr(p, t)
print("dev-dependency features", len(report), "unresolved", len(unresolved))
for u in unresolved[:20]:
    print("UNRESOLVED", u)

if not CHECK:
    locks = [p for p in subprocess.run(["find", SEMIO, "-name", "Cargo.lock", "-not", "-path", "*/target/*", "-not", "-path", "*/node_modules/*", "-not", "-path", "*/🎫️tickets/*"], capture_output=True, text=True).stdout.split("\n") if p]
    for p in locks:
        s = rd(p)
        if "semio-s-artifact-norm-" not in s:
            continue
        head, blocks = lock_blocks(s)
        names = [pkg_name(b) for b in blocks]
        hit = 0
        for i, b in enumerate(blocks):
            add = edges.get(names[i])
            if not add:
                continue
            m = re.search(r"dependencies = \[\n(.*?)\]\n", b, re.S)
            deps = set(re.findall(r'"([^"]+)"', m.group(1))) if m else set()
            add = {a for a in add if a in names or a in {x.split(" ")[0] for x in deps}}
            new = deps | add
            if new == deps:
                continue
            body = "dependencies = [\n" + "".join(' "%s",\n' % d for d in sorted(new)) + "]\n"
            blocks[i] = (b[: m.start()] + body + b[m.end():]) if m else re.sub(r'(version = "[^"]+"\n)', lambda x: x.group(1) + body, b, 1)
            hit += 1
        if hit:
            wr(p, head + "".join(blocks))
            print("lock", p[len(SEMIO):], hit)
