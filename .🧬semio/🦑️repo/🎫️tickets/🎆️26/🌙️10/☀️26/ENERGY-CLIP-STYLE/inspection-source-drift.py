"""🔎 Retain one read-only checkpoint of active body source bindings."""
from pathlib import Path
import sys
import importlib
import json
import time

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
OUTPUT = TICKET / "🗑️generated/i"
sys.path.insert(0, str(ROOT))
RUNTIME = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
SETTINGS = dict(language="de", quality="-ql", resolution="480,270", fps=4.0, audience="spectator")

def checkpoint():
    """📍 Compare current content hashes with the preceding observation."""
    destination = TICKET / "🗑️generated/ip/native-current-bindings.json"
    previous = json.loads(destination.read_text(encoding="utf-8")) if destination.is_file() else {}
    records = {}
    for ordinal in (114, 119, 120, 121):
        files = sorted((OUTPUT / "de/spectator/s" / str(ordinal)).glob("attempt-*.json"))
        attempt = json.loads(files[-1].read_text(encoding="utf-8"))
        command = attempt["command"]
        segment = dict(script=Path(command[-2]).relative_to(ROOT / "🎓️teaching").as_posix(), scene=command[-1])
        state = RUNTIME.segment_state(segment, SETTINGS)
        old = previous.get("sources", {}).get(str(ordinal), {}).get("state", {}).get("sources", {})
        actual = state["sources"]
        delta = [dict(path=path, before=old.get(path), now=actual.get(path))
                 for path in sorted(old.keys() | actual.keys()) if old.get(path) != actual.get(path)] if old else []
        records[str(ordinal)] = dict(attempt=files[-1].name, started=attempt["started"],
                                     state=state, changes_since_observation=delta)
    result = dict(checked=time.time(), previous_checked=previous.get("checked"), sources=records)
    destination.write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    print(json.dumps({key: dict(attempt=value["attempt"], changes=value["changes_since_observation"])
                      for key, value in records.items()}, ensure_ascii=False))

if __name__ == "__main__":
    checkpoint()
