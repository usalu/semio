"""📋️ Registration of new rtb crates: workspace member, owner row, six launch routes (launch.json + seed), Cargo.lock, taxonomy names, external dependency users.
Every function re-reads its file, asserts anchors and writes atomically. DRY=1 only reports."""
import json, os, re, sys, tempfile
from lib import *

DRY = os.environ.get("DRY") == "1"
LAUNCH = [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]
TAX = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
ANCHOR_CRATE = "semio-framework-os-renderer-wgpu-shell-tests-docs-3"


def rd(p):
    return read(REPO + "/" + p)


def wr(p, text):
    if DRY:
        return
    backup(REPO + "/" + p)
    full = REPO + "/" + p
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(full), prefix=".rtb-")
    with os.fdopen(fd, "w", encoding="utf8") as f:
        f.write(text)
    os.replace(tmp, full)


def add_member(member, cargo_toml="Cargo.toml"):
    t = rd(cargo_toml)
    m = re.search(r"^members = \[\n(.*?)^\]", t, re.M | re.S)
    body = m.group(1)
    rows = re.findall(r'^\s*"([^"]+)",?\s*$', body, re.M)
    if member in rows:
        return False
    pos = None
    for mm in re.finditer(r'^\s*"([^"]+)",?\s*$\n?', body, re.M):
        if mm.group(1).encode("utf8") > member.encode("utf8"):
            pos = mm.start()
            break
    assert pos is not None
    wr(cargo_toml, t[:m.start(1)] + body[:pos] + f'    "{member}",\n' + body[pos:] + t[m.end(1):])
    return True


def add_owner(name, root, manifest, owners="📋️build-owners.json"):
    t = rd(owners)
    d = json.loads(t)
    if name in [o["name"] for o in d["owners"]]:
        return False
    anchor = next(o["name"] for o in d["owners"] if o["name"] > name)
    key = f'    {{\n      "name": "{anchor}",'
    assert t.count(key) == 1
    row = f'    {{\n      "name": "{name}",\n      "root": "{root}",\n      "manifest": "{manifest}",\n      "packages": []\n    }},\n'
    out = t.replace(key, row + key)
    json.loads(out)
    wr(owners, out)
    return True


def used_orders():
    s = set()
    for f in LAUNCH:
        s |= set(re.findall(r'"order":\s*([0-9.]+)', rd(f)))
    return s


def next_orders(count):
    """Free order numbers for the three series (build 900.24xxx, check 922.05xxx, test 903.59xxx), above the highest renderer value."""
    used = used_orders()
    out = {}
    for prefix, start in (("900.", 24577), ("922.", 5376), ("903.", 59376)):
        n = start
        res = []
        while len(res) < 2 * count:
            cand = f"{prefix}{n:05d}" if prefix != "922." else f"{prefix}{n:05d}"
            if cand not in used:
                res.append(cand)
                used.add(cand)
            n += 1
        out[prefix] = res
    return out


def route_block(crate, project, build, check, test):
    spec = [("🏗️build 🛠️prototype", "build", "prototype", "4_build", build[0]), ("🏗️build 🚀️release", "build", "release", "4_build", build[1]), ("🔎️check 🛠️prototype", "check", "prototype", "4_gate", check[0]), ("🔎️check 🚀️release", "check", "release", "4_gate", check[1]), ("🧪️test 🛠️prototype", "test", "prototype", "4_test", test[0]), ("🧪️test 🚀️release", "test", "release", "4_test", test[1])]
    out = ""
    for suffix, target, mode, group, order in spec:
        out += (
            "    {\n"
            f'      "name": "🦀️ {crate} {suffix}",\n'
            '      "type": "node-terminal",\n'
            '      "request": "launch",\n'
            f'      "command": "bun repo run {project}:{target} --configuration={mode} --parallel=1 -- --lib --offline",\n'
            '      "cwd": "${workspaceFolder}",\n'
            '      "presentation": {\n'
            f'        "group": "{group}",\n'
            f'        "order": {order}\n'
            "      },\n"
            '      "env": {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0", "NODE_DISABLE_COMPILE_CACHE": "1"}\n'
            "    },\n"
        )
    return out


