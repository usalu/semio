import lib
from lib import S
UNIT = "🎛commands/📝commit/📊metrics/🔀deltas"
split = [(5, 3, 3, 2, 0), (0, 0, 0, 0, 0), (-1, 4, 0, 0, 4), (3, 3, 3, 0, 0), (2, 9, 2, 0, 7), (7, 0, 0, 7, 0)]
prefix = [("a/b/c", ["a/b"], True), ("a/bc", ["a/b"], False), ("a/b", ["a/b"], True), ("a/b/c", ["a/b/"], True), ("x", [], True), ("x", [""], False), ("a\\b\\c", ["a/b"], True), ("./a/b/c", ["a/b"], True), ("a/b/c", ["z", "a/b"], True), ("a", ["a/b"], False), ("a/b/c", [".\\a\\b"], True)]
paths = [("a.ts", ["a.ts"]), ("old.ts\tnew.ts", ["old.ts", "new.ts"]), ("src/{a => b}/x.ts", ["src/a/x.ts", "src/b/x.ts"]), ("old.ts => new.ts", ["old.ts", "new.ts"]), ("", []), ("  ", []), ("a.ts\ta.ts", ["a.ts"]), ("{old => new}.ts", ["old.ts", "new.ts"]), ("pkg/{a => }/x.ts", ["pkg/a/x.ts", "pkg//x.ts"])]
rows = [{"path": "a.ts", "added": 3, "removed": 1}, {"path": "b.py", "added": 0, "removed": 1}, {"path": "dist/skip.ts", "added": 9, "removed": 9}, {"path": "e.go", "added": 1, "removed": 0}, {"path": "notes.xyz", "added": 5, "removed": 5}, {"path": "lib/f.ts", "added": 2, "removed": 2}, {"path": "docs/x.md", "added": 4, "removed": 0}]
acc = {"all": {"TypeScript": {"added": 2, "removed": 0, "edited": 3}, "Python": {"added": 0, "removed": 1, "edited": 0}, "Go": {"added": 1, "removed": 0, "edited": 0}, "Markdown": {"added": 4, "removed": 0, "edited": 0}},
       "lib": {"TypeScript": {"added": 0, "removed": 0, "edited": 2}}}
repo = {"commits": [{"a.ts": "l1\nl2\nl3\n", "b.py": "x\n", "d.md": "doc\n", "dist/skip.ts": "q\n"}, {"a.ts": "l1\nL2\nl3\nl4\nl5\n", "b.py": None, "e.go": "package x\n", "dist/skip.ts": "q2\n"}],
        "numstat": [{"path": "a.ts", "added": 3, "removed": 1}, {"path": "b.py", "added": 0, "removed": 1}, {"path": "dist/skip.ts", "added": 1, "removed": 1}, {"path": "e.go", "added": 1, "removed": 0}],
        "uloc": {"TypeScript": {"added": 2, "removed": 0, "edited": 1}, "Python": {"added": 0, "removed": 1, "edited": 0}, "Go": {"added": 1, "removed": 0, "edited": 0}},
        "size": {"TypeScript": {"added": 6, "removed": 0, "edited": 9}, "Python": {"added": 0, "removed": 2, "edited": 0}, "Go": {"added": 10, "removed": 0, "edited": 0}},
        "ulocPrefixed": {"Go": {"added": 1, "removed": 0, "edited": 0}}, "prefixes": ["e.go"]}
corpus = {"version": 1, "split": [{"added": a, "removed": r, "edited": e, "netAdded": na, "netRemoved": nr} for a, r, e, na, nr in split],
 "prefix": [{"path": p, "prefixes": x, "expected": e} for p, x, e in prefix], "paths": [{"field": f, "expected": e} for f, e in paths if f != "pkg/{a => }/x.ts"],
 "pathDefects": [{"field": "{ => sub}/x.ts", "expected": ["x.ts", "sub/x.ts"], "defect": "📊️metrics/🔀️deltas/🟦️.ts:pathsFromNumstatRow drops the old side when a brace rename has an empty half (`{ => sub}/x.ts` keeps only `sub/x.ts`)"}],
 "accumulate": {"rows": rows, "expected": acc["all"], "prefix": ["lib"], "expectedPrefixed": acc["lib"]}, "repo": repo}
