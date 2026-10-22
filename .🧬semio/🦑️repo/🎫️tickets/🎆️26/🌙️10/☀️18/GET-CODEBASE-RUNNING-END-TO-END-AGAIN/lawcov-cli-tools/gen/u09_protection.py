import lib
from lib import S
UNIT = "🧼workspace-cleanup/🛡protection"
V = "️"
M = "🎫" + V + "ticket.json"
G = "🗑" + V + "generated"
T = "🗑" + V + "temp"
REPO = "🦑" + V + "repo"
TK = "tickets/26/10/21"
def d(p): return {"path": p, "kind": "directory"}
def f(p, text=""): return {"path": p, "kind": "file", "text": text}
nodes = [d("src"), d("src/sub"), d("src/other"), f("src/other/c.txt", "c"), f("src/a.txt", "a"), f("src/sub/b.txt", "b"), d("tickets"), d("tickets/26"), d("tickets/26/10"), d(TK),
 d(TK + "/OPEN"), f(TK + "/OPEN/" + M, '{"status":"open"}'), f(TK + "/OPEN/notes.md", "n"), d(TK + "/OPEN/" + G), f(TK + "/OPEN/" + G + "/x.txt", "x"),
 d(TK + "/DONE"), f(TK + "/DONE/" + M, '{"status":"closed","title":"t"}'), f(TK + "/DONE/out.md", "o"), d(TK + "/DONE/" + G), f(TK + "/DONE/" + G + "/y.txt", "y"),
 d(TK + "/NOMANIFEST"), f(TK + "/NOMANIFEST/f.md", "f"), d(TK + "/BADJSON"), f(TK + "/BADJSON/" + M, "{"),
 d(TK + "/ARRAY"), f(TK + "/ARRAY/" + M, '["closed"]'), d(TK + "/NOSTATUS"), f(TK + "/NOSTATUS/" + M, "{}"), d(TK + "/DIRMANIFEST"), d(TK + "/DIRMANIFEST/" + M),
 d("src2"), {"path": "src2/ln", "kind": "symlink"}, {"path": "linkdir", "kind": "symlink"}, {"path": "src3", "kind": "directory", "childrenUnreadable": True}, f("src3/inner.txt"),
 d("imgs"), d("imgs/vm"), f("imgs/vm/image.json", "{}"), f("imgs/vm/fixed.dmg", "dmg"), d(".cache"), d(".cache/b"), d(".cache/b/t"), f(".cache/b/t/x", "x")]
