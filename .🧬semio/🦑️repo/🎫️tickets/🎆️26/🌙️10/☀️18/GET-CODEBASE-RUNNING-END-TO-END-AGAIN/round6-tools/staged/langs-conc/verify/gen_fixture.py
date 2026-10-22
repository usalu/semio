#!/usr/bin/env python3
# 🧾 Generates the language-agnostic toolchain fixture (staged/toolchains-fixture.json): input rows only, the expected results come from the independent oracle at test time. Publisher checksums are transcribed from the publishers' own listings (CMake: cmake-4.4.4-SHA-256.txt; Ninja: GitHub release asset digests of v1.13.2).
import hashlib, json, os
HERE = os.path.dirname(os.path.abspath(__file__))
policy = json.load(open(os.path.join(HERE, "..", "staged", "toolchains-policy.json"), encoding="utf8"))
def h(text): return hashlib.sha256(text.encode()).hexdigest()
roots = [{"dependencies": "/work/a/.cache/b/d", "tools": "/work/a/.cache/b/t"}, {"dependencies": "/other/checkout/.cache/b/d", "tools": "/other/checkout/.cache/b/t"}]
environments = []
for project in policy["python"]["projects"]:
    for host in ("darwin-arm64", "win32-x64"):
        for index, root in enumerate(roots):
            for variant in ("a", "b"):
                environments.append({"name": "%s %s root%d %s" % (project["id"], host, index, variant), "project": project["id"], "host": host, "roots": root, "manifest": h("manifest-" + project["id"]), "lock": h("lock-" + project["id"] + variant)})
identities = []
base_env = {"PYTHONHASHSEED": "0", "UV_INDEX_URL": "https://example.invalid/simple", "GOFLAGS": "-mod=mod", "CC": "clang", "CXX": "clang++", "Configuration": "Release", "HOME": "/not/declared", "PATH": "/not/declared"}
for host in policy["hosts"]:
    for kind in ("python", "dotnet", "cmake", "go"):
        identities.append({"name": kind + " " + host, "kind": kind, "host": host, "environment": {}})
        identities.append({"name": kind + " " + host + " environment", "kind": kind, "host": host, "environment": base_env})
for project in policy["python"]["projects"]:
    identities.append({"name": "python " + project["id"], "kind": "python", "host": "linux-x64", "project": project["id"], "environment": base_env})
tools = [{"name": "%s %s" % (tool, host), "tool": tool, "host": host, "root": "/work/a/.cache/b/t"} for tool in ("cmake", "ninja", "go") for host in policy["hosts"]]
links = [
    {"name": "sibling", "path": "bin/cmake", "target": "../CMake.app/Contents/bin/cmake"},
    {"name": "same directory", "path": "share/a", "target": "b"},
    {"name": "deep inside", "path": "a/b/c/d", "target": "../../x"},
    {"name": "climbs to the root", "path": "a/b", "target": "../../.."},
    {"name": "climbs above", "path": "bin/x", "target": "../../outside"},
    {"name": "absolute", "path": "bin/x", "target": "/usr/bin/cmake"},
    {"name": "drive", "path": "bin/x", "target": "C:\\Windows\\cmd.exe"},
    {"name": "backslash climb", "path": "bin\\x", "target": "..\\..\\y"},
    {"name": "current directory", "path": "x", "target": "./y/../z"},
]
publisher = {
    "cmake": {"source": "https://github.com/Kitware/CMake/releases/download/v4.4.4/cmake-4.4.4-SHA-256.txt", "lines": [
        "a1b6cc63636a0e55c63257cf3315a8a5f129e42fade25db1afea4ff8ab06f25e  cmake-4.4.4-linux-aarch64.tar.gz",
        "e5bb807f7728cb60cd8b27ebc97a2edb469b68655f21e844a600c3575b76f5bb  cmake-4.4.4-linux-x86_64.tar.gz",
        "4b7b73704b1db9b374e5c9ab8e17ac6148b817b6396ee75cd94a852cbac9d305  cmake-4.4.4-macos-universal.tar.gz",
        "ed673bbc4eb7c1e59b0407fd52f1680fc0ae3229a46b9b7b3ea9133764f07f67  cmake-4.4.4-windows-arm64.zip",
        "bace36e94b31c68ab6fa295f26dfa11219e0701cf7c94b0284a7d1cb13dac536  cmake-4.4.4-windows-x86_64.zip"]},
    "ninja": {"source": "https://api.github.com/repos/ninja-build/ninja/releases/tags/v1.13.2 (assets[].digest)", "lines": [
        "fd2cacc8050a7f12a16a2e48f9e06fca5c14fc4c2bee2babb67b58be17a607fc  ninja-linux-aarch64.zip",
        "5749cbc4e668273514150a80e387a957f933c6ed3f5f11e03fb30955e2bbead6  ninja-linux.zip",
        "c99048673aa765960a99cf10c6ddb9f1fad506099ff0a0e137ad8960a88f321b  ninja-mac.zip",
        "07fc8261b42b20e71d1720b39068c2e14ffcee6396b76fb7a795fb460b78dc65  ninja-win.zip",
        "e52f0bdef9dfb1003229dbd6508a508c4073fd017247002adc66e5e806cb0391  ninja-winarm64.zip"]},
    "go": {"source": "https://go.dev/dl/?mode=json&include=all (go1.26.9 files[].sha256)", "lines": [
        "00c29e3d4c8562f547410daa18d2959b641bd691ab00c4a525ea817a0253d4bd  go1.26.9.darwin-amd64.tar.gz",
        "f9bb7c0a02506c5d9bf0d1eb1f7ee6c7684f844ae49558308a0427b830e022cc  go1.26.9.darwin-arm64.tar.gz",
        "42d158b4d8f7b61ac0a830567c940a86098fb7aac52e467a5ebec03ef5cc2f8d  go1.26.9.linux-amd64.tar.gz",
        "4a97373d49fcacdcf3694fea368a500b00ee3e963974f3e7514132717632f052  go1.26.9.linux-arm64.tar.gz",
        "d722201a9c0c086d1610e111c48203009af690892ed708072bd5ae20160e7a59  go1.26.9.windows-amd64.zip",
        "05215f9e71ea27c9e843409314b1e6bf12dc6a5516318126b2357d497fed42c5  go1.26.9.windows-arm64.zip"]},
}
messages = [{"name": "english", "key": "missing", "values": {"tool": "cmake", "version": "4.4.4"}, "environment": {"SEMIO_LANGUAGE": "en"}},
            {"name": "german", "key": "missing", "values": {"tool": "cmake", "version": "4.4.4"}, "environment": {"SEMIO_LANGUAGE": "de"}},
            {"name": "unresolved shows both", "key": "host", "values": {"host": "freebsd-x64"}, "environment": {}}]
out = {"version": 1, "roots": roots, "environments": environments, "identities": identities, "tools": tools, "links": links, "publisher": publisher, "messages": messages}
open(os.path.join(HERE, "..", "staged", "toolchains-fixture.json"), "w", encoding="utf8").write(json.dumps(out, ensure_ascii=False, indent=2) + "\n")
print("fixture rows:", len(environments), len(identities), len(tools), len(links))
