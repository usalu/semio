#!/usr/bin/env python3
"""Independent model of the vouch decision (shares no code with 🟦️.ts): `🐍️.py VECTORS.json` prints, per vector, the sorted list of paths a journal proves untouched.

A path is eligible when it is a relative non-root path that is neither excluded nor at or below a symbolic link; it is touched when some row names it, names one of its descendants, or (for a row that is not a modification of a directory) names one of its ancestors or itself."""
import json
import sys
import unicodedata


def fold(path):
    return unicodedata.normalize("NFC", path).lower()


def eligible(path, excluded, linked):
    if path in ("", ".") or path.startswith("/") or path.startswith("../") or "\\" in path or path in excluded:
        return False
    return not any(path == link or path.startswith(link + "/") for link in linked)


def touched(path, rows, insensitive):
    key = fold if insensitive else (lambda value: value)
    mine = key(path)
    for kind, directory, row_path in rows:
        other = key(row_path)
        if other == mine or other.startswith(mine + "/"):
            return True
        structural = not (directory == 1 and kind == "m")
        if structural and mine.startswith(other + "/"):
            return True
    return False


def proven(vector):
    seen = set()
    for path in vector["paths"]:
        if eligible(path, set(vector["excluded"]), vector["linked"]) and not touched(path, vector["rows"], vector["insensitive"]):
            seen.add(path)
    return sorted(seen)


if __name__ == "__main__":
    vectors = json.load(open(sys.argv[1], encoding="utf8"))
    print(json.dumps([proven(vector) for vector in vectors], ensure_ascii=False))