protection = [
 ("a plain source directory is removable", "src", None, [], False),
 ("a plain file is removable", "src/a.txt", None, [], False),
 ("the workspace root itself is refused", ".", None, ["."], False),
 ("a path outside the workspace is refused", "../outside", None, ["../outside"], False),
 ("a symlink target is refused", "linkdir", None, ["linkdir"], False),
 ("a missing path is refused", "nothing", None, ["nothing"], False),
 ("a file used as a directory ancestor is refused", "src/a.txt/zzz", None, ["src/a.txt"], False),
 ("an open ticket folder is protected", TK + "/OPEN", None, [TK + "/OPEN"], False),
 ("a file inside an open ticket protects the ticket", TK + "/OPEN/notes.md", None, [TK + "/OPEN"], False),
 ("a closed ticket is removable as a whole", TK + "/DONE", None, [], False),
 ("a file of a closed ticket is removable", TK + "/DONE/out.md", None, [], False),
 ("a day folder reports every non-closed ticket below it", TK, None, [TK + "/ARRAY", TK + "/BADJSON", TK + "/DIRMANIFEST", TK + "/NOMANIFEST", TK + "/NOSTATUS", TK + "/OPEN"], False),
 ("a ticket folder without manifest is protected", TK + "/NOMANIFEST", None, [TK + "/NOMANIFEST"], False),
 ("a ticket manifest that is not valid JSON protects the ticket", TK + "/BADJSON", None, [TK + "/BADJSON"], False),
 ("an array manifest is not a closed ticket", TK + "/ARRAY", None, [TK + "/ARRAY"], False),
 ("a manifest without status is not closed", TK + "/NOSTATUS", None, [TK + "/NOSTATUS"], False),
 ("a manifest that is a directory is not closed", TK + "/DIRMANIFEST", None, [TK + "/DIRMANIFEST"], False),
 ("the allowed open ticket may be cleaned inside", TK + "/OPEN/notes.md", TK + "/OPEN", [], False),
 ("a symlink inside the candidate is refused", "src2", None, ["src2/ln"], False),
 ("an unreadable directory is refused", "src3", None, ["src3"], False),
 ("a managed storage subtree is refused", ".cache/b/t/x", None, [".cache/b/t/x"], True),
 ("an ancestor of a managed storage directory is refused", ".cache", None, [".cache"], True),
 ("a native image directory is refused", "imgs/vm", None, ["imgs/vm"], True),
 ("a native image backing file is refused", "imgs/vm/fixed.dmg", None, ["imgs/vm/fixed.dmg"], True),
 ("the receipt inside a native image directory is refused", "imgs/vm/image.json", None, ["imgs/vm/image.json"], True),
]
protection_defects = [("an ancestor holding a native image directory is refused", "imgs", None, ["imgs"], True, "🛡️protection/🟦️.ts:cleanIsNativeImagePath tests only the path itself and its direct parent, so removing an ancestor of a native image directory (`imgs` above `imgs/vm`) passes the gate although it deletes the image pair")]
generated = [
 ("the generated directory of an open ticket", TK + "/OPEN/" + G, [TK + "/OPEN/" + G]),
 ("a file inside the generated directory of an open ticket", TK + "/OPEN/" + G + "/x.txt", [TK + "/OPEN/" + G + "/x.txt"]),
 ("an ordinary file of an open ticket", TK + "/OPEN/notes.md", []),
 ("the generated directory of a closed ticket", TK + "/DONE/" + G, []),
 ("a day folder containing an open ticket's generated directory", TK, [TK + "/OPEN/" + G]),
 ("a plain source directory", "src", []),
 ("an unreadable directory", "src3", ["src3"]),
 ("a missing path", "nothing", []),
]
boundary = [(TK + "/OPEN", True), (TK, False), ("tickets/26/10", False), (TK + "/OPEN/" + G, False), ("src", False), ("x/mytickets/a/b/c/d", True), ("tickets/a/b/c/d/e", False), (".", False)]
folder_for = [(TK + "/OPEN/notes.md", TK + "/OPEN"), (TK + "/OPEN/" + G + "/x.txt", TK + "/OPEN"), (TK + "/OPEN", TK), (TK + "/OPEN", None), (TK, None), ("src/a.txt", None), ("../out", None)]
folder_for = [(TK + "/OPEN/notes.md", TK + "/OPEN"), (TK + "/OPEN/" + G + "/x.txt", TK + "/OPEN"), (TK + "/DONE/" + G, TK + "/DONE"), (TK, None), ("src/a.txt", None), ("../out", None)]
gen_root = [(TK + "/OPEN/" + G, TK + "/OPEN"), (TK + "/DONE/" + T, TK + "/DONE"), (TK + "/OPEN/" + "🧪" + V + "purity-1", TK + "/OPEN"), (TK + "/OPEN/notes.md", None), ("src/" + G, None)]
closed = [(TK + "/OPEN", False), (TK + "/DONE", True), (TK + "/NOMANIFEST", False), (TK + "/BADJSON", False), (TK + "/ARRAY", False), (TK + "/NOSTATUS", False), (TK + "/DIRMANIFEST", False), ("src", False)]
rows = [
 {"kind": "misplaced", "path": "src", "bytes": 1}, {"kind": "ticket-generated", "path": TK + "/OPEN/" + G, "bytes": 1}, {"kind": "ticket-generated", "path": TK + "/DONE/" + G, "bytes": 1},
 {"kind": "misplaced", "path": TK + "/OPEN", "bytes": 1}, {"kind": "build-artifact", "path": "src/sub", "bytes": 1}, {"kind": "cargo-incremental", "path": "src2/incremental", "bytes": 1}, {"kind": "build-artifact", "path": "src2", "bytes": 1}, {"kind": "misplaced", "path": "src/other", "bytes": 1},
]
projection = {"rows": rows, "protectedPrefixes": ["src/sub"], "expectedKept": [TK + "/DONE/" + G, "src/other"], "expectedProtected": [TK + "/OPEN/" + G, "src", "src/sub", "src2/incremental", "src2/ln"]}
dedupe = [
 {"rows": [{"kind": "misplaced", "path": p, "bytes": 1} for p in ["a/b/c", "a/b", "a", "a2", "a/bc", "a/b"]], "expected": ["a", "a2"]},
 {"rows": [{"kind": "misplaced", "path": p, "bytes": 1} for p in ["x/y", "x/yz", "x/y/z"]], "expected": ["x/y", "x/yz"]},
 {"rows": [], "expected": []},
]
huge = [
 {"name": "under the limit nothing is removed", "dir": {"path": "c/debug/incremental", "crates": [{"name": "a-1", "bytes": 10, "working": False}]}, "max": 100, "expected": []},
 {"name": "at the limit nothing is removed", "dir": {"path": "c/debug/incremental", "crates": [{"name": "a-1", "bytes": 60, "working": False}, {"name": "b-2", "bytes": 40, "working": False}]}, "max": 100, "expected": []},
 {"name": "over the limit with only idle crates removes the whole directory", "dir": {"path": "c/debug/incremental", "crates": [{"name": "a-1", "bytes": 60, "working": False}, {"name": "b-2", "bytes": 41, "working": False}]}, "max": 100, "expected": ["c/debug/incremental"]},
 {"name": "a working crate stays while idle crates go, sorted", "dir": {"path": "c/debug/incremental", "crates": [{"name": "z-1", "bytes": 60, "working": False}, {"name": "w-2", "bytes": 41, "working": True}, {"name": "b-3", "bytes": 1, "working": False}]}, "max": 100, "expected": ["c/debug/incremental/b-3", "c/debug/incremental/z-1"]},
 {"name": "only working crates leave everything", "dir": {"path": "c/debug/incremental", "crates": [{"name": "a-1", "bytes": 500, "working": True}]}, "max": 100, "expected": []},
 {"name": "backslashes and trailing slashes normalize", "dir": {"path": "c\\debug\\incremental\\\\", "crates": [{"name": "a-1", "bytes": 500, "working": False}]}, "max": 100, "expected": ["c/debug/incremental"]},
 {"name": "an empty directory is never over the limit", "dir": {"path": "c/incremental", "crates": []}, "max": 0, "expected": []},
]
huge_errors = [
 {"name": "a negative limit", "dir": {"path": "c/incremental", "crates": []}, "max": -1, "message": "non-negative safe integer"},
 {"name": "a fractional limit", "dir": {"path": "c/incremental", "crates": []}, "max": 1.5, "message": "non-negative safe integer"},
 {"name": "a parent segment in the path", "dir": {"path": "c/../incremental", "crates": []}, "max": 1, "message": "inside the workspace"},
 {"name": "an empty path", "dir": {"path": "", "crates": []}, "max": 1, "message": "inside the workspace"},
 {"name": "a crate name with a slash", "dir": {"path": "c/incremental", "crates": [{"name": "a/b", "bytes": 1, "working": False}]}, "max": 1, "message": "one path segment"},
 {"name": "a crate name that is dot-dot", "dir": {"path": "c/incremental", "crates": [{"name": "..", "bytes": 1, "working": False}]}, "max": 1, "message": "one path segment"},
 {"name": "a negative crate size", "dir": {"path": "c/incremental", "crates": [{"name": "a", "bytes": -1, "working": False}]}, "max": 1, "message": "non-negative safe integer"},
 {"name": "a sum beyond the safe integer range", "dir": {"path": "c/incremental", "crates": [{"name": "a", "bytes": 9007199254740991, "working": False}, {"name": "b", "bytes": 9007199254740991, "working": False}]}, "max": 1, "message": "non-negative safe integer"},
]
inc_paths = [("c/debug/incremental", ["c"], True), ("c/debug/incremental/crate-abc", ["c"], True), ("c/debug/incremental/crate/x", ["c"], False), ("c/debug", ["c"], False), ("d/incremental", ["c"], False), ("c/incremental/incremental", ["c"], True), ("../c/incremental", ["c"], False), ("c/debug/incremental", [], False), ("c/incremental", ["c/incremental"], True), ("c/x/../debug/incremental", ["c"], True), ("cc/incremental", ["c"], False)]
names = {
 "misplacedRepo": [("repo", True), ("🦑repo", True), (REPO, False), ("repository", False), ("my-repo", True), ("", False)],
 "misplacedTickets": [("tickets", True), ("🎫tickets", True), ("🎫" + V + "tickets", False), ("ticketsx", False)],
 "buildArtifact": [("target", True), ("dist", True), ("build", True), ("out", True), ("target-x86", True), ("targets", False), ("Target", False), ("distro", False), ("target-", True)],
 "cargoTargetName": [("target", True), ("target-wasm", True), ("targetx", False), ("build", False)],
 "semioRoot": [(".🧬semio", True), (".🧬" + V + "semio", True), (".semio", False), (".🧬semio2", False), (".🧬x", False), ("semio", False)],
 "rootTransient": [(".generation3d-foo-link", True), (".generation3d--link", False), (".w-g3-ticket", True), ("generation3d-foo-link", False), (".generation3d-foo-link2", False)],
 "windowsIllegal": [("", True), (".", False), ("..", False), ("a ", True), ("a.", True), (" a", True), ("a<b", True), ("a:b", True), ('a"b', True), ("a|b", True), ("a?b", True), ("a*b", True), ("a\u0001b", True), ("CON", True), ("con.txt", True), ("Com1", True), ("com¹", True), ("LPT9.log", True), ("nul", True), ("aux", True), ("COM0", False), ("console", False), ("normal.txt", False), ("   ", True), ("a>b", True), ("a\u001fb", True), ("🧪" + V + "tests", False), ("prn.tar.gz", True)],
 "generatedDir": [(G, True), (T, True), ("🧾" + V + "runs", True), ("🧪" + V + "runs", True), ("🧾" + V + "taxonomy-transaction", True), ("🧪" + V + "purity-a", True), ("🧪" + V + "cli-b", True), ("🧪" + V + "inventory-1", True), ("runs", False), ("generated", False), ("🧪" + V + "other", False)],
 "endsWithAscii": [("a-repo", "repo", True), ("repo", "repo", True), ("repos", "repo", False), ("", "x", False)],
}
corpus = {"version": 1, "constants": {"repoDir": REPO, "ticketsDir": "🎫" + V + "tickets", "buildNames": ["build", "dist", "out", "target"], "cacheDir": "⚡" + V + "cache", "generated": [G, T, "🧾" + V + "runs", "🧪" + V + "runs", "🧾" + V + "taxonomy-transaction"], "probePrefixes": ["🧪" + V + "purity-", "🧪" + V + "cli-", "🧪" + V + "inventory-"], "incrementalMaxBytes": 1073741824, "nativeBacking": ["fixed.dir", "fixed.dmg"], "manifest": M},
 "nodes": nodes, "protection": [{"name": n, "candidate": c, "allowed": a, "expected": e, "environment": env} for n, c, a, e, env in protection],
 "protectionDefects": [{"name": n, "candidate": c, "allowed": a, "expected": e, "environment": env, "defect": df} for n, c, a, e, env, df in protection_defects],
 "generated": [{"name": n, "candidate": c, "expected": e} for n, c, e in generated], "boundary": [{"path": p, "expected": e} for p, e in boundary], "folderFor": [{"path": p, "expected": e} for p, e in folder_for],
 "generatedRoot": [{"path": p, "expected": e} for p, e in gen_root], "closed": [{"path": p, "expected": e} for p, e in closed], "projection": projection, "dedupe": dedupe, "huge": huge, "hugeErrors": huge_errors,
 "incrementalPaths": [{"path": p, "roots": r, "expected": e} for p, r, e in inc_paths],
 "names": {k: [dict(zip(("name", "expected") if len(v[0]) == 2 else ("name", "suffix", "expected"), row)) for row in v] for k, v in names.items()}}
