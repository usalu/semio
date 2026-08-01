#!/usr/bin/env python3
"""Merge legacy meta roots into .🦑️repo; ensure FE0F everywhere; fix emojiText to keep FE0F."""
from __future__ import annotations

import json
import os
import re
import shutil
import unicodedata
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
os.chdir(REPO)

FE0E, FE0F, ZWJ = "\ufe0e", "\ufe0f", "\u200d"
CANON_META = Path(".🦑️repo")  # squid + FE0F + repo

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
    if unicodedata.category(c) == "So" and o > 0x2000:
        return True
    return False


def to_emoji_style(s: str) -> str:
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


def emoji_date_path(year: int, month: int, day: int) -> Path:
    # EmojiYear/Month/Day with FE0F — year padded to 2 (26 not 2026) matching FormatYearDir
    yy = year % 100
    return Path(f"🎆️{FE0F}{yy:02d}") / f"🌙️{FE0F}{month:02d}" / f"☀️{FE0F}{day:02d}"


def parse_numeric_date_parts(parts: list[str]) -> tuple[int, int, int] | None:
    """From .../26/08/01/... or .../2026/08/01/... return y,m,d."""
    for i in range(len(parts) - 2):
        a, b, c = parts[i], parts[i + 1], parts[i + 2]
        if a.isdigit() and b.isdigit() and c.isdigit():
            y, m, d = int(a), int(b), int(c)
            if 1 <= m <= 12 and 1 <= d <= 31 and (y < 100 or 2000 <= y <= 2100):
                return y, m, d
    return None


def merge_tree(src_root: Path, label: str) -> list[str]:
    """Copy files from legacy meta into CANON_META with FE0F path segments."""
    moved: list[str] = []
    if not src_root.exists():
        return moved
    tickets_dst = CANON_META / f"🎫️{FE0F}tickets"
    for dirpath, dirnames, filenames in os.walk(src_root):
        rel = os.path.relpath(dirpath, src_root)
        parts = [] if rel == "." else rel.split(os.sep)
        for name in filenames:
            src = Path(dirpath) / name
            # Build destination
            full_parts = parts + [name]
            # Detect ticket layout: tickets?/YY/MM/DD/SLUG/file or 🎫️/YY/MM/DD/SLUG/file
            date = parse_numeric_date_parts(full_parts)
            if date and any("ticket" in p.lower() or p.startswith("🎫️") or p.startswith("%F0%9F%8E%AB") for p in full_parts):
                y, m, d = date
                # find slug = segment after day
                idx = None
                for i, p in enumerate(full_parts):
                    if p.isdigit() and i + 2 < len(full_parts) and full_parts[i + 1].isdigit() and full_parts[i + 2].isdigit():
                        # year at i if matches
                        if int(full_parts[i]) == y or int(full_parts[i]) == y % 100 or int(full_parts[i]) == 2000 + y % 100:
                            if int(full_parts[i + 1]) == m and int(full_parts[i + 2]) == d:
                                idx = i
                                break
                if idx is not None and idx + 3 < len(full_parts):
                    slug = full_parts[idx + 3]
                    rest = full_parts[idx + 4 :]  # files under slug
                    dst_dir = tickets_dst / emoji_date_path(y, m, d) / slug
                    for seg in rest[:-1] if rest else []:
                        dst_dir = dst_dir / to_emoji_style(seg)
                    dst_dir.mkdir(parents=True, exist_ok=True)
                    dst = dst_dir / to_emoji_style(name)
                    if not dst.exists():
                        shutil.copy2(src, dst)
                        moved.append(f"{src} -> {dst}")
                    else:
                        moved.append(f"SKIP exists {dst} (from {src})")
                    continue
            # fallback: place under ⚡️cache/legacy-import/<label>/...
            legacy = CANON_META / f"⚡️{FE0F}cache" / "legacy-import" / label
            dst_dir = legacy.joinpath(*[to_emoji_style(p) for p in parts]) if parts else legacy
            dst_dir.mkdir(parents=True, exist_ok=True)
            dst = dst_dir / to_emoji_style(name)
            if not dst.exists():
                shutil.copy2(src, dst)
                moved.append(f"{src} -> {dst}")
            else:
                moved.append(f"SKIP exists {dst}")
    return moved


# region Merge legacy metas
CANON_META.mkdir(exist_ok=True)
all_moved: list[str] = []
for legacy, label in [
    (Path(".repo"), "dot-repo"),
    (Path(".🦑️repo"), "bare-squid-repo"),
    (Path(".%F0%9F%A6%91repo"), "urlencoded-squid-repo"),
]:
    m = merge_tree(legacy, label)
    all_moved.extend(m)
    print(f"merged {len(m)} from {legacy}")

