"""🎬 Full Physical Fundamentals video — all eight beats in curriculum order.

Recommended: run this file as a script (beat renders + ffmpeg concat).
"""

from __future__ import annotations

import argparse
import importlib.util
import shutil
import subprocess
import sys
from pathlib import Path
from types import ModuleType

from manim import Scene

_PF_ROOT = Path(__file__).resolve().parent
_SEMIO_ROOT = next(
    p for p in _PF_ROOT.parents if (p / ".venv").is_dir() or (p / "package.json").is_file()
)
# region Intro
_INTRO_SCENE = _SEMIO_ROOT / "tutorial" / "intro" / "intro_scene.py"
INTRO_CLASS_NAME = "Demo_Intro_PhysikalischeGrundlagen"
# endregion


def _load_module(module_name: str, path: Path) -> ModuleType:
    spec = importlib.util.spec_from_file_location(module_name, path)
    if spec is None or spec.loader is None:
        raise ImportError(f"Cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[module_name] = module
    spec.loader.exec_module(module)
    return module


_m1 = _load_module("pf_full_m1", _PF_ROOT / "scene_1.py")

PHYSICAL_FUNDAMENTALS_PLAYLIST: list[tuple[str, list[type[Scene]]]] = [
    (
        "Physikalische Grundlagen",
        [
            _m1.Beat1_UnsichtbareDimension,
            _m1.Beat2_KraftUndArbeit,
            _m1.Beat3_ArbeitZuLeistung,
            _m1.Beat4_Kilowattstunde,
            _m1.Beat5_Groessenordnungen,
            _m1.Beat6_Energieerhaltung,
            _m1.Beat7_Waermepumpe,
            _m1.Beat8_Ausblick,
        ],
    ),
]

_BIND_ATTRS = ("NARRATION", "topic_de", "topic_explain_de", "series_de", "hold_seconds")


def _bind_scene_attrs(host: Scene, scene_cls: type[Scene]) -> None:
    for name in _BIND_ATTRS:
        if hasattr(scene_cls, name):
            setattr(host, name, getattr(scene_cls, name))
    _m1.begin_vo_beat(host, scene_cls.__name__)


class PhysicalFundamentals_FullSection(Scene):
    section_beats = PHYSICAL_FUNDAMENTALS_PLAYLIST[0][1]

    def construct(self):
        for scene_cls in self.section_beats:
            _bind_scene_attrs(self, scene_cls)
            scene_cls.construct(self)
            self.clear()


class FullPhysicalFundamentalsVideo(Scene):
    def construct(self):
        for _section, scenes in PHYSICAL_FUNDAMENTALS_PLAYLIST:
            for scene_cls in scenes:
                _bind_scene_attrs(self, scene_cls)
                scene_cls.construct(self)
                self.clear()


def _manim_bin() -> Path:
    candidate = _SEMIO_ROOT / ".venv" / "bin" / "manim"
    if candidate.is_file():
        return candidate
    which = shutil.which("manim")
    if which:
        return Path(which)
    raise FileNotFoundError("manim not found — activate .venv or install Manim")


def _quality_folder(quality_flag: str) -> str:
    return {"-ql": "480p15", "-qm": "720p30", "-qh": "1080p60"}.get(quality_flag, "480p15")


def _find_section_mp4(media_dir: Path, scene_name: str, quality_flag: str) -> Path:
    folder = _quality_folder(quality_flag)
    preferred = [
        media_dir / "videos" / "full_physical_fundamentals_video" / folder / f"{scene_name}.mp4",
        media_dir / "videos" / "intro_scene" / folder / f"{scene_name}.mp4",
        media_dir / "videos" / "scene_1" / folder / f"{scene_name}.mp4",
    ]
    for path in preferred:
        if path.is_file() and path.stat().st_size > 1000:
            return path
    matches = sorted(
        (media_dir / "videos").rglob(f"{scene_name}.mp4"),
        key=lambda p: p.stat().st_mtime,
        reverse=True,
    )
    for match in matches:
        if folder in match.parts and match.stat().st_size > 1000:
            return match
    raise FileNotFoundError(f"Rendered mp4 not found for {scene_name} under {media_dir}")


def _ffmpeg_concat_silent(clips: list[Path], output: Path, list_path: Path) -> None:
    """🎞️ Concatenate clips and drop any audio stream."""
    lines = []
    for clip in clips:
        escaped = str(clip.resolve()).replace("'", r"'\''")
        lines.append(f"file '{escaped}'")
    list_path.parent.mkdir(parents=True, exist_ok=True)
    list_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    output.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        [
            "ffmpeg", "-y",
            "-f", "concat", "-safe", "0",
            "-i", str(list_path),
            "-c:v", "copy", "-an",
            str(output),
        ],
        check=True,
    )


