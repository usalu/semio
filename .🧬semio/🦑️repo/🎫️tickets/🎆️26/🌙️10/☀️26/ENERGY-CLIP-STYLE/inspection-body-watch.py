"""👀 Observe only current publisher body failures and completion without launching renderers."""
from pathlib import Path
import json
import time

TICKET = Path(__file__).resolve().parent
OUTPUT = TICKET / "🗑️generated" / "i"
seen = set()
owner_id, epoch = None, time.time()

def read(path):
    """🧾 Read an atomic checkpoint or retry a writer's incomplete diagnostic update."""
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, ValueError):
        return None

def emit(log, value):
    """📣 Retain one meaningful observer event without repeating unchanged state."""
    line = "[DEBUG] " + json.dumps(value, ensure_ascii=False)
    print(line, flush=True)
    log.write(line + "\n")
    log.flush()

with (OUTPUT / "body-watch.log").open("a", encoding="utf-8", buffering=1) as log:
    emit(log, dict(state="observing", started=epoch))
    while True:
        owner = read(OUTPUT / "publish-owner.json") or {}
        if owner.get("pid") != owner_id and owner.get("state") == "current":
            owner_id, epoch = owner["pid"], owner["loaded"]
            emit(log, dict(state="owner", pid=owner_id, loaded=epoch))
        if owner.get("state") == "restart_required":
            key = ("restart", owner.get("pid"), owner.get("reason"))
            if key not in seen:
                seen.add(key)
                emit(log, owner)
        for directory in (OUTPUT / "de" / "spectator" / "s").glob("*"):
            if not directory.is_dir():
                continue
            for path in sorted(directory.glob("attempt-*.json"), reverse=True):
                data = read(path)
                if not data or data.get("started", 0) < epoch:
                    break
                if data.get("exit_code") not in (None, 0) and str(path) not in seen:
                    seen.add(str(path))
                    command = data["command"]
                    emit(log, dict(state="native_failure", ordinal=directory.name,
                        scene=command[-1], script=command[-2], exit_code=data["exit_code"], log=str(path.with_suffix(".log"))))
        states = read(OUTPUT / "publish.json") or {}
        for identity, state in states.items():
            if state.get("state") in ("waiting_for_input_change", "failed"):
                key = (owner_id, identity, state.get("state"), state.get("error"))
                if key not in seen:
                    seen.add(key)
                    emit(log, {**state, "kind": "video_failure", "video": identity})
        complete = read(OUTPUT / "publish-complete.json")
        if complete and complete.get("completed", 0) >= epoch:
            emit(log, complete)
            break
        time.sleep(10)
