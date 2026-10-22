import lib
from lib import S
UNIT = "🧼workspace-cleanup/🔍candidate-discovery"
V = "️"
M = "🎫" + V + "ticket.json"
G = "🗑" + V + "generated"
REPO = "🦑" + V + "repo"
TICKETS = "🎫" + V + "tickets"
SEMIO = ".🧬semio"
MB = 1024 * 1024
R = SEMIO + "/" + REPO + "/" + TICKETS
DAY = R + "/26/10/21"
def d(p): return {"path": p, "kind": "directory"}
def f(p, text="x"): return {"path": p, "kind": "file", "text": text}
def s(p, size): return {"path": p, "kind": "file", "size": size}
nodes = [
 f(SEMIO + "/" + REPO + "/keep.txt", "keep"), f(SEMIO + "/old-repo/x.txt", "xx"), f(SEMIO + "/" + REPO + "/old-tickets/t.md", "tttt"), f(REPO + "/stray.txt", "stray!"),
 f("pkg/" + SEMIO + "/pkg-repo/z.txt", "zzz"), f("node_modules/" + SEMIO + "/n-repo/a", "a"), f("target/" + SEMIO + "/t-repo/a", "a"), f("src/" + SEMIO + "/p-repo/a", "aaaa"), f("src/main.ts", "export {}"),
 f(".generation3d-foo-link/data", "12345"), f(".generation3d-bar-link", "123"), f(".w-g3-ticket", "w"), f(".generation3d-keep", "k"), f("other-link", "o"),
 f(DAY + "/BIG/" + M, '{"status":"closed"}'), s(DAY + "/BIG/a.bin", 6 * MB), s(DAY + "/BIG/sub/b.bin", 4 * MB), s(DAY + "/BIG/sub/c.bin", 4 * MB), s(DAY + "/BIG/sub/d.bin", 4 * MB), f(DAY + "/BIG/" + G + "/g.txt", "gen"), f(DAY + "/BIG/small.txt", "s"), f(DAY + "/BIG/deep/x/y/z.txt", "z"),
 f(DAY + "/OPEN/" + M, '{"status":"open"}'), s(DAY + "/OPEN/huge.bin", 101 * MB), f(DAY + "/OPEN/" + G + "/o.txt", "o"), f(DAY + "/NEW/notes.md", "notes"),
 f("pkg/target/CACHEDIR.TAG", "Signature: 8a477f597d28d172789f06886806bc55"), f("pkg/target/debug/a.o", "obj"), f("pkg/dist/x.js", "js"), f("target-x/CACHEDIR.TAG", "sig"), f("node_modules/target/CACHEDIR.TAG", "sig"), f(R + "/target/CACHEDIR.TAG", "sig"), f("build/y.txt", "y"),
]
def size(prefix):
    total = 0
    for n in nodes:
        if n["kind"] == "file" and (n["path"] == prefix or n["path"].startswith(prefix + "/")):
            total += n.get("size", len(n.get("text", "").encode()))
    return total
