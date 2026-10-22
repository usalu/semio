"""👁️ Independent Python oracle of the Bazel closure evidence (standard library only): it derives the label map, the queried labels and the declared environment names from a task plan and a query answer. Requests arrive as JSON on stdin."""
import json, re, sys


def closure(row):
    tasks = row["plan"]["tasks"]
    label = lambda task: "//:" + tasks[task]["label"]
    queried = set()
    for line in row["answer"].split("\n"):
        line = re.sub(r"\s+\(.*\)\s*$", "", line).strip()
        if line.startswith("//"):
            queried.add(line)
    return {"version": 1, "labelToTask": {label(task): task for task in tasks}, "queried": sorted(queried),
            "declaredEnvironment": {label(task): sorted((tasks[task]["env"] or [])) for task in tasks}}


print(json.dumps([closure(row) for row in json.load(sys.stdin)["rows"]], ensure_ascii=False))
