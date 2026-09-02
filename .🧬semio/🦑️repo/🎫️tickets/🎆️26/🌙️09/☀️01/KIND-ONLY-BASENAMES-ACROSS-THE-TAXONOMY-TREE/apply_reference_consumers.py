#!/usr/bin/env python3
"""Applies the nested-cargo-packages-v1 wgpu-renderer referenceTokenTransforms to the outside
consumer files that are NOT part of the package's own mappings (those are handled separately).
Generated files (bun.lock, 🔒️dependencies.json) are intentionally left for their own tool to
regenerate. .vscode/launch.json IS updated directly (deterministic derivation of the authored seed;
no quick regenerate path was available) alongside its authored seed + generator source.
"""
import argparse, json, os, sys

CAT_PATH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️nested-cargo-package-projection/🔣️.json"

TASKS = [
    (".storybook/scopes.ts", "wgpu-root"),
    (".vscode/launch.json", "wgpu-root"),
    (".vscode/🧩️launch.seed.jsonc", "wgpu-root"),
    ("Cargo.toml", "wgpu-root"),
    ("package.json", "wgpu-root"),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🖥️launch.ts", "wgpu-root"),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/⚙️vite.config.ts", "wgpu-root"),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts", "wgpu-root"),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs", "wgpu-engine-relative"),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️.rs", "wgpu-owner-glue"),
    ("🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation.rs", "wgpu-owner-glue"),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs", "wgpu-owner-runtime-shard"),
    ("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs", "wgpu-owner-runtime-self"),
]

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()
    apply = args.apply

    with open(CAT_PATH, encoding="utf-8") as f:
        cat = json.load(f)
    transforms = cat["referenceTokenTransforms"]
    expected_counts = {rc["path"]: rc["occurrenceCount"] for rc in cat["referenceConsumers"] if rc["packageId"] == "wgpu-renderer"}

    results = []
    for path, tid in TASKS:
        t = transforms[tid]
        src, dst = t["sourceToken"], t["destinationToken"]
        if not os.path.exists(path):
            results.append({"path": path, "status": "MISSING"})
            continue
        content = open(path, "r", encoding="utf-8").read()
        count = content.count(src)
        expected = expected_counts.get(path)
        status = f"count={count} expected={expected}"
        results.append({"path": path, "transform": tid, "status": status})
        if apply and count > 0:
            content = content.replace(src, dst)
            open(path, "w", encoding="utf-8").write(content)

    print(json.dumps(results, ensure_ascii=False, indent=2))

if __name__ == "__main__":
    main()
