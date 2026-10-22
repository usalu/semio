"""🧱️ Independent Python oracle of the Bazel host resource rendering (standard library only): the local concurrency as a closed form, the granted worker host as a minimum, and the Starlark resource sets executed as Python (Starlark is a Python subset) to read the estimates they return. Requests arrive as JSON on stdin."""
import json, math, sys


def concurrency(jobs, cpu, memory, action_cpu, action_memory, count):
    if count < 1 or jobs < 1:
        return 0
    bounds = [math.inf if action_cpu == 0 else cpu // action_cpu, math.inf if action_memory == 0 else memory // action_memory]
    return int(min(count, jobs, max(1, min(bounds))))


def host(plan, machine):
    if plan is None:
        return machine
    return {"cores": min(machine["cores"], plan["cpu"]), "memoryBytes": min(machine["memoryBytes"], (plan["memoryMegabytes"] + plan["fixedMegabytes"]) * 1048576)}


def resource_sets(text):
    scope = {}
    exec(compile(text, "resource_sets.bzl", "exec"), scope)
    return {name: scope[function.__name__](None, None) for name, function in scope["_RESOURCE_SETS"].items()}


def main():
    request = json.load(sys.stdin)
    op = request["op"]
    if op == "concurrency":
        print(json.dumps([concurrency(*row) for row in request["rows"]]))
    elif op == "host":
        print(json.dumps([host(row["plan"], row["host"]) for row in request["rows"]]))
    else:
        print(json.dumps([resource_sets(text) for text in request["texts"]]))


main()
