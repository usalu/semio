"""🔎 Seek trace-defined transition and settled frames from one newly certified source."""
import argparse
import hashlib
import json
from pathlib import Path
import time

import av

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
BASE = TICKET / "🗑️generated/i/de/spectator"
OUTPUT = TICKET / "🗑️generated/iv"


def sha(path):
    """🔏 Fingerprint current certified inputs with bounded reads."""
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main():
    """📸 Use real measured clause windows and keyframe seeks without a full-source decode."""
    parser = argparse.ArgumentParser()
    parser.add_argument("ordinal", type=int)
    parser.add_argument("--keys", nargs="+", required=True)
    args = parser.parse_args()
    document = json.loads((ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json").read_text(encoding="utf-8"))
    sources, occurrence, selected = {}, 0, None
    for video in document["videos"]:
        for chapter in video["chapters"]:
            for segment in chapter["segments"]:
                occurrence += 1
                identity = (segment["script"], segment["scene"])
                sources.setdefault(identity, occurrence)
                if sources[identity] == args.ordinal:
                    selected = segment
    assert selected is not None
    source = BASE / "s" / str(args.ordinal) / "v" / (selected["scene"] + ".mp4")
    certificate = source.with_suffix(".render.json")
    state_bytes = certificate.read_bytes()
    state = json.loads(state_bytes)
    assert all(sha(Path(path)) == digest for path, digest in state["sources"].items())
    owner = (ROOT / "🎓️teaching" / selected["script"]).parent
    identity = hashlib.sha256(owner.relative_to(ROOT).as_posix().encode()).hexdigest()[:10]
    trace = BASE / "trace" / identity / selected["scene"] / "🔣️.json"
    trace_bytes = trace.read_bytes()
    marks = json.loads(trace_bytes)["beats"][selected["scene"]]
    initial = source.stat()
    images = []
    OUTPUT.mkdir(parents=True, exist_ok=True)
    with av.open(str(source)) as media:
        stream = media.streams.video[0]
        stream.codec_context.thread_count = 1
        assert (stream.width, stream.height) == (480, 270)
        rate = float(stream.average_rate)
        assert rate == 4
        duration = float(stream.duration * stream.time_base)
        for key in args.keys:
            start, window = float(marks[key]["start"]), float(marks[key]["window"])
            times = {"early": start + 0.25, "settled": start + min(2.5, window / 2),
                     "late": start + window - 0.25}
            for phase, target in times.items():
                target = max(0, min(duration - 1 / rate, target))
                media.seek(int(target / stream.time_base), stream=stream, backward=True)
                frame = next(frame for frame in media.decode(stream)
                             if float(frame.pts * frame.time_base) + 1e-8 >= target)
                path = OUTPUT / f"source-{args.ordinal}-{key}-{phase}.png"
                frame.to_image().save(path)
                images.append({"key": key, "phase": phase, "target": target,
                               "seconds": float(frame.pts * frame.time_base), "path": str(path), "sha256": sha(path)})
    final = source.stat()
    assert (initial.st_size, initial.st_mtime_ns) == (final.st_size, final.st_mtime_ns)
    assert certificate.read_bytes() == state_bytes and trace.read_bytes() == trace_bytes
    result = {"ordinal": args.ordinal, "scene": selected["scene"], "path": str(source), "source_sha256": sha(source),
              "certificate_sha256": hashlib.sha256(state_bytes).hexdigest(),
              "trace_sha256": hashlib.sha256(trace_bytes).hexdigest(), "images": images, "checked": time.time()}
    pending = OUTPUT / f"source-{args.ordinal}-sampling.new"
    pending.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    pending.replace(OUTPUT / f"source-{args.ordinal}-sampling.json")
    with (TICKET / "inspection-trace-samples.md").open("a", encoding="utf-8") as report:
        report.write("\n## Source " + str(args.ordinal) + "\n\nCertified current-input, trace-window sampling with bounded keyframe seeks.\n\n```json\n"
                     + json.dumps(result, ensure_ascii=False, indent=2) + "\n```\n")
    print("[DEBUG] Trace sample images", len(images), args.ordinal, flush=True)


if __name__ == "__main__":
    main()
