import lib
from lib import S
UNIT = "🧼workspace-cleanup/🗑removal"
V = "️"
M = "🎫" + V + "ticket.json"
G = "🗑" + V + "generated"
REPO = "🦑" + V + "repo"
TICKETS = "🎫" + V + "tickets"
MB = 1024 * 1024
DAY = ".🧬semio/" + REPO + "/" + TICKETS + "/26/10/21"
def f(p, text="x"): return {"path": p, "kind": "file", "text": text}
def s(p, size): return {"path": p, "kind": "file", "size": size}
nodes = [f(".🧬semio/old-repo/x.txt", "old"), f(".generation3d-foo-link/data", "12345"), f("pkg/target/CACHEDIR.TAG", "sig"), f("pkg/target/debug/a.o", "obj"), f("src/main.ts", "keep"), f("src/dist/keep.js", "keep"),
 f(DAY + "/DONE/" + M, '{"status":"closed"}'), f(DAY + "/DONE/" + G + "/g.txt", "gen"), s(DAY + "/DONE/fat.bin", 6 * MB), f(DAY + "/DONE/keep.md", "keep"),
 f(DAY + "/OPEN/" + M, '{"status":"open"}'), f(DAY + "/OPEN/" + G + "/o.txt", "open-gen"), f(DAY + "/OPEN/notes.md", "keep"),
 f("bad:name.txt", "illegal"), f("trail./inner", "illegal-dir"), f(".cache/b/t/x", "managed"), f("imgs/vm/image.json", "{}"), f("imgs/vm/fixed.dmg", "img")]
removed = [("misplaced", ".🧬semio/old-repo"), ("root-transient", ".generation3d-foo-link"), ("build-artifact", "pkg/target"), ("ticket-generated", DAY + "/DONE/" + G), ("ticket-file", DAY + "/DONE/fat.bin"), ("windows-illegal", "bad:name.txt"), ("windows-illegal", "trail.")]
survivors = ["src/main.ts", "src/dist/keep.js", DAY + "/DONE/" + M, DAY + "/DONE/keep.md", DAY + "/OPEN/" + M, DAY + "/OPEN/" + G + "/o.txt", DAY + "/OPEN/notes.md", ".cache/b/t/x", "imgs/vm/image.json", "imgs/vm/fixed.dmg"]
gone = ["pkg/target", ".generation3d-foo-link", ".🧬semio/old-repo", DAY + "/DONE/" + G, DAY + "/DONE/fat.bin", "bad:name.txt", "trail."]
remove_path = [
 {"name": "an ordinary directory is removed", "path": "scratch/plain", "dry": False, "ticket": False, "illegal": False, "cargo": False, "expected": True, "exists": False},
 {"name": "a dry run reports success and keeps the directory", "path": "scratch/plain2", "dry": True, "ticket": False, "illegal": False, "cargo": False, "expected": True, "exists": True},
 {"name": "a missing path is refused", "path": "scratch/missing", "dry": False, "ticket": False, "illegal": False, "cargo": False, "expected": False, "exists": False},
 {"name": "a managed storage directory is refused", "path": ".cache/b/t/x", "dry": False, "ticket": False, "illegal": False, "cargo": False, "expected": False, "exists": True},
 {"name": "a native image directory is refused", "path": "imgs/vm", "dry": False, "ticket": False, "illegal": False, "cargo": False, "expected": False, "exists": True},
 {"name": "the generated output of an open ticket is refused even with ticket cleanup", "path": DAY + "/OPEN/" + G, "dry": False, "ticket": True, "illegal": False, "cargo": False, "expected": False, "exists": True},
 {"name": "an open ticket folder is refused", "path": DAY + "/OPEN", "dry": False, "ticket": False, "illegal": False, "cargo": False, "expected": False, "exists": True},
 {"name": "a path outside any cargo output root is refused for incremental removal", "path": "scratch/plain", "dry": False, "ticket": False, "illegal": False, "cargo": True, "expected": False, "exists": True},
 {"name": "the generated output of a closed ticket is removed with ticket cleanup", "path": DAY + "/DONE/" + G, "dry": False, "ticket": True, "illegal": False, "cargo": False, "expected": True, "exists": False},
]
nodes += [f("scratch/plain/a.txt", "p"), f("scratch/plain2/a.txt", "p")]
corpus = {"version": 1, "nodes": nodes, "dryRun": {"removals": [{"kind": k, "path": p} for k, p in sorted(removed, key=lambda r: r[1])], "survivors": survivors + gone}, "applied": {"gone": gone, "survivors": survivors}, "removePath": remove_path, "skipped": {"contains": [".cache/b/t"], "protectedRows": []}}
nd = S.obj({"path": S.str, "kind": S.enum("file"), "text": S.str, "size": S.int}, required=["path", "kind"])
schema = S.obj({"version": S.const(1), "nodes": S.arr(nd, 1), "dryRun": S.obj({"removals": S.arr(S.obj({"kind": S.str, "path": S.str})), "survivors": S.strs()}), "applied": S.obj({"gone": S.strs(), "survivors": S.strs()}),
 "removePath": S.arr(S.obj({"name": S.str, "path": S.str, "dry": S.bool, "ticket": S.bool, "illegal": S.bool, "cargo": S.bool, "expected": S.bool, "exists": S.bool}), 1), "skipped": S.obj({"contains": S.strs(), "protectedRows": S.strs()})})
