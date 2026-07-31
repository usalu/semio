#!/usr/bin/env python3
"""Fast collapse of consecutive identical emoji doubles + fix stale Cargo/storybook paths."""
from __future__ import annotations

import os
import re
from pathlib import Path

REPO = Path(__file__).resolve().parents[6]
os.chdir(REPO)
TICKET = Path(__file__).resolve().parent

SKIP_DIRS = frozenset(
    {
        "node_modules",
        "target",
        ".git",
        ".nx",
        "dist",
        "pkg",
        ".repo-cache",
        "storybook-static",
        "_vendor",
        "⚡️cache",
    }
)
SKIP_FILES = frozenset({"AGENTS.md"})

# Same codepoint repeated 2+ times within common emoji ranges
DOUBLE_RE = re.compile(
    r"([\U0001F300-\U0001FAFF\u2600-\u27BF\u2300-\u23FF\u2B00-\u2BFF])\1+"
)


def collapse(text: str) -> str:
    return DOUBLE_RE.sub(r"\1", text)


changed: list[str] = []
for dirpath, dirnames, filenames in os.walk(".", topdown=True):
    parts = set(Path(dirpath).parts)
    if parts & SKIP_DIRS:
        dirnames[:] = []
        continue
    dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
    for name in filenames:
        if name in SKIP_FILES:
            continue
        path = Path(dirpath) / name
        try:
            raw = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError, IsADirectoryError):
            continue
        if not DOUBLE_RE.search(raw):
            continue
        new = collapse(raw)
        if new != raw:
            path.write_text(new, encoding="utf-8")
            changed.append(str(path))

(TICKET / "collapsed-double-emoji-files.txt").write_text(
    "\n".join(changed) + ("\n" if changed else ""), encoding="utf-8"
)
print(f"collapsed doubles in {len(changed)} files")

PLACEHOLDER = "🧰️framework/🔨️module/🖼️asset/⚡️implementation/🟦️typescript/🥽️mesh/🧊️placeholder.glb"
ARCH = Path("✏️s/🔌️plugin/🏛️architect/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust/Cargo.toml")
HUB = Path("compose/server/hub/rs/Cargo.toml")
STORIES = Path(".storybook/stories/puzzle/3d/World.stories.tsx")

arch_deps = """vcs = { path = "../../../../../../../🧰️framework/🛍️product/💻️os/🔨️module/🌿️vcs/⚡️implementation/🦀️rust" }
store = { path = "../../../../../../../🧰️framework/🛍️product/💻️os/🔨️module/🏪️store/⚡️implementation/🦀️rust" }
protocol = { path = "../../../../../../../🧰️framework/🛍️product/💻️os/🔨️module/📡️protocol/⚡️implementation/🦀️rust" }
semio-framework-plugin = { path = "../../../../../../../🧰️framework/🛍️product/💻️os/🔨️module/🔌️plugin/⚡️implementation/🦀️rust", features = ["component-guest"]}
semio-framework-core = { path = "../../../../../../../🧰️framework/⚡️implementation/🦀️rust" }
"""

if ARCH.exists():
    text = ARCH.read_text(encoding="utf-8")
    text2 = re.sub(
        r"vcs = \{ path = \"[^\"]+\" \}\n"
        r"store = \{ path = \"[^\"]+\" \}\n"
        r"protocol = \{ path = \"[^\"]+\" \}\n"
        r"semio-framework-plugin = \{ path = \"[^\"]+\", features = \[\"component-guest\"\]\}\n"
        r"semio-framework-core = \{ path = \"[^\"]+\" \}\n",
        arch_deps,
        text,
        count=1,
    )
    if text2 == text:
        # maybe already fixed
        if "framework/product/os/module/vcs/rs" in text:
            raise SystemExit("architect Cargo.toml deps block not matched")
        print("architect Cargo.toml already plain-emoji")
    else:
        ARCH.write_text(text2, encoding="utf-8")
        print("fixed architect Cargo.toml deps")