node = S.obj({"path": S.str, "kind": S.enum("directory", "file", "symlink"), "text": S.str, "childrenUnreadable": S.bool}, required=["path", "kind"])
pc = S.obj({"name": S.str, "candidate": S.str, "allowed": S.one(S.str, S.null), "expected": S.strs(), "environment": S.bool})
nm = lambda extra=None: S.arr(S.obj({"name": S.str, "expected": S.bool}), 1)
schema = S.obj({"version": S.const(1), "constants": S.obj({"repoDir": S.str, "ticketsDir": S.str, "buildNames": S.strs(), "cacheDir": S.str, "generated": S.strs(), "probePrefixes": S.strs(), "incrementalMaxBytes": S.int, "nativeBacking": S.strs(), "manifest": S.str}),
 "nodes": S.arr(node, 1), "protection": S.arr(pc, 1), "protectionDefects": S.arr(dict(pc, properties=dict(pc["properties"], defect=S.str), required=pc["required"] + ["defect"]), 1),
 "generated": S.arr(S.obj({"name": S.str, "candidate": S.str, "expected": S.strs()}), 1), "boundary": S.arr(S.obj({"path": S.str, "expected": S.bool}), 1),
 "folderFor": S.arr(S.obj({"path": S.str, "expected": S.one(S.str, S.null)}), 1), "generatedRoot": S.arr(S.obj({"path": S.str, "expected": S.one(S.str, S.null)}), 1), "closed": S.arr(S.obj({"path": S.str, "expected": S.bool}), 1),
 "projection": S.obj({"rows": S.arr(S.obj({"kind": S.str, "path": S.str, "bytes": S.int})), "protectedPrefixes": S.strs(), "expectedKept": S.strs(), "expectedProtected": S.strs()}),
 "dedupe": S.arr(S.obj({"rows": S.arr(S.obj({"kind": S.str, "path": S.str, "bytes": S.int})), "expected": S.strs()}), 1),
 "huge": S.arr(S.obj({"name": S.str, "dir": S.obj({"path": S.str, "crates": S.arr(S.obj({"name": S.str, "bytes": S.int, "working": S.bool}))}), "max": S.num, "expected": S.strs()}), 1),
 "hugeErrors": S.arr(S.obj({"name": S.str, "dir": S.obj({"path": S.str, "crates": S.arr(S.obj({"name": S.str, "bytes": S.int, "working": S.bool}))}), "max": S.num, "message": S.str}), 1),
 "incrementalPaths": S.arr(S.obj({"path": S.str, "roots": S.strs(), "expected": S.bool}), 1),
 "names": S.obj({k: (S.arr(S.obj({"name": S.str, "suffix": S.str, "expected": S.bool}), 1) if k == "endsWithAscii" else nm()) for k in names})})
