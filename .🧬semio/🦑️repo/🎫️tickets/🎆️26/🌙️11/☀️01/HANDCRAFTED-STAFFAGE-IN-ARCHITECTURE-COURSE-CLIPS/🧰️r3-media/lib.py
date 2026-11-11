"""🎞️ Shared helpers of the architecture course media pipeline: canonical plan, freshness, provenance, posters and thumbnails.

Run through `run.sh` so the teaching Python environment is used. Freshness is the runtime's own check
(`reusable_clip(clip, segment_state(...))`), whose state embeds the staffage cast keys of every scene,
so artwork rendered from an older asset, exporter or cast manifest is never reported fresh.

@see 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/⚙️runtime/🐍️.py — the render runtime
@see 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🐍️.py — cast keys
"""
import functools
import hashlib
import importlib
import json
import os
import sys
from pathlib import Path

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
CACHE = Path(os.environ.get("SEMIO_TEACHING_CACHE_ROOT") or ROOT / ".🧬semio/🎓️teaching/⚡️cache")
VIDEOS = CACHE / "videos"
AUTHORING = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
POSTER_SIZE = (854, 480)
THUMB_SIZE = (320, 180)
POSTER_AT = 0.5
QUALITY, FPS, LABEL = "-ql", 15.0, "480p15"

sys.path.insert(0, str(ROOT))
os.environ.setdefault("SEMIO_TEACHING_OUTPUT_ROOT", str(VIDEOS))
os.environ.setdefault("SEMIO_TEACHING_CACHE_DIR", str(CACHE / "fonts"))
if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")


def runtime():
    """⚙️ The render runtime module, imported with the repository on the path."""
    return importlib.import_module(f"{AUTHORING}.🎬️clip.⚙️runtime.🐍️")


def staffage():
    """🧍 The staffage bridge module that owns the cast keys."""
    return importlib.import_module(f"{AUTHORING}.🎨️visuals.🧍️staffage.🐍️")


def digest(path: Path) -> str:
    """🔏 SHA-256 of a file, read in chunks."""
    sha = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            sha.update(chunk)
    return sha.hexdigest()


def canonical(value) -> str:
    """🧾 Stable JSON text used for hashing states."""
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def settings(language: str, audience: str, quality: str, fps: float | None) -> dict:
    """🎚️ The render settings dict the runtime fingerprints (`-ql` and 15 fps is the course preview)."""
    return {"language": language, "quality": quality, "resolution": None, "fps": fps, "audience": audience}


def base(language: str, audience: str) -> Path:
    """📁 The output folder of one language and audience."""
    return VIDEOS / language / audience


@functools.lru_cache(maxsize=None)
def _record(name: str) -> dict:
    """🏷️ The validated provenance record of one cast, read once per process."""
    return staffage().staffage_provenance(name)


def provenance(state: dict, records: bool = True) -> dict:
    """🧬 Staffage provenance of a segment state: the cast keys it was rendered with and, for a fresh clip, the validated provenance record of each cast (asset revision, view, scale, clock, scenario)."""
    casts = state["casts"]
    return {"casts": casts, **({"records": {name: _record(name) for name in casts}} if records else {})}


def segment_clip(base_dir: Path, job: dict) -> Path:
    """🎬 The canonical source clip of a plan entry."""
    return base_dir / "s" / str(job["source_ordinal"]) / "v" / f"{job['segment']['scene']}.mp4"


def segment_states(job_settings: dict, plan: list[dict], cache: dict | None = None) -> dict:
    """🧩 Map every plan entry to its current segment state once per source ordinal."""
    rt = runtime()
    cache = {} if cache is None else cache
    for job in plan:
        if job["source_ordinal"] not in cache:
            cache[job["source_ordinal"]] = rt.segment_state({key: job["segment"][key] for key in ("script", "scene")}, job_settings)
    return cache


def video_status(video_id: str, language: str, audience: str, output: str, states: dict | None = None) -> dict:
    """📋 Freshness of every segment clip and of the lesson master of one manifest video."""
    rt = runtime()
    plan = [job for job in rt.segment_plan() if job["video_id"] == video_id]
    config = settings(language, audience, QUALITY, FPS)
    states = segment_states(config, plan, states)
    folder = base(language, audience)
    segments = []
    for job in plan:
        clip = segment_clip(folder, job)
        state = states[job["source_ordinal"]]
        fresh = rt.reusable_clip(clip, state)
        segments.append({"scene": job["segment"]["scene"], "script": job["segment"]["script"], "ordinal": job["source_ordinal"],
                         "clip": clip, "fresh": fresh, "declared": job["segment"], "state": state,
                         "state_sha256": hashlib.sha256(canonical(state).encode()).hexdigest()})
    master = folder / "rendered" / f"{output}_{audience}_{language}_{LABEL}.mp4"
    return {"video": video_id, "segments": segments, "master": master, "content": folder / str(plan[0]["video_index"]) / "content.mp4", "settings": config}


def master_fresh(status: dict) -> bool:
    """♻️ A lesson master is fresh when it exists and was assembled from exactly the current state of every segment."""
    master = status["master"]
    sidecar = master.with_suffix(".render.json") if master else None
    if not master or not master.is_file() or not sidecar.is_file() or not all(segment["fresh"] for segment in status["segments"]):
        return False
    saved = json.loads(sidecar.read_text(encoding="utf-8"))
    recorded = [segment for chapter in saved["content"]["chapters"] for segment in chapter["segments"]]
    expected = [{**segment["state"], "segment": segment["declared"]} for segment in status["segments"]]
    current = runtime().presentation_state(status["video"], status["settings"], status["settings"]["language"])
    return (recorded == expected and saved.get("presentation") == current and runtime().state_inputs_current(saved)
            and runtime().reusable_clip(status["content"], saved["content"])
            and saved.get("cursor") == runtime()._cursor_hash(status["content"]))


def frame(path: Path, at: float, size: tuple[int, int]):
    """🖼️ Decode the frame at fraction `at` of a clip as a resized Pillow image through PyAV (the repo ffmpeg is unusable)."""
    import av
    from PIL import Image

    with av.open(str(path)) as container:
        stream = container.streams.video[0]
        stream.thread_type = "AUTO"
        seconds = float(stream.duration * stream.time_base) if stream.duration else float(container.duration) / 1_000_000
        container.seek(int(max(seconds * at, 0) / stream.time_base), stream=stream, backward=True)
        picked = None
        for decoded in container.decode(stream):
            picked = decoded
            if decoded.time is not None and decoded.time >= seconds * at:
                break
        if picked is None:
            raise ValueError(f"No video frame in {path}")
        return picked.to_image().resize(size, Image.LANCZOS)


def stills(path: Path, folder: Path, name: str) -> dict:
    """📸 Write `<name>.poster.png` and `<name>.thumb.png` from the middle of a clip into `folder`; return their paths and hashes."""
    folder.mkdir(parents=True, exist_ok=True)
    poster, thumb = folder / f"{name}.poster.png", folder / f"{name}.thumb.png"
    image = frame(path, POSTER_AT, POSTER_SIZE)
    image.save(poster, optimize=True)
    image.resize(THUMB_SIZE).save(thumb, optimize=True)
    return {"poster": poster, "thumbnail": thumb, "poster_sha256": digest(poster), "thumbnail_sha256": digest(thumb)}


def duration(path: Path) -> float:
    """⏱️ Container duration of a clip in seconds."""
    import av

    with av.open(str(path)) as container:
        return float(container.duration) / 1_000_000