def row(kind, path): return {"kind": kind, "path": path, "bytes": size(path)}
bytes_cases = [("src/main.ts", 9), ("src", 9 + 4 * 0 + 0), (".generation3d-foo-link", 5), ("nothing", 0), (DAY + "/BIG/sub", 12 * MB)]
bytes_cases = [(p, size(p)) for p in ["src/main.ts", ".generation3d-foo-link", DAY + "/BIG/sub", DAY + "/BIG", "pkg/target"]] + [("nothing", 0)]
misplaced = {
 "all": {"prefixes": [], "expected": [row("misplaced", p) for p in [REPO, SEMIO + "/old-repo", SEMIO + "/" + REPO + "/old-tickets", "pkg/" + SEMIO + "/pkg-repo", "src/" + SEMIO + "/p-repo"]]},
 "protected": {"prefixes": ["src/" + SEMIO + "/p-repo", SEMIO + "/old-repo"], "expected": [row("misplaced", p) for p in [REPO, SEMIO + "/" + REPO + "/old-tickets", "pkg/" + SEMIO + "/pkg-repo"]]},
}
transient = {"prefixes": [], "expected": [row("root-transient", p) for p in [".generation3d-foo-link", ".generation3d-bar-link", ".w-g3-ticket"]], "protected": [".generation3d-bar-link"], "expectedProtected": [row("root-transient", p) for p in [".generation3d-foo-link", ".w-g3-ticket"]]}
folders = [DAY + "/BIG", DAY + "/NEW", DAY + "/OPEN"]
size_removals = [
 {"folder": DAY + "/BIG", "prefixes": [], "expected": [row("ticket-file", DAY + "/BIG/a.bin"), row("ticket-dir", DAY + "/BIG/sub")]},
 {"folder": DAY + "/BIG", "prefixes": [DAY + "/BIG/sub"], "expected": [row("ticket-file", DAY + "/BIG/a.bin")]},
 {"folder": DAY + "/NEW", "prefixes": [], "expected": []},
]
gen_removals = [
 {"folder": DAY + "/BIG", "prefixes": [], "expected": [row("ticket-generated", DAY + "/BIG/" + G)]},
 {"folder": DAY + "/BIG", "prefixes": [DAY + "/BIG/" + G], "expected": []},
 {"folder": DAY + "/OPEN", "prefixes": [], "expected": []},
 {"folder": DAY + "/NEW", "prefixes": [], "expected": []},
]
open_oversized = [
 {"folder": DAY + "/OPEN", "prefixes": [], "expected": [row("ticket-generated", DAY + "/OPEN/huge.bin")]},
 {"folder": DAY + "/OPEN", "prefixes": [DAY + "/OPEN/huge.bin"], "expected": []},
 {"folder": DAY + "/BIG", "prefixes": [], "expected": []},
]
build = {"prefixes": [], "expected": [row("build-artifact", p) for p in ["pkg/target", "target-x"]], "protected": ["target-x"], "expectedProtected": [row("build-artifact", "pkg/target")]}
dedupe = [
 {"rows": [{"kind": "ticket-dir", "path": p, "bytes": 1} for p in ["a/b", "a/b/c", "a", "z"]], "expected": ["a/b/c", "z"]},
 {"rows": [{"kind": "ticket-dir", "path": p, "bytes": 1} for p in ["x/y", "x/yz", "x/y/z", "x/y"]], "expected": ["x/y/z", "x/yz"]},
 {"rows": [], "expected": []},
]
git_repo = {"files": {".gitignore": "big.bin\nignored-dir/\nnode_modules/\n*.tmp\n", "tracked.txt": "t", "big.bin": None, "small.tmp": "s", "ignored-dir/inner.bin": None, "node_modules/pkg/big.bin": None, "keep.txt": "k",
                      ".🧬semio/" + REPO + "/" + TICKETS + "/26/10/21/SLUG/ignored.tmp": "i", ".🧬semio/" + REPO + "/" + TICKETS + "/26/10/21/SLUG/big.bin": None, ".🧬semio/" + REPO + "/" + TICKETS + "/26/10/21/SLUG/" + M: "{}", "tickets/26/10/21/ASCII/ignored.tmp": "i", "tickets/26/10/21/ASCII/readme.md": "r"},
            "sizes": {"big.bin": 101 * MB, "ignored-dir/inner.bin": 101 * MB, "node_modules/pkg/big.bin": 101 * MB, ".🧬semio/" + REPO + "/" + TICKETS + "/26/10/21/SLUG/big.bin": 101 * MB},
            "oversized": [{"kind": "gitignore", "path": "big.bin", "bytes": 101 * MB}, {"kind": "ticket-generated", "path": ".🧬semio/" + REPO + "/" + TICKETS + "/26/10/21/SLUG/big.bin", "bytes": 101 * MB}],
            "ticketRoots": [".🧬semio/" + REPO + "/" + TICKETS], "ignoredMap": {".🧬semio/" + REPO + "/" + TICKETS + "/26/10/21/SLUG": [".🧬semio/" + REPO + "/" + TICKETS + "/26/10/21/SLUG/big.bin", ".🧬semio/" + REPO + "/" + TICKETS + "/26/10/21/SLUG/ignored.tmp"]},
            "asciiRoots": ["tickets"], "asciiMap": {"tickets/26/10/21/ASCII": ["tickets/26/10/21/ASCII/ignored.tmp"]}}
