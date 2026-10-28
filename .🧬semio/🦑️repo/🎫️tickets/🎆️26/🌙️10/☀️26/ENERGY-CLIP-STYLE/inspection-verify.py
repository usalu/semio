"""🔎 Independently stream-decode and compare every complete inspection lesson."""
from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import math
import time
from pathlib import Path

import av
import numpy as np
from PIL import Image, ImageDraw

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
GENERATED = TICKET / "🗑️generated/iv"
MANIFEST = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json"
CATALOG = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🔣️.json"
FPS = 4
SIZE = (480, 270)
TRANSITION = max(3, round(3.6 * FPS))
INFO = {}


def sha(path):
    """🔐 Fingerprint exact bytes without trusting production certificates."""
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def frames(path):
    """🖼️ Decode RGB with bounded memory and one codec thread."""
    with av.open(str(path)) as media:
        stream = media.streams.video[0]
        stream.codec_context.thread_count = 1
        for frame in media.decode(stream):
            yield frame.to_ndarray(format="rgb24")


def info(path):
    """📽️ Decode all frames and verify every timestamp, dimension and frame interval."""
    key = str(path.resolve())
    initial = path.stat()
    signature = (initial.st_size, initial.st_mtime_ns)
    if key in INFO and INFO[key][0] == signature:
        return INFO[key][1]
    count, previous, first = 0, None, None
    with av.open(key) as media:
        assert len(media.streams.video) == 1, key
        stream = media.streams.video[0]
        stream.codec_context.thread_count = 1
        assert (stream.width, stream.height) == SIZE, key
        assert abs(float(stream.average_rate) - FPS) < 0.001, key
        for frame in media.decode(stream):
            stamp = float(frame.pts * frame.time_base)
            if first is None:
                first = stamp
            if previous is not None:
                assert abs(stamp - previous - 1 / FPS) < 0.002, (key, count, previous, stamp)
            count += 1
            previous = stamp
        assert count > 0, key
        value = {"path": key, "sha256": sha(path), "width": stream.width, "height": stream.height,
                 "fps": float(stream.average_rate), "frames": count, "duration": count / FPS,
                 "first_pts": first, "last_pts": previous, "audio_streams": len(media.streams.audio)}
    final = path.stat()
    assert (final.st_size, final.st_mtime_ns) == signature, (key, "changed during decode")
    INFO[key] = (signature, value)
    return value


def error(actual, expected):
    """📐 Compare every pixel and all four edges to expose body chrome or overlays."""
    assert actual.shape == expected.shape
    delta = np.abs(actual.astype(np.int16) - expected.astype(np.int16))
    edge = np.concatenate((delta[0].ravel(), delta[-1].ravel(), delta[:, 0].ravel(), delta[:, -1].ravel()))
    return [float(delta.mean()), float(edge.mean()), int(delta.max())]


def audio_info(path):
    """🔊 Inspect every decoded audio sample and its complete duration."""
    energy, samples, count, peak = 0.0, 0, 0, 0.0
    with av.open(str(path)) as media:
        assert len(media.streams.audio) == 1, str(path)
        stream = media.streams.audio[0]
        rate = stream.rate
        for frame in media.decode(stream):
            values = frame.to_ndarray()
            divisor = np.iinfo(values.dtype).max if np.issubdtype(values.dtype, np.integer) else 1
            values = values.astype(np.float64) / divisor
            energy += float(np.square(values).sum())
            samples += values.size
            count += frame.samples
            peak = max(peak, float(np.abs(values).max()))
    return {"rate": rate, "samples_per_channel": count, "seconds": count / rate,
            "rms": math.sqrt(energy / samples), "peak": peak}


def plan(videos, render):
    """🧩 Independently derive canonical ordinals and semantic excerpt filenames."""
    sources, ordinal, result = {}, 0, {}
    for video_index, video in enumerate(videos, 1):
        entries = []
        for chapter in video["chapters"]:
            for segment in chapter["segments"]:
                identity = (segment["script"], segment["scene"])
                sources.setdefault(identity, ordinal + 1)
                ordinal += 1
                source = render / "s" / str(sources[identity]) / "v" / (segment["scene"] + ".mp4")
                selected = source
                if "clauses" in segment:
                    digest = hashlib.sha256(json.dumps(segment["clauses"], sort_keys=True).encode()).hexdigest()[:10]
                    selected = source.with_name(f"{segment['scene']}.{digest}.mp4")
                entries.append({"chapter": chapter["scene"], "segment": segment, "source": source, "selected": selected})
        result[video["id"]] = (video_index, entries)
    return result


