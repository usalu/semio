#!/usr/bin/env python3
"""📒️ Registers one new Rust library crate: workspace member, owner row and the six launch routes in both launch files.

usage: register.py <crate name> <package dir relative to the repo> <project id> [--apply]
The text of each shared registry is read once, edited in memory and written back only if it did not change meanwhile.
"""
import json, os, re, sys

REPO = "/Users/ueli/Documents/semio"
ROOT_CARGO = REPO + "/Cargo.toml"
OWNERS = REPO + "/📋️build-owners.json"
LAUNCH = REPO + "/.vscode/launch.json"
SEED = REPO + "/.vscode/🧩️launch.seed.jsonc"
SEED_ENV = '\n      "env": {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0", "NODE_DISABLE_COMPILE_CACHE": "1"}'
ROUTE_ANCHOR = '"name": "🦀️ semio-framework-os-mcp-http-transport 🧪️test 🚀️release"'


def insert_members(text, rels):
    m = re.search(r"members = \[\n(.*?)\n\]", text, re.S)
    lines = m.group(1).split("\n")
    assert all(l.strip().startswith('"') for l in lines)
    existing = [l.strip().strip(",").strip('"') for l in lines]
    for r in sorted(set(rels) - set(existing), key=lambda x: x.encode("utf-8")):
        pos = next((i for i, e in enumerate(existing) if e.encode("utf-8") > r.encode("utf-8")), len(existing))
        existing.insert(pos, r)
        lines.insert(pos, f'    "{r}",')
    return text[: m.start(1)] + "\n".join(lines) + text[m.end(1):]


def insert_owner_row(text, name, root_rel, manifest_rel):
    if f'"name": "{name}"' in text:
        return text
    block = f'''    {{
      "name": "{name}",
      "root": "{root_rel}",
      "manifest": "{manifest_rel}",
      "packages": []
    }},
'''
    pos = None
    for m in re.finditer(r'    \{\n      "name": "([^"]+)"', text):
        if m.group(1).encode("utf-8") > name.encode("utf-8"):
            pos = m.start()
            break
    assert pos is not None, name
    return text[:pos] + block + text[pos:]


def order_bands(texts, count):
    used = set()
    for t in texts:
        used |= set(re.findall(r'"order":\s*([0-9.]+)', t))
    out = {}
    for kind, prefix in (("build", "900.33"), ("check", "922.33"), ("test", "903.33")):
        vals, n = [], 1
        while len(vals) < 2 * count:
            v = f"{prefix}{n:03d}"
            n += 1
            if v not in used:
                vals.append(v)
        out[kind] = vals
    return out


def route_blocks(cname, pid, orders):
    out = []
    for target, label, group, key in (("build", "🏗️build", "4_build", "build"), ("check", "🔎️check", "4_gate", "check"), ("test", "🧪️test", "4_test", "test")):
        for ci, (conf, clabel) in enumerate((("prototype", "🛠️prototype"), ("release", "🚀️release"))):
            order = orders[key][ci]
            out.append(f'''    {{
      "name": "🦀️ {cname} {label} {clabel}",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun repo run {pid}:{target} --configuration={conf} --parallel=1 -- --lib --offline",
      "cwd": "${{workspaceFolder}}",
      "presentation": {{
        "group": "{group}",
        "order": {order}
      }}{"," + SEED_ENV}
    }},
''')
    return "".join(out)


def insert_routes(text, cname, pid, orders):
    if f'"name": "🦀️ {cname} 🏗️build 🛠️prototype"' in text:
        return text
    i = text.index(ROUTE_ANCHOR)
    end = text.index("\n    },\n", i) + len("\n    },\n")
    return text[:end] + route_blocks(cname, pid, orders) + text[end:]


def main():
    cname, pkg, pid = sys.argv[1:4]
    apply = "--apply" in sys.argv
    texts = {p: open(p, encoding="utf-8").read() for p in (ROOT_CARGO, OWNERS, LAUNCH, SEED)}
    orders = order_bands([texts[LAUNCH], texts[SEED]], 1)
    out = {
        ROOT_CARGO: insert_members(texts[ROOT_CARGO], [pkg]),
        OWNERS: insert_owner_row(texts[OWNERS], pid, pkg, pkg + "/📋️project.json"),
        LAUNCH: insert_routes(texts[LAUNCH], cname, pid, orders),
        SEED: insert_routes(texts[SEED], cname, pid, orders),
    }
    json.loads(out[OWNERS])
    for p, t in out.items():
        print(("changed " if t != texts[p] else "same    ") + os.path.relpath(p, REPO), len(t) - len(texts[p]))
    if not apply:
        return
    for p in out:
        if open(p, encoding="utf-8").read() != texts[p]:
            raise RuntimeError("concurrent edit detected on " + p)
    for p, t in out.items():
        if t != texts[p]:
            open(p, "w", encoding="utf-8").write(t)
    print("applied")


main()
