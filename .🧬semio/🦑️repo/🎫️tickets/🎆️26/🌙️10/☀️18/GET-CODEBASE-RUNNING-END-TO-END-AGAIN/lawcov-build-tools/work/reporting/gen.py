"""📝 Authoring aid: writes the report name corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
names = [
    ("a.json", True), ("A0._-.json", True), ("9.json", True), ("report-1.2.3.json", True), ("a..json", True), ("a-.json", True), ("a_b.json", True),
    ("", False), (".json", False), (".a.json", False), ("-a.json", False), ("_a.json", False), ("a.JSON", False), ("a.json ", False), (" a.json", False),
    ("a.json\n", False), ("a\n.json", False), ("a/b.json", False), ("a\\b.json", False), ("a:b.json", False), ("é.json", False), ("a.jsonl", False), ("a", False),
    ("a.json.json", True), ("a b.json", False), ("a\u0000.json", False), ("..\\a.json", False), ("../a.json", False),
]
c = {"version": 1, "names": [{"name": n, "accepted": ok} for n, ok in names], "lengths": {"stem": "a", "from": 60, "to": 72, "maximumAccepted": 69}, "notStrings": [1, None, True, {}, [], 1.5]}
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
