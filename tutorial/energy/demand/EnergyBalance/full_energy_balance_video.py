"""🎬 Full EnergyBalance series — Chapters 1–7 in curriculum order.

Recommended: run this file as a script (chapter renders + ffmpeg concat).
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
_SERIES_ROOT = Path(__file__).resolve().parent
_SEMIO_ROOT = next(
    p for p in _SERIES_ROOT.parents if (p / ".venv").is_dir() or (p / "package.json").is_file()
)
_TUTORIAL_ROOT = _SEMIO_ROOT / "tutorial"
if str(_TUTORIAL_ROOT) not in sys.path:
    sys.path.insert(0, str(_TUTORIAL_ROOT))
_INTRO_SCRIPT = _TUTORIAL_ROOT / "intro" / "intro_scene.py"
SERIES_INTRO_SCENE = "Demo_Intro_Energiebilanz"
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


#region Chapter Modules
_c1 = _load_module("eb_full_c1", _SERIES_ROOT / "1_recap_heizen_kuehlen" / "scene_1.py")
_c2 = _load_module("eb_full_c2", _SERIES_ROOT / "2_regulatory_framework" / "scene_2.py")
_c3 = _load_module("eb_full_c3", _SERIES_ROOT / "3_bedarf_vs_leistung" / "scene_3.py")
_c4 = _load_module("eb_full_c4", _SERIES_ROOT / "4_system_losses" / "scene_4.py")
_c5 = _load_module("eb_full_c5", _SERIES_ROOT / "5_primary_energy" / "scene_5.py")
_c6 = _load_module("eb_full_c6", _SERIES_ROOT / "6_energieausweis" / "scene_6.py")
_c7 = _load_module("eb_full_c7", _SERIES_ROOT / "7_praxis_vs_theorie" / "scene_7.py")
#endregion


#region Playlist
ENERGY_BALANCE_PLAYLIST: list[tuple[str, list[type[Scene]]]] = [
    (
        "Kapitel 1 · Die Energiebilanz des Gebäudes",
        [
            _c1.Beat1_Bilanzgrenze,
            _c1.Beat2_Verlustseite,
            _c1.Beat3_Gewinnseite,
            _c1.Beat4_WinterUndSommer,
        ],
    ),
    (
        "Kapitel 2 · Regelwerk & Klimadaten",
        [
            _c2.Beat1_GEGundNormen,
            _c2.Beat2_Monatsbilanz,
            _c2.Beat3_Klimadaten,
            _c2.Beat4_Gradtagszahl,
        ],
    ),
    (
        "Kapitel 3 · Bedarf vs. Leistung",
        [
            _c3.Beat1_LeistungUndEnergie,
            _c3.Beat2_Normheizlast,
            _c3.Beat3_Jahresdauerlinie,
            _c3.Beat4_RichtigAuslegen,
        ],
    ),
    (
        "Kapitel 4 · Von der Nutzenergie zur Endenergie",
        [
            _c4.Beat1_Verlustkette,
            _c4.Beat2_UebergabeVerteilungSpeicherung,
            _c4.Beat3_Erzeugung,
            _c4.Beat4_Anlagenaufwandszahl,
        ],
    ),
    (
        "Kapitel 5 · Primärenergie & das GEG",
        [
            _c5.Beat1_DreiBilanzgrenzen,
            _c5.Beat2_Primaerenergiefaktor,
            _c5.Beat3_Rechenbeispiel,
            _c5.Beat4_Referenzgebaeude,
        ],
    ),
    (
        "Kapitel 6 · Der Energieausweis",
        [
            _c6.Beat1_BedarfVsVerbrauch,
            _c6.Beat2_DieSkala,
            _c6.Beat3_PflichtangabenUndFolgen,
        ],
    ),
    (
        "Kapitel 7 · Theorie vs. Praxis",
        [
            _c7.Beat1_BerechnungVsAbrechnung,
            _c7.Beat2_HydraulischerAbgleich,
            _c7.Beat3_SerienAbschluss,
        ],
    ),
]
#endregion


#region Host Binding
def _bind_scene_attrs(host: Scene, scene_cls: type[Scene]) -> None:
    """🔗 Mirror beat class attrs/methods onto the host chapter scene.

    Chapter hosts call ``scene_cls.construct(self)``, so class constants
    and helpers must live on the host. Reset VO spend so shared clause keys
    (``intro`` / ``outro``) do not collapse later beats to ``min_wait``.
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


#region Chapter Scenes
class _EnergyBalanceChapter(Scene):
    """📚 One curriculum chapter — keeps Manim partial counts manageable."""

    chapter_beats: list[type[Scene]] = []

    def construct(self):
        for scene_cls in self.chapter_beats:
            _bind_scene_attrs(self, scene_cls)
            scene_cls.construct(self)
            self.clear()


class EnergyBalance_01_Energiebilanz(_EnergyBalanceChapter):
    """1️⃣ Kapitel 1 — Die Energiebilanz des Gebäudes."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[0][1]


class EnergyBalance_02_RegelwerkKlima(_EnergyBalanceChapter):
    """2️⃣ Kapitel 2 — Regelwerk & Klimadaten."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[1][1]


