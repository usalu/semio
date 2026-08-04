#!/usr/bin/env python3
"""🎞️ Export Cooling section HQ silent videos + separate Seraphina voices.

Writes into ``tutorial/energy/demand/Cooling/hq_export/``:
  videos/   — one silent 1080p60 full-section MP4 per part
  voices/   — one continuous WAV per part (+ per-beat copies under voices/beats/)
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

COOLING = Path(__file__).resolve().parents[1] if False else Path(
    "/Users/niloufarghandehariyoon/Documents/Master LUH/Hiwi/semio/tutorial/energy/demand/Cooling"
)
# Prefer running from Cooling; fall back to absolute above when invoked from ticket.
if (Path.cwd() / "1_heating_vs_cooling").is_dir():
    COOLING = Path.cwd()
elif (Path(__file__).resolve().parent.parent / "1_heating_vs_cooling").is_dir():
    COOLING = Path(__file__).resolve().parent.parent

SEMIO = COOLING.parents[3]
MANIM = SEMIO / ".venv" / "bin" / "manim"
EXPORT = COOLING / "hq_export"
VIDEOS = EXPORT / "videos"
VOICES = EXPORT / "voices"
BEATS_VOICE = VOICES / "beats"
TICKET_AUDIO = SEMIO / ".repo/🎫/26/08/04/CREATE-SYSTEMAUSLEGUNG-COOLING-VIDEO"

SECTIONS: list[dict] = [
    {
        "slug": "01_heating_vs_cooling",
        "title": "Heizlast vs. Kühllast",
        "dir": "1_heating_vs_cooling",
        "script": "heating_vs_cooling.py",
        "module_stem": "heating_vs_cooling",
        "scenes": ["HeatingVsCooling"],
        "audio_job": "heating-vs-cooling-audio",
        "local_full_audio": "HeatingVsCooling_audio.wav",
    },
    {
        "slug": "02_internal_gains",
        "title": "Interne Wärmegewinne",
        "dir": "2_internal_gains",
        "script": "merged_scenes.py",
        "module_stem": "merged_scenes",
        "scenes": [
            "Scene1",
            "Scene2",
            "Scene3",
            "EquipmentAndPlugLoads",
            "ArtificialLightingScene",
            "InternalGainEquation",
            "SensibleVsLatentHeat",
            "Scene8",
            "Scene9",
            "Scene10",
        ],
        "audio_job": "internal-gains-audio",
    },
    {
        "slug": "03_transmission_humidity",
        "title": "Transmission & Feuchte",
        "dir": "3_transmission_humidity",
        "script": "scene_3.py",
        "module_stem": "scene_3",
        "scenes": [
            "Beat1_TransmissionOpaque",
            "Beat2_TimeLag",
            "Beat3_VentilationHeat",
            "Beat4_SensibleVsLatent",
        ],
        "audio_job": "transmission-humidity-audio",
    },
    {
        "slug": "04_solar_radiation",
        "title": "Solare Strahlung",
        "dir": "4_solar_radiation",
        "script": "scene_4.py",
        "module_stem": "scene_4",
        "scenes": [
            "Beat1_SolarIrradiance",
            "Beat2_FrameFactor",
            "Beat3_ShadingFactor",
            "Beat4_GlassTransmittance",
            "Beat5_SolarCoolingLoad",
        ],
        "audio_job": "solar-radiation-audio",
    },
    {
        "slug": "05_systemauslegung",
        "title": "Systemauslegung",
        "dir": "5_systemauslegung",
        "script": "scene_5.py",
        "module_stem": "scene_5",
        "scenes": [
            "Beat1_MechanicalVentilation",
            "Beat2_VolumeFlowEquation",
            "Beat3_IsolateAirflow",
            "Beat4_DuctCrossSection",
            "Beat5_CalculateRadius",
        ],
        "audio_job": "systemauslegung-audio",
    },
    {
        "slug": "06_lueftungssysteme",
        "title": "Lüftungssysteme",
        "dir": "6_lueftungssysteme",
        "script": "scene_6.py",
        "module_stem": "scene_6",
        "scenes": [
            "Beat1_Systemuebersicht",
            "Beat2_FreieLueftung",
            "Beat3_MechanischeGrundtypen",
            "Beat4_Waermerueckgewinnung",
            "Beat5_Luftfuehrung",
            "Beat6_RLTFunktionen",
        ],
        "audio_job": "lueftungssysteme-audio",
    },
]


def run(cmd: list[str], cwd: Path | None = None) -> None:
    print("+", " ".join(cmd))
    res = subprocess.run(cmd, cwd=str(cwd) if cwd else None, text=True, capture_output=True)
    if res.returncode != 0:
        sys.stderr.write(res.stderr[-4000:] if res.stderr else res.stdout[-4000:])
        raise RuntimeError(f"command failed ({res.returncode}): {cmd[0]}")


def find_hq_clip(section_dir: Path, module_stem: str, scene: str) -> Path | None:
    candidates = [
        section_dir / "media" / "videos" / module_stem / "1080p60" / f"{scene}.mp4",
        section_dir / "rendered" / "media" / "videos" / module_stem / "1080p60" / f"{scene}.mp4",
    ]
    for path in candidates:
        if path.is_file() and path.stat().st_size > 10_000:
            return path
    hits = list(section_dir.rglob(f"1080p60/{scene}.mp4"))
    hits = [h for h in hits if "partial_movie_files" not in str(h)]
    return hits[0] if hits else None


def render_scene(section_dir: Path, script: str, module_stem: str, scene: str) -> Path:
    existing = find_hq_clip(section_dir, module_stem, scene)
    # Always re-render to guarantee current code + silent HQ (user asked for highest quality).
    media_dir = section_dir / "media"
    run(
        [
            str(MANIM),
            "render",
            "-qh",
            "--format",
            "mp4",
            "--media_dir",
            str(media_dir),
            str(section_dir / script),
            scene,
        ],
        cwd=section_dir,
    )
    out = find_hq_clip(section_dir, module_stem, scene)
    if out is None:
        raise FileNotFoundError(f"missing HQ render for {section_dir.name}/{scene}")
    return out


def concat_silent(clips: list[Path], out: Path) -> None:
    out.parent.mkdir(parents=True, exist_ok=True)
    list_file = out.with_suffix(".txt")
    list_file.write_text(
        "".join(f"file '{c.resolve()}'\n" for c in clips),
        encoding="utf-8",
    )
    # Drop any accidental audio track; keep video stream only.
    run(
        [
            "ffmpeg",
            "-y",
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            str(list_file),
            "-an",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "-crf",
            "18",
            "-preset",
            "slow",
            str(out),
        ]
    )
    list_file.unlink(missing_ok=True)


def concat_wavs(parts: list[Path], out: Path) -> None:
    out.parent.mkdir(parents=True, exist_ok=True)
    if len(parts) == 1:
        out.write_bytes(parts[0].read_bytes())
        return
    list_file = out.with_suffix(".txt")
    list_file.write_text(
        "".join(f"file '{p.resolve()}'\n" for p in parts),
        encoding="utf-8",
    )
    run(
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
            str(out),
        ]
    )
    list_file.unlink(missing_ok=True)


def collect_voices(section: dict) -> None:
    slug = section["slug"]
    beat_dir = BEATS_VOICE / slug
    beat_dir.mkdir(parents=True, exist_ok=True)
    job = TICKET_AUDIO / section["audio_job"] / "audio_work" / "scenes"
    wavs: list[Path] = []
    if job.is_dir():
        for i, scene_dir in enumerate(sorted(job.glob("scene_*")), start=1):
            src = scene_dir / "audio.wav"
            if not src.is_file():
                continue
            dst = beat_dir / f"beat_{i:02d}.wav"
            dst.write_bytes(src.read_bytes())
            wavs.append(dst)
    section_dir = COOLING / section["dir"]
    if not wavs:
        local = section.get("local_full_audio")
        if local and (section_dir / local).is_file():
            full = VOICES / f"{slug}.wav"
            full.write_bytes((section_dir / local).read_bytes())
            print(f"  voice ← local {local}")
            return
        # Fall back to per-beat audio in section folder
        for i, path in enumerate(sorted(section_dir.glob("beat_*_audio.wav")), start=1):
            dst = beat_dir / f"beat_{i:02d}.wav"
            dst.write_bytes(path.read_bytes())
            wavs.append(dst)
    if not wavs:
        print(f"  ⚠ no voice found for {slug}")
        return
    concat_wavs(wavs, VOICES / f"{slug}.wav")
    print(f"  voice → {VOICES / f'{slug}.wav'} ({len(wavs)} beats)")


def export_section(section: dict, *, skip_render: bool = False) -> None:
    print(f"\n{'=' * 64}\n{section['slug']} — {section['title']}\n{'=' * 64}")
    section_dir = COOLING / section["dir"]
    clips: list[Path] = []
    for scene in section["scenes"]:
        if skip_render:
            clip = find_hq_clip(section_dir, section["module_stem"], scene)
            if clip is None:
                clip = render_scene(section_dir, section["script"], section["module_stem"], scene)
        else:
            clip = render_scene(section_dir, section["script"], section["module_stem"], scene)
        print(f"  clip {scene} → {clip}")
        clips.append(clip)
    out_video = VIDEOS / f"{section['slug']}.mp4"
    concat_silent(clips, out_video)
    print(f"  video → {out_video}")
    collect_voices(section)


def write_readme() -> None:
    lines = [
        "# Cooling HQ Export",
        "",
        "- `videos/` — silent 1080p60 section films (no audio track)",
        "- `voices/` — continuous German Seraphina WAV per section",
        "- `voices/beats/<section>/` — per-beat WAV copies",
        "",
        "| File | Section |",
        "|------|---------|",
    ]
    for s in SECTIONS:
        lines.append(f"| `{s['slug']}.mp4` / `.wav` | {s['title']} |")
    (EXPORT / "README.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> None:
    VIDEOS.mkdir(parents=True, exist_ok=True)
    VOICES.mkdir(parents=True, exist_ok=True)
    BEATS_VOICE.mkdir(parents=True, exist_ok=True)
    if not MANIM.is_file():
        raise SystemExit(f"manim not found: {MANIM}")
    skip = "--reuse-hq" in sys.argv
    only = None
    for arg in sys.argv[1:]:
        if arg.startswith("--only="):
            only = arg.split("=", 1)[1]
    write_readme()
    for section in SECTIONS:
        if only and only not in (section["slug"], section["dir"], section["slug"][:2]):
            continue
        export_section(section, skip_render=skip)
    print("\nDone →", EXPORT)


if __name__ == "__main__":
    main()
