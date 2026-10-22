"""📝 Authoring aid: writes the extra pristine-disposition corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
CLOSED = "Closed pristine file observation required"
CANON = "Canonical pristine package path required"


def obs(path="source.js", present=True, links=1, archive=True, shared=False, matches=False):
    return {"path": path, "present": present, "links": links, "archive": archive, "shared": shared, "matches": matches}


def row(name, observation, expected=None, error=""):
    return {"name": name, "observation": observation, "expected": expected, "error": error}


c = {"version": 1}
c["cases"] = [
    row("nested package paths are never classified further", obs("node_modules/a/b.js", present=False, links=0, archive=False), "nested"),
    row("a nested path wins over an unknown hardlink", obs("node_modules/a.js", links=9, shared=False), "nested"),
    row("a file called node_modules itself is an ordinary file", obs("node_modules"), "replace"),
    row("a path that merely contains node_modules deeper is ordinary", obs("a/node_modules/b.js"), "replace"),
    row("a path starting with node_modules but no slash is ordinary", obs("node_modules.js"), "replace"),
    row("a shared exact binary keeps its link", obs(links=2, shared=True, matches=True), "preserve"),
    row("a shared file linked once is refused", obs(links=1, shared=True, matches=True), None, "Exact shared binary registry bytes required"),
    row("a missing archive source is refused", obs(present=False, links=0), None, "Registry source must replace an existing regular file"),
    row("a missing non-archive file is refused too", obs(present=False, links=0, archive=False), None, "Registry source must replace an existing regular file"),
    row("two links to an unshared file are refused", obs(links=2), None, "Unknown shared package inode refused"),
    row("zero links to a present file are refused", obs(links=0), None, "Unknown shared package inode refused"),
    row("an absolute path is not canonical", obs("/etc/passwd"), None, CANON),
    row("a parent segment is not canonical", obs("a/../b.js"), None, CANON),
    row("a dot segment is not canonical", obs("./a.js"), None, CANON),
    row("an empty segment is not canonical", obs("a//b.js"), None, CANON),
    row("a trailing slash is not canonical", obs("a/"), None, CANON),
    row("a backslash is not canonical", obs("a\\b.js"), None, CANON),
    row("an empty path is not closed", obs(""), None, CLOSED),
    row("a 257 character path is not closed", obs("p" * 257), None, CLOSED),
    row("a 256 character path is accepted", obs("p" * 256), "replace"),
    row("negative links are not closed", obs(links=-1), None, CLOSED),
    row("a fractional link count is not closed", obs(links=1.5), None, CLOSED),
    row("a string where a flag belongs is not closed", {**obs(), "present": "yes"}, None, CLOSED),
    row("an unknown field is not closed", {**obs(), "extra": 1}, None, CLOSED),
    row("a missing field is not closed", {k: v for k, v in obs().items() if k != "matches"}, None, CLOSED),
]
c["truthTable"] = {"paths": ["source.js", "node_modules/x/y.js"], "present": [True, False], "links": [0, 1, 2, 3], "flags": [True, False]}
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