def cut_info(entry):
    """✂️ Reconstruct clause windows from measured source traces independently."""
    segment = entry["segment"]
    if "clauses" not in segment:
        return None
    trace = json.loads(entry["source"].with_suffix(".trace.json").read_text(encoding="utf-8"))
    marks = trace["marks"]
    keys = sorted(marks, key=lambda key: float(marks[key]["start"]))
    begin, end = (keys.index(segment["clauses"][key]) for key in ("first", "last"))
    assert begin <= end
    start = float(marks[keys[begin]]["start"])
    stop = float(marks[keys[end]]["start"]) + float(marks[keys[end]]["window"])
    certificate = json.loads(entry["selected"].with_suffix(".render.json").read_text(encoding="utf-8"))
    assert certificate["trace"] == sha(entry["source"].with_suffix(".trace.json"))
    assert np.allclose(certificate["interval"], [start, stop], atol=1e-8)
    count = info(entry["selected"])["frames"]
    assert abs(count - (stop - start) * FPS) <= 1.01, (segment, count, start, stop)
    assert 0 <= start < stop <= info(entry["source"])["duration"] + 1 / FPS
    estimate = max(0, math.ceil(start * FPS - 1e-7))
    lower, upper = max(0, estimate - 1), estimate + 1
    probe_count = min(8, count)
    selected_probe = list(itertools.islice(frames(entry["selected"]), probe_count))
    source_probe = list(itertools.islice(frames(entry["source"]), lower, upper + probe_count))
    candidates = []
    for first in range(lower, upper + 1):
        if first - lower + probe_count <= len(source_probe):
            mean = sum(error(actual, expected)[0] for actual, expected in zip(selected_probe, source_probe[first - lower:first - lower + probe_count])) / probe_count
            candidates.append((mean, abs(first - estimate), first))
    assert candidates, segment
    _, _, source_start = min(candidates)
    canonical = itertools.islice(frames(entry["source"]), source_start, source_start + count)
    maximum_mean, maximum_edge = 0.0, 0.0
    for index, (actual, expected) in enumerate(zip(frames(entry["selected"]), canonical, strict=True)):
        mean, edge, _ = error(actual, expected)
        maximum_mean, maximum_edge = max(maximum_mean, mean), max(maximum_edge, edge)
        assert mean < 4 and edge < 4, (segment, index, "excerpt canonical pixels", mean, edge)
    return {"clauses": segment["clauses"], "interval": [start, stop], "frames": count,
            "canonical_start_frame": source_start, "expected_start_frame": estimate,
            "source_max_mean_error": maximum_mean, "source_max_edge_error": maximum_edge}


def sheet(path, captures, tiles, columns=3):
    """🧾 Assemble labeled decoded frames with Pillow for independent visual review."""
    image = Image.new("RGB", (SIZE[0] * columns, (SIZE[1] + 26) * math.ceil(len(tiles) / columns)), "#222222")
    draw = ImageDraw.Draw(image)
    for ordinal, (index, label) in enumerate(tiles):
        x, y = ordinal % columns * SIZE[0], ordinal // columns * (SIZE[1] + 26)
        image.paste(Image.fromarray(captures[index]), (x, y))
        draw.text((x + 5, y + SIZE[1] + 5), f"{label} | f{index} ({index / FPS:.2f}s)", fill="white")
    image.save(path)


