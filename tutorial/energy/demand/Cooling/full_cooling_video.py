"""🎬 Full Cooling demand series — Parts 1–6 in curriculum order.

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
_COOLING_ROOT = Path(__file__).resolve().parent
_SEMIO_ROOT = next(
    p for p in _COOLING_ROOT.parents if (p / ".venv").is_dir() or (p / "package.json").is_file()
)
#endregion


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
_p1 = _load_module("cooling_full_p1", _COOLING_ROOT / "1_heating_vs_cooling" / "scene_1.py")
_p2 = _load_module("cooling_full_p2", _COOLING_ROOT / "2_internal_gains" / "scene_2.py")
_p3 = _load_module("cooling_full_p3", _COOLING_ROOT / "3_transmission_humidity" / "scene_3.py")
_p4 = _load_module("cooling_full_p4", _COOLING_ROOT / "4_solar_radiation" / "scene_4.py")
_p5 = _load_module("cooling_full_p5", _COOLING_ROOT / "5_systemauslegung" / "scene_5.py")
_p6 = _load_module("cooling_full_p6", _COOLING_ROOT / "6_lueftungssysteme" / "scene_6.py")
#endregion


#region Playlist
COOLING_PLAYLIST: list[tuple[str, list[type[Scene]]]] = [
    (
        "Teil 1 · Heizen vs. Kühlen",
        [
            _p1.Beat1_WinterGains,
            _p1.Beat2_SummerOverheat,
            _p1.Beat3_CoolingSystem,
        ],
    ),
    (
        "Teil 2 · Interne Wärmegewinne",
        [
            _p2.Beat1_OfficeRoom,
            _p2.Beat2_HumanFactor,
            _p2.Beat3_DevicesLighting,
            _p2.Beat4_CumulativeLoad,
            _p2.Beat8_Mitigation,
        ],
    ),
    (
        "Teil 3 · Transmission & Feuchte",
        [
            _p3.Beat1_TransmissionOpaque,
            _p3.Beat2_TimeLag,
            _p3.Beat3_VentilationHeat,
            _p3.Beat4_SensibleVsLatent,
        ],
    ),
    (
        "Teil 4 · Solarstrahlung",
        [
            _p4.Beat1_SolarIrradiance,
            _p4.Beat2_FrameFactor,
            _p4.Beat3_ShadingFactor,
            _p4.Beat4_GlassTransmittance,
            _p4.Beat5_SolarCoolingLoad,
        ],
    ),
    (
        "Teil 5 · Systemauslegung",
        [
            _p5.Beat1_MechanicalVentilation,
            _p5.Beat2_VolumeFlowEquation,
            _p5.Beat3_IsolateAirflow,
            _p5.Beat4_DuctCrossSection,
            _p5.Beat5_CalculateRadius,
        ],
    ),
    (
        "Teil 6 · Lüftungssysteme",
        [
            _p6.Beat1_PassivhausIdee,
            _p6.Beat2_Fensterregeln,
            _p6.Beat3_Querlueftung,
            _p6.Beat4_Auftrieb,
            _p6.Beat5_Nachtlueftung,
            _p6.Beat6_GrenzenDerFreienLueftung,
            _p6.Beat7_MechanischeGrundtypen,
            _p6.Beat8_Waermerueckgewinnung,
            _p6.Beat9_KomfortStrategie,
        ],
    ),
]
#endregion


#region Host Binding
def _bind_scene_attrs(host: Scene, scene_cls: type[Scene]) -> None:
    """🔗 Mirror beat class attrs/methods onto the host section scene.

    Section hosts call ``scene_cls.construct(self)``, so class constants
    (e.g. ``SHAFT_X``) and helpers (e.g. ``_stack_paths``) must live on the host.
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
#endregion


#region Section Scenes
class _CoolingSection(Scene):
    """📚 One curriculum section — keeps Manim partial counts manageable."""

    section_beats: list[type[Scene]] = []

    def construct(self):
        for scene_cls in self.section_beats:
            _bind_scene_attrs(self, scene_cls)
            scene_cls.construct(self)
            self.clear()


class Cooling_01_HeatingVsCooling(_CoolingSection):
    """1️⃣ Teil 1 — Heizen vs. Kühlen."""

    section_beats = COOLING_PLAYLIST[0][1]


class Cooling_02_InternalGains(_CoolingSection):
    """2️⃣ Teil 2 — Interne Wärmegewinne."""

    section_beats = COOLING_PLAYLIST[1][1]


