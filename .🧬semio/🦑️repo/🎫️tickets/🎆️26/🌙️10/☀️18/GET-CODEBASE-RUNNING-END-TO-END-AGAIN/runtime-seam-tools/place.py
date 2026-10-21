#!/usr/bin/env python3
"""Places ASCII-named sources from src/ at their emoji paths in the staged tree (and optionally a dev overlay)."""
import os, shutil, sys
V = "️"
def n(e, s=""): return e + V + s
HERE = os.path.dirname(os.path.abspath(__file__))
MODULE = "/".join([n("🧰", "framework"), n("🔨", "modules"), n("⚙", "runtime")])
TS = n("🟦") + ".ts"
TREE = {
    "contract.ts": [n("📋", "contract"), TS],
    "schema.json": [n("🧬", "schema"), n("🔣") + ".json"],
    "selection.ts": [n("🧭", "selection"), TS],
    "facade.ts": [TS],
    "bun.ts": [n("🍞", "bun"), TS],
    "node.ts": [n("🟢", "node"), TS],
    "glob.ts": [n("🪶", "portable"), n("🌐", "glob"), TS],
    "toml.ts": [n("🪶", "portable"), n("📄", "toml"), TS],
    "jsonc.ts": [n("🪶", "portable"), n("📝", "jsonc"), TS],
    "which.ts": [n("🪶", "portable"), n("🔎", "which"), TS],
    "resolve.ts": [n("🪶", "portable"), n("🧭", "resolve"), TS],
    "spawn.ts": [n("🪶", "portable"), n("🚀", "spawn"), TS],
    "serve.ts": [n("🪶", "portable"), n("🌍", "serve"), TS],
    "sqlite.ts": [n("🪶", "portable"), n("🗃", "sqlite"), TS],
    "semver.ts": [n("🪶", "portable"), n("🔢", "semver"), TS],
    "stdin.ts": [n("🪶", "portable"), n("📥", "stdin"), TS],
    "file.ts": [n("🪶", "portable"), n("📂", "file"), TS],
    "source.ts": [n("🪶", "portable"), n("🔬", "source"), TS],
    "yaml.ts": [n("🪶", "portable"), n("📃", "yaml"), TS],
    "plugin.ts": [n("🟢", "node"), n("🔌", "plugin"), TS],
    "esbuild.ts": [n("🟢", "node"), n("📦", "esbuild"), TS],
    "native.ts": [n("🟢", "node"), n("🧲", "native"), TS],
    "testapi-node.ts": [n("🟢", "node"), n("🧪", "test"), TS],
    "testapi-bun.ts": [n("🍞", "bun"), n("🧪", "test"), TS],
    "testapi.ts": [n("🧪", "test"), TS],
    "missing.ts": [n("🚫", "missing"), TS],
    "expect.ts": [n("🪶", "portable"), n("✅", "expect"), TS],
    "main.ts": [n("🪶", "portable"), n("🏁", "main"), TS],
    "law.ts": [n("⚖", "law"), TS],
    "ledger.json": [n("⚖", "law"), n("📒") + ".json"],
    "tests.ts": [n("🧪", "tests"), TS],
    "fixtures.json": [n("🧪", "tests"), n("🧫", "fixtures"), n("🔣") + ".json"],
    "native-provider.cjs": [n("🧪", "tests"), n("🧫", "fixtures"), n("🧲", "native-provider") + ".cjs"],
    "probe-plugin.ts": [n("🧪", "tests"), n("🧫", "fixtures"), n("🔌", "probe-plugin") + ".ts"],
    "probe-test.ts": [n("🧪", "tests"), n("🧫", "fixtures"), n("🧪", "probe-test") + ".ts"],
}
def target(root, key): return os.path.join(root, MODULE, *TREE[key])
def main():
    roots = [os.path.join(HERE, "staged", "new")] + sys.argv[1:]
    for key in sorted(TREE):
        src = os.path.join(HERE, "src", key)
        if not os.path.exists(src): continue
        for root in roots:
            dst = target(root, key)
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(src, dst)
    print("placed into", roots)
if __name__ == "__main__": main()
