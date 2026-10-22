"""📝 Authoring aid: writes the Nx host resource corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
plan = lambda jobs, cpu=2, memory=3072, fixed=1536, floor=4096: {"version": 1, "id": "p", "jobs": jobs, "cpu": cpu, "memoryMegabytes": memory, "fixedMegabytes": fixed, "floorMegabytes": floor}
c = {"version": 1, "plans": [{"name": name, "plan": plan(jobs), "parallel": str(jobs)} for name, jobs in [("one job", 1), ("two jobs", 2), ("a dozen jobs", 12), ("the largest grant", 1024)]]}
c["plans"].append({"name": "other fields do not change the parallelism", "plan": plan(3, 64, 999999, 1, 2), "parallel": "3"})
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
