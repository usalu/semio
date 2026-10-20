#!/usr/bin/env python3
"""🧾 Dependency-key differential of every Cargo.toml that differs between two trees, parsed by Bun's TOML parser (third-party oracle). Compares the result with the carve's expected edge set.

usage: verify_manifests.py <pre-root> <post-root>
"""
import json, os, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
V = "🧰️framework/🔨️modules/🌱️value"
EXPECT = {
    f"{V}/📦️packages/🦀️rust/Cargo.toml": ({}, {"semio-framework-value-clone-source", "semio-framework-value-native-decoding", "semio-framework-value-paged-list", "semio-framework-value-allocation-return", "semio-framework-hash", "semio-framework-value-edit", "semio-framework-value-schema", "semio-framework-value-registry", "semio-framework-schema-registry"}),
    f"{V}/✨️derive/📦️packages/🦀️rust/Cargo.toml": ({"semio-framework-value-derive-resources"}, {"semio-framework-hash", "semio-framework-native-build-compiler-resources"}),
    f"{V}/📋️list/🔺️edit/📦️packages/🦀️rust/Cargo.toml": (set(), {"semio-framework-value-native-encoding"}),
    f"{V}/📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml": (set(), {"semio-framework-value-original-aliases"}),
    f"{V}/🌳️intrinsic/📦️packages/🦀️rust/Cargo.toml": (set(), {"semio-framework-value-ordered"}),
    f"{V}/🗂️ordered/📦️packages/🦀️rust/Cargo.toml": ({"semio-framework-value-intrinsic"}, set()),
    "🧰️framework/🛍️products/📓️print/📦️packages/🦀️rust/Cargo.toml": ({"semio-framework-value-derive-resources"}, set()),
    f"{V}/✨️derive/🚪️io/📝️text/📸️snapshot/📦️packages/🦀️rust/Cargo.toml": ({"syn", "quote", "proc-macro2", "semio-framework-native-build-compiler-resources"}, set()),
}


def keys(path):
    if not os.path.isfile(path):
        return {"dependencies": [], "devDependencies": []}
    return json.loads(subprocess.run(["bun", HERE + "/toml_keys.ts", path], capture_output=True, text=True, check=True).stdout)


def main():
    pre, post = sys.argv[1], sys.argv[2]
    bad = 0
    for rel, (added, removed) in EXPECT.items():
        a, b = keys(os.path.join(pre, rel)), keys(os.path.join(post, rel))
        got_add = (set(b["dependencies"]) | set(b["devDependencies"])) - (set(a["dependencies"]) | set(a["devDependencies"]))
        got_rem = (set(a["dependencies"]) | set(a["devDependencies"])) - (set(b["dependencies"]) | set(b["devDependencies"]))
        ok = got_add == set(added) and got_rem == set(removed)
        bad += not ok
        print(("ok  " if ok else "FAIL"), rel[-72:], "+", sorted(got_add), "-", sorted(got_rem))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