def add_routes(crates_projects):
    """crates_projects: [(crate, project)] in registration order; blocks are inserted after the last route of the anchor crate, in both launch files."""
    orders = next_orders(len(crates_projects))
    blocks = ""
    for i, (crate, project) in enumerate(crates_projects):
        blocks += route_block(crate, project, orders["900."][2 * i:2 * i + 2], orders["922."][2 * i:2 * i + 2], orders["903."][2 * i:2 * i + 2])
    for f in LAUNCH:
        t = rd(f)
        if any(f'"name": "🦀️ {c} 🏗️build 🛠️prototype"' in t for c, p in crates_projects):
            continue
        key = f'      "name": "🦀️ {ANCHOR_CRATE} 🧪️test 🚀️release",'
        assert t.count(key) == 1, (f, t.count(key))
        i = t.index(key)
        j = t.index("\n    },\n", i) + len("\n    },\n")
        wr(f, t[:j] + blocks + t[j:])


def add_taxonomy_members(names):
    """Appends crate folder names to the wgpu folder member list (after the last shell-tests bin)."""
    t = rd(TAX)
    anchor = '        "🥣️shell-tests-docs-3"\n      ],'
    assert t.count(anchor) == 1
    add = [n for n in names if f'"{n}"' not in t]
    if not add:
        return []
    ins = ',\n'.join(f'        "{n}"' for n in add)
    out = t.replace(anchor, '        "🥣️shell-tests-docs-3",\n' + ins + "\n      ],")
    json.loads(out)
    wr(TAX, out)
    return add


def add_taxonomy_children(names):
    t = rd(TAX)
    i = t.index('"crate-module-children": {')
    j = t.index('"memberNames": [', i)
    k = t.index("\n      ]", j)
    add = [n for n in names if f'"{n}"' not in t[i:k + 10]]
    if not add:
        return []
    ins = "".join(f',\n        "{n}"' for n in add)
    out = t[:k] + ins + t[k:]
    json.loads(out)
    wr(TAX, out)
    return add


# -- Cargo.lock ---------------------------------------------------------------------------------------------------------------------
def manifest_dep_packages(manifest_text):
    """Package names (renames resolved) of every dependency table of a manifest."""
    out = set()
    sec = None
    for line in manifest_text.splitlines():
        m = re.match(r"^\[(.+)\]\s*$", line)
        if m:
            sec = m.group(1)
            continue
        if sec and sec.endswith("dependencies") and "=" in line and not line.lstrip().startswith("#"):
            key = line.split("=", 1)[0].strip().split(".")[0]
            if not re.match(r"^[A-Za-z0-9_\-]+$", key):
                continue
            pk = re.search(r'package\s*=\s*"([^"]+)"', line)
            out.add(pk.group(1) if pk else key)
    return out


def lock_sync(lock, crate_name, deps, create=False):
    t = rd(lock)
    key = f'[[package]]\nname = "{crate_name}"\n'
    names = sorted(deps)
    multi = [d for d in names if len(re.findall(r'(?m)^name = "' + re.escape(d) + r'"$', t)) > 1]
    entry = f'[[package]]\nname = "{crate_name}"\nversion = "0.1.0"\n' + ("dependencies = [\n" + "".join(f' "{d}",\n' for d in names) + "]\n" if names else "") + "\n"
    if key in t:
        i = t.index(key)
        j = t.index("\n\n", i) + 2
        old = t[i:j]
        if old == entry:
            return multi
        ver = re.search(r'version = "([^"]+)"', old).group(1)
        new = entry.replace('version = "0.1.0"', f'version = "{ver}"')
        # keep `name version` forms the old entry used
        for d in multi:
            m = re.search(r'^ "(' + re.escape(d) + r' [^"]+)",$', old, re.M)
            if m:
                new = new.replace(f' "{d}",\n', f' "{m.group(1)}",\n')
        wr(lock, t[:i] + new + t[j:])
        return multi
    assert create, ("lock entry missing", crate_name)
    pos = None
    for mm in re.finditer(r'^\[\[package\]\]\nname = "([^"]+)"\n', t, re.M):
        if mm.group(1) > crate_name:
            pos = mm.start()
            break
    assert pos is not None
    wr(lock, t[:pos] + entry + t[pos:])
    return multi
