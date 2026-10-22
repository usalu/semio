"""📝 Authoring aid: writes the leases command corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
c = {"version": 1, "leaseDirectory": ".cache/b/l/m/data", "floorMegabytes": {"minimum": 4096}}
c["commands"] = [
    {"args": ["status"], "outcome": "text"},
    {"args": ["status", "--json"], "outcome": "json"},
    {"args": ["status", "--json", "--json"], "outcome": "json"},
    {"args": ["status", "--all"], "outcome": "text"},
    {"args": ["status", "--violations=2"], "outcome": "text"},
    {"args": ["status", "--violations=0", "--json"], "outcome": "json"},
    {"args": ["status", "--violations=x"], "outcome": "throw", "message": "Leases option --violations requires a non-negative integer"},
    {"args": ["status", "--violations=-1"], "outcome": "throw", "message": "Leases option --violations requires a non-negative integer"},
    {"args": ["status", "--bogus"], "outcome": "throw", "message": "Unknown leases option: --bogus"},
    {"args": ["status", "--lang=de"], "outcome": "throw", "message": "Unknown leases option: --lang=de"},
    {"args": ["bogus"], "outcome": "throw", "message": "Expected repo leases status"},
    {"args": [], "outcome": "throw", "message": "Expected repo leases status"},
    {"args": ["--help"], "outcome": "throw", "message": "Expected repo leases status"},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