# Remove legacy roots
for legacy in [Path(".repo"), Path(".🦑️repo"), Path(".%F0%9F%A6%91repo")]:
    if legacy.exists():
        shutil.rmtree(legacy)
        print("removed", legacy)
# endregion

# region Rename any remaining bare-emoji / FE0E basenames under repo
renamed = []
for dirpath, dirnames, filenames in os.walk(".", topdown=False):
    parts = list(Path(dirpath).parts)
    if any(p in SKIP_DIRS for p in parts):
        continue
    if any(p.replace(FE0E, "").replace(FE0F, "") == "⚡️cache" for p in parts):
        continue
    for name in filenames + dirnames:
        new_name = to_emoji_style(name)
        if new_name == name:
            continue
        old = Path(dirpath) / name
        new = Path(dirpath) / new_name
        if not old.exists():
            continue
        if new.exists() and old.resolve() != new.resolve():
            print("collision", old, new)
            continue
        old.rename(new)
        renamed.append(f"{old} -> {new}")
print(f"renamed {len(renamed)} bare/FE0E basenames")
# endregion

# region Fix emojiText in Go to preserve/force FE0F
go = next(x for x in Path("🧰️framework").rglob("main.go") if "⌨️cli" in str(x) and "client" in str(x))
go_text = go.read_text(encoding="utf-8")
old_fn = '''func emojiText(emoji string) string {
	stripped := strings.ReplaceAll(emoji, "\\uFE0E", "")
	textDefaultEmojis := []string{
		"\\U0001F3D7", "\\u2328", "\\U0001F5B1", "\\U0001F5C3",
		"\\u2699", "\\u2696", "\\U0001F3F7", "\\U0001F6E0",
		"\\u2702", "\\U0001F6E1", "\\U0001F5D1",
		"\\u2600", "\\u23F1", "\\u270F", "\\U0001F46E",
	}
	base := strings.ReplaceAll(stripped, "\\uFE0F", "")
	for _, td := range textDefaultEmojis {
		if strings.Contains(base, td) {
			return strings.ReplaceAll(base, td, td+"\\uFE0F")
		}
	}
	return base
}'''
# Use actual file content (unicode escapes as real chars in source may be \uFE0E strings)
old_actual = """func emojiText(emoji string) string {
	stripped := strings.ReplaceAll(emoji, \"\\uFE0E\", \"\")
	textDefaultEmojis := []string{
		\"\\U0001F3D7\", \"\\u2328\", \"\\U0001F5B1\", \"\\U0001F5C3\",
		\"\\u2699\", \"\\u2696\", \"\\U0001F3F7\", \"\\U0001F6E0\",
		\"\\u2702\", \"\\U0001F6E1\", \"\\U0001F5D1\",
		\"\\u2600\", \"\\u23F1\", \"\\u270F\", \"\\U0001F46E\",
	}
	base := strings.ReplaceAll(stripped, \"\\uFE0F\", \"\")
	for _, td := range textDefaultEmojis {
		if strings.Contains(base, td) {
			return strings.ReplaceAll(base, td, td+\"\\uFE0F\")
		}
	}
	return base
}"""

new_fn = """func emojiText(emoji string) string {
	// Emoji style only: never text presentation (U+FE0E). Always keep/ensure U+FE0F.
	s := strings.ReplaceAll(emoji, \"\\uFE0E\", \"\\uFE0F\")
	var b strings.Builder
	runes := []rune(s)
	for i := 0; i < len(runes); i++ {
		r := runes[i]
		b.WriteRune(r)
		if r == '\\uFE0E' || r == '\\uFE0F' || r == '\\u200d' {
			continue
		}
		// skin tones
		if r >= 0x1F3FB && r <= 0x1F3FF {
			continue
		}
		isEmoji := (r >= 0x1F300 && r <= 0x1FAFF) || (r >= 0x2600 && r <= 0x27BF) || (r >= 0x2300 && r <= 0x23FF) || (r >= 0x2B00 && r <= 0x2BFF)
		if !isEmoji {
			continue
		}
		// skip if next is already FE0F
		if i+1 < len(runes) && runes[i+1] == '\\uFE0F' {
			continue
		}
		if i+1 < len(runes) && runes[i+1] == '\\uFE0E' {
			continue
		}
		b.WriteRune('\\uFE0F')
	}
	return b.String()
}"""