illegal = {"files": {"ok.txt": "ok", "weird:name.txt": "w", "dir/aux": "a", "trail.": "t", "sub/CON.txt": "c", "node_modules/bad:x": "n", "target-x/nul": "x", "good dir/file": "g"}, "tracked": ["ok.txt", "weird:name.txt", "dir/aux", "sub/CON.txt", "good dir/file"],
           "expected": ["dir/aux", "sub/CON.txt", "trail.", "weird:name.txt"], "expectedProtected": ["dir/aux", "sub/CON.txt", "trail."], "protected": ["weird:name.txt"]}
incremental = {"files": {"inc/crate-a/s-1-aaa/dep-graph.bin": "g" * 100, "inc/crate-b/s-2-bbb-working/q.bin": "q" * 100, "inc/crate-b/s-3-ccc/w.bin": "w" * 100, "inc/crate-c/s-4-ddd/e.bin": "e" * 100, "inc/crate-d/notes.txt": "no session"},
               "target": "tgt", "max": 1, "mixedExpected": ["tgt/debug/incremental/crate-a", "tgt/debug/incremental/crate-c"],
               "idleFiles": {"idle/crate-a/s-1-aaa/x.bin": "x", "idle/crate-b/s-2-bbb/y.bin": "y"}, "idleExpected": ["tgt/release/incremental"], "idleUnder": ["tgt/release/incremental/crate-a/s-1-aaa/x.bin", "tgt/release/incremental/crate-b/s-2-bbb/y.bin"],
               "mixed": ["tgt/debug/incremental/crate-a/s-1-aaa/dep-graph.bin", "tgt/debug/incremental/crate-b/s-2-bbb-working/q.bin", "tgt/debug/incremental/crate-b/s-3-ccc/w.bin", "tgt/debug/incremental/crate-c/s-4-ddd/e.bin", "tgt/debug/incremental/crate-d/notes.txt"]}
corpus = {"version": 1, "nodes": nodes, "bytes": [{"path": p, "expected": n} for p, n in bytes_cases], "misplaced": misplaced, "transient": transient, "ticketRoots": [R], "ticketFolders": folders,
 "sizeRemovals": size_removals, "generatedRemovals": gen_removals, "openOversized": open_oversized, "build": build, "dedupe": dedupe, "git": git_repo, "illegal": illegal, "incremental": incremental,
 "thresholds": {"ticketFile": 5 * MB, "ticketDir": 10 * MB, "buildArtifact": 10 * 1024 * MB, "oversizedIgnored": 100 * MB, "openTicketFile": 100 * MB},
 "walk": {"skipNames": ["node_modules", "target"], "stopAt": "src", "visitedAll": ["src", "pkg", "node_modules", "target", "build", "target-x", "other"], "tree": ["src/inner", "pkg/inner", "node_modules/inner", "target/inner", "build/inner", "target-x/inner", "other/inner"]}}
rw = S.obj({"kind": S.str, "path": S.str, "bytes": S.int})
nd = S.obj({"path": S.str, "kind": S.enum("directory", "file"), "text": S.str, "size": S.int}, required=["path", "kind"])
case = lambda extra: S.obj(dict({"folder": S.str, "prefixes": S.strs(), "expected": S.arr(rw)}, **extra))
pe = S.obj({"prefixes": S.strs(), "expected": S.arr(rw), "protected": S.strs(), "expectedProtected": S.arr(rw)})
schema = S.obj({"version": S.const(1), "nodes": S.arr(nd, 1), "bytes": S.arr(S.obj({"path": S.str, "expected": S.int}), 1),
 "misplaced": S.obj({"all": S.obj({"prefixes": S.strs(), "expected": S.arr(rw)}), "protected": S.obj({"prefixes": S.strs(), "expected": S.arr(rw)})}), "transient": pe,
 "ticketRoots": S.strs(), "ticketFolders": S.strs(), "sizeRemovals": S.arr(case({}), 1), "generatedRemovals": S.arr(case({}), 1), "openOversized": S.arr(case({}), 1), "build": pe,
 "dedupe": S.arr(S.obj({"rows": S.arr(rw), "expected": S.strs()}), 1),
 "git": S.obj({"files": S.map(S.one(S.str, S.null)), "sizes": S.map(S.int), "oversized": S.arr(rw), "ticketRoots": S.strs(), "ignoredMap": S.map(S.strs()), "asciiRoots": S.strs(), "asciiMap": S.map(S.strs())}),
 "illegal": S.obj({"files": S.map(S.str), "tracked": S.strs(), "expected": S.strs(), "expectedProtected": S.strs(), "protected": S.strs()}),
 "incremental": S.obj({"files": S.map(S.str), "target": S.str, "max": S.int, "mixedExpected": S.strs(), "idleFiles": S.map(S.str), "idleExpected": S.strs(), "idleUnder": S.strs(), "mixed": S.strs()}),
 "thresholds": S.obj({"ticketFile": S.int, "ticketDir": S.int, "buildArtifact": S.int, "oversizedIgnored": S.int, "openTicketFile": S.int}),
 "walk": S.obj({"skipNames": S.strs(), "stopAt": S.str, "visitedAll": S.strs(), "tree": S.strs()})})
