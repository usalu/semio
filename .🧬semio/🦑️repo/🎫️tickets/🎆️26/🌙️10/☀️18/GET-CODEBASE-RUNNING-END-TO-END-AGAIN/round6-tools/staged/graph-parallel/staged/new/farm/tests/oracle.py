"""🐍️ Independent model of the graph farm laws (stdlib only): the sizing of the farm, the language choice, the message rendering, the runtime heap environment and the intern tables. It never imports the JavaScript implementation.

usage: oracle.py expect FIXTURE   prints the expected answers of every table as JSON"""
import json, math, re, sys


def language(policy, environment):
    for name in policy["languageEnvironment"]:
        value = environment.get(name)
        if value is None:
            continue
        head = re.split(r"[_.@-]", value)[0].lower()
        if head in policy["languages"]:
            return [head]
    return list(policy["languages"])


def render(policy, messages, key, fields, environment):
    return [re.sub(r"\{([a-z]+)\}", lambda match: str(fields[match.group(1)]), messages[lang][key]) for lang in language(policy, environment)]


def plan(policy, case):
    environment = case.get("environment", {})
    named = environment.get(policy["workerMegabytesEnvironment"])
    named = named.strip() if named is not None else None
    if named not in (None, "") and (not re.fullmatch(r"[0-9]+", named) or int(named) < policy["minimumWorkerMegabytes"]):
        return {"error": True}
    megabytes = policy["workerMegabytes"] if named in (None, "") else int(named)
    base = {"workerMegabytes": megabytes}
    text = environment.get(policy["workersEnvironment"])
    text = text.strip().lower() if text is not None else None
    if text in ("0", "off", "false"):
        return {"workers": 0, "reason": "disabled", **base}
    if text not in (None, "", "auto"):
        if not re.fullmatch(r"[0-9]+", text) or int(text) > policy["maximumWorkers"]:
            return {"error": True}
        forced = int(text)
        return {"workers": forced, "reason": "disabled" if forced == 0 else "forced", **base}
    if case["projects"] < policy["minimumProjects"]:
        return {"workers": 0, "reason": "small", **base}
    host = case["host"]
    budget = min(host["totalMegabytes"] * policy["hostShareBasisPoints"] / 10000, host.get("availableMegabytes", math.inf))
    reserve = max(case.get("parentMegabytes", 0), policy["parentReserveMegabytes"])
    bounds = [
        ("work", math.floor(case["projects"] / policy["projectsPerWorker"])),
        ("memory", math.floor((budget - reserve) / megabytes)),
        ("cpu", host["cores"] - policy["cpuReserve"]),
        ("limit", policy["maximumWorkers"]),
    ]
    best = bounds[0]
    for row in bounds[1:]:
        if row[1] < best[1]:
            best = row
    if best[1] < 2:
        return {"workers": 0, "reason": "single", **base}
    return {"workers": best[1], "reason": best[0], **base}


def runtime_environment(policy, runtime, megabytes):
    row = policy["runtime"].get(runtime)
    if row is None:
        return {}
    environment = dict(row["environment"])
    if runtime == "bun":
        environment["BUN_JSC_forceRAMSize"] = str(math.floor(megabytes * row["ramFactor"]) * 1048576)
    elif runtime == "node":
        environment["NODE_OPTIONS"] = "--max-old-space-size=" + str(math.floor(megabytes * row["ramFactor"]))
    return environment


def intern(batches):
    seen, out = {}, []
    for batch in batches:
        ids, fresh = [], []
        for key in batch:
            if key not in seen:
                seen[key] = len(seen) + 1
                fresh.append(key)
            ids.append(seen[key])
        out.append({"ids": ids, "fresh": fresh})
    return out


def expect(fixture):
    policy, messages = fixture["policy"], fixture["messages"]
    return {
        "plan": [plan(policy, row["input"]) for row in fixture["plan"]],
        "languages": [language(policy, row["environment"]) for row in fixture["languages"]],
        "messages": [render(policy, messages, row["key"], row["fields"], row["environment"]) for row in fixture["messageCases"]],
        "runtime": [runtime_environment(policy, row["runtime"], row["megabytes"]) for row in fixture["runtime"]],
        "tables": [intern(row["batches"]) for row in fixture["tables"]],
    }


if __name__ == "__main__":
    if len(sys.argv) != 3 or sys.argv[1] != "expect":
        raise SystemExit(__doc__)
    print(json.dumps(expect(json.load(open(sys.argv[2], encoding="utf8"))), ensure_ascii=False))
