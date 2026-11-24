"""🛡️ Samples every worker's mirrored warden record (`memory.json`) from the group store into a JSONL time series."""
import json, sys, time, glob, os
out, interval = sys.argv[1], float(sys.argv[2]) if len(sys.argv) > 2 else 15
store = "/Users/Shared/semio-store/v1/origins"
seen = {}
while True:
    row = {"atMs": int(time.time() * 1000), "workers": {}}
    for path in sorted(glob.glob(store + "/g1-*/memory.json")):
        origin = os.path.basename(os.path.dirname(path))
        try:
            with open(path) as handle:
                state = json.load(handle)
        except Exception:
            continue
        if state.get("absent"):
            row["workers"][origin] = {"absent": True, "atMs": state.get("atMs")}
            continue
        alarms = [alarm for alarm in state.get("lastAlarms", []) if alarm.get("atMs", 0) > seen.get(origin, 0)]
        if alarms:
            seen[origin] = max(alarm["atMs"] for alarm in alarms)
        row["workers"][origin] = {"atMs": state["atMs"], "mode": state["mode"], "level": state["level"], "host": state["host"], "guard": state["guardMegabytes"], "danger": state["dangerMegabytes"], "subjects": state["subjects"], "suspended": len(state["suspended"]), "counters": state["counters"], "overhead": state["overhead"], "alarms": alarms}
    with open(out, "a") as handle:
        handle.write(json.dumps(row, ensure_ascii=False) + "\n")
    time.sleep(interval)
