import lib
from lib import S
V = "️"
EM = "📌" + V + ".empty.md"
AB = "🚫" + V + ".absent"
IMP = "📝" + V + ".md"
DIFF = "🔺" + V + "diff"
def d(p): return {"path": p, "kind": "directory"}
def f(p, text=""): return {"path": p, "kind": "file", "text": text}
nodes = [f("a/" + EM), f("b/" + EM), f("b/real.txt", "x"), f("c/inner/" + EM), f("c/inner/deep/" + AB), f("d/" + DIFF + "/" + EM), f(DIFF + "/" + EM), d("e"), d("e2/e3"), f("f/" + EM, "1"), f("g/" + EM), f("node_modules/h/" + EM), f("target/i/" + EM), f("j/" + IMP), f("k/other.md"), f("l/" + EM), d("l/empty-sub"), f("m/" + EM), f("m/n/" + EM), f("m/n/o/not-marker.txt"), d("p/q/r")]
def sorted_rows(paths):
    return sorted(paths, key=lambda p: (-len(p), p))
marker = {"expectedNames": sorted([EM, AB, IMP]), "files": [{"path": "a/" + EM, "expected": True}, {"path": "f/" + EM, "expected": False}, {"path": "k/other.md", "expected": False}, {"path": "nothing/" + EM, "expected": False}, {"path": "j/" + IMP, "expected": True}, {"path": "c/inner/deep/" + AB, "expected": True}],
          "trees": [{"path": "a", "expected": True}, {"path": "b", "expected": False}, {"path": "c", "expected": True}, {"path": "c/inner", "expected": True}, {"path": "d", "expected": False}, {"path": DIFF, "expected": False}, {"path": "e", "expected": False}, {"path": "f", "expected": False}, {"path": "j", "expected": True}, {"path": "l", "expected": True}, {"path": "m", "expected": False}, {"path": "m/n", "expected": False}],
          "collect": [{"prefixes": [], "expected": ["a", "c", "g", "j", "l"]}, {"prefixes": ["g"], "expected": ["a", "c", "j", "l"]}, {"prefixes": ["a", "c/inner"], "expected": ["c", "g", "j", "l"]}]}
empty = {"dirs": [{"path": "e", "expected": True}, {"path": "e2", "expected": False}, {"path": "e2/e3", "expected": True}, {"path": "a", "expected": False}, {"path": "nothing", "expected": False}, {"path": "a/" + EM, "expected": False}],
         "collect": [{"prefixes": [], "expected": ["e", "e2/e3", "l/empty-sub", "p/q/r"]}, {"prefixes": ["e2"], "expected": ["e", "l/empty-sub", "p/q/r"]}, {"prefixes": ["p/q"], "expected": ["e", "e2/e3", "l/empty-sub"]}]}
marker_rows = lambda items: items
for c in marker["collect"]:
    c["expected"] = sorted_rows(c["expected"])
for c in empty["collect"]:
    c["expected"] = sorted_rows(c["expected"])
