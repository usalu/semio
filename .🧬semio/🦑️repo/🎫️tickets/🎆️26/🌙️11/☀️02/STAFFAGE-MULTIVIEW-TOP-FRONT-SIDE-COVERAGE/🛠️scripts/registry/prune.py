"""🗑️ Build the prune list of the staffage cast cache: every cast folder whose key no longer matches the current manifest, or whose index lacks a valid viewpoint. Read-only; the course performs the prune."""

import importlib.util
import json
import sys
import time
from collections import Counter
from pathlib import Path

module_path, output = Path(sys.argv[1]), Path(sys.argv[2])
spec = importlib.util.spec_from_file_location("staffage_cast", module_path)
staffage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(staffage)

manifest = staffage._manifest()["casts"]
keys: dict[str, str] = {}
unkeyable: dict[str, str] = {}
for cast, entry in manifest.items():
    try:
        keys[staffage.cast_key(entry)] = cast
    except ValueError as error:
        unkeyable[cast] = str(error)

workspace = staffage._WORKSPACE
root = staffage._cache_root()
entries = []
reasons_count: Counter = Counter()
key_folders = 0
cast_folders = 0
now = time.time()


def relative(path: Path) -> str:
    return path.relative_to(workspace).as_posix()


def add(path: Path, key: str, cast: str | None, reasons: list[str], viewpoint: str | None = None, frames: int = 0) -> None:
    entries.append({"path": relative(path), "key": key, "cast": cast, "viewpoint": viewpoint, "frames": frames, "reasons": reasons})
    reasons_count.update(reasons)


for key_dir in sorted(root.iterdir()):
    if not key_dir.is_dir():
        continue
    if key_dir.name.startswith(".tmp-"):
        add(key_dir, key_dir.name, None, ["tmp-folder"])
        continue
    key_folders += 1
    current_cast = keys.get(key_dir.name)
    cast_dirs = sorted(p for p in key_dir.iterdir() if p.is_dir())
    if not cast_dirs:
        add(key_dir, key_dir.name, None, ["empty-key"])
        continue
    for cast_dir in cast_dirs:
        cast_folders += 1
        cast = cast_dir.name
        reasons = []
        if current_cast is None:
            reasons.append("key-not-current")
        elif current_cast != cast:
            reasons.append("cast-not-the-key-cast")
        if cast not in manifest:
            reasons.append("cast-not-in-manifest")
        elif cast in unkeyable:
            reasons.append("manifest-row-without-viewpoint")
        viewpoint = None
        index = cast_dir / staffage._INDEX
        if not index.is_file():
            reasons.append("index-missing")
        else:
            data = json.loads(index.read_text(encoding="utf-8"))
            viewpoint = data.get("viewpoint")
            if "viewpoint" not in data:
                reasons.append("index-without-viewpoint")
            elif viewpoint not in staffage._VIEWPOINTS:
                reasons.append("index-viewpoint-unknown")
        if reasons:
            frames = sum(1 for f in cast_dir.iterdir() if f.suffix == ".svg")
            add(cast_dir, key_dir.name, cast, reasons, viewpoint, frames)

listed_keys = {e["key"] for e in entries if e["cast"] is not None}
summary = {
    "keyFolders": key_folders,
    "castFolders": cast_folders,
    "currentKeys": len(keys),
    "manifestCasts": len(manifest),
    "unkeyableManifestCasts": len(unkeyable),
    "listedEntries": len(entries),
    "listedCastFolders": sum(1 for e in entries if e["cast"] is not None),
    "listedFrames": sum(e["frames"] for e in entries),
    "keyFoldersWithAnyListedCast": len(listed_keys),
    "reasons": dict(reasons_count.most_common()),
    "generatedAt": time.strftime("%Y-%m-%d", time.gmtime(now)),
}
result = {
    "generatedBy": "haiku-registry",
    "note": "Read-only prune list. The course performs the prune; nothing here is deleted.",
    "cacheRoot": relative(root),
    "summary": summary,
    "unkeyableManifestCasts": unkeyable,
    "entries": entries,
}
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text(json.dumps(result, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
print(json.dumps(summary, ensure_ascii=False))
