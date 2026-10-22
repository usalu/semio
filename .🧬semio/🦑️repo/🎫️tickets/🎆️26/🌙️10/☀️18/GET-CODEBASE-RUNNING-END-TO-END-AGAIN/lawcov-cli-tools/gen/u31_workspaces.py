import json, lib
from lib import S
V = "️"
# ------------- payload
UP = "🗂workspaces/📦payload"
def case(name, owner, payload, expected):
    return {"name": name, "owner": owner, "payload": payload, "expected": expected}
o = lambda d, name="pkg", exports=None: ({"absDir": d, "name": name, "exports": exports} if exports is not None else {"absDir": d, "name": name})
pl = lambda d, name="pkg": {"absDir": d, "name": name}
cases = [
 case("a string export covering the payload directory", o("/w/pkg", exports="./dist/web/index.js"), pl("/w/pkg/dist/web"), True),
 case("a string export outside the payload directory", o("/w/pkg", exports="./dist/web/index.js"), pl("/w/pkg/other"), False),
 case("the names must be equal", o("/w/pkg", exports="./dist/a.js"), pl("/w/pkg/dist", "other"), False), case("an owner without a name", {"absDir": "/w/pkg", "exports": "./dist/a.js"}, pl("/w/pkg/dist"), False),
 case("subpath exports are all considered", o("/w/pkg", exports={".": "./a.js", "./x": "./dist/x/index.js"}), pl("/w/pkg/dist/x"), True), case("a wildcard subpath makes the whole map unusable", o("/w/pkg", exports={".": "./dist/a.js", "./*": "./dist/*.js"}), pl("/w/pkg/dist"), False),
 case("conditions are walked in order up to default", o("/w/pkg", exports={"import": "./dist/m.js", "default": "./dist/d.js", "require": "./dist/r.js"}), pl("/w/pkg/dist"), True), case("a target after default is not considered", o("/w/pkg", exports={"default": "./a.js", "require": "./other/r.js"}), pl("/w/pkg/other"), False),
 case("arrays are flattened without subpaths", o("/w/pkg", exports=["./dist/a.js", "./other/b.js"]), pl("/w/pkg/other"), True), case("a numeric key is not a condition map", o("/w/pkg", exports={"0": "./dist/a.js"}), pl("/w/pkg/dist"), False),
 case("an empty key is not a condition map", o("/w/pkg", exports={"": "./dist/a.js"}), pl("/w/pkg/dist"), False), case("targets must start with ./", o("/w/pkg", exports="dist/a.js"), pl("/w/pkg/dist"), False), case("a parent segment is refused", o("/w/pkg", exports="./../pkg/dist/a.js"), pl("/w/pkg/dist"), False),
 case("node_modules segments are refused", o("/w/pkg", exports="./node_modules/dist/a.js"), pl("/w/pkg/node_modules/dist"), False), case("special characters are refused", o("/w/pkg", exports="./dist/a%20b.js"), pl("/w/pkg/dist"), False), case("an empty segment is refused", o("/w/pkg", exports="./dist//a.js"), pl("/w/pkg/dist"), False),
 case("an owner without exports owns nothing", o("/w/pkg"), pl("/w/pkg/dist"), False), case("a null export owns nothing", {"absDir": "/w/pkg", "name": "pkg", "exports": None}, pl("/w/pkg/dist"), False), case("a nested condition map under subpaths", o("/w/pkg", exports={".": {"import": "./dist/m.js"}, "./x": {"default": "./y/z.js"}}), pl("/w/pkg/y"), True),
 case("a subpath key without a dot is refused wholesale", o("/w/pkg", exports={".": "./dist/a.js", "x": "./dist/b.js"}), pl("/w/pkg/dist"), False), case("the payload outside the owner", o("/w/pkg", exports="./dist/a.js"), pl("/w/elsewhere/dist"), False), case("a prefix that is only a string prefix", o("/w/pkg", exports="./dist-extra/a.js"), pl("/w/pkg/dist"), False),
 case("backslash paths in the payload relation", o("/w/pkg", exports="./dist/a.js"), pl("/w/pkg/dist"), True),
]
corpus_p = {"version": 1, "cases": cases}
own = S.obj({"absDir": S.str, "name": S.str, "exports": S.anyv}, required=["absDir"])
schema_p = S.obj({"version": S.const(1), "cases": S.arr(S.obj({"name": S.str, "owner": own, "payload": S.obj({"absDir": S.str, "name": S.str}), "expected": S.bool}), 1)})
body_p = '''const platform=(path:string):string=>process.platform==="win32"?"C:"+path.split("/").join("\\\\"):path;
const adapt=<T extends {absDir:string}>(value:T):T=>({...value,absDir:platform(value.absDir)});
for(const row of corpus.cases)test("owns payload: "+row.name,()=>{expect(ownsPayload(adapt(row.owner as never),adapt(row.payload as never))).toBe(row.expected);});
test("the TypeScript facade is the same function",()=>{expect(typeof ownsPayload).toBe("function");});'''
ts_p = lib.ts_wrap(UP, "📦️ Contract: a payload belongs to the nearest package owner exactly when the owner's authored export namespace (string, subpaths, conditions up to default, arrays) names a portable `./` target inside the payload directory and both carry the same package name; the corpus is language-agnostic and `@@O@@` re-derives the verdicts.",
 'import {ownsPayload} from "@@' + UP + '/🟦.ts@@";', body_p)
