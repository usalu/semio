"""🧪 Check the inspection verifier with independently encoded small media fixtures."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path

import av
import numpy as np

TICKET = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("inspection_verify", TICKET / "inspection-verify.py")
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)
ROOT = TICKET / "🗑️generated/iv/selftest-input"


def encode(path, values, *, audio=False, fps=4):
    """🎞️ Encode a tiny finite MP4 with third-party PyAV and the real H.264 decoder path."""
    path.parent.mkdir(parents=True, exist_ok=True)
    with av.open(str(path), "w") as container:
        video = container.add_stream("libx264", rate=fps)
        video.width, video.height, video.pix_fmt = 480, 270, "yuv420p"
        video.options = {"crf": "0", "preset": "ultrafast", "threads": "1"}
        sound = container.add_stream("aac", rate=48000) if audio else None
        if sound:
            sound.layout = "stereo"
        for index, value in enumerate(values):
            frame = av.VideoFrame.from_ndarray(np.full((270, 480, 3), value, dtype=np.uint8), format="rgb24")
            frame.pts = index
            for packet in video.encode(frame):
                container.mux(packet)
        for packet in video.encode():
            container.mux(packet)
        if sound:
            sample_count = round(len(values) / fps * 48000)
            for index in range(0, sample_count, 1024):
                frame = av.AudioFrame.from_ndarray(np.zeros((2, min(1024, sample_count - index)), dtype=np.float32), format="fltp", layout="stereo")
                frame.sample_rate, frame.pts = 48000, index
                for packet in sound.encode(frame):
                    container.mux(packet)
            for packet in sound.encode():
                container.mux(packet)


def main():
    """🔬 Prove valid media passes while chrome, root and timing regressions fail."""
    source = ROOT / "s/1/v/Beat.mp4"
    chapter = ROOT / "1/v/Chapter.mp4"
    content = ROOT / "1/content.mp4"
    body = [50, 80, 120, 160]
    for path in (source, chapter, content):
        encode(path, body)
    values = [32] * 13 + [50] + body + [160] + [32] * 13
    output = ROOT / "valid.mp4"
    encode(output, values, audio=True)
    segment = {"script": "fixture.py", "scene": "Beat"}
    video = {"id": "verifier_selftest", "chapters": [{"scene": "Chapter", "segments": [segment]}]}
    entries = [{"chapter": "Chapter", "segment": segment, "source": source, "selected": source}]
    result = VERIFY.verify(video, 1, entries, ROOT, output)
    assert result["pass"] and result["frames"] == 32 and result["audio"]["peak"] == 0
    checks = []
    for name, bad_values, fps in [("body-overlay", values[:14] + [220] + values[15:], 4), ("root-mismatch", values[:-1] + [200], 4), ("wrong-frame-rate", values, 5)]:
        bad = ROOT / (name + ".mp4")
        encode(bad, bad_values, audio=True, fps=fps)
        try:
            VERIFY.verify(video, 1, entries, ROOT, bad)
        except AssertionError:
            checks.append(name)
        else:
            raise AssertionError("Verifier accepted " + name)
    assert len(checks) == 3
    selected = source.with_name("cut.mp4")
    encode(selected, [80, 120])
    trace = source.with_suffix(".trace.json")
    trace.write_text(json.dumps({"marks": {"part": {"start": 0.25, "window": 0.5}}}), encoding="utf-8")
    selected.with_suffix(".render.json").write_text(json.dumps({"trace": VERIFY.sha(trace), "interval": [0.25, 0.75]}), encoding="utf-8")
    cut_segment = {**segment, "clauses": {"first": "part", "last": "part"}}
    cut_entry = {"segment": cut_segment, "source": source, "selected": selected}
    cut = VERIFY.cut_info(cut_entry)
    assert cut["canonical_start_frame"] == 1 and cut["frames"] == 2
    previous = VERIFY.info(selected)
    encode(selected, [80, 120, 160])
    replaced = VERIFY.info(selected)
    assert replaced["frames"] == 3 and replaced["sha256"] != previous["sha256"]
    encode(selected, [120, 120])
    try:
        VERIFY.cut_info(cut_entry)
    except AssertionError:
        checks.append("wrong-canonical-cut")
    else:
        raise AssertionError("Verifier accepted a wrong canonical excerpt")
    marker = ROOT / "publication.json"
    cases = json.loads((TICKET / "inspection-verifier-cases.json").read_text(encoding="utf-8"))["publication"]
    for case in cases:
        marker.unlink(missing_ok=True)
        if case["marker"] is not None:
            marker.write_text(json.dumps(case["marker"]), encoding="utf-8")
        modified = marker.stat().st_mtime_ns + 1 if case["future"] else 0
        assert VERIFY.publication_ready(marker, {"fixture": (1, modified)}) == case["pass"], case["name"]
    (TICKET / "inspection-verifier-test.md").write_text("# Inspection Verifier Test\n\nStatus: passed. PyAV independently encoded 480 x 270 source, chapter, content and complete delivery fixtures. A valid 32-frame (8s) H.264/AAC lesson passed full decode, ordered body comparison, navigation seam, identical-root and silent-audio checks. A valid measured two-frame clause excerpt also passed canonical source pixel comparison. Four deliberate regressions were rejected: a changed body frame (overlay), a mismatched final overview root, an incorrect frame rate and an excerpt with incorrect canonical source pixels despite valid timing/certificate metadata. Five language-neutral completion-marker cases reject missing/incomplete/stale gates and accept the current complete production gate. A same-path media replacement also refreshed decoded metadata and SHA without explicit cache eviction. Fixtures remain disposable under generated/iv/selftest-input; this retained test input introduces no runtime dependency.\n", encoding="utf-8")
    print("[DEBUG] Inspection verifier two positive fixtures, four negative controls and five publication-marker cases passed", flush=True)


if __name__ == "__main__":
    main()
