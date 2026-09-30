#!/usr/bin/env python3
"""🪟 Point live references at the shortened fixture directories."""

import os
from collections import defaultdict

src = open(os.path.join(os.path.dirname(__file__), "📜️fit.py"), encoding="utf-8").read().split("def main")[0]
ns: dict = {}
exec(src, ns)
u16, elastic, shorten, excess = ns["u16"], ns["elastic"], ns["shorten"], ns["excess"]
FLOOR = ns["FLOOR"]


def plan() -> dict[str, str]:
    paths = ns["tracked"]()
    remove = {path for path in paths if "🎫️tickets" in path and excess(path) > 0}
    product = [path for path in paths if path not in remove]
    need: dict[str, int] = {}
    for path in product:
        over = excess(path)
        if over <= 0:
            continue
        segs = path.split("/")
        donors = [(u16(seg) - FLOOR, index) for index, seg in enumerate(segs[:-1]) if elastic(seg)]
        donors.sort(reverse=True)
        left = over
        assigned: list[tuple[str, int]] = []
        for slack, index in donors:
            if slack <= 0 or left <= 0:
                continue
            give = min(left, slack)
            assigned.append(("/".join(segs[: index + 1]), give))
            left -= give
        if left > 0:
            continue
        for dirpath, give in assigned:
            need[dirpath] = max(need.get(dirpath, 0), give)
    renames: dict[str, str] = {}
    for dirpath, cut in need.items():
        parent, base = dirpath.rsplit("/", 1) if "/" in dirpath else ("", dirpath)
        new = shorten(base, u16(base) - cut)
        renames[dirpath] = f"{parent}/{new}" if parent else new
    return renames


def final_dir(old: str, renames: dict[str, str]) -> str:
    parts = old.split("/")
    acc = ""
    out: list[str] = []
    for seg in parts:
        acc = seg if not acc else acc + "/" + seg
        mapped = renames.get(acc)
        out.append(mapped.rsplit("/", 1)[-1] if mapped else seg)
    return "/".join(out)


def main() -> None:
    renames = plan()
    by_len: dict[int, dict[tuple[str, ...], set[tuple[str, ...]]]] = {length: defaultdict(set) for length in range(2, 13)}
    full: list[tuple[str, str]] = []
    for old in renames:
        new = final_dir(old, renames)
        full.append((old, new))
        old_parts, new_parts = old.split("/"), new.split("/")
        for length in by_len:
            if len(old_parts) >= length:
                by_len[length][tuple(old_parts[-length:])].add(tuple(new_parts[-length:]))
    replacements: list[tuple[str, str]] = sorted(full, key=lambda item: len(item[0]), reverse=True)
    for length in range(12, 1, -1):
        for old_parts, news in by_len[length].items():
            if len(news) != 1:
                continue
            old_s, new_s = "/".join(old_parts), "/".join(next(iter(news)))
            if old_s != new_s:
                replacements.append((old_s, new_s))
    replacements.sort(key=lambda item: len(item[0]), reverse=True)
    print(f"replacements {len(replacements)}")
    roots = ["✏️s", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"]
    files: list[str] = []
    for root in roots:
        if os.path.isfile(root):
            files.append(root)
            continue
        for dirpath, _, names in os.walk(root):
            if "🎫️tickets" in dirpath:
                continue
            for name in names:
                files.append(os.path.join(dirpath, name))
    rewritten = 0
    for path in files:
        try:
            blob = open(path, "rb").read()
        except OSError:
            continue
        if b"\0" in blob[:4096]:
            continue
        try:
            text = blob.decode("utf-8")
        except UnicodeDecodeError:
            continue
        updated = text
        for old, new in replacements:
            if old in updated:
                updated = updated.replace(old, new)
        if updated != text:
            with open(path, "w", encoding="utf-8", newline="") as handle:
                handle.write(updated)
            rewritten += 1
    print(f"rewrote {rewritten}")


if __name__ == "__main__":
    main()
