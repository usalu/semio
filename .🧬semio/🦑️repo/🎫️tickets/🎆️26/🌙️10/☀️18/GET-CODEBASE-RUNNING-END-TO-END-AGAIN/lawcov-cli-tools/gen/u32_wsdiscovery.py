import json, lib
from lib import S
V = "️"
UW = "🗂workspaces"
def pj(**kw): return json.dumps(kw)
main = {"apps/web/package.json": pj(name="web"), "apps/web/dist/package.json": pj(name="web-dist"), "libs/a/package.json": pj(name="a", exports="./payload/web/index.js"), "libs/a/payload/web/package.json": pj(name="a"), "libs/b/package.json": pj(name="b"), "libs/b/nested/package.json": pj(name="nested"),
        "node_modules/x/package.json": pj(name="x"), ".hidden/package.json": pj(name="hidden"), "temp/package.json": pj(name="temp"), "compose/p/package.json": pj(name="composed"), "🗑" + V + "generated/g/package.json": pj(name="gen"), "🤖" + V + "generated/g/package.json": pj(name="gen2"), "target/t/package.json": pj(name="t"),
        "no-name/package.json": "{}", "libs/b/readme.md": "x", "plain/file.txt": "x", "coverage/c/package.json": pj(name="cov"), "storybook-static/s/package.json": pj(name="sb"), "🔌" + V + "plugin-modules/m/package.json": pj(name="pm"), "build/b/package.json": pj(name="bb"), "numeric/package.json": pj(name=5)}
expected = ["apps/web", "libs/a", "libs/b", "libs/b/nested", "no-name", "numeric"]
cases_err = [
 {"name": "two packages with the same name", "extra": {"dup1/package.json": pj(name="dup"), "dup2/package.json": pj(name="dup")}, "message": "duplicate package name \"dup\" at both \"dup1\" and \"dup2\""},
 {"name": "a malformed manifest", "extra": {"bad/package.json": "{"}, "message": "Workspace manifest is malformed: "},
 {"name": "a manifest that is an array", "extra": {"arr/package.json": "[]"}, "message": "Workspace manifest must contain an object"},
 {"name": "a manifest that is a directory", "extra": {"dirm/package.json/inner.txt": "x"}, "message": "must be a regular file"},
]
membership = [
 {"name": "a fresh membership", "current": expected, "missing": [], "stale": [], "fresh": True, "orderChanged": False},
 {"name": "a reordered membership", "current": ["libs/a", "apps/web", "libs/b", "libs/b/nested", "no-name", "numeric"], "missing": [], "stale": [], "fresh": False, "orderChanged": True},
 {"name": "missing and stale entries", "current": ["apps/web", "gone/pkg", "libs/a"], "missing": ["libs/b", "libs/b/nested", "no-name", "numeric"], "stale": ["gone/pkg"], "fresh": False, "orderChanged": False},
 {"name": "an empty membership", "current": [], "missing": expected, "stale": [], "fresh": False, "orderChanged": False},
 {"name": "a duplicate entry makes it stale-free but not fresh", "current": expected + ["apps/web"], "missing": [], "stale": [], "fresh": False, "orderChanged": True},
]
env = [
 {"name": "the execution workspace wins and is trimmed", "env": {"SEMIO_WORKSPACE_ROOT": "  /exec/root  ", "REPO_ROOT": "/other"}, "expected": "/exec/root"}, {"name": "REPO_ROOT is next", "env": {"REPO_ROOT": " /repo/root "}, "expected": "/repo/root"},
 {"name": "blank values are ignored", "env": {"SEMIO_WORKSPACE_ROOT": "   ", "REPO_ROOT": ""}, "expected": None}, {"name": "a relative value is resolved", "env": {"REPO_ROOT": "rel/root"}, "expected": "REL"},
]
corpus = {"version": 1, "tree": main, "expected": expected, "errors": cases_err, "membership": membership, "env": env, "progressDirs": ["", "apps", "apps/web", "libs", "libs/a", "libs/a/payload", "libs/a/payload/web", "libs/b", "libs/b/nested", "no-name", "numeric", "plain"],
          "workspaceMarker": {"name": "workspace", "nested": "a/b/c"}}
schema = S.obj({"version": S.const(1), "tree": S.map(S.str), "expected": S.strs(), "errors": S.arr(S.obj({"name": S.str, "extra": S.map(S.str), "message": S.str}), 1),
 "membership": S.arr(S.obj({"name": S.str, "current": S.strs(), "missing": S.strs(), "stale": S.strs(), "fresh": S.bool, "orderChanged": S.bool}), 1), "env": S.arr(S.obj({"name": S.str, "env": S.map(S.str), "expected": S.one(S.str, S.null)}), 1), "progressDirs": S.strs(1), "workspaceMarker": S.obj({"name": S.str, "nested": S.str})})
