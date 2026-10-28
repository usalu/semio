"""🔎 Independently decode the complete physical lesson and audit its rendered media."""
from __future__ import annotations

import argparse
import hashlib
import itertools
import json
import math
from pathlib import Path

import av
import numpy as np
from PIL import Image, ImageDraw

TICKET = Path(__file__).resolve().parent
WORKSPACE = next(parent for parent in TICKET.parents if (parent / ".🧬semio").is_dir() and (parent / "🎓️teaching").is_dir())
MANIFEST = WORKSPACE / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json"
CATALOG = WORKSPACE / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🔣️.json"
EXPECTED = ["Beat1_Leistung", "Beat2_EnergieImAlltag", "Beat3_Energieerhaltung", "Beat4_Waermepumpe", "Beat5_Strahlung", "Beat6_ThermischeMasse", "Beat7_SensibelLatent", "Beat8_Zusammenfassung"]


def sha(path):
    """🔐 Record exact evidence identities without production cache helpers."""
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(chunk)
    return value.hexdigest()


def info(path):
    """📽️ Count successfully decoded frames rather than trusting a container header."""
    with av.open(str(path)) as media:
        stream = media.streams.video[0]
        times = [float(frame.pts * frame.time_base) for frame in media.decode(stream)]
        fps = float(stream.average_rate)
        assert times and all(b > a for a, b in zip(times, times[1:])), f"Nonmonotone video clock: {path}"
        assert all(abs((b - a) - 1 / fps) < 0.002 for a, b in zip(times, times[1:])), f"Irregular frame clock: {path}"
        return {"path": str(path), "sha256": sha(path), "width": stream.width, "height": stream.height, "fps": fps,
                "frames": len(times), "duration": len(times) / fps, "first_pts": times[0], "last_pts": times[-1],
                "audio_streams": len(media.streams.audio)}


def frames(path):
    """🖼️ Let the independent PyAV decoder produce RGB source frames."""
    with av.open(str(path)) as media:
        for frame in media.decode(media.streams.video[0]):
            yield frame.to_ndarray(format="rgb24")


def error(actual, expected):
    """📐 Measure all pixels and the four outer edges for unwanted presentation chrome."""
    assert actual.shape == expected.shape
    delta = np.abs(actual.astype(np.int16) - expected.astype(np.int16))
    edge = np.concatenate((delta[0].ravel(), delta[-1].ravel(), delta[:, 0].ravel(), delta[:, -1].ravel()))
    return float(delta.mean()), float(edge.mean()), int(delta.max())


def silence(path):
    """🔊 Inspect the actual decoded audio samples over the whole lesson."""
    energy, samples, peak, count = 0.0, 0, 0.0, 0
    with av.open(str(path)) as media:
        assert len(media.streams.audio) == 1, "The delivery requires one explicit audio stream"
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
    return {"sample_rate": rate, "decoded_samples_per_channel": count, "seconds": count / rate,
            "rms": math.sqrt(energy / samples), "peak": peak}