orc_p = lib.PY_HEAD.format(doc="🐍️ Independent model of the export-namespace ownership (stdlib only).") + r'''
def targets(v, subpaths=True):
    if isinstance(v, str):
        return [v]
    if isinstance(v, list):
        return [t for e in v for t in targets(e, False)]
    if not isinstance(v, dict):
        return []
    items = list(v.items())
    if any(k.startswith(".") for k, _ in items):
        if not subpaths or any(k != "." and (not k.startswith(".") or "*" in k) for k, _ in items):
            return []
        return [t for _, e in items for t in targets(e, False)]
    if any(k == "" or re.match(r"^\d+$", k) for k, _ in items):
        return []
    out = []
    for cond, e in items:
        out += targets(e, False)
        if cond == "default":
            break
    return out


def owns(owner, payload):
    if not owner.get("name") or owner["name"] != payload.get("name"):
        return False
    owner_dir, payload_dir = owner["absDir"], payload["absDir"]
    if not payload_dir.startswith(owner_dir + "/"):
        prefix = "../" * 99 + "/"
    else:
        prefix = payload_dir[len(owner_dir) + 1:] + "/"
    for t in targets(owner.get("exports")):
        if not t.startswith("./") or re.search(r"[\\:*?%#\x00]", t):
            continue
        segs = t[2:].split("/")
        if any(s in ("", ".", "..", "node_modules") for s in segs) or not "/".join(segs).startswith(prefix):
            continue
        return True
    return False


for c in corpus["cases"]:
    if owns(c["owner"], c["payload"]) != c["expected"]:
        failures.append(c["name"])
''' + lib.PY_TAIL
lib.write_unit(UP, corpus_p, schema_p, ts_p, orc_p)
# ------------- manifest projection
UM = "🗂workspaces/📄manifest-projection"
docs = [
 {"name": "a document with workspaces", "source": '{"name":"root","version":"1.0.0","workspaces":["a","b"],"scripts":{"x":"y"}}', "workspaces": ["a", "b"]},
 {"name": "a document without workspaces", "source": '{"name":"root"}', "workspaces": []},
 {"name": "unrelated fields keep their order", "source": '{"z":1,"workspaces":["q"],"a":{"deep":[1,2,{"k":null}]},"m":"é"}', "workspaces": ["q"]},
]
rejects = [("an array", "[]", "Root package.json must contain an object"), ("null", "null", "Root package.json must contain an object"), ("a string", '"x"', "Root package.json must contain an object"), ("a number", "7", "Root package.json must contain an object"), ("workspaces as an object", '{"workspaces":{"a":1}}', "workspaces must be a string array"),
           ("workspaces with a number", '{"workspaces":["a",1]}', "workspaces must be a string array"), ("workspaces as a string", '{"workspaces":"a"}', "workspaces must be a string array"), ("invalid JSON", "{", "")]
renders = [{"source": '{"name":"root","version":"1.0.0"}', "workspaces": ["a", "b/c"], "expected": '{\n  "name": "root",\n  "version": "1.0.0",\n  "workspaces": [\n    "a",\n    "b/c"\n  ]\n}\n'}, {"source": '{"workspaces":["old"],"x":1}', "workspaces": [], "expected": '{\n  "workspaces": [],\n  "x": 1\n}\n'},
           {"source": '{"a":{"b":[1,{"c":true}]},"workspaces":["x"],"é":"ü"}', "workspaces": ["y", "z"], "expected": '{\n  "a": {\n    "b": [\n      1,\n      {\n        "c": true\n      }\n    ]\n  },\n  "workspaces": [\n    "y",\n    "z"\n  ],\n  "é": "ü"\n}\n'}]