bodyts = '''type Node={path:string;kind:"directory"|"file"|"symlink";text?:string;childrenUnreadable?:boolean};
const root=realpathSync(mkdtempSync(join(tmpdir(),"protection-")));
const absolute=(path:string):string=>path==="."?root:resolve(root,...path.split("/"));
const relativeOf=(path:string):string=>{const value=relative(root,path).split(sep).join("/");return value===""?".":value;};
const byPath=new Map<string,Node>(corpus.nodes.map(node=>[absolute(node.path),node as Node]));
for(const node of corpus.nodes){const target=absolute(node.path);if(node.kind==="directory")mkdirSync(target,{recursive:true});else if(node.kind==="file"){mkdirSync(dirname(target),{recursive:true});writeFileSync(target,node.text??"");}}
const view:CleanProtectionView={
 kind:path=>{const found=byPath.get(resolve(path));return path===root?"directory":found===undefined?"missing":found.kind;},
 children:path=>{const found=byPath.get(resolve(path));if(path!==root&&(found===undefined||found.kind!=="directory"||found.childrenUnreadable===true))return undefined;return[...byPath.keys()].filter(other=>dirname(other)===resolve(path)).map(other=>other.slice(resolve(path).length+1));},
 read:path=>byPath.get(resolve(path))?.text,
};
const sorted=(paths:string[]):string[]=>[...paths].sort();
const protectedRelative=(paths:string[]):string[]=>sorted(paths.map(relativeOf));
afterAll(()=>rmSync(root,{recursive:true,force:true}));
test("the constants match the corpus",()=>{
 expect(CLEAN_CANONICAL_REPO_DIR).toBe(corpus.constants.repoDir);expect(CLEAN_CANONICAL_TICKETS_DIR).toBe(corpus.constants.ticketsDir);expect([...CLEAN_BUILD_DIR_NAMES].sort()).toEqual(corpus.constants.buildNames);expect(CLEAN_CACHE_DIR_NAME).toBe(corpus.constants.cacheDir);
 expect([...CLEAN_TICKET_GENERATED_OUTPUT_DIRS].sort()).toEqual([...corpus.constants.generated].sort());expect([...CLEAN_TICKET_GENERATED_PROBE_PREFIXES]).toEqual(corpus.constants.probePrefixes);expect(CLEAN_RUST_INCREMENTAL_MAX_BYTES).toBe(corpus.constants.incrementalMaxBytes);expect([...CLEAN_NATIVE_IMAGE_BACKING_NAMES].sort()).toEqual(corpus.constants.nativeBacking);
});
const predicates:[string,(name:string)=>boolean][]=[["misplacedRepo",cleanIsMisplacedRepoDir],["misplacedTickets",cleanIsMisplacedTicketsDir],["buildArtifact",cleanIsBuildArtifactDirName],["cargoTargetName",cleanIsCargoTargetDirName],["semioRoot",cleanIsSemioRootName],["rootTransient",cleanIsRootTransientName],["windowsIllegal",cleanIsWindowsIllegalName],["generatedDir",cleanIsTicketGeneratedOutputDir]];
for(const[key,predicate]of predicates)for(const row of (corpus.names as Record<string,{name:string;expected:boolean}[]>)[key]!)test(key+": "+JSON.stringify(row.name),()=>{expect(predicate(row.name)).toBe(row.expected);});
for(const row of corpus.names.endsWithAscii)test("ends with ascii "+JSON.stringify(row.name),()=>{expect(cleanEndsWithAscii(row.name,row.suffix)).toBe(row.expected);});
test("canonical directory predicates are exact",()=>{expect(cleanIsCanonicalRepoDir(corpus.constants.repoDir)).toBe(true);expect(cleanIsCanonicalRepoDir("repo")).toBe(false);expect(cleanIsCanonicalTicketsDir(corpus.constants.ticketsDir)).toBe(true);expect(cleanIsCanonicalTicketsDir("tickets")).toBe(false);});
for(const row of corpus.huge)test("incremental selection: "+row.name,()=>{expect(cleanSelectHugeRustIncremental(row.dir,row.max)).toEqual(row.expected);});
for(const row of corpus.hugeErrors)test("incremental selection fails: "+row.name,()=>{expect(()=>cleanSelectHugeRustIncremental(row.dir,row.max)).toThrow(row.message);});
for(const row of corpus.incrementalPaths)test("incremental removal path "+row.path+" under "+JSON.stringify(row.roots),()=>{expect(cleanIsRustIncrementalRemovalPath(root,absolute(row.path),row.roots.map(absolute))).toBe(row.expected);});
for(const row of corpus.dedupe)test("shallowest dedupe of "+row.rows.length+" rows",()=>{expect(cleanDedupePreferShallowest(row.rows as CleanRemoval[]).map(item=>item.path).sort()).toEqual([...row.expected].sort());});
for(const row of corpus.boundary)test("ticket boundary "+row.path,()=>{expect(cleanIsTicketFolderBoundary(root,absolute(row.path))).toBe(row.expected);});
for(const row of corpus.folderFor)test("ticket folder for "+row.path,()=>{const found=cleanTicketFolderForPath(root,absolute(row.path));expect(found===undefined?null:relativeOf(found)).toBe(row.expected);});
for(const row of corpus.generatedRoot)test("generated ticket root of "+row.path,()=>{const found=cleanTicketGeneratedOutputTicketRoot(root,absolute(row.path));expect(found===undefined?null:relativeOf(found)).toBe(row.expected);});
for(const row of corpus.closed)test("manifest closed at "+row.path,()=>{expect(cleanTicketManifestIsClosed(absolute(row.path),view)).toBe(row.expected);});
for(const row of corpus.protection)test("removal protection: "+row.name,()=>{expect(protectedRelative(cleanRemovalProtection(root,row.candidate==="."||row.candidate.startsWith("..")?resolve(root,row.candidate):row.candidate,view,row.allowed===null?undefined:absolute(row.allowed)))).toEqual(sorted(row.expected));});
for(const row of corpus.protectionDefects)test.failing("removal protection defect: "+row.name,()=>{expect(protectedRelative(cleanRemovalProtection(root,row.candidate,view))).toEqual(sorted(row.expected));});
for(const row of corpus.generated)test("open ticket generated output protection: "+row.name,()=>{expect(protectedRelative(cleanOpenTicketGeneratedOutputProtection(root,row.candidate,view))).toEqual(sorted(row.expected));});
test("projection drops protected rows, keeps eligible siblings and reports every protected path",()=>{
 const seen:string[]=[],kept=cleanProjectRemovals(root,corpus.projection.rows as CleanRemoval[],corpus.projection.protectedPrefixes.map(absolute),view,path=>seen.push(path));
 expect(sorted(kept.map(row=>row.path))).toEqual(sorted(corpus.projection.expectedKept));expect(protectedRelative([...new Set(seen)])).toEqual(sorted(corpus.projection.expectedProtected));
});
test("protected prefixes cover and intersect their subtrees",()=>{
 const prefixes=[absolute("a/b")];
 expect(cleanIsProtected(absolute("a/b"),prefixes)).toBe(true);expect(cleanIsProtected(absolute("a/b/c"),prefixes)).toBe(true);expect(cleanIsProtected(absolute("a/bc"),prefixes)).toBe(false);expect(cleanIsProtected(absolute("a"),prefixes)).toBe(false);
 expect(cleanIntersectsProtected(absolute("a"),prefixes)).toBe(true);expect(cleanIntersectsProtected(absolute("a/b/c"),prefixes)).toBe(true);expect(cleanIntersectsProtected(absolute("a/bc"),prefixes)).toBe(false);expect(cleanIntersectsProtected(absolute("z"),prefixes)).toBe(false);
});
test("the default filesystem view sees directories, files, symlinks and missing paths",()=>{
 const base=join(root,"real");mkdirSync(join(base,"dir"),{recursive:true});writeFileSync(join(base,"file.txt"),"é");
 expect(CLEAN_PROTECTION_VIEW.kind(join(base,"dir"))).toBe("directory");expect(CLEAN_PROTECTION_VIEW.kind(join(base,"file.txt"))).toBe("file");expect(CLEAN_PROTECTION_VIEW.kind(join(base,"nothing"))).toBe("missing");
 expect(CLEAN_PROTECTION_VIEW.children(join(base,"dir"))).toEqual([]);expect(CLEAN_PROTECTION_VIEW.children(join(base,"nothing"))).toBeUndefined();expect(CLEAN_PROTECTION_VIEW.read(join(base,"file.txt"))).toBe("é");expect(CLEAN_PROTECTION_VIEW.read(join(base,"nothing"))).toBeUndefined();
 writeFileSync(join(base,"binary.bin"),Buffer.from([0xff,0xfe,0x00]));expect(CLEAN_PROTECTION_VIEW.read(join(base,"binary.bin"))).toBeUndefined();
 if(process.platform!=="win32"){symlinkSync(join(base,"dir"),join(base,"link"));expect(CLEAN_PROTECTION_VIEW.kind(join(base,"link"))).toBe("symlink");expect(cleanRemovalProtection(root,"real/link")).toEqual([join(base,"link")]);}
 expect(cleanRemovalProtection(root,"real/dir")).toEqual([]);
});
test("a Cargo target directory is recognised by its CACHEDIR.TAG",()=>{
 const base=join(root,"cargo");mkdirSync(join(base,"target"),{recursive:true});mkdirSync(join(base,"target-x"),{recursive:true});mkdirSync(join(base,"dist"),{recursive:true});writeFileSync(join(base,"target","CACHEDIR.TAG"),"Signature: 8a477f597d28d172789f06886806bc55");writeFileSync(join(base,"dist","CACHEDIR.TAG"),"x");
 expect(cleanIsCargoTargetDir(join(base,"target"),"target")).toBe(true);expect(cleanIsCargoTargetDir(join(base,"target-x"),"target-x")).toBe(false);expect(cleanIsCargoTargetDir(join(base,"dist"),"dist")).toBe(false);
});
test("a rustc working session is found below an incremental directory or crate",()=>{
 const base=join(root,"inc","debug","incremental");mkdirSync(join(base,"idle-1","s-aaa"),{recursive:true});mkdirSync(join(base,"busy-2","s-bbb-working"),{recursive:true});mkdirSync(join(base,"file-3"),{recursive:true});writeFileSync(join(base,"file-3","s-ccc-working"),"not a directory");
 expect(cleanRustIncrementalHasWorkingSession(join(base,"idle-1"))).toBe(false);expect(cleanRustIncrementalHasWorkingSession(join(base,"busy-2"))).toBe(true);expect(cleanRustIncrementalHasWorkingSession(join(base,"file-3"))).toBe(false);expect(cleanRustIncrementalHasWorkingSession(base)).toBe(true);
 expect(cleanRustIncrementalHasWorkingSession(join(root,"inc","nothing"))).toBe(true);
});
test("native image paths are the backing files, their receipt directory and its children",()=>{
 expect(cleanIsNativeImagePath(absolute("imgs/vm"))).toBe(true);expect(cleanIsNativeImagePath(absolute("imgs/vm/fixed.dmg"))).toBe(true);expect(cleanIsNativeImagePath(absolute("imgs/vm/image.json"))).toBe(true);expect(cleanIsNativeImagePath(absolute("src"))).toBe(false);expect(cleanIsNativeImagePath(absolute("nothing/fixed.dir"))).toBe(true);
});'''
ts = lib.ts_wrap(UNIT, "🛡️ Contract: the cleanup deletion gate refuses managed storage, native images, open or unreadable tickets and anything that escapes or symlinks out of the workspace, keeps closed tickets and eligible siblings removable and classifies directory names; the corpus is language-agnostic and `@@O@@` re-derives the ticket, name and incremental-cache verdicts.",
 'import {afterAll} from "bun:test";\nimport {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,relative,resolve,sep} from "node:path";\nimport {CLEAN_BUILD_DIR_NAMES,CLEAN_CACHE_DIR_NAME,CLEAN_CANONICAL_REPO_DIR,CLEAN_CANONICAL_TICKETS_DIR,CLEAN_NATIVE_IMAGE_BACKING_NAMES,CLEAN_PROTECTION_VIEW,CLEAN_RUST_INCREMENTAL_MAX_BYTES,CLEAN_TICKET_GENERATED_OUTPUT_DIRS,CLEAN_TICKET_GENERATED_PROBE_PREFIXES,cleanDedupePreferShallowest,cleanEndsWithAscii,cleanIntersectsProtected,cleanIsBuildArtifactDirName,cleanIsCanonicalRepoDir,cleanIsCanonicalTicketsDir,cleanIsCargoTargetDir,cleanIsCargoTargetDirName,cleanIsMisplacedRepoDir,cleanIsMisplacedTicketsDir,cleanIsNativeImagePath,cleanIsProtected,cleanIsRootTransientName,cleanIsRustIncrementalRemovalPath,cleanIsSemioRootName,cleanIsTicketFolderBoundary,cleanIsTicketGeneratedOutputDir,cleanIsWindowsIllegalName,cleanOpenTicketGeneratedOutputProtection,cleanProjectRemovals,cleanRemovalProtection,cleanRustIncrementalHasWorkingSession,cleanSelectHugeRustIncremental,cleanTicketFolderForPath,cleanTicketGeneratedOutputTicketRoot,cleanTicketManifestIsClosed,type CleanProtectionView,type CleanRemoval} from "@@' + UNIT + '/🟦.ts@@";', bodyts)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent model of the cleanup protection gate: names, ticket boundaries, manifests, removal protection, open-ticket generated output, incremental-cache selection and deduplication over the corpus node tree (stdlib only; storage and native-image rows depend on the checkout and are skipped).") + r'''import posixpath