bodyts = '''type Row={kind:string;path:string;bytes:number};
const sparse=process.platform==="win32";
const posix=(path:string):string=>path.split(sep).join("/");
const make=(base:string,path:string,node:{text?:string;size?:number}|"directory")=>{const target=join(base,...path.split("/"));if(node==="directory"){mkdirSync(target,{recursive:true});return;}mkdirSync(dirname(target),{recursive:true});if(node.size!==undefined){const fd=openSync(target,"w");ftruncateSync(fd,node.size);closeSync(fd);}else writeFileSync(target,node.text??"");};
const scratch=():string=>realpathSync(mkdtempSync(join(tmpdir(),"discovery-")));
const absolute=(root:string,path:string):string=>join(root,...path.split("/"));
const rowsOf=(rows:readonly Row[]):Row[]=>rows.map(row=>({kind:row.kind,path:posix(row.path),bytes:row.bytes})).sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0);
const expectedRows=(rows:readonly Row[]):Row[]=>[...rows].sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0);
const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString();};
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const world=(skipSparse:boolean):string=>{const root=scratch();roots.push(root);for(const node of corpus.nodes){if(skipSparse&&"size" in node&&node.size!==undefined&&node.size>50*1024*1024)continue;make(root,node.path,node.kind==="directory"?"directory":node as {text?:string;size?:number});}return root;};
const shared=world(sparse);
test("the thresholds match the corpus",()=>{expect(CLEAN_TICKET_FILE_MAX_BYTES).toBe(corpus.thresholds.ticketFile);expect(CLEAN_TICKET_DIR_MAX_BYTES).toBe(corpus.thresholds.ticketDir);expect(CLEAN_BUILD_ARTIFACT_MAX_BYTES).toBe(corpus.thresholds.buildArtifact);expect(CLEAN_OVERSIZED_IGNORED_FILE_MAX_BYTES).toBe(corpus.thresholds.oversizedIgnored);expect(CLEAN_OPEN_TICKET_OVERSIZED_FILE_MAX_BYTES).toBe(corpus.thresholds.openTicketFile);});
for(const row of corpus.bytes)test("path bytes of "+row.path,()=>{expect(cleanPathBytes(absolute(shared,row.path))).toBe(row.expected);});
test("the directory walk honours enter, skip and stop",()=>{
 const root=scratch();roots.push(root);for(const path of corpus.walk.tree)make(root,path,"directory");
 const seen:string[]=[];cleanWalkDirs(root,(_abs,name)=>{seen.push(name);return corpus.walk.skipNames.includes(name)?"skip":"enter";});
 for(const name of corpus.walk.visitedAll)expect(seen).toContain(name);expect(seen.filter(name=>name==="inner").length).toBe(corpus.walk.tree.length-corpus.walk.skipNames.length);
 const stopped:string[]=[];cleanWalkDirs(root,(_abs,name)=>{stopped.push(name);return name===corpus.walk.stopAt?"stop":"enter";});expect(stopped[stopped.length-1]).toBe(corpus.walk.stopAt);expect(stopped.length).toBeLessThan(seen.length);
});
for(const[name,row]of Object.entries(corpus.misplaced))test("misplaced repository and ticket folders: "+name,()=>{expect(rowsOf(cleanCollectMisplaced(shared,row.prefixes.map(path=>absolute(shared,path))))).toEqual(expectedRows(row.expected));});
test("root transient folders are listed unless protected",()=>{
 expect(rowsOf(cleanCollectRootTransient(shared,[]))).toEqual(expectedRows(corpus.transient.expected));
 expect(rowsOf(cleanCollectRootTransient(shared,corpus.transient.protected.map(path=>absolute(shared,path))))).toEqual(expectedRows(corpus.transient.expectedProtected));
 expect(cleanCollectRootTransient(join(shared,"nothing"),[])).toEqual([]);
});
test("ticket roots and ticket folders follow the canonical depth",()=>{
 expect(cleanDiscoverTicketRoots(shared).map(path=>posix(relative(shared,path))).sort()).toEqual([...corpus.ticketRoots].sort());
 expect(cleanDiscoverTicketFolders(absolute(shared,corpus.ticketRoots[0]!)).map(path=>posix(relative(shared,path))).sort()).toEqual([...corpus.ticketFolders].sort());
});
for(const[index,row]of corpus.sizeRemovals.entries())test("ticket size removals "+index+": "+row.folder,()=>{const root=world(sparse);expect(rowsOf(cleanTicketSizeRemovals(root,absolute(root,row.folder),row.prefixes.map(path=>absolute(root,path))))).toEqual(expectedRows(row.expected.filter(item=>!sparse||item.bytes<50*1024*1024)));});
for(const[index,row]of corpus.generatedRemovals.entries())test("ticket generated removals "+index+": "+row.folder,()=>{expect(rowsOf(cleanTicketGeneratedOutputRemovals(shared,absolute(shared,row.folder),row.prefixes.map(path=>absolute(shared,path))))).toEqual(expectedRows(row.expected));});
for(const[index,row]of corpus.openOversized.entries())test("open ticket oversized files "+index+": "+row.folder,()=>{expect(rowsOf(cleanOpenTicketOversizedFileRemovals(shared,absolute(shared,row.folder),row.prefixes.map(path=>absolute(shared,path))))).toEqual(sparse?[]:expectedRows(row.expected));});
test("build artifacts are Cargo target directories with a cache tag, skipping ticket trees and protected prefixes",()=>{
 expect(rowsOf(cleanBuildArtifactRemovals(shared,[]))).toEqual(expectedRows(corpus.build.expected));
 expect(rowsOf(cleanBuildArtifactRemovals(shared,corpus.build.protected.map(path=>absolute(shared,path))))).toEqual(expectedRows(corpus.build.expectedProtected));
});
for(const[index,row]of corpus.dedupe.entries())test("deepest dedupe "+index,()=>{expect(cleanDedupePreferDeepest(row.rows as CleanRemoval[]).map(item=>item.path).sort()).toEqual([...row.expected].sort());});
const ignoredRepo=():string=>{const root=scratch();roots.push(root);git(root,"init","-q");for(const[path,text]of Object.entries(corpus.git.files)){const size=(corpus.git.sizes as Record<string,number>)[path];make(root,path,size!==undefined?{size}:{text:text??""});}return root;};
const ignoredMap=(root:string,ticketRoots:string[]):Record<string,string[]>=>Object.fromEntries([...cleanGitignoredMapForTicketRoots(root,ticketRoots.map(path=>absolute(root,path))).entries()].map(([folder,list])=>[posix(relative(root,folder)),list.map(path=>posix(relative(root,path))).sort()]));
test("ignored files above the size limit are listed with ticket files kept apart",()=>{
 if(sparse)return;
 expect(rowsOf(cleanOversizedIgnoredRemovals(ignoredRepo(),[]))).toEqual(expectedRows(corpus.git.oversized));
});
test("ignored files of ASCII-named ticket folders are mapped to their folder",()=>{expect(ignoredMap(ignoredRepo(),corpus.git.asciiRoots)).toEqual(corpus.git.asciiMap);});
test.failing("ignored files of emoji-named ticket folders are mapped to their folder",()=>{expect(ignoredMap(ignoredRepo(),corpus.git.ticketRoots)).toEqual(corpus.git.ignoredMap);});
test("windows-illegal names are found on disk and in the index",()=>{
 if(process.platform==="win32")return;
 const root=scratch();roots.push(root);git(root,"init","-q");
 for(const[path,text]of Object.entries(corpus.illegal.files))make(root,path,{text});
 git(root,"add","--",...corpus.illegal.tracked);
 expect(cleanCollectWindowsIllegal(root,[]).map(row=>posix(row.path)).sort()).toEqual(corpus.illegal.expected);
 expect(cleanCollectWindowsIllegal(root,corpus.illegal.protected.map(path=>absolute(root,path))).map(row=>posix(row.path)).sort()).toEqual(corpus.illegal.expectedProtected);
});
test("rustc incremental caches over the limit are listed per profile or per idle crate",()=>{
 const saved:Record<string,string|undefined>={};for(const key of["CARGO_TARGET_DIR","CARGO_BUILD_TARGET_DIR","CARGO_BUILD_BUILD_DIR"]){saved[key]=process.env[key];delete process.env[key];}
 const root=scratch();roots.push(root);
 try{
  make(root,".cargo/config.toml",{text:'[build]\\ntarget-dir = "'+corpus.incremental.target+'"\\n'});
  for(const[path,text]of Object.entries(corpus.incremental.files))make(root,corpus.incremental.target+"/debug/incremental/"+path.slice("inc/".length),{text});
  for(const[path,text]of Object.entries(corpus.incremental.idleFiles))make(root,corpus.incremental.target+"/release/incremental/"+path.slice("idle/".length),{text});
  const rows=cleanCollectRustIncrementalRemovals(root,corpus.incremental.max);
  expect(rows.every(row=>row.kind==="cargo-incremental"&&row.bytes>0)).toBe(true);
  expect(rows.map(row=>posix(row.path)).sort()).toEqual([...corpus.incremental.mixedExpected,...corpus.incremental.idleExpected].sort());
  expect(cleanCollectRustIncrementalRemovals(root,Number.MAX_SAFE_INTEGER)).toEqual([]);
 }finally{for(const[key,value]of Object.entries(saved))if(value===undefined)delete process.env[key];else process.env[key]=value;}
});'''
ts = lib.ts_wrap(UNIT, "🔍️ Contract: cleanup candidate discovery lists misplaced repository folders, root transients, ticket folders over their size limits, closed-ticket generated output, oversized ignored files, Windows-illegal names, Cargo targets and rustc incremental caches in a synthetic workspace without ever listing an open ticket; the corpus is language-agnostic and `@@O@@` rebuilds the workspace and recomputes the filesystem-only verdicts.",
 'import {afterAll} from "bun:test";\nimport {closeSync,ftruncateSync,mkdirSync,mkdtempSync,openSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join,relative,sep} from "node:path";\nimport {CLEAN_BUILD_ARTIFACT_MAX_BYTES,CLEAN_OPEN_TICKET_OVERSIZED_FILE_MAX_BYTES,CLEAN_OVERSIZED_IGNORED_FILE_MAX_BYTES,CLEAN_TICKET_DIR_MAX_BYTES,CLEAN_TICKET_FILE_MAX_BYTES,cleanBuildArtifactRemovals,cleanCollectMisplaced,cleanCollectRootTransient,cleanCollectRustIncrementalRemovals,cleanCollectWindowsIllegal,cleanDedupePreferDeepest,cleanDiscoverTicketFolders,cleanDiscoverTicketRoots,cleanGitignoredMapForTicketRoots,cleanOpenTicketOversizedFileRemovals,cleanOversizedIgnoredRemovals,cleanPathBytes,cleanTicketGeneratedOutputRemovals,cleanTicketSizeRemovals,cleanWalkDirs} from "@@' + UNIT + '/🟦.ts@@";\nimport type {CleanRemoval} from "@@🧼workspace-cleanup/🛡protection/🟦.ts@@";', bodyts)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent model of the cleanup candidate discovery: rebuilds the corpus workspace (sparse files via truncate) in a scratch directory and recomputes the filesystem-only verdicts with os.walk (stdlib only).") + r'''import os, tempfile, shutil