class Cooling_03_TransmissionHumidity(_CoolingSection):
    """3️⃣ Teil 3 — Transmission & Feuchte."""

    section_beats = COOLING_PLAYLIST[2][1]


class Cooling_04_SolarRadiation(_CoolingSection):
    """4️⃣ Teil 4 — Solarstrahlung."""

    section_beats = COOLING_PLAYLIST[3][1]


class Cooling_05_Systemauslegung(_CoolingSection):
    """5️⃣ Teil 5 — Systemauslegung."""

    section_beats = COOLING_PLAYLIST[4][1]


class Cooling_06_Lueftungssysteme(_CoolingSection):
    """6️⃣ Teil 6 — Lüftungssysteme."""

    section_beats = COOLING_PLAYLIST[5][1]


SECTION_SCENES: list[type[Scene]] = [
    Cooling_01_HeatingVsCooling,
    Cooling_02_InternalGains,
    Cooling_03_TransmissionHumidity,
    Cooling_04_SolarRadiation,
    Cooling_05_Systemauslegung,
    Cooling_06_Lueftungssysteme,
]
#endregion


#region Full Series Scene (fragile mega-combine)
class FullCoolingDemandVideo(Scene):
    """❄️ Entire Cooling series in one Manim scene.

    Prefer running this file as a script (section renders + ffmpeg).
    Mega-combine can fail with ``InvalidDataError`` on long paths / many partials.
    """

    def construct(self):
        for _section, scenes in COOLING_PLAYLIST:
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


def _find_section_mp4(media_dir: Path, scene_name: str, quality_flag: str) -> Path:
    """🔎 Locate a rendered section mp4 under media_dir."""
    folder = _quality_folder(quality_flag)
    direct = media_dir / "videos" / "full_cooling_video" / folder / f"{scene_name}.mp4"
    if direct.is_file() and direct.stat().st_size > 1000:
        return direct
    matches = sorted(
        (media_dir / "videos").rglob(f"{scene_name}.mp4"),
        key=lambda p: p.stat().st_mtime,
        reverse=True,
    )
    for match in matches:
        if match.stat().st_size > 1000:
            return match
    raise FileNotFoundError(f"Rendered mp4 not found for {scene_name} under {media_dir}")


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
        "-c", "copy",
        str(output),
    ]
    subprocess.run(cmd, check=True)


def compose_full_cooling_video(
    *,
    quality_flag: str = "-ql",
    play: bool = True,
    media_dir: Path | None = None,
    force: bool = False,
    concat_only: bool = False,
) -> Path:
    """🎬 Render each section with Manim, then merge into one series mp4."""
    media_dir = media_dir or (_COOLING_ROOT / "media")
    manim = _manim_bin()
    clips: list[Path] = []

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

        if existing is not None and not force:
            print(f"\n=== Skipping {name} (already rendered) ===")
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
    output = media_dir / "videos" / "full_cooling_video" / folder / "FullCoolingDemandVideo.mp4"
    list_path = media_dir / "videos" / "full_cooling_video" / folder / "section_concat_list.txt"
    print(f"\n=== Merging {len(clips)} sections → {output.name} ===")
    _ffmpeg_concat(clips, output, list_path)
    print(f"\n✅ Single full video: {output}")

    if play:
        opener = {"darwin": "open", "win32": "start"}.get(sys.platform, "xdg-open")
        if sys.platform == "win32":
            subprocess.run(["cmd", "/c", "start", "", str(output)], check=False)
        else:
            subprocess.run([opener, str(output)], check=False)
    return output


def main(argv: list[str] | None = None) -> int:
    """▶️ CLI: section renders + ffmpeg merge into one full-series mp4."""
    parser = argparse.ArgumentParser(
        description=(
            "Render Cooling sections and merge them into one FullCoolingDemandVideo.mp4."
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
        help="Re-render every section even if mp4s already exist",
    )
    parser.add_argument(
        "--concat-only",
        action="store_true",
        help="Only merge existing section mp4s (no Manim render)",
    )
    args = parser.parse_args(argv)
    quality_flag = {"l": "-ql", "m": "-qm", "h": "-qh"}[args.q]
    compose_full_cooling_video(
        quality_flag=quality_flag,
        play=not args.no_play,
        force=args.force,
        concat_only=args.concat_only,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
#endregion