def compose_full_physical_fundamentals_video(
    *,
    quality_flag: str = "-ql",
    play: bool = True,
    media_dir: Path | None = None,
    force: bool = False,
) -> Path:
    """🎬 Render intro + each beat, silent ffmpeg-concat, copy to ``rendered/``."""
    media_dir = media_dir or (_PF_ROOT / "media")
    manim = _manim_bin()
    clips: list[Path] = []
    folder = _quality_folder(quality_flag)
    beat_script = _PF_ROOT / "scene_1.py"

    def _maybe_existing(scene_name: str) -> Path | None:
        if force:
            return None
        try:
            return _find_section_mp4(media_dir, scene_name, quality_flag)
        except FileNotFoundError:
            return None

    def _render(script: Path, scene_name: str) -> Path:
        existing = _maybe_existing(scene_name)
        if existing is not None:
            print(f"\n=== Skipping {scene_name} (already rendered) ===")
            return existing
        print(f"\n=== Rendering {scene_name} ===")
        subprocess.run(
            [
                str(manim),
                quality_flag,
                "--media_dir", str(media_dir),
                str(script),
                scene_name,
            ],
            check=True,
            cwd=str(_SEMIO_ROOT),
        )
        return _find_section_mp4(media_dir, scene_name, quality_flag)

    clips.append(_render(_INTRO_SCENE, INTRO_CLASS_NAME))
    for beat_cls in PHYSICAL_FUNDAMENTALS_PLAYLIST[0][1]:
        clips.append(_render(beat_script, beat_cls.__name__))

    out_dir = media_dir / "videos" / "full_physical_fundamentals_video" / folder
    output = out_dir / "FullPhysicalFundamentalsVideo.mp4"
    out_dir.mkdir(parents=True, exist_ok=True)
    print(f"\n=== Merging {len(clips)} silent clips (intro + beats) → {output.name} ===")
    _ffmpeg_concat_silent(clips, output, out_dir / "concat_list.txt")

    rendered_dir = _PF_ROOT / "rendered"
    rendered_dir.mkdir(parents=True, exist_ok=True)
    rendered_copy = rendered_dir / f"Full_Physical_Fundamentals_{folder}.mp4"
    noaudio_copy = rendered_dir / f"Full_Physical_Fundamentals_NoAudio_{folder}.mp4"
    shutil.copy2(output, rendered_copy)
    shutil.copy2(output, noaudio_copy)
    print(f"\n✅ Ready: {output}")
    print(f"✅ Copy:  {rendered_copy}")
    print(f"✅ Copy:  {noaudio_copy}")
    if play:
        opener = {"darwin": "open", "win32": "start"}.get(sys.platform, "xdg-open")
        if sys.platform == "win32":
            subprocess.run(["cmd", "/c", "start", "", str(rendered_copy)], check=False)
        else:
            subprocess.run([opener, str(rendered_copy)], check=False)
    return rendered_copy


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Render the Physical Fundamentals video.")
    parser.add_argument("-q", choices=("l", "m", "h"), default="l", help="Quality: l/m/h")
    parser.add_argument("--no-play", action="store_true", help="Do not open the finished mp4")
    parser.add_argument(
        "--force",
        action="store_true",
        help="Re-render intro and every beat even if mp4s already exist",
    )
    args = parser.parse_args(argv)
    quality_flag = {"l": "-ql", "m": "-qm", "h": "-qh"}[args.q]
    compose_full_physical_fundamentals_video(
        quality_flag=quality_flag,
        play=not args.no_play,
        force=args.force,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
