#!/usr/bin/env python3
"""Independent oracle of the concurrent dependency generation contract: pointer evolution, retention decisions, wait scheduling and message language.

usage: concurrent.oracle.py <pointer|collect|wait|language|admission> < cases.json > result.json
It reads only the schema constants and the cases; it shares no code with the TypeScript implementation.
"""
import hashlib
import json
import os
import re
import sys

SCHEMA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧬️schema", "🔣️.json")
with open(SCHEMA, encoding="utf-8") as handle:
    PROTOCOL = json.load(handle)["$defs"]["Protocol"]["const"]


def canonical(value):
    if isinstance(value, dict):
        return "{" + ",".join(json.dumps(key, ensure_ascii=False) + ":" + canonical(value[key]) for key in sorted(value, key=lambda k: k.encode("utf-16-be"))) + "}"
    if isinstance(value, list):
        return "[" + ",".join(canonical(item) for item in value) + "]"
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def seal(previous, scope, generation, now, size):
    if previous is not None and previous["generation"] == generation:
        return previous
    at = max(now, previous["publishedAtMs"] if previous else 0)
    retired = []
    if previous is not None:
        retired = [row for row in previous["retired"] if row["generation"] not in (generation, previous["generation"])]
        retired.append({"generation": previous["generation"], "atMs": at})
    retired.sort(key=lambda row: (row["atMs"], row["generation"].encode("utf-8")))
    retired = retired[-PROTOCOL["pointer"]["maximumRetired"]:]
    body = {"version": 1, "purpose": "dependency-generation-pointer", "scope": scope, "generation": generation, "epoch": (previous["epoch"] if previous else 0) + 1, "publishedAtMs": at, "bytes": size, "retired": retired}
    return {**body, "sha256": sha(canonical(body))}


def pointer(cases):
    out = []
    for case in cases:
        state, rows = None, []
        for step in case["steps"]:
            state = seal(state, case["scope"], step["generation"], step["nowMs"], step["bytes"])
            rows.append(state)
        out.append({"name": case["name"], "pointers": rows, "file": sha(case["scope"])[:PROTOCOL["pointer"]["scopeKeyUnits"]] + PROTOCOL["pointer"]["suffix"]})
    return out


def collect(cases):
    policy = PROTOCOL["retention"]
    out = []
    for case in cases:
        pressure = case["freeFraction"] < policy["pressureFreeFraction"] or ("neededBytes" in case and "freeBytes" in case and case["freeBytes"] < case["neededBytes"])
        retired_at, rollback = {}, set()
        for pointer_row in case["pointers"]:
            for row in pointer_row["retired"]:
                key = row["generation"][:20]
                retired_at[key] = max(retired_at.get(key, 0), row["atMs"])
            if pointer_row["retired"]:
                rollback.add(max(pointer_row["retired"], key=lambda row: row["atMs"])["generation"][:20])
        rows = []
        for row in case["rows"]:
            since = retired_at.get(row["name"], row["modifiedAtMs"])
            age = max(0, case["nowMs"] - since)
            is_rollback = row["name"] in rollback
            if pressure:
                needed = policy["pressureGraceMilliseconds"]
            elif is_rollback:
                needed = max(policy["graceMilliseconds"], policy["rollbackMilliseconds"])
            else:
                needed = policy["graceMilliseconds"]
            rows.append((since, row["name"], age, needed, is_rollback))
        rows.sort(key=lambda row: (row[0], row[1].encode("utf-8")))
        due = [row for row in rows if row[2] >= row[3]]
        keep = [{"name": row[1], "reason": "rollback" if row[4] else "grace"} for row in rows if row[2] < row[3]]
        out.append({"name": case["name"], "collect": [row[1] for row in due[:policy["maximumCollectedPerRun"]]], "keep": keep})
    return out


def admission(cases):
    spec = PROTOCOL["admission"]
    out = []
    for case in cases:
        own = next((row["bytes"] for row in case["pointers"] if row["scope"] == case["scope"]), 0)
        largest = max([row["bytes"] for row in case["pointers"]] or [0])
        needed = own if own > 0 else largest if largest > 0 else spec["defaultGenerationBytes"]
        required = -(-needed * spec["marginPercent"] // 100)
        out.append({"name": case["name"], "neededBytes": needed, "requiredBytes": required, "freeBytes": case["freeBytes"], "admitted": case["freeBytes"] >= required})
    return out


def wait(cases):
    spec = PROTOCOL["wait"]
    return [{"name": case["name"], "expired": case["elapsedMs"] >= case["maximumMs"], "recheck": bool(case["canRecheck"]) and case["sinceRecheckMs"] >= spec["recheckMilliseconds"], "progress": case["sinceProgressMs"] >= spec["progressMilliseconds"]} for case in cases]


def language(cases):
    out = []
    for case in cases:
        picked = PROTOCOL["languages"][0]
        for key in ("SEMIO_LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"):
            value = case["environment"].get(key)
            tag = re.split(r"[._@-]", value.lower())[0] if value else None
            if tag and tag in PROTOCOL["languages"]:
                picked = tag
                break
        out.append({"name": case["name"], "language": picked})
    return out


if __name__ == "__main__":
    command = sys.argv[1] if len(sys.argv) > 1 else ""
    handlers = {"pointer": pointer, "collect": collect, "wait": wait, "language": language, "admission": admission}
    if command not in handlers:
        raise SystemExit(__doc__)
    json.dump(handlers[command](json.load(sys.stdin)), sys.stdout, ensure_ascii=False)
