"""🪚 Crate-budget helpers: register a new sibling crate in the artifact workspace, every Cargo.lock that knows the old crate, the artifact script.ts and the old crate manifest."""
import bisect, os, re, subprocess

SEMIO = "/home/deck/semio"
NORM = SEMIO + "/✏️s/🔌️plugins/📕️norm"
SKIP = ("/target/", "/node_modules/", "/⚡️cache/", "/🎫️tickets/", "/.git/")


def rd(p):
    return open(p, encoding="utf-8").read()


def wr(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf-8").write(t)


def relp(a, b):
    return os.path.relpath(a, b)


def lock_files(old):
    out = subprocess.run(["grep", "-rl", "--include=Cargo.lock", '-e', 'name = "%s"' % old, SEMIO], capture_output=True, text=True).stdout.split("\n")
    return [p for p in out if p and not any(s in p for s in SKIP)]


def lock_blocks(t):
    parts = re.split(r"(?m)^(?=\[\[package\]\]\n)", t)
    return parts[0], parts[1:]


def pkg_name(b):
    return re.search(r'name = "([^"]+)"', b).group(1)


def lock_register(old, new, extra_old_deps=(), new_deps=None, drop_old_deps=()):
    for p in lock_files(old):
        head, pk = lock_blocks(rd(p))
        names = [pkg_name(b) for b in pk]
        if new in names:
            continue
        i = names.index(old)
        ob = pk[i]
        m = re.search(r"dependencies = \[\n(.*?)\]\n", ob, re.S)
        odeps = re.findall(r"(?m)^ \"([^\"\n]+)\",$", m.group(1)) if m else []
        nd = sorted(set(new_deps if new_deps is not None else odeps))
        nb = '[[package]]\nname = "%s"\nversion = "0.1.0"\n' % new + ("dependencies = [\n" + "".join(' "%s",\n' % d for d in nd) + "]\n" if nd else "") + "\n"
        od = sorted((set(odeps) | {new} | set(extra_old_deps)) - set(drop_old_deps))
        body = "dependencies = [\n" + "".join(' "%s",\n' % d for d in od) + "]\n"
        ob = (ob[: m.start()] + body + ob[m.end():]) if m else re.sub(r'(version = "[^"]+"\n)', lambda x: x.group(1) + body, ob, 1)
        if not ob.endswith("\n\n"):
            ob = ob.rstrip("\n") + "\n\n"
        pk[i] = ob
        j = bisect.bisect_left(names, new)
        if j == len(pk) and not pk[-1].endswith("\n\n"):
            pk[-1] = pk[-1].rstrip("\n") + "\n\n"
        pk.insert(j, nb if j < len(pk) else nb.rstrip("\n") + "\n")
        if j == len(pk) - 1 and j > 0:
            pk[j] = nb.rstrip("\n") + "\n"
        wr(p, head + "".join(pk))
        print("lock", p[len(SEMIO):])


def register_workspace(art_root, new, crate_pkg_dir):
    f = art_root + "/Cargo.toml"
    s = rd(f)
    rel = relp(crate_pkg_dir, art_root)
    if '"%s"' % rel not in s:
        s = s.replace("members = [\n", 'members = [\n    "%s",\n' % rel, 1)
    if "[workspace.dependencies.%s]" % new not in s:
        s = s.replace("[workspace.lints.rust]", '[workspace.dependencies.%s]\npath = "%s"\n\n[workspace.lints.rust]' % (new, rel), 1)
    wr(f, s)


def add_dep(toml, name):
    s = rd(toml)
    if name + " =" not in s:
        s = s.replace("[dependencies]\n", "[dependencies]\n%s = { workspace = true }\n" % name, 1)
        wr(toml, s)


def script_register(art_root, old, new):
    f = art_root + "/📦️packages/🦀️rust/📜️script.ts"
    s = rd(f)
    if '"%s"' % new not in s:
        assert '"%s"' % old in s
        s = s.replace('"%s"' % old, '"%s","%s"' % (old, new), 1)
        wr(f, s)


def manifest(name, art_root, pkg, desc, base_toml, drop=()):
    src = rd(base_toml)
    bpkg = os.path.dirname(base_toml)

    def fix(m):
        return m.group(1) + relp(os.path.normpath(os.path.join(bpkg, m.group(2))), pkg) + '"'

    deps = re.search(r"\[dependencies\]\n(.*?)(?:\n\[|\Z)", src, re.S).group(1)
    deps = [d for d in re.sub(r'(path = ")([^"]+)"', fix, deps).strip().split("\n") if d and not any(d.startswith(x + " ") for x in drop)]
    dev = re.search(r"\[dev-dependencies\]\n.*", src, re.S)
    t = '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n' % (relp(art_root, pkg), name, desc) + "\n".join(sorted(set(deps))) + "\n"
    if dev:
        t += "\n" + dev.group(0).strip() + "\n"
    return t


def toml_deps(t):
    out = set()
    for sec in re.findall(r"(?ms)^\[(?:dev-)?dependencies\]\n(.*?)(?=^\[|\Z)", t):
        out |= set(re.findall(r"(?m)^([\w-]+) =", sec))
    return sorted(out)
