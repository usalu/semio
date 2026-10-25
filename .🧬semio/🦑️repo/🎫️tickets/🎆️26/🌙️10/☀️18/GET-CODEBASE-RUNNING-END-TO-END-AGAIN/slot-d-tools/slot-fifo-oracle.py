"""Independent oracle for slot-fifo-live.ts: reads the JSONL event log and checks the queue semantics without any repo code."""
import json, sys

rows = [json.loads(line) for line in open(sys.argv[1]) if line.strip()]
verdict, failures = {}, []

starts = {r["name"]: r["t"] for r in rows if r.get("type") == "start" and r["name"].startswith("W")}
claims = [(r["t"], r["name"], r["type"].split(":")[1]) for r in rows if str(r.get("type", "")).startswith("claimed:")]
killed = {"W4"}
expected = [n for n, _ in sorted(starts.items(), key=lambda kv: kv[1]) if n not in killed]
got = [n for _, n, _ in sorted(claims)]
verdict["expected_claim_order"], verdict["observed_claim_order"] = expected, got
if got != expected:
    failures.append(f"claim order {got} != arrival order {expected}")

free_times = [r["t"] for r in rows if r.get("type") == "action" and (r["action"].startswith("SIGKILL the holder") or r["action"].startswith("release s1") or r["action"].startswith("W1 finishes"))]
lat = [round(c[0] - f) for c, f in zip(sorted(claims), sorted(free_times))]
verdict["slot_free_to_claim_ms"] = lat
if any(v > 2000 for v in lat) or len(lat) != 3:
    failures.append(f"a freed slot took longer than 2 s to be claimed: {lat}")

for phase in ("control", "fifo"):
    probes = [r for r in rows if r.get("type") == "probe" and r["phase"] == phase]
    ok = [p for p in probes if p["ok"]]
    verdict[phase] = {"probes": len(probes), "gate_free": len(ok), "max_latency_ms": max((p["latencyMs"] for p in ok), default=None), "max_waiter_files": max((p["waiterFiles"] for p in probes), default=0)}
if verdict["control"]["gate_free"] != 0:
    failures.append("control: the gate was free although the old-style waiter held it")
fifo = verdict["fifo"]
if fifo["gate_free"] != fifo["probes"]:
    failures.append(f"fifo: the gate was not free for {fifo['probes'] - fifo['gate_free']} of {fifo['probes']} probes")
if fifo["max_latency_ms"] is not None and fifo["max_latency_ms"] > 700:
    failures.append(f"fifo: a probe waited {fifo['max_latency_ms']} ms for the gate")
if fifo["max_waiter_files"] < 3:
    failures.append(f"fifo: expected at least 3 simultaneous waiter files, saw {fifo['max_waiter_files']}")
end = [r for r in rows if r.get("type") == "end"]
verdict["waiter_files_left"] = end[0]["waiterFilesLeft"] if end else None
if not end or end[0]["waiterFilesLeft"] != 0:
    failures.append("waiter files left behind")
verdict["failures"] = failures
print(json.dumps(verdict, indent=1))
sys.exit(1 if failures else 0)
