"""🛰️ W13: samples, read-only, every worker's ready.json and slice-memory.json from the replica origins (taken, ledger, fleet counters, presence body operations, phase, ready tail) and appends one JSON line per worker per sample to argv[1]; argv[2] samples every argv[3] seconds."""
import json, os, sys, time
ORIGINS = "/Users/Shared/semio-store/v1/origins"
WORKERS = ["g1-m01", "g1-m02", "g1-m03", "g1-m06", "g1-m07", "g1-m08", "g1-m10", "g1-m11", "g1-m12", "g1-m13"]
def read(path):
    try:
        with open(path) as handle: return json.load(handle)
    except Exception: return None
def sample(now):
    rows = []
    for worker in WORKERS:
        ready, report = read(f"{ORIGINS}/{worker}/ready.json") or {}, read(f"{ORIGINS}/{worker}/slice-memory.json") or {}
        owners = (report.get("sources") or {}).get("owners") or {}
        presence, looks, fleet = owners.get("presence") or {}, owners.get("looks") or {}, owners.get("fleet") or {}
        rows.append({"t": round(now), "w": worker, "state": ready.get("state"), "ready": ready.get("ready"), "stage": ready.get("stage"), "reason": (ready.get("reason") or {}).get("code"),
            "tail": (ready.get("tail") or [""])[-1][:240], "reportAge": None if "atMs" not in report else round(now - report["atMs"] / 1000), "pid": report.get("pid"), "up": report.get("uptimeSeconds"),
            "phase": report.get("phase"), "taken": owners.get("taken"), "ledger": owners.get("ledger"), "inflight": owners.get("inflight"), "pass": owners.get("pass"),
            "fleet": {k: v for k, v in fleet.items() if isinstance(v, int) and v}, "gate": {k: v for k, v in (fleet.get("gateTimings") or {}).items() if v.get("started")},
            "timings": {k: [v.get("started"), v.get("active"), v.get("milliseconds")] for k, v in (presence.get("timings") or {}).items()},
            "body": {k: [v.get("started"), v.get("active"), v.get("milliseconds")] for k, v in (presence.get("bodyOperations") or {}).items()},
            "verified": presence.get("verified"), "waiting": (presence.get("mappingQueue") or {}).get("waiting"),
            "looks": [looks.get("count"), looks.get("averageMilliseconds"), (looks.get("phases") or {}).get("authority"), (looks.get("phases") or {}).get("lookup.reusable")]})
    return rows
out, count, every = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
for index in range(count):
    now = time.time()
    with open(out, "a") as handle:
        for row in sample(now): handle.write(json.dumps(row) + "\n")
    if index + 1 < count: time.sleep(every)
