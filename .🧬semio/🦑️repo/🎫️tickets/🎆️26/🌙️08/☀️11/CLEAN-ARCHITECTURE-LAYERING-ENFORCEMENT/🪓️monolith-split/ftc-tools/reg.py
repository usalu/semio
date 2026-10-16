"""📋 FTC registration helpers: workspace members, owner rows and launch routes. Every function re-reads the target right before the edit, asserts a unique anchor, writes atomically and re-parses the result.
usage:
  reg.py member <framework|root> <parent-member> <new-member>
  reg.py owner <name> <root>
  reg.py routes <crate-pkg> <project-name>
  reg.py check <crate-pkg> <project-name> <framework|root> <new-member> <root>
"""
import json, os, re, sys
from decimal import Decimal

ROOT = "/Users/ueli/Documents/semio"
LAUNCH = ROOT + "/.vscode/launch.json"
SEED = ROOT + "/.vscode/🧩️launch.seed.jsonc"
OWNERS = ROOT + "/📋️build-owners.json"
MANIFESTS = {"framework": ROOT + "/🧰️framework/Cargo.toml", "root": ROOT + "/Cargo.toml"}
ANCHOR_PKG = "semio-framework-os-spr-laws-tests"


def read(p):
    with open(p, encoding="utf-8") as f:
        return f.read()


def write_atomic(p, text):
    tmp = p + ".ftc-tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        f.write(text)
    os.replace(tmp, p)


def insert_member(which, parent, new):
    p = MANIFESTS[which]
    s = read(p)
    m = re.search(r"^members\s*=\s*\[\n(.*?)^\]", s, re.S | re.M)
    body = m.group(1)
    lines = body.split("\n")
    ents = [re.match(r'^(\s*)"([^"]+)",?\s*$', l) for l in lines]
    paths = [e.group(2) if e else None for e in ents]
    assert new not in paths, f"member already present: {new}"
    assert parent in paths, f"parent member missing: {parent}"
    indent = next(e.group(1) for e in ents if e)
    i = paths.index(parent)
    j = i + 1
    while j < len(paths) and paths[j] is not None and paths[j].encode() < new.encode():
        j += 1
    lines.insert(j, f'{indent}"{new}",')
    nb = "\n".join(lines)
    s2 = s[: m.start(1)] + nb + s[m.end(1):]
    assert s2.count(f'"{new}"') == 1
    write_atomic(p, s2)
    print(f"member inserted at index {j} of {len(paths)} in {which}: {new}")


def insert_owner(name, root):
    s = read(OWNERS)
    d = json.loads(s)
    names = [o["name"] for o in d["owners"]]
    assert name not in names, f"owner exists: {name}"
    row = f'    {{\n      "name": "{name}",\n      "root": "{root}",\n      "manifest": "{root}/📋️project.json",\n      "packages": []\n    }},\n'
    nxt = next(n for n in names if n > name)
    anchor = f'    {{\n      "name": "{nxt}",\n'
    assert s.count(anchor) == 1, nxt
    s2 = s.replace(anchor, row + anchor)
    d2 = json.loads(s2)
    nn = [o["name"] for o in d2["owners"]]
    assert nn == sorted(nn) and len(set(nn)) == len(nn)
    write_atomic(OWNERS, s2)
    print(f"owner inserted before {nxt}: {name}")


ROUTES = [("🏗️build", "build", "4_build", "prototype", "🛠️prototype"), ("🏗️build", "build", "4_build", "release", "🚀️release"), ("🔎️check", "check", "4_gate", "prototype", "🛠️prototype"), ("🔎️check", "check", "4_gate", "release", "🚀️release"), ("🧪️test", "test", "4_test", "prototype", "🛠️prototype"), ("🧪️test", "test", "4_test", "release", "🚀️release")]


def orders(s, group):
    return [Decimal(m.group(1)) for m in re.finditer(r'"group": "' + re.escape(group) + r'",\s*"order": ([0-9.]+)', s)]


def insert_routes(path, pkg, project):
    s = read(path)
    assert f'"🦀️ {pkg} ' not in s, f"routes exist for {pkg} in {path}"
    taken = {g: set(orders(s, g)) for g in ("4_build", "4_gate", "4_test")}
    base = {}
    for g, t in taken.items():
        below = [o for o in t if o < 1000]
        base[g] = max(below)
    blocks = []
    alloc = {g: [] for g in taken}
    for label, tgt, g, cfg, cfglabel in ROUTES:
        cur = max(base[g], *(alloc[g] or [base[g]]))
        nxt = (cur + Decimal("0.001")).quantize(Decimal("0.001"))
        while nxt in taken[g] or nxt in alloc[g]:
            nxt += Decimal("0.001")
        alloc[g].append(nxt)
        blocks.append(f'    {{\n      "name": "🦀️ {pkg} {label} {cfglabel}",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "bun repo run {project}:{tgt} --configuration={cfg} --parallel=1 -- --lib --offline",\n      "cwd": "${{workspaceFolder}}",\n      "presentation": {{\n        "group": "{g}",\n        "order": {nxt}\n      }}\n    }},\n')
    name = f'"name": "🦀️ {ANCHOR_PKG} 🧪️test 🚀️release"'
    assert s.count(name) == 1
    i = s.index(name)
    end = s.index("\n    },\n", i) + len("\n    },\n")
    s2 = s[:end] + "".join(blocks) + s[end:]
    json.loads(s2)
    write_atomic(path, s2)
    print(f"routes inserted in {os.path.basename(path)}: " + ", ".join(f"{g}={[str(x) for x in v]}" for g, v in alloc.items()))


def check(pkg, project, which, member, root):
    ok = True
    s = read(MANIFESTS[which])
    ok &= f'"{member}"' in s
    d = json.loads(read(OWNERS))
    row = [o for o in d["owners"] if o["name"] == project]
    ok &= len(row) == 1 and row[0]["root"] == root
    for p in (LAUNCH, SEED):
        t = read(p)
        n = len(re.findall(re.escape(f'"🦀️ {pkg} ') + r"[^\"]+\"", t))
        cmds = len(re.findall(re.escape(f"bun repo run {project}:") + r"(?:build|check|test) --configuration=(?:prototype|release) --parallel=1 -- --lib --offline", t))
        print(f"{os.path.basename(p)}: {n} named routes, {cmds} commands")
        ok &= n == 6 and cmds == 6
    print("REGISTRATION", "OK" if ok else "INCOMPLETE", pkg)
    return ok


if __name__ == "__main__":
    a = sys.argv[1:]
    if a[0] == "member":
        insert_member(a[1], a[2], a[3])
    elif a[0] == "owner":
        insert_owner(a[1], a[2])
    elif a[0] == "routes":
        for p in (LAUNCH, SEED):
            insert_routes(p, a[1], a[2])
    elif a[0] == "check":
        check(*a[1:])
