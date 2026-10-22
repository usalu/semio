#!/usr/bin/env python3
"""Builds staged-paths/fixture.json: the language-agnostic corpus. Expected values come from oracle.py (independent of the TypeScript law). usage: gen-fixture.py [<live root>]"""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "/Users/ueli/Documents/semio"
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
schema = json.load(open(os.path.join(HERE, "schema.json"), encoding="utf8"))
component = json.load(open(os.path.join(ROOT, B, "🧬️schema/🔣️.json"), encoding="utf8"))
oracle = open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
gis = "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎥️set-camera/🧾️wire-witness/🦠️mutation/🔣️.json"
short = "🧰️framework/📦️packages/🟦️typescript/🟦️.ts"
median = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔣️.json"


def u16(text):
    return len(text.encode("utf-16-le")) // 2


def pad(units):
    base = "✏️s/" + "d/" * 3
    return base + "x" * (units - u16(base) - u16("/🟦️.ts")) + "/🟦️.ts"


p240, p241 = pad(240), pad(241)
assert u16(p240) == 240 and u16(p241) == 241, (u16(p240), u16(p241))
tree = ["0/typescript@5.9.3/node_modules/typescript/lib/lib.es2022.d.ts", "0/" + "p" * 30 + "@1.0.0/node_modules/" + "q" * 30 + "/dist/" + "r" * 40 + ".js"]
deep_tree = ["0/" + "p" * 60 + "@1.0.0/node_modules/" + "q" * 60 + "/dist/" + "r" * 40 + ".js"]
targets = [
    {"id": "hermetic-short", "kind": "hermetic", "sources": [short, median], "tree": tree},
    {"id": "hermetic-deep", "kind": "hermetic", "sources": [short, gis]},
    {"id": "hermetic-tree-deep", "kind": "hermetic", "sources": [short], "tree": deep_tree},
    {"id": "hermetic-240", "kind": "hermetic", "sources": [p240]},
    {"id": "hermetic-241", "kind": "hermetic", "sources": [p241]},
    {"id": "host-deep", "kind": "host", "sources": [gis]},
    {"id": "empty", "kind": "hermetic", "sources": []},
]
hosts = [
    {"platform": "darwin", "arch": "arm64", "longPaths": "unprobed", "outputBase": "/Users/dev/semio/.cache/b/p/m/bazel/outputs/server/s0", "checkout": "/Users/dev/semio"},
    {"platform": "linux", "arch": "x64", "longPaths": "unprobed", "outputBase": "/home/dev/semio/.cache/b/p/m/bazel/outputs/server/s0", "checkout": "/home/dev/semio"},
    {"platform": "win32", "arch": "x64", "longPaths": "disabled", "outputBase": "C:\\b\\0123abcd\\s0", "checkout": "C:\\Users\\abcdefgh\\semio"},
    {"platform": "win32", "arch": "arm64", "longPaths": "enabled", "outputBase": "C:\\b\\0123abcd\\s0", "checkout": "C:\\Users\\abcdefgh\\semio"},
    {"platform": "win32", "arch": "x64", "longPaths": "disabled", "outputBase": "C:\\Users\\abcdefgh\\semio\\.cache\\b\\p\\m\\bazel\\outputs\\server\\s0", "checkout": "C:\\Users\\abcdefgh\\semio"},
    {"platform": "win32", "arch": "x64", "longPaths": "disabled", "checkout": "C:\\Users\\abcdefgh\\semio"},
    {"platform": "darwin", "arch": "x64", "longPaths": "unprobed"},
]
cases = {"law": [{"name": f"{h['platform']}-{h['arch']}-{h['longPaths']}-{'base' if 'outputBase' in h else 'nobase'}-{i}", "host": h, "targets": targets} for i, h in enumerate(hosts)],
         "roots": [{"platform": "win32", "checkout": "C:\\Users\\abcdefgh\\semio"}, {"platform": "win32", "checkout": "c:/users/abcdefgh/semio"}, {"platform": "win32", "checkout": "D:\\work\\semio"}, {"platform": "win32", "checkout": "\\\\server\\share\\semio"}, {"platform": "darwin", "checkout": "/Users/dev/semio"}, {"platform": "linux", "checkout": "/home/dev/semio"}]}
result = subprocess.run([sys.executable, "-I", "-c", oracle], input=json.dumps({"schema": schema, "component": component, "cases": cases}), capture_output=True, text=True)
if result.returncode:
    sys.exit(result.stderr)
expected = json.loads(result.stdout)
for case, row in zip(cases["law"], expected["law"]):
    case["expected"] = row
for case, row in zip(cases["roots"], expected["roots"]):
    case["expected"] = row
json.dump({"version": 1, "oracle": oracle, "cases": cases}, open(os.path.join(HERE, "fixture.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
open(os.path.join(HERE, "fixture.json"), "a").write("\n")
print("fixture written", len(cases["law"]), "law cases", len(cases["roots"]), "roots")
for case in cases["law"]:
    e = case["expected"]
    print(case["name"], "fits" if e["fits"] else "over:" + ",".join(e["over"]), e["limit"])
