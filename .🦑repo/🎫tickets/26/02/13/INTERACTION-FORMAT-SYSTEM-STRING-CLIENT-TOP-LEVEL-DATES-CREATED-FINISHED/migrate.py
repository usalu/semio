#!/usr/bin/env python3
"""Migrate interactions: system string, client top-level, dates.created/finished."""

import json
import os
import sys

ROOT = os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", "..")


def migrate_interaction(ia):
    out = dict(ia)
    dates = ia.get("dates") or {}
    if "started" in dates:
        out["dates"] = dict(dates)
        out["dates"]["created"] = out["dates"].pop("started")
    system = ia.get("system")
    if isinstance(system, dict):
        out["system"] = system.get("version") or system.get("client") or "linux"
        if system.get("client"):
            out["client"] = system["client"]
    elif "client" not in out:
        out["client"] = ia.get("client", "")
    return out


def migrate_file(path):
    with open(path, "r", encoding="utf-8") as f:
        data = json.load(f)
    changed = False
    for key in ("interactions",):
        arr = data.get(key)
        if not isinstance(arr, list):
            continue
        new_arr = []
        for item in arr:
            new_item = migrate_interaction(item)
            if new_item != item:
                changed = True
            new_arr.append(new_item)
        data[key] = new_arr
    if changed:
        with open(path, "w", encoding="utf-8") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        return True
    return False


def main():
    count = 0
    for base in ["tickets", "goals"]:
        base_path = os.path.join(ROOT, ".repo", base)
        if not os.path.isdir(base_path):
            continue
        for root, _, files in os.walk(base_path):
            for name in files:
                if name in ("ticket.json", "goal.json"):
                    path = os.path.join(root, name)
                    if migrate_file(path):
                        count += 1
                        print(path)
    print(f"Migrated {count} files", file=sys.stderr)


if __name__ == "__main__":
    main()
