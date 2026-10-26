"""🔎 Independently verify every authored segment, chapter and complete teaching movie."""
import argparse
from fractions import Fraction
import hashlib
import json
from pathlib import Path
import subprocess


def parse_args(argv=None):
    """📐 Select an explicit format profile while retaining the original defaults."""
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    parser.add_argument("--ffmpeg", default="ffmpeg")
    parser.add_argument("--ffprobe", default="ffprobe")
    parser.add_argument("--report", required=True, type=Path)
    parser.add_argument("--teaching-root", required=True, type=Path)
    parser.add_argument("--width", type=int, default=854)
    parser.add_argument("--height", type=int, default=480)
    parser.add_argument("--fps", type=float, default=15)
    args = parser.parse_args(argv)
    if args.width <= 0 or args.height <= 0 or args.width % 2 or args.height % 2 or args.fps <= 0:
        parser.error("Expected positive even width/height and positive fps")
    return args


def movie_suffix(args):
    """🏷️ Match the renderer explicit-resolution filename convention."""
    dimensions = str(args.height) if (args.width, args.height) == (854, 480) else f"{args.width}x{args.height}"
    return f"{dimensions}p{args.fps:g}"


def main():
    """🎞️ Compare source-plan coverage, container totals and two complete decoders."""
    args = parse_args()
    import av
    root = args.root.resolve()
    teaching = args.teaching_root.resolve()
    videos = json.loads((teaching / "🏛️architecture/⚡️energy/🎬️clip/🔣️.json").read_text(encoding="utf-8"))["videos"]
    assert {video["id"] for video in videos} == {"physical_fundamentals", "ventilation", "heating", "cooling", "energy_balance"}
    assert [len(video["chapters"]) for video in videos] == [1, 6, 6, 6, 7]
    assert sum(len(chapter["segments"]) for video in videos for chapter in video["chapters"]) == 112
    expected_movies = {video["output"] + f"_{movie_suffix(args)}.mp4" for video in videos}
    movies = sorted((root / "rendered").glob("*.mp4"))
    assert len(movies) == 5 and {movie.name for movie in movies} == expected_movies, movies
    font_path = teaching / "🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py"
    font_hash = hashlib.sha256(font_path.read_bytes()).hexdigest()
    metadata = {}
    ffmpeg_decoded = set()

    def decode_ffmpeg(path):
        """🎞️ Independently decode every raw, logical and final artifact without tolerated errors."""
        if path not in ffmpeg_decoded:
            subprocess.run([args.ffmpeg, "-v", "error", "-xerror", "-i", str(path), "-map", "0:v:0", "-f", "null", "-"], check=True)
            ffmpeg_decoded.add(path)

    def probe(path, count=False):
        """📐 Read a declared clip format and frame total through independent FFprobe."""
        key = (path, count)
        if key not in metadata:
            command = [args.ffprobe, "-v", "error"] + (["-count_frames"] if count else [])
            data = json.loads(subprocess.check_output([*command, "-show_streams", "-show_format", "-of", "json", str(path)], text=True, encoding="utf-8"))
            streams = [stream for stream in data["streams"] if stream["codec_type"] == "video"]
            assert len(streams) == 1, (path, streams)
            stream = streams[0]
            assert (stream["width"], stream["height"]) == (args.width, args.height), (path, stream)
            assert Fraction(stream["r_frame_rate"]) == Fraction(str(args.fps)), (path, stream)
            frames = int(stream["nb_read_frames"] if count else stream["nb_frames"])
            seconds = float(data["format"]["duration"])
            assert frames > 0 and seconds > 0, path
            assert abs(frames / args.fps - seconds) < .2, (path, frames, seconds)
            metadata[key] = {"frames": frames, "duration": seconds, "audio_streams": sum(s["codec_type"] == "audio" for s in data["streams"])}
        return metadata[key]

    def unit(ordinal, descriptor, samples, offset):
        """🧬 Require one actual raw render and current visual-source provenance."""
        clip = root / "s" / str(ordinal) / "v" / (descriptor["scene"] + ".mp4")
        state = json.loads(clip.with_suffix(".render.json").read_text(encoding="utf-8"))
        assert state["sources"].get(font_path.as_posix()) == font_hash, (clip, "Incorrect visual font provenance")
        source = (teaching / descriptor["script"]).resolve()
        required = [font_path, teaching / "🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py", source]
        timing = source.parent / "⏱️timing" / "🔣️.json"
        if timing.is_file():
            required.append(timing)
        if source == teaching / "🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py":
            required.append(source.parent / "🏰️institution" / "📕️.pdf")
        for dependency in required:
            assert state["sources"].get(dependency.as_posix()) == hashlib.sha256(dependency.read_bytes()).hexdigest(), (clip, dependency, "Missing or stale required dependency")
        for recorded, digest in state["sources"].items():
            dependency = Path(recorded).resolve()
            assert dependency.is_relative_to(teaching), (clip, dependency, "Source outside teaching root")
            assert hashlib.sha256(dependency.read_bytes()).hexdigest() == digest, (clip, dependency, "Stale recorded dependency")
        assert state["settings"]["quality"] == "-ql", (clip, state["settings"])
        assert state["settings"]["resolution"] in ([None, f"{args.width},{args.height}"] if (args.width, args.height) == (854, 480) else [f"{args.width},{args.height}"]), (clip, state["settings"])
        assert state["settings"]["fps"] in ([None, args.fps] if args.fps == 15 else [args.fps]), (clip, state["settings"])
        identity = state["segment"]
        assert identity["scene"] == descriptor["scene"], (clip, "Incorrect scene identity")
        recorded_source = Path(identity["script"])
        if not recorded_source.is_absolute():
            recorded_source = teaching / recorded_source
        assert recorded_source.resolve() == source, (clip, "Incorrect script identity")
        expected_segments.add(clip)
        result = probe(clip)
        sample_indices = {0, result["frames"] // 2, result["frames"] - 1}
        decoded = 0
        last_raw_pts = None
        with av.open(str(clip)) as container:
            for number, frame in enumerate(container.decode(video=0)):
                assert frame.pts is not None and (last_raw_pts is None or frame.pts > last_raw_pts), (clip, number, "Invalid raw presentation timestamps")
                last_raw_pts = frame.pts
                if number in sample_indices:
                    samples[offset + number] = hashlib.sha256(frame.to_ndarray(format="yuv420p").tobytes()).hexdigest()
                decoded += 1
        assert decoded == result["frames"], (clip, "Raw decoded frame total differs")
        decode_ffmpeg(clip)
        result["state"] = state
        return result

    def decode(path, result, samples):
        """🎨 Compare every decoded frame count and ordered beat boundary samples."""
        count = 0
        first = last = None
        with av.open(str(path)) as container:
            for frame in container.decode(video=0):
                assert frame.pts is not None, (path, count, "Missing presentation timestamp")
                if first is None:
                    first = frame.pts
                if last is not None:
                    assert frame.pts > last, (path, count, "Non-increasing presentation timestamps")
                last = frame.pts
                if count in samples:
                    actual_hash = hashlib.sha256(frame.to_ndarray(format="yuv420p").tobytes()).hexdigest()
                    assert actual_hash == samples[count], (path, count, "Segment frames were reordered or altered")
                count += 1
        assert count == result["frames"] and first is not None and last > first, (path, count, result)
        decode_ffmpeg(path)
        return count

    expected_segments = set()
    expected_logical = set()
    rows = []
    ordinal = 0
    for index, video in enumerate(videos, 1):
        ordinal += 1
        intro = video["intro"]
        samples = {}
        raw_intro = unit(ordinal, intro, samples, 0)
        intro_clip = root / str(index) / "v" / (intro["scene"] + ".mp4")
        assert probe(intro_clip)["frames"] == raw_intro["frames"], intro_clip
        intro_state = json.loads(intro_clip.with_suffix(".render.json").read_text(encoding="utf-8"))
        assert intro_state["scene"] == intro["scene"] and intro_state["segments"] == [raw_intro["state"]] and intro_state["settings"] == raw_intro["state"]["settings"], intro_clip
        decode(intro_clip, probe(intro_clip), samples)
        logical_states = [intro_state]
        clips = [intro_clip]
        segment_frames = raw_intro["frames"]
        beat_count = 0
        for chapter in video["chapters"]:
            raw = []
            chapter_start = segment_frames
            for segment in chapter["segments"]:
                ordinal += 1
                item = unit(ordinal, segment, samples, segment_frames + sum(item["frames"] for item in raw))
                raw.append(item)
            chapter_clip = root / str(index) / "v" / (chapter["scene"] + ".mp4")
            compiled = probe(chapter_clip)
            assert compiled["frames"] == sum(item["frames"] for item in raw), (chapter_clip, "Missing or duplicated beat frames")
            assert abs(compiled["duration"] - sum(item["duration"] for item in raw)) < .2, (chapter_clip, "Segment timing differs")
            chapter_state = json.loads(chapter_clip.with_suffix(".render.json").read_text(encoding="utf-8"))
            assert chapter_state["scene"] == chapter["scene"] and chapter_state["segments"] == [item["state"] for item in raw] and all(item["state"]["settings"] == chapter_state["settings"] for item in raw), chapter_clip
            local_samples = {position - chapter_start: digest for position, digest in samples.items() if chapter_start <= position < chapter_start + compiled["frames"]}
            decode(chapter_clip, compiled, local_samples)
            logical_states.append(chapter_state)
            segment_frames += compiled["frames"]
            beat_count += len(raw)
            clips.append(chapter_clip)
        expected_logical.update(clips)
        movie = root / "rendered" / (video["output"] + f"_{movie_suffix(args)}.mp4")
        result = probe(movie, count=True)
        assert result["audio_streams"] == 0, (movie, "Canonical movie should be silent")
        assert result["frames"] == segment_frames == sum(probe(clip)["frames"] for clip in clips), (movie, "Movie frame total differs from authored segments")
        clip_seconds = sum(probe(clip)["duration"] for clip in clips)
        assert abs(result["duration"] - clip_seconds) < .2, (movie, "Chapter timing differs")
        movie_state = json.loads(movie.with_suffix(".render.json").read_text(encoding="utf-8"))
        assert movie_state["video"] == video["id"] and movie_state["chapters"] == logical_states and all(state["settings"] == movie_state["settings"] for state in logical_states), movie
        count = decode(movie, result, samples)
        rows.append({"id": video["id"], "file": str(movie), "frames": count, "duration": result["duration"], "width": args.width, "height": args.height, "fps": args.fps, "chapters": len(video["chapters"]), "beats": beat_count, "clips": len(clips), "source_frames": segment_frames, "source_seconds": clip_seconds, "pyav_frames": count, "timestamps_monotonic": True, "ffmpeg_decode": "pass", "ffmpeg_artifacts": beat_count + len(clips) + 2, "raw_segments": beat_count + 1, "font_sha256": font_hash, "ordered_frame_samples": len(samples)})
        print("[DEBUG] Verified", movie.name, count, "frames", round(result["duration"], 3), "seconds", flush=True)
    assert ordinal == len(expected_segments) == 117
    assert len(expected_logical) == 31
    assert set(root.glob("s/*/v/*.mp4")) == expected_segments, "Unexpected or missing raw segment files"
    assert set(root.glob("[1-5]/v/*.mp4")) == expected_logical, "Unexpected or missing logical clips"
    assert len(ffmpeg_decoded) == 117 + 31 + 5, ("Incomplete independent FFmpeg coverage", len(ffmpeg_decoded))
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(rows, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
