#!/usr/bin/env python3
"""🪟 Locate the on-disk directory for two shortened fixture leaves."""

import os

NEEDLES = [
    "set-layer-visibility-applied",
    "set-layer",
    "turns-multiple-resisting-systems",
]


def main() -> None:
    src = open(os.path.join(os.path.dirname(__file__), "📜️fit.py"), encoding="utf-8").read().split("def main")[0]
    ns: dict = {}
    exec(src, ns)
    u16, elastic, shorten, excess = ns["u16"], ns["elastic"], ns["shorten"], ns["excess"]
    floor = ns["FLOOR"]
    paths = ns["tracked"]()
    remove = {path for path in paths if "🎫️tickets" in path and excess(path) > 0}
    product = [path for path in paths if path not in remove]
    need: dict[str, int] = {}
    for path in product:
        over = excess(path)
        if over <= 0:
            continue
        segs = path.split("/")
        donors = [(u16(seg) - floor, index) for index, seg in enumerate(segs[:-1]) if elastic(seg)]
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

    def final_dir(old: str) -> str:
        parts = old.split("/")
        acc = ""
        out: list[str] = []
        for seg in parts:
            acc = seg if not acc else acc + "/" + seg
            mapped = renames.get(acc)
            out.append(mapped.rsplit("/", 1)[-1] if mapped else seg)
        return "/".join(out)

    for needle in NEEDLES:
        print(f"\n== {needle}")
        hits = [old for old in renames if needle in old.rsplit("/", 1)[-1]]
        print("rename hits", len(hits))
        for old in hits[:8]:
            new = final_dir(old)
            print(" old", old[-120:])
            print(" new", new[-120:], "exists", os.path.isdir(new))


if __name__ == "__main__":
    main()
