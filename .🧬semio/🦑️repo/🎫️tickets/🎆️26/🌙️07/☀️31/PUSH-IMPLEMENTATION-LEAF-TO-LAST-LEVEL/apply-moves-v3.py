#!/usr/bin/env python3
"""B2 (round 3): physically apply moves-v3.json, deepest-first (by OLD path
segment count) so a directory's own path is still valid when its turn comes and
children are always extracted before their parent's own remaining content moves."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
with open(os.path.join(HERE, "moves-v3.json"), encoding="utf-8") as f:
    MOVES = json.load(f)


def main():
    moved = 0
    errors = []
    for r in MOVES:
        old, new = r["old"], r["new"]
        if not os.path.exists(old):
            errors.append(("MISSING_OLD", old, new))
            continue
        if os.path.exists(new):
            errors.append(("TARGET_EXISTS", old, new))
            continue
        os.makedirs(os.path.dirname(new) or ".", exist_ok=True)
        os.rename(old, new)
        moved += 1
    print(f"moved: {moved}")
    print(f"errors: {len(errors)}")
    for e in errors[:50]:
        print(" ", e)


if __name__ == "__main__":
    main()
