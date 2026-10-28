"""🖼️ Bind only current native clause/stage evidence and verify its decoded pixels."""
import hashlib
import json
import math
from pathlib import Path

import av
from PIL import Image

TICKET = Path(__file__).resolve().parent
CONFIG_PATH = TICKET / "inspection-evidence-cases.json"
CONFIG = json.loads(CONFIG_PATH.read_text(encoding="utf-8"))


def sha(path):
    """🔏 Fingerprint finite files with bounded memory."""
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def exact_coverage(actual, expected):
    """📚 Require exact authored order without omitted or repeated visual samples."""
    assert actual == expected, (actual, expected)


def compare_samples(source, requests):
    """🎞️ Stream native frames and compare every declared image/tile with exact decoded pixels."""
    assert requests and requests == sorted(requests, key=lambda item: item["seconds"])
    cursor = 0
    page, image = None, None
    with av.open(str(source)) as media:
        stream = media.streams.video[0]
        stream.codec_context.thread_count = 1
        assert (stream.width, stream.height, float(stream.average_rate)) == (CONFIG["width"], CONFIG["height"], CONFIG["fps"])
        for frame in media.decode(stream):
            seconds = float(frame.pts * frame.time_base)
            while cursor < len(requests) and seconds + 1e-8 >= requests[cursor]["seconds"]:
                request = requests[cursor]
                assert abs(seconds - request["seconds"]) < 1e-8, (source, seconds, request)
                if page != request["path"]:
                    if image is not None:
                        image.close()
                    with Image.open(request["path"]) as opened:
                        assert opened.format == "PNG"
                        image = opened.convert("RGB")
                    page = request["path"]
                tile = image.crop(request["box"])
                assert tile.size == (CONFIG["width"], CONFIG["height"])
                assert tile.tobytes() == frame.to_image().convert("RGB").tobytes(), (source, cursor, page, "native evidence pixels")
                cursor += 1
            if cursor == len(requests):
                break
    if image is not None:
        image.close()
    assert cursor == len(requests), (source, cursor, len(requests))


