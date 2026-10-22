import lib
from lib import S
UNIT = "🧼workspace-cleanup/🎮command"
ok = [([], "workspace", False, []), (["--dry"], "workspace", True, []), (["dry"], "workspace", True, []), (["workspace"], "workspace", False, []), (["workspace", "dry"], "workspace", True, []), (["coverage"], "coverage", False, []), (["coverage", "--dry"], "coverage", True, []),
 (["empty-folders", "dry"], "empty-folders", True, []), (["marker-only-folders"], "marker-only-folders", False, []), (["taxonomy"], "taxonomy", False, []), (["taxonomy", "a", "--dry"], "taxonomy", False, ["a", "--dry"]), (["test"], "test", False, []), (["test", "x", "y"], "test", False, ["x", "y"])]
bad = [["--dry", "--dry"], ["foo"], ["coverage", "x"], ["coverage", "dry", "dry"], ["--dry", "coverage"], ["dry", "extra"], [""], ["Workspace"], ["empty-folders", "--dryrun"], ["marker-only-folders", "dry", "dry"]]
corpus = {"version": 1, "accepted": [{"segments": a, "mode": m, "dry": d, "arguments": r} for a, m, d, r in ok], "rejected": [{"segments": b} for b in bad], "nonArrays": ["workspace", None, {"0": "dry"}, 7]}
schema = S.obj({"version": S.const(1), "accepted": S.arr(S.obj({"segments": S.strs(), "mode": S.enum("workspace", "coverage", "empty-folders", "marker-only-folders", "taxonomy", "test"), "dry": S.bool, "arguments": S.strs()}), 1), "rejected": S.arr(S.obj({"segments": S.strs()}), 1), "nonArrays": S.arr(S.anyv, 1)})
body = '''for(const row of corpus.accepted)test("accepts "+JSON.stringify(row.segments),()=>{const entry=readWorkspaceCleanupEntryV1(row.segments);expect({mode:entry.mode,dry:entry.dry,arguments:[...entry.arguments]}).toEqual({mode:row.mode,dry:row.dry,arguments:row.arguments});expect(Object.isFrozen(entry)).toBe(true);expect(Object.isFrozen(entry.arguments)).toBe(true);});
for(const row of corpus.rejected)test("rejects "+JSON.stringify(row.segments),()=>{expect(()=>readWorkspaceCleanupEntryV1(row.segments)).toThrow("Closed workspace cleanup command required");});
for(const[index,value]of corpus.nonArrays.entries())test("rejects the non-array input "+index,()=>{expect(()=>readWorkspaceCleanupEntryV1(value as never)).toThrow("Closed workspace cleanup command required");});
test("an array holding a non-string is rejected",()=>{expect(()=>readWorkspaceCleanupEntryV1(["dry",1] as never)).toThrow("Closed workspace cleanup command required");});'''
ts = lib.ts_wrap(UNIT, "🎮️ Contract: the workspace cleanup command accepts exactly a closed mode plus an optional dry marker (and passes the rest of `taxonomy` and `test` through) and rejects everything else before touching a resource; the corpus is language-agnostic and `@@O@@` parses the same argument vectors.",
 'import {readWorkspaceCleanupEntryV1} from "@@' + UNIT + '/🟦.ts@@";', body)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent model of the workspace cleanup entry grammar: closed mode, optional dry marker, pass-through for taxonomy and test (stdlib only).") + r'''
MODES = ("workspace", "coverage", "empty-folders", "marker-only-folders")


def parse(seg):
    if not isinstance(seg, list) or any(not isinstance(s, str) for s in seg):
        raise ValueError("closed")
    if seg and seg[0] in ("taxonomy", "test"):
        return {"mode": seg[0], "dry": False, "arguments": seg[1:]}
    marker = lambda s: s in ("--dry", "dry")
    mode = "workspace" if not seg or marker(seg[0]) else seg[0]
    tail = [] if not seg else seg if marker(seg[0]) else seg[1:]
    if mode not in MODES or len(tail) > 1 or any(not marker(t) for t in tail):
        raise ValueError("closed")
    return {"mode": mode, "dry": len(tail) == 1, "arguments": []}


for c in corpus["accepted"]:
    if parse(c["segments"]) != {"mode": c["mode"], "dry": c["dry"], "arguments": c["arguments"]}:
        failures.append("accept %r" % c["segments"])
for c in corpus["rejected"]:
    try:
        parse(c["segments"])
        failures.append("reject %r" % c["segments"])
    except ValueError:
        pass
for v in corpus["nonArrays"]:
    try:
        parse(v)
        failures.append("non-array %r" % v)
    except ValueError:
        pass
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, oracle)