nd = S.obj({"path": S.str, "kind": S.enum("directory", "file"), "text": S.str}, required=["path", "kind"])
chk = S.arr(S.obj({"path": S.str, "expected": S.bool}), 1)
col = S.arr(S.obj({"prefixes": S.strs(), "expected": S.strs()}), 1)
TS_COMMON = '''const posix=(path:string):string=>path.split(sep).join("/");
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const world=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"folders-")));roots.push(root);for(const node of corpus.nodes){const target=join(root,...node.path.split("/"));if(node.kind==="directory")mkdirSync(target,{recursive:true});else{mkdirSync(dirname(target),{recursive:true});writeFileSync(target,(node as {text?:string}).text??"");}}return root;};
const root=world();
const absolute=(path:string):string=>join(root,...path.split("/"));
'''
# marker-only
UNIT1 = "🧼workspace-cleanup/🔍marker-only-folders"
c1 = {"version": 1, "nodes": nodes, "marker": marker}
s1 = S.obj({"version": S.const(1), "nodes": S.arr(nd, 1), "marker": S.obj({"expectedNames": S.strs(), "files": chk, "trees": chk, "collect": col})})
b1 = TS_COMMON + '''const taxonomyPath=fileURLToPath(new URL("@@🔣taxonomy.json@@",import.meta.url));
const names=cleanEmptyMarkerFilenames();
test("marker names are the taxonomy marker kinds plus the ticket important leaf",()=>{
 const taxonomy=JSON.parse(readFileSync(taxonomyPath,"utf8")) as {fileKinds:Record<string,{emoji:string;extensionChains:string[];role:string}>};
 const derived=Object.values(taxonomy.fileKinds).filter(kind=>kind.role==="marker").map(kind=>kind.emoji+kind.extensionChains[0]!);
 expect([...names].sort()).toEqual([...derived,corpus.marker.expectedNames.find(name=>name.endsWith("\\ud83d\\udcdd\\ufe0f.md"))!].sort());expect([...names].sort()).toEqual([...corpus.marker.expectedNames].sort());
});
for(const row of corpus.marker.files)test("empty marker file "+row.path,()=>{expect(cleanIsEmptyMarkerFile(absolute(row.path),names)).toBe(row.expected);});
for(const row of corpus.marker.trees)test("only empty markers below "+row.path,()=>{expect(cleanDirectoryTreeIsOnlyEmptyMarkers(absolute(row.path),names)).toBe(row.expected);});
for(const row of corpus.marker.collect)test("topmost marker-only folders with protected "+JSON.stringify(row.prefixes),()=>{expect(cleanCollectMarkerOnlyFolderRemovals(root,row.prefixes.map(absolute)).map(item=>posix(item.path))).toEqual(row.expected);expect(cleanCollectMarkerOnlyFolderRemovals(root,row.prefixes.map(absolute)).every(item=>item.kind==="marker-only-folder")).toBe(true);});
test("a taxonomy without marker kinds only keeps the ticket important leaf",()=>{expect([...cleanEmptyMarkerFilenames({fileKinds:{x:{emoji:"x",extensionChains:[".x"],role:"specification"}}} as never)]).toEqual([corpus.marker.expectedNames.find(name=>name.endsWith(".md")&&!name.includes("empty"))]);});'''
t1 = lib.ts_wrap(UNIT1, "📌️ Contract: marker-only folder discovery lists the topmost directories whose whole tree is zero-byte taxonomy markers, never a diff facet, a protected tree or a build directory; marker names are re-derived from the taxonomy file independently and the corpus is language-agnostic with `@@O@@` walking the same workspace.",
 'import {afterAll} from "bun:test";\nimport {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join,sep} from "node:path";\nimport {cleanCollectMarkerOnlyFolderRemovals,cleanDirectoryTreeIsOnlyEmptyMarkers,cleanEmptyMarkerFilenames,cleanIsEmptyMarkerFile} from "@@' + UNIT1 + '/🟦.ts@@";', b1)
