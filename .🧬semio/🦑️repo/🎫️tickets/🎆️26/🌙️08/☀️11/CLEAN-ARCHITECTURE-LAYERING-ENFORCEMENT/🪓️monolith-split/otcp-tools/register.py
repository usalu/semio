"""Registration of companion crates: workspace member, owner row, six launch routes (launch.json + seed), Cargo.lock entry, taxonomy names.
Every function re-reads its file, asserts anchors and writes atomically. DRY=1 only reports."""
import json, os, re, sys, tempfile
sys.path.insert(0, os.path.dirname(__file__))
import edit as E

ROOT = E.ROOT
DRY = os.environ.get("DRY") == "1"


def atomic_write(path, text):
    E.pre_copy(path)
    p = E.abspath(path)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(p), prefix=".otcp-")
    with os.fdopen(fd, "w", encoding="utf-8") as f:
        f.write(text)
    os.replace(tmp, p)
    E.note("edit", p)


def add_member(cargo_toml, member):
    t = E.read(cargo_toml)
    m = re.search(r"^members = \[\n(.*?)^\]", t, re.M | re.S)
    rows = re.findall(r'^(\s*)"([^"]+)",?\s*$', m.group(1), re.M)
    assert not any(r[1] == member for r in rows), "member exists"
    line = f'    "{member}",\n'
    body = m.group(1)
    pos = None
    for mm in re.finditer(r'^\s*"([^"]+)",?\s*$\n?', body, re.M):
        if mm.group(1).encode("utf-8") > member.encode("utf-8"):
            pos = mm.start()
            break
    assert pos is not None, "no larger member found"
    new_body = body[:pos] + line + body[pos:]
    out = t[:m.start(1)] + new_body + t[m.end(1):]
    if not DRY:
        atomic_write(cargo_toml, out)
    return pos


def add_owner(name, root, manifest, owners="📋️build-owners.json"):
    t = E.read(owners)
    d = json.loads(t)
    names = [o["name"] for o in d["owners"]]
    assert name not in names, "owner exists"
    anchor = None
    for o in d["owners"]:
        if o["name"] > name:
            anchor = o["name"]
            break
    assert anchor, "no larger owner"
    key = f'    {{\n      "name": "{anchor}",'
    assert t.count(key) == 1, "owner anchor count"
    row = f'    {{\n      "name": "{name}",\n      "root": "{root}",\n      "manifest": "{manifest}",\n      "packages": []\n    }},\n'
    out = t.replace(key, row + key)
    json.loads(out)
    if not DRY:
        atomic_write(owners, out)


def route_block(crate, project, order_base, group_prefix):
    b, c, t = order_base
    spec = [
        ("🏗️build 🛠️prototype", "build", "prototype", "4_build", f"900.{b}"),
        ("🏗️build 🚀️release", "build", "release", "4_build", f"900.{b + 1}"),
        ("🔎️check 🛠️prototype", "check", "prototype", "4_gate", f"922.{c}"),
        ("🔎️check 🚀️release", "check", "release", "4_gate", f"922.{c + 1}"),
        ("🧪️test 🛠️prototype", "test", "prototype", "4_test", f"903.{t}"),
        ("🧪️test 🚀️release", "test", "release", "4_test", f"903.{t + 1}"),
    ]
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
            "      }\n"
            "    },\n"
        )
    return out


def used_orders(text):
    return set(re.findall(r'"order":\s*([0-9.]+)', text))


def add_routes(path, anchor_name, crate, project, order_base):
    t = E.read(path)
    assert f'"name": "🦀️ {crate} 🏗️build 🛠️prototype"' not in t, "routes exist"
    blk = route_block(crate, project, order_base, None)
    used = used_orders(t)
    for o in re.findall(r'"order":\s*([0-9.]+)', blk):
        assert o not in used, f"order {o} already used"
    key = f'      "name": "🦀️ {anchor_name}",'
    assert t.count(key) == 1, f"anchor count {t.count(key)} for {anchor_name}"
    i = t.index(key)
    j = t.index("\n    },\n", i) + len("\n    },\n")
    out = t[:j] + blk + t[j:]
    if not DRY:
        atomic_write(path, out)
    return f"{crate} 🧪️test 🚀️release"


def add_lock(lock, name, deps):
    t = E.read(lock)
    assert f'name = "{name}"\n' not in t, "lock entry exists"
    entry = f'[[package]]\nname = "{name}"\nversion = "0.1.0"\ndependencies = [\n' + "".join(f' "{d}",\n' for d in sorted(deps)) + "]\n\n"
    if not deps:
        entry = f'[[package]]\nname = "{name}"\nversion = "0.1.0"\n\n'
    pos = None
    for mm in re.finditer(r'^\[\[package\]\]\nname = "([^"]+)"\n', t, re.M):
        if mm.group(1) > name:
            pos = mm.start()
            break
    assert pos is not None
    out = t[:pos] + entry + t[pos:]
    if not DRY:
        atomic_write(lock, out)


def add_taxonomy(names, tax="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"):
    t = E.read(tax)
    i = t.index('"crate-module-children": {')
    j = t.index('"memberNames": [', i)
    k = t.index("\n      ]", j)
    block = t[j:k]
    add = [n for n in names if f'"{n}"' not in t]
    if not add:
        return []
    ins = "".join(f',\n        "{n}"' for n in add)
    out = t[:k] + ins + t[k:]
    json.loads(out)
    if not DRY:
        atomic_write(tax, out)
    return add
