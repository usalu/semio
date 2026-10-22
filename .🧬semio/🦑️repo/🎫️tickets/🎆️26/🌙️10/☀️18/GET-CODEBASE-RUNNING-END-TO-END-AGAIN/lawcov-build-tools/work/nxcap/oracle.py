"""🎚️ Independent Python oracle of the Nx host resource rendering (standard library only): the single Nx knob is the number of concurrent tasks, honoured exactly; core and memory budgets are bounded only through it. Requests arrive as JSON on stdin."""
import json, sys


def render(plan):
    jobs = str(plan["jobs"])
    return {"environment": {"NX_PARALLEL": jobs}, "mapping": [
        {"field": "jobs", "support": "exact", "setting": "NX_PARALLEL=" + jobs},
        {"field": "cpu", "support": "approximate", "setting": "bounded by NX_PARALLEL=" + jobs + "; Nx has no core budget"},
        {"field": "memoryMegabytes", "support": "approximate", "setting": "bounded by NX_PARALLEL=" + jobs + "; Nx has no memory budget"},
        {"field": "fixedMegabytes", "support": "unsupported", "setting": "held by the invocation outside Nx"},
        {"field": "floorMegabytes", "support": "unsupported", "setting": "enforced by admission before Nx starts"}]}


print(json.dumps([render(plan) for plan in json.load(sys.stdin)["plans"]]))
