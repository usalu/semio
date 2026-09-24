"""🔎️ S15: from a Chromium net log, every request for a hub plugin-module file — whether it went to the network or was
answered by the HTTP cache — with the bytes read from the network. usage: python3 s15-netlog-modules.py <netlog.json>"""
import json, sys, urllib.parse
log = json.load(open(sys.argv[1], encoding="utf-8"))
types = {value: key for key, value in log["constants"]["logEventTypes"].items()}
by_source = {}
for event in log["events"]:
    source = event["source"]["id"]
    row = by_source.setdefault(source, {"url": None, "network": False, "cache": False, "bytes": 0})
    kind = types.get(event["type"], str(event["type"]))
    params = event.get("params", {})
    if kind == "URL_REQUEST_START_JOB" and "url" in params: row["url"] = params["url"]
    if kind in ("HTTP_TRANSACTION_SEND_REQUEST", "HTTP_TRANSACTION_HTTP2_SEND_REQUEST_HEADERS"): row["network"] = True
    if kind in ("HTTP_CACHE_READ_DATA",): row["cache"] = True
    if kind == "URL_REQUEST_JOB_FILTERED_BYTES_READ": row["bytes"] += params.get("byte_count", 0)
rows = [row for row in by_source.values() if row["url"] and "/_semio/hub/trusted-catalog/plugin-modules/" in row["url"]]
network = [row for row in rows if row["network"]]
cached = [row for row in rows if not row["network"]]
print(f"module requests={len(rows)} network={len(network)} network_bytes={sum(r['bytes'] for r in network)} cache_only={len(cached)} cache_bytes={sum(r['bytes'] for r in cached)}")
for row in sorted(rows, key=lambda r: -r["bytes"])[:8]:
    print(("NET  " if row["network"] else "CACHE"), row["bytes"], urllib.parse.unquote(row["url"]).split("/plugin-modules/")[1][:110])
