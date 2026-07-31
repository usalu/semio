#!/usr/bin/env python3
"""Strip U+FE0F/U+FE0E from path basenames + text; fix stale launch/mcp script paths."""
from __future__ import annotations

import json
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
    }
)
SKIP_FILES = frozenset({"AGENTS.md"})
VS = ("\ufe0f", "\ufe0e")

OLD_WGPU = "bun ./framework/product/os/module/renderer/wgpu/script.ts"
NEW_WGPU = "bun ./🧰️framework/🛍️product/💻️os/🔨️module/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementation/🦀️rust/📜️script.ts"
OLD_REPO_CLI = "bun ./repo/cli/rs/script.ts"
NEW_REPO_CLI = "bun ./🧰️framework/🛍️product/🦑️repo/🔨️module/⌨️cli/⚡️implementation/🦀️rust/📜️script.ts"


def should_skip_dir(parts: list[str]) -> bool:
    for p in parts:
        if p in SKIP_DIRS:
            return True
    return False


def plain(s: str) -> str:
    for v in VS:
        s = s.replace(v, "")
    return s


# region Filesystem renames
renamed: list[str] = []
for dirpath, dirnames, filenames in os.walk(".", topdown=False):
    parts = Path(dirpath).parts
    if should_skip_dir(list(parts)):
        continue
    for name in filenames + dirnames:
        if not any(v in name for v in VS):
            continue
        new_name = plain(name)
        if new_name == name:
            continue
        old_path = Path(dirpath) / name
        new_path = Path(dirpath) / new_name
        if new_path.exists():
            raise SystemExit(f"collision: {old_path} -> {new_path}")
        old_path.rename(new_path)
        renamed.append(f"{old_path} -> {new_path}")

(TICKET / "renamed-paths.txt").write_text("\n".join(renamed) + ("\n" if renamed else ""), encoding="utf-8")
print(f"renamed {len(renamed)} filesystem entries")
# endregion

# region Text FE0F/FE0E strip
text_changed: list[str] = []
for dirpath, dirnames, filenames in os.walk(".", topdown=True):
    parts = list(Path(dirpath).parts)
    if should_skip_dir(parts):
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
        if not any(v in raw for v in VS):
            continue
        new = plain(raw)
        if new != raw:
            path.write_text(new, encoding="utf-8")
            text_changed.append(str(path))

(TICKET / "text-vs16-stripped.txt").write_text(
    "\n".join(text_changed) + ("\n" if text_changed else ""), encoding="utf-8"
)
print(f"stripped VS from {len(text_changed)} text files")
# endregion

# region Stale path fixes in configs
CONFIG_FIXES = [
    # Root entrypoint (package.json already correct; launch/mcp often stale)
    ("bun ./script.ts", "bun ./📜️script.ts"),
    ('"args": ["script.ts"', '"args": ["📜️script.ts"'),
    ("bun script.ts ", "bun 📜️script.ts "),
    (OLD_WGPU, NEW_WGPU),
    (OLD_REPO_CLI, NEW_REPO_CLI),
    # Pre-strip forms that may remain if a file was skipped
    (
        "bun ./🧰️framework/🛍️product/💻️os/🔨️module/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementation/🦀️rust/📜️script.ts",
        NEW_WGPU,
    ),
    (
        "bun ./🧰️framework/🛍️product/🦑️repo/🔨️module/⌨️cli/⚡️implementation/🦀️rust/📜️script.ts",
        NEW_REPO_CLI,
    ),
]

config_hits: dict[str, int] = {a: 0 for a, _ in CONFIG_FIXES}
config_files: list[str] = []
CONFIG_GLOBS = [
    ".vscode/launch.json",
    ".vscode/mcp.json",
    ".mcp.json",
    ".devcontainer/devcontainer.json",
    "package.json",
]

extra_targets: list[Path] = []
for rel in CONFIG_GLOBS:
    p = Path(rel)
    if p.is_file():
        extra_targets.append(p)

# Also sweep any remaining ./script.ts (no emoji) in repo text outside skips
SCRIPT_RE = re.compile(r"(?<![📜️\w./-])\./script\.ts\b")
MCP_SCRIPT_RE = re.compile(r'\["script\.ts"')

for dirpath, dirnames, filenames in os.walk(".", topdown=True):
    parts = list(Path(dirpath).parts)
    if should_skip_dir(parts):
        dirnames[:] = []
        continue
    dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
    for name in filenames:
        if name in SKIP_FILES:
            continue
        if not name.endswith(
            (
                ".json",
                ".ts",
                ".tsx",
                ".js",
                ".mjs",
                ".cjs",
                ".md",
                ".toml",
                ".yml",
                ".yaml",
                ".sh",
                ".ps1",
                ".html",
            )
        ):
            continue
        path = Path(dirpath) / name
        try:
            raw = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        new = raw
        for a, b in CONFIG_FIXES:
            if a in new:
                count = new.count(a)
                new = new.replace(a, b)
                config_hits[a] = config_hits.get(a, 0) + count
        # bare ./script.ts → ./📜️script.ts (not already 📜️)
        new2, n_sub = SCRIPT_RE.subn("./📜️script.ts", new)
        new = new2
        if n_sub:
            config_hits["./script.ts→📜️"] = config_hits.get("./script.ts→📜️", 0) + n_sub
        new2, n_sub = MCP_SCRIPT_RE.subn('["📜️script.ts"', new)
        new = new2
        if n_sub:
            config_hits['["script.ts"'] = config_hits.get('["script.ts"', 0) + n_sub
        if new != raw:
            path.write_text(new, encoding="utf-8")
            config_files.append(str(path))

(TICKET / "config-fix-report.json").write_text(
    json.dumps({"hits": config_hits, "files": config_files}, ensure_ascii=False, indent=2) + "\n",
    encoding="utf-8",
)
print(f"fixed stale paths in {len(config_files)} files")
print(json.dumps(config_hits, ensure_ascii=False, indent=2))
# endregion

# region Verify remaining VS16 basenames
left = 0
for dirpath, dirnames, filenames in os.walk(".", topdown=True):
    parts = list(Path(dirpath).parts)
    if should_skip_dir(parts):
        dirnames[:] = []
        continue
    dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
    for name in dirnames + filenames:
        if any(v in name for v in VS):
            left += 1
            if left <= 20:
                print("REMAINING", Path(dirpath) / name)
print(f"remaining VS16 basenames: {left}")
(TICKET / "remaining-vs16-basenames.txt").write_text(f"remaining={left}\n", encoding="utf-8")
# endregion