def main():
    """🧪 Check manifest coverage, source order, full body pixels, transitions and audio."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--render-root", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    generated = TICKET / "🗑️generated/full-physical-verification"
    generated.mkdir(parents=True, exist_ok=True)
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    lesson = next(video for video in manifest["videos"] if video["id"] == "physical_fundamentals")
    assert len(lesson["chapters"]) == 1
    chapter = lesson["chapters"][0]
    assert chapter["scene"] == "PhysicalFundamentals_FullSection"
    segments = chapter["segments"]
    assert [item["scene"] for item in segments] == EXPECTED
    assert all("clauses" not in item for item in segments)
    root = args.render_root / "de/spectator"
    sources = []
    for scene in EXPECTED:
        matches = list((root / "s").glob(f"*/v/{scene}.mp4"))
        assert len(matches) == 1, (scene, matches)
        sources.append(matches[0])
    content = root / "1/content.mp4"
    sources_info = [info(path) for path in sources]
    content_info = info(content)
    final_info = info(args.output)
    assert all((row["width"], row["height"]) == (640, 360) and abs(row["fps"] - 6) < 0.001 for row in [*sources_info, content_info, final_info])
    body_count = sum(row["frames"] for row in sources_info)
    assert content_info["frames"] == body_count
    count = max(3, round(3.6 * final_info["fps"]))
    assert final_info["frames"] == body_count + count * 2
    boundaries = list(itertools.accumulate([0, *[row["frames"] for row in sources_info]]))
    snapshots = set(range(count)) | set(range(count + body_count, final_info["frames"])) | {count, count + body_count - 1}
    for left, right in zip(boundaries, boundaries[1:]):
        snapshots.update((count + left, count + (left + right - 1) // 2, count + right - 1))
    expected_frames = itertools.chain.from_iterable(frames(path) for path in sources)
    content_frames = frames(content)
    captures, body_max, source_max, edges_max, max_pixel = {}, 0.0, 0.0, 0.0, 0
    for index, actual in enumerate(frames(args.output)):
        if index in snapshots:
            captures[index] = actual.copy()
        if count <= index < count + body_count:
            authored, composed = next(expected_frames), next(content_frames)
            source_mean, source_edge, source_pixel = error(composed, authored)
            body_mean, body_edge, body_pixel = error(actual, composed)
            source_max = max(source_max, source_mean, source_edge)
            body_max = max(body_max, body_mean)
            edges_max = max(edges_max, body_edge)
            max_pixel = max(max_pixel, source_pixel, body_pixel)
            assert source_mean < 0.01 and source_edge < 0.01, (index, "source order/content mismatch", source_mean, source_edge)
            assert body_mean < 1.0 and body_edge < 1.0, (index, "fullframe mismatch", body_mean, body_edge)
    assert next(content_frames, None) is None and next(expected_frames, None) is None
    inward = error(captures[count - 1], captures[count])
    outward = error(captures[count + body_count], captures[count + body_count - 1])
    root_difference = error(captures[0], captures[final_info["frames"] - 1])
    assert inward[0] < 2 and outward[0] < 2, ("Content camera seam", inward, outward)
    assert root_difference[0] == 0, ("Overview does not return to the same root", root_difference)
    audio = silence(args.output)
    assert all(row["audio_streams"] == 0 for row in sources_info), "Source audio expectation changed"
    assert audio["peak"] < 1e-6 and abs(audio["seconds"] - final_info["duration"]) < 0.1, audio
    tiles = [(0, "Opening: Topics"), (count - 1, "Opening: Content Focus")]
    tiles += [(count + (left + right - 1) // 2, scene) for scene, left, right in zip(EXPECTED, boundaries, boundaries[1:])]
    tiles += [(count + body_count, "Closing: Content Focus"), (final_info["frames"] - 1, "Closing: Topics")]
    sheet = Image.new("RGB", (640 * 3, 386 * 4), "#222222")
    draw = ImageDraw.Draw(sheet)
    for ordinal, (index, label) in enumerate(tiles):
        x, y = ordinal % 3 * 640, ordinal // 3 * 386
        sheet.paste(Image.fromarray(captures[index]), (x, y))
        draw.text((x + 8, y + 365), f"{label} | frame {index}", fill="white")
    sheet.save(generated / "coverage.png")
    for name, index in [("overview", 0), ("focus-seam", count - 1), ("body-first", count), ("body-last", count + body_count - 1), ("closing-seam", count + body_count), ("return", final_info["frames"] - 1)]:
        Image.fromarray(captures[index]).save(generated / f"{name}.png")
    for phase, start in [("opening", 0), ("closing", count + body_count)]:
        navigation = Image.new("RGB", (1280, 206 * math.ceil(count / 4)), "#222222")
        labels = ImageDraw.Draw(navigation)
        for offset in range(count):
            index = start + offset
            image = Image.fromarray(captures[index])
            image.save(generated / f"{phase}-{offset:02d}.png")
            x, y = offset % 4 * 320, offset // 4 * 206
            navigation.paste(image.resize((320, 180)), (x, y))
            labels.text((x + 6, y + 184), f"{phase} {offset / 6:.3f}s | frame {index}", fill="white")
        navigation.save(generated / f"{phase}-navigation.png")
    result = {"manifest_sha256": sha(MANIFEST), "presentation_catalog_sha256": sha(CATALOG),
              "sources": sources_info, "content": content_info, "final": final_info, "transition_frames_each": count,
              "authored_transition_seconds_each": 3.6, "encoded_transition_seconds_each": count / final_info["fps"],
              "source_to_content_max_mean_rgb_error": source_max, "body_max_mean_rgb_error": body_max,
              "body_max_edge_mean_rgb_error": edges_max, "max_single_channel_error": max_pixel,
              "opening_seam_rgb_error": inward, "closing_seam_rgb_error": outward,
              "overview_root_rgb_error": root_difference, "audio": audio}
    (generated / "measurements.json").write_text(json.dumps(result, indent=2), encoding="utf-8")
    rows = "\n".join(f"| {Path(row['path']).name} | {row['frames']} | {row['duration']:.3f}s | {boundaries[index] / 6:.3f}s | {row['sha256']} |" for index, row in enumerate(sources_info))
    report = f"""# Full Physical Fundamentals Verification