if HUB.exists():
    text = HUB.read_text(encoding="utf-8")
    text2 = text.replace(
        'db = { path = "../../../../framework/os/kernel/db/rs" }',
        'db = { path = "../../../../🧰️framework/🛍️product/💻️os/🔨️module/🛢️db/⚡️implementation/🦀️rust" }',
    ).replace(
        'protocol = { path = "../../../../framework/os/kernel/protocol/rs" }',
        'protocol = { path = "../../../../🧰️framework/🛍️product/💻️os/🔨️module/📡️protocol/⚡️implementation/🦀️rust" }',
    )
    if text2 != text:
        HUB.write_text(text2, encoding="utf-8")
        print("fixed compose hub Cargo.toml deps")
    else:
        print("compose hub Cargo.toml already fixed or unmatched")

for rel in [
    "🧰️framework/🛍️product/🦑️repo/🔨️module/⌨️cli/⚡️implementation/🦀️rust/📦️lib.rs",
    "🧰️framework/🛍️product/💻️os/🔨️module/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementation/🦀️rust/🟦️typescript/🟨️boot.js",
]:
    p = Path(rel)
    if not p.exists():
        print("skip missing", rel)
        continue
    text = p.read_text(encoding="utf-8")
    text2 = text.replace("framework/asset/mesh/🧊️placeholder.glb", PLACEHOLDER)
    if text2 != text:
        p.write_text(text2, encoding="utf-8")
        print("fixed placeholder in", rel)

if STORIES.exists():
    text = STORIES.read_text(encoding="utf-8")
    replacements = {
        "../../../../framework/product/os/module/renderer/js/react/index.tsx":
            "../../../../🧰️framework/🛍️product/💻️os/🔨️module/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementation/🟦️typescript/📦️index.tsx",
        "../../../../framework/product/os/module/infinite/fixture/abbau-aufbau-masterarbeit-grundriss.jpg":
            "../../../../🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🧫️fixture/🖼️abbau-aufbau-masterarbeit-grundriss.jpg",
        "../../../../framework/product/os/module/infinite/fixture/rathaus-ahlen-grundriss.png":
            "../../../../🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🧫️fixture/🖼️rathaus-ahlen-grundriss.png",
    }
    text2 = text
    for a, b in replacements.items():
        text2 = text2.replace(a, b)
    if text2 != text:
        STORIES.write_text(text2, encoding="utf-8")
        print("fixed World.stories.tsx imports")
    else:
        print("World.stories.tsx: no old imports found")

# verify cargo targets
for cargo_rel, checks in [
    (
        str(ARCH),
        [
            "../../../../../../../🧰️framework/🛍️product/💻️os/🔨️module/🌿️vcs/⚡️implementation/🦀️rust",
            "../../../../../../../🧰️framework/🛍️product/💻️os/🔨️module/🏪️store/⚡️implementation/🦀️rust",
            "../../../../../../../🧰️framework/🛍️product/💻️os/🔨️module/📡️protocol/⚡️implementation/🦀️rust",
            "../../../../../../../🧰️framework/🛍️product/💻️os/🔨️module/🔌️plugin/⚡️implementation/🦀️rust",
            "../../../../../../../🧰️framework/⚡️implementation/🦀️rust",
        ],
    ),
    (
        str(HUB),
        [
            "../../../../🧰️framework/🛍️product/💻️os/🔨️module/🛢️db/⚡️implementation/🦀️rust",
            "../../../../🧰️framework/🛍️product/💻️os/🔨️module/📡️protocol/⚡️implementation/🦀️rust",
        ],
    ),
]:
    base = Path(cargo_rel).parent
    for rel in checks:
        tgt = (base / rel).resolve()
        print(("OK" if tgt.exists() else "MISSING"), rel)

left_files = []
for dirpath, dirnames, filenames in os.walk(".", topdown=True):
    parts = set(Path(dirpath).parts)
    if parts & SKIP_DIRS or "🎫️tickets" in parts:
        dirnames[:] = []
        continue
    dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
    for name in filenames:
        if name in SKIP_FILES:
            continue
        path = Path(dirpath) / name
        try:
            text = path.read_text(encoding="utf-8")
        except Exception:
            continue
        if DOUBLE_RE.search(text):
            left_files.append(str(path))
print(f"files still with identical-emoji runs: {len(left_files)}")
for f in left_files[:20]:
    print(" ", f)
