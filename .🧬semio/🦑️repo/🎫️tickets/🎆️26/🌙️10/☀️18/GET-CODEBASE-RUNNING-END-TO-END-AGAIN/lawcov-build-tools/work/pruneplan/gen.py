"""📝 Authoring aid: writes the prune plan corpus (corpus.json): hand-written scenarios with authored expectations, deterministic pseudo-random scenarios from a fixed seed and byte counts for the size formatter."""
import json, os, random

W = os.path.dirname(os.path.abspath(__file__)) + "/"
HOUR = 3600000
NOW = 1_800_000_000_000


def unit(path, bytes_, age_hours, lock=False, kind="directory"):
    return {"path": path, "bytes": bytes_, "recencyMs": NOW - int(age_hours * HOUR), "lockHeld": lock, "kind": kind}


def area(name, units, budget=None, unused=48, guard=None):
    row = {"name": name, "budgetBytes": budget, "unusedAgeMs": int(unused * HOUR), "units": units}
    if guard is not None:
        row["guardAgeMs"] = int(guard * HOUR)
    return row


c = {"version": 1, "nowMs": NOW, "guardAgeMs": 24 * HOUR}
authored = []


def scenario(name, areas, expect=None):
    authored.append({"name": name, "areas": areas, "expect": expect})


scenario("old units are deleted by age and the rest is kept", [area("a", [unit("old", 100, 100), unit("new", 50, 1)], None, 48)],
         [{"name": "a", "totalBytes": 150, "unitCount": 2, "age": ["old"], "budget": [], "retainedBytes": 50, "guarded": 0}])
scenario("a unit exactly at the age cutoff is deleted", [area("a", [unit("edge", 10, 48), unit("inside", 10, 47.999)], None, 48)],
         [{"name": "a", "totalBytes": 20, "unitCount": 2, "age": ["edge"], "budget": [], "retainedBytes": 10, "guarded": 0}])
scenario("a held unit is never deleted by age", [area("a", [unit("held", 10, 500, True), unit("free", 10, 500)], None, 48)],
         [{"name": "a", "totalBytes": 20, "unitCount": 2, "age": ["free"], "budget": [], "retainedBytes": 10, "guarded": 0}])
scenario("over budget evicts the oldest first until it fits", [area("a", [unit("c", 40, 26), unit("a", 40, 60), unit("b", 40, 40)], 70, 1000)],
         [{"name": "a", "totalBytes": 120, "unitCount": 3, "age": [], "budget": ["a", "b"], "retainedBytes": 40, "guarded": 0}])
scenario("equal recency is evicted in path order", [area("a", [unit("z", 40, 30), unit("m", 40, 30), unit("a", 40, 30)], 50, 1000)],
         [{"name": "a", "totalBytes": 120, "unitCount": 3, "age": [], "budget": ["a", "m"], "retainedBytes": 40, "guarded": 0}])
scenario("units inside the guard age are reported instead of deleted", [area("a", [unit("hot", 100, 2), unit("cold", 30, 30)], 10, 1000)],
         [{"name": "a", "totalBytes": 130, "unitCount": 2, "age": [], "budget": ["cold"], "retainedBytes": 100, "guarded": 100}])
scenario("an area guard age overrides the shared one", [area("a", [unit("mid", 100, 5)], 10, 1000, 2), area("b", [unit("mid", 100, 5)], 10, 1000)],
         [{"name": "a", "totalBytes": 100, "unitCount": 1, "age": [], "budget": ["mid"], "retainedBytes": 0, "guarded": 0}, {"name": "b", "totalBytes": 100, "unitCount": 1, "age": [], "budget": [], "retainedBytes": 100, "guarded": 100}])
scenario("age deletions count against the budget before eviction", [area("a", [unit("old", 100, 100), unit("x", 30, 30), unit("y", 30, 20)], 50, 48)],
         [{"name": "a", "totalBytes": 160, "unitCount": 3, "age": ["old"], "budget": ["x"], "retainedBytes": 30, "guarded": 0}])
scenario("a held unit over budget is reported as guarded", [area("a", [unit("held", 100, 500, True)], 10, 1000)],
         [{"name": "a", "totalBytes": 100, "unitCount": 1, "age": [], "budget": [], "retainedBytes": 100, "guarded": 100}])
scenario("a null budget never evicts", [area("a", [unit("a", 1000000, 10)], None, 1000)],
         [{"name": "a", "totalBytes": 1000000, "unitCount": 1, "age": [], "budget": [], "retainedBytes": 1000000, "guarded": 0}])
scenario("a zero budget evicts everything that may be", [area("a", [unit("a", 10, 30), unit("b", 10, 30, True), unit("c", 10, 1)], 0, 1000)],
         [{"name": "a", "totalBytes": 30, "unitCount": 3, "age": [], "budget": ["a"], "retainedBytes": 20, "guarded": 20}])
scenario("an age of zero deletes every unit that is not in the future", [area("a", [unit("now", 1, 0), unit("future", 1, -5)], None, 0)],
         [{"name": "a", "totalBytes": 2, "unitCount": 2, "age": ["now"], "budget": [], "retainedBytes": 1, "guarded": 0}])
scenario("no areas and empty areas", [area("empty", [], 5, 1)], [{"name": "empty", "totalBytes": 0, "unitCount": 0, "age": [], "budget": [], "retainedBytes": 0, "guarded": 0}])
c["authored"] = authored
rng = random.Random(20261021)
random_scenarios = []
for index in range(150):
    areas = []
    for a in range(rng.randint(1, 3)):
        units = []
        for u in range(rng.randint(0, 9)):
            units.append(unit("u%02d" % u, rng.choice([1, 7, 100, 1000, 4096, 65536]) * rng.randint(1, 5), rng.choice([0, 0.5, 1, 23, 24, 25, 47, 48, 49, 72, 300, 1000]), rng.random() < 0.15))
        budget = rng.choice([None, 0, 100, 5000, 100000, 1000000])
        guard = rng.choice([None, None, 0, 6, 24, 100])
        areas.append(area("area%d" % a, units, budget, rng.choice([0, 1, 24, 48, 72, 400]), guard))
    random_scenarios.append({"name": "seeded %03d" % index, "areas": areas, "expect": None})
c["random"] = random_scenarios
c["bytes"] = [0, 1, 512, 1023, 1024, 1025, 1152, 1536, 10240, 1048575, 1048576, 1048577, 1310720, 123456789, 1073741823, 1073741824, 5368709120, 1099511627775, 1099511627776, 1125899906842624, 1152921504606846976, 2 ** 53 - 1]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False)
print(len(random_scenarios))