body = '''const sparse=process.platform==="win32"||process.platform==="darwin"&&false;
const posix=(path:string):string=>path.split(sep).join("/");
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const world=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"removal-")));roots.push(root);for(const node of corpus.nodes){const target=join(root,...node.path.split("/"));mkdirSync(dirname(target),{recursive:true});if((node as {size?:number}).size!==undefined){const fd=openSync(target,"w");ftruncateSync(fd,(node as {size:number}).size);closeSync(fd);}else writeFileSync(target,(node as {text?:string}).text??"");}return root;};
const platformRows=(rows:{kind:string;path:string}[]):{kind:string;path:string}[]=>process.platform==="win32"?rows.filter(row=>row.kind!=="windows-illegal"):rows;
const presence=(root:string,paths:string[]):Record<string,boolean>=>Object.fromEntries(paths.map(path=>[path,existsSync(join(root,...path.split("/")))]));
const savedEnvironment:Record<string,string|undefined>={};
const isolate=():void=>{for(const key of["CARGO_TARGET_DIR","CARGO_BUILD_TARGET_DIR","CARGO_BUILD_BUILD_DIR"]){savedEnvironment[key]=process.env[key];delete process.env[key];}};
const restore=():void=>{for(const[key,value]of Object.entries(savedEnvironment))if(value===undefined)delete process.env[key];else process.env[key]=value;};
test("a dry run lists every removable row, spares open tickets, managed storage and images, and deletes nothing",()=>{
 if(process.platform==="win32")return;
 isolate();const root=world();
 try{
  const report=runWorkspaceClean(root,true);
  expect(report.removals.map(row=>({kind:row.kind,path:posix(row.path)})).sort((a,b)=>a.path<b.path?-1:1)).toEqual(platformRows(corpus.dryRun.removals));
  expect(report.removals.every(row=>row.bytes>=0)).toBe(true);
  for(const path of corpus.dryRun.survivors)expect(existsSync(join(root,...path.split("/"))),path).toBe(true);
  for(const needle of corpus.skipped.contains)expect(report.skippedProtected.map(posix).some(path=>path.includes(needle))).toBe(true);
  expect(report.skippedProtected.map(posix).some(path=>path.includes("/OPEN"))).toBe(false);
 }finally{restore();}
});
test("an applied clean deletes exactly the listed rows",()=>{
 if(process.platform==="win32")return;
 isolate();const root=world();
 try{
  const report=runWorkspaceClean(root,false);
  expect(report.removals.length).toBe(corpus.dryRun.removals.length);
  for(const[path,present]of Object.entries(presence(root,corpus.applied.gone)))expect(present,path).toBe(false);
  for(const[path,present]of Object.entries(presence(root,corpus.applied.survivors)))expect(present,path).toBe(true);
  expect(runWorkspaceClean(root,false).removals).toEqual([]);
 }finally{restore();}
});
for(const row of corpus.removePath)test("remove path: "+row.name,()=>{
 if(process.platform==="win32"&&row.illegal)return;
 isolate();const root=world();
 try{
  const target=join(root,...row.path.split("/"));
  expect(cleanRemovePath(root,target,row.dry,[],row.ticket,row.illegal,row.cargo)).toBe(row.expected);
  expect(existsSync(target)).toBe(row.exists);
 }finally{restore();}
});
test("a protected prefix blocks removal of everything inside it",()=>{
 isolate();const root=world();
 try{
  expect(cleanRemovePath(root,join(root,"scratch","plain"),false,[join(root,"scratch")])).toBe(false);expect(existsSync(join(root,"scratch","plain"))).toBe(true);
  expect(cleanRemovePath(root,join(root,"scratch","plain"),false,[join(root,"scratch","plain","a.txt")])).toBe(false);
 }finally{restore();}
});'''
ts = lib.ts_wrap(UNIT, "🗑️ Contract: the workspace cleaner removes exactly the discovered misplaced folders, root transients, Cargo targets, closed-ticket output and illegal names, and refuses open tickets, managed storage, native images and protected prefixes in dry and applied runs; the corpus is language-agnostic and `@@O@@` replays the expected survivors on a rebuilt workspace.",
 'import {afterAll} from "bun:test";\nimport {closeSync,existsSync,ftruncateSync,mkdirSync,mkdtempSync,openSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join,sep} from "node:path";\nimport {cleanRemovePath,runWorkspaceClean} from "@@' + UNIT + '/🟦.ts@@";', body)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent model of the cleaner outcome: the expected removals and survivors of the corpus workspace must be disjoint, complete and consistent with the node tree (stdlib only).") + r'''
