"""📋 Report which segment clips and lesson masters of the course are fresh, stale or missing against the current sources and staffage cast keys.

Usage: python status.py <en|de> [speaker|spectator] [--video <id>]... [--stale-only]
Exit code 1 when anything is stale or missing.
"""
import argparse
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import lib

parser = argparse.ArgumentParser()
parser.add_argument("language", choices=["en", "de"])
parser.add_argument("audience", nargs="?", default="speaker", choices=["speaker", "spectator"])
parser.add_argument("--video", action="append")
parser.add_argument("--stale-only", action="store_true")
args = parser.parse_args()
runtime = lib.runtime()
states, bad, started = {}, 0, time.time()
for video in runtime.load_manifest():
    if args.video and video["id"] not in args.video:
        continue
    status = lib.video_status(video["id"], args.language, args.audience, video.get("output", video["id"]), states)
    fresh = sum(segment["fresh"] for segment in status["segments"])
    master = "fresh" if lib.master_fresh(status) else ("missing" if not status["master"].is_file() else "stale")
    bad += master != "fresh" or fresh != len(status["segments"])
    print(f"{video['id']:28} segments {fresh}/{len(status['segments'])} fresh   master {master}", flush=True)
    for segment in status["segments"]:
        if not segment["fresh"] or not args.stale_only:
            kind = "stale" if segment["clip"].is_file() else "missing"
            print(f"    {'fresh' if segment['fresh'] else kind:8} {segment['scene']}  casts={len(segment['state'].get('casts', {}))}")
print(f"[DEBUG] {time.time() - started:.0f} s; {bad} videos not fully fresh")
sys.exit(1 if bad else 0)
