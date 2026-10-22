"""👷️ Independent Python oracle of the Nx worker plan rendering (standard library only): the local job count is NX_PARALLEL, batch mode is switched on only when a worker kind multiplexes and an executor can batch, and every other contract field is reported as unsupported or approximate. Requests arrive as JSON on stdin."""
import json, sys

KINDS = ["test", "check", "generate", "build", "script", "service"]


def render(plan, batch_capable):
    multiplexed = plan["enabled"] and any(plan["kinds"][kind]["workers"] and plan["kinds"][kind]["multiplex"] for kind in KINDS)
    environment = {"NX_PARALLEL": str(plan["localJobs"])}
    if multiplexed and batch_capable:
        environment["NX_BATCH_MODE"] = "true"
    none = "Nx forks one process per task"
    memory = "Nx has no memory budget; lower --parallel instead"
    mapping = [
        ("protocol", "unsupported", none), ("localJobs", "exact", "NX_PARALLEL=" + str(plan["localJobs"])), ("resetAfterRequests", "unsupported", none),
        ("sandboxing", "unsupported", "Nx has no task sandbox"), ("cancellation", "approximate", "Nx terminates task process trees on interrupt"),
        ("memory.totalMegabytes", "unsupported", memory), ("memory.perWorkerMegabytes", "unsupported", memory)]
    for kind in KINDS:
        row = plan["kinds"][kind]
        mapping += [("kinds." + kind + ".workers", "unsupported", none), ("kinds." + kind + ".maxInstances", "unsupported", "Nx bounds all tasks together by NX_PARALLEL"), ("kinds." + kind + ".inProcess", "unsupported", none)]
        if row["workers"] and row["multiplex"]:
            mapping.append(("kinds." + kind + ".multiplex", "approximate", "NX_BATCH_MODE=true") if batch_capable else ("kinds." + kind + ".multiplex", "unsupported", "no executor registers a batchImplementation"))
        else:
            mapping.append(("kinds." + kind + ".multiplex", "unsupported", none))
    return {"environment": environment, "mapping": [{"field": f, "support": s, "setting": t} for f, s, t in mapping]}


def batch(executors):
    if not isinstance(executors, dict):
        return False
    rows = executors.get("executors")
    if not isinstance(rows, dict):
        return False
    return any(isinstance(row, dict) and isinstance(row.get("batchImplementation"), str) for row in rows.values())


request = json.load(sys.stdin)
if request["op"] == "render":
    print(json.dumps([render(row["plan"], row["batchCapable"]) for row in request["rows"]]))
else:
    print(json.dumps([batch(value) for value in request["values"]]))