root = tempfile.mkdtemp(prefix="discovery-oracle-")
MB = 1024 * 1024
M = next(n["path"] for n in corpus["nodes"] if n["path"].endswith("ticket.json")).rsplit("/", 1)[1]
for n in corpus["nodes"]:
    target = os.path.join(root, *n["path"].split("/"))
    if n["kind"] == "directory":
        os.makedirs(target, exist_ok=True)
        continue
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "wb") as fh:
        if "size" in n:
            fh.truncate(n["size"])
        else:
            fh.write(n.get("text", "").encode())


def rel(p):
    return os.path.relpath(p, root).replace(os.sep, "/")


def path_bytes(p):
    if os.path.islink(p) or os.path.isfile(p):
        return os.lstat(p).st_size
    return sum(path_bytes(os.path.join(p, c)) for c in os.listdir(p)) if os.path.isdir(p) else 0


def under(p, prefixes):
    return any(p == q or p.startswith(q + os.sep) for q in prefixes)


def intersects(p, prefixes):
    return any(p == q or p.startswith(q + os.sep) or q.startswith(p + os.sep) for q in prefixes)


def absl(lst):
    return [os.path.join(root, *p.split("/")) for p in lst]


def walk_dirs(top, visit):
    stack = [top]
    while stack:
        d = stack.pop()
        try:
            names = os.listdir(d)
        except OSError:
            continue
        for name in names:
            a = os.path.join(d, name)
            if not os.path.isdir(a) or os.path.islink(a):
                continue
            act = visit(a, name)
            if act == "stop":
                return
            if act == "enter":
                stack.append(a)


