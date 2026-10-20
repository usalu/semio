#!/usr/bin/env python3
# 🧾 Generates the authored toolchain policy document (staged/toolchains-policy.json) from named constants so every pin appears once.
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
HOSTS = ["darwin-arm64", "darwin-x64", "linux-arm64", "linux-x64", "win32-arm64", "win32-x64"]
CMAKE = "4.4.4"
NINJA = "1.13.2"
cm = {
    "darwin-arm64": ("cmake-%s-macos-universal.tar.gz" % CMAKE, 89454352, "4b7b73704b1db9b374e5c9ab8e17ac6148b817b6396ee75cd94a852cbac9d305"),
    "darwin-x64": ("cmake-%s-macos-universal.tar.gz" % CMAKE, 89454352, "4b7b73704b1db9b374e5c9ab8e17ac6148b817b6396ee75cd94a852cbac9d305"),
    "linux-arm64": ("cmake-%s-linux-aarch64.tar.gz" % CMAKE, 52039672, "a1b6cc63636a0e55c63257cf3315a8a5f129e42fade25db1afea4ff8ab06f25e"),
    "linux-x64": ("cmake-%s-linux-x86_64.tar.gz" % CMAKE, 64865570, "e5bb807f7728cb60cd8b27ebc97a2edb469b68655f21e844a600c3575b76f5bb"),
    "win32-arm64": ("cmake-%s-windows-arm64.zip" % CMAKE, 52553164, "ed673bbc4eb7c1e59b0407fd52f1680fc0ae3229a46b9b7b3ea9133764f07f67"),
    "win32-x64": ("cmake-%s-windows-x86_64.zip" % CMAKE, 54400562, "bace36e94b31c68ab6fa295f26dfa11219e0701cf7c94b0284a7d1cb13dac536"),
}
nj = {
    "darwin-arm64": ("ninja-mac.zip", 314051, "c99048673aa765960a99cf10c6ddb9f1fad506099ff0a0e137ad8960a88f321b"),
    "darwin-x64": ("ninja-mac.zip", 314051, "c99048673aa765960a99cf10c6ddb9f1fad506099ff0a0e137ad8960a88f321b"),
    "linux-arm64": ("ninja-linux-aarch64.zip", 126256, "fd2cacc8050a7f12a16a2e48f9e06fca5c14fc4c2bee2babb67b58be17a607fc"),
    "linux-x64": ("ninja-linux.zip", 134040, "5749cbc4e668273514150a80e387a957f933c6ed3f5f11e03fb30955e2bbead6"),
    "win32-arm64": ("ninja-winarm64.zip", 270354, "e52f0bdef9dfb1003229dbd6508a508c4073fd017247002adc66e5e806cb0391"),
    "win32-x64": ("ninja-win.zip", 291570, "07fc8261b42b20e71d1720b39068c2e14ffcee6396b76fb7a795fb460b78dc65"),
}
def cmake_rows():
    rows = []
    for host in HOSTS:
        archive, size, sha = cm[host]
        mac = host.startswith("darwin")
        win = host.startswith("win32")
        rows.append({"host": host, "archive": archive, "format": "zip" if archive.endswith(".zip") else "tar.gz", "bytes": size, "sha256": sha,
                     "prefix": archive.rsplit(".tar.gz", 1)[0] if archive.endswith(".tar.gz") else archive[:-4],
                     "binaryDirectory": "CMake.app/Contents/bin" if mac else "bin",
                     "binaries": ["cmake", "ctest", "cpack"] if not win else ["cmake.exe", "ctest.exe", "cpack.exe"]})
    return rows
GO = "1.26.9"
go = {
    "darwin-x64": ("go%s.darwin-amd64.tar.gz" % GO, 67810349, "00c29e3d4c8562f547410daa18d2959b641bd691ab00c4a525ea817a0253d4bd"),
    "darwin-arm64": ("go%s.darwin-arm64.tar.gz" % GO, 64675343, "f9bb7c0a02506c5d9bf0d1eb1f7ee6c7684f844ae49558308a0427b830e022cc"),
    "linux-x64": ("go%s.linux-amd64.tar.gz" % GO, 66935201, "42d158b4d8f7b61ac0a830567c940a86098fb7aac52e467a5ebec03ef5cc2f8d"),
    "linux-arm64": ("go%s.linux-arm64.tar.gz" % GO, 63830030, "4a97373d49fcacdcf3694fea368a500b00ee3e963974f3e7514132717632f052"),
    "win32-x64": ("go%s.windows-amd64.zip" % GO, 75002909, "d722201a9c0c086d1610e111c48203009af690892ed708072bd5ae20160e7a59"),
    "win32-arm64": ("go%s.windows-arm64.zip" % GO, 71522198, "05215f9e71ea27c9e843409314b1e6bf12dc6a5516318126b2357d497fed42c5"),
}
def go_rows():
    rows = []
    for host in HOSTS:
        archive, size, sha = go[host]
        win = host.startswith("win32")
        rows.append({"host": host, "archive": archive, "format": "zip" if win else "tar.gz", "bytes": size, "sha256": sha, "prefix": "go", "binaryDirectory": "bin", "binaries": ["go.exe", "gofmt.exe"] if win else ["go", "gofmt"]})
    return rows
