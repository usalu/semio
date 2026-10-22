"""📝 Authoring aid: writes the extra selected-Rust inventory corpus (corpus.json), same row shape as the unit's own corpus plus an error message."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
R = "{workspaceRoot}/"
BASE = ["nx.json", "package.json", "📋️project.json"]


def req(tasks, engine=None):
    return {"version": 1, "tasks": tasks, "engineInputs": engine or []}


def unit(*inputs):
    return {"kind": "unit", "inputs": [R + p if not p.startswith("@") else p[1:] for p in inputs]}


def row(id, request, paths=None, error="", expand=None, count=None):
    return {"id": id, "request": request, "paths": paths, "error": error, "expand": expand, "count": count}


REFUSED = "Closed selected Rust Native request required"
LITERAL = "Original literal Native Source path required"
c = {"version": 1, "cases": [
    row("sorted, deduplicated and always carrying the required files", req([unit("b/Z.rs", "a/a.rs", "a/a.rs")], [R + "A.ts"]), sorted(set(BASE + ["b/Z.rs", "a/a.rs", "A.ts"]))),
    row("the same path from a task and the engine is kept once", req([unit("x.rs")], [R + "x.rs"]), sorted(set(BASE + ["x.rs"]))),
    row("a task with no unit makes the whole request fall back", req([{"kind": "aggregate", "inputs": []}])),
    row("an aggregate that lists inputs makes the request fall back", req([unit("x.rs"), {"kind": "aggregate", "inputs": [R + "y.rs"]}])),
    row("an unclassified task makes the request fall back", req([unit("x.rs"), {"kind": "other", "inputs": []}])),
    row("an unresolved input list makes the request fall back", req([unit("x.rs"), {"kind": "unit", "inputs": None}])),
    row("an input that is not a workspace path makes the request fall back", req([unit("x.rs", "@default")])),
    row("an engine input that is not a workspace path makes the request fall back", req([unit("x.rs")], ["named"])),
    row("a glob star falls back", req([unit("crate/*.rs")])),
    row("a glob question mark falls back", req([unit("crate/?.rs")])),
    row("a brace group falls back", req([unit("crate/{a,b}.rs")])),
    row("a character class falls back", req([unit("crate/[ab].rs")])),
    row("a negation falls back", req([unit("!crate/a.rs")])),
    row("a parent segment is refused", req([unit("../outside.rs")]), None, LITERAL),
    row("an empty path is refused", req([unit("")]), None, LITERAL),
    row("an empty segment is refused", req([unit("a//b.rs")]), None, LITERAL),
    row("a dot segment is refused", req([unit("a/./b.rs")]), None, LITERAL),
    row("a backslash is refused", req([unit("a\\b.rs")]), None, LITERAL),
    row("a colon is refused", req([unit("C:b.rs")]), None, LITERAL),
    row("a NUL byte is refused", req([unit("a\u0000b.rs")]), None, LITERAL),
    row("a trailing slash is refused", req([unit("a/")]), None, LITERAL),
    row("a path of exactly 256 characters is kept", req([unit("a" * 256)]), sorted(set(BASE + ["a" * 256]))),
    row("a path of 257 characters is refused", req([unit("a" * 257)]), None, LITERAL),
    row("exactly the maximum of distinct inputs is kept", req([unit("seed.rs")]), None, "", {"prefix": "f", "suffix": ".rs", "count": 8188}, 8192),
    row("one more distinct input than the maximum falls back", req([unit("seed.rs")]), None, "", {"prefix": "f", "suffix": ".rs", "count": 8189}),
    row("a request with another version is refused", {**req([unit("x.rs")]), "version": 2}, None, REFUSED),
    row("an extra property is refused", {**req([unit("x.rs")]), "extra": 1}, None, REFUSED),
    row("no tasks are refused", req([]), None, REFUSED),
    row("a missing engine list is refused", {"version": 1, "tasks": [unit("x.rs")]}, None, REFUSED),
    row("a task with an unknown kind is refused", req([{"kind": "wild", "inputs": []}]), None, REFUSED),
    row("an input longer than 512 characters is refused", req([unit("a" * 520)]), None, REFUSED),
]}
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
