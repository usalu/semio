"""🎬 Full Ventilation series — Chapters 1–6 in curriculum order (video 2 of 5).

Recommended: run this file as a script (section renders + ffmpeg concat).
A single Manim mega-scene can fail when combining hundreds of partials
(especially with spaces in the project path).

See ``README.md`` in this folder.
"""

from __future__ import annotations

import argparse
import importlib.util
import shutil
import subprocess
import sys
import types
from pathlib import Path
from types import ModuleType

from manim import Scene

#region Paths
_VENTILATION_ROOT = Path(__file__).resolve().parent
_SEMIO_ROOT = next(
    p for p in _VENTILATION_ROOT.parents if (p / ".venv").is_dir() or (p / "package.json").is_file()
)
_TUTORIAL_ROOT = _SEMIO_ROOT / "tutorial"
if str(_TUTORIAL_ROOT) not in sys.path:
    sys.path.insert(0, str(_TUTORIAL_ROOT))
_INTRO_SCRIPT = _TUTORIAL_ROOT / "intro" / "intro_scene.py"
SERIES_INTRO_SCENE = "Demo_Intro_Lueftung"
#endregion

from manim_visuals import begin_vo_beat  # noqa: E402


#region Module Loader
def _load_module(module_name: str, path: Path) -> ModuleType:
    """📦 Load a scene file under a unique module name (avoids class-name clashes)."""
    spec = importlib.util.spec_from_file_location(module_name, path)
    if spec is None or spec.loader is None:
        raise ImportError(f"Cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[module_name] = module
    spec.loader.exec_module(module)
    return module
#endregion


#region Section Modules
_p1 = _load_module("ventilation_full_p1", _VENTILATION_ROOT / "1_warum_lueften" / "scene_1.py")
_p2 = _load_module("ventilation_full_p2", _VENTILATION_ROOT / "2_luftmenge" / "scene_2.py")
_p3 = _load_module("ventilation_full_p3", _VENTILATION_ROOT / "3_waermestrom_luft" / "scene_3.py")
_p4 = _load_module("ventilation_full_p4", _VENTILATION_ROOT / "4_natuerliche_lueftung" / "scene_4.py")
_p5 = _load_module("ventilation_full_p5", _VENTILATION_ROOT / "5_mechanische_lueftung" / "scene_5.py")
_p6 = _load_module("ventilation_full_p6", _VENTILATION_ROOT / "6_uebergabe" / "scene_6.py")
#endregion


#region Playlist
VENTILATION_PLAYLIST: list[tuple[str, list[type[Scene]]]] = [
    (
        "Kapitel 1 · Warum lüften?",
        [_p1.Beat1_Lueftungsbedarf, _p1.Beat2_Lueftungsweg, _p1.Beat3_Fahrplan],
    ),
    (
        "Kapitel 2 · Wie viel Luft?",
        [_p2.Beat1_Innenvolumen, _p2.Beat2_Luftwechselrate, _p2.Beat3_Volumenstrom],
    ),
    (
        "Kapitel 3 · Was die Luft trägt",
        [_p3.Beat1_Luftwaerme, _p3.Beat2_SensibleWaerme, _p3.Beat3_LatenteLast, _p3.Beat4_WinterUndSommer],
    ),
    (
        "Kapitel 4 · Natürliche Lüftung",
        [
            _p4.Beat1_Fensterregeln,
            _p4.Beat2_StossOderKipp,
            _p4.Beat3_Querlueftung,
            _p4.Beat4_Auftrieb,
            _p4.Beat5_Nachtlueftung,
            _p4.Beat6_GrenzenDerFreienLueftung,
        ],
    ),
    (
        "Kapitel 5 · Mechanische Lüftung und Wärmerückgewinnung",
        [_p5.Beat1_Grundtypen, _p5.Beat2_Waermerueckgewinnung, _p5.Beat3_Anlagenvergleich, _p5.Beat4_Kanalquerschnitt],
    ),
    (
        "Kapitel 6 · Übergabe an Heizung und Kühlung",
        [_p6.Beat1_Zusammenfassung, _p6.Beat2_UebergabeHeizungKuehlung],
    ),
]
#endregion


#region Host Binding
def _bind_scene_attrs(host: Scene, scene_cls: type[Scene]) -> None:
    """🔗 Mirror beat class attrs/methods onto the host section scene.

    Section hosts call ``scene_cls.construct(self)``, so class constants
    (e.g. ``SHAFT_X``) and helpers (e.g. ``_stack_paths``) must live on the host.
    Reset VO spend so shared clause keys (``intro`` / ``outro``) do not
    collapse later beats to ``min_wait``.
    """
    skip = {"construct"}
    for cls in scene_cls.__mro__:
        if cls is Scene:
            break
        for name, value in cls.__dict__.items():
            if name.startswith("__") or name in skip:
                continue
            if isinstance(value, staticmethod):
                setattr(host, name, value.__func__)
            elif isinstance(value, classmethod):
                setattr(host, name, value.__get__(host, type(host)))
            elif isinstance(value, types.FunctionType):
                setattr(host, name, types.MethodType(value, host))
            else:
                setattr(host, name, value)
    begin_vo_beat(host, scene_cls.__name__)
#endregion


#region Section Scenes
class _VentilationSection(Scene):
    """📚 One curriculum section — keeps Manim partial counts manageable."""

    section_beats: list[type[Scene]] = []

    def construct(self):
        for scene_cls in self.section_beats:
            _bind_scene_attrs(self, scene_cls)
            scene_cls.construct(self)
            self.clear()


class Ventilation_01_WarumLueften(_VentilationSection):
    """1️⃣ Kapitel 1 — Warum lüften?"""

    section_beats = VENTILATION_PLAYLIST[0][1]


class Ventilation_02_Luftmenge(_VentilationSection):
    """2️⃣ Kapitel 2 — Wie viel Luft?"""

    section_beats = VENTILATION_PLAYLIST[1][1]


class Ventilation_03_WaermestromLuft(_VentilationSection):
    """3️⃣ Kapitel 3 — Was die Luft trägt."""

    section_beats = VENTILATION_PLAYLIST[2][1]


class Ventilation_04_NatuerlicheLueftung(_VentilationSection):
    """4️⃣ Kapitel 4 — Natürliche Lüftung."""

    section_beats = VENTILATION_PLAYLIST[3][1]


class Ventilation_05_MechanischeLueftung(_VentilationSection):
    """5️⃣ Kapitel 5 — Mechanische Lüftung und Wärmerückgewinnung."""

    section_beats = VENTILATION_PLAYLIST[4][1]


class Ventilation_06_Uebergabe(_VentilationSection):
    """6️⃣ Kapitel 6 — Übergabe an Heizung und Kühlung."""

    section_beats = VENTILATION_PLAYLIST[5][1]


SECTION_SCENES: list[type[Scene]] = [
    Ventilation_01_WarumLueften,
    Ventilation_02_Luftmenge,
    Ventilation_03_WaermestromLuft,
    Ventilation_04_NatuerlicheLueftung,
    Ventilation_05_MechanischeLueftung,
    Ventilation_06_Uebergabe,
]
#endregion


#region Full Series Scene (fragile mega-combine)
class FullVentilationVideo(Scene):
    """🌬️ Entire Ventilation series in one Manim scene.

    Prefer running this file as a script (section renders + ffmpeg).
    Mega-combine can fail with ``InvalidDataError`` on long paths / many partials.
    """

    def construct(self):
        for _section, scenes in VENTILATION_PLAYLIST:
            for scene_cls in scenes:
                _bind_scene_attrs(self, scene_cls)
                scene_cls.construct(self)
                self.clear()
#endregion


#region Compose (recommended)
def _manim_bin() -> Path:
    """🛠️ Repo venv manim, else PATH manim."""
    candidate = _SEMIO_ROOT / ".venv" / "bin" / "manim"
    if candidate.is_file():
        return candidate
    which = shutil.which("manim")
    if which:
        return Path(which)
    raise FileNotFoundError("manim not found — activate .venv or install Manim")


def _quality_folder(quality_flag: str) -> str:
    """📁 Manim media subfolder for -ql / -qm / -qh."""
    return {"-ql": "480p15", "-qm": "720p30", "-qh": "1080p60"}.get(quality_flag, "480p15")


def _find_named_mp4(media_dir: Path, scene_name: str, quality_flag: str) -> Path:
    """🔎 Locate any rendered mp4 named ``scene_name`` under media_dir."""
    folder = _quality_folder(quality_flag)
    preferred = [
        media_dir / "videos" / "full_ventilation_video" / folder / f"{scene_name}.mp4",
        media_dir / "videos" / "intro_scene" / folder / f"{scene_name}.mp4",
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


def _find_section_mp4(media_dir: Path, scene_name: str, quality_flag: str) -> Path:
    """🔎 Locate a rendered section mp4 under media_dir."""
    return _find_named_mp4(media_dir, scene_name, quality_flag)


_SHARED_SOURCES: tuple[Path, ...] = (
    Path(__file__).resolve(),
    _TUTORIAL_ROOT / "manim_visuals.py",
    _TUTORIAL_ROOT / "manim_fonts.py",
)


def _section_sources(scene_cls: type[Scene]) -> list[Path]:
    """📚 Files a section clip is built from: its beat modules, their VO timings and the shared helpers."""
    folders = {Path(sys.modules[beat.__module__].__file__).resolve().parent for beat in scene_cls.section_beats}
    files = [p for folder in folders for p in (*folder.glob("scene_*.py"), folder / "vo_timing.json")]
    return [p for p in (*files, *_SHARED_SOURCES) if p.is_file()]


def _is_current(clip: Path, sources: list[Path]) -> bool:
    """⏱️ A clip is reusable only when it is newer than every source it was rendered from."""
    built = clip.stat().st_mtime
    return all(src.stat().st_mtime <= built for src in sources)


def _ffmpeg_concat(clips: list[Path], output: Path, list_path: Path) -> None:
    """🎞️ Concatenate mp4 clips with stream copy (safe paths, no Manim URI bug)."""
    lines = []
    for clip in clips:
        escaped = str(clip.resolve()).replace("'", r"'\''")
        lines.append(f"file '{escaped}'")
    list_path.parent.mkdir(parents=True, exist_ok=True)
    list_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    output.parent.mkdir(parents=True, exist_ok=True)
    cmd = [
        "ffmpeg", "-y",
        "-f", "concat", "-safe", "0",
        "-i", str(list_path),
        "-c:v", "copy", "-an",
        str(output),
    ]
    subprocess.run(cmd, check=True)


def compose_full_ventilation_video(
    *,
    quality_flag: str = "-ql",
    play: bool = True,
    media_dir: Path | None = None,
    force: bool = False,
    concat_only: bool = False,
) -> Path:
    """🎬 Render series intro + each section with Manim, then merge into one series mp4."""
    media_dir = media_dir or (_VENTILATION_ROOT / "media")
    manim = _manim_bin()
    clips: list[Path] = []

    #region Series Intro
    intro_existing: Path | None = None
    if not force:
        try:
            intro_existing = _find_named_mp4(media_dir, SERIES_INTRO_SCENE, quality_flag)
        except FileNotFoundError:
            intro_existing = None

    if concat_only:
        if intro_existing is None:
            raise FileNotFoundError(
                f"Missing intro mp4 for {SERIES_INTRO_SCENE} (cannot --concat-only).",
            )
        print(f"\n=== Reusing intro {SERIES_INTRO_SCENE} → {intro_existing.name} ===")
        clips.append(intro_existing)
    elif intro_existing is not None and _is_current(intro_existing, [_INTRO_SCRIPT, *_SHARED_SOURCES[1:]]):
        print(f"\n=== Skipping intro {SERIES_INTRO_SCENE} (up to date) ===")
        clips.append(intro_existing)
    else:
        print(f"\n=== Rendering intro {SERIES_INTRO_SCENE} ===")
        subprocess.run(
            [
                str(manim),
                quality_flag,
                "--media_dir", str(media_dir),
                str(_INTRO_SCRIPT),
                SERIES_INTRO_SCENE,
            ],
            check=True,
            cwd=str(_SEMIO_ROOT),
        )
        clips.append(_find_named_mp4(media_dir, SERIES_INTRO_SCENE, quality_flag))
    #endregion

    for scene_cls in SECTION_SCENES:
        name = scene_cls.__name__
        existing: Path | None = None
        if not force:
            try:
                existing = _find_section_mp4(media_dir, name, quality_flag)
            except FileNotFoundError:
                existing = None

        if concat_only:
            if existing is None:
                raise FileNotFoundError(
                    f"Missing section mp4 for {name} (cannot --concat-only).",
                )
            print(f"\n=== Reusing {name} → {existing.name} ===")
            clips.append(existing)
            continue

        if existing is not None and _is_current(existing, _section_sources(scene_cls)):
            print(f"\n=== Skipping {name} (up to date) ===")
            clips.append(existing)
            continue

        print(f"\n=== Rendering {name} ===")
        cmd = [
            str(manim),
            quality_flag,
            "--media_dir", str(media_dir),
            str(Path(__file__).resolve()),
            name,
        ]
        subprocess.run(cmd, check=True, cwd=str(_SEMIO_ROOT))
        clips.append(_find_section_mp4(media_dir, name, quality_flag))

    folder = _quality_folder(quality_flag)
    out_dir = media_dir / "videos" / "full_ventilation_video" / folder
    output = out_dir / "FullVentilationVideo.mp4"
    list_path = out_dir / "section_concat_list.txt"
    rendered_copy = _VENTILATION_ROOT / "rendered" / f"Full_Ventilation_{folder}.mp4"
    noaudio_copy = _VENTILATION_ROOT / "rendered" / f"Full_Ventilation_NoAudio_{folder}.mp4"
    print(f"\n=== Merging {len(clips)} silent clips (intro + sections) → {output.name} ===")
    _ffmpeg_concat(clips, output, list_path)
    rendered_copy.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(output, rendered_copy)
    shutil.copy2(output, noaudio_copy)
    print(f"\n✅ Single full video: {output}")
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
    """▶️ CLI: series intro + section renders + ffmpeg merge into one full-series mp4."""
    parser = argparse.ArgumentParser(
        description=(
            "Render Ventilation intro + sections and merge them into one FullVentilationVideo.mp4."
        ),
    )
    parser.add_argument(
        "-q",
        choices=("l", "m", "h"),
        default="l",
        help="Quality: l=low (default), m=medium, h=high",
    )
    parser.add_argument(
        "--no-play",
        action="store_true",
        help="Do not open the finished mp4",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="Re-render intro and every section even if mp4s already exist",
    )
    parser.add_argument(
        "--concat-only",
        action="store_true",
        help="Only merge existing intro + section mp4s (no Manim render)",
    )
    args = parser.parse_args(argv)
    quality_flag = {"l": "-ql", "m": "-qm", "h": "-qh"}[args.q]
    compose_full_ventilation_video(
        quality_flag=quality_flag,
        play=not args.no_play,
        force=args.force,
        concat_only=args.concat_only,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
#endregion