NODES = {n["path"]: n for n in corpus["nodes"]}
C = corpus["constants"]
GEN_DIRS = set(C["generated"])
PROBES = tuple(C["probePrefixes"])


def kind(p):
    return "directory" if p in ("", ".") else NODES[p]["kind"] if p in NODES else "missing"


def children(p):
    p = "" if p == "." else p
    n = NODES.get(p)
    if p != "" and (n is None or n["kind"] != "directory" or n.get("childrenUnreadable")):
        return None
    return sorted(q[len(p) + 1:] if p else q for q in NODES if posixpath.dirname(q) == p and q != "")


def read(p):
    return NODES.get(p, {}).get("text")


def is_gen(name):
    return name in GEN_DIRS or name.startswith(PROBES)


def boundary(p):
    segs = [s for s in ("" if p == "." else p).split("/") if s]
    return any(s.endswith("tickets") and len(segs) - i - 1 == 4 for i, s in enumerate(segs))


def closed(p):
    m = (p + "/" if p else "") + C["manifest"]
    if kind(m) != "file":
        return False
    try:
        doc = json.loads(read(m))
    except (ValueError, TypeError):
        return False
    return isinstance(doc, dict) and doc.get("status") == "closed"


def ticket_for(p):
    a = posixpath.dirname(p)
    while True:
        if a == "" or a.startswith(".."):
            return None
        if boundary(a):
            return a
        parent = posixpath.dirname(a)
        if parent == a:
            return None
        a = parent


