"""🧊 Independent Python oracle of the stable file capture (standard library only): it classifies a path the way the capture must (regular file, link, directory, missing), and digests the bytes of a regular file. Requests arrive as JSON on stdin: {"paths": [...]}."""
import hashlib, json, os, stat, sys


def classify(path):
    try:
        info = os.lstat(path)
    except FileNotFoundError:
        return {"kind": "missing"}
    if stat.S_ISLNK(info.st_mode):
        return {"kind": "symlink"}
    if stat.S_ISDIR(info.st_mode):
        return {"kind": "directory"}
    data = open(path, "rb").read()
    return {"kind": "file", "size": len(data), "sha256": hashlib.sha256(data).hexdigest()}


print(json.dumps([classify(p) for p in json.load(sys.stdin)["paths"]]))