paths = {n["path"] for n in corpus["nodes"]}
gone, alive = corpus["applied"]["gone"], corpus["applied"]["survivors"]


def exists(p):
    return any(q == p or q.startswith(p + "/") for q in paths)


for p in gone + alive:
    if not exists(p):
        failures.append("unknown path " + p)
for g in gone:
    for a in alive:
        if a == g or a.startswith(g + "/") or g.startswith(a + "/"):
            failures.append("overlap %s %s" % (g, a))
if sorted(corpus["dryRun"]["survivors"]) != sorted(alive + gone):
    failures.append("dry survivors")
rows = corpus["dryRun"]["removals"]
if [r["path"] for r in rows] != sorted(r["path"] for r in rows):
    failures.append("rows order")
if sorted(r["path"] for r in rows) != sorted(gone):
    failures.append("rows vs gone")
for r in rows:
    if r["kind"] == "ticket-generated" and "ticket" not in r["path"] and "🎫" not in r["path"]:
        failures.append("ticket-generated outside tickets " + r["path"])
    if r["kind"] == "build-artifact" and not exists(r["path"] + "/CACHEDIR.TAG"):
        failures.append("build artifact without cache tag " + r["path"])
    if r["kind"] == "windows-illegal" and not any(re.search(r'[<>:"|?*]', s) or s.endswith(".") for s in r["path"].split("/")):
        failures.append("legal name listed " + r["path"])
open_ticket = [p for p in paths if "/OPEN/" in p]
if not open_ticket or any(a for a in gone if "/OPEN" in a):
    failures.append("open ticket treated as removable")
for c in corpus["removePath"]:
    if c["expected"] is False and c["exists"] is False and c["path"] in paths:
        failures.append("refused but gone " + c["name"])
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, oracle)
