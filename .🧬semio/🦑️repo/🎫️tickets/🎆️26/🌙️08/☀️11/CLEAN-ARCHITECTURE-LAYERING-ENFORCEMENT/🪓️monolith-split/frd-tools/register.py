import re, json, sys, os
ROOT = "/Users/ueli/Documents/semio"
FE = "️"
NODE = "\U0001F9EA" + FE + "test/\U0001F9F2" + FE + "fixture-reader"
PKG = "\U0001F9F0" + FE + "framework/\U0001F528" + FE + "modules/" + NODE + "/\U0001F4E6" + FE + "packages/\U0001F980" + FE + "rust"
NAME = "semio-framework-test-fixtures"; PROJECT = "@semio-tech/test-fixtures-rs"
ORDERS = {"build": 900.33083, "check": 922.33083, "test": 903.33083}
def routes():
    out = []
    for verb, emoji, group in (("build", "\U0001F3D7" + FE + "build", "4_build"), ("check", "\U0001F50E" + FE + "check", "4_gate"), ("test", "\U0001F9EA" + FE + "test", "4_test")):
        for k, (cfg, cemoji) in enumerate((("prototype", "\U0001F6E0" + FE), ("release", "\U0001F680" + FE))):
            order = round(ORDERS[verb] + k * 0.00001, 5)
            out.append(f'''    {{
      "name": "\U0001F980{FE} {NAME} {emoji} {cemoji}{cfg}",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun repo run {PROJECT}:{verb} --configuration={cfg} --parallel=1 -- --lib --offline",
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
def launch(path):
    t = open(path, encoding="utf-8").read()
    if NAME in t: print("already", path); return
    anchor = '"name": "\U0001F980' + FE + ' semio-framework-test-original-snapshot-owners \U0001F9EA' + FE + 'test \U0001F680' + FE + 'release"'
    assert t.count(anchor) == 1, path
    i = t.index(anchor)
    j = t.index("\n    },\n", i) + len("\n    },\n")
    new = t[:j] + routes() + t[j:]
    open(path, "w", encoding="utf-8").write(new)
    print("routes added", path)
for p in (".vscode/launch.json", ".vscode/\U0001F9E9" + FE + "launch.seed.jsonc"):
    launch(os.path.join(ROOT, p))
