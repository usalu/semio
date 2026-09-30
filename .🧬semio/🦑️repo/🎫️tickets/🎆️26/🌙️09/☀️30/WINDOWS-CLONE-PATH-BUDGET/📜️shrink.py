#!/usr/bin/env python3
"""🪟 Drop tracked paths that Windows cannot create under the declared clone root."""

import os
import subprocess

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../../../../../.."))
FILE_BUDGET = 239
DIR_BUDGET = 227

RENAMES = {
    "💦️drops-the-provided-humidification-to-1-point-25-kg-per-hour": "💦️drops-the-provided-humidification-1-point-25-kg-per-hour",
    "🚪️raises-the-infiltration-allowance-to-52-point-5-m3-per-hour": "🚪️raises-the-infiltration-allowance-52-point-5-m3-per-hour",
    "☁️raises-the-required-humidification-to-3-point-5-kg-per-hour": "☁️raises-the-required-humidification-3-point-5-kg-per-hour",
    "✅️replace-query-result-applied": "✅️replace-query-result-apply",
    "✅️set-editor-selection-applied": "✅️set-editor-selection-apply",
}

SCOPED = {
    "✅️replace-query-result-applied": "📊️replace-query-result/",
    "✅️set-editor-selection-applied": "🔤️set-editor-selection/",
}


def u16(value: str) -> int:
    return len(value.encode("utf-16-le")) // 2


def tracked() -> list[str]:
    raw = subprocess.check_output(["git", "-c", "core.quotepath=off", "ls-files", "-z"], cwd=ROOT)
    return [path for path in raw.decode().split("\0") if path]


def main() -> None:
    os.chdir(ROOT)
    paths = tracked()
    removed = []
    for path in paths:
        if "🎫️tickets" in path and u16(path) > FILE_BUDGET:
            os.remove(path)
            removed.append(path)
    print(f"removed {len(removed)} ticket files over {FILE_BUDGET}")

    replaced_files = 0
    for path in paths:
        if "🎫️tickets" in path or not os.path.isfile(path):
            continue
        with open(path, "rb") as handle:
            blob = handle.read()
        if b"\0" in blob:
            continue
        try:
            text = blob.decode("utf-8")
        except UnicodeDecodeError:
            continue
        updated = text
        for old, new in RENAMES.items():
            scope = SCOPED.get(old)
            if scope is not None and scope not in path and scope not in text:
                continue
            if scope is not None and scope not in text and old not in text:
                continue
            if scope is not None:
                # Keep the writer fixture of the same leaf name.
                if old == "✅️set-editor-selection-applied" and "🔱️trinity/" not in path and "🔤️set-editor-selection/" not in text:
                    continue
                if old == "✅️replace-query-result-applied" and "📊️replace-query-result/" not in path and "📊️replace-query-result/" not in text:
                    continue
            updated = updated.replace(old, new)
        if updated != text:
            with open(path, "w", encoding="utf-8", newline="") as handle:
                handle.write(updated)
            replaced_files += 1
    print(f"rewrote {replaced_files} referencing files")

    renamed = 0
    for path in paths:
        if "🎫️tickets" in path:
            continue
        parts = path.split("/")
        for old, new in RENAMES.items():
            if old not in parts:
                continue
            if old == "✅️set-editor-selection-applied" and "🔱️trinity" not in parts:
                continue
            if old == "✅️replace-query-result-applied" and "📊️replace-query-result" not in parts:
                continue
            index = parts.index(old)
            parent = "/".join(parts[:index])
            source = os.path.join(ROOT, parent, old) if parent else os.path.join(ROOT, old)
            target = os.path.join(ROOT, parent, new) if parent else os.path.join(ROOT, new)
            if os.path.isdir(source) and not os.path.exists(target):
                os.rename(source, target)
                renamed += 1
                print(f"renamed {parent}/{old}")
            break
    print(f"renamed {renamed} directories")


if __name__ == "__main__":
    main()
