import json, lib
from lib import S
UNIT = "🗂workspaces/📣publication"
pj = lambda **kw: json.dumps(kw)
tree = {"a/package.json": pj(name="a"), "b/c/package.json": pj(name="c"), "skip/node_modules/x/package.json": pj(name="x")}
expected = ["a", "b/c"]
root_doc = {"name": "root", "version": "1.0.0", "workspaces": ["a"], "scripts": {"t": "x"}}
fresh_doc = dict(root_doc, workspaces=expected)
rendered = json.dumps(fresh_doc, indent=2, ensure_ascii=False) + "\n"
scenarios = {
 "stale": {"document": root_doc, "mode": "check", "throws": "Root workspace membership is stale", "stderr": ["root package.json workspaces is stale (2 expected, 1 current).", "  missing: b/c", "run `bun ./📜️script.ts workspaces --write` to refresh."], "stdout": []},
 "stalePlusStaleEntries": {"document": dict(root_doc, workspaces=["a", "old/gone"]), "mode": "check", "throws": "Root workspace membership is stale", "stderr": ["  missing: b/c", "  stale:   old/gone"], "stdout": []},
 "reordered": {"document": dict(root_doc, workspaces=["b/c", "a"]), "mode": "check", "throws": "Root workspace membership is stale", "stderr": ["  (same set, different order)"], "stdout": []},
 "fresh": {"document": fresh_doc, "mode": "check", "throws": None, "stderr": [], "stdout": ["root package.json workspaces is fresh (2 packages)."], "written": False, "fresh": True},
 "writeStale": {"document": root_doc, "mode": "write", "throws": None, "stderr": [], "stdout": ["root package.json workspaces regenerated -> 2 packages."], "written": True, "fresh": False},
 "writeFresh": {"document": fresh_doc, "mode": "write", "throws": None, "stderr": [], "stdout": ["root package.json workspaces already fresh (2 packages) — no write needed."], "written": False, "fresh": True},
}
corpus = {"version": 1, "tree": tree, "expected": expected, "rootDocument": root_doc, "rendered": rendered, "scenarios": scenarios}
sc = S.obj({"document": S.anyv, "mode": S.enum("check", "write"), "throws": S.one(S.str, S.null), "stderr": S.strs(), "stdout": S.strs(), "written": S.bool, "fresh": S.bool}, required=["document", "mode", "throws", "stderr", "stdout"])
schema = S.obj({"version": S.const(1), "tree": S.map(S.str), "expected": S.strs(), "rootDocument": S.anyv, "rendered": S.str, "scenarios": S.map(sc)})
body = '''const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const build=(document:unknown):string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"publication-")));roots.push(root);for(const[path,text]of Object.entries(corpus.tree)){const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}writeFileSync(join(root,"package.json"),JSON.stringify(document,null,2)+"\\n");return root;};
const collect=()=>{const out:{stdout:string[];stderr:string[]}={stdout:[],stderr:[]};return{out,report:(stream:"stderr"|"stdout",message:string)=>{out[stream].push(message);}};};
for(const[name,row]of Object.entries(corpus.scenarios))test("publication: "+name,()=>{
 const root=build(row.document),{out,report}=collect(),before=readFileSync(join(root,"package.json"),"utf8");
 const run=()=>publishWorkspaceMembership(root,row.mode as "check"|"write",{report});
 if(row.throws!==null){expect(run).toThrow(row.throws);expect(readFileSync(join(root,"package.json"),"utf8")).toBe(before);}
 else{const result=run();expect({written:result.written,fresh:result.fresh,expectedCount:result.expectedCount}).toEqual({written:(row as {written:boolean}).written,fresh:(row as {fresh:boolean}).fresh,expectedCount:corpus.expected.length});}
 for(const line of row.stderr)expect(out.stderr,line).toContain(line);for(const line of row.stdout)expect(out.stdout,line).toContain(line);
 if(row.mode==="check")expect(readFileSync(join(root,"package.json"),"utf8")).toBe(before);
});
test("a write regenerates the root document with the discovered membership and a second run is a no-op",()=>{
 const root=build(corpus.rootDocument),first=publishWorkspaceMembership(root,"write",{report:()=>{}});expect(first.written).toBe(true);expect(first.missing).toEqual(["b/c"]);expect(first.stale).toEqual([]);
 const text=readFileSync(join(root,"package.json"),"utf8");expect(JSON.parse(text).workspaces).toEqual(corpus.expected);expect(text.endsWith("\\n")).toBe(true);expect(JSON.parse(text).scripts).toEqual(corpus.rootDocument.scripts);
 expect(publishWorkspaceMembership(root,"write",{report:()=>{}}).written).toBe(false);expect(publishWorkspaceMembership(root,"check",{report:()=>{}}).fresh).toBe(true);
 if(python){const result=Bun.spawnSync([python,"-I",join(here,"@@O@@"),join(here,"@@C@@"),join(root,"package.json")]);expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});}
});
test("a document that changes during discovery is refused",()=>{
 const root=build(corpus.rootDocument);let reads=0;
 expect(()=>publishWorkspaceMembership(root,"write",{report:()=>{},operations:{readText:path=>{reads+=1;const text=readFileSync(path,"utf8");return reads>1?text+" ":text;}}})).toThrow("Root package.json changed during workspace discovery");
});
test("a root document that is not a regular file is refused",()=>{
 const root=build(corpus.rootDocument);for(const state of["symlink","other"]as const)expect(()=>publishWorkspaceMembership(root,"check",{report:()=>{},operations:{state:()=>state}})).toThrow("Root package.json must be a physical regular file ("+state+")");
 expect(()=>publishWorkspaceMembership(join(root,"missing"),"check",{report:()=>{}})).toThrow();
});
test("a malformed or ill-typed root document is refused before any discovery",()=>{
 const root=build(corpus.rootDocument);writeFileSync(join(root,"package.json"),"[]");expect(()=>publishWorkspaceMembership(root,"check",{report:()=>{}})).toThrow("Root package.json must contain an object");
 writeFileSync(join(root,"package.json"),'{"workspaces":"a"}');expect(()=>publishWorkspaceMembership(root,"write",{report:()=>{}})).toThrow("workspaces must be a string array");
});'''
ts = lib.ts_wrap(UNIT, "📣️ Contract: workspace publication compares the root package document's membership with discovery, reports missing, stale and reordered entries on stderr and fails a stale check, rewrites the root document with the discovered membership and a trailing newline on write, writes nothing when already fresh and refuses a document that is not a regular file or changes during discovery; the corpus is language-agnostic and `@@O@@` checks the written file against `json.dumps`.",
 'import {afterAll} from "bun:test";\nimport {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join} from "node:path";\nimport {publishWorkspaceMembership} from "@@' + UNIT + '/🟦.ts@@";', body)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent check of the written root document: the file must equal `json.dumps` of the corpus document with the expected membership (stdlib only).") + r'''
if len(sys.argv) > 2:
    written = open(sys.argv[2], encoding="utf8").read()
    doc = dict(corpus["rootDocument"])
    doc["workspaces"] = corpus["expected"]
    if written != json.dumps(doc, indent=2, ensure_ascii=False) + "\n":
        failures.append("written document")
if corpus["rendered"] != json.dumps(dict(corpus["rootDocument"], workspaces=corpus["expected"]), indent=2, ensure_ascii=False) + "\n":
    failures.append("rendered fixture")
for name, s in corpus["scenarios"].items():
    cur = s["document"]["workspaces"]
    exp = corpus["expected"]
    fresh = cur == exp
    if s["mode"] == "check" and (s["throws"] is None) != fresh:
        failures.append("scenario " + name)
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, oracle)