def py(doc, kind):
    return lib.PY_HEAD.format(doc=doc) + r'''import os, tempfile, shutil

root = tempfile.mkdtemp(prefix="folders-oracle-")
for n in corpus["nodes"]:
    t = os.path.join(root, *n["path"].split("/"))
    if n["kind"] == "directory":
        os.makedirs(t, exist_ok=True)
    else:
        os.makedirs(os.path.dirname(t), exist_ok=True)
        open(t, "w", encoding="utf8").write(n.get("text", ""))
SKIP = {"node_modules", ".git", "⚡️cache"}
skipped = lambda n: n in SKIP or n in ("target", "dist", "build", "out") or n.startswith("target-")
DIFF = "🔺️diff"


def walk(visit):
    stack = [root]
    while stack:
        d = stack.pop()
        for name in os.listdir(d):
            a = os.path.join(d, name)
            if not os.path.isdir(a) or os.path.islink(a):
                continue
            act = visit(a, name)
            if act == "enter":
                stack.append(a)


def under(a, prefixes):
    return any(a == p or a.startswith(p + os.sep) for p in prefixes)


def rel(a):
    return os.path.relpath(a, root).replace(os.sep, "/")


def ordered(paths):
    return sorted(paths, key=lambda p: (-len(p), p))
''' + kind + r'''
shutil.rmtree(root, ignore_errors=True)
''' + lib.PY_TAIL
o1 = py("🐍️ Independent model of the marker-only folder discovery: derives the marker names from the taxonomy JSON and walks a rebuilt workspace with os.walk (stdlib only).", r'''
tax = json.load(open(os.path.join(os.path.dirname(os.path.abspath(sys.argv[1])), *([".."] * 3), "🔣️taxonomy.json"), encoding="utf8"))
names = {k["emoji"] + k["extensionChains"][0] for k in tax["fileKinds"].values() if k["role"] == "marker"} | {"📝️.md"}
if sorted(names) != sorted(corpus["marker"]["expectedNames"]):
    failures.append("names")


def is_marker(a):
    return os.path.isfile(a) and not os.path.islink(a) and os.path.getsize(a) == 0 and os.path.basename(a) in names


def only_markers(d):
    if os.path.basename(d) == DIFF:
        return False
    saw = False
    for dp, dn, fn in os.walk(d):
        if DIFF in dn or any(os.path.islink(os.path.join(dp, x)) for x in dn + fn):
            return False
        for x in fn:
            saw = True
            if not is_marker(os.path.join(dp, x)):
                return False
    return saw


for c in corpus["marker"]["files"]:
    if is_marker(os.path.join(root, *c["path"].split("/"))) != c["expected"]:
        failures.append("file " + c["path"])
for c in corpus["marker"]["trees"]:
    if only_markers(os.path.join(root, *c["path"].split("/"))) != c["expected"]:
        failures.append("tree " + c["path"])
for c in corpus["marker"]["collect"]:
    found = set()
    prefixes = [os.path.join(root, *p.split("/")) for p in c["prefixes"]]

    def visit(a, name):
        if skipped(name) or under(a, prefixes):
            return "skip"
        if only_markers(a):
            found.add(a)
        return "enter"
    walk(visit)
    tops = [rel(a) for a in found if os.path.dirname(a) not in found]
    if ordered(tops) != c["expected"]:
        failures.append("collect %s" % c["prefixes"])
''')
lib.write_unit(UNIT1, c1, s1, t1, o1)
# empty folders
UNIT2 = "🧼workspace-cleanup/🔍empty-folders"
c2 = {"version": 1, "nodes": nodes, "empty": empty}
s2 = S.obj({"version": S.const(1), "nodes": S.arr(nd, 1), "empty": S.obj({"dirs": chk, "collect": col})})
b2 = TS_COMMON + '''for(const row of corpus.empty.dirs)test("empty directory "+row.path,()=>{expect(cleanDirectoryIsEmpty(absolute(row.path))).toBe(row.expected);});
test("a symlink to an empty directory is not an empty directory",()=>{if(process.platform==="win32")return;const link=absolute("link-to-e");symlinkSync(absolute("e"),link);try{expect(cleanDirectoryIsEmpty(link)).toBe(false);}finally{rmSync(link);}});
for(const row of corpus.empty.collect)test("empty folders with protected "+JSON.stringify(row.prefixes),()=>{const rows=cleanCollectEmptyFolderRemovals(root,row.prefixes.map(absolute));expect(rows.map(item=>posix(item.path))).toEqual(row.expected);expect(rows.every(item=>item.kind==="empty-folder"&&item.bytes===0)).toBe(true);});'''
t2 = lib.ts_wrap(UNIT2, "📭️ Contract: empty-folder discovery lists every directory without entries, deepest first, never inside protected, build or cache trees and never a symlink; the corpus is language-agnostic and `@@O@@` walks the same workspace.",
 'import {afterAll} from "bun:test";\nimport {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {dirname,join,sep} from "node:path";\nimport {cleanCollectEmptyFolderRemovals,cleanDirectoryIsEmpty} from "@@' + UNIT2 + '/🟦.ts@@";', b2)
o2 = py("🐍️ Independent model of the empty-folder discovery: walks a rebuilt workspace with os.walk and lists directories with no entries (stdlib only).", r'''
for c in corpus["empty"]["dirs"]:
    a = os.path.join(root, *c["path"].split("/"))
    if (os.path.isdir(a) and not os.path.islink(a) and os.listdir(a) == []) != c["expected"]:
        failures.append("dir " + c["path"])
for c in corpus["empty"]["collect"]:
    found = []
    prefixes = [os.path.join(root, *p.split("/")) for p in c["prefixes"]]

    def visit(a, name):
        if skipped(name) or under(a, prefixes):
            return "skip"
        if os.listdir(a) == []:
            found.append(rel(a))
        return "enter"
    walk(visit)
    if ordered(found) != c["expected"]:
        failures.append("collect %s" % c["prefixes"])
''')
lib.write_unit(UNIT2, c2, s2, t2, o2)