def protection(candidate, allowed=None):
    target = posixpath.normpath(candidate)
    if target == "." or target.startswith(".."):
        return [target]
    prot, closed_set = set(), set()
    chain = [("", "directory")]

    def inspect(d):
        if not boundary(d) and kind((d + "/" if d else "") + C["manifest"]) == "missing":
            return
        if closed(d):
            closed_set.add(d)
        elif d != allowed:
            prot.add(d)
    inspect("")
    anc = ""
    for seg in target.split("/"):
        anc = anc + "/" + seg if anc else seg
        k = kind(anc)
        if k in ("symlink", "missing", "unreadable") or (anc != target and k != "directory"):
            return [anc]
        chain.append((anc, k))
        if k == "directory":
            inspect(anc)
    if prot:
        return sorted(prot)
    stack = [target] if kind(target) == "directory" else []
    while stack:
        d = stack.pop()
        inspect(d)
        if d in prot:
            continue
        names = children(d)
        if names is None:
            prot.add(d)
            continue
        for name in names:
            child = d + "/" + name
            k = kind(child)
            if k == "directory":
                stack.append(child)
            elif k != "file":
                prot.add(child)
    for p, k in chain:
        if kind(p) != k:
            prot.add(p)
    for d in closed_set:
        if kind(d) != "directory" or not closed(d):
            prot.add(d)
    return sorted(prot)