REPO = corpus["nodes"][0]["path"].split("/")[1]
SEMIO_ROOT = lambda n: n == ".🧬semio" or (n.startswith(".🧬") and n.endswith("semio"))
BUILD = lambda n: n in ("target", "dist", "build", "out") or n == "target" or n.startswith("target-")
SKIP = {"node_modules", ".git", "⚡️cache"}
TICKETS = "🎫️tickets"


def misplaced(prefixes):
    out = {}

    def push(a):
        if not intersects(a, prefixes):
            out[rel(a)] = {"kind": "misplaced", "path": rel(a), "bytes": path_bytes(a)}
    if os.path.exists(os.path.join(root, REPO)):
        push(os.path.join(root, REPO))

    def visit(a, name):
        if name in SKIP or BUILD(name):
            return "skip"
        if SEMIO_ROOT(name):
            for c in os.listdir(a):
                ca = os.path.join(a, c)
                if not os.path.isdir(ca):
                    continue
                if c.endswith("repo") and c != REPO:
                    push(ca)
                if c == REPO:
                    for t in os.listdir(ca):
                        if t.endswith("tickets") and t != TICKETS and os.path.isdir(os.path.join(ca, t)):
                            push(os.path.join(ca, t))
            return "skip"
        return "enter"
    walk_dirs(root, visit)
    return sorted(out.values(), key=lambda r: r["path"])


