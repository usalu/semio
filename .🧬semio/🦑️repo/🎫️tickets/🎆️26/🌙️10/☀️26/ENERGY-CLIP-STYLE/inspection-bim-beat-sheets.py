"""🖼️ Sample every current BIM clause from certified native movies without rendering."""
import argparse
import hashlib
import importlib.util
import json
import math
from pathlib import Path
import time

import av
from PIL import Image, ImageDraw

TICKET = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("trace_samples", TICKET / "inspection-trace-samples.py")
TRACE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(TRACE)
BASE = TRACE.BASE
OUTPUT = TRACE.OUTPUT / "bim-beats"
FIXTURE = TRACE.ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🔣️.cases.json"


def current(source):
    """🔐 Require every actual certified input before sampling a source."""
    certificate = source.with_suffix(".render.json")
    data = certificate.read_bytes()
    state = json.loads(data)
    assert all(TRACE.sha(Path(name)) == digest for name, digest in state["sources"].items())
    return data


def sample(case, ordinal, source, certificate):
    """📸 Stream once and retain late clause frames with exact source and trace identities."""
    owner = (TRACE.ROOT / "🎓️teaching" / case["script"]).parent
    identity = hashlib.sha256(owner.relative_to(TRACE.ROOT).as_posix().encode()).hexdigest()[:10]
    trace_path = BASE / "trace" / identity / case["scene"] / "🔣️.json"
    trace_bytes = trace_path.read_bytes()
    marks = json.loads(trace_bytes)["beats"][case["scene"]]
    assert list(marks) == case["keys"], (case["id"], "trace clause coverage")
    initial = (source.stat().st_size, source.stat().st_mtime_ns)
    digest = TRACE.sha(source)
    pages, images, targets = [], [], []
    for index, key in enumerate(case["keys"]):
        mark = marks[key]
        targets.append((float(mark["start"]) + max(0, float(mark["window"]) - .25), index, key))
    assert targets == sorted(targets)
    with av.open(str(source)) as media:
        stream = media.streams.video[0]
        stream.codec_context.thread_count = 1
        assert (stream.width, stream.height, float(stream.average_rate)) == (480, 270, 4)
        cursor = 0
        for frame in media.decode(stream):
            seconds = float(frame.pts * frame.time_base)
            while cursor < len(targets) and seconds + 1e-8 >= targets[cursor][0]:
                target, index, key = targets[cursor]
                page = index // 18
                if page == len(pages):
                    rows = math.ceil(min(18, len(targets) - page * 18) / 3)
                    pages.append(Image.new("RGB", (1440, rows * 296), "#222222"))
                x, y = index % 3 * 480, index % 18 // 3 * 296
                pages[page].paste(frame.to_image(), (x, y))
                ImageDraw.Draw(pages[page]).text((x + 5, y + 275), f"{ordinal} {key} | {seconds:.2f}s", fill="white")
                images.append({"key": key, "target": target, "seconds": seconds, "page": page + 1})
                cursor += 1
            if cursor == len(targets):
                break
    assert len(images) == len(case["keys"])
    assert initial == (source.stat().st_size, source.stat().st_mtime_ns)
    assert current(source) == certificate and trace_path.read_bytes() == trace_bytes
    sheets = []
    for index, image in enumerate(pages, 1):
        output = OUTPUT / f"bim-{ordinal}-{digest[:8]}-{index:02}.png"
        image.save(output)
        sheets.append(str(output))
    return {"id": case["id"], "ordinal": ordinal, "scene": case["scene"], "path": str(source),
            "source_sha256": digest, "certificate_sha256": hashlib.sha256(certificate).hexdigest(),
            "trace_sha256": hashlib.sha256(trace_bytes).hexdigest(), "sheets": sheets, "images": images}


def publish(records):
    """📜 Retain current-only clause coverage and explicit sampling limits."""
    result = {"checked": time.time(), "expected": 113, "sampled": sum(len(item["images"]) for item in records.values()),
              "sources": list(records.values())}
    (OUTPUT / "sampling.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    text = "# Current BIM Clause Sampling\n\nRead-only settled-frame sampling from genuine current native certificates. Each clause is sampled at its trace window end minus one4fps frame. Every source/input and trace identity is rechecked after streaming. This supports layout review; full-frame preservation, audio, joins and final currentness remain in the independent55-movie verifier.\n\n"
    text += f"Current coverage: {result['sampled']}/113 clauses across {len(records)}/6 sources.\n"
    for item in records.values():
        text += f"\n## {item['id']}\n\nSource SHA-256: `{item['source_sha256']}`\n\n"
        text += "\n".join(f"- {entry['key']}: {entry['seconds']:.2f}s, sheet{entry['page']}" for entry in item["images"]) + "\n"
    (TICKET / "inspection-bim-beat-sampling.md").write_text(text, encoding="utf-8")


def main():
    """👁️ Follow current BIM outputs while the independent publisher and verifier run."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--watch", action="store_true")
    args = parser.parse_args()
    cases = json.loads(FIXTURE.read_text(encoding="utf-8"))["chapters"]
    OUTPUT.mkdir(parents=True, exist_ok=True)
    records = {}
    while True:
        for ordinal, case in enumerate(cases, 114):
            source = BASE / "s" / str(ordinal) / "v" / (case["scene"] + ".mp4")
            try:
                certificate = current(source)
            except (OSError, AssertionError, ValueError):
                records.pop(case["id"], None)
                continue
            digest = hashlib.sha256(certificate).hexdigest()
            if records.get(case["id"], {}).get("certificate_sha256") == digest:
                continue
            try:
                records[case["id"]] = sample(case, ordinal, source, certificate)
            except (OSError, AssertionError, ValueError):
                records.pop(case["id"], None)
                continue
            print("[DEBUG] Current BIM clause sheets", case["id"], *records[case["id"]]["sheets"], flush=True)
        publish(records)
        marker = BASE.parents[1] / "publish-complete.json"
        if not args.watch or (len(records) == 6 and marker.is_file() and json.loads(marker.read_text())["state"] == "complete"):
            break
        time.sleep(30)


if __name__ == "__main__":
    main()
