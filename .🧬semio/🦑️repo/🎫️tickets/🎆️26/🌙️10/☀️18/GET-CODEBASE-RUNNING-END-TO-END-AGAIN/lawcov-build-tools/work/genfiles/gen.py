"""📝 Authoring aid: writes the generation files window corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
KINDS = ["read", "write", "names", "list", "remove"]


def seq(name, window, count, fail=None, kinds=None):
    return {"name": name, "window": window, "operations": [{"kind": (kinds or KINDS)[i % len(kinds or KINDS)], "path": "/store/p%d" % i} for i in range(count)], "failRechecks": fail or []}


c = {"version": 1, "defaultWindow": 64}
c["sequences"] = [
    seq("a window of one rechecks before every operation", 1, 7),
    seq("a window of two rechecks every second operation", 2, 7),
    seq("a window of three over nine operations", 3, 9),
    seq("fewer operations than the window never recheck", 64, 63),
    seq("the operation that fills the window rechecks", 64, 64),
    seq("two windows and a half", 64, 160),
    seq("no operations no rechecks", 5, 0),
    seq("a refused first recheck refuses only the operation of its window", 2, 6, [1]),
    seq("every refused recheck refuses its window and the next window checks again", 2, 8, [1, 2, 3]),
    seq("only reads cross the window", 2, 6, None, ["read"]),
    seq("only removals cross the window", 3, 9, None, ["remove"]),
]
c["concurrent"] = [
    {"name": "many readers on a slow recheck", "workers": 8, "each": 20, "window": 16, "sleepMs": 15},
    {"name": "a window of one with concurrent operations", "workers": 4, "each": 5, "window": 1, "sleepMs": 5},
    {"name": "a window larger than the whole workload", "workers": 4, "each": 5, "window": 1000, "sleepMs": 1},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