dl = S.obj({"added": S.int, "removed": S.int, "edited": S.int})
schema = S.obj({"version": S.const(1),
 "split": S.arr(S.obj({"added": S.int, "removed": S.int, "edited": S.int, "netAdded": S.int, "netRemoved": S.int}), 1),
 "prefix": S.arr(S.obj({"path": S.str, "prefixes": S.strs(), "expected": S.bool}), 1), "paths": S.arr(S.obj({"field": S.str, "expected": S.strs()}), 1),
 "pathDefects": S.arr(S.obj({"field": S.str, "expected": S.strs(), "defect": S.str}), 1),
 "accumulate": S.obj({"rows": S.arr(S.obj({"path": S.str, "added": S.int, "removed": S.int})), "expected": S.map(dl), "prefix": S.strs(), "expectedPrefixed": S.map(dl)}),
 "repo": S.obj({"commits": S.arr(S.map(S.one(S.str, S.null)), 2), "numstat": S.arr(S.obj({"path": S.str, "added": S.int, "removed": S.int})), "uloc": S.map(dl), "size": S.map(dl), "ulocPrefixed": S.map(dl), "prefixes": S.strs()})})
bodyts = '''const plain=(map:Map<string,Row>):Record<string,Row>=>Object.fromEntries([...map.entries()].sort(([a],[b])=>a<b?-1:1));
type Row={added:number;removed:number;edited:number};
const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const replay=(commits:Record<string,string|null>[]):{root:string;revisions:string[]}=>{
 const root=mkdtempSync(join(tmpdir(),"deltas-"));git(root,"init","-q");const revisions:string[]=[];
 for(const files of commits){for(const[path,text]of Object.entries(files)){const target=join(root,path);if(text===null)rmSync(target);else{mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}}git(root,"add","-A");git(root,"commit","-q","-m","c");revisions.push(git(root,"rev-parse","HEAD"));}
 return{root,revisions};
};
for(const row of corpus.split)test("split numstat "+row.added+"/"+row.removed,()=>{expect(splitGitNumstatDelta(row.added,row.removed)).toEqual({edited:row.edited,added:row.netAdded,removed:row.netRemoved});});
for(const row of corpus.prefix)test("path "+JSON.stringify(row.path)+" under "+JSON.stringify(row.prefixes),()=>{expect(pathUnderPrefixes(row.path,row.prefixes)).toBe(row.expected);});
for(const row of corpus.paths)test("numstat path field "+JSON.stringify(row.field),()=>{expect(pathsFromNumstatRow(row.field)).toEqual(row.expected);});
for(const row of corpus.pathDefects)test.failing("numstat path defect "+JSON.stringify(row.field),()=>{expect(pathsFromNumstatRow(row.field)).toEqual(row.expected);});
test("the line total sums added, edited and removed",()=>{expect(gitDeltaLineTotal({added:2,edited:3,removed:4})).toBe(9);expect(gitDeltaLineTotal({added:0,edited:0,removed:0})).toBe(0);});
test("rows accumulate per language, skipping skipped paths and unknown languages",()=>{
 expect(plain(accumulateGitDeltasFromNumstat("/repo",corpus.accumulate.rows))).toEqual(corpus.accumulate.expected);
 expect(plain(accumulateGitDeltasFromNumstat("/repo",corpus.accumulate.rows,corpus.accumulate.prefix))).toEqual(corpus.accumulate.expectedPrefixed);
 const total=sumGitLangDeltas(accumulateGitDeltasFromNumstat("/repo",corpus.accumulate.rows));expect(total).toEqual({added:7,removed:1,edited:3});
 expect(sumGitLangDeltas(new Map())).toEqual({added:0,removed:0,edited:0});
});
test("a real git range reproduces numstat, uloc and byte deltas",()=>{
 const {root,revisions}=replay(corpus.repo.commits);
 try{
  const[base,head]=revisions as [string,string];
  expect(gitRangeNumstat(root,base,head)).toEqual(corpus.repo.numstat);
  expect(plain(accumulateRangeUlocDeltas(root,base,head))).toEqual(corpus.repo.uloc);
  expect(plain(accumulateRangeSizeDeltas(root,base,head))).toEqual(corpus.repo.size);
  expect(plain(accumulateRangeUlocDeltas(root,base,head,corpus.repo.prefixes))).toEqual(corpus.repo.ulocPrefixed);
 }finally{rmSync(root,{recursive:true,force:true});}
});'''
ts = lib.ts_wrap(UNIT, "🔀️ Contract: commit metric deltas split numstat into edited, added and removed lines, scope rows by path prefix and language, and reproduce a real git range; the corpus is language-agnostic and `@@O@@` replays the commits with the git CLI and recomputes the deltas from `git diff --numstat`.",
 'import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {accumulateGitDeltasFromNumstat,accumulateRangeSizeDeltas,accumulateRangeUlocDeltas,gitDeltaLineTotal,gitRangeNumstat,pathUnderPrefixes,pathsFromNumstatRow,splitGitNumstatDelta,sumGitLangDeltas} from "@@' + UNIT + '/🟦.ts@@";', bodyts)
