"""🎬 Join all fresh course lesson masters into a chaptered stereo preview.

Usage: python course_preview.py <en|de> --work-root <ticket-generated-directory>
"""
import argparse
import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import lib
import chapter_titles
import av
import imageio_ffmpeg

FFMPEG = imageio_ffmpeg.get_ffmpeg_exe()


def join(parts, output: Path, work: Path, titles: dict):
    """🎞️ Normalize each ordered lesson and concatenate its video and stereo audio."""
    work.mkdir(parents=True, exist_ok=True)
    normalised = []
    for index, (name, record, source) in enumerate(parts):
        target = work / f"{index:02d}-{name}.mp4"
        with av.open(str(source)) as container:
            has_audio = bool(container.streams.audio)
            video = container.streams.video[0]
            duration = float(video.duration * video.time_base)
        command = [FFMPEG, "-y", "-hide_banner", "-loglevel", "error", "-i", str(source)]
        if not has_audio:
            command += ["-f", "lavfi", "-i", "anullsrc=channel_layout=stereo:sample_rate=48000"]
        command += ["-map", "0:v:0", "-map", "0:a:0" if has_audio else "1:a:0",
                    "-vf", "scale=854:480:force_original_aspect_ratio=decrease,pad=854:480:(ow-iw)/2:(oh-ih)/2,tpad=stop_mode=clone:stop_duration=0.1,fps=15,format=yuv420p",
                    "-af", "aresample=48000,apad", "-ac", "2", "-ar", "48000", "-c:a", "aac",
                    "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-g", "30", "-t", str(duration), str(target)]
        subprocess.run(command, check=True)
        with av.open(str(target)) as container:
            video = container.streams.video[0]
            seconds = float(video.duration * video.time_base)
        normalised.append((name, record, target, seconds))
        print("[DEBUG] normalised", name, round(seconds, 1), "s", flush=True)
    listing = work / "list.txt"
    listing.write_text("".join(f"file '{target.as_posix()}'\n" for _, _, target, _ in normalised), encoding="utf-8")
    chapters, start = [";FFMETADATA1\n"], 0.0
    for name, _, _, seconds in normalised:
        chapters.append(f"[CHAPTER]\nTIMEBASE=1/1000\nSTART={round(start * 1000)}\nEND={round((start + seconds) * 1000)}\ntitle={chapter_titles.ffmetadata(titles[name])}\n")
        start += seconds
    metadata = work / "chapters.txt"
    metadata.write_text("".join(chapters), encoding="utf-8")
    output.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run([FFMPEG, "-y", "-hide_banner", "-loglevel", "error", "-f", "concat", "-safe", "0", "-i", str(listing), "-i", str(metadata),
                    "-map", "0:v:0", "-map", "0:a:0", "-map_metadata", "1", "-map_chapters", "1", "-c", "copy", "-movflags", "+faststart", str(output)], check=True)
    return normalised


def current_parts(language: str, document: dict):
    """🔎 Verify the manifest records against current sources and master digests."""
    parts, missing, states = [], [], {}
    for video in lib.runtime().load_manifest():
        name = video["id"]
        record = document["videos"].get(name, {}).get("master", {})
        status = lib.video_status(name, language, "speaker", video["output"], states)
        source = lib.ROOT / record.get("file", "")
        if not record.get("fresh") or source != status["master"] or not lib.master_fresh(status) or not source.is_file() or lib.digest(source) != record.get("sha256"):
            missing.append(name)
        else:
            parts.append((name, record, source))
    if missing:
        raise SystemExit("stale or missing lesson masters: " + ", ".join(missing))
    return parts


def main(language: str, work_root: Path):
    """🌍 Publish only after every declared master remains fresh throughout the join."""
    if language not in {"en", "de"}:
        raise ValueError("language must be en or de")
    folder = lib.base(language, "speaker")
    document = json.loads((folder / "media-manifest.json").read_text(encoding="utf-8"))
    parts = current_parts(language, document)
    titles = chapter_titles.titles(lib.runtime().load_manifest(), language)
    preview = folder / "preview"
    preview.mkdir(parents=True, exist_ok=True)
    output = preview / f"Architecture_course_speaker_{language}_480p15.mp4"
    work_root.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="course-", dir=work_root) as temporary:
        partial = preview / f".{output.stem}.{Path(temporary).name}.partial.mp4"
        try:
            normalised = join(parts, partial, Path(temporary), titles)
            current_parts(language, document)
            if chapter_titles.titles(lib.runtime().load_manifest(), language) != titles:
                raise ValueError("localized lesson titles changed during join")
            partial.replace(output)
        finally:
            partial.unlink(missing_ok=True)
    made = lib.stills(output, preview, output.stem)
    lessons = [{"id": name, "title": titles[name], "master": record["file"], "media": record["media"], "sha256": record["sha256"], "seconds": round(seconds, 1),
                "casts": record["provenance"]["casts"]} for name, record, _, seconds in normalised]
    key = hashlib.sha256(lib.canonical({"sha256": lib.digest(output), "lessons": [lesson["media"] for lesson in lessons]}).encode()).hexdigest()
    result = {"language": language, "audience": "speaker", "size": [854, 480], "fps": 15, "seconds": round(lib.duration(output), 1),
              "sha256": lib.digest(output), "media": key, "lessons": lessons,
              "stills": {"poster": made["poster"].name, "thumbnail": made["thumbnail"].name}}
    output.with_suffix(".json").write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print("[DEBUG] course", output, result["seconds"], "s", len(lessons), "lessons", flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("language", choices=("en", "de"))
    parser.add_argument("--work-root", type=Path, required=True)
    args = parser.parse_args()
    main(args.language, args.work_root)
