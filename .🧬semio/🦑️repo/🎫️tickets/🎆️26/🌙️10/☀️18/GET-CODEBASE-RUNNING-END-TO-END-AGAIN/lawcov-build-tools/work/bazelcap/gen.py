"""📝 Authoring aid: writes the Bazel host capacity corpus (corpus.json)."""
import itertools, json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
plan = lambda jobs, cpu, memory, fixed=1536, floor=4096: {"version": 1, "id": "p", "jobs": jobs, "cpu": cpu, "memoryMegabytes": memory, "fixedMegabytes": fixed, "floorMegabytes": floor}
c = {"version": 1}
c["settings"] = [
    {"name": "a small grant", "plan": plan(2, 2, 3072), "flags": ["--local_resources=cpu=2", "--local_resources=memory=3072"]},
    {"name": "a large grant", "plan": plan(16, 12, 65536, 5120, 8192), "flags": ["--local_resources=cpu=12", "--local_resources=memory=65536"]},
    {"name": "a grant without per job memory", "plan": plan(1, 1, 0, 0, 0), "flags": ["--local_resources=cpu=1", "--local_resources=memory=0"]},
]
c["hosts"] = [
    {"name": "no grant leaves the host as it is", "plan": None, "host": {"cores": 10, "memoryBytes": 34359738368}},
    {"name": "a grant below the host", "plan": plan(2, 2, 3072), "host": {"cores": 10, "memoryBytes": 34359738368}},
    {"name": "a grant above the host cores", "plan": plan(2, 64, 3072), "host": {"cores": 4, "memoryBytes": 34359738368}},
    {"name": "a grant above the host memory", "plan": plan(2, 2, 999999, 1000), "host": {"cores": 10, "memoryBytes": 1048576}},
    {"name": "a grant exactly at the host", "plan": plan(2, 4, 1023, 1), "host": {"cores": 4, "memoryBytes": 1073741824}},
]
c["kinds"] = [
    {"name": "the authored estimates", "kinds": {"test": {"cpu": 1, "megabytes": 1536}, "check": {"cpu": 1, "megabytes": 4096}, "generate": {"cpu": 1, "megabytes": 1024}, "build": {"cpu": 1, "megabytes": 2048}, "script": {"cpu": 1, "megabytes": 768}, "service": {"cpu": 1, "megabytes": 2048}}},
    {"name": "estimates that differ per kind", "kinds": {"test": {"cpu": 2, "megabytes": 100}, "check": {"cpu": 3, "megabytes": 200}, "generate": {"cpu": 4, "megabytes": 300}, "build": {"cpu": 5, "megabytes": 400}, "script": {"cpu": 6, "megabytes": 500}, "service": {"cpu": 7, "megabytes": 600}}},
]
cases = []
for jobs, cpu, memory in itertools.product([0, 1, 2, 4, 16], [1, 2, 3, 8], [1000, 4096, 10000]):
    for acpu, amem in [(1, 512), (1, 2048), (2, 1000), (3, 4096), (8, 8000), (0, 1000), (1, 0), (16, 100000)]:
        for count in [0, 1, 3, 20]:
            cases.append([jobs, cpu, memory, acpu, amem, count])
c["concurrency"] = cases
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False)
print(len(cases))