def generated(candidate):
    target = posixpath.normpath(candidate)
    ticket = ticket_for(target)
    if ticket is not None and not closed(ticket):
        inside = posixpath.relpath(target, ticket).split("/")
        if any(is_gen(s) for s in inside):
            return [target]
    if kind(target) != "directory":
        return []
    stack = [target]
    while stack:
        d = stack.pop()
        names = children(d)
        if names is None:
            return [d]
        for name in names:
            child = d + "/" + name
            if kind(child) != "directory":
                continue
            if is_gen(name):
                owner = ticket_for(child)
                if owner is not None and not closed(owner):
                    return [child]
                continue
            stack.append(child)
    return []


for c in corpus["protection"]:
    if not c["environment"] and protection(c["candidate"], c["allowed"]) != sorted(c["expected"]):
        failures.append("protection: " + c["name"])
for c in corpus["generated"]:
    if generated(c["candidate"]) != sorted(c["expected"]):
        failures.append("generated: " + c["name"])
for c in corpus["boundary"]:
    if boundary(c["path"]) != c["expected"]:
        failures.append("boundary " + c["path"])
for c in corpus["folderFor"]:
    if ticket_for(posixpath.normpath(c["path"])) != c["expected"]:
        failures.append("folderFor " + c["path"])
for c in corpus["generatedRoot"]:
    p = posixpath.normpath(c["path"])
    got = ticket_for(p) if is_gen(posixpath.basename(p)) else None
    if got != c["expected"]:
        failures.append("generatedRoot " + c["path"])
