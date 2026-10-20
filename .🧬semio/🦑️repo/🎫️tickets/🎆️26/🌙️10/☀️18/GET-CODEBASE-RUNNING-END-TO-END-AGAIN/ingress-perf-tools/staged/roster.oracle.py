#!/usr/bin/env python3
"""🐍 Independent reference model of the roster and gate fixtures: replays the same steps with a plain event simulation and prints the expected captures, served serials and gate fulls as JSON."""
import json, sys


def simulate(case):
    clock = 0.0
    serial = 0
    latest = None
    running = None
    queued = None
    captures = 0
    served = {}

    def finish(call, number):
        nonlocal latest
        if number in case["rejectSerials"]:
            if latest is not None and latest["serial"] == number:
                latest = None
            request(call, True)
        else:
            served[call["id"]] = number

    def request(call, force=False):
        nonlocal running, queued, captures
        age = call.get("maxAge", 0)
        after = call.get("after", 0)
        if not force and age > 0 and latest is not None and latest["serial"] > after and clock - latest["at"] <= age:
            finish(call, latest["serial"])
        elif queued is not None:
            queued["waiters"].append(call)
        elif running is None:
            running = {"at": clock, "waiters": [call]}
            captures += 1
        else:
            queued = {"at": None, "waiters": [call]}
            captures += 1

    for step in case["steps"]:
        op = step["op"]
        if op == "advance":
            clock += step["ms"]
        elif op == "call":
            request(dict(step))
        elif op == "release":
            entry = running
            serial += 1
            latest = {"serial": serial, "at": entry["at"]}
            running = queued
            if running is not None:
                running["at"] = clock
            queued = None
            for waiter in entry["waiters"]:
                finish(waiter, serial)
    return {"captures": captures, "served": served}


def gate(case):
    verified = None
    fulls = 0
    for at in case["runs"]:
        if case["window"] > 0 and verified is not None and at - verified <= case["window"]:
            continue
        fulls += 1
        verified = at
    return {"fulls": fulls}


fixture = json.load(open(sys.argv[1]))
print(json.dumps({"cases": [simulate(case) for case in fixture["cases"]], "gate": [gate(case) for case in fixture["gate"]]}))
