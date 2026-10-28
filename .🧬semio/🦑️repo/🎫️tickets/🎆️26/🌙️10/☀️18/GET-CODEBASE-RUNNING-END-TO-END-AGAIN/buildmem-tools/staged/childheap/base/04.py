"""🐍️ Independent model of the graph memory laws (stdlib only): the admission decision, the budget resolution, the language choice, the message rendering and a checker for governor traces. It never imports the JavaScript implementation.

usage: 🐍️.py expect FIXTURE   prints the expected verdicts of every table as JSON
       🐍️.py trace FIXTURE TRACE   checks one recorded governor trace against its scenario and prints a list of violations (empty list = lawful)"""
import json, math, re, sys

MEGABYTE = 1048576


def decide(policy, state):
    if state["running"] >= state["workers"]:
        return {"admit": False, "collect": False, "reason": "workers"}
    limit = state["budget"] * policy["resumeRatio"] if state["blocked"] else state["budget"]
    if state["used"] <= limit:
        return {"admit": True, "collect": False, "reason": "within"}
    if not state["collected"]:
        return {"admit": False, "collect": True, "reason": "collect"}
    if state["running"] == 0:
        return {"admit": True, "collect": False, "reason": "forced"}
    return {"admit": False, "collect": False, "reason": "over"}


def budget(policy, environment, host):
    text = environment.get(policy["budgetEnvironment"])
    if text is not None:
        try:
            value = float(text)
        except ValueError:
            return {"error": True}
        if not math.isfinite(value) or value < policy["minimumBudgetMegabytes"]:
            return {"error": True}
        return {"bytes": math.floor(value) * MEGABYTE}
    try:
        plan = json.loads(environment.get(policy["planEnvironment"], "null"))
    except ValueError:
        plan = None

    def part(value):
        return value if isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value) and value > 0 else 0
    granted = part(plan.get("memoryMegabytes")) + part(plan.get("fixedMegabytes")) if isinstance(plan, dict) else 0
    derived = math.floor(granted * policy["grantShare"]) if granted > 0 else math.floor(part(host) * policy["hostShare"])
    return {"bytes": max(policy["minimumBudgetMegabytes"], min(policy["maximumBudgetMegabytes"], derived)) * MEGABYTE}


def languages(policy, environment):
    for name in policy["languageEnvironment"]:
        value = environment.get(name)
        if value:
            head = re.split(r"[_.@-]", value)[0].lower()
            if head in policy["languages"]:
                return [head]
    return list(policy["languages"])


def render(policy, messages, key, fields, environment):
    return [re.sub(r"\{([a-z]+)\}", lambda match: str(fields[match.group(1)]), messages[language][key]) for language in languages(policy, environment)]


def expect(fixture):
    policy, messages = fixture["policy"], fixture["messages"]
    return {
        "admission": [decide(policy, row["state"]) for row in fixture["admission"]],
        "budgets": [budget(policy, row["environment"], row["hostMegabytes"]) for row in fixture["budgets"]],
        "languages": [languages(policy, row["environment"]) for row in fixture["languages"]],
        "messages": [render(policy, messages, row["key"], row["fields"], row["environment"]) for row in fixture["messageCases"]],
    }


def check_trace(fixture, trace):
    """✅️ Invariants of a governor run: arrival order is start order, never more than `workers` run, a start beside running projects happens only within the budget, every project completes, collections are rate limited, waiting lines are spaced, the serial line appears once."""
    policy = fixture["policy"]
    scenario = next(row for row in fixture["governor"] if row["name"] == trace["scenario"])
    budget_bytes = scenario["budgetMegabytes"] * MEGABYTE
    baseline_bytes = scenario.get("baselineMegabytes", 0) * MEGABYTE
    problems = []
    starts = trace["starts"]
    if scenario.get("abortAtMilliseconds") is None:
        if [row["task"] for row in starts] != list(range(len(scenario["tasks"]))):
            problems.append("start order is not arrival order")
        if len(trace["completed"]) != len(scenario["tasks"]):
            problems.append("not every project completed")
    else:
        if trace["rejected"] != trace["expectRejected"]:
            problems.append("waiters were not rejected by the abort")
        if any(row["at"] > scenario["abortAtMilliseconds"] for row in starts):
            problems.append("a project started after the abort")
    events = sorted([(row["at"], 1) for row in starts] + [(row["at"], -1) for row in trace["finished"]], key=lambda item: (item[0], item[1]))
    running, peak = 0, 0
    for _, delta in events:
        running += delta
        peak = max(peak, running)
    if peak > scenario["workers"]:
        problems.append("more projects ran than workers: %d" % peak)
    for row in starts:
        if row["runningBefore"] > 0 and row["usedBytes"] - baseline_bytes > budget_bytes:
            problems.append("task %d started beside running projects at %d bytes over budget %d" % (row["task"], row["usedBytes"], budget_bytes))
    if scenario.get("expectParallel") and peak < scenario["expectParallel"]:
        problems.append("expected at least %d projects in parallel, saw %d" % (scenario["expectParallel"], peak))
    if scenario.get("expectSerial") and peak != 1:
        problems.append("a budget exceeded by retained memory must run one project at a time, saw %d" % peak)
    duration = trace["durationMilliseconds"]
    if trace["collections"] > duration // policy["collectMinimumMilliseconds"] + 1:
        problems.append("collections are not rate limited: %d in %d ms" % (trace["collections"], duration))
    waiting = [row["at"] for row in trace["messages"] if row["key"] == "waiting"]
    if any(b - a < policy["progressMilliseconds"] for a, b in zip(waiting, waiting[1:])):
        problems.append("waiting lines are closer than the progress interval")
    serial = [row for row in trace["messages"] if row["key"] == "serial"]
    if scenario.get("expectSerial") and len(serial) != 1:
        problems.append("the serial line must appear exactly once, saw %d" % len(serial))
    if not scenario.get("expectSerial") and serial:
        problems.append("unexpected serial line")
    if scenario.get("expectWaits") is not None and (trace["stats"]["waits"] > 0) != scenario["expectWaits"]:
        problems.append("waits expectation violated: %s" % trace["stats"]["waits"])
    return problems


if __name__ == "__main__":
    command = sys.argv[1]
    fixture = json.load(open(sys.argv[2], encoding="utf8"))
    if command == "expect":
        print(json.dumps(expect(fixture), ensure_ascii=False))
    elif command == "trace":
        print(json.dumps(check_trace(fixture, json.load(open(sys.argv[3], encoding="utf8")))))
    else:
        raise SystemExit("unknown command")
