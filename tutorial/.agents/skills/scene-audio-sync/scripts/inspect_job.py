#!/usr/bin/env python3
"""Inspect an animation job folder and emit structured scene facts for transcript drafting."""

from __future__ import annotations

import argparse
import json
import re
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
            check=False,
        )
        return float((proc.stdout or "0").strip() or 0)
    except Exception:  # noqa: BLE001
        return 0.0


def _load_json(path: Path) -> Optional[dict[str, Any]]:
    if not path.exists():
        return None
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
        return data if isinstance(data, dict) else None
    except Exception:  # noqa: BLE001
        return None


def _code_cues(code: str, *, limit: int = 12) -> list[str]:
    labels = re.findall(r"""(?:Text|MarkupText)\(\s*['"]([^'"\n]{1,60})['"]""", code)
    # Keep order, drop empties/dupes
    out: list[str] = []
    for lab in labels:
        lab = lab.strip()
        if lab and lab not in out:
            out.append(lab)
        if len(out) >= limit:
            break
    return out


def _pick_video(scene_dir: Path) -> Optional[Path]:
    for name in ("scene.mp4", "scene_vo.mp4"):
        cand = scene_dir / name
        if cand.exists() and cand.stat().st_size > 0:
            return cand
    mp4s = sorted(scene_dir.glob("*.mp4"))
    return mp4s[0] if mp4s else None


def _scene_from_section(
    section: dict[str, Any],
    *,
    scene_dir: Optional[Path],
    job_dir: Path,
) -> dict[str, Any]:
    sid = str(section.get("id") or (scene_dir.name if scene_dir else "scene"))
    beats = section.get("beats") if isinstance(section.get("beats"), list) else []
    narration = section.get("narration")
    if not narration and beats:
        narration = " ".join(
            str(b.get("narration") or "").strip()
            for b in beats
            if isinstance(b, dict) and str(b.get("narration") or "").strip()
        )
    code_cues: list[str] = []
    video_path = None
    video_dur = 0.0
    if scene_dir and scene_dir.is_dir():
        code_path = scene_dir / "code_final.py"
        if not code_path.exists():
            revs = sorted(scene_dir.glob("code_r*.py"))
            code_path = revs[-1] if revs else code_path
        if code_path.exists():
            code_cues = _code_cues(code_path.read_text(encoding="utf-8", errors="ignore"))
        vid = _pick_video(scene_dir)
        if vid is not None:
            video_path = str(vid.relative_to(job_dir))
            video_dur = probe_duration(vid)
    return {
        "id": sid,
        "title": str(section.get("title") or sid),
        "visual_description": str(section.get("visual_description") or ""),
        "camera_notes": str(section.get("camera_notes") or ""),
        "visual_device": str(section.get("visual_device") or ""),
        "style_tags": section.get("style_tags") or [],
        "duration_seconds_plan": float(section.get("duration_seconds") or 0) or None,
        "existing_narration": (str(narration).strip() if narration else ""),
        "beats": beats,
        "animation_beats": section.get("animation_beats") or [],
        "code_cues": code_cues,
        "video_path": video_path,
        "video_duration_seconds": video_dur,
        "scene_dir": str(scene_dir.relative_to(job_dir)) if scene_dir else None,
    }


def inspect_job(job_dir: Path) -> dict[str, Any]:
    job_dir = job_dir.resolve()
    plan = _load_json(job_dir / "scene_plan.json") or {}
    meta = _load_json(job_dir / "meta.json") or {}
    scenes_out: list[dict[str, Any]] = []

    plan_scenes = plan.get("scenes") if isinstance(plan.get("scenes"), list) else []
    scenes_root = job_dir / "scenes"

    if plan_scenes:
        for raw in plan_scenes:
            if not isinstance(raw, dict):
                continue
            sid = str(raw.get("id") or "")
            sdir = scenes_root / sid if sid and (scenes_root / sid).is_dir() else None
            section = raw
            if sdir is not None:
                disk = _load_json(sdir / "section.json")
                if disk:
                    # Disk section may be more up to date for duration/beats
                    section = {**raw, **disk}
            scenes_out.append(_scene_from_section(section, scene_dir=sdir, job_dir=job_dir))
    elif scenes_root.is_dir():
        for sdir in sorted(p for p in scenes_root.iterdir() if p.is_dir()):
            section = _load_json(sdir / "section.json") or {"id": sdir.name, "title": sdir.name}
            scenes_out.append(_scene_from_section(section, scene_dir=sdir, job_dir=job_dir))
    else:
        # Generic: any subfolder with mp4
        for sdir in sorted(p for p in job_dir.iterdir() if p.is_dir()):
            if _pick_video(sdir) is None:
                continue
            section = _load_json(sdir / "section.json") or {"id": sdir.name, "title": sdir.name}
            scenes_out.append(_scene_from_section(section, scene_dir=sdir, job_dir=job_dir))

    return {
        "job_dir": str(job_dir),
        "job_id": job_dir.name,
        "title": plan.get("title") or meta.get("title") or job_dir.name,
        "concept_summary": plan.get("concept_summary") or "",
        "style_notes": plan.get("style_notes") or "",
        "visual_identity": plan.get("visual_identity") or "",
        "language": (meta.get("settings") or {}).get("language")
        if isinstance(meta.get("settings"), dict)
        else "en",
        "scene_count": len(scenes_out),
        "scenes": scenes_out,
        "notes": [
            "Agent must author transcript.json — do not call external LLMs for narration.",
            "Prefer video_duration_seconds as speaking budget; freeze-frame on mux if audio longer.",
            "Voice should be slow, continuous, and understandable across the whole job.",
        ],
    }


def main(argv: Optional[list[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("job_dir", type=Path)
    parser.add_argument("--out", type=Path, default=None)
    args = parser.parse_args(argv)
    job_dir = args.job_dir
    if not job_dir.is_dir():
        print(f"Not a directory: {job_dir}", file=sys.stderr)
        return 2
    data = inspect_job(job_dir)
    text = json.dumps(data, indent=2, ensure_ascii=False)
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text + "\n", encoding="utf-8")
        print(f"Wrote {args.out}")
    else:
        print(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
