"""🔑 Find staffage cache folders that no manifest entry produces any more, recomputing the cast keys exactly as the Python loader `cast_key` does.

@see ../../🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py — the loader whose key derivation is mirrored here
"""

import argparse
import hashlib
import json
import os
import re
import shutil
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve()
W = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
SOURCES = W / "🎓️teaching/🏛️architecture/🧍️staffage"
MANIFEST = W / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json"
BASE = os.environ.get("SEMIO_TEACHING_CACHE_ROOT")
CACHE = (W / BASE if BASE else W / ".🧬semio/🎓️teaching/⚡️cache") / "staffage"
KEY = re.compile(r"^[0-9a-f]{32}$")
TMP_MIN_AGE_S = 3600


def source_digest(root):
    """🔐 Same hash as the loader: every file of the exporter and schema folders in Path order."""
    digest = hashlib.sha256()
    files = sorted(f for folder in ("🔨️modules", "🧬️schema") for f in (root / folder).rglob("*") if f.is_file() and "node_modules" not in f.parts)
    for f in files:
        digest.update(f.relative_to(root).as_posix().encode("utf-8") + b"\0" + f.read_bytes() + b"\0")
    return digest.hexdigest()


def catalog_bytes(slug, root):
    """🗂️ The catalog file of an asset, first match in Path order."""
    matches = sorted((root / "🗂️catalog").glob(f"*/{slug}/🔣️.json"))
    if not matches:
        raise FileNotFoundError(f"Asset {slug!r} is not in the staffage catalog")
    return matches[0].read_bytes()


def environment_bytes(root):
    """🌤️ All environment preset files, in Path order."""
    return b"".join(f.read_bytes() for f in sorted((root / "🗂️catalog").glob("*environments*/🔣️.json")))


def cast_key(entry, sources_digest, root=SOURCES):
    """🔑 Content address of a cast: its entry, its catalog file, the environment presets when needed, and the exporter sources."""
    digest = hashlib.sha256()
    digest.update(json.dumps(entry, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8") + b"\0")
    digest.update(catalog_bytes(entry["asset"], root) + b"\0")
    if "environment" in entry or "timeline" in entry:
        digest.update(environment_bytes(root) + b"\0")
    digest.update(sources_digest.encode("ascii"))
    return digest.hexdigest()[:32]


def tree_size(path):
    """📏 Bytes and file count of a folder tree."""
    size = count = 0
    for f in path.rglob("*"):
        if f.is_file():
            size += f.stat().st_size
            count += 1
    return size, count


def main():
    """🧹 Report the cache, list stale folders and optionally delete them."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--report", required=True)
    parser.add_argument("--delete", action="store_true")
    args = parser.parse_args()

    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    casts = manifest["casts"]
    sd = source_digest(SOURCES)
    current = {}
    for cast, entry in casts.items():
        current.setdefault(cast_key(entry, sd), []).append(cast)

    now = time.time()
    entries = []
    for child in sorted(CACHE.iterdir(), key=lambda p: p.name):
        if not child.is_dir():
            continue
        size, count = tree_size(child)
        inner = sorted(p.name for p in child.iterdir() if p.is_dir())
        kind = "current" if child.name in current else ("key-stale" if KEY.match(child.name) else "temporary")
        complete = all((child / c / "🔣️.json").is_file() for c in current.get(child.name, []))
        entries.append({"name": child.name, "kind": kind, "bytes": size, "files": count, "casts": inner, "age_s": round(now - child.stat().st_mtime), "complete": complete if kind == "current" else None})

    stale = [e for e in entries if e["kind"] != "current"]
    deleted = []
    if args.delete:
        for e in stale:
            if e["kind"] == "temporary" and e["age_s"] < TMP_MIN_AGE_S:
                continue
            shutil.rmtree(CACHE / e["name"], ignore_errors=False)
            deleted.append(e["name"])

    missing = sorted(c for k, cs in current.items() if not any(e["name"] == k for e in entries) for c in cs)
    incomplete = sorted(e["name"] for e in entries if e["kind"] == "current" and not e["complete"])
    summary = {
        "cache": str(CACHE),
        "manifest_casts": len(casts),
        "current_keys": len(current),
        "cache_entries": len(entries),
        "current_entries": sum(1 for e in entries if e["kind"] == "current"),
        "stale_key_entries": sum(1 for e in entries if e["kind"] == "key-stale"),
        "temporary_entries": sum(1 for e in entries if e["kind"] == "temporary"),
        "total_bytes": sum(e["bytes"] for e in entries),
        "stale_bytes": sum(e["bytes"] for e in stale),
        "current_bytes": sum(e["bytes"] for e in entries if e["kind"] == "current"),
        "casts_without_folder": len(missing),
        "incomplete_current_keys": incomplete,
        "deleted": deleted,
        "stale": stale,
    }
    Path(args.report).write_text(json.dumps(summary, ensure_ascii=False, indent=2), encoding="utf-8")
    for k in ("cache", "manifest_casts", "current_keys", "cache_entries", "current_entries", "stale_key_entries", "temporary_entries", "total_bytes", "stale_bytes", "current_bytes", "casts_without_folder"):
        print(f"{k}: {summary[k]}")
    print(f"incomplete_current_keys: {len(incomplete)}")
    print(f"deleted: {len(deleted)}")


if __name__ == "__main__":
    sys.exit(main())