def ninja_rows():
    rows = []
    for host in HOSTS:
        archive, size, sha = nj[host]
        win = host.startswith("win32")
        rows.append({"host": host, "archive": archive, "format": "zip", "bytes": size, "sha256": sha, "prefix": "", "binaryDirectory": ".", "binaries": ["ninja.exe" if win else "ninja"]})
    return rows
policy = {
    "$schema": "./🧬️schema/🔣️.json",
    "version": 1,
    "hosts": HOSTS,
    "roots": {"dependencies": ".cache/b/d/m", "tools": ".cache/b/t/m"},
    "python": {
        "uv": "0.11.6",
        "interpreters": {
            "main": {"series": "3.14"},
            "oracle": {"series": "3.12", "reason": {
                "en": "The compiled OpenStudio bindings publish no cp314 win_amd64 wheel (measured with `uv pip compile --python-version 3.14 --python-platform windows openstudio==3.11.0`), so the energy oracle stays on the widest common interpreter of the macOS, Linux and Windows wheel matrix.",
                "de": "Die kompilierten OpenStudio-Bindings veröffentlichen kein cp314-win_amd64-Wheel (gemessen mit `uv pip compile --python-version 3.14 --python-platform windows openstudio==3.11.0`); das Energie-Orakel bleibt daher auf dem breitesten gemeinsamen Interpreter der Wheel-Matrix von macOS, Linux und Windows."}},
        },
        "projects": [
            {"id": "workspace", "root": ".", "interpreter": "main", "groups": ["dev", "test"]},
            {"id": "ui-styling", "root": "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python", "interpreter": "main", "groups": ["build"]},
            {"id": "energy-oracle", "root": "✏️s/🔌️plugins/🔋️energy/🔮️oracles/📦️packages/🐍️python", "interpreter": "oracle", "groups": ["test"]},
        ],
        "layout": {"environments": "python/environments", "interpreters": "python/interpreters", "cache": "python/uv-cache"},
        "variables": {"UV_NO_CONFIG": "1", "UV_PYTHON_PREFERENCE": "only-managed", "UV_PYTHON_DOWNLOADS": "manual", "PYTHONDONTWRITEBYTECODE": "1"},
        "environment": ["PYTHONHASHSEED", "PYTHONOPTIMIZE", "PYTHONPATH", "UV_COMPILE_BYTECODE", "UV_EXTRA_INDEX_URL", "UV_INDEX_URL", "UV_NO_BINARY", "UV_NO_BUILD"],
    },
    "dotnet": {
        "sdk": "8.0.421",
        "rollForward": "disable",
        "file": "global.json",
        "artifacts": ".cache/b/p/m/dotnet/raw",
        "variables": {"DOTNET_CLI_TELEMETRY_OPTOUT": "1", "DOTNET_NOLOGO": "1", "DOTNET_SKIP_FIRST_TIME_EXPERIENCE": "1", "DOTNET_CLI_WORKLOAD_UPDATE_NOTIFY_DISABLE": "true"},
        "environment": ["Configuration", "DOTNET_ROLL_FORWARD", "RuntimeIdentifier"],
    },
    "cmake": {
        "cmake": {"version": CMAKE, "release": "https://github.com/Kitware/CMake/releases/download/v" + CMAKE, "versionArguments": ["--version"], "distributions": cmake_rows()},
        "ninja": {"version": NINJA, "release": "https://github.com/ninja-build/ninja/releases/download/v" + NINJA, "versionArguments": ["--version"], "distributions": ninja_rows()},
        "environment": ["CC", "CFLAGS", "CMAKE_GENERATOR", "CMAKE_TOOLCHAIN_FILE", "CXX", "CXXFLAGS", "DEVELOPER_DIR", "LDFLAGS", "MACOSX_DEPLOYMENT_TARGET", "SDKROOT"],
    },
    "go": {
        "version": GO,
        "work": "go.work",
        "tool": {"version": GO, "release": "https://dl.google.com/go", "versionArguments": ["version"], "distributions": go_rows()},
        "variables": {"GOTOOLCHAIN": "local"},
        "environment": ["CGO_CFLAGS", "CGO_ENABLED", "CGO_LDFLAGS", "GOARCH", "GOEXPERIMENT", "GOFLAGS", "GOOS"],
    },
    "extract": {
        "tar.gz": {"darwin": ["tar", "-xzf", "{archive}", "-C", "{destination}"], "linux": ["tar", "-xzf", "{archive}", "-C", "{destination}"], "win32": ["tar", "-xzf", "{archive}", "-C", "{destination}"]},
        "zip": {"darwin": ["tar", "-xf", "{archive}", "-C", "{destination}"], "linux": ["unzip", "-q", "-o", "{archive}", "-d", "{destination}"], "win32": ["tar", "-xf", "{archive}", "-C", "{destination}"]},
    },
    "messages": {
        "languages": ["en", "de"],
        "texts": {
            "host": {"en": "No pinned native toolchain for the host {host}.", "de": "Kein gepinntes natives Toolchain-Paket für den Host {host}."},
            "missing": {"en": "The pinned {tool} {version} is not provisioned; run `bun repo run {target}` (it downloads the checksum-pinned archive once).", "de": "Das gepinnte {tool} {version} ist nicht bereitgestellt; führe `bun repo run {target}` aus (es lädt das prüfsummen-gepinnte Archiv einmalig)."},
            "offline": {"en": "The pinned {tool} {version} is not provisioned and the network is offline.", "de": "Das gepinnte {tool} {version} ist nicht bereitgestellt und das Netzwerk ist offline."},
            "download": {"en": "Downloading the pinned {tool} {version} for {host} ({bytes} bytes).", "de": "Lade das gepinnte {tool} {version} für {host} ({bytes} Bytes)."},
            "checksum": {"en": "The archive of {tool} {version} does not match its pinned size and SHA-256.", "de": "Das Archiv von {tool} {version} entspricht nicht der gepinnten Größe und dem SHA-256."},
            "member": {"en": "The archive of {tool} {version} holds an unsafe member: {member}.", "de": "Das Archiv von {tool} {version} enthält ein unsicheres Element: {member}."},
            "binary": {"en": "The archive of {tool} {version} holds no {binary}.", "de": "Das Archiv von {tool} {version} enthält kein {binary}."},
            "receipt": {"en": "The installation of {tool} {version} differs from its receipt: {path}.", "de": "Die Installation von {tool} {version} weicht von ihrer Quittung ab: {path}."},
            "python": {"en": "The pinned interpreter {series} is not provisioned; run `bun repo run deps-python`.", "de": "Der gepinnte Interpreter {series} ist nicht bereitgestellt; führe `bun repo run deps-python` aus."},
            "pin": {"en": "{tool} {actual} differs from the pin {expected} ({source}).", "de": "{tool} {actual} weicht vom Pin {expected} ab ({source})."},
            "usage": {"en": "{command} takes {choices}.", "de": "{command} erwartet {choices}."},
            "project": {"en": "Unknown Python project: {project}.", "de": "Unbekanntes Python-Projekt: {project}."},
            "digests": {"en": "The digests of the manifest and the lock are required.", "de": "Die Prüfsummen von Manifest und Lock sind erforderlich."},
            "file": {"en": "{file} of the Python project {project} is missing.", "de": "{file} des Python-Projekts {project} fehlt."},
            "directory": {"en": "A directory of the native tools is not a plain directory: {path}.", "de": "Ein Verzeichnis der nativen Werkzeuge ist kein gewöhnliches Verzeichnis: {path}."},
            "http": {"en": "The download failed with status {status}: {url}.", "de": "Der Download scheiterte mit Status {status}: {url}."},
            "depth": {"en": "The archive nests deeper than the finite ceiling allows.", "de": "Das Archiv ist tiefer verschachtelt, als die endliche Grenze erlaubt."},
            "size": {"en": "The archive holds more entries than the finite ceiling allows.", "de": "Das Archiv enthält mehr Einträge, als die endliche Grenze erlaubt."},
            "link": {"en": "The archive link leaves its tree: {path} -> {target}.", "de": "Ein Archiv-Link verlässt seinen Baum: {path} -> {target}."},
            "special": {"en": "The archive holds a special file: {path}.", "de": "Das Archiv enthält eine Spezialdatei: {path}."},
        },
    },
}
out = os.path.join(HERE, "..", "staged", "toolchains-policy.json")
open(out, "w", encoding="utf8").write(json.dumps(policy, ensure_ascii=False, indent=2) + "\n")
print("wrote", os.path.relpath(out, HERE))
