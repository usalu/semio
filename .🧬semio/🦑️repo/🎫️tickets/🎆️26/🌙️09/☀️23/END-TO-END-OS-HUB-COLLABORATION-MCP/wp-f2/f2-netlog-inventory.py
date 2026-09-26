"""🔌️ F2 — per-origin HTTP connection inventory from a Chromium NetLog.

For every URL request: start of its job, the moment its request was sent on a socket (HTTP_TRANSACTION_SEND_REQUEST), its end
(REQUEST_ALIVE end, or open at the end of the log). Per origin: requests, queue time (job start → sent) p50/p95/max and the
number queued ≥ 100 ms / ≥ 1 s, the maximum number of requests concurrently on the wire (sent → end), the long-lived ones
(on the wire ≥ 10 s, with their paths), and the socket-pool stall events (SOCKET_POOL_STALLED_MAX_SOCKETS_PER_GROUP /
_MAX_SOCKETS). WebSockets are listed apart: Chromium pools them separately from the 6-per-origin HTTP/1.1 group.

usage: python3 f2-netlog-inventory.py <netlog.json> [--json out.json]"""
import json, sys, urllib.parse, collections

path = sys.argv[1]
text = open(path, encoding="utf-8").read().rstrip()
if not text.endswith("}"):
    text = text.rstrip(",\n ") + "]}"
log = json.loads(text)
types = {value: key for key, value in log["constants"]["logEventTypes"].items()}
phases = {value: key for key, value in log["constants"]["logEventPhase"].items()}
source_types = {value: key for key, value in log["constants"]["logSourceType"].items()}
requests = {}
stalls = collections.Counter()
stall_times = []
t_last = 0
for event in log["events"]:
    t = int(event["time"])
    t_last = max(t_last, t)
    kind = types.get(event["type"], str(event["type"]))
    phase = phases.get(event.get("phase"), "")
    source = event["source"]
    if kind in ("SOCKET_POOL_STALLED_MAX_SOCKETS_PER_GROUP", "SOCKET_POOL_STALLED_MAX_SOCKETS"):
        stalls[kind] += 1
        stall_times.append(t)
    if source_types.get(source["type"]) != "URL_REQUEST":
        continue
    row = requests.setdefault(source["id"], {"url": None, "start": None, "sent": None, "end": None, "ws": False})
    params = event.get("params", {})
    if kind == "REQUEST_ALIVE" and phase == "PHASE_BEGIN":
        row["start"] = t
    if kind == "URL_REQUEST_START_JOB" and "url" in params:
        row["url"] = row["url"] or params["url"]
        row["start"] = row["start"] if row["start"] is not None else t
    if kind == "HTTP_TRANSACTION_SEND_REQUEST" and phase == "PHASE_BEGIN" and row["sent"] is None:
        row["sent"] = t
    if kind == "REQUEST_ALIVE" and phase == "PHASE_END":
        row["end"] = t
    if kind.startswith("WEBSOCKET"):
        row["ws"] = True

def origin_of(url):
    parsed = urllib.parse.urlsplit(url)
    return f"{parsed.scheme}://{parsed.netloc}"

def quantile(values, q):
    if not values:
        return None
    ordered = sorted(values)
    return ordered[min(len(ordered) - 1, int(q * len(ordered)))]

by_origin = collections.defaultdict(list)
for row in requests.values():
    if row["url"] and row["url"].startswith(("http", "ws")):
        by_origin[origin_of(row["url"])].append(row)
report = {"netlog": path, "spanMs": t_last - min((r["start"] for r in requests.values() if r["start"] is not None), default=t_last), "stalls": dict(stalls), "origins": {}}
for origin, rows in sorted(by_origin.items(), key=lambda item: -len(item[1])):
    http = [row for row in rows if not row["ws"] and not row["url"].startswith("ws")]
    sockets = [row for row in rows if row["ws"] or row["url"].startswith("ws")]
    queues = [row["sent"] - row["start"] for row in http if row["sent"] is not None and row["start"] is not None]
    wire = sorted([(row["sent"], 1) for row in http if row["sent"] is not None] + [((row["end"] if row["end"] is not None else t_last + 1), -1) for row in http if row["sent"] is not None])
    level = peak = 0
    for _, delta in wire:
        level += delta
        peak = max(peak, level)
    long_lived = [row for row in http if row["sent"] is not None and ((row["end"] if row["end"] is not None else t_last) - row["sent"]) >= 10_000]
    never_sent = [row for row in http if row["sent"] is None and row["end"] is None]
    report["origins"][origin] = {
        "requests": len(http),
        "queueMs": {"p50": quantile(queues, 0.5), "p95": quantile(queues, 0.95), "max": max(queues, default=None)},
        "queued100ms": sum(1 for q in queues if q >= 100),
        "queued1s": sum(1 for q in queues if q >= 1000),
        "neverSentAtEnd": len(never_sent),
        "peakOnWire": peak,
        "longLived": [f"{((row['end'] if row['end'] is not None else t_last) - row['sent']) // 1000}s{'' if row['end'] is not None else '+open'} {urllib.parse.unquote(urllib.parse.urlsplit(row['url']).path)[:80]}" for row in long_lived],
        "openAtEnd": sum(1 for row in long_lived if row["end"] is None),
        "websockets": [f"{urllib.parse.unquote(urllib.parse.urlsplit(row['url']).path)[:80]}{' (open)' if row['end'] is None else ''}" for row in sockets],
    }
out = json.dumps(report, indent=1, ensure_ascii=False)
if "--json" in sys.argv:
    open(sys.argv[sys.argv.index("--json") + 1], "w", encoding="utf-8").write(out)
for origin, row in report["origins"].items():
    print(f"{origin}: requests={row['requests']} queue p50/p95/max={row['queueMs']['p50']}/{row['queueMs']['p95']}/{row['queueMs']['max']} ms, queued≥100ms={row['queued100ms']} ≥1s={row['queued1s']}, peak on wire={row['peakOnWire']}, long-lived={len(row['longLived'])} (open at end {row['openAtEnd']}), never sent={row['neverSentAtEnd']}, websockets={len(row['websockets'])}")
    for line in row["longLived"][:12]:
        print("   ", line)
    for line in row["websockets"][:8]:
        print("    ws", line)
print("stalls:", dict(stalls))
