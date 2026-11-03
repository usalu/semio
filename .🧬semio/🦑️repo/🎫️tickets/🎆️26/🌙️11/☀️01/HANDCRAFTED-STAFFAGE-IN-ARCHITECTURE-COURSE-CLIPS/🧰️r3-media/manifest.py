"""🗂️ Write the media manifest of one language and audience: every fresh clip and lesson master with its staffage provenance, poster and thumbnail.

Usage: python manifest.py <en|de> [speaker|spectator] [--video <id>]...
Output: .🧬semio/🎓️teaching/⚡️cache/videos/<lang>/<audience>/media-manifest.json and posters/<...>.poster.png, .thumb.png
A stale or missing clip is listed with `fresh: false` and gets neither poster nor thumbnail, so old artwork is never served.
The `media` key of an entry is its cache key: the SHA-256 of the file, the segment state hash and the staffage provenance.
"""
import argparse
import hashlib
import json
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import lib

VERSION = 1
parser = argparse.ArgumentParser()
parser.add_argument("language", choices=["en", "de"])
parser.add_argument("audience", nargs="?", default="speaker", choices=["speaker", "spectator"])
parser.add_argument("--video", action="append")
args = parser.parse_args()
runtime = lib.runtime()
folder = lib.base(args.language, args.audience)
target = folder / "media-manifest.json"
previous = json.loads(target.read_text(encoding="utf-8")) if target.is_file() else {"videos": {}}


def reuse(record: dict | None, file: Path, key: str) -> bool:
    """♻️ Keep stills whose clip, provenance and files are unchanged."""
    return bool(record and record.get("media") == key and all((folder / record["stills"][name]).is_file() for name in ("poster", "thumbnail")))


def entry(name: str, file: Path, state_hashes: list[str], provenance: dict, fresh: bool, kept: dict | None, relative: str) -> dict:
    """🧾 One manifest entry; stills only for a fresh file."""
    if not file.is_file():
        return {"file": str(file.relative_to(lib.ROOT)), "fresh": False, "missing": True}
    sha = lib.digest(file)
    key = hashlib.sha256(lib.canonical({"sha256": sha, "states": state_hashes, "provenance": provenance, "version": VERSION}).encode()).hexdigest()
    record = {"file": str(file.relative_to(lib.ROOT)), "fresh": fresh, "sha256": sha, "bytes": file.stat().st_size,
              "seconds": round(lib.duration(file), 2), "states": state_hashes, "provenance": provenance, "media": key}
    if fresh:
        if reuse(kept, file, key):
            record["stills"] = kept["stills"]
        else:
            made = lib.stills(file, folder / "posters" / relative, name)
            record["stills"] = {"poster": str(made["poster"].relative_to(folder)), "thumbnail": str(made["thumbnail"].relative_to(folder)),
                                "poster_sha256": made["poster_sha256"], "thumbnail_sha256": made["thumbnail_sha256"]}
    return record


def merged(a: dict, b: dict) -> dict:
    """🧬 Union of the staffage provenance of several segments."""
    return {"casts": {**a["casts"], **b["casts"]}, "records": {**a["records"], **b.get("records", {})}}


states, videos, started = {}, {}, time.time()
for video in runtime.load_manifest():
    if args.video and video["id"] not in args.video:
        continue
    output = video.get("output", video["id"])
    status = lib.video_status(video["id"], args.language, args.audience, output, states)
    kept = previous["videos"].get(video["id"], {})
    segments, whole = {}, {"casts": {}, "records": {}}
    for segment in status["segments"]:
        provenance = lib.provenance(segment["state"], segment["fresh"])
        whole = merged(whole, provenance)
        key = f"{segment['ordinal']}-{segment['scene']}"
        segments[key] = {"scene": segment["scene"], "script": segment["script"],
                         **entry(segment["scene"], segment["clip"], [segment["state_sha256"]], provenance, segment["fresh"], kept.get("segments", {}).get(key), f"{video['id']}/s"),
                         }
    master = status["master"]
    hashes = [segment["state_sha256"] for segment in status["segments"]]
    videos[video["id"]] = {"output": output, "master": entry(output, master, hashes, whole, lib.master_fresh(status), kept.get("master"), "lessons"), "segments": segments}
    fresh = sum(record["fresh"] for record in segments.values())
    print(f"[DEBUG] {video['id']}: segments {fresh}/{len(segments)} fresh, master fresh={videos[video['id']]['master']['fresh']}", flush=True)
document = {"version": VERSION, "language": args.language, "audience": args.audience, "settings": lib.settings(args.language, args.audience, lib.QUALITY, lib.FPS),
            "videos": {**previous["videos"], **videos}}
target.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(f"[DEBUG] wrote {target} in {time.time() - started:.0f} s")
