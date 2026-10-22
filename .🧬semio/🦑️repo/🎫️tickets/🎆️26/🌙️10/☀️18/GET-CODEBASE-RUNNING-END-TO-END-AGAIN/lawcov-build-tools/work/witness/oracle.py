"""🧾️ Independent Python oracle of the input witness digest (standard library only): it hashes declared files into `path NUL size NUL content-sha256 LF` rows over the sorted paths and returns the digest document. Requests arrive as JSON on stdin."""
import hashlib, json, sys


def content(spec):
    return (spec["repeat"] * spec["times"]).encode() if "repeat" in spec else spec["text"].encode()


def witness(files):
    outer = hashlib.sha256()
    for path, data in sorted(((f["path"], content(f["content"])) for f in files), key=lambda row: row[0]):
        outer.update(path.encode() + b"\0" + str(len(data)).encode() + b"\0" + hashlib.sha256(data).hexdigest().encode() + b"\n")
    return {"version": 1, "files": len(files), "sha256": outer.hexdigest()}


print(json.dumps([witness(row["files"]) for row in json.load(sys.stdin)["rows"]]))
