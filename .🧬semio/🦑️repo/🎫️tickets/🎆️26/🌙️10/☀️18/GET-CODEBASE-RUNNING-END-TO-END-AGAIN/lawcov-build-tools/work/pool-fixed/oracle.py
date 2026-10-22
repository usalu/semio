"""🏊️ Independent Python model of the custody session pool (standard library only): it replays scripted steps sequentially against the written rules (shared session, idle close, maximum lifetime when idle, drop on failure, close) and returns the counters after every step plus the session and outcome of every request. Requests arrive as JSON on stdin."""
import json, sys


def replay(scenario):
    idle_ms, maximum = scenario["idleMilliseconds"], scenario["maximumLifetimeMilliseconds"]
    now, current, inflight, idle, opens, closes, sessions, fail_next = 0, None, 0, None, 0, 0, 0, False
    requests, entered, outcomes, series = {}, {}, {}, []

    def drop(entry):
        nonlocal current, closes
        if current is entry:
            current = None
        if entry.get("dropped"):
            return
        entry["dropped"] = True
        if not entry["failed"]:
            closes += 1

    def arm():
        nonlocal idle
        idle = None
        if current is not None and inflight == 0:
            idle = {"deadline": now + idle_ms, "entry": current}

    for step in scenario["steps"]:
        op = step["op"]
        if op == "failNextOpen":
            fail_next = True
        elif op == "with":
            idle = None
            if current is not None and inflight == 0 and now - current["opened"] >= maximum:
                drop(current)
            if current is None:
                entry = {"opened": now, "failed": fail_next, "session": None}
                fail_next = False
                opens += 1
                if not entry["failed"]:
                    sessions += 1
                    entry["session"] = sessions
                    current = entry
                inflight += 1
                if entry["failed"]:
                    drop(entry)
                    inflight -= 1
                    arm()
                    outcomes[step["id"]] = "open-error"
                    series.append([opens, closes, current is not None])
                    continue
            else:
                entry = current
                inflight += 1
            requests[step["id"]] = entry
            entered[step["id"]] = entry["session"]
        elif op in ("release", "fail"):
            entry = requests[step["id"]]
            if op == "fail":
                if current is entry:
                    current = None
                drop(entry)
                outcomes[step["id"]] = "error"
            else:
                outcomes[step["id"]] = "ok"
            inflight -= 1
            arm()
        elif op == "advance":
            target = now + step["ms"]
            if idle is not None and idle["deadline"] <= target:
                now = idle["deadline"]
                fired, idle = idle, None
                if current is fired["entry"] and inflight == 0:
                    drop(fired["entry"])
            now = target
        elif op == "close":
            idle = None
            if current is not None:
                drop(current)
        series.append([opens, closes, current is not None])
    return {"series": series, "entered": entered, "outcomes": outcomes}


request = json.load(sys.stdin)
print(json.dumps([replay(scenario) for scenario in request["scenarios"]]))
