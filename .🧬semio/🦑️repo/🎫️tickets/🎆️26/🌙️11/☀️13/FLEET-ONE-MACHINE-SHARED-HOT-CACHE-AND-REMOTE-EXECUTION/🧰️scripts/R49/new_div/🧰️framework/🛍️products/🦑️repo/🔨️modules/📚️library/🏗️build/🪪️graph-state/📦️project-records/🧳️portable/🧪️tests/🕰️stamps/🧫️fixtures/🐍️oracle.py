import hashlib, json, sys

doc = json.load(open(sys.argv[1], encoding="utf8"))
case = doc["cases"][0]
where = case["roots"][0]
stamp = case["stamps"][0]
TOKEN = "⟦semio-root⟧"


def neutral(rest):
    return TOKEN + rest


def row_file(rest, size, mtime, ctime, ino, sha):
    return {"kind": "file", "path": neutral(rest), "fact": {"size": size, "mtimeMs": 0, "ctimeMs": 0, "ino": 0, "sha256": sha}}


a = "a" * 64
source = {"size": 12, "mtimeMs": 0, "ctimeMs": 0, "ino": 0, "sha256": a}
record = {
    "digest": "d" * 64,
    "authority": [],
    "observations": [
        row_file("/libs/alpha/project.json", 12, stamp[0], stamp[1], stamp[2], a),
        {"kind": "file", "path": neutral("/libs/alpha/missing.json"), "fact": {"error": "ENOENT"}},
        {"kind": "analysis", "path": neutral("/libs/alpha/src/a.ts"), "fact": {"provider": "p", "revision": "r", "resultSha256": "b" * 64, "source": source, "nestedObservations": [row_file("/libs/alpha/src/b.ts", 3, 0, 0, 0, "c" * 64)]}},
    ],
    "image": json.dumps({"name": "alpha", "root": "libs/alpha"}, separators=(",", ":")),
}
print(hashlib.sha256(json.dumps(record, separators=(",", ":"), ensure_ascii=False).encode("utf8")).hexdigest())
