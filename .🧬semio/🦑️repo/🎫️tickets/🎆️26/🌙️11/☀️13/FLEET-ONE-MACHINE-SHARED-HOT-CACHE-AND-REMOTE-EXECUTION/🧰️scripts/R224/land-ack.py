#!/usr/bin/env python3
"""Delivery acknowledgement of a landing: waits until the replica daemon of this machine published the new files and until every healthy peer attests that it read them and applied everything it read (cursor at or after the head that holds the files, empty incoming queue). Bounded: after the limit it reports who did not acknowledge and exits 0; exit 5 only when the files were never published. / Lieferbestätigung einer Landung."""
import json, os, sys, time, unicodedata

def read(path):
    try:
        with open(path, encoding="utf8") as handle:
            return json.load(handle)
    except (OSError, ValueError):
        return None

def main(argv):
    store, origin, listing, before_file, limit = argv[1], argv[2], argv[3], argv[4], float(argv[5])
    poll = float(argv[6]) if len(argv) > 6 else 2.0
    publish_limit = min(60.0, limit)
    now_ms = lambda: int(time.time() * 1000)
    wanted = {unicodedata.normalize("NFC", line.rstrip("\n")) for line in open(listing, encoding="utf8") if line.strip()}
    before = read(before_file) or {"segment": 0, "bytes": 0}
    base = os.path.join(store, "v1", "origins")
    start = time.time()
    while True:
        head = read(os.path.join(base, origin, "head.json")) or before
        found = set()
        for segment in range(before["segment"], head["segment"] + 1):
            name = os.path.join(base, origin, "segments", "%012d.jsonl" % segment)
            try:
                with open(name, "rb") as handle:
                    handle.seek(before["bytes"] if segment == before["segment"] else 0)
                    for line in handle:
                        try:
                            event = json.loads(line)
                        except ValueError:
                            continue
                        if event.get("operation") == "put":
                            found.add(unicodedata.normalize("NFC", event["path"]))
            except OSError:
                pass
        if wanted <= found:
            break
        if time.time() - start > publish_limit:
            print("unpublished " + str(len(wanted - found)))
            return 5
        time.sleep(poll)
    target = (head["segment"], head["bytes"])
    awaited, rows = set(), []
    for peer in sorted(os.listdir(base)):
        if peer == origin:
            continue
        attest = read(os.path.join(base, peer, "attest.json"))
        if attest is None or not attest.get("sync", False):
            rows.append("skipped %s off-or-unattested" % peer)
        elif now_ms() - attest.get("wallMs", 0) > 60000:
            rows.append("skipped %s stale-attestation" % peer)
        elif attest.get("queue", {}).get("incoming", 0) > 50:
            rows.append("skipped %s backlog-%d" % (peer, attest["queue"]["incoming"]))
        else:
            awaited.add(peer)
    acked = {}
    while True:
        for peer in sorted(awaited - set(acked)):
            attest = read(os.path.join(base, peer, "attest.json")) or {}
            cursor = attest.get("cursors", {}).get(origin, {"segment": 0, "bytes": 0})
            if (cursor["segment"], cursor["bytes"]) >= target and attest.get("queue", {}).get("incoming", 1) == 0:
                acked[peer] = round(time.time() - start)
        if set(acked) == awaited or time.time() - start > limit:
            break
        time.sleep(poll)
    for peer in sorted(acked):
        rows.append("acked %s %ds" % (peer, acked[peer]))
    for peer in sorted(awaited - set(acked)):
        rows.append("lagging %s" % peer)
    print("; ".join(rows) if rows else "no peers")
    return 0

sys.exit(main(sys.argv))
