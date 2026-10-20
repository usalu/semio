#!/usr/bin/env python3
# Freezes the realistic change sets of the affected cross-check from the working tree against HEAD (usage: change-sets.py; writes ../🗑️generated/bzl-lazy/change-sets.json).
import json, os, subprocess
HERE = os.path.dirname(os.path.abspath(__file__)); G = HERE + "/../🗑️generated/bzl-lazy"
names = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "diff", "--name-only", "HEAD", "--", ":!.🧬semio"], cwd="/Users/ueli/Documents/semio", capture_output=True, text=True).stdout.splitlines()
under = lambda *pre: [n for n in names if any(n.startswith(p) for p in pre)]
root = [n for n in names if "/" not in n and not n.endswith("bun.lock")]
sets = {
    "framework-build-library": under("🧰️framework/🛍️products/🦑️repo/"),
    "plugins-layout-and-writer": under("✏️s/🔌️plugins/📏️layout/", "✏️s/🔌️plugins/✒️writer/"),
    "root-manifests-teaching-mit-bestand": root + [n for n in under("🎓️teaching/", "♻️mit-bestand/", ".devcontainer/", ".cargo/", ".codex/") if not n.endswith("bun.lock")],
    "single-file-hub": ["🌎️hub/📦️packages/🟦️typescript/package.json"],
    "lockfile-only-known-divergence": ["bun.lock"],
}
os.makedirs(G, exist_ok=True)
json.dump(sets, open(G + "/change-sets.json", "w"), ensure_ascii=False)
print({k: len(v) for k, v in sets.items()})
