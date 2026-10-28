"""🧪 Verify current provenance and complete decoding of every canonical source movie."""

from __future__ import annotations

import hashlib
import importlib
import json
from pathlib import Path
import sys

ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))

import av

RUNTIME = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated" / "full"
REPORT = TICKET / "🗑️generated" / "source-verification.json"
SETTINGS = {"quality": "-ql", "resolution": "640,360", "fps": 6.0}


def verify(job: dict, prior: dict) -> dict:
    """🔎 Validate source identity, media headers, every frame and timeline completion."""
    segment = {key: job["segment"][key] for key in ("script", "scene")}
    path = OUTPUT / "s" / str(job["source_ordinal"]) / "v" / f"{segment['scene']}.mp4"
    state = RUNTIME.segment_state(segment, SETTINGS)
    if not RUNTIME.reusable_clip(path, state):
        raise ValueError(f"Stale or incomplete canonical source: {path}")
    provenance = hashlib.sha256(json.dumps(state, sort_keys=True).encode()).hexdigest()
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    if prior.get("media_sha256") == digest and prior.get("provenance_sha256") == provenance:
        return prior
    with av.open(path) as container:
        stream = container.streams.video[0]
        rate = float(stream.average_rate)
        duration = float(stream.duration * stream.time_base)
        declared = stream.frames
        count, previous = 0, -1.0
        if (stream.width, stream.height) != (640, 360) or abs(rate - 6) > 0.001 or duration <= 0:
            raise ValueError(f"Unexpected source media format: {path}")
        for frame in container.decode(video=0):
            current = float(frame.time)
            if current < previous or (frame.width, frame.height) != (640, 360):
                raise ValueError(f"Invalid source frame timeline: {path}")
            count += 1
            previous = current
        if count <= 0 or (declared and count != declared) or abs(previous + 1 / rate - duration) > 0.02:
            raise ValueError(f"Incomplete source decoding: {path}")
        audio = len(container.streams.audio)
    return {"ordinal": job["source_ordinal"], **segment, "path": str(path), "media_sha256": digest,
            "provenance_sha256": provenance, "width": 640, "height": 360, "fps": rate, "duration": duration,
            "declared_frames": declared, "decoded_frames": count, "last_frame_time": previous, "audio_streams": audio}


def main() -> None:
    """📋 Persist per-source evidence and summarize actual current source coverage."""
    plan = RUNTIME.segment_plan()
    sources = RUNTIME.source_plan(plan)
    previous = {row["ordinal"]: row for row in json.loads(REPORT.read_text(encoding="utf-8"))} if REPORT.is_file() else {}
    partial = {job["source_ordinal"] for job in plan if "clauses" in job["segment"]}
    rows = []
    for index, job in enumerate(sources, 1):
        row = verify(job, previous.get(job["source_ordinal"], {}))
        if job["source_ordinal"] in partial:
            segment = {key: job["segment"][key] for key in ("script", "scene")}
            if RUNTIME.source_marks(Path(row["path"]), RUNTIME.segment_state(segment, SETTINGS)) is None:
                raise ValueError(f"Missing current source narration trace: {row['path']}")
        rows.append(row)
        if index % 10 == 0 or index == len(sources):
            print(f"[DEBUG] Verified canonical sources: {index}/{len(sources)}", flush=True)
    REPORT.write_text(json.dumps(rows, indent=2) + "\n", encoding="utf-8")
    summary = ["# Canonical Source Verification", "",
               f"All {len(rows)} unique canonical sources have current render provenance and were completely decoded with PyAV at 640 × 360, nominal 6 fps.",
               f"Decoded source frames: {sum(row['decoded_frames'] for row in rows):,}.",
               "Every source retained its dimensions, nondecreasing presentation timestamps, declared frame count where present, and a complete end-of-video timeline. Every reused semantic source also retained a current measured narration trace bound to its source fingerprint.",
               "", "Detailed SHA256, provenance digests, frame counts and durations are in generated `source-verification.json`. The retained `verify-sources.py` input reproduces this check. This establishes media/provenance integrity; independent sampled visual findings are recorded in `full-render-audit.md`.", ""]
    (TICKET / "source-verification.md").write_text("\n".join(summary), encoding="utf-8")


if __name__ == "__main__":
    main()