if old_actual in go_text:
    go_text = go_text.replace(old_actual, new_fn)
    go.write_text(go_text, encoding="utf-8")
    print("updated emojiText in", go)
else:
    # try locate and replace by regex
    m = re.search(r"func emojiText\(emoji string\) string \{[\s\S]*?\n\}", go_text)
    if not m:
        raise SystemExit("emojiText not found")
    go_text = go_text[: m.start()] + new_fn + go_text[m.end() :]
    go.write_text(go_text, encoding="utf-8")
    print("updated emojiText via regex in", go)
# endregion

# region Text FE0E -> FE0F (and bare emoji bases get FE0F) in non-AGENTS text
NEED = re.compile(r"\ufe0e|[\U0001F300-\U0001FAFF\u2600-\u27BF](?!\ufe0f)")
text_changed = []
for dirpath, dirnames, filenames in os.walk(".", topdown=True):
    parts = list(Path(dirpath).parts)
    if any(p in SKIP_DIRS for p in parts):
        dirnames[:] = []
        continue
    if any(p.replace(FE0E, "").replace(FE0F, "") == "⚡️cache" for p in parts):
        dirnames[:] = []
        continue
    dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
    for name in filenames:
        if name == "AGENTS.md":
            continue
        path = Path(dirpath) / name
        try:
            raw = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        if not NEED.search(raw) and FE0E not in raw:
            continue
        new = to_emoji_style(raw)
        if new == raw:
            continue
        path.write_text(new, encoding="utf-8")
        text_changed.append(str(path))
print(f"text FE0F-normalized {len(text_changed)} files")
# endregion

# region Ensure gitignore keeps ignoring legacy roots
gi_path = Path(".gitignore")
gi = gi_path.read_text(encoding="utf-8")
legacy_lines = [
    ".repo",
    ".🦑️repo",
    ".%F0%9F%A6%91repo",
]
changed_gi = False
for line in legacy_lines:
    # ignore bare squid without FE0F
    if line == ".🦑️repo":
        entry = ".🦑️repo"
    else:
        entry = line
    if entry not in gi.splitlines():
        # insert after .repo line if present
        if ".repo\n" in gi or gi.endswith(".repo"):
            gi = gi.replace(".repo\n", ".repo\n.🦑️repo\n.%F0%9F%A6%91repo\n", 1)
            # may duplicate - fix below
        else:
            gi += f"\n{entry}\n"
        changed_gi = True
# dedupe consecutive
lines = gi.splitlines()
out_lines = []
seen_block = set()
for ln in lines:
    if ln in {".repo", ".🦑️repo", ".%F0%9F%A6%91repo", ".repo-cache"}:
        if ln in seen_block and ln != ".repo-cache":
            changed_gi = True
            continue
        seen_block.add(ln)
    out_lines.append(ln)
# ensure all three legacy ignores present near REPO section
needed = [".repo", ".🦑️repo", ".%F0%9F%A6%91repo"]
have = set(out_lines)
for n in needed:
    if n not in have:
        # after .repo or at REPO section
        try:
            idx = out_lines.index(".repo")
            out_lines.insert(idx + 1, n)
        except ValueError:
            out_lines.append(n)
        changed_gi = True
if changed_gi:
    gi_path.write_text("\n".join(out_lines) + "\n", encoding="utf-8")
    print("updated .gitignore legacy ignores")
# endregion

# region Report
ticket = next(Path(".🦑️repo").rglob("FINISH-FE0F-EMOJI-STYLE-AND-REPO-PATHS"), None)
if ticket is None:
    ticket = next(Path(".🦑️repo").rglob("ENFORCE-EMOJI-STYLE-FE0F-EVERYWHERE"), Path("."))
report = {
    "moved": len(all_moved),
    "renamed": len(renamed),
    "text_changed": len(text_changed),
    "legacy_metas_present": {
        ".repo": Path(".repo").exists(),
        ".🦑️repo": Path(".🦑️repo").exists(),
        ".urlencoded": Path(".%F0%9F%A6%91repo").exists(),
        ".fe0f": CANON_META.exists(),
    },
    "moved_samples": all_moved[:30],
}
(ticket / "finish-fe0f-report.json").write_text(
    json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
)
(ticket / "finish-fe0f-moved.txt").write_text("\n".join(all_moved) + "\n", encoding="utf-8")
print(json.dumps(report["legacy_metas_present"], ensure_ascii=False, indent=2))
print("report written", ticket)
# endregion