Status: all independent media assertions passed. Visual review of the generated coverage and navigation sheets is recorded separately in full-physical-visual-audit.md.

The canonical manifest contains the complete PhysicalFundamentals_FullSection in exactly the eight declared beats, without an institute introduction or clause excerpts. PyAV successfully decoded every source, authored content and delivery frame. Pillow assembled the independent coverage sheet.

| Source | Frames | Duration | Body Start | SHA-256 |
| --- | ---: | ---: | ---: | --- |
{rows}

Source durations use container average-rate metadata; minor millisecond differences reflect timestamp quantization. Body starts use the nominal 6fps frame budget.

The authored body has {body_count} frames ({body_count / 6:.3f}s). Each 3.6s navigation sequence is rounded to {count} frames ({count / 6:.3f}s) at 6fps. Delivery has {final_info['frames']} frames, lasts {final_info['duration']:.3f}s, and is 640 × 360 at 6fps. Frame timestamps are strictly increasing with a regular 1/6s cadence.

Every body frame was compared in order against both the concatenated content and the corresponding canonical source. Maximum source-to-content mean/edge RGB error: {source_max:.8f}; maximum body mean RGB error: {body_max:.8f}; maximum body outer-edge mean RGB error: {edges_max:.8f}. This checks source ordering, all eight complete segments and unobstructed full-frame playback across every pixel.

Opening-camera seam mean RGB error: {inward[0]:.8f}; closing-camera seam: {outward[0]:.8f}. The opening and closing overview root frames are pixel-identical. The exact endpoint images and source midpoint coverage sheet are in the owned generated verification directory during review.

All eight canonical rendered sources contain no audio stream. The delivery contains one decoded {audio['sample_rate']}Hz audio stream with {audio['seconds']:.3f}s decoded samples, peak {audio['peak']:.10f} and RMS {audio['rms']:.10f}; this is explicit silence rather than narration.

## Evidence Identities

- Manifest SHA-256: {result['manifest_sha256']}
- Presentation catalog SHA-256: {result['presentation_catalog_sha256']}
- Authored content SHA-256: {content_info['sha256']}
- Delivery SHA-256: {final_info['sha256']}
- Delivery path: {args.output}

The verifier is retained as full-physical-verification.py. Detailed transient measurements and PNGs belong to 🗑️generated/full-physical-verification and may be removed after review.
"""
    (TICKET / "full-physical-verification.md").write_text(report, encoding="utf-8")
    print(json.dumps({"status": "passed", "frames": final_info["frames"], "duration": final_info["duration"], "report": str(TICKET / "full-physical-verification.md"), "sheet": str(generated / "coverage.png")}, ensure_ascii=False))


if __name__ == "__main__":
    main()
