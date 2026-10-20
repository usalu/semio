#!/usr/bin/env python3
"""Authors staged/paths.fixture.json from spec.py with plain hashlib and string arithmetic (no TypeScript involved).

usage: genfixture.py <out.json>
"""
import hashlib, json, os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from spec import ALIAS, budget, utf16, utf8

DEEP = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🕸️units/📦️package/🌱️environment/📖️references/🧪️testing/🔣️json/🟦️.ts"
TYPED = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🧪️tests/🧪️typednodefields-preflights-every-capture-before-transfer-under-private-r/🟦️.ts"
TUTORIAL = "🧰️framework/🔨️modules/🎓️onboarding/🧪️tests/🧫️fixtures/🖱️tutorial-local-interaction.json"
COURSE = "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json"
LONG_NAME = "x" * 71 + ".sqlite"
FILES = [
    "package.json",
    "README.md",
    COURSE,
    TUTORIAL,
    DEEP,
    TYPED,
    "src/ünï/コード/mod.ts",
    "src/ünï/コード/index.ts",
]


def digest(directory):
    return hashlib.sha256(directory.encode("utf-8")).hexdigest()[: ALIAS["hexLength"]]


def directory_of(path):
    cut = path.rfind("/")
    return "" if cut < 0 else path[:cut]


def label(path):
    return ALIAS["link"] + "/" + digest(directory_of(path)) + "/" + path[path.rfind("/") + 1 :]


def measure(labels):
    return {"units": max(utf16(x) for x in labels), "bytes": max(utf8(x) for x in labels)}


ZERO = {"units": 0, "bytes": 0}
ALIAS_SIZE = measure([label(p) for p in FILES])
LEGACY_SIZE = measure(["s/" + p for p in FILES])
HASH = "0123456789abcdef"
hosts = [
    ("macOS arm64 under /Users/ueli/Documents/semio", "darwin", "arm64", "/Users/ueli/Documents/semio", "unprobed", "/"),
    ("macOS x64 under /Users/dev/Documents/GitHub/semio-workspace", "darwin", "x64", "/Users/dev/Documents/GitHub/semio-workspace", "unprobed", "/"),
    ("devcontainer Linux arm64 under /workspaces/semio", "linux", "arm64", "/workspaces/semio", "unprobed", "/"),
    ("Linux x64 under /home/ueli/Documents/semio", "linux", "x64", "/home/ueli/Documents/semio", "unprobed", "/"),
    ("Windows x64 under C:\\Users\\ueli\\Documents\\semio without long paths", "win32", "x64", "C:\\Users\\ueli\\Documents\\semio", "disabled", "\\"),
    ("Windows x64 under C:\\Users\\ueli\\Documents\\semio with long paths", "win32", "x64", "C:\\Users\\ueli\\Documents\\semio", "enabled", "\\"),
    ("Windows arm64 under C:\\Users\\firstname.lastname\\source\\repos\\semio without long paths", "win32", "arm64", "C:\\Users\\firstname.lastname\\source\\repos\\semio", "disabled", "\\"),
    ("Windows x64 under C:\\semio without long paths", "win32", "x64", "C:\\semio", "disabled", "\\"),
]


def base(root, sep):
    return root + sep + sep.join([".cache", "b", "p", "m", "bazel", "outputs", "server", HASH])


rows = []
for name, platform, arch, root, long_paths, sep in hosts:
    for layout, closure, derive in (
        ("alias layout", {"alias": ALIAS_SIZE, "real": ZERO}, "alias"),
        ("legacy repository-path layout", {"alias": LEGACY_SIZE, "real": ZERO}, "legacy"),
    ):
        output_base = base(root, sep)
        rows.append(
            {
                "name": name + ": " + layout,
                "derive": derive,
                "input": {"platform": platform, "arch": arch, "outputBase": output_base, "longPaths": long_paths, "closure": closure},
                "expect": budget(platform, arch, output_base, long_paths, closure),
            }
        )

hermetic = {"alias": ALIAS_SIZE, "real": {"units": utf16("s/" + DEEP), "bytes": utf8("s/" + DEEP)}}
for name, platform, arch, root, long_paths, sep in hosts[:4]:
    output_base = base(root, sep)
    rows.append(
        {
            "name": name + ": hermetic task keeps the repository layout inside the sandbox",
            "derive": None,
            "input": {"platform": platform, "arch": arch, "outputBase": output_base, "longPaths": long_paths, "closure": hermetic},
            "expect": budget(platform, arch, output_base, long_paths, hermetic),
        }
    )

LONGEST = measure([label("🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🕸️units/" + LONG_NAME)])
synthetic = [
    ("synthetic: the longest tracked basename of the repository on macOS", "darwin", "arm64", "/Users/ueli/Documents/semio", "unprobed", "/", {"alias": LONGEST, "real": ZERO}),
    ("synthetic: the longest tracked basename of the repository on Linux", "linux", "x64", "/home/ueli/Documents/semio", "unprobed", "/", {"alias": LONGEST, "real": ZERO}),
    ("synthetic: macOS byte limit binds before the unit limit", "darwin", "arm64", "/Users/ueli/Documents/semio", "unprobed", "/", {"alias": {"units": 40, "bytes": 1000}, "real": ZERO}),
    ("synthetic: Windows long paths still obey the 256 repository rule", "win32", "x64", "C:\\" + "d" * 150, "enabled", "\\", {"alias": ALIAS_SIZE, "real": ZERO}),
    ("synthetic: no sources, only outputs", "linux", "x64", "/home/ueli/Documents/semio", "unprobed", "/", {"alias": ZERO, "real": ZERO}),
]
for name, platform, arch, root, long_paths, sep, closure in synthetic:
    output_base = base(root, sep)
    rows.append(
        {
            "name": name,
            "derive": None,
            "input": {"platform": platform, "arch": arch, "outputBase": output_base, "longPaths": long_paths, "closure": closure},
            "expect": budget(platform, arch, output_base, long_paths, closure),
        }
    )

directories = sorted({directory_of(p) for p in FILES})
fixture = {
    "version": 1,
    "alias": ALIAS,
    "directories": [{"directory": d, "alias": digest(d)} for d in directories],
    "labels": [{"path": p, "label": label(p)} for p in FILES],
    "closure": {"files": FILES, "alias": ALIAS_SIZE, "legacy": LEGACY_SIZE},
    "budgets": rows,
}
json.dump(fixture, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=2)
open(sys.argv[1], "a").write("\n")
print("rows", len(rows), "alias", ALIAS_SIZE, "legacy", LEGACY_SIZE)
for row in rows:
    e = row["expect"]
    print(("OK  " if e["fits"] else "REF ") + row["name"][:96], max(f["units"] for f in e["families"].values()), "slack", e["slackUnits"])