def verify(video, video_index, entries, render, output):
    """🧪 Verify complete coverage, body preservation, navigation endpoints and audio."""
    final = info(output)
    content = render / str(video_index) / "content.mp4"
    content_info = info(content)
    selected_info = [info(entry["selected"]) for entry in entries]
    canonical_info = [info(entry["source"]) for entry in entries]
    cuts = [cut_info(entry) for entry in entries]
    total = sum(value["frames"] for value in selected_info)
    assert content_info["frames"] == total, video["id"]
    assert final["frames"] == total + 2 * TRANSITION, video["id"]
    chapters = []
    for chapter in video["chapters"]:
        media = render / str(video_index) / "v" / (chapter["scene"] + ".mp4")
        expected = sum(value["frames"] for entry, value in zip(entries, selected_info) if entry["chapter"] == chapter["scene"])
        chapter_info = info(media)
        assert chapter_info["frames"] == expected
        chapters.append({"scene": chapter["scene"], "frames": expected, "sha256": chapter_info["sha256"]})
    boundaries = list(itertools.accumulate([0, *[value["frames"] for value in selected_info]]))
    snapshots = set(range(TRANSITION)) | set(range(TRANSITION + total, final["frames"]))
    tiles = [(0, "Opening: Topics"), (TRANSITION - 1, "Focus: authored content")]
    for entry, left, right in zip(entries, boundaries, boundaries[1:]):
        index = TRANSITION + (left + right - 1) // 2
        snapshots.add(index)
        tiles.append((index, entry["segment"]["scene"]))
    snapshots.update([TRANSITION, TRANSITION + total - 1])
    tiles += [(TRANSITION + total, "Restore: content"), (final["frames"] - 1, "Closing: Topics")]
    captures, source_error, body_error, edge_error, pixel_error = {}, 0.0, 0.0, 0.0, 0
    authored = itertools.chain.from_iterable(frames(entry["selected"]) for entry in entries)
    composed = frames(content)
    for index, actual in enumerate(frames(output)):
        if index in snapshots:
            captures[index] = actual.copy()
        if TRANSITION <= index < TRANSITION + total:
            source, body = next(authored), next(composed)
            selected_error, selected_edge, _ = error(body, source)
            body_mean, body_edge, body_pixel = error(actual, body)
            source_error = max(source_error, selected_error, selected_edge)
            body_error, edge_error = max(body_error, body_mean), max(edge_error, body_edge)
            pixel_error = max(pixel_error, body_pixel)
            assert selected_error < 0.01 and selected_edge < 0.01, (video["id"], index, "content ordering", selected_error, selected_edge)
            assert body_mean < 1 and body_edge < 1, (video["id"], index, "body preservation", body_mean, body_edge)
    assert next(authored, None) is None and next(composed, None) is None
    inward = error(captures[TRANSITION - 1], captures[TRANSITION])
    outward = error(captures[TRANSITION + total], captures[TRANSITION + total - 1])
    root = error(captures[0], captures[final["frames"] - 1])
    assert inward[0] < 2 and outward[0] < 2, (video["id"], "navigation seams", inward, outward)
    assert root[0] == 0, (video["id"], "shared root", root)
    audio = audio_info(output)
    has_audio = any(value["audio_streams"] for value in selected_info)
    assert abs(audio["seconds"] - final["duration"]) < 0.1
    if not has_audio:
        assert audio["peak"] < 1e-6, (video["id"], audio)
    directory = GENERATED / video["id"]
    directory.mkdir(exist_ok=True)
    sheet(directory / "coverage.png", captures, tiles)
    sheet(directory / "navigation.png", captures, [(offset, "Opening") for offset in range(TRANSITION)] + [(TRANSITION + total + offset, "Closing") for offset in range(TRANSITION)])
    Image.fromarray(captures[0]).save(directory / "opening.png")
    Image.fromarray(captures[final["frames"] - 1]).save(directory / "closing.png")
    return {"id": video["id"], **final, "pass": True, "source_frames": sum(value["frames"] for value in canonical_info),
            "selected_source_frames": total, "content_frames": content_info["frames"], "content_sha256": content_info["sha256"],
            "transition_frames_each": TRANSITION, "chapter_count": len(chapters), "segment_count": len(entries),
            "chapters": chapters, "cuts": [cut for cut in cuts if cut is not None], "audio": audio,
            "source_audio_streams": [value["audio_streams"] for value in selected_info],
            "source_content_max_mean_edge_error": source_error, "body_max_mean_error": body_error,
            "body_max_edge_error": edge_error, "body_max_channel_error": pixel_error,
            "opening_seam": inward, "closing_seam": outward, "shared_root_error": root,
            "root_rgb_sha256": hashlib.sha256(captures[0].tobytes()).hexdigest(),
            "coverage": str(directory / "coverage.png"), "navigation": str(directory / "navigation.png")}


