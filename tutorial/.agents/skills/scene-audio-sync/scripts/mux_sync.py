#!/usr/bin/env python3
"""Mux scene narration onto animation clips with freeze-frame / silence padding."""

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


def _run(cmd: list[str]) -> bool:
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
        return proc.returncode == 0
    except Exception:  # noqa: BLE001
        return False


def mux_scene(
    video: Path,
    audio: Path,
    output: Path,
    *,
    subtitle_text: str = "",
    burn_subtitles: bool = False,
) -> Optional[Path]:
    if not shutil.which("ffmpeg"):
        print("ffmpeg not found", file=sys.stderr)
        return None
    if not video.exists() or not audio.exists():
        return None
    output.parent.mkdir(parents=True, exist_ok=True)
    v_dur = probe_duration(video)
    a_dur = probe_duration(audio)
    target = max(v_dur, a_dur, 0.5) + 0.15

    srt: Optional[Path] = None
    subs = ""
    if burn_subtitles and subtitle_text.strip():
        srt = output.with_suffix(".srt")
        # Single cue spanning full narration — fine for skill; compose.py has richer chunking
        srt.write_text(
            f"1\n00:00:00,000 --> {_srt(target)}\n{subtitle_text.strip()}\n",
            encoding="utf-8",
        )
        path = str(srt.resolve()).replace("\\", "\\\\").replace(":", "\\:").replace("'", r"\'")
        style = (
            "FontName=Arial,FontSize=18,PrimaryColour=&H00FFFFFF&,"
            "OutlineColour=&HAA000000&,BorderStyle=3,Outline=2,Shadow=0,"
            "MarginV=42,Alignment=2"
        )
        subs = f",subtitles='{path}':force_style='{style}'"

    write_path = output
    if video.resolve() == output.resolve():
        write_path = output.with_name(output.stem + ".mux.tmp.mp4")

    video_chain = (
        f"[0:v]tpad=stop_mode=clone:stop_duration={max(0.0, target - v_dur):.3f},"
        f"fps=30,format=yuv420p{subs}[v]"
    )
    audio_chain = (
        "[1:a]aformat=sample_fmts=fltp:sample_rates=44100:channel_layouts=stereo,"
        f"apad=whole_dur={target:.3f},atrim=0:{target:.3f}[a]"
    )
    cmd = [
        "ffmpeg",
        "-y",
        "-i",
        str(video),
        "-i",
        str(audio),
        "-filter_complex",
        f"{video_chain};{audio_chain}",
        "-map",
        "[v]",
        "-map",
        "[a]",
        "-c:v",
        "libx264",
        "-preset",
        "veryfast",
        "-crf",
        "23",
        "-c:a",
        "aac",
        "-b:a",
        "192k",
        "-movflags",
        "+faststart",
        str(write_path),
    ]
    ok = _run(cmd)
    if not ok and subs:
        # retry without subs
        cmd_no = [
            "ffmpeg",
            "-y",
            "-i",
            str(video),
            "-i",
            str(audio),
            "-filter_complex",
            (
                f"[0:v]tpad=stop_mode=clone:stop_duration={max(0.0, target - v_dur):.3f},"
                f"fps=30,format=yuv420p[v];"
                f"[1:a]aformat=sample_fmts=fltp:sample_rates=44100:channel_layouts=stereo,"
                f"apad=whole_dur={target:.3f},atrim=0:{target:.3f}[a]"
            ),
            "-map",
            "[v]",
            "-map",
            "[a]",
            "-c:v",
            "libx264",
            "-preset",
            "veryfast",
            "-crf",
            "23",
            "-c:a",
            "aac",
            "-b:a",
            "192k",
            "-movflags",
            "+faststart",
            str(write_path),
        ]
        ok = _run(cmd_no)

    if write_path != output:
        if ok and write_path.exists() and write_path.stat().st_size > 0:
            write_path.replace(output)
        else:
            write_path.unlink(missing_ok=True)
            return None
    return output if output.exists() and output.stat().st_size > 0 else None


def _srt(seconds: float) -> str:
    ms = int(round(max(0.0, seconds) * 1000))
    h, rem = divmod(ms, 3_600_000)
    m, rem = divmod(rem, 60_000)
    s, ms = divmod(rem, 1000)
    return f"{h:02d}:{m:02d}:{s:02d},{ms:03d}"