def bundle(document, cases, runtime, base):
    """🔐 Resolve explicit final ledgers and bind their current native media, traces and image evidence."""
    output = TICKET / "🗑️generated/iv"
    teaching = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir()) / "🎓️teaching"
    jobs = {job["source_ordinal"]: job for job in runtime.source_plan(runtime.segment_plan(document["videos"]))}
    files, records = {}, []

    def bind(path, expected=None):
        """📌 Bind a single stable byte identity and reject historical ledger references."""
        path = path.resolve()
        value = sha(path)
        assert expected is None or value == expected, (path, "stale evidence binding")
        key = path.as_posix()
        assert key not in files or files[key] == value, (key, "conflicting evidence")
        files[key] = value
        return value

    def native(ordinal, item):
        """🧬 Require ledger provenance to equal the genuine current canonical native certificate."""
        segment = jobs[ordinal]["segment"]
        assert item["ordinal"] == ordinal and item["scene"] == segment["scene"]
        source = base / "s" / str(ordinal) / "v" / (segment["scene"] + ".mp4")
        assert Path(item["path"]).resolve() == source.resolve()
        certificate = source.with_suffix(".render.json")
        state = json.loads(certificate.read_text(encoding="utf-8"))
        bind(source, item["source_sha256"])
        bind(certificate, item["certificate_sha256"])
        assert runtime.reusable_clip(source, state) and runtime.state_inputs_current(state), str(source)
        for name, digest in state["sources"].items():
            bind(Path(name), digest)
        owner = (teaching / segment["script"]).parent
        identity = hashlib.sha256(owner.relative_to(teaching.parent).as_posix().encode()).hexdigest()[:10]
        trace = base / "trace" / identity / segment["scene"] / "🔣️.json"
        bind(trace, item["trace_sha256"])
        marks = json.loads(trace.read_text(encoding="utf-8"))["beats"][segment["scene"]]
        return source, marks

    ledger = output / "bim-beats/sampling.json"
    bind(ledger)
    bim = json.loads(ledger.read_text(encoding="utf-8"))
    authored = cases["chapters"]
    assert len(authored) == CONFIG["bim_sources"] and sum(len(case["keys"]) for case in authored) == CONFIG["clauses"]
    assert bim["expected"] == bim["sampled"] == CONFIG["clauses"]
    by_id = {item["id"]: item for item in bim["sources"]}
    assert len(by_id) == len(bim["sources"]) == len(authored) and set(by_id) == {case["id"] for case in authored}
    for case in authored:
        item = by_id[case["id"]]
        source, marks = native(item["ordinal"], item)
        assert jobs[item["ordinal"]]["segment"]["script"] == case["script"] and item["scene"] == case["scene"]
        exact_coverage(list(marks), case["keys"])
        exact_coverage([image["key"] for image in item["images"]], case["keys"])
        capacity = CONFIG["columns"] * CONFIG["rows_per_page"]
        assert len(item["sheets"]) == math.ceil(len(case["keys"]) / capacity)
        requests = []
        for index, image in enumerate(item["images"]):
            mark = marks[image["key"]]
            target = float(mark["start"]) + max(0, float(mark["window"]) - 1 / CONFIG["fps"])
            assert abs(image["target"] - target) < 1e-8 and target - 1e-8 <= image["seconds"] < target + 1 / CONFIG["fps"] + 1e-8
            assert image["page"] == index // capacity + 1
            x, y = index % CONFIG["columns"] * CONFIG["width"], index % capacity // CONFIG["columns"] * CONFIG["row_height"]
            requests.append({"path": Path(item["sheets"][index // capacity]), "seconds": image["seconds"],
                             "box": (x, y, x + CONFIG["width"], y + CONFIG["height"])})
        for page, name in enumerate(item["sheets"]):
            path = Path(name).resolve()
            assert path.parent == ledger.parent.resolve() and path.name == f"bim-{item['ordinal']}-{item['source_sha256'][:8]}-{page + 1:02}.png"
            with Image.open(path) as image:
                assert image.size == (CONFIG["columns"] * CONFIG["width"],
                                      math.ceil(min(capacity, len(case["keys"]) - page * capacity) / CONFIG["columns"]) * CONFIG["row_height"])
            chapter = next(chapter["scene"] for video in document["videos"] if video["id"] == case["id"]
                           for chapter in video["chapters"] if any(segment["scene"] == item["scene"] for segment in chapter["segments"]))
            records.append({"path": path.as_posix(), "sha256": bind(path), "clip_id": case["id"], "kind": "bim-clause-sheet",
                            "source_sha256": item["source_sha256"], "scene": chapter, "raw_scene": item["scene"]})
        compare_samples(source, requests)
    chosen = CONFIG["optimization"]
    ledger = output / "source-122-sampling.json"
    bind(ledger)
    item = json.loads(ledger.read_text(encoding="utf-8"))
    assert item["ordinal"] == chosen["ordinal"] and item["scene"] == chosen["scene"]
    owner_video = next(video for video in document["videos"] if video["id"] == chosen["clip_id"])
    assert any(segment["scene"] == item["scene"] for chapter in owner_video["chapters"] for segment in chapter["segments"])
    source, marks = native(chosen["ordinal"], item)
    exact_coverage([(image["key"], image["phase"]) for image in item["images"]],
                   [(key, phase) for key in chosen["keys"] for phase in chosen["phases"]])
    requests = []
    for image in item["images"]:
        path = Path(image["path"]).resolve()
        assert path.parent == output.resolve() and path.name == f"source-122-{image['key']}-{image['phase']}.png"
        mark = marks[image["key"]]
        start, window = float(mark["start"]), float(mark["window"])
        target = {"early": start + .25, "settled": start + min(2.5, window / 2), "late": start + window - .25}[image["phase"]]
        assert abs(target - image["target"]) < 1e-8 and target - 1e-8 <= image["seconds"] < target + .25 + 1e-8
        with Image.open(path) as opened:
            assert opened.size == (CONFIG["width"], CONFIG["height"])
        chapter = next(chapter["scene"] for chapter in owner_video["chapters"] if any(segment["scene"] == item["scene"] for segment in chapter["segments"]))
        records.append({"path": path.as_posix(), "sha256": bind(path, image["sha256"]), "clip_id": chosen["clip_id"],
                        "kind": "optimization-122-stage", "source_sha256": item["source_sha256"], "scene": chapter, "raw_scene": item["scene"]})
        requests.append({"path": path, "box": (0, 0, CONFIG["width"], CONFIG["height"]), "seconds": image["seconds"]})
    compare_samples(source, requests)
    for path in (Path(__file__), CONFIG_PATH, TICKET / "inspection-evidence-test.py",
                 TICKET / "inspection-evidence.schema.json", TICKET / "inspection-evidence-schema.mjs",
                 TICKET / "inspection-bim-beat-sheets.py", TICKET / "inspection-trace-samples.py"):
        bind(path)
    assert all(sha(Path(path)) == digest for path, digest in files.items()), "Supplementary evidence changed during pixel verification"
    return {"files": files, "evidence": records,
            "evidence_counts": {"clauses": CONFIG["clauses"], "bim_sources": len(authored), "optimization_122_frames": len(item["images"])}}
