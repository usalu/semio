#!/usr/bin/env python3
"""Convert all text-style (U+FE0E) emojis to emoji-style (U+FE0F) in paths + text."""
from __future__ import annotations

import json
import os
import re
import unicodedata
from pathlib import Path

REPO = Path(__file__).resolve().parents[6]
# parents may already be FE0E-styled; resolve via cwd if needed
if not (REPO / ".gitignore").exists():
    REPO = Path("/Users/ueli/Documents/semio")
os.chdir(REPO)

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
FE0E, FE0F, ZWJ = "\ufe0e", "\ufe0f", "\u200d"


def is_emoji_base(c: str) -> bool:
    o = ord(c)
    if c in (ZWJ, FE0F, FE0E, "\u20e3"):
        return False
    if 0x1F3FB <= o <= 0x1F3FF:
        return False
    if 0x1F300 <= o <= 0x1FAFF:
        return True
    if 0x2600 <= o <= 0x27BF:
        return True
    if 0x2300 <= o <= 0x23FF and unicodedata.category(c) == "So":
        return True
    if 0x2B00 <= o <= 0x2BFF and unicodedata.category(c) == "So":
        return True
    if unicodedata.category(c) == "So" and o > 0x2000:
        return True
    return False


def to_emoji_style(s: str) -> str:
    """After every emoji base, ensure exactly one FE0F; strip FE0E; keep ZWJ/skin tones."""
    out: list[str] = []
    i = 0
    n = len(s)
    while i < n:
        c = s[i]
        if is_emoji_base(c):
            out.append(c)
            i += 1
            while i < n and s[i] in (FE0E, FE0F):
                i += 1
            out.append(FE0F)
            while i < n and 0x1F3FB <= ord(s[i]) <= 0x1F3FF:
                out.append(s[i])
                i += 1
            if i < n and s[i] == ZWJ:
                out.append(ZWJ)
                i += 1
            continue
        if c in (FE0E, FE0F):
            i += 1
            continue
        out.append(c)
        i += 1
    return "".join(out)


def skip_dir(parts: list[str]) -> bool:
    for p in parts:
        if p in SKIP_DIRS:
            return True
        plain = p.replace(FE0E, "").replace(FE0F, "")
        if plain == "⚡️cache":
            return True
    return False


def ticket_dir() -> Path:
    for cand in (
        Path(".🦑️repo/🎫️tickets/🎆️26/🌕️02/☀️08/ENFORCE-EMOJI-STYLE-FE0F-EVERYWHERE"),
        Path(".🦑️repo/🎫️tickets/🎆️26/🌙️02/☀️08/ENFORCE-EMOJI-STYLE-FE0F-EVERYWHERE"),
        Path(".🦑️repo/🎫️tickets/🎆️26/🌙️02/☀️08/ENFORCE-EMOJI-STYLE-FE0F-EVERYWHERE"),
    ):
        if cand.is_dir():
            return cand
    # search
    for meta in Path(".").glob(".🦑️*repo"):
        for p in meta.rglob("ENFORCE-EMOJI-STYLE-FE0F-EVERYWHERE"):
            if p.is_dir():
                return p
    return Path(".🦑️repo/🎫️tickets/🎆️26/🌙️02/☀️08/ENFORCE-EMOJI-STYLE-FE0F-EVERYWHERE")


# region Filesystem
renamed: list[str] = []
for dirpath, dirnames, filenames in os.walk(".", topdown=False):
    parts = list(Path(dirpath).parts)
    if skip_dir(parts):
        continue
    for name in filenames + dirnames:
        new_name = to_emoji_style(name)
        if new_name == name:
            continue
        old_path = Path(dirpath) / name
        new_path = Path(dirpath) / new_name
        if not old_path.exists():
            continue
        if new_path.exists() and old_path.resolve() != new_path.resolve():
            raise SystemExit(f"collision: {old_path} -> {new_path}")
        old_path.rename(new_path)
        renamed.append(f"{old_path} -> {new_path}")
print(f"renamed {len(renamed)} filesystem entries")
# endregion

# region Text (fast need check)
NEED = re.compile(r"\ufe0e|[\U0001F300-\U0001FAFF\u2600-\u27BF\u2300-\u23FF\u2B00-\u2BFF](?!\ufe0f)")
text_changed: list[str] = []
for dirpath, dirnames, filenames in os.walk(".", topdown=True):
    parts = list(Path(dirpath).parts)
    if skip_dir(parts):
        dirnames[:] = []
        continue
    dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
    for name in filenames:
        if name in SKIP_FILES:
            continue
        path = Path(dirpath) / name
        if not path.is_file():
            continue
        try:
            raw = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        if not NEED.search(raw) and FE0E not in raw:
            # still run if any FE0E
            if FE0E not in raw:
                continue
        new = to_emoji_style(raw)
        if new == raw:
            continue
        try:
            path.write_text(new, encoding="utf-8")
            text_changed.append(str(path))
        except OSError as e:
            print("write fail", path, e)
print(f"text-styled {len(text_changed)} files")
# endregion

# region Remove non-emoji-style script symlinks
for link in (Path("script.ts"), Path("📜️script.ts"), Path("📜️script.ts")):
    if link.is_symlink():
        target = link.readlink()
        link.unlink()
        print("removed symlink", link, "->", target)
# endregion

# region Verify + report
bad_fe0e_names: list[str] = []
bad_miss_fe0f: list[str] = []
for dirpath, dirnames, filenames in os.walk(".", topdown=True):
    parts = list(Path(dirpath).parts)
    if skip_dir(parts):
        dirnames[:] = []
        continue
    dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
    for name in dirnames + filenames:
        if FE0E in name:
            bad_fe0e_names.append(str(Path(dirpath) / name))
        chars = list(name)
        i = 0
        while i < len(chars):
            if is_emoji_base(chars[i]):
                if i + 1 >= len(chars) or chars[i + 1] != FE0F:
                    bad_miss_fe0f.append(str(Path(dirpath) / name))
                    break
                i += 2
                continue
            i += 1

ticket = ticket_dir()
ticket.mkdir(parents=True, exist_ok=True)
(ticket / "emoji-style-renamed.txt").write_text(
    "\n".join(renamed) + ("\n" if renamed else ""), encoding="utf-8"
)
(ticket / "emoji-style-text-files.txt").write_text(
    "\n".join(text_changed) + ("\n" if text_changed else ""), encoding="utf-8"
)

gi = Path(".gitignore").read_text(encoding="utf-8") if Path(".gitignore").exists() else ""
report = {
    "renamed": len(renamed),
    "text_changed": len(text_changed),
    "bad_fe0e_basenames": len(bad_fe0e_names),
    "bad_missing_fe0f_basenames": len(bad_miss_fe0f),
    "bad_fe0e_samples": bad_fe0e_names[:10],
    "bad_miss_samples": bad_miss_fe0f[:10],
    "gitignore_fe0e": gi.count(FE0E),
    "gitignore_fe0f": gi.count(FE0F),
    "script": Path("📜️script.ts").exists() or any(Path(".").glob("📜️*script.ts")),
    "meta": any(Path(".").glob(".🦑️*repo")),
}
(ticket / "emoji-style-report.json").write_text(
    json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
)
print(json.dumps(report, ensure_ascii=False, indent=2))
if bad_fe0e_names or bad_miss_fe0f:
    raise SystemExit("verification failed")
# endregion
