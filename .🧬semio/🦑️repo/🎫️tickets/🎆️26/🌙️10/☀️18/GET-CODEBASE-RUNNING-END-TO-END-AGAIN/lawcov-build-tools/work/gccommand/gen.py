"""📝 Authoring aid: writes the cache command argument corpus (corpus.json): systematic argument vectors for `repo cache status|gc`, the area ids of a small ledger and the authored parses."""
import itertools, json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
P = {"en": "p", "de": "q"}
H = 3600000
tree = lambda id, directory: {"id": id, "root": "repository", "kind": "tree", "directory": directory, "budget": {"bytes": 100000}, "collect": {"unusedAgeMs": 86400000, "quietMs": H, "custody": "quiet"}, "writers": ["w"], "purpose": P}
ledger = {"version": 1, "roots": {"repository": {"directory": "r/cache", "maximumBytes": 1073741824, "undeclaredUnusedAgeMs": 1209600000, "undeclaredQuietMs": H, "ignored": []}, "native": {"directory": ".cache", "maximumBytes": 1048576, "undeclaredUnusedAgeMs": 604800000, "undeclaredQuietMs": H, "ignored": []}},
          "areas": [tree("t1", "t1"), tree("t2", "t2")], "retirements": [{"root": "repository", "directory": "old", "quietMs": H, "reason": P}]}
operations = [["status"], ["gc"], ["clear"], [], ["STATUS"], ["status "]]
options = [[], ["--json"], ["--dry-run"], ["--json", "--dry-run"], ["--json", "--json"], ["--json=1"], ["--dry-run=true"], ["--bogus"], ["-j"],
           ["--area", "t1"], ["--area=t1,t2"], ["--area=t1,retired,undeclared"], ["--area=nowhere"], ["--area=t1,nowhere"], ["--area"], ["--area", "--json"], ["--area="], ["--area=,"], ["--area=,t1,"], ["--area", "t1", "--area=t2"],
           ["--lang", "de"], ["--lang=en"], ["--lang"], ["--lang=--json"], ["--lang=", "--json"], ["--lang", "de", "--lang", "en"], ["--json", "--area=t2", "--lang=de"], ["--dry-run", "--area=retired", "--lang", "en"]]
vectors = [op + opt for op, opt in itertools.product(operations, options)]
vectors += [["gc", "--dry-run", "--dry-run"], ["gc", "t1"], ["gc", "--json", "extra"], ["status", "--area=t1", "--dry-run"]]
c = {"version": 1, "ledger": ledger, "vectors": vectors}
c["authored"] = [
    {"args": ["gc", "--dry-run", "--area=t1,retired", "--lang", "de", "--json"], "parsed": {"operation": "gc", "json": True, "dryRun": True, "only": ["t1", "retired"], "language": "de"}},
    {"args": ["status"], "parsed": {"operation": "status", "json": False, "dryRun": False, "only": None, "language": None}},
    {"args": ["status", "--area=undeclared"], "parsed": {"operation": "status", "json": False, "dryRun": False, "only": ["undeclared"], "language": None}},
]
c["refused"] = [
    {"args": ["clear"], "key": "error.syntax", "variables": {}},
    {"args": ["status", "--bogus"], "key": "error.option", "variables": {"option": "--bogus"}},
    {"args": ["status", "--json", "--json"], "key": "error.repeated", "variables": {"option": "--json"}},
    {"args": ["status", "--dry-run"], "key": "error.option", "variables": {"option": "--dry-run"}},
    {"args": ["gc", "--area=,"], "key": "error.areaEmpty", "variables": {}},
    {"args": ["gc", "--area=t1,,t2"], "key": "error.areaEmpty", "variables": {}},
    {"args": ["status", "--area=t1,"], "key": "error.areaEmpty", "variables": {}},
    {"args": ["status", "--area=nowhere,"], "key": "error.area", "variables": {"id": "nowhere"}},
    {"args": ["status", "--area=,nowhere"], "key": "error.areaEmpty", "variables": {}},
    {"args": ["gc", "--area"], "key": "error.value", "variables": {"option": "--area"}},
    {"args": ["gc", "--area=nowhere"], "key": "error.area", "variables": {"id": "nowhere"}},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False)
print(len(vectors))
