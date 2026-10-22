"""📝 Authoring aid: writes the revision graph corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
H = "a" * 64
revisions = [(H, True), ("0123456789abcdef" * 4, True), ("A" * 64, False), ("a" * 63, False), ("a" * 65, False), ("", False), ("g" * 64, False), ("a" * 63 + "\n", False), (" " + "a" * 63, False), ("a" * 32 + "A" + "a" * 31, False)]
keys = [("a", "a"), ("source-analyses", "sourceAnalyses"), ("a-b-c", "aBC"), ("a1-2b", "a12b"), ("x-1", "x1"), ("revision-graph", "revisionGraph"), ("a-bc-d", "aBcD")]
E = lambda name, path=None, needs=None: {"name": name, "path": path if path is not None else "./" + name + ".mjs", "needs": needs if needs is not None else []}
ok = lambda name, comps: {"name": name, "manifest": {"version": 1, "components": comps}, "error": None}
bad = lambda name, manifest, error: {"name": name, "manifest": manifest, "error": error}
V1 = lambda comps: {"version": 1, "components": comps}
INVALID = lambda index: "Invalid component roster entry %d" % index
rosters = [
    ok("one component", [E("a")]),
    ok("later components need earlier ones", [E("a"), E("b", needs=["a"]), E("c", needs=["a", "b"])]),
    ok("names with digits and dashes", [E("a1-2b"), E("x-1", needs=["a1-2b"])]),
    ok("nested relative paths and emoji path names", [E("a", "./📍️coordinates/🟨️.mjs"), E("b", "./deep/er/file.mjs"), E("c", "./a b/c d.mjs")]),
    ok("a dot inside a segment is fine", [E("a", "./a.b/c.d.mjs")]),
    bad("a missing manifest", None, "Component roster version 1 with components required"),
    bad("a wrong version", {"version": 2, "components": [E("a")]}, "Component roster version 1 with components required"),
    bad("no components", {"version": 1}, "Component roster version 1 with components required"),
    bad("empty components", V1([]), "Component roster version 1 with components required"),
    bad("components that are not a list", {"version": 1, "components": {"a": 1}}, "Component roster version 1 with components required"),
    bad("an entry that is not an object", V1([None]), INVALID(0)),
    bad("an entry that is a string", V1(["a"]), INVALID(0)),
    bad("an uppercase name", V1([E("Bad")]), INVALID(0)),
    bad("an underscore name", V1([E("a_b")]), INVALID(0)),
    bad("a name starting with a dash", V1([E("-a")]), INVALID(0)),
    bad("a name ending with a dash", V1([E("a-")]), INVALID(0)),
    bad("a name with a double dash", V1([E("a--b")]), INVALID(0)),
    bad("a name starting with a digit", V1([E("1a")]), INVALID(0)),
    bad("an empty name", V1([E("")]), INVALID(0)),
    bad("a path without the leading dot slash", V1([E("a", "a.mjs")]), INVALID(0)),
    bad("an absolute path", V1([E("a", "/a.mjs")]), INVALID(0)),
    bad("a parent segment", V1([E("a", "./a/../b.mjs")]), INVALID(0)),
    bad("a hidden segment", V1([E("a", "./.hidden/a.mjs")]), INVALID(0)),
    bad("a hidden file", V1([E("a", "./.a.mjs")]), INVALID(0)),
    bad("a backslash", V1([E("a", "./a\\b.mjs")]), INVALID(0)),
    bad("a query", V1([E("a", "./a.mjs?x")]), INVALID(0)),
    bad("a fragment", V1([E("a", "./a.mjs#x")]), INVALID(0)),
    bad("a trailing slash", V1([E("a", "./a/")]), INVALID(0)),
    bad("only the dot slash", V1([E("a", "./")]), INVALID(0)),
    bad("an empty segment", V1([E("a", "./a//b.mjs")]), INVALID(0)),
    bad("a path that is not a string", V1([{"name": "a", "path": 5, "needs": []}]), INVALID(0)),
    bad("needs that are not a list", V1([{"name": "a", "path": "./a.mjs", "needs": "b"}]), INVALID(0)),
    bad("missing needs", V1([{"name": "a", "path": "./a.mjs"}]), INVALID(0)),
    bad("an entry without a name", V1([{"path": "./a.mjs", "needs": []}]), INVALID(0)),
    bad("a null name", V1([{"name": None, "path": "./a.mjs", "needs": []}]), INVALID(0)),
    bad("a boolean name", V1([{"name": True, "path": "./a.mjs", "needs": []}]), INVALID(0)),
    bad("an array entry", V1([[]]), INVALID(0)),
    bad("the second entry is the invalid one", V1([E("a"), E("Bad")]), INVALID(1)),
    bad("a duplicate name", V1([E("a"), E("a", "./other.mjs")]), "Duplicate component a"),
    bad("a repeated need", V1([E("a"), E("b", needs=["a", "a"])]), "Component b repeats a need"),
    bad("a need on a later component", V1([E("a", needs=["b"]), E("b")]), "Component a needs b, which is not an earlier component"),
    bad("a need on itself", V1([E("a", needs=["a"])]), "Component a needs a, which is not an earlier component"),
    bad("a need on an unknown component", V1([E("a", needs=["ghost"])]), "Component a needs ghost, which is not an earlier component"),
    bad("the invalid entry is reported before a later duplicate", V1([E("a"), E("Bad"), E("a")]), INVALID(1)),
]
C = lambda name, needs=None, **kw: {"name": name, "needs": needs or [], **kw}
wiring = [
    {"name": "components see only their needs plus the shared services", "components": [C("coordinates"), C("bootstrap", ["coordinates"]), C("source-analyses", ["coordinates", "bootstrap"]), C("leaf")],
     "attempts": [{"fail": None, "error": None, "order": ["coordinates", "bootstrap", "source-analyses", "leaf"], "visible": {"coordinates": [], "bootstrap": ["coordinates"], "source-analyses": ["coordinates", "bootstrap"], "leaf": []}}]},
    {"name": "a failed wiring leaves every module free to be wired again, a complete one does not", "components": [C("a"), C("b", ["a"]), C("c", ["a", "b"])],
     "attempts": [{"fail": "c", "error": "wire refused: c", "order": ["a", "b", "c"], "visible": None},
                  {"fail": None, "error": None, "order": ["a", "b", "c"], "visible": {"a": [], "b": ["a"], "c": ["a", "b"]}},
                  {"fail": None, "error": "Component a is already wired into another revision", "order": [], "visible": None}]},
    {"name": "a module that fails first is not marked and nothing after it is wired", "components": [C("a"), C("b", ["a"])],
     "attempts": [{"fail": "a", "error": "wire refused: a", "order": ["a"], "visible": None}, {"fail": None, "error": None, "order": ["a", "b"], "visible": {"a": [], "b": ["a"]}}]},
    {"name": "a module that names another component is refused before any wiring", "components": [C("a"), C("b", ["a"], component="other")],
     "attempts": [{"fail": None, "error": "Component b does not match its roster entry", "order": [], "visible": None}]},
    {"name": "a module without a wire function is refused before any wiring", "components": [C("a"), C("b", ["a"], wire=False)],
     "attempts": [{"fail": None, "error": "Component b does not match its roster entry", "order": [], "visible": None}]},
    {"name": "a missing module is refused", "components": [C("a"), C("b", ["a"], absent=True)],
     "attempts": [{"fail": None, "error": "Component modules differ from the roster", "order": [], "visible": None}]},
]
UNBOUND = "Revision identity is not bound yet"
BADREV = "Revision graph requires a lowercase sha256 revision"
identity = [
    {"name": "reading before binding fails", "ops": [{"op": "read", "error": UNBOUND}]},
    {"name": "binding is idempotent for one revision and final", "ops": [{"op": "bind", "value": H}, {"op": "read", "revision": H}, {"op": "bind", "value": H}, {"op": "bind", "value": "b" * 64, "error": "Revision identity is already bound"}, {"op": "read", "revision": H}]},
    {"name": "an invalid revision binds nothing", "ops": [{"op": "bind", "value": "A" * 64, "error": BADREV}, {"op": "read", "error": UNBOUND}, {"op": "bind", "value": H}, {"op": "read", "revision": H}]},
    {"name": "a revision that is not a string is refused", "ops": [{"op": "bind", "value": 5, "error": BADREV}, {"op": "bind", "value": None, "error": BADREV}, {"op": "read", "error": UNBOUND}]},
    {"name": "an invalid revision is reported before an already bound one", "ops": [{"op": "bind", "value": H}, {"op": "bind", "value": "x", "error": BADREV}]},
]
imports = [
    {"name": "every component is imported under the revision token, in roster order", "paths": ["./a.mjs", "./deep/b.mjs"], "revision": H, "calls": ["./a.mjs?revision=" + H, "./deep/b.mjs?revision=" + H], "error": None},
    {"name": "an invalid revision imports nothing", "paths": ["./a.mjs"], "revision": "nope", "calls": [], "error": BADREV},
    {"name": "a failing import rejects the whole load", "paths": ["./a.mjs", "./b.mjs", "./c.mjs"], "revision": H, "failOn": "./b.mjs", "calls": ["./a.mjs?revision=" + H, "./b.mjs?revision=" + H, "./c.mjs?revision=" + H], "error": "load refused: ./b.mjs"},
]
loader = [
    {"name": "an adopted scope is returned for its revision and cannot be adopted twice", "revision": H},
]
json.dump({"version": 1, "revision": H, "identity": identity, "imports": imports, "revisions": [{"value": v, "valid": ok_} for v, ok_ in revisions], "keys": [{"name": a, "key": b} for a, b in keys], "rosters": rosters, "wiring": wiring}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(rosters), len(wiring))
