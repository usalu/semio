"""🔬 Independently verify native PNG normalization and every fixed EB07 body pixel."""
import hashlib
import json
from pathlib import Path

import av
import numpy as np
from PIL import Image

TICKET = Path(__file__).resolve().parent
BASE = TICKET / "🗑️generated"
RAW = BASE / "i/de/spectator/5/v/EnergyBalance_07_TheorieVsPraxis.mp4"
ORIGINAL = BASE / "ie/varied"
FIXED = BASE / "ie/varied-fixed"


def sha(path):
    """🔏 Fingerprint immutable diagnostic inputs with bounded reads."""
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def pixels(path):
    """🎞️ Stream actual decoded frames rather than trusting container frame metadata."""
    with av.open(str(path)) as media:
        stream = media.streams.video[0]
        stream.codec_context.thread_count = 1
        for frame in media.decode(stream):
            yield float(frame.pts * frame.time_base), frame.to_ndarray(format="rgb24")


def main():
    """✅ Prove pixel preservation independently of the producer's normalization/clock helpers."""
    movie = FIXED / "encoded.mp4"
    inputs = [RAW, movie, *sorted(ORIGINAL.glob("[io]/*.png")), *sorted((FIXED / "normalized").glob("[io]/*.png"))]
    before = {str(path): sha(path) for path in inputs}
    images = []
    for phase, count in (("i", 9), ("o", 18)):
        paths = sorted((ORIGINAL / phase).glob("*.png"))
        assert len(paths) == count
        for path in paths:
            target = FIXED / "normalized" / phase / path.name
            with Image.open(path) as old, Image.open(target) as new:
                np.testing.assert_array_equal(np.asarray(old.convert("RGB")), np.asarray(new))
                assert old.size == new.size == (480, 270) and new.mode == "RGB" and not new.info
                alpha = old.getchannel("A").getextrema() if old.mode == "RGBA" else None
                images.append({"source": str(path), "normalized": str(target), "source_mode": old.mode,
                               "alpha_extrema": alpha, "source_sha256": before[str(path)], "normalized_sha256": before[str(target)]})
    body = iter(pixels(RAW))
    total, matched = 0, 0
    for index, (pts, image) in enumerate(pixels(movie)):
        assert abs(pts - index / 4) < 1e-8
        if 9 <= index < 1318:
            body_pts, expected = next(body)
            assert abs(body_pts - matched / 4) < 1e-8
            np.testing.assert_array_equal(image, expected)
            matched += 1
        total += 1
    assert total == 1336 and matched == 1309 and next(body, None) is None
    with av.open(str(RAW)) as media:
        assert not media.streams.audio
    audio = {"maximum_amplitude": 0.0, "maximum_gap": 0.0}
    with av.open(str(movie)) as media:
        assert len(media.streams.audio) == 1
        stream = media.streams.audio[0]
        assert stream.codec_context.name == "aac"
        audio["duration"] = float(stream.duration * stream.time_base)
        assert abs(audio["duration"] - total / 4) < .025
        end = 0
        for frame in media.decode(stream):
            start = float(frame.pts * frame.time_base)
            audio["maximum_gap"] = max(audio["maximum_gap"], abs(start - end))
            audio["maximum_amplitude"] = max(audio["maximum_amplitude"], float(np.max(np.abs(frame.to_ndarray()))))
            end = start + frame.samples / frame.sample_rate
        audio["decoded_end"] = end
        assert audio["maximum_gap"] < 1 / stream.codec_context.sample_rate and audio["maximum_amplitude"] == 0
    assert before == {str(path): sha(path) for path in inputs}
    result = {"pass": True, "frames": total, "body_frames_exact": matched, "focus_frames": 9, "restore_frames": 18,
              "files": before, "images": images, "audio": audio}
    (BASE / "iv/encoder-normalization-proof.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    (TICKET / "inspection-encoder-normalization-proof.md").write_text(
        "# Independent Encoder Normalization Proof\n\nAll27 normalized PNGs preserve the original decoded RGB values exactly; all are fresh RGB8 with no inherited metadata. Original files were unchanged. The corrected replay decodes1336 contiguous4fps frames, with every one of1309 body RGB frames exactly matching raw at offset9. Silent AAC spans334seconds without internal clock gaps. This is a diagnostic replay proof; final publications still require current owner/source gates and independent55-movie decoding.\n\n```json\n"
        + json.dumps(result, ensure_ascii=False, indent=2) + "\n```\n", encoding="utf-8")
    print("[DEBUG] Independent PNG27/body1309/frames1336/audio normalization PASS", flush=True)


if __name__ == "__main__":
    main()
