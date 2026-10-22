import json, sys
F = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains"
pins = json.load(open("🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json", encoding="utf8"))
rows = [row for row in pins["definitions"]["Pins"]["const"]["assets"] if row["tool"] == "uv"]
sizes = {"darwin-arm64": 20807020, "darwin-x64": 22542643, "linux-x64": 24284812, "linux-arm64": 22677572, "win32-x64": 23498773, "win32-arm64": 22068511}
dist = []
for host in ["darwin-arm64", "darwin-x64", "linux-arm64", "linux-x64", "win32-arm64", "win32-x64"]:
    platform, arch = host.split("-")
    row = next(item for item in rows if item["platform"] == platform and item["arch"] == arch)
    archive = row["url"].rsplit("/", 1)[1]
    windows = platform == "win32"
    prefix = "" if windows else archive[: -len(".tar.gz")]
    dist.append({"host": host, "archive": archive, "format": "zip" if windows else "tar.gz", "bytes": sizes[host], "sha256": row["sha256"], "prefix": prefix, "binaryDirectory": ".", "binaries": ["uv.exe", "uvx.exe"] if windows else ["uv", "uvx"]})
version = rows[0]["version"]
tool = {"version": version, "release": "https://github.com/astral-sh/uv/releases/download/" + version, "versionArguments": ["--version"], "distributions": dist}
path = F + "/🔣️.json"
document = json.load(open(path, encoding="utf8"))
assert document["python"]["uv"] == version
python = {}
for key, value in document["python"].items():
    python[key] = value
    if key == "uv": python["tool"] = tool
document["python"] = python
open(path, "w", encoding="utf8").write(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