def resolve_video(job_dir: Path, scene: dict[str, Any]) -> Optional[Path]:
    rel = scene.get("video_path")
    if rel:
        p = (job_dir / str(rel)).resolve()
        if p.exists():
            return p
    sid = str(scene.get("id") or "")
    for name in ("scene.mp4", "scene_vo.mp4"):
        cand = job_dir / "scenes" / sid / name
        if cand.exists():
            return cand
    return None


def resolve_audio(transcript_dir: Path, scene: dict[str, Any]) -> Optional[Path]:
    ap = scene.get("audio_path")
    if ap and Path(ap).exists():
        return Path(ap)
    sid = str(scene.get("id") or "")
    for name in ("audio.wav", "audio.mp3"):
        cand = transcript_dir / "scenes" / sid / name
        if cand.exists():
            return cand
    return None


def compose_final(clips: list[Path], output: Path) -> Optional[Path]:
    if not clips or not shutil.which("ffmpeg"):
        return None
    output.parent.mkdir(parents=True, exist_ok=True)
    if len(clips) == 1:
        shutil.copy2(clips[0], output)
        return output
    list_file = output.parent / "concat_list_skill.txt"
    list_file.write_text(
        "\n".join(f"file '{c.resolve()}'" for c in clips) + "\n",
        encoding="utf-8",
    )
    # Normalize then concat
    norms: list[Path] = []
    for i, src in enumerate(clips):
        norm = output.parent / f"skill_norm_{i:02d}.mp4"
        ok = _run(
            [
                "ffmpeg",
                "-y",
                "-i",
                str(src),
                "-vf",
                "scale=1280:720:force_original_aspect_ratio=decrease,"
                "pad=1280:720:(ow-iw)/2:(oh-ih)/2,fps=30,format=yuv420p",
                "-c:v",
                "libx264",
                "-preset",
                "veryfast",
                "-crf",
                "23",
                "-c:a",
                "aac",
                "-ar",
                "44100",
                "-ac",
                "2",
                "-b:a",
                "192k",
                "-movflags",
                "+faststart",
                str(norm),
            ]
        )
        norms.append(norm if ok and norm.exists() else src)
    list_file.write_text(
        "\n".join(f"file '{c.resolve()}'" for c in norms) + "\n",
        encoding="utf-8",
    )
    ok = _run(
        [
            "ffmpeg",
            "-y",
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            str(list_file),
            "-c",
            "copy",
            "-movflags",
            "+faststart",
            str(output),
        ]
    )
    return output if ok and output.exists() else None


def main(argv: Optional[list[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("transcript", type=Path)
    parser.add_argument("--job-dir", type=Path, required=True)
    parser.add_argument("--publish", action="store_true", help="Copy audio+VO into scenes/<id>/")
    parser.add_argument("--subtitles", action="store_true")
    parser.add_argument("--final", action="store_true", help="Also stitch final.mp4")
    args = parser.parse_args(argv)

    if not shutil.which("ffmpeg"):
        print("ffmpeg required", file=sys.stderr)
        return 2

    data = json.loads(args.transcript.read_text(encoding="utf-8"))
    job_dir = args.job_dir.resolve()
    tdir = args.transcript.parent.resolve()
    vo_clips: list[Path] = []

    for scene in data.get("scenes") or []:
        if not isinstance(scene, dict):
            continue
        sid = str(scene.get("id") or "")
        video = resolve_video(job_dir, scene)
        audio = resolve_audio(tdir, scene)
        if video is None or audio is None:
            print(f"SKIP {sid}: missing video or audio", file=sys.stderr)
            continue
        work_vo = tdir / "scenes" / sid / "scene_vo.mp4"
        muxed = mux_scene(
            video,
            audio,
            work_vo,
            subtitle_text=str(scene.get("full_narration") or ""),
            burn_subtitles=args.subtitles,
        )
        if not muxed:
            print(f"FAIL mux {sid}", file=sys.stderr)
            continue
        print(
            f"{sid}: video={probe_duration(video):.2f}s audio={probe_duration(audio):.2f}s "
            f"→ vo={probe_duration(muxed):.2f}s"
        )
        vo_clips.append(muxed)
        if args.publish:
            dest_dir = job_dir / "scenes" / sid
            dest_dir.mkdir(parents=True, exist_ok=True)
            shutil.copy2(audio, dest_dir / audio.name)
            shutil.copy2(muxed, dest_dir / "scene_vo.mp4")
            # Prefer publishing as scene.mp4 only when user expects VO as main — keep both
            print(f"  published → {dest_dir / 'scene_vo.mp4'}")

    if args.final and vo_clips:
        final = job_dir / "final.mp4"
        out = compose_final(vo_clips, final)
        print(f"final → {out}" if out else "final compose failed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
