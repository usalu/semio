#!/usr/bin/env python3
"""🧱️ R10 prepared window-3 step (session 14b): after the host modules reach three.js only through the ui module
(`verify interface-owners` PASS), the two packages that no longer import it in production — the infinite world r3f
package and the renderer's React target — keep `three` only for their tests (oracle use): the declaration moves from
`dependencies` to `devDependencies` in each `package.json` AND in that workspace's block of the root `bun.lock`, textually
(no `bun install` resolution, so `node_modules` is untouched). Idempotent; exact-once anchors; dry run by default.
After `--apply`: `bun install --frozen-lockfile --dry-run` must report no change, then `verify dependencies literal-external`
(three's oracle conflict users shrink to none: the ui module's declaration is the interface owner's).
Usage: python3 three-manifests.py [--apply]
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PACKAGES = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript",
]
NAME = "three"


def move_in_manifest(text: str) -> str:
    manifest = json.loads(text)
    if NAME in manifest.get("devDependencies", {}) and NAME not in manifest.get("dependencies", {}):
        return text
    version = manifest["dependencies"][NAME]
    line = re.compile(r'\n(\s*)"three": "[^"]+",?(?=\n)')
    deps = re.search(r'"dependencies": \{[^}]*\}', text)
    block = deps.group(0)
    moved = line.sub("", block, count=1)
    moved = re.sub(r",(\n\s*\})$", r"\1", moved)
    text = text[: deps.start()] + moved + text[deps.end():]
    dev = re.search(r'"devDependencies": \{([^}]*)\}', text)
    entries = [entry.strip() for entry in dev.group(1).strip().split("\n") if entry.strip()]
    indent = re.search(r"\n(\s*)\"", dev.group(0)).group(1)
    entries = [entry.rstrip(",") for entry in entries] + [f'"{NAME}": "{version}"']
    entries.sort(key=lambda entry: entry.split('"')[1])
    closing = re.search(r"\n(\s*)\}$", dev.group(0)).group(1)
    rendered = '"devDependencies": {\n' + ",\n".join(f"{indent}{entry}" for entry in entries) + f"\n{closing}}}"
    text = text[: dev.start()] + rendered + text[dev.end():]
    json.loads(text)
    return text


def move_in_lock(text: str, workspace: str) -> str:
    start = text.index(f'    "{workspace}": {{\n')
    end = text.index("\n    },\n", start)
    block = text[start:end]
    deps_at = block.index('      "dependencies": {\n')
    deps_end = block.index("      },\n", deps_at)
    deps = block[deps_at:deps_end]
    line = re.search(r'        "three": "[^"]+",\n', deps)
    if line is None:
        return text
    dev_at = block.index('      "devDependencies": {\n')
    dev_end = block.index("      },", dev_at)
    dev_lines = block[dev_at + len('      "devDependencies": {\n'):dev_end].splitlines(keepends=True) + [line.group(0)]
    dev_lines.sort(key=lambda entry: entry.split('"')[1])
    block = block[:deps_at] + deps.replace(line.group(0), "", 1) + block[deps_end:dev_at] + '      "devDependencies": {\n' + "".join(dev_lines) + block[dev_end:]
    return text[:start] + block + text[end:]


def main() -> int:
    apply = "--apply" in sys.argv
    lock_path = ROOT / "bun.lock"
    lock = lock_path.read_text(encoding="utf-8")
    writes: dict[Path, str] = {}
    for package in PACKAGES:
        path = ROOT / package / "package.json"
        before = path.read_text(encoding="utf-8")
        after = move_in_manifest(before)
        if after != before:
            writes[path] = after
        lock = move_in_lock(lock, package)
    if lock != lock_path.read_text(encoding="utf-8"):
        writes[lock_path] = lock
    for path, text in writes.items():
        print(f"{'write' if apply else 'would write'} {path.relative_to(ROOT)}")
        if apply:
            path.write_text(text, encoding="utf-8")
    print("nothing to do" if not writes else "applied" if apply else "dry run clean")
    return 0


if __name__ == "__main__":
    sys.exit(main())
