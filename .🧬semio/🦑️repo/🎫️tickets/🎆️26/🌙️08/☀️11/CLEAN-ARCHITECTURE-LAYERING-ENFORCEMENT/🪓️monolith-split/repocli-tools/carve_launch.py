"""🚀 Wave-4 repo CLI carve, launch step: six prototype/release build/check/test routes per new crate in launch.json and the launch seed."""
import re, sys

ROOT = "/Users/ueli/Documents/semio/"
CRATES = ["event-stream", "command-framework", "tree-source-and-search", "mcp-verb", "verbs"]
TARGETS = [("build", "🏗️build", "4_build", 900.015), ("check", "🔎️check", "4_gate", 922.045), ("test", "🧪️test", "4_test", 903.585)]
CONF = [("prototype", "🛠️prototype"), ("release", "🚀️release")]
FILES = {
    ".vscode/launch.json": "semio-framework-repo-graphql-schema 🧪️test 🚀️release",
    ".vscode/🧩️launch.seed.jsonc": "semio-framework-pets-staging 🧪️test 🚀️release",
}


def num(base, k):
    return repr(round(base + k / 100000, 5))


def routes():
    out = []
    for ti, (target, label, group, base) in enumerate(TARGETS):
        pass
    k = 0
    objs = []
    for i, c in enumerate(CRATES):
        for target, label, group, base in TARGETS:
            for j, (conf, conflabel) in enumerate(CONF):
                order = num(base, 20 + 2 * i + j + 1)
                name = f"🦀️ semio-framework-repo-cli-{c} {label} {conflabel}"
                cmd = f"bun repo run @semio-tech/repo-cli-{c}-rs:{target} --configuration={conf} --parallel=1 -- --lib --offline"
                objs.append((name, group, order, f'    {{\n      "name": "{name}",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "{cmd}",\n      "cwd": "${{workspaceFolder}}",\n      "presentation": {{\n        "group": "{group}",\n        "order": {order}\n      }}\n    }},\n'))
    return objs


def main():
    objs = routes()
    assert len({(g, float(o)) for _, g, o, _ in objs}) == len(objs)
    for rel, anchor in FILES.items():
        t = open(ROOT + rel, encoding="utf-8").read()
        for name, group, order, _ in objs:
            assert t.count(f'"name": "{name}"') == 0, name
            for m in re.finditer(r'"group": "' + group + r'",\s*"order":\s*([0-9.]+)', t):
                assert float(m.group(1)) != float(order), (rel, order)
        key = f'"name": "🦀️ {anchor}",'
        assert t.count(key) == 1, (rel, t.count(key))
        i = t.index(key)
        j = t.index("\n    },\n", i) + len("\n    },\n")
        assert t[j:j + 6] == "    {\n", repr(t[j:j + 20])
        text = "".join(o[3] for o in objs)
        if "--write" in sys.argv:
            open(ROOT + rel, "w", encoding="utf-8").write(t[:j] + text + t[j:])
        print(rel, "ok", len(objs))


main()
