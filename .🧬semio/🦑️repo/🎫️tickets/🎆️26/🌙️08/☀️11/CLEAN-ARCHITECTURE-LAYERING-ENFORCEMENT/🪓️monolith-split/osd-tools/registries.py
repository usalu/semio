"""🗂 OSD: registers the five new crates: root workspace members, build owners, six launch routes each in launch.json and the seed, OS boundary corpus rows. Re-reads every file right before its edit and asserts the anchors."""
import json, os, re, sys
ROOT = "/Users/ueli/Documents/semio"
MOD = "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory"
NEW = [  # (key, node relative to MOD)
    ("browser-actor", "🧬️schema/🌐️browser-actor"),
    ("schema", "🧬️schema"),
    ("check-in", "🧬️schema/📌️document-check-in-v1"),
    ("binary", "🚪️io/🧱️binary"),
    ("access-policy", "🛡️access-policy"),
]
def rd(p): return open(p, encoding="utf-8").read()
def wr(p, t): open(p, "w", encoding="utf-8").write(t)

# 1 workspace members (byte-sorted insertion inside the contiguous directory block)
p = ROOT + "/Cargo.toml"
t = rd(p)
for key, node in NEW:
    member = f"{MOD}/{node}/📦️packages/🦀️rust"
    line = f'    "{member}",\n'
    if line in t:
        continue
    i = t.index("[workspace]"); members = t.index("members = [", i); end = t.index("]\n", members)
    block = t[members:end]
    lines = block.split("\n")
    idx = None
    for n, l in enumerate(lines):
        m = re.match(r'^    "(.*)",$', l)
        if m and m.group(1).encode() > member.encode() and m.group(1).startswith("🧰️framework/🛍️products/💻️os/🔨️modules/"):
            idx = n; break
    assert idx is not None
    lines.insert(idx, line.rstrip("\n"))
    t = t[:members] + "\n".join(lines) + t[end:]
wr(p, t)
mem = [m for m in re.findall(r'^    "([^"]+)",$', t[t.index("members = ["):], re.M)]
print("members sorted around directory:", [m for m in mem if "📇️directory" in m] == sorted([m for m in mem if "📇️directory" in m], key=lambda s: s.encode()))

# 2 build owners
p = ROOT + "/📋️build-owners.json"
t = rd(p)
d = json.loads(t)
names = [o["name"] for o in d["owners"]]
for key, node in NEW:
    name = f"@semio-tech/os-directory-{key}-rs"
    if name in names:
        continue
    root = f"{MOD}/{node}/📦️packages/🦀️rust"
    block = f'    {{\n      "name": "{name}",\n      "root": "{root}",\n      "manifest": "{root}/📋️project.json",\n      "packages": []\n    }},\n'
    nxt = next(n for n in sorted(names + [name]) if n > name) if any(n > name for n in names) else None
    anchor = f'    {{\n      "name": "{nxt}",\n'
    assert t.count(anchor) == 1, (anchor, t.count(anchor))
    t = t.replace(anchor, block + anchor)
    names.append(name)
wr(p, t)
d = json.loads(t); nn = [o["name"] for o in d["owners"]]
print("owners", len(nn), "sorted", nn == sorted(nn), "unique", len(set(nn)) == len(nn))

# 3 launch routes
def routes(order_band):
    out = []
    for i, (key, node) in enumerate(NEW):
        crate = f"semio-framework-os-directory-{key}"
        proj = f"@semio-tech/os-directory-{key}-rs"
        for target, glyph, group, band in (("build", "🏗️build", "4_build", 900), ("check", "🔎️check", "4_gate", 922), ("test", "🧪️test", "4_test", 903)):
            for k, (mode, mglyph) in enumerate((("prototype", "🛠️prototype"), ("release", "🚀️release"))):
                n = 1 + 2 * i + k
                order = f"{band}.99{n:03d}"
                out.append((f"🦀️ {crate} {glyph} {mglyph}", f'bun repo run {proj}:{target} --configuration={mode} --parallel=1 -- --lib --offline', group, order))
    return out
def block(name, command, group, order):
    return f'    {{\n      "name": "{name}",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "{command}",\n      "cwd": "${{workspaceFolder}}",\n      "presentation": {{\n        "group": "{group}",\n        "order": {order}\n      }}\n    }},\n'
for f in (".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"):
    p = ROOT + "/" + f
    t = rd(p)
    used = set(re.findall(r'"order": ([0-9.]+)', t))
    rows = routes(None)
    if any(f'"name": "{r[0]}"' in t for r in rows):
        assert all(f'"name": "{r[0]}"' in t for r in rows)
        continue
    for r in rows:
        assert r[3] not in used, (f, r[3])
    anchor_name = "🦀️ semio-framework-os-directory-client-tests 🧪️test 🚀️release"
    a = t.index(f'      "name": "{anchor_name}"')
    end = t.index("\n    },\n", a) + len("\n    },\n")
    t = t[:end] + "".join(block(*r) for r in rows) + t[end:]
    wr(p, t)
    print(f, "inserted", len(rows))

# 4 OS corpus rows
p = ROOT + "/🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json"
t = rd(p)
anchor_end = '''      "entry": "🔨️modules/📇️directory/🦀️.rs",
      "maximumActiveLines": 4000,
      "forbiddenDependencies": [
        "semio-framework-os-kernel",
        "semio-framework-plugin",
        "semio-framework"
      ]
    },
'''
assert t.count(anchor_end) == 1
rows = ""
for key, node in NEW:
    pkg = f"semio-framework-os-directory-{key}"
    if f'"package": "{pkg}"' in t:
        continue
    rows += f'''    {{
      "package": "{pkg}",
      "manifest": "🔨️modules/📇️directory/{node}/📦️packages/🦀️rust/Cargo.toml",
      "entry": "🔨️modules/📇️directory/{node}/🦀️.rs",
      "maximumActiveLines": 4000,
      "forbiddenDependencies": [
        "semio-framework-os-kernel",
        "semio-framework-plugin",
        "semio-framework"
      ]
    }},
'''
t = t.replace(anchor_end, anchor_end + rows)
wr(p, t)
json.loads(t)
print("corpus rows", t.count('"package"'))
