"""🖌️ Preview-only BIM geometry cache: compute each `building_geometry` once, persist it, and reload it instead of recomputing.

The exact rational building geometry costs about 6000 s per model. This wraps the imported `building_geometry` of the shared
BIM model module (the file on disk is untouched) and stores every result in `🗑️generated/bim-geometry.pkl`, keyed by the
sha256 of every project module imported at install time and by the sha256 of the model json, so a stale entry is never used.
"""
import hashlib
import importlib
import json
import os
import pickle
import sys
import time
from copy import deepcopy
from pathlib import Path

TICKET = Path(__file__).resolve().parents[1]
ROOT = next(p for p in TICKET.parents if (p / "🧰️framework").is_dir())
STORE = TICKET / "🗑️generated/bim-geometry.pkl"
STATS = {"hits": 0, "misses": 0, "miss_seconds": 0.0, "source": None}


def source_hash() -> str:
    """🔑 Hash the source of every project module imported so far, sorted by path."""
    digest = hashlib.sha256()
    root = str(ROOT).lower()
    files = sorted({str(Path(m.__file__)) for m in list(sys.modules.values()) if getattr(m, "__file__", None) and str(m.__file__).lower().startswith(root)})
    for path in files:
        digest.update(path.encode("utf-8", "surrogatepass"))
        digest.update(Path(path).read_bytes())
    return digest.hexdigest()


def install():
    """🔁 Wrap `building_geometry` of the imported BIM model module with the persistent cache."""
    model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
    source = source_hash()
    STATS["source"] = source
    original = model.building_geometry
    memory = {}
    if STORE.exists():
        try:
            stored = pickle.loads(STORE.read_bytes())
            if stored.get("source") == source:
                memory.update(stored["values"])
        except Exception as error:
            print("[DEBUG] geometry cache unreadable", error, flush=True)

    def persist():
        STORE.parent.mkdir(parents=True, exist_ok=True)
        temporary = STORE.with_suffix(".tmp")
        temporary.write_bytes(pickle.dumps({"source": source, "values": memory}, protocol=pickle.HIGHEST_PROTOCOL))
        os.replace(temporary, STORE)

    def building_geometry(model_value=None):
        """🗺️ Return the cached geometry of this exact model, computing it once on a miss."""
        key = hashlib.sha256(json.dumps(model._m(model_value), sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")).hexdigest()
        if key in memory:
            STATS["hits"] += 1
            return deepcopy(memory[key])
        started = time.time()
        print("[DEBUG] geometry miss", key[:12], flush=True)
        value = original(model_value)
        memory[key] = deepcopy(value)
        STATS["misses"] += 1
        STATS["miss_seconds"] += time.time() - started
        persist()
        print("[DEBUG] geometry stored", key[:12], round(time.time() - started), "s", flush=True)
        return value

    model.building_geometry = building_geometry
    return model