body = '''const roots:string[]=[];afterAll(()=>{process.chdir(original);for(const root of roots)rmSync(root,{recursive:true,force:true});});
const original=process.cwd();
const build=(files:Record<string,string>):string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"workspaces-")));roots.push(root);for(const[path,text]of Object.entries(files)){const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}return root;};
const virtual=(files:Record<string,string>,root:string):PackagePayloadOperations=>{
 const table=new Map<string,string>(Object.entries(files).map(([path,text])=>[join(root,...path.split("/")),text])),directories=new Set<string>([root]);
 for(const path of table.keys()){let parent=dirname(path);while(parent.length>=root.length){directories.add(parent);if(parent===root)break;parent=dirname(parent);}}
 return{list:path=>directories.has(path)?[...new Set([...table.keys(),...directories].filter(other=>dirname(other)===path&&other!==path))].sort().map(other=>({name:other.slice(path.length+1),kind:directories.has(other)?"directory":"file"} as PackagePayloadEntry)):[],readText:path=>table.get(path)??"",state:path=>directories.has(path)&&path!==root?"directory":table.has(path)?"file":path===root?"directory":"missing"};
};
test("physical packages are emitted once, payloads and skipped trees never",()=>{const root=build(corpus.tree);expect(computeWorkspaces(root)).toEqual(corpus.expected);});
test("the same discovery over injected operations agrees with the physical one",()=>{const root=resolve("/virtual-root");expect(computeWorkspaces(root,{operations:virtual(corpus.tree,root)})).toEqual(corpus.expected);});
for(const row of corpus.errors)test("discovery fails on "+row.name,()=>{const root=build({...corpus.tree,...row.extra});expect(()=>computeWorkspaces(root)).toThrow(row.message);});
test("a missing or non-directory root is refused",()=>{const root=build({"file.txt":"x"});expect(()=>computeWorkspaces(join(root,"missing"))).toThrow("Workspace root must be a physical directory");expect(()=>computeWorkspaces(join(root,"file.txt"))).toThrow("Workspace root must be a physical directory");});
test("progress reports every scanned directory and a raised signal cancels",()=>{
 const root=build(corpus.tree),seen:string[]=[];computeWorkspaces(root,{onProgress:progress=>seen.push(progress.relativeDirectory)});
 for(const directory of corpus.progressDirs)expect(seen,directory).toContain(directory);expect(seen[0]).toBe("");expect(seen.some(directory=>directory.startsWith("node_modules")||directory.startsWith("target"))).toBe(false);
 expect(()=>computeWorkspaces(root,{signal:{aborted:true}})).toThrow("Workspace discovery cancelled");
 let calls=0;expect(()=>computeWorkspaces(root,{signal:{get aborted(){calls+=1;return calls>3;}}})).toThrow("Workspace discovery cancelled");
});
for(const row of corpus.membership)test("membership comparison: "+row.name,()=>{
 const root=build(corpus.tree),diff=diffWorkspaces(root,row.current),comparison=compareWorkspaceMembership(root,row.current);
 expect(diff.expected).toEqual(corpus.expected);expect(diff.missing).toEqual(row.missing);expect(diff.stale).toEqual(row.stale);
 expect({fresh:comparison.fresh,orderChanged:comparison.orderChanged,missing:comparison.missing,stale:comparison.stale,expected:comparison.expected}).toEqual({fresh:row.fresh,orderChanged:row.orderChanged,missing:row.missing,stale:row.stale,expected:corpus.expected});
});
for(const row of corpus.env)test("workspace root from the environment: "+row.name,()=>{
 const root=build({"package.json":JSON.stringify({name:"unrelated"})});process.chdir(root);
 try{const found=getWorkspaceRoot(row.env);expect(found).toBe(row.expected===null?root:row.expected==="REL"?resolve("rel/root"):resolve(row.expected));}finally{process.chdir(original);}
});
test("without hints the nearest package named workspace above the working directory is the root",()=>{
 const root=build({"package.json":JSON.stringify({name:corpus.workspaceMarker.name}),[corpus.workspaceMarker.nested+"/package.json"]:JSON.stringify({name:"inner"}),[corpus.workspaceMarker.nested+"/x.txt"]:"x"});
 try{process.chdir(join(root,...corpus.workspaceMarker.nested.split("/")));expect(realpathSync(getWorkspaceRoot({}))).toBe(root);}finally{process.chdir(original);}
 const bare=build({"a/b/x.txt":"x"});try{process.chdir(join(bare,"a","b"));expect(realpathSync(getWorkspaceRoot({}))).toBe(realpathSync(process.cwd()));}finally{process.chdir(original);}
 const broken=build({"package.json":"{","a/x.txt":"x"});try{process.chdir(join(broken,"a"));expect(typeof getWorkspaceRoot({})).toBe("string");}finally{process.chdir(original);}
});'''
ts = lib.ts_wrap(UW, "🗂️ Contract: workspace discovery lists every directory that carries its own manifest below the root without descending into build, vendor, hidden and scratch trees, drops payload packages owned through an export namespace, rejects duplicate names and malformed manifests, reports progress and honours cancellation, and the membership comparison separates missing, stale and reordered entries; the corpus is language-agnostic and `@@O@@` walks the same tree with os.walk.",
 'import {afterAll} from "bun:test";\nimport {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join,resolve} from "node:path";\nimport type {PackagePayloadEntry,PackagePayloadOperations} from "@@🗂workspaces/📦payload/🟦.ts@@";\nimport {compareWorkspaceMembership} from "@@🗂workspaces/⚖membership-comparison/🟦.ts@@";\nimport {computeWorkspaces,diffWorkspaces,getWorkspaceRoot} from "@@' + UW + '/🟦.ts@@";', body)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent model of the workspace discovery: os.walk with the same skip rules and the export-namespace ownership (stdlib only).") + r'''import os, tempfile, shutil