for key, c in corpus["misplaced"].items():
    if misplaced(absl(c["prefixes"])) != sorted(c["expected"], key=lambda r: r["path"]):
        failures.append("misplaced " + key)
t = corpus["transient"]
for prefixes, expected in ((t["prefixes"], t["expected"]), (t["protected"], t["expectedProtected"])):
    got = [{"kind": "root-transient", "path": n, "bytes": path_bytes(os.path.join(root, n))} for n in sorted(os.listdir(root)) if (re.match(r"^\.generation3d-.+-link$", n) or n == ".w-g3-ticket") and not intersects(os.path.join(root, n), absl(prefixes))]
    if got != sorted(expected, key=lambda r: r["path"]):
        failures.append("transient")
roots_found = []


def find_roots(a, name):
    if name in SKIP or BUILD(name):
        return "skip"
    if name == TICKETS:
        roots_found.append(rel(a))
        return "skip"
    return "enter"


walk_dirs(root, find_roots)
if sorted(roots_found) != sorted(corpus["ticketRoots"]):
    failures.append("ticket roots")
troot = os.path.join(root, *corpus["ticketRoots"][0].split("/"))
folders = []
for y in os.listdir(troot):
    ya = os.path.join(troot, y)
    for m_ in (os.listdir(ya) if os.path.isdir(ya) else []):
        ma = os.path.join(ya, m_)
        for dd in (os.listdir(ma) if os.path.isdir(ma) else []):
            da = os.path.join(ma, dd)
            for slug in (os.listdir(da) if os.path.isdir(da) else []):
                if os.path.isdir(os.path.join(da, slug)):
                    folders.append(rel(os.path.join(da, slug)))
if sorted(folders) != sorted(corpus["ticketFolders"]):
    failures.append("ticket folders")


def closed(folder):
    try:
        doc = json.load(open(os.path.join(folder, M), encoding="utf8"))
    except (OSError, ValueError):
        return False
    return isinstance(doc, dict) and doc.get("status") == "closed"