def publish(results, videos):
    """📝 Publish a streaming measurement contract and retain the complete audit as Markdown."""
    payload = {"profile": {"width": SIZE[0], "height": SIZE[1], "fps": FPS, "language": "de", "audience": "spectator"},
               "expected": len(videos), "verified": len(results), "manifest_sha256": sha(MANIFEST),
               "catalog_sha256": sha(CATALOG),
               "shared_root_pass": len({row["root_rgb_sha256"] for row in results}) == 1 if len(results) == len(videos) else None,
               "videos": results}
    temporary = GENERATED / "measurements.new"
    temporary.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    temporary.replace(GENERATED / "measurements.json")
    lines = ["# Inspection Verification", "", f"Verified {len(results)} of {len(videos)} complete canonical lessons at 480 x 270 and 4fps, German spectator profile.", "", "PyAV independently decoded every selected source, assembled chapter, body and final frame. All clocks were checked frame by frame. Streaming pixel comparisons check every body frame against the ordered canonical source or measured clause excerpt and the final presentation; the four complete outer edges are compared separately to detect chrome. Independent semantic cut reconstruction uses narration trace keys and frame windows. Pillow produces coverage and every-navigation-frame sheets for visual audit.", "", "| Lesson | Frames | Duration | Chapters | Segments | Cuts | Source/Content Error | Body Error | Edge Error | Silent Audio |", "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |"]
    for row in results:
        lines.append(f"| {row['id']} | {row['frames']} | {row['duration']:.2f}s | {row['chapter_count']} | {row['segment_count']} | {len(row['cuts'])} | {row['source_content_max_mean_edge_error']:.6f} | {row['body_max_mean_error']:.6f} | {row['body_max_edge_error']:.6f} | {row['audio']['peak'] < 1e-6} |")
    lines += ["", "Each authored 3.6s navigation is rounded to 14 frames (3.5s) at this inspection frame rate. The two root overview endpoints are pixel-identical within every verified lesson. Camera endpoint seams have mean RGB error below 2. Sources with no audio require explicit decoded silent audio in the output; all audio clocks are checked against the complete video duration.", "", f"Manifest SHA-256: {payload['manifest_sha256']}", "", f"Presentation catalog SHA-256: {payload['catalog_sha256']}", "", "## Evidence Identities", ""]
    for row in results:
        lines += [f"### {row['id']}", "", f"- Delivery: {row['path']}", f"- SHA-256: {row['sha256']}", f"- Content SHA-256: {row['content_sha256']}", f"- Root RGB SHA-256: {row['root_rgb_sha256']}", f"- Audio: {json.dumps(row['audio'])}", f"- Opening seam RGB mean/edge/max: {row['opening_seam']}", f"- Closing seam RGB mean/edge/max: {row['closing_seam']}", f"- Selected clause windows: {json.dumps(row['cuts'])}", ""]
    (TICKET / "inspection-verification.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def publication_ready(path, identities):
    """🏁 Require the final production gate after the latest measured publication."""
    if path is None:
        return True
    try:
        state = json.loads(path.read_text(encoding="utf-8"))
        return (all(state.get(key) == value for key, value in
                    {"state": "complete", "videos": 17, "sources": 117, "chapters": 38}.items())
                and path.stat().st_mtime_ns >= max(stamp[1] for stamp in identities.values()))
    except (OSError, ValueError):
        return False


def main():
    """⏳ Verify each atomic publication as it arrives, with cancellation between frames."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--render-root", type=Path, default=TICKET / "🗑️generated/i")
    parser.add_argument("--watch", action="store_true")
    parser.add_argument("--publication-complete", type=Path)
    args = parser.parse_args()
    GENERATED.mkdir(parents=True, exist_ok=True)
    videos = json.loads(MANIFEST.read_text(encoding="utf-8"))["videos"]
    render = args.render_root / "de/spectator"
    expected = plan(videos, render)
    results, done = [], {}
    publish(results, videos)
    while True:
        for video in videos:
            output = render / "rendered" / f"{video['output']}_spectator_de_480x270p4.mp4"
            if not output.with_suffix(".render.json").is_file() or not output.is_file():
                continue
            stamp = (output.stat().st_size, output.stat().st_mtime_ns)
            if done.get(video["id"]) == stamp:
                continue
            INFO.pop(str(output.resolve()), None)
            index, entries = expected[video["id"]]
            print("[DEBUG] Verifying", video["id"], flush=True)
            result = verify(video, index, entries, render, output)
            assert stamp == (output.stat().st_size, output.stat().st_mtime_ns), (video["id"], "publication replaced during verification; rerun verifier")
            results = [row for row in results if row["id"] != video["id"]]
            results.append(result)
            order = {value["id"]: index for index, value in enumerate(videos)}
            results.sort(key=lambda row: order[row["id"]])
            done[video["id"]] = stamp
            publish(results, videos)
            print("[DEBUG] Verified", video["id"], result["frames"], result["duration"], flush=True)
        if len(done) == len(videos):
            changed = []
            for video in videos:
                output = render / "rendered" / f"{video['output']}_spectator_de_480x270p4.mp4"
                state = output.stat()
                if done[video["id"]] != (state.st_size, state.st_mtime_ns):
                    changed.append(video["id"])
            if changed:
                for identity in changed:
                    done.pop(identity)
                results = [row for row in results if row["id"] not in changed]
                publish(results, videos)
                continue
            if publication_ready(args.publication_complete, done):
                roots = {row["root_rgb_sha256"] for row in results}
                assert len(roots) == 1, ("Final lessons do not share one exact overview root", roots)
                break
        if not args.watch:
            break
        time.sleep(15)
    print("[DEBUG] Verification complete", len(done), "of", len(videos), flush=True)


if __name__ == "__main__":
    main()
