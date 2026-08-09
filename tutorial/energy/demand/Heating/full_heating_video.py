"""🎬 Full Heating demand series — Modul 1–5 + final calculation.

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
from pathlib import Path
from types import ModuleType

from manim import Scene

# region Paths
_HEATING_ROOT = Path(__file__).resolve().parent
_SEMIO_ROOT = next(
    p for p in _HEATING_ROOT.parents if (p / ".venv").is_dir() or (p / "package.json").is_file()
)
# endregion


# region Module Loader
def _load_module(module_name: str, path: Path) -> ModuleType:
    """📦 Load a scene file under a unique module name (avoids class-name clashes)."""
    spec = importlib.util.spec_from_file_location(module_name, path)
    if spec is None or spec.loader is None:
        raise ImportError(f"Cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[module_name] = module
    spec.loader.exec_module(module)
    return module
# endregion


# region Section Modules
_m1 = _load_module("heating_full_m1", _HEATING_ROOT / "1_introduction" / "scene_1.py")
_m2 = _load_module("heating_full_m2", _HEATING_ROOT / "2_conduction" / "scene_2.py")
_m3 = _load_module("heating_full_m3", _HEATING_ROOT / "3_convection" / "scene_3.py")
_m4 = _load_module("heating_full_m4", _HEATING_ROOT / "4_internal_heat_gain" / "scene_4.py")
_m5 = _load_module("heating_full_m5", _HEATING_ROOT / "5_solar_heat_gain" / "scene_5.py")
_final = _load_module(
    "heating_full_final",
    _HEATING_ROOT / "final_calculation" / "merged_scenes.py",
)
# endregion


# region Playlist
HEATING_PLAYLIST: list[tuple[str, list[type[Scene]]]] = [
    (
        "Modul 1 · Einführung",
        [
            _m1.Beat1_DreiWegeDerWaerme,
            _m1.Beat2_Waermeleitung,
            _m1.Beat3_Konvektion,
            _m1.Beat4_Strahlung,
            _m1.Beat5_Zusammenfassung,
            _m1.Beat6_VonWegenZuZahlen,
            _m1.Beat7_Waermedurchlasswiderstand,
            _m1.Beat8_UWert,
            _m1.Beat9_WaermestromFormel,
        ],
    ),
    (
        "Modul 2 · Transmission / Leitung",
        [
            _m2.Beat1_MakroUndMikro,
            _m2.Beat2_RWert,
            _m2.Beat3_UWertUndGradient,
            _m2.Beat4_Gebaeudehuelle,
        ],
    ),
    (
        "Modul 3 · Konvektion / Lüftung",
        [
            _m3.Beat1_GebaeudeKonvektion,
            _m3.Beat2_Innenvolumen,
            _m3.Beat3_Luftwechselrate,
            _m3.Beat4_SpezWaermekapazitaet,
            _m3.Beat5_Lueftungsverlust,
            _m3.Beat6_Waermerueckgewinnung,
            _m3.Beat7_Lueftungssysteme,
        ],
    ),
    (
        "Modul 4 · Interne Wärmegewinne",
        [
            _m4.Beat1_WinterInterneGewinne,
            _m4.Beat2_PersonenPhiP,
            _m4.Beat3_GeraetePhiE,
            _m4.Beat4_BeleuchtungPhiL,
            _m4.Beat5_SummeUndDichte,
        ],
    ),
    (
        "Modul 5 · Solarer Wärmegewinn",
        [
            _m5.Beat1_VerlustZuGewinn,
            _m5.Beat2_BestrahlungUndFlaeche,
            _m5.Beat3_GWert,
            _m5.Beat4_SaisonaleWinkel,
            _m5.Beat5_Verschattung,
            _m5.Beat6_Waermespeicherung,
            _m5.Beat7_SpeichermasseFormel,
            _m5.Beat8_Hauptgleichung,
        ],
    ),
    (
        "Final Calculation · Heizwärmebedarf",
        [
            _final.ReviewingHeatLosses,
            _final.Scene2,
            _final.ReviewingHeatGains,
            _final.Scene4,
            _final.UltimateEnergyBalance,
        ],
    ),
]
# endregion


# region Host Binding
_BIND_ATTRS = (
    "NARRATION",
    "topic_de",
    "topic_explain_de",
    "series_de",
    "hold_seconds",
)


def _bind_scene_attrs(host: Scene, scene_cls: type[Scene]) -> None:
    """🔗 Copy scene class attrs onto the host (needed for ``scene_cls.construct(host)``)."""
    for name in _BIND_ATTRS:
        if hasattr(scene_cls, name):
            setattr(host, name, getattr(scene_cls, name))
# endregion


# region Section Scenes
class _HeatingSection(Scene):
    """📚 One curriculum section — keeps Manim partial counts manageable."""

    section_beats: list[type[Scene]] = []

    def construct(self):
        for scene_cls in self.section_beats:
            _bind_scene_attrs(self, scene_cls)
            scene_cls.construct(self)
            self.clear()


class Heating_01_Introduction(_HeatingSection):
    """1️⃣ Modul 1 — Einführung."""

    section_beats = HEATING_PLAYLIST[0][1]


class Heating_02_Conduction(_HeatingSection):
    """2️⃣ Modul 2 — Transmission / Leitung."""

    section_beats = HEATING_PLAYLIST[1][1]


class Heating_03_Convection(_HeatingSection):
    """3️⃣ Modul 3 — Konvektion / Lüftung."""

    section_beats = HEATING_PLAYLIST[2][1]


class Heating_04_InternalGains(_HeatingSection):
    """4️⃣ Modul 4 — Interne Wärmegewinne."""

    section_beats = HEATING_PLAYLIST[3][1]


class Heating_05_Solar(_HeatingSection):
    """5️⃣ Modul 5 — Solarer Wärmegewinn."""

    section_beats = HEATING_PLAYLIST[4][1]


class Heating_06_FinalCalculation(_HeatingSection):
    """6️⃣ Final calculation — Heizwärmebedarf."""

    section_beats = HEATING_PLAYLIST[5][1]


SECTION_SCENES: list[type[Scene]] = [
    Heating_01_Introduction,
    Heating_02_Conduction,
    Heating_03_Convection,
    Heating_04_InternalGains,
    Heating_05_Solar,
    Heating_06_FinalCalculation,
]
# endregion


# region Full Series Scene (fragile mega-combine)
class FullHeatingDemandVideo(Scene):
    """🔥 Entire series in one Manim scene.

    Prefer running this file as a script (section renders + ffmpeg).
    Mega-combine can fail with ``InvalidDataError`` on long paths / many partials.
    """

    def construct(self):
        for _section, scenes in HEATING_PLAYLIST:
            for scene_cls in scenes:
                _bind_scene_attrs(self, scene_cls)
                scene_cls.construct(self)
                self.clear()
# endregion


# region Compose (recommended)
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
    direct = media_dir / "videos" / "full_heating_video" / folder / f"{scene_name}.mp4"
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


def compose_full_heating_video(
    *,
    quality_flag: str = "-ql",
    play: bool = True,
    media_dir: Path | None = None,
) -> Path:
    """🎬 Render each section with Manim, ffmpeg-concat, optionally open the result."""
    media_dir = media_dir or (_HEATING_ROOT / "media")
    manim = _manim_bin()
    clips: list[Path] = []

    for scene_cls in SECTION_SCENES:
        name = scene_cls.__name__
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
    output = media_dir / "videos" / "full_heating_video" / folder / "FullHeatingDemandVideo.mp4"
    list_path = media_dir / "videos" / "full_heating_video" / folder / "section_concat_list.txt"
    print(f"\n=== Concatenating {len(clips)} sections → {output} ===")
    _ffmpeg_concat(clips, output, list_path)
    print(f"\n✅ Ready: {output}")

    if play:
        opener = {"darwin": "open", "win32": "start"}.get(sys.platform, "xdg-open")
        if sys.platform == "win32":
            subprocess.run(["cmd", "/c", "start", "", str(output)], check=False)
        else:
            subprocess.run([opener, str(output)], check=False)
    return output


def main(argv: list[str] | None = None) -> int:
    """▶️ CLI: section renders + ffmpeg concat (recommended full-series path)."""
    parser = argparse.ArgumentParser(
        description="Render the full Heating series (sections + ffmpeg concat).",
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
    args = parser.parse_args(argv)
    quality_flag = {"l": "-ql", "m": "-qm", "h": "-qh"}[args.q]
    compose_full_heating_video(quality_flag=quality_flag, play=not args.no_play)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
# endregion
