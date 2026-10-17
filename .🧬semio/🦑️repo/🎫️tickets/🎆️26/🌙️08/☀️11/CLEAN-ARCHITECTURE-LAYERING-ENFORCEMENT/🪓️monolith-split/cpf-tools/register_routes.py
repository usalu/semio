#!/usr/bin/env python3
"""📒️ Registers the six launch routes (build, check, test × prototype, release) of crates that own a tests/ tree but no route, in launch.json and the seed (same text, same anchor)."""
import json, os, re, sys
REPO = "/Users/ueli/Documents/semio"
LAUNCH = REPO + "/.vscode/launch.json"
SEED = REPO + "/.vscode/🧩️launch.seed.jsonc"
CRATES = [
    # (crate, anchor crate whose test release route precedes, has_lib)
    ("semio-framework-3d-mesh-tests", "semio-framework-3d-testing-support", True),
    ("semio-framework-plugin-command-page-laws", "semio-framework-plugin-testing-laws", False),
    ("semio-framework-plugin-child-group-wire-laws", "semio-framework-plugin-command-page-laws", False),
    ("semio-framework-plugin-surface-carrier-laws", "semio-framework-plugin-child-group-wire-laws", False),
]
def order_bands(texts, count):
    used = set()
    for t in texts:
        used |= set(re.findall(r'"order":\s*([0-9.]+)', t))
    out = {}
    for kind, prefix in (("build", "900.98"), ("check", "922.98"), ("test", "903.98")):
        vals, n = [], 1
        while len(vals) < 2 * count:
            v = f"{prefix}{n:03d}"; n += 1
            if v not in used: vals.append(v)
        out[kind] = vals
    return out
def route_blocks(cname, has_lib, orders):
    out = []
    for target, label, group, key in (("build", "🏗️build", "4_build", "build"), ("check", "🔎️check", "4_gate", "check"), ("test", "🧪️test", "4_test", "test")):
        args = "--offline" if target == "test" else ("--lib --offline" if has_lib else "--tests --offline")
        for ci, (conf, clabel) in enumerate((("prototype", "🛠️prototype"), ("release", "🚀️release"))):
            order = orders[key][ci]
            out.append(f'''    {{
      "name": "🦀️ {cname} {label} {clabel}",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun repo run {cname}:{target} --configuration={conf} --parallel=1 -- {args}",
      "cwd": "${{workspaceFolder}}",
      "presentation": {{
        "group": "{group}",
        "order": {order}
      }},
      "env": {{
        "BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0",
        "NODE_DISABLE_COMPILE_CACHE": "1"
      }}
    }},
''')
    return "".join(out)
def insert_routes(text, cname, anchor, has_lib, orders):
    if f'"name": "🦀️ {cname} 🏗️build 🛠️prototype"' in text: return text
    a = f'"name": "🦀️ {anchor} 🧪️test 🚀️release"'
    assert text.count(a) == 1, anchor
    i = text.index(a)
    end = text.index("\n    },\n", i) + len("\n    },\n")
    return text[:end] + route_blocks(cname, has_lib, orders) + text[end:]
def main():
    apply = "--apply" in sys.argv
    texts = {p: open(p, encoding="utf-8").read() for p in (LAUNCH, SEED)}
    orders = order_bands(list(texts.values()), len(CRATES))
    out = dict(texts)
    for index, (cname, anchor, has_lib) in enumerate(CRATES):
        mine = {k: orders[k][2 * index: 2 * index + 2] for k in orders}
        for p in (LAUNCH, SEED):
            out[p] = insert_routes(out[p], cname, anchor, has_lib, mine)
    for p, t in out.items():
        print(("changed " if t != texts[p] else "same    ") + os.path.relpath(p, REPO), len(t) - len(texts[p]))
    if not apply: return
    json.loads(out[LAUNCH])
    for p in out:
        if open(p, encoding="utf-8").read() != texts[p]: raise RuntimeError("concurrent edit detected on " + p)
    for p, t in out.items():
        if t != texts[p]:
            tmp = p + ".cpf.tmp"; open(tmp, "w", encoding="utf-8").write(t); os.replace(tmp, p)
    print("applied")
main()
