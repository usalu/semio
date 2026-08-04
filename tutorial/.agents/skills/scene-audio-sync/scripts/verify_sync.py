#!/usr/bin/env python3
"""Verify narration ↔ animation sync after synthesis / mux."""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any, Optional


def probe_duration(path: Path) -> float:
    if not path.exists() or not shutil.which("ffprobe"):
        return 0.0
    try:
        proc = subprocess.run(
            [
                "ffprobe",
                "-v",
                "error",
                "-show_entries",
                "format=duration",
                "-of",
                "default=nw=1:nk=1",
                str(path),
            ],
            capture_output=True,
            text=True,
            timeout=30,
        )
        return float((proc.stdout or "0").strip() or 0)
    except Exception:  # noqa: BLE001
        return 0.0


def main(argv: Optional[list[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("transcript", type=Path)
    parser.add_argument("--job-dir", type=Path, required=True)
    parser.add_argument("--tolerance", type=float, default=1.2)
    args = parser.parse_args(argv)

    data = json.loads(args.transcript.read_text(encoding="utf-8"))
    job_dir = args.job_dir.resolve()
    tdir = args.transcript.parent.resolve()
    warnings = 0
    errors = 0

    for scene in data.get("scenes") or []:
        if not isinstance(scene, dict):
            continue
        sid = str(scene.get("id") or "")
        narration = str(scene.get("full_narration") or "").strip()
        if not narration:
            print(f"ERROR {sid}: empty full_narration")
            errors += 1
            continue

        audio = Path(scene["audio_path"]) if scene.get("audio_path") else tdir / "scenes" / sid / "audio.wav"
        if not audio.exists():
            audio = tdir / "scenes" / sid / "audio.mp3"
        vo = tdir / "scenes" / sid / "scene_vo.mp4"
        if not vo.exists():
            vo = job_dir / "scenes" / sid / "scene_vo.mp4"
        video = None
        if scene.get("video_path"):
            video = job_dir / str(scene["video_path"])
        if video is None or not video.exists():
            video = job_dir / "scenes" / sid / "scene.mp4"

        a = probe_duration(audio) if audio.exists() else float(scene.get("audio_duration_seconds") or 0)
        v = probe_duration(video) if video and video.exists() else float(scene.get("video_duration_seconds") or 0)
        vo_d = probe_duration(vo) if vo.exists() else 0.0

        print(f"{sid}: narr_words={len(narration.split())} audio={a:.2f}s video={v:.2f}s vo={vo_d:.2f}s")

        if a <= 0:
            print(f"  ERROR missing audio")
            errors += 1
        if v > 0 and a > 0 and abs(a - v) > args.tolerance:
            # Freeze-frame mux is expected when audio > video; warn only if VO didn't cover
            if vo_d + 0.05 < max(a, v) - args.tolerance:
                print(
                    f"  WARN |audio-video|={abs(a - v):.2f}s and VO shorter than needed — "
                    "reword narration or re-mux"
                )
                warnings += 1
            else:
                print(
                    f"  NOTE |audio-video|={abs(a - v):.2f}s (OK if VO used freeze/pad; vo={vo_d:.2f}s)"
                )

        beats = scene.get("beats") if isinstance(scene.get("beats"), list) else []
        for b in beats:
            if not isinstance(b, dict):
                continue
            line = str(b.get("narration") or "").strip()
            if line and len(line.split()) > 28:
                print(f"  WARN beat {b.get('index')}: long line ({len(line.split())} words) — may feel rushed")
                warnings += 1

    print(f"Done: {errors} error(s), {warnings} warning(s)")
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
