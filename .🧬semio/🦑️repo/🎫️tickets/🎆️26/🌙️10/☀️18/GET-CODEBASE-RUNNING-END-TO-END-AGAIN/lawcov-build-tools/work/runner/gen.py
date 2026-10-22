"""📝 Authoring aid: writes the runner corpus (corpus.json) from the cases below; the corpus is the language-agnostic artifact, this script only keeps it readable."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"


def f(path, size=100, h=24):
    return {"kind": "file", "path": path, "size": size, "ageHours": h}


def d(path, h=None):
    row = {"kind": "dir", "path": path}
    if h is not None:
        row["ageHours"] = h
    return row


def sl(path, to):
    return {"kind": "symlink", "path": path, "to": to}


def hl(path, of):
    return {"kind": "hardlink", "path": path, "of": of}


T4242 = "0000000abc-000039u-owner"
T4243 = "0000000abc-000039v-owner"
H = 3600000
P = {"en": "p", "de": "q"}


def expected(present, groups, entries, partial, sqlite, queues, age):
    return {"present": present, "countedGroups": groups, "entries": entries, "partial": partial, "sqlite": sqlite, "queues": queues, "recencyAgeHours": age}


def tree(id, directory, budget, age, quiet, custody):
    return {"id": id, "root": "repository", "kind": "tree", "directory": directory, "budget": {"bytes": budget},
            "collect": {"unusedAgeMs": age, "quietMs": quiet, "custody": custody}, "writers": ["w"], "purpose": P}


c = {"schemaVersion": 1}
c["measure"] = [
    {"name": "hardlinks count once, links never, recency from files only", "limit": 1000,
     "entries": [f("a.bin", 5000, 72), f("deep/er/b.bin", 100, 216), hl("hard.bin", "a.bin"), sl("link", "@outside"), d("deep", 0)],
     "outside": [f("huge.bin", 100000, 1)],
     "expected": expected(True, [["a.bin", "hard.bin"], ["deep/er/b.bin"]], 6, False, [], [], 72)},
    {"name": "sqlite lease files and queue directories are found at any depth", "limit": 1000,
     "entries": [f("x.sqlite", 10, 5), f("sub/y.sqlite", 10, 6), f("sub/z.db", 10, 7), f("q/a.queue/ticket", 1, 8), f("deep/er/b.queue/t", 1, 9), d("plain.queued")],
     "expected": expected(True, [["x.sqlite"], ["sub/y.sqlite"], ["sub/z.db"], ["q/a.queue/ticket"], ["deep/er/b.queue/t"]], 12, False, ["sub/y.sqlite", "x.sqlite"], ["deep/er/b.queue", "q/a.queue"], 5)},
    {"name": "an empty directory has no files so its own time is its recency", "limit": 10, "entries": [d("", 30)],
     "expected": expected(True, [], 0, False, [], [], 30)},
    {"name": "a missing path is absent", "limit": 10, "entries": [], "missing": True,
     "expected": expected(False, [], 0, False, [], [], None)},
    {"name": "a path that is one sqlite file measures as itself", "limit": 10, "singleFile": "one.sqlite", "entries": [f("one.sqlite", 4000, 12)],
     "expected": expected(True, [["one.sqlite"]], 1, False, ["one.sqlite"], [], 12)},
    {"name": "only links inside a tree count nothing", "limit": 10, "entries": [sl("l1", "@outside/target.bin"), sl("l2", "@outside")],
     "outside": [f("target.bin", 9000, 1)],
     "expected": expected(True, [], 2, False, [], [], None)},
    {"name": "the entry limit makes a partial walk that stops one entry past the limit", "limit": 4,
     "entries": [f("f" + str(i), 10, i + 1) for i in range(10)],
     "expected": expected(True, None, 5, True, [], [], None)},
]
c["held"] = [
    {"name": "a live queue ticket holds the tree", "entries": [f("q/a.queue/" + T4242, 1, 72)], "alive": [4242], "heldSqlite": [], "expected": "pid:4242"},
    {"name": "a dead queue ticket does not", "entries": [f("q/a.queue/" + T4243, 1, 72)], "alive": [4242], "heldSqlite": [], "expected": None},
    {"name": "names that are not tickets never hold",
     "entries": [f("q/a.queue/not-a-ticket", 1, 72), f("q/a.queue/0000000abc-000039u", 1, 72), f("q/a.queue/0000000abc-000039U-owner", 1, 72), f("q/a.queue/0000000abc-000039u-", 1, 72)],
     "alive": [4242], "heldSqlite": [], "expected": None},
    {"name": "process zero never holds", "entries": [f("q/a.queue/0000000abc-0000000-owner", 1, 72)], "alive": [0], "heldSqlite": [], "expected": None},
    {"name": "a directory that is not named .queue is no queue", "entries": [f("q/a.queue2/" + T4242, 1, 72), f("q/queue/" + T4242, 1, 72)], "alive": [4242], "heldSqlite": [], "expected": None},
    {"name": "a queue deep inside holds the tree", "entries": [f("d/e/x.queue/" + T4242, 1, 72)], "alive": [4242], "heldSqlite": [], "expected": "pid:4242"},
    {"name": "a locked sqlite file is named before any queue", "entries": [f("s/db.sqlite", 10, 72), f("q/a.queue/" + T4242, 1, 72)], "alive": [4242], "heldSqlite": ["s/db.sqlite"], "expected": "sqlite:s/db.sqlite"},
    {"name": "a free sqlite file and no queue hold nothing", "entries": [f("s/db.sqlite", 10, 72)], "alive": [], "heldSqlite": [], "expected": None},
]
c["remove"] = [
    {"name": "links are unlinked never entered", "entries": [f("a.bin", 10, 1), f("d/b.bin", 10, 1), sl("link", "@outside"), sl("flink", "@outside/keep.txt")],
     "outside": [f("keep.txt", 20, 5), f("sub/more.txt", 30, 6)], "target": "t"},
    {"name": "a missing path is a no-op", "entries": [], "outside": [f("keep.txt", 20, 5)], "target": "t", "missing": True},
    {"name": "a single file is unlinked", "entries": [f("one.bin", 10, 1)], "outside": [f("keep.txt", 20, 5)], "target": "t/one.bin"},
    {"name": "a link given as the path is unlinked and its target stays", "entries": [sl("selflink", "@outside")],
     "outside": [f("keep.txt", 20, 5), f("sub/more.txt", 30, 6)], "target": "t/selflink"},
    {"name": "a tree larger than one batch and deeper than one level goes entirely",
     "entries": [f("many/f" + str(i), 1, 1) for i in range(100)] + [f("/".join(["n"] * 40) + "/leaf", 1, 1)],
     "outside": [f("keep.txt", 20, 5)], "target": "t"},
]
c["ledger"] = {
    "version": 1,
    "roots": {
        "repository": {"directory": "r/cache", "maximumBytes": 1073741824, "undeclaredUnusedAgeMs": 1209600000, "undeclaredQuietMs": H, "ignored": [".DS_Store"]},
        "native": {"directory": ".cache", "maximumBytes": 1048576, "undeclaredUnusedAgeMs": 604800000, "undeclaredQuietMs": H, "ignored": []},
    },
    "areas": [
        tree("t1", "t1", 100000, 864000000, H, "quiet"),
        tree("t2", "t2", 150000, None, 0, "quiet"),
        tree("agents", "agents", 100000, 86400000, 0, "sqlite"),
        {"id": "hex", "root": "repository", "kind": "tree", "pattern": "^[0-9a-f]{8}$", "budget": {"bytes": 100000},
         "collect": {"unusedAgeMs": 86400000, "quietMs": 0, "custody": "quiet"}, "writers": ["w"], "purpose": P},
        {"id": "kept", "root": "repository", "kind": "tree", "directory": "kept", "budget": {"bytes": 100000}, "managed": {"owner": "test-evidence"}, "writers": ["w"], "purpose": P},
        tree("play", "tools/play", 100000, 86400000, 0, "quiet"),
    ],
    "retirements": [
        {"root": "repository", "directory": "old", "quietMs": H, "reason": P},
        {"root": "repository", "directory": "tools/dead", "quietMs": H, "reason": P},
        {"root": "native", "directory": "gone", "quietMs": H, "reason": P},
    ],
}
c["undeclared"] = [
    {"name": "containers are searched, declared homes, patterns, retirements and ignored names are not",
     "entries": [f("stray", 1, 1), f("tools/stray", 1, 1), f("tools/dead/x", 1, 1), f("tools/play/p", 1, 1), f(".DS_Store", 1, 1), f("t1/x", 1, 1), f("aaaaaaaa/x", 1, 1),
                 f("old/x", 1, 1), f("kept/y", 1, 1), f("zzzzzzzz/x", 1, 1), f("Abcdefab/x", 1, 1)],
     "expected": {"repository": ["Abcdefab", "stray", "zzzzzzzz", "tools/stray"], "native": []}},
    {"name": "nothing is declared in the native root but its retirement", "entries": [f("n:junk/a", 1, 1), f("n:recent/a", 1, 1), f("n:gone/x", 1, 1)],
     "expected": {"repository": [], "native": ["junk", "recent"]}},
    {"name": "absent roots have nothing undeclared", "entries": [], "expected": {"repository": [], "native": []}},
    {"name": "a plain file standing where a container is expected is refused with its path", "entries": [f("tools", 1, 1)], "expected": {"repository": [], "native": []}, "error": {"root": "repository", "path": "tools"}},
    {"name": "undeclared entries inside a container are all reported", "entries": [f("tools/x", 1, 1), d("tools/nothing"), f("tools/y", 1, 1)], "expected": {"repository": ["tools/nothing", "tools/x", "tools/y"], "native": []}},
]
c["collect"] = [
    {"name": "age, budget, custody, retirement and undeclared rules at once",
     "entries": [f("t1/old/a.bin", 3000, 264), f("t1/fresh/a.bin", 3000, 48),
                 f("t2/u1/a.bin", 60000, 72), f("t2/u2/a.bin", 60000, 48), f("t2/u3/a.bin", 60000, 24),
                 f("agents/db1/state.sqlite", 100, 72), f("agents/db2/state.sqlite", 100, 72),
                 f("aaaaaaaa/q.queue/" + T4242, 10, 72), f("bbbbbbbb/q.queue/" + T4243, 10, 72), f("cccccccc/f", 10, 1),
                 f("kept/x/a.bin", 10, 9600),
                 f("old/x/a.bin", 10, 48), f("old/y/a.bin", 10, 0.5),
                 f("tools/dead/z/a.bin", 10, 48), f("tools/play/p/a.bin", 10, 48), f("tools/stray/a.bin", 10, 400),
                 f("stray-new/a", 10, 24), f(".DS_Store", 10, 4000),
                 f("n:junk/a.bin", 10, 240), f("n:recent/a", 10, 2), d("n:gone")],
     "alive": [4242], "heldSqlite": ["r/cache/agents/db1/state.sqlite"],
     "removed": {"t1/old": "age", "t2/u1": "budget", "agents/db2": "age", "hex/bbbbbbbb": "age", "repository:old/x": "retired", "repository:tools/dead/z": "retired",
                 "play/p": "age", "repository:undeclared/tools/stray": "undeclared", "native:undeclared/junk": "undeclared"},
     "kept": {"agents/db1": "held", "hex/aaaaaaaa": "held", "repository:old/y": "quiet"},
     "absentAfterApply": ["r/cache/t1/old", "r/cache/t2/u1", "r/cache/agents/db2", "r/cache/bbbbbbbb", "r/cache/old/x", "r/cache/tools/dead", "r/cache/tools/play/p",
                          "r/cache/tools/stray", ".cache/junk", ".cache/gone"],
     "presentAfterApply": ["r/cache/t1/fresh", "r/cache/t2/u2", "r/cache/t2/u3", "r/cache/agents/db1", "r/cache/aaaaaaaa", "r/cache/cccccccc", "r/cache/kept/x",
                           "r/cache/old/y", "r/cache/stray-new", "r/cache/.DS_Store", ".cache/recent", "r/cache/tools/play", "r/cache/old"],
     "only": [{"only": ["t2"], "removed": {"t2/u1": "budget"}, "kept": {}},
              {"only": ["retired"], "removed": {"repository:old/x": "retired", "repository:tools/dead/z": "retired"}, "kept": {"repository:old/y": "quiet"}}]},
    {"name": "an empty collection plans nothing and removes nothing", "entries": [d("t1")], "alive": [], "heldSqlite": [], "removed": {}, "kept": {},
     "absentAfterApply": [], "presentAfterApply": ["r/cache/t1"], "only": []},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