class EnergyBalance_03_BedarfVsLeistung(_EnergyBalanceChapter):
    """3️⃣ Kapitel 3 — Bedarf vs. Leistung."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[2][1]


class EnergyBalance_04_NutzZuEndenergie(_EnergyBalanceChapter):
    """4️⃣ Kapitel 4 — Von der Nutzenergie zur Endenergie."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[3][1]


class EnergyBalance_05_Primaerenergie(_EnergyBalanceChapter):
    """5️⃣ Kapitel 5 — Primärenergie & das GEG."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[4][1]


class EnergyBalance_06_Energieausweis(_EnergyBalanceChapter):
    """6️⃣ Kapitel 6 — Der Energieausweis."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[5][1]


class EnergyBalance_07_TheorieVsPraxis(_EnergyBalanceChapter):
    """7️⃣ Kapitel 7 — Theorie vs. Praxis."""

    chapter_beats = ENERGY_BALANCE_PLAYLIST[6][1]


CHAPTER_SCENES: list[type[Scene]] = [
    EnergyBalance_01_Energiebilanz,
    EnergyBalance_02_RegelwerkKlima,
    EnergyBalance_03_BedarfVsLeistung,
    EnergyBalance_04_NutzZuEndenergie,
    EnergyBalance_05_Primaerenergie,
    EnergyBalance_06_Energieausweis,
    EnergyBalance_07_TheorieVsPraxis,
]
#endregion


#region Full Series Scene (fragile mega-combine)
class FullEnergyBalanceVideo(Scene):
    """📜 Entire EnergyBalance series in one Manim scene.

    Prefer running this file as a script (chapter renders + ffmpeg).
    Mega-combine can fail with ``InvalidDataError`` on long paths / many partials.
    """

    def construct(self):
        for _chapter, scenes in ENERGY_BALANCE_PLAYLIST:
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
        media_dir / "videos" / "full_energy_balance_video" / folder / f"{scene_name}.mp4",
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
        if match.stat().st_size > 1000:
            return match
    raise FileNotFoundError(f"Rendered mp4 not found for {scene_name} under {media_dir}")


def _find_chapter_mp4(media_dir: Path, scene_name: str, quality_flag: str) -> Path:
    """🔎 Locate a rendered chapter mp4 under media_dir."""
    return _find_named_mp4(media_dir, scene_name, quality_flag)


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


def compose_full_energy_balance_video(
    *,
    quality_flag: str = "-ql",
    play: bool = True,
    media_dir: Path | None = None,
    force: bool = False,
    concat_only: bool = False,
) -> Path:
    """🎬 Render series intro + each chapter with Manim, then merge into one series mp4."""
    media_dir = media_dir or (_SERIES_ROOT / "media")
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
    elif intro_existing is not None and not force:
        print(f"\n=== Skipping intro {SERIES_INTRO_SCENE} (already rendered) ===")
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

    for scene_cls in CHAPTER_SCENES:
        name = scene_cls.__name__
        existing: Path | None = None
        if not force:
            try:
                existing = _find_chapter_mp4(media_dir, name, quality_flag)
            except FileNotFoundError:
                existing = None

        if concat_only:
            if existing is None:
                raise FileNotFoundError(
                    f"Missing chapter mp4 for {name} (cannot --concat-only).",
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
        clips.append(_find_chapter_mp4(media_dir, name, quality_flag))

    folder = _quality_folder(quality_flag)
    out_dir = media_dir / "videos" / "full_energy_balance_video" / folder
    output = out_dir / "FullEnergyBalanceVideo.mp4"
    list_path = out_dir / "chapter_concat_list.txt"
    rendered_copy = _SERIES_ROOT / "rendered" / f"Full_EnergyBalance_{folder}.mp4"
    print(f"\n=== Merging {len(clips)} clips (intro + chapters) → {output.name} ===")
    _ffmpeg_concat(clips, output, list_path)
    rendered_copy.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(output, rendered_copy)
    print(f"\n✅ Single full video: {output}")
    print(f"✅ Copy:  {rendered_copy}")

    if play:
        opener = {"darwin": "open", "win32": "start"}.get(sys.platform, "xdg-open")
        if sys.platform == "win32":
            subprocess.run(["cmd", "/c", "start", "", str(rendered_copy)], check=False)
        else:
            subprocess.run([opener, str(rendered_copy)], check=False)
    return rendered_copy


def main(argv: list[str] | None = None) -> int:
    """▶️ CLI: series intro + chapter renders + ffmpeg merge into one full-series mp4."""
    parser = argparse.ArgumentParser(
        description=(
            "Render EnergyBalance intro + chapters and merge them into one FullEnergyBalanceVideo.mp4."
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
        help="Re-render intro and every chapter even if mp4s already exist",
    )
    parser.add_argument(
        "--concat-only",
        action="store_true",
        help="Only merge existing intro + chapter mp4s (no Manim render)",
    )
    args = parser.parse_args(argv)
    quality_flag = {"l": "-ql", "m": "-qm", "h": "-qh"}[args.q]
    compose_full_energy_balance_video(
        quality_flag=quality_flag,
        play=not args.no_play,
        force=args.force,
        concat_only=args.concat_only,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
#endregion