corpus_m = {"version": 1, "parse": docs, "rejects": [{"name": n, "source": s, "message": m} for n, s, m in rejects], "render": renders}
schema_m = S.obj({"version": S.const(1), "parse": S.arr(S.obj({"name": S.str, "source": S.str, "workspaces": S.strs()}), 1), "rejects": S.arr(S.obj({"name": S.str, "source": S.str, "message": S.str}), 1), "render": S.arr(S.obj({"source": S.str, "workspaces": S.strs(), "expected": S.str}), 1)})
body_m = '''for(const row of corpus.parse)test("parse: "+row.name,()=>{const document=parseWorkspaceRootDocument(row.source);expect(document.workspaces??[]).toEqual(row.workspaces);expect(Object.keys(document)).toEqual(Object.keys(JSON.parse(row.source)));});
for(const row of corpus.rejects)test("parse refuses "+row.name,()=>{if(row.message==="")expect(()=>parseWorkspaceRootDocument(row.source)).toThrow();else expect(()=>parseWorkspaceRootDocument(row.source)).toThrow(row.message);});
for(const[index,row]of corpus.render.entries())test("render "+index,()=>{const rendered=renderWorkspaceRootDocument(parseWorkspaceRootDocument(row.source),row.workspaces);expect(rendered).toBe(row.expected);expect(JSON.parse(rendered).workspaces).toEqual(row.workspaces);});
test("rendering does not mutate the parsed document",()=>{const document=parseWorkspaceRootDocument('{"workspaces":["a"]}');renderWorkspaceRootDocument(document,["b"]);expect(document.workspaces).toEqual(["a"]);});'''
ts_m = lib.ts_wrap(UM, "📄️ Contract: the root package document is parsed as an object whose `workspaces` is a string array and re-rendered with two-space indentation, every other field in its original order and a trailing newline; the corpus is language-agnostic and `@@O@@` renders the same documents with Python's `json`.",
 'import {parseWorkspaceRootDocument,renderWorkspaceRootDocument} from "@@' + UM + '/🟦.ts@@";', body_m)
orc_m = lib.PY_HEAD.format(doc="🐍️ Independent rendering of the root package document with Python's json module (stdlib only).") + r'''
for c in corpus["render"]:
    doc = json.loads(c["source"])
    doc["workspaces"] = list(c["workspaces"])
    if json.dumps(doc, indent=2, ensure_ascii=False) + "\n" != c["expected"]:
        failures.append("render")
for c in corpus["parse"]:
    doc = json.loads(c["source"])
    if doc.get("workspaces", []) != c["workspaces"]:
        failures.append("parse " + c["name"])
for c in corpus["rejects"]:
    try:
        doc = json.loads(c["source"])
    except ValueError:
        continue
    ok = isinstance(doc, dict) and ("workspaces" not in doc or (isinstance(doc["workspaces"], list) and all(isinstance(w, str) for w in doc["workspaces"])))
    if ok:
        failures.append("accepted " + c["name"])
''' + lib.PY_TAIL
lib.write_unit(UM, corpus_m, schema_m, ts_m, orc_m)
# ------------- metadata + data-directories
UD = "🗂workspaces/📁data-directories"
UMD = "🗂workspaces/🧬metadata"
meta = {"constants": {"SEMIO_ROOT_DIR": ".🧬semio", "REPO_META_DIR_NAME": "🦑" + V + "repo", "HUB_DATA_DIR_NAME": "🌐hub", "SPACE_DATA_DIR_NAME": "🔗space", "MAP_CACHE_DIR_NAME": "🗺" + V + "map", "TICKET_GENERATED_OUTPUT_DIRECTORY": "🗑" + V + "temp"}, "roots": ["/repo", "/a b/c", "rel/root"]}
corpus_md = {"version": 1, **meta, "semio": [{"root": r, "expected": r + "/.🧬semio"} for r in meta["roots"]], "repo": [{"root": r, "expected": r + "/.🧬semio/🦑" + V + "repo"} for r in meta["roots"]]}
schema_md = S.obj({"version": S.const(1), "constants": S.map(S.str), "roots": S.strs(1), "semio": S.arr(S.obj({"root": S.str, "expected": S.str}), 1), "repo": S.arr(S.obj({"root": S.str, "expected": S.str}), 1)})
body_md = '''const native=(path:string):string=>path.split("/").join(sep);
test("the directory name constants are the documented ones",()=>{expect({SEMIO_ROOT_DIR,REPO_META_DIR_NAME,HUB_DATA_DIR_NAME,SPACE_DATA_DIR_NAME,MAP_CACHE_DIR_NAME,TICKET_GENERATED_OUTPUT_DIRECTORY}).toEqual(corpus.constants);});
for(const row of corpus.semio)test("semio root of "+row.root,()=>{expect(getSemioRoot(native(row.root))).toBe(native(row.expected));});
for(const row of corpus.repo)test("repository metadata of "+row.root,()=>{expect(getRepoMetaDir(native(row.root))).toBe(native(row.expected));});'''
ts_md = lib.ts_wrap(UMD, "🧬️ Contract: workspace metadata lives under `<root>/.🧬semio/🦑️repo` with fixed directory names; the corpus is language-agnostic and `@@O@@` joins the same paths with `os.path`.",
 'import {sep} from "node:path";\nimport {HUB_DATA_DIR_NAME,MAP_CACHE_DIR_NAME,REPO_META_DIR_NAME,SEMIO_ROOT_DIR,SPACE_DATA_DIR_NAME,TICKET_GENERATED_OUTPUT_DIRECTORY,getRepoMetaDir,getSemioRoot} from "@@' + UMD + '/🟦.ts@@";', body_md)