for c in corpus["closed"]:
    if closed(c["path"]) != c["expected"]:
        failures.append("closed " + c["path"])
for c in corpus["dedupe"]:
    rows = sorted((r["path"] for r in c["rows"]), key=lambda p: (len(p), p))
    kept = []
    for p in rows:
        if not any(p == k or p.startswith(k + "/") for k in kept):
            kept.append(p)
    if sorted(kept) != sorted(c["expected"]):
        failures.append("dedupe")


def huge(d, limit):
    if limit < 0 or limit != int(limit):
        raise ValueError("limit")
    base = d["path"].replace("\\", "/").rstrip("/")
    if not base or any(s in ("", ".", "..") for s in base.split("/")):
        raise ValueError("path")
    for cr in d["crates"]:
        if not cr["name"] or "/" in cr["name"] or "\\" in cr["name"] or cr["name"] in (".", "..") or cr["bytes"] < 0:
            raise ValueError("crate")
    total = sum(cr["bytes"] for cr in d["crates"])
    if total > 9007199254740991:
        raise ValueError("sum")
    if total <= limit:
        return []
    idle = [cr for cr in d["crates"] if not cr["working"]]
    if not idle:
        return []
    if len(idle) == len(d["crates"]):
        return [base]
    return sorted(base + "/" + cr["name"] for cr in idle)


for c in corpus["huge"]:
    if huge(c["dir"], c["max"]) != c["expected"]:
        failures.append("huge: " + c["name"])
for c in corpus["hugeErrors"]:
    try:
        huge(c["dir"], c["max"])
        failures.append("huge error: " + c["name"])
    except ValueError:
        pass
for c in corpus["incrementalPaths"]:
    p = posixpath.normpath(c["path"])
    parts = p.split("/")
    ok = False
    if not (p == "." or p.startswith("..")) and "incremental" in parts:
        at = len(parts) - 1 - parts[::-1].index("incremental")
        ok = len(parts) in (at + 1, at + 2) and any(p == posixpath.normpath(r) or p.startswith(posixpath.normpath(r) + "/") for r in c["roots"])
    if ok != c["expected"]:
        failures.append("incremental path " + c["path"])
N = corpus["names"]
RESERVED = re.compile(r"^(con|prn|aux|nul|com[1-9¹²³]|lpt[1-9¹²³])(\..*)?$", re.I)


def illegal(n):
    if not n:
        return True
    if n in (".", ".."):
        return False
    return n.strip() == "" or n.endswith(" ") or n.endswith(".") or n.startswith(" ") or re.search(r'[<>:"|?*\x00-\x1f]', n) is not None or RESERVED.match(n) is not None


predicates = {
    "misplacedRepo": lambda n: n.endswith("repo") and n != C["repoDir"],
    "misplacedTickets": lambda n: n.endswith("tickets") and n != C["ticketsDir"],
    "buildArtifact": lambda n: n in C["buildNames"] or n == "target" or n.startswith("target-"),
    "cargoTargetName": lambda n: n == "target" or n.startswith("target-"),
    "semioRoot": lambda n: n == ".🧬semio" or (n.startswith(".🧬") and n.endswith("semio")),
    "rootTransient": lambda n: re.match(r"^\.generation3d-.+-link$", n) is not None or n == ".w-g3-ticket",
    "windowsIllegal": illegal,
    "generatedDir": is_gen,
}
for key, fn in predicates.items():
    for row in N[key]:
        if fn(norm(row["name"]) if key == "semioRoot" and False else row["name"]) != row["expected"]:
            failures.append("%s %r" % (key, row["name"]))
for row in N["endsWithAscii"]:
    if row["name"].endswith(row["suffix"]) != row["expected"]:
        failures.append("endsWith %r" % row["name"])
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, oracle)
