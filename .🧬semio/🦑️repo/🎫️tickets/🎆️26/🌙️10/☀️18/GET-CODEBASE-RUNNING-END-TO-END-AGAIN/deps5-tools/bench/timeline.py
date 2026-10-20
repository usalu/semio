#!/usr/bin/env python3
import re, sys

PROGRESS = re.compile(r'"resource":"([^"]*?)(?::(\d+))?","mode":"(\w+)","elapsedMs":([\d.]+)')


def frames(path):
    with open(path, "rb") as handle:
        for line in handle:
            match = PROGRESS.search(line.decode("utf8", "replace"))
            if match:
                name, count, mode, elapsed = match.groups()
                yield name, int(count) if count else None, float(elapsed)


def scans(path):
    rows, current = [], None
    for name, count, elapsed in frames(path):
        kind = name if name in ("javascript-installed-paths", "javascript-source-provider") else None
        if kind is None:
            continue
        if current is None or current["kind"] != kind or count < current["count"]:
            if current:
                rows.append(current)
            current = {"kind": kind, "frames": 0, "count": 0, "seconds": 0.0}
        current["frames"] += 1
        current["count"] = count
        current["seconds"] = elapsed / 1000
    if current:
        rows.append(current)
    return rows


def cycles(path):
    blocks, current, between = [], None, {"passes": 0, "pass_ms": 0.0, "admissions": 0, "replays": 0}
    for name, count, elapsed in frames(path):
        if name.startswith("dependencies:store-ownership"):
            if current is None:
                current = {"start": elapsed, "end": elapsed, "before": between}
                between = {"passes": 0, "pass_ms": 0.0, "admissions": 0, "replays": 0}
            current["end"] = elapsed
            continue
        if current is not None:
            blocks.append(current)
            current = None
        if name == "javascript-preserving-source" and count == 308:
            between["passes"] += 1
            between["pass_ms"] += elapsed
        elif name.startswith("resource-directory-admission"):
            between["admissions"] += 1
        elif name == "dependencies:store-input-replay":
            between["replays"] += 1
    if current:
        blocks.append(current)
    return blocks


if __name__ == "__main__":
    log = sys.argv[1]
    print("scan\tframes\tfinal-count\tseconds")
    for row in scans(log):
        print("%s\t%d\t%d\t%.1f" % (row["kind"], row["frames"], row["count"], row["seconds"]))
    print("tree pass (elapsed since sync start)\tgap before (s)\tbindings\tpass total (s)")
    previous_end = 0.0
    for block in cycles(log):
        before = block["before"]
        print("%.0f..%.0f\t%.0f\t%d\t%.0f" % (block["start"] / 1000, block["end"] / 1000, (block["start"] - previous_end) / 1000, before["passes"], before["pass_ms"] / 1000))
        previous_end = block["end"]
