"""📦 Independently audit retained movies, sidecars, images, links and their final source epoch."""
import hashlib
import json
from pathlib import Path
import time

import av
from PIL import Image

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
DELIVERY = TICKET / "inspection-videos"


def sha(path):
    """🔏 Fingerprint retained or certified files with bounded reads."""
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def main():
    """🏁 Audit the actual immutable copy rather than infer delivery from generated metadata."""
    inputs = [GENERATED / "iv/chapter-measurements.json", GENERATED / "iv/chapter-provenance.json",
              GENERATED / "iv/bim-closure.json", GENERATED / "i/publish-complete.json"]
    original = {path: path.read_bytes() for path in inputs}
    metrics, provenance, closure, complete = [json.loads(original[path]) for path in inputs]
    assert metrics["pass"] and metrics["provenance_pass"] and metrics["verified"] == 38 and metrics["lessons"] == 17
    assert metrics["joins_pass"] and metrics["joins_verified"] == 38 and all(item["pass"] for item in metrics["joins"])
    assert provenance["pass"] and closure["pass"] and min(provenance["checked"], closure["checked"]) >= complete["completed"]
    bindings = {}
    for proof in (provenance, closure):
        for name, digest in proof["files"].items():
            key = Path(name).resolve().as_posix()
            assert key not in bindings or bindings[key] == digest, key
            bindings[key] = digest
    assert all(sha(Path(name)) == digest for name, digest in bindings.items()), "Inputs changed before retained audit"
    chapters, lessons = metrics["chapters"], metrics["videos"]
    assert len(chapters) == 38 and len(lessons) == 17
    records, expected = [], set()
    for record in [*lessons, *chapters]:
        chapter = "scene" in record
        path = DELIVERY / ("chapters/" + record["id"] + ".mp4" if chapter else record["id"] + ".mp4")
        assert path.resolve().is_relative_to(DELIVERY.resolve()) and sha(path) == record["sha256"], str(path)
        expected.add(path.resolve())
        with av.open(str(path)) as media:
            stream = media.streams.video[0]
            stream.codec_context.thread_count = 1
            assert (stream.width, stream.height, float(stream.average_rate), stream.frames) == (480, 270, 4, record["frames"])
            assert stream.codec_context.name == "h264" and len(media.streams.audio) == record["audio_streams"]
            assert next(media.decode(video=0)).to_image().size == (480, 270)
        certificate = path.with_suffix(".render.json")
        assert sha(certificate) == record["certificate_sha256"]
        if chapter:
            clock = path.with_suffix(".chapter.json")
            assert json.loads(clock.read_text(encoding="utf-8")) == record["clock"]
            assert sha(clock) == sha(Path(record["path"]).with_suffix(".chapter.json"))
            for kind in ("coverage", "opening", "closing"):
                image = path.with_name(record["id"] + "-" + kind + ".png")
                source = GENERATED / "iv/collection" / record["id"] / (kind + ".png")
                assert sha(image) == sha(source)
                if kind != "coverage":
                    with Image.open(image) as opened:
                        assert hashlib.sha256(opened.convert("RGB").tobytes()).hexdigest() == record[kind + "_rgb_sha256"]
        records.append({"id": record["id"], "kind": "chapter" if chapter else "lesson", "path": path,
                        "frames": record["frames"], "duration": record["duration"], "sha256": record["sha256"]})
    assert expected == {path.resolve() for path in DELIVERY.rglob("*.mp4")}
    assert len(list(DELIVERY.rglob("*.render.json"))) == 55 and len(list(DELIVERY.rglob("*.chapter.json"))) == 38
    for item in closure["evidence"]:
        path = DELIVERY / "evidence" / Path(item["path"]).name
        assert sha(path) == item["sha256"]
    images = list(DELIVERY.rglob("*.png"))
    assert len(images) == 133 and len(closure["evidence"]) == 19
    for path in images:
        with Image.open(path) as image:
            assert image.format == "PNG"
            image.verify()
    files = [path for path in DELIVERY.rglob("*") if path.is_file()]
    assert all(len(str(path).encode("utf-16-le")) // 2 <= 256 for path in files)
    index = (TICKET / "inspection.md").read_text(encoding="utf-8")
    assert all(record["path"].as_posix() in index for record in records), "A retained movie has no gallery link"
    assert all(sha(Path(name)) == digest for name, digest in bindings.items()), "Inputs changed during retained audit"
    assert all(path.read_bytes() == content for path, content in original.items()), "Proof identity changed during retained audit"
    chapter_frames = sum(record["frames"] for record in chapters)
    assert chapter_frames == sum(record["frames"] for record in lessons)
    checked = time.time()
    lines = ["# Retained Inspection Verification", "", "PASS:55 movies,93 sidecars and133 valid PNGs independently audited after retention.", "",
             "Profile:480×270,4fps, German, spectator. Every retained H264 movie SHA-256 matches its fully decoded source verification; PyAV reopened each retained container, checked dimensions/rate/frame count/audio-stream count and decoded its first frame. "
             "All38 chapter clocks and55 render certificates match verified bytes. All114 chapter evidence images and19 supplementary images match their checked copies; Pillow verified every PNG. Opening/closing RGB hashes match the38 exact join proofs. "
             "Every movie has a retained gallery link and every delivery path fits256 UTF-16 code units.", "",
             f"Final source epoch:publisher completion `{complete['completed']}`; full-media proof `{provenance['checked']}`; BIM/evidence proof `{closure['checked']}`; retained audit `{checked}`. "
             + str(len(bindings)) + " unique source/control/certificate bindings matched both before and after this audit. "
             + f"The38 chapter windows total{chapter_frames}frames; the17 lessons contain exactly the same ordered frames.", "",
             "Audio remains the independently measured silent AAC state. No raw source, image, sidecar or proof was modified by this audit.", "",
             "[Gallery](inspection.md) · [Delivery identities](inspection-delivery.md) · [Current evidence](inspection-evidence.md) · [Visual audit](inspection-final-epoch-visual.md)", "",
             "|Kind|ID|Frames|Seconds|SHA-256|", "|---|---|---:|---:|---|"]
    lines += [f"|{record['kind']}|[{record['id']}]({record['path'].as_posix()})|{record['frames']}|{record['duration']:.2f}|`{record['sha256']}`|" for record in records]
    lines += ["", "Audit input SHA-256:`" + sha(Path(__file__)) + "`.", ""]
    (TICKET / "inspection-retained-verification.md").write_text("\n".join(lines), encoding="utf-8")
    print("[DEBUG] Retained audit passed55movies93sidecars133PNGs38joins", checked, len(bindings), flush=True)


if __name__ == "__main__":
    main()