SKIP = {"🗑" + VS + "generated", "node_modules", "target", "dist", "build", "🤖" + VS + "generated", "storybook-static", "temp", "coverage", "🔌" + VS + "plugin-modules", ".🧬semio", "compose"}


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
    for c, e in items:
        out += targets(e, False)
        if c == "default":
            break
    return out


def owns(owner, payload):
    if not owner.get("name") or owner["name"] != payload.get("name"):
        return False
    prefix = os.path.relpath(payload["abs"], owner["abs"]).replace(os.sep, "/") + "/"
    for t in targets(owner.get("exports")):
        if not t.startswith("./") or re.search(r"[\\:*?%#\x00]", t):
            continue
        segs = t[2:].split("/")
        if any(s in ("", ".", "..", "node_modules") for s in segs) or not "/".join(segs).startswith(prefix):
            continue
        return True
    return False


root = tempfile.mkdtemp(prefix="workspaces-oracle-")
try:
    for p, t in corpus["tree"].items():
        f = os.path.join(root, *p.split("/"))
        os.makedirs(os.path.dirname(f), exist_ok=True)
        open(f, "w", encoding="utf8").write(t)
    cands = {}
    for dp, dn, fn in os.walk(root):
        dn[:] = sorted(d for d in dn if not d.startswith(".") and d not in SKIP)
        for d in dn:
            m = os.path.join(dp, d, "package.json")
            if os.path.isfile(m):
                doc = json.load(open(m, encoding="utf8"))
                cands[os.path.join(dp, d)] = {"abs": os.path.join(dp, d), "rel": os.path.relpath(os.path.join(dp, d), root).replace(os.sep, "/"), "name": doc.get("name") if isinstance(doc.get("name"), str) else None, "exports": doc.get("exports")}
    keep = []
    for c in cands.values():
        parent = os.path.dirname(c["abs"])
        ok = True
        while parent != root and parent != os.path.dirname(parent):
            if parent in cands:
                ok = not owns(cands[parent], c)
                break
            parent = os.path.dirname(parent)
        if ok:
            keep.append(c["rel"])
    if sorted(keep) != sorted(corpus["expected"]):
        failures.append("expected %r" % sorted(keep))
    for m in corpus["membership"]:
        cur = m["current"]
        missing = [e for e in corpus["expected"] if e not in set(cur)]
        stale = [e for e in cur if e not in set(corpus["expected"])]
        fresh = cur == corpus["expected"]
        if (missing, stale, fresh) != (m["missing"], m["stale"], m["fresh"]) or (not fresh and not missing and not stale) != m["orderChanged"]:
            failures.append("membership " + m["name"])
finally:
    shutil.rmtree(root, ignore_errors=True)
''' + lib.PY_TAIL
lib.write_unit(UW, corpus, schema, ts, oracle)