orc_md = lib.PY_HEAD.format(doc="🐍️ Independent path model of the workspace metadata directories (stdlib os.path).") + r'''import os
for c in corpus["semio"]:
    if os.path.join(c["root"], corpus["constants"]["SEMIO_ROOT_DIR"]) != c["expected"] and os.sep == "/":
        failures.append("semio " + c["root"])
for c in corpus["repo"]:
    if os.path.join(c["root"], corpus["constants"]["SEMIO_ROOT_DIR"], corpus["constants"]["REPO_META_DIR_NAME"]) != c["expected"] and os.sep == "/":
        failures.append("repo " + c["root"])
''' + lib.PY_TAIL
lib.write_unit(UMD, corpus_md, schema_md, ts_md, orc_md)
corpus_dd = {"version": 1, "hub": [{"root": r, "expected": r + "/.🧬semio/🌐hub"} for r in meta["roots"]], "space": [{"root": r, "instance": None, "expected": r + "/.🧬semio/🔗space"} for r in meta["roots"]] + [{"root": "/r", "instance": "main", "expected": "/r/.🧬semio/🔗space/main"}, {"root": "/r", "instance": "", "expected": "/r/.🧬semio/🔗space"}],
             "map": [{"root": "/r", "provider": "osm", "expected": "/r/.🧬semio/🗺" + V + "map/osm"}, {"root": "/r", "provider": "a/b", "expected": "/r/.🧬semio/🗺" + V + "map/a/b"}]}
schema_dd = S.obj({"version": S.const(1), "hub": S.arr(S.obj({"root": S.str, "expected": S.str}), 1), "space": S.arr(S.obj({"root": S.str, "instance": S.one(S.str, S.null), "expected": S.str}), 1), "map": S.arr(S.obj({"root": S.str, "provider": S.str, "expected": S.str}), 1)})
body_dd = '''const native=(path:string):string=>path.split("/").join(sep);
for(const row of corpus.hub)test("hub data directory of "+row.root,()=>{expect(getHubDataDir(native(row.root))).toBe(native(row.expected));});
for(const row of corpus.space)test("space data directory of "+row.root+" instance "+JSON.stringify(row.instance),()=>{expect(getSpaceDataDir(native(row.root),row.instance??undefined)).toBe(native(row.expected));});
for(const row of corpus.map)test("map cache of "+row.provider,()=>{expect(getMapCacheDir(native(row.root),row.provider)).toBe(native(row.expected));});'''
ts_dd = lib.ts_wrap(UD, "📁️ Contract: hub, space and map-cache data directories hang below the semio root; the space directory appends a non-empty instance and the map cache a provider; the corpus is language-agnostic and `@@O@@` joins the same paths with `os.path`.",
 'import {sep} from "node:path";\nimport {getHubDataDir,getMapCacheDir,getSpaceDataDir} from "@@' + UD + '/🟦.ts@@";', body_dd)
orc_dd = lib.PY_HEAD.format(doc="🐍️ Independent path model of the data directories (stdlib os.path).") + r'''import os
if os.sep == "/":
    for c in corpus["hub"]:
        if os.path.join(c["root"], ".🧬semio", "🌐hub") != c["expected"]:
            failures.append("hub")
    for c in corpus["space"]:
        base = os.path.join(c["root"], ".🧬semio", "🔗space")
        if (os.path.join(base, c["instance"]) if c["instance"] else base) != c["expected"]:
            failures.append("space")
    for c in corpus["map"]:
        if os.path.join(c["root"], ".🧬semio", "🗺" + VS + "map", c["provider"]) != c["expected"]:
            failures.append("map")
''' + lib.PY_TAIL
lib.write_unit(UD, corpus_dd, schema_dd, ts_dd, orc_dd)
