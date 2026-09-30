#!/usr/bin/env python3
"""🪟 Fit every tracked path under a 20-character Windows profile Documents checkout."""

import os
import subprocess

PREFIX = "C:\\Users\\abcdefghijklmnopqrst\\Documents\\semio\\"
FILE_BUDGET = 259 - len(PREFIX.encode("utf-16-le")) // 2
DIR_BUDGET = 247 - len(PREFIX.encode("utf-16-le")) // 2
FLOOR = 8
STOP = {"the", "a", "an", "to", "of", "and", "its", "from", "that", "with", "for", "into", "onto", "by", "in", "on", "at"}


def u16(value: str) -> int:
    return len(value.encode("utf-16-le")) // 2


def tracked() -> list[str]:
    raw = subprocess.check_output(["git", "-c", "core.quotepath=off", "ls-files", "-z"])
    return [path for path in raw.decode().split("\0") if path]


def excess(path: str) -> int:
    over = max(0, u16(path) - FILE_BUDGET)
    acc = ""
    for seg in path.split("/")[:-1]:
        acc = seg if not acc else acc + "/" + seg
        over = max(over, u16(acc) - DIR_BUDGET)
    return over


def elastic(seg: str) -> bool:
    ascii_tail = "".join(ch for ch in seg if ord(ch) < 128)
    return len(ascii_tail) >= 12 and u16(seg) >= 16


def split_emoji(seg: str) -> tuple[str, str]:
    index = 0
    while index < len(seg) and ord(seg[index]) > 127:
        index += 1
    return seg[:index], seg[index:]


def tokens(slug: str) -> tuple[list[str], str]:
    for separator in ("-", "_", "."):
        if separator in slug:
            return [word for word in slug.split(separator) if word], separator
    return [slug], "-"


def shorten(seg: str, target: int) -> str:
    if u16(seg) <= target:
        return seg
    emoji, slug = split_emoji(seg)
    words, separator = tokens(slug)
    kept = [word for word in words if word not in STOP] or words
    while len(kept) > 1 and u16(emoji + separator.join(kept)) > target:
        kept.pop()
    name = emoji + separator.join(kept)
    if u16(name) > target:
        budget = max(1, target - u16(emoji))
        name = emoji + (separator.join(kept)[:budget].rstrip(separator))
    return name


def main() -> None:
    paths = tracked()
    print(f"prefix {u16(PREFIX)} file<={FILE_BUDGET} dir<={DIR_BUDGET}")
    remove = [path for path in paths if "🎫️tickets" in path and excess(path) > 0]
    for path in remove:
        if os.path.isfile(path):
            os.remove(path)
    print(f"removed {len(remove)} ticket files")
    product = [path for path in paths if path not in set(remove)]

    need: dict[str, int] = {}
    unfixable: list[str] = []
    for path in product:
        over = excess(path)
        if over <= 0:
            continue
        segs = path.split("/")
        donors = [(u16(seg), index) for index, seg in enumerate(segs) if elastic(seg) and index < len(segs) - 1]
        donors.sort(reverse=True)
        left = over
        for length, index in donors:
            give = min(left, length - FLOOR)
            if give <= 0:
                continue
            dirpath = "/".join(segs[: index + 1])
            need[dirpath] = max(need.get(dirpath, 0), give if need.get(dirpath, 0) == 0 else max(need[dirpath], give))
            # The cut recorded is the max give this dir must make. Recompute left from the worst assignment below.
            left -= give
            if left <= 0:
                break
        if left > 0:
            unfixable.append(path)
    # Recompute need as the max excess each dir must absorb for every path, not the greedy slice.
    need = {}
    unfixable = []
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
            unfixable.append(path)
            continue
        for dirpath, give in assigned:
            need[dirpath] = max(need.get(dirpath, 0), give)
    print(f"shorten {len(need)} directories, unfixable {len(unfixable)}")
    for path in unfixable[:12]:
        print("  UNFIX", excess(path), path[-140:])
    if unfixable:
        raise SystemExit(1)

    renames: dict[str, str] = {}
    for dirpath, cut in need.items():
        parent, base = dirpath.rsplit("/", 1) if "/" in dirpath else ("", dirpath)
        target = u16(base) - cut
        new = shorten(base, target)
        sibling = os.path.join(parent, new) if parent else new
        if new == base:
            raise SystemExit(f"no shorten {dirpath}")
        if os.path.exists(sibling):
            raise SystemExit(f"collision {sibling}")
        renames[dirpath] = sibling
        print(f"  -{cut} {base} -> {new}")

    def final(path: str) -> str:
        parts = path.split("/")
        acc = ""
        out: list[str] = []
        for seg in parts:
            acc = seg if not acc else acc + "/" + seg
            mapped = renames.get(acc)
            out.append(mapped.rsplit("/", 1)[-1] if mapped else seg)
        return "/".join(out)

    still = [final(path) for path in product if excess(final(path)) > 0]
    print(f"simulated remaining {len(still)}")
    if still:
        for path in still[:8]:
            print("  STILL", excess(path), path[-140:])
        raise SystemExit(1)

    replacements = sorted(((old, new) for old, new in renames.items()), key=lambda item: len(item[0]), reverse=True)
    basenames: dict[str, int] = {}
    for path in product:
        for seg in path.split("/")[:-1]:
            basenames[seg] = basenames.get(seg, 0) + 1
    unique = [(old.rsplit("/", 1)[-1], new.rsplit("/", 1)[-1]) for old, new in replacements if basenames.get(old.rsplit("/", 1)[-1], 0) == 1]

    rewritten = 0
    for path in product:
        if not os.path.isfile(path) or "🎫️tickets" in path:
            continue
        with open(path, "rb") as handle:
            blob = handle.read()
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
        for old, new in unique:
            if old in updated:
                updated = updated.replace(old, new)
        if updated != text:
            with open(path, "w", encoding="utf-8", newline="") as handle:
                handle.write(updated)
            rewritten += 1
    print(f"rewrote {rewritten} files")

    for old, new in sorted(renames.items(), key=lambda item: item[0].count("/"), reverse=True):
        if os.path.isdir(old):
            os.rename(old, new)
    print(f"renamed {len(renames)} directories")


if __name__ == "__main__":
    main()
