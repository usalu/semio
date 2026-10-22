"""📝 Authoring aid: writes the Nx worker mapping corpus (corpus.json)."""
import copy, json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
KINDS = ["test", "check", "generate", "build", "script", "service"]
base_kinds = {k: {"workers": False, "maxInstances": 1, "multiplex": False, "inProcess": False} for k in KINDS}


def plan(name, enabled=True, jobs=4, kinds=None):
    row = copy.deepcopy(base_kinds)
    for kind, change in (kinds or {}).items():
        row[kind].update(change)
    return {"name": name, "plan": {"version": 1, "enabled": enabled, "protocol": "json", "localJobs": jobs, "memory": {"totalMegabytes": 1024, "perWorkerMegabytes": 512, "instances": 2}, "sandboxing": False, "cancellation": True, "resetAfterRequests": 256, "kinds": row}}


plans = [plan("no kind uses workers", True, 8), plan("workers disabled globally", False, 2, {"test": {"workers": True, "multiplex": True}})]
for kind in KINDS:
    plans.append(plan("only " + kind + " uses a worker", True, 3, {kind: {"workers": True, "maxInstances": 2, "inProcess": kind != "service"}}))
    plans.append(plan("only " + kind + " multiplexes", True, 3, {kind: {"workers": True, "maxInstances": 2, "multiplex": True}}))
    plans.append(plan("only " + kind + " multiplexes without a worker", True, 3, {kind: {"workers": False, "multiplex": True}}))
plans.append(plan("every kind uses a multiplexing worker", True, 16, {k: {"workers": True, "maxInstances": 4, "multiplex": True} for k in KINDS}))
plans.append(plan("every kind uses a plain worker", True, 1, {k: {"workers": True, "maxInstances": 1} for k in KINDS}))
plans.append(plan("the largest job count", True, 1024))
executors = [
    {"name": "no executors file", "value": None, "expected": False},
    {"name": "not an object", "value": "executors", "expected": False},
    {"name": "an array", "value": [], "expected": False},
    {"name": "no executors section", "value": {}, "expected": False},
    {"name": "an executors section that is not an object", "value": {"executors": 5}, "expected": False},
    {"name": "an empty executors section", "value": {"executors": {}}, "expected": False},
    {"name": "executors without batch", "value": {"executors": {"a": {"implementation": "./a"}, "b": {"implementation": "./b"}}}, "expected": False},
    {"name": "one executor with a batch implementation", "value": {"executors": {"a": {"implementation": "./a"}, "b": {"batchImplementation": "./b"}}}, "expected": True},
    {"name": "a batch implementation that is not text", "value": {"executors": {"a": {"batchImplementation": 5}}}, "expected": False},
    {"name": "a null batch implementation", "value": {"executors": {"a": {"batchImplementation": None}}}, "expected": False},
    {"name": "a null executor", "value": {"executors": {"a": None, "b": {"batchImplementation": "./b"}}}, "expected": True},
    {"name": "an executor that is a string", "value": {"executors": {"a": "batchImplementation"}}, "expected": False},
    {"name": "an empty batch implementation text still counts", "value": {"executors": {"a": {"batchImplementation": ""}}}, "expected": True},
    {"name": "a nested batch implementation does not count", "value": {"executors": {"a": {"x": {"batchImplementation": "./b"}}}}, "expected": False},
]
json.dump({"version": 1, "plans": plans, "executors": executors}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(plans))
