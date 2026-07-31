#!/usr/bin/env python3
"""🔧 Fix broken include_bytes!/include_str!/#[path] after compound-emoji layout."""
from pathlib import Path
import re
import os

ROOT = Path("/Users/ueli/Documents/semio").resolve()
INCLUDE_RE = re.compile(r'(include_(?:bytes|str)!\()(")([^"]+)("\))')
PATH_ATTR_RE = re.compile(r'(#\[path\s*=\s*)(")([^"]+)("\])')
EMOJI_PREFIXES = ["🔤", "🔣", "🧊", "🌊", "📦", "🦀", "🟦", "🖼️", "🌱", "🎨", "😀", "♾️"]


def strip_doubled_emoji(name: str) -> str:
    for e in EMOJI_PREFIXES:
        while name.startswith(e + e):
            name = name[len(e) :]
        name = name.replace(e + e, e)
    return name


def try_resolve(from_dir: Path, rel: str):
    cand = (from_dir / rel)
    try:
        if cand.exists():
            return None
    except OSError:
        pass
    parts = Path(rel).parts
    for marker in ("🧰framework", "✏️s", "♻️mit-bestand", "compose", "puzzle"):
        if marker not in parts:
            continue
        idx = parts.index(marker)
        abs_target = ROOT.joinpath(*parts[idx:])
        leaf = strip_doubled_emoji(abs_target.name)
        candidates = [abs_target]
        if leaf != abs_target.name:
            candidates.append(abs_target.with_name(leaf))
        fixed_parts = [strip_doubled_emoji(p) for p in parts[idx:]]
        candidates.append(ROOT.joinpath(*fixed_parts))
        for c in candidates:
            if c.exists():
                return os.path.relpath(c, from_dir)
        base = strip_doubled_emoji(Path(rel).name)
        search_root = ROOT / marker
        if search_root.exists():
            hits = list(search_root.rglob(base))
            if hits:
                hits.sort(key=lambda p: len(str(p)))
                return os.path.relpath(hits[0], from_dir)
    fixed = "/".join(strip_doubled_emoji(p) for p in Path(rel).parts)
    if fixed != rel and (from_dir / fixed).exists():
        return fixed
    return False


def main() -> None:
    changed_files = []
    missing = []
    checked = 0

    def make_replacer(path: Path):
        def replacer(m):
            nonlocal checked
            checked += 1
            prefix, q1, rel, q2 = m.group(1), m.group(2), m.group(3), m.group(4)
            result = try_resolve(path.parent, rel)
            if result is None:
                return m.group(0)
            if result is False:
                missing.append((str(path.relative_to(ROOT)), rel))
                return m.group(0)
            return f"{prefix}{q1}{result}{q2}"

        return replacer

    for path in ROOT.rglob("*.rs"):
        if any(x in path.parts for x in (".git", "target", "node_modules", "pkg", ".repo")):
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except Exception:
            continue
        orig = text
        replacer = make_replacer(path)
        text = INCLUDE_RE.sub(replacer, text)
        text = PATH_ATTR_RE.sub(replacer, text)
        if text != orig:
            path.write_text(text, encoding="utf-8")
            changed_files.append(str(path.relative_to(ROOT)))

    print(f"checked={checked} changed_files={len(changed_files)} still_missing={len(missing)}")
    for f in changed_files:
        print("CHANGED", f)
    for p, r in missing[:40]:
        print("MISSING", p, "->", r[:120])


if __name__ == "__main__":
    main()