ts = ts.replace('import {testPythonV1}', 'import {testPythonV1}', 1)
ORACLE = lib.PY_HEAD.format(doc="🐍️ Independent model of the commit metric deltas: replays the corpus commits in a scratch repository with the git CLI and recomputes numstat, line and byte deltas (stdlib + git).") + r'''import os, subprocess, tempfile, shutil

SKIP = (".🧬semio", "node_modules", "dist", "build", "target", ".git", ".nx", "coverage", ".cache", ".turbo", ".next", "out", "vendor", "third_party", "Carthage")
LANG = {".ts": "TypeScript", ".py": "Python", ".go": "Go", ".md": "Markdown"}


def split(a, r):
    a, r = max(0, a), max(0, r)
    e = min(a, r)
    return {"added": a - e, "removed": r - e, "edited": e}


for c in corpus["split"]:
    if split(c["added"], c["removed"]) != {"added": c["netAdded"], "removed": c["netRemoved"], "edited": c["edited"]}:
        failures.append("split %s/%s" % (c["added"], c["removed"]))


def under(path, prefixes):
    if not prefixes:
        return True
    n = path.replace("\\", "/")
    n = n[2:] if n.startswith("./") else n
    for raw in prefixes:
        p = raw.replace("\\", "/")
        p = p[2:] if p.startswith("./") else p
        p = p.rstrip("/")
        if p and (n == p or n.startswith(p + "/")):
            return True
    return False


for c in corpus["prefix"]:
    if under(c["path"], c["prefixes"]) != c["expected"]:
        failures.append("prefix %s" % c["path"])


def lang(path):
    return LANG.get(os.path.splitext(path)[1], "")


def skipped(path):
    return any(path == d or path.startswith(d + "/") for d in SKIP) or any(s.startswith(".") for s in path.split("/"))


def accumulate(rows, prefixes=None):
    out = {}
    for r in rows:
        if skipped(r["path"]) or (prefixes and not under(r["path"], prefixes)) or not lang(r["path"]):
            continue
        d = out.setdefault(lang(r["path"]), {"added": 0, "removed": 0, "edited": 0})
        for k, v in split(r["added"], r["removed"]).items():
            d[k] += v
    return out


acc = corpus["accumulate"]
if accumulate(acc["rows"]) != acc["expected"]:
    failures.append("accumulate")
if accumulate(acc["rows"], acc["prefix"]) != acc["expectedPrefixed"]:
    failures.append("accumulate prefixed")


def git(cwd, *args):
    env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "-c", "user.name=t", "-c", "user.email=t@example.com"] + list(args), cwd=cwd, env=env, check=True, capture_output=True, text=True).stdout


root = tempfile.mkdtemp(prefix="deltas-oracle-")
try:
    git(root, "init", "-q")
    revs = []
    for files in corpus["repo"]["commits"]:
        for path, text in files.items():
            full = os.path.join(root, path)
            if text is None:
                os.remove(full)
            else:
                os.makedirs(os.path.dirname(full), exist_ok=True)
                open(full, "w").write(text)
        git(root, "add", "-A")
        git(root, "commit", "-q", "-m", "c")
        revs.append(git(root, "rev-parse", "HEAD").strip())
    rows = []
    for line in git(root, "diff", "--numstat", revs[0] + ".." + revs[1]).splitlines():
        a, r, p = line.split("\t")
        rows.append({"path": p, "added": int(a), "removed": int(r)})
    if rows != corpus["repo"]["numstat"]:
        failures.append("numstat")
    if accumulate(rows) != corpus["repo"]["uloc"]:
        failures.append("uloc")
    if accumulate(rows, corpus["repo"]["prefixes"]) != corpus["repo"]["ulocPrefixed"]:
        failures.append("uloc prefixed")

    def blob(rev, path):
        r = subprocess.run(["git", "cat-file", "-s", rev + ":" + path], cwd=root, capture_output=True, text=True)
        return int(r.stdout) if r.returncode == 0 else 0

    sizes = {}
    for r in rows:
        if skipped(r["path"]) or not lang(r["path"]):
            continue
        d = sizes.setdefault(lang(r["path"]), {"added": 0, "removed": 0, "edited": 0})
        for k, v in split(blob(revs[1], r["path"]), blob(revs[0], r["path"])).items():
            d[k] += v
    if sizes != corpus["repo"]["size"]:
        failures.append("size")
finally:
    shutil.rmtree(root, ignore_errors=True)
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, ORACLE)
