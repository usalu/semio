#!/usr/bin/env python3
"""Independent model of the graph memo decision (shares no code with 🟦️.ts): `🐍️.py CASES.json` prints, per case, what each fact row reads, which directories a change must not touch, and which changes are relevant.

The policy table (which kind reads what) is DATA and is read from the schema file; everything else is re-derived here: a row reads the entries it names (`path`), the entries inside its key (`text`), a request and its result (`resolve`), nothing (`none`) or follows symbolic links (`link`, always checked again); a read outside the root or inside an excluded name is checked again; a bare module request is checked again; the directories are every entry, its directory and all directories above up to the root; a change is relevant when it names one of them or lies directly inside one, except that the modification of a directory matters only to the directories themselves."""
import json
import os
import posixpath
import re
import sys
import unicodedata

SCHEMA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧬️schema", "🔣️.json")
TABLE = json.load(open(SCHEMA, encoding="utf8"))["definitions"]["Policy"]["const"]["relevance"]


def fold(path):
    return unicodedata.normalize("NFC", path).lower()


def relative(root, path):
    text = path.replace("\\", "/")
    base = root.replace("\\", "/").rstrip("/")
    if not (text.startswith("/") or re.match(r"^[A-Za-z]:/", text)):
        clean = posixpath.normpath(text)
        return "" if clean == "." else clean
    if text == base:
        return ""
    if text.startswith(base + "/"):
        return posixpath.normpath(text[len(base) + 1:])
    return None


def reach(row, root, excluded_names):
    how = TABLE.get(row["kind"])
    if how is None:
        return None
    if how == "none":
        return {"mentions": [], "recheck": False}
    mentions, state = [], {"recheck": how == "link"}

    def take(path):
        if not isinstance(path, str):
            state["recheck"] = True
            return
        rel = relative(root, path)
        if rel is None or any(part in excluded_names for part in rel.split("/")):
            state["recheck"] = True
        else:
            mentions.append(rel)

    if how in ("path", "analysis", "link"):
        take(row["path"])
    elif how == "resolve":
        try:
            parsed = json.loads(row["path"])
            origin, request = parsed[0], parsed[1]
            assert isinstance(origin, str) and isinstance(request, str)
        except Exception:
            return {"mentions": [], "recheck": True}
        take(origin)
        if request.startswith("."):
            take(posixpath.join(posixpath.dirname(origin.replace("\\", "/")), request))
        elif request.startswith("/") or re.match(r"^[A-Za-z]:[\\/]", request):
            take(request)
        else:
            state["recheck"] = True
        if isinstance(row["fact"], str) and not row["fact"].startswith("!"):
            take(row["fact"])
    else:
        base = root.replace("\\", "/").rstrip("/")
        for found in re.finditer(re.escape(base) + r'(?:/[^"\\\x00-\x1f]*)?', row["path"].replace("\\\\", "/")):
            take(found.group(0))
    return {"mentions": mentions, "recheck": state["recheck"]}


def directories(mentions):
    seen = set()
    for mention in mentions:
        path = mention
        while path not in seen:
            seen.add(path)
            if path == "":
                break
            path = path.rsplit("/", 1)[0] if "/" in path else ""
    return seen


def relevant(dirs, change, insensitive):
    key = fold if insensitive else (lambda value: value)
    held = {key(item) for item in dirs}
    kind, directory, path = change
    parent = path.rsplit("/", 1)[0] if "/" in path else ""
    return key(path) in held or (not (kind == "m" and directory == 1) and key(parent) in held)


def leaves(row):
    yield row
    if row["kind"] == "analysis" and isinstance(row["fact"], dict):
        for inner in row["fact"].get("nestedObservations", []) or []:
            yield from leaves(inner)


def solve(case):
    root, names = case["root"], set(case["excludedNames"])
    rows_out, vouched = [], []
    for row in case["rows"]:
        for leaf in leaves(row):
            result = reach(leaf, root, names)
            rows_out.append(None if result is None else {"kind": leaf["kind"], "mentions": result["mentions"], "recheck": result["recheck"]})
            if result is not None and not result["recheck"]:
                vouched.extend(result["mentions"])
    dirs = directories(vouched)
    return {"rows": rows_out, "directories": sorted(dirs), "relevant": [relevant(dirs, change, case["insensitive"]) for change in case["changes"]]}


if __name__ == "__main__":
    json.dump([solve(case) for case in json.load(open(sys.argv[1], encoding="utf8"))], sys.stdout, ensure_ascii=False)
