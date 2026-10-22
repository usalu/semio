#!/usr/bin/env python3
"""🐍️ Independent model of the deferred optional accounting: replays the steps of every case of fixture.json with plain integers and prints the expectation (`--write` stores it into the fixture, otherwise the fixture's expectation is compared and the exit code says whether they agree). Stdlib only."""
import json, pathlib, re, sys

HERE = pathlib.Path(__file__).resolve().parent
PATH = next(path for path in (HERE / "🔣️.json", HERE / "fixture.json") if path.exists())


def replay(case):
    q, p = case["quota"], case["policy"]
    used, pending, available = q["usedBytes"], 0, q["availableBytes"]
    owed_bytes = owed_leaves = since = now = 0
    grants, results, ledger, removed, refused = [], [], [], [], []
    taken, declined = set(), set()
    immutable = re.compile(r"(?:^|/)ob/[a-f0-9]{64}\.br$")

    def reserve(peak):
        nonlocal pending
        if used + pending + peak > q["maximumBytes"] or available - pending - peak < q["minimumAvailableBytes"]:
            return False
        pending += peak
        return True

    def commit(peak, delta):
        nonlocal pending, used, available
        pending -= peak
        used += delta
        available -= delta

    def owe(delta):
        nonlocal owed_bytes, owed_leaves, since
        if owed_leaves == 0:
            since = now
        owed_bytes += delta
        owed_leaves += 1

    def flush(final, accepts):
        nonlocal owed_bytes, owed_leaves
        if owed_leaves == 0:
            return True
        for _ in range(p["finalFlushAttempts"] if final else 1):
            ledger.append({"peakBytes": max(owed_bytes, 0), "retainedDeltaBytes": owed_bytes, "accepted": accepts})
            if accepts:
                owed_bytes = owed_leaves = 0
                return True
        return False

    def maybe(accepts):
        if owed_bytes >= p["flushBytes"] or owed_leaves >= p["flushLeaves"] or now - since >= p["flushMilliseconds"]:
            flush(False, accepts)

    for step in case["steps"]:
        op, accepts = step["op"], step.get("ledgerAccepts", True)
        if op == "publish":
            size = step["bytes"]
            if not reserve(size):
                results.append("declined")
            elif not step.get("deferred", False):
                ledger.append({"peakBytes": size, "retainedDeltaBytes": size, "accepted": accepts})
                pending -= size
                if accepts:
                    used += size
                    available -= size
                    results.append("published")
                else:
                    results.append("declined")
            else:
                commit(size, size)
                owe(size)
                maybe(accepts)
                results.append("deferred")
        elif op == "adopt":
            kept = 0
            for leaf in step["leaves"]:
                fixed = immutable.search(leaf["name"]) is not None
                if fixed and (leaf["name"] in taken or leaf["name"] in declined):
                    continue
                if not reserve(leaf["bytes"]):
                    if fixed:
                        declined.add(leaf["name"])
                    removed.append(leaf["name"])
                    refused.append(leaf["name"])
                    continue
                if fixed:
                    taken.add(leaf["name"])
                commit(leaf["bytes"], leaf["bytes"])
                owe(leaf["bytes"])
                kept += 1
            if kept:
                maybe(True)
            results.append("adopted:%d" % kept)
        elif op == "clock":
            now += step["milliseconds"]
            results.append("clock")
        elif op == "flush":
            results.append("flushed" if flush(step.get("final", False), accepts) else "postponed")
        elif op == "grant":
            if reserve(step["bytes"]):
                grants.append(step["bytes"])
                results.append("granted")
            else:
                results.append("refused")
        elif op == "release":
            pending -= grants.pop()
            results.append("released")
    return {"results": results, "ledger": ledger, "used": used, "owedBytes": owed_bytes, "owedLeaves": owed_leaves, "removed": removed, "refused": refused}


def main():
    document = json.loads(PATH.read_text(encoding="utf8"))
    bad = 0
    for case in document["cases"]:
        model = replay(case)
        if "--write" in sys.argv:
            case["expect"] = model
        elif model != case["expect"]:
            bad += 1
            print("DISAGREE", case["name"], json.dumps(model), json.dumps(case["expect"]))
    if "--write" in sys.argv:
        PATH.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n", encoding="utf8")
    print("cases", len(document["cases"]), "disagreements", bad)
    sys.exit(1 if bad else 0)


main()
