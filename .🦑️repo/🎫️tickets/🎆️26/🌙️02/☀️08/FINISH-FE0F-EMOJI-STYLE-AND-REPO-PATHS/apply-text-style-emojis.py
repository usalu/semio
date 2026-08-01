#!/usr/bin/env python3
"""Force U+FE0E text presentation on all emojis in paths + text (not U+FE0F emoji style)."""
from __future__ import annotations

import json
import os
import unicodedata
from pathlib import Path

REPO = Path(__file__).resolve().parents[6]
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
# cache dir name changes after rename; skip both forms mid-run via prefix check
SKIP_DIR_PREFIXES = ("⚡️cache", "⚡️cache")
SKIP_FILES = frozenset({"AGENTS.md"})
FE0E = "\ufe0e"
FE0F = "\ufe0f"
ZWJ = "\u200d"


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


def is_skin_tone(c: str) -> bool:
    return 0x1F3FB <= ord(c) <= 0x1F3FF


def to_text_style(s: str) -> str:
    """After every emoji base, ensure exactly one FE0E; strip FE0F; keep ZWJ/skin tones."""
    chars = list(s)
    out: list[str] = []
    i = 0
    n = len(chars)
    while i < n:
        c = chars[i]
        if is_emoji_base(c):
            out.append(c)
            i += 1
            while i < n and chars[i] in (FE0E, FE0F):
                i += 1
            out.append(FE0E)
            while i < n and is_skin_tone(chars[i]):
                out.append(chars[i])
                i += 1
            if i < n and chars[i] == ZWJ:
                out.append(ZWJ)
                i += 1
            continue
        # stray VS not after a base — drop FE0F, keep lone FE0E only if intentional? drop both strays
        if c in (FE0E, FE0F):
            i += 1
            continue
        out.append(c)
        i += 1
    return "".join(out)


def should_skip_dir(parts: list[str]) -> bool:
    for p in parts:
        if p in SKIP_DIRS:
            return True
        if p.startswith(SKIP_DIR_PREFIXES) or p in SKIP_DIR_PREFIXES:
            return True
        # generated cache under repo meta
        if p.endswith("cache") and p.startswith(("⚡️", ".")):
            # only skip ⚡️cache forms, not random *cache*
            if p.replace(FE0E, "").replace(FE0F, "") in ("⚡️cache",):
                return True
    return False


def ticket_dir() -> Path:
    # resolve after possible meta rename
    for meta in (Path(".🦑️repo"), Path(".🦑️repo")):
        base = meta / "🎫️tickets" if (meta / "🎫️tickets").is_dir() else meta / "🎫️tickets"
        cand = base / "26" / "02" / "08" / "ENFORCE-TEXT-STYLE-EMOJIS-END-TO-END"
        if cand.is_dir():
            return cand
    return Path(".🦑️repo/🎫️tickets/26/02/08/ENFORCE-TEXT-STYLE-EMOJIS-END-TO-END")


# region Filesystem renames
renamed: list[str] = []
for dirpath, dirnames, filenames in os.walk(".", topdown=False):
    parts = list(Path(dirpath).parts)
    if should_skip_dir(parts):
        continue
    for name in filenames + dirnames:
        new_name = to_text_style(name)
        if new_name == name:
            continue
        old_path = Path(dirpath) / name
        new_path = Path(dirpath) / new_name
        if new_path.exists() and old_path.resolve() != new_path.resolve():
            raise SystemExit(f"collision: {old_path} -> {new_path}")
        if not old_path.exists():
            continue
        old_path.rename(new_path)
        renamed.append(f"{old_path} -> {new_path}")
print(f"renamed {len(renamed)} filesystem entries")
# endregion

# region Text transforms
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
        new = to_text_style(raw)
        if new != raw:
            path.write_text(new, encoding="utf-8")
            text_changed.append(str(path))
print(f"text-styled {len(text_changed)} files")
# endregion

# region Reports + verify
ticket = ticket_dir()
ticket.mkdir(parents=True, exist_ok=True)
(ticket / "text-style-renamed-paths.txt").write_text(
    "\n".join(renamed) + ("\n" if renamed else ""), encoding="utf-8"
)
(ticket / "text-style-text-files.txt").write_text(
    "\n".join(text_changed) + ("\n" if text_changed else ""), encoding="utf-8"
)

# verify no FE0F in basenames; every emoji base in basenames followed by FE0E
bad_fe0f = []
bad_missing_fe0e = []
for dirpath, dirnames, filenames in os.walk(".", topdown=True):
    parts = list(Path(dirpath).parts)
    if should_skip_dir(parts):
        dirnames[:] = []
        continue
    dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
    for name in dirnames + filenames:
        if FE0F in name:
            bad_fe0f.append(str(Path(dirpath) / name))
        chars = list(name)
        i = 0
        while i < len(chars):
            if is_emoji_base(chars[i]):
                if i + 1 >= len(chars) or chars[i + 1] != FE0E:
                    bad_missing_fe0e.append(str(Path(dirpath) / name))
                    break
                i += 2
                continue
            i += 1

gi = Path(".gitignore")
gi_text = gi.read_text(encoding="utf-8") if gi.exists() else ""
report = {
    "renamed": len(renamed),
    "text_changed": len(text_changed),
    "bad_fe0f_basenames": len(bad_fe0f),
    "bad_missing_fe0e_basenames": len(bad_missing_fe0e),
    "gitignore_fe0e": gi_text.count(FE0E),
    "gitignore_fe0f": gi_text.count(FE0F),
    "gitignore_sample": [ln for ln in gi_text.splitlines() if "repo" in ln or "framework" in ln or "mit-bestand" in ln][:20],
    "bad_fe0f_samples": bad_fe0f[:20],
    "bad_missing_fe0e_samples": bad_missing_fe0e[:20],
    "checks": {
        "script": Path(to_text_style("📜️script.ts")).exists() or Path("📜️script.ts").exists(),
        "meta": Path(".🦑️repo").is_dir() or Path(".🦑️repo").is_dir(),
        "gitignore": gi.exists(),
    },
}
(ticket / "text-style-report.json").write_text(
    json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
)
print(json.dumps(report, ensure_ascii=False, indent=2))
if bad_fe0f or bad_missing_fe0e:
    raise SystemExit("verification failed")
# endregion
