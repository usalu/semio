import itertools
import json
import subprocess
import sys

R = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher"
fixture_path = R + "/🧫️fixtures/📌️runtime-pin/🔣️.json"
oracle = R + "/🧪️tests/📌️runtime-pin-contract/🐍️oracle/🐍️.py"
sha = {k: (k * 64) for k in "abcdef"}
registry = {
    "selected": "bun",
    "languages": ["en", "de"],
    "runtimes": {"bun": {
        "version": "9.9.9",
        "executable": {"unix": "bun", "windows": "bun.exe"},
        "distributions": [
            {"platform": "darwin", "arch": "arm64", "libc": "any", "archive": "bun-darwin-aarch64.zip", "bytes": 5, "sha256": sha["a"]},
            {"platform": "linux", "arch": "x64", "libc": "gnu", "archive": "bun-linux-x64-baseline.zip", "bytes": 5, "sha256": sha["b"]},
            {"platform": "linux", "arch": "x64", "libc": "musl", "archive": "bun-linux-x64-musl-baseline.zip", "bytes": 5, "sha256": sha["c"]},
            {"platform": "win32", "arch": "x64", "libc": "any", "archive": "bun-windows-x64-baseline.zip", "bytes": 5, "sha256": sha["d"]},
        ]}},
    "cache": {"environment": "SEMIO_RUNTIME_CACHE", "runtimeEnvironment": "SEMIO_RUNTIME", "toolsPartition": [".cache", "b", "t", "m"], "user": ["semio", "runtimes"]},
}
hosts = {
    "mac": {"platform": "darwin", "arch": "arm64", "musl": False},
    "linux": {"platform": "linux", "arch": "x64", "musl": False},
    "alpine": {"platform": "linux", "arch": "x64", "musl": True},
    "windows": {"platform": "win32", "arch": "x64", "musl": False},
}
select = [dict(id=name, **host) for name, host in hosts.items()] + [
    {"id": "linux-arm64-unpinned", "platform": "linux", "arch": "arm64", "musl": False},
    {"id": "freebsd-unpinned", "platform": "freebsd", "arch": "x64", "musl": False},
]
store = lambda where, executable="present", receipt="ok": {"store": where, "executable": executable, "receipt": receipt}
locate = []
for name, host in hosts.items():
    locate += [
        dict(id=name + "-tools-partition", **host, stores=[store("tools")]),
        dict(id=name + "-user-cache", **host, stores=[store("user")]),
        dict(id=name + "-tools-before-user", **host, stores=[store("user"), store("tools")]),
        dict(id=name + "-nothing", **host, stores=[]),
        dict(id=name + "-receipt-for-another-archive", **host, stores=[store("tools", receipt="wrong-archive")]),
        dict(id=name + "-no-receipt", **host, stores=[store("tools", receipt="absent")]),
        dict(id=name + "-size-mismatch-falls-back-to-user", **host, stores=[store("tools", receipt="size-mismatch"), store("user")]),
        dict(id=name + "-receipt-with-matching-size", **host, stores=[store("user", receipt="ok-sized")]),
        dict(id=name + "-no-executable", **host, stores=[store("tools", executable="absent")]),
        dict(id=name + "-not-executable", **host, stores=[store("tools", executable="plain")]),
    ]
locate += [dict(id="freebsd-unpinned", platform="freebsd", arch="x64", musl=False, stores=[store("tools")])]
cache = [
    {"id": "linux-xdg", "platform": "linux", "environment": {"XDG_CACHE_HOME": "/x/cache"}, "home": "/home/u"},
    {"id": "linux-home", "platform": "linux", "environment": {}, "home": "/home/u"},
    {"id": "linux-nothing", "platform": "linux", "environment": {}, "home": ""},
    {"id": "mac-home", "platform": "darwin", "environment": {"XDG_CACHE_HOME": "/ignored"}, "home": "/Users/u"},
    {"id": "windows-local-app-data", "platform": "win32", "environment": {"LocalAppData": "C:\\Users\\u\\AppData\\Local"}, "home": "C:\\Users\\u"},
    {"id": "windows-nothing", "platform": "win32", "environment": {}, "home": "C:\\Users\\u"},
]
decide = []
for running, mode, command_line, located, launcher, depth in itertools.product(["1.3.14", "9.9.9"], ["run", "test"], [False, True], [False, True], [False, True], [0, 1]):
    decide.append({"running": running, "pin": "9.9.9", "mode": mode, "commandLine": command_line, "located": located, "launcher": launcher, "depth": depth})
split = [
    {"id": "ms-docs-plain", "line": 'prog.exe "abc" d e', "expected": ["prog.exe", "abc", "d", "e"]},
    {"id": "ms-docs-backslashes-before-quote", "line": 'prog.exe a\\\\\\b d"e f"g h', "expected": ["prog.exe", "a\\\\\\b", "de fg", "h"]},
    {"id": "ms-docs-escaped-quote", "line": 'prog.exe a\\\\\\"b c d', "expected": ["prog.exe", 'a\\"b', "c", "d"]},
    {"id": "ms-docs-even-backslashes-open-quote", "line": 'prog.exe a\\\\\\\\"b c" d e', "expected": ["prog.exe", "a\\\\b c", "d", "e"]},
    {"id": "program-in-quotes-keeps-backslashes", "line": '"C:\\Program Files\\bun.exe" test ./a.ts', "expected": ["C:\\Program Files\\bun.exe", "test", "./a.ts"]},
]
for index, arguments in enumerate([
    ["test", "./🧪️tests/🟦️.ts", "--timeout", "30000"],
    ["test", "-t", "a name with spaces"],
    ["x", 'say "hi"', "back\\slash", "trail\\"],
    ["", "empty before"],
    ["tab\tseparated", "multi  space"],
    ["C:\\dir with space\\", "next"],
]):
    split.append({"id": "round-trip-%d" % index, "arguments": arguments})
procargs = [
    {"id": "three-arguments", "executable": "/Users/u/.bun/bin/bun", "padding": 3, "arguments": ["/Users/u/.bun/bin/bun", "test", "./a.ts"]},
    {"id": "no-padding-unicode", "executable": "/b/bun", "padding": 0, "arguments": ["bun", "--smol", "./🧪️tests/🟦️.ts", "a b"]},
]
cmdline = [
    {"id": "linux-two", "arguments": ["/usr/bin/bun", "./a.ts"]},
    {"id": "linux-unicode-and-empty", "arguments": ["bun", "test", "./🧪️tests/🟦️.ts", "", "x"]},
]
body = {"registry": registry, "select": select, "locate": locate, "cache": cache, "decide": decide, "split": split, "procargs": procargs, "cmdline": cmdline}
inputs = sys.argv[1]
open(inputs, "w").write(json.dumps(body, ensure_ascii=False))
out = json.loads(subprocess.run(["python3", "-I", oracle, inputs], check=True, capture_output=True, text=True).stdout)
for case, value in zip(select, out["select"]): case["archive"] = value
for case, value in zip(locate, out["locate"]): case["origin"] = value
for case, value in zip(cache, out["cache"]): case["directory"] = value
for case, value in zip(decide, out["decide"]): case["decision"] = value
for case, value in zip(split, out["split"]):
    case["line"] = value["line"]
    case["expected"] = value["arguments"]
for case, value in zip(procargs, out["procargs"]): case["hex"] = value
for case, value in zip(cmdline, out["cmdline"]): case["hex"] = value
open(fixture_path, "w").write(json.dumps(body, ensure_ascii=False, indent=1) + "\n")
print(len(select), len(locate), len(cache), len(decide), len(split), len(procargs), len(cmdline))