def size_removals(folder, prefixes):
    out = []

    def node(a):
        if under(a, prefixes):
            return None
        if os.path.islink(a) or os.path.isfile(a):
            return (a, False, os.lstat(a).st_size, [])
        kids = [k for k in (node(os.path.join(a, n)) for n in sorted(os.listdir(a))) if k]
        return (a, True, sum(k[2] for k in kids), kids)

    def visit(n):
        for k in n[3]:
            visit(k)
        if not n[1] and n[2] > corpus["thresholds"]["ticketFile"]:
            out.append({"kind": "ticket-file", "path": rel(n[0]), "bytes": n[2]})
        elif n[1] and n[2] > corpus["thresholds"]["ticketDir"]:
            out.append({"kind": "ticket-dir", "path": rel(n[0]), "bytes": n[2]})
    tree = node(folder)
    for k in (tree[3] if tree else []):
        visit(k)
    return sorted(out, key=lambda r: r["path"])


for c in corpus["sizeRemovals"]:
    if size_removals(os.path.join(root, *c["folder"].split("/")), absl(c["prefixes"])) != sorted(c["expected"], key=lambda r: r["path"]):
        failures.append("size removals " + c["folder"])


def gen_removals(folder, prefixes):
    out = []
    GEN = set(corpus_generated)

    def visit(a, name):
        if name not in GEN and not name.startswith(("🧪️purity-", "🧪️cli-", "🧪️inventory-")):
            return "enter"
        if intersects(a, [p for p in prefixes if p != folder]) or not closed(folder):
            return "skip"
        out.append({"kind": "ticket-generated", "path": rel(a), "bytes": path_bytes(a)})
        return "skip"
    walk_dirs(folder, visit)
    return sorted(out, key=lambda r: r["path"])


corpus_generated = ["🗑️generated", "🗑️temp", "🧾️runs", "🧪️runs", "🧾️taxonomy-transaction"]
for c in corpus["generatedRemovals"]:
    if gen_removals(os.path.join(root, *c["folder"].split("/")), absl(c["prefixes"])) != sorted(c["expected"], key=lambda r: r["path"]):
        failures.append("generated removals " + c["folder"])


def open_oversized(folder, prefixes):
    if closed(folder):
        return []
    out = []
    for dp, dn, fn in os.walk(folder):
        dn[:] = [d for d in dn if not intersects(os.path.join(dp, d), [p for p in prefixes if p != folder])]
        for name in fn:
            a = os.path.join(dp, name)
            if intersects(a, [p for p in prefixes if p != folder]) or os.path.islink(a):
                continue
            if os.lstat(a).st_size > corpus["thresholds"]["openTicketFile"]:
                out.append({"kind": "ticket-generated", "path": rel(a), "bytes": os.lstat(a).st_size})
    return sorted(out, key=lambda r: r["path"])


for c in corpus["openOversized"]:
    if open_oversized(os.path.join(root, *c["folder"].split("/")), absl(c["prefixes"])) != sorted(c["expected"], key=lambda r: r["path"]):
        failures.append("open oversized " + c["folder"])


def build(prefixes):
    out = []

    def visit(a, name):
        if name in SKIP or name == TICKETS or under(a, prefixes):
            return "skip"
        if BUILD(name):
            if intersects(a, prefixes):
                return "skip"
            b = path_bytes(a)
            if os.path.exists(os.path.join(a, "CACHEDIR.TAG")) and (name == "target" or name.startswith("target-")) or b > corpus["thresholds"]["buildArtifact"]:
                out.append({"kind": "build-artifact", "path": rel(a), "bytes": b})
            return "skip"
        return "enter"
    walk_dirs(root, visit)
    return sorted(out, key=lambda r: r["path"])


b = corpus["build"]
if build([]) != sorted(b["expected"], key=lambda r: r["path"]) or build(absl(b["protected"])) != sorted(b["expectedProtected"], key=lambda r: r["path"]):
    failures.append("build")
for c in corpus["dedupe"]:
    rows = sorted((r["path"] for r in c["rows"]), key=lambda p: (-len(p), p))
    kept = []
    for p in rows:
        if not any(k == p or k.startswith(p + "/") for k in kept):
            kept.append(p)
    if sorted(kept) != sorted(c["expected"]):
        failures.append("dedupe")
shutil.rmtree(root, ignore_errors=True)
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, oracle)
