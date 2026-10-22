import re, lib
from lib import S
from spec_metrics import body_line
V = "️"
DATE = lambda yy, mm, dd: "🎆" + V + yy + "🌙" + V + mm + "☀" + V + dd
D21, D22 = DATE("26", "10", "21"), DATE("26", "10", "22")
STAMP = lambda d: d + "⏰" + V + "10⌚" + V + "00⏱" + V + "00"
B0, B1 = "🏘" + V + "framework🧰" + V + "core", "🖱" + V + "ui🧩" + V + "button"
def bundle(label, dates):
    return {"label": label, "dates": [{"dateLine": d, "bullets": ["🚀x"]} for d in dates]}
P = "🐙" + V + "ueli"
repo = {"base": {"README.md": "readme\n"},
        "commits": [{"subject": P + D21 + "🚩" + V + "1", "body": STAMP(D21), "files": {"framework/core/a.ts": "l1\nl2\nl3\n", "ui/button/b.ts": "m1\nm2\n"}},
                    {"subject": P + D22 + "🚩" + V + "2", "body": STAMP(D22), "files": {"framework/core/a.ts": "l1\nl2\nl3\nl4\nl5\n"}}]}
bundles = [bundle(B0, [D21, D22]), bundle(B1, [D21])]
zero = {"added": 0, "removed": 0, "edited": 0}
def d(**kw): return dict(zero, **kw)
expected = {
 "prefixSets": [["framework/core/a.ts", "framework/core"], ["ui/button/b.ts", "ui/button"]],
 "assigned": [["framework/core/a.ts"], ["ui/button/b.ts"]],
 "infer": [{"label": B0, "paths": None, "expected": ["framework/core/a.ts"]}, {"label": B1, "paths": None, "expected": ["ui/button/b.ts"]}, {"label": B0, "paths": ["framework/core/a.ts", "framework/core/c.ts"], "expected": ["framework/core"]}, {"label": B0, "paths": ["docs/x.md"], "expected": []}, {"label": "🏘" + V, "paths": None, "expected": []}, {"label": B0, "paths": ["Framework/Core/Z.ts", "framework/core/y.ts"], "expected": ["framework/core"][:0] + ["Framework/Core/Z.ts"][:0] + [""][:0] + ["framework/core"][:0] + ["framework"][:0] + [], }],
 "dateUloc": [[[D21, d(added=3)], [D22, d(added=2)]], [[D21, d(added=2)]]],
 "dateSize": [[[D21, d(added=9)], [D22, d(added=6, edited=9)]], [[D21, d(added=6)]]],
 "partitionUloc": {"bundleTotals": [d(added=5), d(added=2)], "rangeTotal": d(added=7)},
 "partitionSize": {"bundleTotals": [d(added=15), d(added=6)], "rangeTotal": d(added=21)},
}
expected["infer"][5] = {"label": B0, "paths": ["Framework/Core/Z.ts", "framework/core/y.ts"], "expected": ["framework/core/y.ts"][:0] + ["Framework/Core/Z.ts"][:0] + []}
# longest common prefix of differently-cased paths: first segments differ -> len 0 -> first segment of the first path
expected["infer"][5]["expected"] = ["Framework"]
row_match = [("framework/core/a.ts", [0]), ("ui/button/b.ts", [1]), ("docs/x.md", []), ("old/x.ts\tframework/core/a.ts", [0]), ("framework/core/a.ts\tui/button/b.ts", [0, 1]), ("{ui => framework}/core/a.ts", [0])]
validate_pure = [
 {"name": "days that add up are accepted", "totals": [d(added=5), d(added=2)], "days": [[[D21, d(added=3)], [D22, d(added=2)]], [[D21, d(added=2)]]], "additive": True, "message": None},
 {"name": "a day missing from the bundle body is refused", "bundles": [bundle(B0, [D21]), bundle(B1, [D21])], "totals": [d(added=5), d(added=2)], "days": [[[D21, d(added=3)], [D22, d(added=2)]], [[D21, d(added=2)]]], "additive": True, "message": "that day is missing from your bundle body"},
 {"name": "days that do not add up to the bundle are refused", "totals": [d(added=6), d(added=2)], "days": [[[D21, d(added=3)], [D22, d(added=2)]], [[D21, d(added=2)]]], "additive": True, "message": "does not add up to the bundle total"},
 {"name": "the byte kind need not add up", "totals": [d(added=60), d(added=2)], "days": [[[D21, d(added=3)], [D22, d(added=2)]], [[D21, d(added=2)]]], "additive": False, "message": None},
 {"name": "an unlisted day with no change is ignored", "bundles": [bundle(B0, [D21]), bundle(B1, [D21])], "totals": [d(added=3), d(added=2)], "days": [[[D21, d(added=3)], [D22, d()]], [[D21, d(added=2)]]], "additive": True, "message": None},
 {"name": "a bundle without day data counts as zero", "totals": [d(), d()], "days": [[], []], "additive": True, "message": None},
]
brief = [(d(added=1, edited=2, removed=3), "➕" + V + "1✏" + V + "2➖" + V + "3🟰" + V + "6"), (d(), "➕" + V + "0✏" + V + "0➖" + V + "0🟰" + V + "0")]
header = []
for label, u, s, ub, sb in [(B0 + "📊" + V + "metric📃uloc", d(added=5), d(added=15), 100, 4000), (" " + B1 + " ", d(), d(), 0, 0)]:
    n = re.sub(r"📊.*$", "", label).strip()
    header.append({"label": label, "uloc": u, "size": s, "ulocBloc": ub, "sizeBloc": sb, "expected": n + body_line("uloc", ub, u["added"], u["edited"], u["removed"]) + body_line("size", sb, s["added"], s["edited"], s["removed"])})
commit_errors = [
 {"name": "a changed path that no bundle owns", "extra": {"docs/readme.md": "x\n"}, "bundles": bundles, "message": "changed path is not attributed to any bundle"},
 {"name": "a changed path that two bundles own", "extra": {"framework/other/z.ts": "x\n"}, "bundles": [bundle("🥅" + V + "framework", [D21, D22])] + bundles, "message": "changed path matches multiple bundles"},
 {"name": "a bundle body that omits a micro-commit day", "extra": {}, "bundles": [bundle(B0, [D21]), bundle(B1, [D21])], "message": "that day is missing from your bundle body"},
]
corpus = {"version": 1, "repo": repo, "bundles": bundles, "expected": expected, "rowMatch": [{"field": f, "expected": e} for f, e in row_match], "validate": validate_pure, "brief": [{"delta": a, "expected": b} for a, b in brief], "header": header, "commitErrors": commit_errors,
          "pathMatch": [{"path": "framework/core/a.ts", "index": 0, "prefixSets": [["framework/core"], ["ui"]], "expected": True}, {"path": "framework/core/a.ts", "index": 1, "prefixSets": [["framework/core"], ["ui"]], "expected": False}, {"path": "framework/core/a.ts", "index": 0, "prefixSets": [[], []], "expected": True}, {"path": "ui/button/z.ts", "index": 0, "prefixSets": [[], []], "expected": False}, {"path": "ui/button/z.ts", "index": 1, "prefixSets": [[], []], "expected": True}],
          "sortReverse": {"expectedOrder": [B0, B1]}}
dl = S.obj({"added": S.int, "removed": S.int, "edited": S.int})
bs = S.obj({"label": S.str, "dates": S.arr(S.obj({"dateLine": S.str, "bullets": S.strs()}))})
days = S.arr(S.arr(S.arr(S.anyv, 2)))
schema = S.obj({"version": S.const(1), "repo": S.obj({"base": S.map(S.str), "commits": S.arr(S.obj({"subject": S.str, "body": S.str, "files": S.map(S.str)}), 2)}), "bundles": S.arr(bs, 2),
 "expected": S.obj({"prefixSets": S.arr(S.strs()), "assigned": S.arr(S.strs()), "infer": S.arr(S.obj({"label": S.str, "paths": S.one(S.strs(), S.null), "expected": S.strs()})), "dateUloc": days, "dateSize": days, "partitionUloc": S.obj({"bundleTotals": S.arr(dl), "rangeTotal": dl}), "partitionSize": S.obj({"bundleTotals": S.arr(dl), "rangeTotal": dl})}),
 "rowMatch": S.arr(S.obj({"field": S.str, "expected": S.arr(S.int)}), 1),
 "validate": S.arr(S.obj({"name": S.str, "bundles": S.arr(bs, 2), "totals": S.arr(dl), "days": days, "additive": S.bool, "message": S.one(S.str, S.null)}, required=["name", "totals", "days", "additive", "message"]), 1),
 "brief": S.arr(S.obj({"delta": dl, "expected": S.str}), 1), "header": S.arr(S.obj({"label": S.str, "uloc": dl, "size": dl, "ulocBloc": S.int, "sizeBloc": S.int, "expected": S.str}), 1),
 "commitErrors": S.arr(S.obj({"name": S.str, "extra": S.map(S.str), "bundles": S.arr(bs, 1), "message": S.str}), 1),
 "pathMatch": S.arr(S.obj({"path": S.str, "index": S.int, "prefixSets": S.arr(S.strs()), "expected": S.bool}), 1), "sortReverse": S.obj({"expectedOrder": S.strs()})})
bodyts = '''type Delta={added:number;removed:number;edited:number};
const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const put=(root:string,files:Record<string,string>)=>{for(const[path,text]of Object.entries(files)){const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}};
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const replay=(extra:Record<string,string>={}):{root:string;base:string;head:string}=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"attrib-")));roots.push(root);git(root,"init","-q");put(root,corpus.repo.base);git(root,"add","-A");git(root,"commit","-q","-m","base");const base=git(root,"rev-parse","HEAD");
 for(const[index,commit]of corpus.repo.commits.entries()){put(root,commit.files);if(index===corpus.repo.commits.length-1)put(root,extra);git(root,"add","-A");git(root,"commit","-q","-m",commit.subject,"-m",commit.body);}
 return{root,base,head:git(root,"rev-parse","HEAD")};
};
const asRows=(map:BundleDateDeltasMap):unknown=>[...map.keys()].sort().map(index=>[...map.get(index)!.entries()].sort(([a],[b])=>a<b?-1:1));
const stub={countRepoUlocByLanguage:()=>({TypeScript:100}),countRepoSizeByLanguage:()=>({TypeScript:4000})};
const fixed=replay(),sets=buildBundlePathPrefixSets(fixed.root,fixed.base,fixed.head,corpus.bundles);
test("changed paths are assigned to the bundle whose label tokens they contain",()=>{expect(assignChangedPathsToBundles(fixed.root,fixed.base,fixed.head,corpus.bundles)).toEqual(corpus.expected.assigned);});
test("each bundle owns its assigned paths and every parent that still carries the label tokens",()=>{expect(sets).toEqual(corpus.expected.prefixSets);});
for(const[index,row]of corpus.expected.infer.entries())test("inferred prefix "+index+" of "+row.label,()=>{expect(inferPathPrefixesForBundleLabel(fixed.root,fixed.base,fixed.head,row.label,row.paths??undefined)).toEqual(row.expected);});
for(const row of corpus.rowMatch)test("bundle owners of the numstat row "+JSON.stringify(row.field),()=>{expect(resolveBundleIndicesForNumstatRow(row.field,sets,corpus.bundles).sort()).toEqual(row.expected);});
for(const[index,row]of corpus.pathMatch.entries())test("path match "+index,()=>{expect(pathMatchesBundleIndex(row.path,row.index,row.prefixSets,corpus.bundles)).toBe(row.expected);});
test("per-day uloc and byte deltas are summed per bundle over the micro-commits",()=>{
 expect(asRows(buildBundleDateDeltasMap(fixed.root,fixed.base,fixed.head,corpus.bundles))).toEqual(corpus.expected.dateUloc);expect(asRows(buildBundleDateSizeDeltasMap(fixed.root,fixed.base,fixed.head,corpus.bundles))).toEqual(corpus.expected.dateSize);
});
test("the range partitions across bundles with matching totals",()=>{
 expect(partitionRangeDeltasByBundle(fixed.root,fixed.base,fixed.head,corpus.bundles,sets)).toEqual(corpus.expected.partitionUloc);expect(partitionRangeSizeDeltasByBundle(fixed.root,fixed.base,fixed.head,corpus.bundles,sets)).toEqual(corpus.expected.partitionSize);
 expect(rangeGitDeltaTotal(fixed.root,fixed.base,fixed.head)).toEqual(corpus.expected.partitionUloc.rangeTotal);
});
test("bundles are ranked by their edit total",()=>{
 const reversed=[corpus.bundles[1]!,corpus.bundles[0]!],ranked=sortCommitBundlesByEditTotal(fixed.root,fixed.base,fixed.head,reversed,[["b"],["a"]]);
 expect(ranked.bundles.map(item=>item.label)).toEqual(corpus.sortReverse.expectedOrder);expect(ranked.pathAssignments).toEqual([["a"],["b"]]);expect(ranked.pathPrefixSets).toEqual(corpus.expected.prefixSets);
});
for(const row of corpus.brief)test("brief delta "+JSON.stringify(row.delta),()=>{expect(formatGitDeltaSumBrief(row.delta)).toBe(row.expected);});
for(const[index,row]of corpus.header.entries())test("bundle header line "+index,()=>{expect(formatBundleHeaderLine(row.label,row.uloc,row.size,row.ulocBloc,row.sizeBloc)).toBe(row.expected);});
for(const row of corpus.validate)test("day attribution: "+row.name,()=>{
 const bundles=row.bundles??corpus.bundles,dateDeltas:BundleDateDeltasMap=new Map(row.days.map((days,index)=>[index,new Map(days as [string,Delta][])]));
 const run=()=>validateBundleDayDeltasAttribution(bundles,[],dateDeltas,row.totals,"uloc",row.additive);
 if(row.message===null)expect(run).not.toThrow();else expect(run).toThrow(row.message);
});
test("a complete bundle body passes the whole attribution check",()=>{expect(()=>validateBundleCommitAttribution(fixed.root,fixed.base,fixed.head,corpus.bundles,stub)).not.toThrow();});
for(const row of corpus.commitErrors)test("commit attribution refuses "+row.name,()=>{const world=replay(row.extra);expect(()=>validateBundleCommitAttribution(world.root,world.base,world.head,row.bundles,stub)).toThrow(row.message);});'''
ts_ = lib.ts_wrap("🎛commands/📝commit/📈attribution", "📈️ Contract: commit attribution assigns every changed path to exactly one bundle by label tokens, sums micro-commit uloc and byte deltas per bundle and per day over a real git range, and refuses unowned or doubly owned paths, missing days and non-additive totals; the corpus is language-agnostic and `@@O@@` replays the commits with the git CLI and recomputes assignment and sums from `git diff --numstat`.",
 'import {afterAll} from "bun:test";\nimport {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {assignChangedPathsToBundles,buildBundlePathPrefixSets,inferPathPrefixesForBundleLabel,pathMatchesBundleIndex,resolveBundleIndicesForNumstatRow} from "@@🎛commands/📝commit/🧭path-prefixes/🟦.ts@@";\nimport {buildBundleDateDeltasMap,buildBundleDateSizeDeltasMap,formatBundleHeaderLine,formatGitDeltaSumBrief,partitionRangeDeltasByBundle,partitionRangeSizeDeltasByBundle,rangeGitDeltaTotal,sortCommitBundlesByEditTotal,validateBundleCommitAttribution,validateBundleDayDeltasAttribution,type BundleDateDeltasMap} from "@@🎛commands/📝commit/📈attribution/🟦.ts@@";', bodyts)
spec = open("spec_metrics.py", encoding="utf8").read()
oracle = lib.PY_HEAD.format(doc="🐍️ Independent model of the commit attribution: replays the corpus commits with the git CLI, assigns paths to bundles by label tokens and recomputes the per-bundle and per-day sums from `git diff --numstat` (stdlib + git).") + spec + r'''import os, subprocess, tempfile, shutil

EMOJI = lambda c: 0x1F000 <= ord(c) <= 0x1FAFF or 0x2600 <= ord(c) <= 0x27BF or 0x2B00 <= ord(c) <= 0x2BFF


def tokens(label):
    t = label.strip()
    k = t.find("📊" + VS + "metric")
    t = (t[:k] if k >= 0 else t).replace("🔀" + VS, "").replace("🚩" + VS, "").strip()
    t = "".join(" " if EMOJI(c) else c for c in t).lower()
    return [x for x in re.split(r"[^a-z0-9]+", t) if len(x) >= 2 and x not in {"uloc", "repo", "the", "and"}]


def assign(path, bundles):
    best, score = -1, 0
    for i, b in enumerate(bundles):
        tk = tokens(b["label"])
        if tk and all(t in path.lower() for t in tk) and len(tk) > score:
            best, score = i, len(tk)
    return best


def git(cwd, *a):
    env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "-c", "user.name=t", "-c", "user.email=t@e.x"] + list(a), cwd=cwd, env=env, check=True, capture_output=True, text=True).stdout.strip()


def split(a, r):
    e = min(a, r)
    return a - e, e, r - e


root = tempfile.mkdtemp(prefix="attrib-oracle-")
try:
    git(root, "init", "-q")

    def put(files):
        for p, t in files.items():
            f = os.path.join(root, *p.split("/"))
            os.makedirs(os.path.dirname(f), exist_ok=True)
            open(f, "w").write(t)
    put(corpus["repo"]["base"])
    git(root, "add", "-A")
    git(root, "commit", "-q", "-m", "base")
    revs = [git(root, "rev-parse", "HEAD")]
    for c in corpus["repo"]["commits"]:
        put(c["files"])
        git(root, "add", "-A")
        git(root, "commit", "-q", "-m", c["subject"], "-m", c["body"])
        revs.append(git(root, "rev-parse", "HEAD"))
    bundles = corpus["bundles"]
    assigned = [[] for _ in bundles]
    for line in git(root, "diff", "--name-only", revs[0] + ".." + revs[-1]).split("\n"):
        i = assign(line, bundles)
        if i >= 0:
            assigned[i].append(line)
    if assigned != corpus["expected"]["assigned"]:
        failures.append("assigned")

    def numstat(a, b):
        out = []
        for l in git(root, "diff", "--numstat", a + ".." + b).split("\n"):
            x, y, p = l.split("\t")
            out.append((p, int(x), int(y)))
        return out

    def blob_size(rev, p):
        r = subprocess.run(["git", "cat-file", "-s", rev + ":" + p], cwd=root, capture_output=True, text=True)
        return int(r.stdout) if r.returncode == 0 else 0
    for kind, key in (("uloc", "dateUloc"), ("size", "dateSize")):
        got = [dict() for _ in bundles]
        for n in range(1, len(revs)):
            day = re.search("🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2}", corpus["repo"]["commits"][n - 1]["body"]).group(0)
            for p, a, r in numstat(revs[n - 1], revs[n]):
                i = assign(p, bundles)
                ad, ed, rm = split(a, r) if kind == "uloc" else split(blob_size(revs[n], p), blob_size(revs[n - 1], p))
                cur = got[i].setdefault(day, {"added": 0, "removed": 0, "edited": 0})
                cur["added"] += ad
                cur["edited"] += ed
                cur["removed"] += rm
        want = [{d: v for d, v in days} for days in corpus["expected"][key]]
        if got != want:
            failures.append("per-day " + kind)
    totals = [{"added": 0, "removed": 0, "edited": 0} for _ in bundles]
    for p, a, r in numstat(revs[0], revs[-1]):
        ad, ed, rm = split(a, r)
        i = assign(p, bundles)
        totals[i]["added"] += ad
        totals[i]["edited"] += ed
        totals[i]["removed"] += rm
    if totals != corpus["expected"]["partitionUloc"]["bundleTotals"]:
        failures.append("partition")
    stotals = [{"added": 0, "removed": 0, "edited": 0} for _ in bundles]
    for p, a, r in numstat(revs[0], revs[-1]):
        ad, ed, rm = split(blob_size(revs[-1], p), blob_size(revs[0], p))
        i = assign(p, bundles)
        stotals[i]["added"] += ad
        stotals[i]["edited"] += ed
        stotals[i]["removed"] += rm
    if stotals != corpus["expected"]["partitionSize"]["bundleTotals"]:
        failures.append("size partition")
finally:
    shutil.rmtree(root, ignore_errors=True)
for c in corpus["brief"]:
    d = c["delta"]
    if "➕" + VS + str(d["added"]) + "✏" + VS + str(d["edited"]) + "➖" + VS + str(d["removed"]) + "🟰" + VS + str(d["added"] + d["edited"] + d["removed"]) != c["expected"]:
        failures.append("brief")
for c in corpus["header"]:
    n = c["label"].strip()
    k = n.find("📊" + VS + "metric")
    n = (n[:k] if k >= 0 else n).replace("🔀" + VS, "").replace("🚩" + VS, "").strip()
    u, s = c["uloc"], c["size"]
    if n + body_line("uloc", c["ulocBloc"], u["added"], u["edited"], u["removed"]) + body_line("size", c["sizeBloc"], s["added"], s["edited"], s["removed"]) != c["expected"]:
        failures.append("header")
for c in corpus["validate"]:
    bundles_ = c.get("bundles", corpus["bundles"])
    err = None
    for i, b in enumerate(bundles_):
        listed = {d["dateLine"] for d in b["dates"]}
        per = dict(c["days"][i]) if i < len(c["days"]) else {}
        for day, v in per.items():
            if day not in listed and v["added"] + v["edited"] + v["removed"] != 0:
                err = "missing"
        if c["additive"]:
            tot = c["totals"][i]
            s = {k: sum(per.get(d, {"added": 0, "removed": 0, "edited": 0})[k] for d in listed) for k in ("added", "removed", "edited")}
            if s != tot:
                err = err or "additive"
    if (err is None) != (c["message"] is None):
        failures.append("validate: " + c["name"])
''' + lib.PY_TAIL
lib.write_unit("🎛commands/📝commit/📈attribution", corpus, schema, ts_, oracle)
# path-prefixes unit shares the scenario: its own tests dir holds a pure-logic copy
UP = "🎛commands/📝commit/🧭path-prefixes"
corpus_p = {k: corpus[k] for k in ("version", "repo", "bundles", "rowMatch", "pathMatch")}
corpus_p["expected"] = {k: expected[k] for k in ("prefixSets", "assigned", "infer")}
schema_p = S.obj({"version": S.const(1), "repo": schema["properties"]["repo"], "bundles": schema["properties"]["bundles"], "rowMatch": schema["properties"]["rowMatch"], "pathMatch": schema["properties"]["pathMatch"],
                  "expected": S.obj({"prefixSets": S.arr(S.strs()), "assigned": S.arr(S.strs()), "infer": schema["properties"]["expected"]["properties"]["infer"]})})
body_p = '''const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const put=(root:string,files:Record<string,string>)=>{for(const[path,text]of Object.entries(files)){const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}};
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const root=realpathSync(mkdtempSync(join(tmpdir(),"prefixes-")));roots.push(root);git(root,"init","-q");put(root,corpus.repo.base);git(root,"add","-A");git(root,"commit","-q","-m","base");const base=git(root,"rev-parse","HEAD");
for(const commit of corpus.repo.commits){put(root,commit.files);git(root,"add","-A");git(root,"commit","-q","-m",commit.subject,"-m",commit.body);}
const head=git(root,"rev-parse","HEAD"),sets=buildBundlePathPrefixSets(root,base,head,corpus.bundles);
test("changed paths are assigned to the bundle whose label tokens they contain",()=>{expect(assignChangedPathsToBundles(root,base,head,corpus.bundles)).toEqual(corpus.expected.assigned);});
test("each bundle owns its assigned paths and every parent that still carries the label tokens",()=>{expect(sets).toEqual(corpus.expected.prefixSets);});
for(const[index,row]of corpus.expected.infer.entries())test("inferred prefix "+index+" of "+row.label,()=>{expect(inferPathPrefixesForBundleLabel(root,base,head,row.label,row.paths??undefined)).toEqual(row.expected);});
for(const row of corpus.rowMatch)test("bundle owners of the numstat row "+JSON.stringify(row.field),()=>{expect(resolveBundleIndicesForNumstatRow(row.field,sets,corpus.bundles).sort()).toEqual(row.expected);});
for(const[index,row]of corpus.pathMatch.entries())test("path match "+index,()=>{expect(pathMatchesBundleIndex(row.path,row.index,row.prefixSets,corpus.bundles)).toBe(row.expected);});
test("a revision range without label matches or with an unknown base yields no prefixes",()=>{expect(inferPathPrefixesForBundleLabel(root,"no-such-revision",head,corpus.bundles[0]!.label)).toEqual([]);expect(assignChangedPathsToBundles(root,"no-such-revision",head,corpus.bundles)).toEqual([[],[]]);});'''
ts_p = lib.ts_wrap(UP, "🧭️ Contract: bundle path prefixes follow the label tokens: a path belongs to the bundle with the most matching tokens (first wins a tie), a bundle owns its assigned paths and their token-carrying parents, and a numstat row resolves to the bundles owning any of its paths; the corpus is language-agnostic and `@@O@@` replays the commits with the git CLI and recomputes the assignment.",
 'import {afterAll} from "bun:test";\nimport {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {assignChangedPathsToBundles,buildBundlePathPrefixSets,inferPathPrefixesForBundleLabel,pathMatchesBundleIndex,resolveBundleIndicesForNumstatRow} from "@@' + UP + '/🟦.ts@@";', body_p)
oracle_p = lib.PY_HEAD.format(doc="🐍️ Independent model of the bundle path prefixes: replays the commits with the git CLI and assigns changed paths by label tokens, then derives the token-carrying parent prefixes (stdlib + git).") + r'''import os, subprocess, tempfile, shutil

EMOJI = lambda c: 0x1F000 <= ord(c) <= 0x1FAFF or 0x2600 <= ord(c) <= 0x27BF or 0x2B00 <= ord(c) <= 0x2BFF


def tokens(label):
    t = "".join(" " if EMOJI(c) else c for c in label.strip()).lower()
    return [x for x in re.split(r"[^a-z0-9]+", t) if len(x) >= 2 and x not in {"uloc", "repo", "the", "and"}]


def assign(path, bundles):
    best, score = -1, 0
    for i, b in enumerate(bundles):
        tk = tokens(b["label"])
        if tk and all(t in path.lower() for t in tk) and len(tk) > score:
            best, score = i, len(tk)
    return best


def common_prefix(paths):
    if not paths:
        return ""
    parts = [p.split("/") for p in paths]
    n = 0
    for i, seg in enumerate(parts[0]):
        if all(len(q) > i and q[i] == seg for q in parts):
            n = i + 1
        else:
            break
    return paths[0].split("/")[0] if n == 0 else "/".join(parts[0][:n])


def git(cwd, *a):
    env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "-c", "user.name=t", "-c", "user.email=t@e.x"] + list(a), cwd=cwd, env=env, check=True, capture_output=True, text=True).stdout.strip()


root = tempfile.mkdtemp(prefix="prefix-oracle-")
try:
    git(root, "init", "-q")

    def put(files):
        for p, t in files.items():
            f = os.path.join(root, *p.split("/"))
            os.makedirs(os.path.dirname(f), exist_ok=True)
            open(f, "w").write(t)
    put(corpus["repo"]["base"])
    git(root, "add", "-A")
    git(root, "commit", "-q", "-m", "base")
    base = git(root, "rev-parse", "HEAD")
    for c in corpus["repo"]["commits"]:
        put(c["files"])
        git(root, "add", "-A")
        git(root, "commit", "-q", "-m", c["subject"], "-m", c["body"])
    head = git(root, "rev-parse", "HEAD")
    changed = [p for p in git(root, "diff", "--name-only", base + ".." + head).split("\n") if p]
    bundles = corpus["bundles"]
    assigned = [[] for _ in bundles]
    for p in changed:
        i = assign(p, bundles)
        if i >= 0:
            assigned[i].append(p)
    if assigned != corpus["expected"]["assigned"]:
        failures.append("assigned")
    sets = []
    for i, b in enumerate(bundles):
        tk, s = tokens(b["label"]), []
        for p in assigned[i]:
            if p not in s:
                s.append(p)
            acc = ""
            for seg in p.split("/"):
                acc = acc + "/" + seg if acc else seg
                if tk and all(t in acc.lower() for t in tk) and acc not in s:
                    s.append(acc)
        sets.append(s)
    if sets != corpus["expected"]["prefixSets"]:
        failures.append("prefix sets")
    for c in corpus["expected"]["infer"]:
        tk = tokens(c["label"])
        pool = c["paths"] if c["paths"] is not None else changed
        matched = [p for p in pool if tk and all(t in p.lower() for t in tk)]
        got = [common_prefix(matched)] if matched and common_prefix(matched) else []
        if got != c["expected"]:
            failures.append("infer %s" % c["label"])
finally:
    shutil.rmtree(root, ignore_errors=True)


def owners(field, sets_, bundles_):
    out = set()
    paths = []
    for part in [p.strip() for p in field.split("\t") if p.strip()]:
        m = re.match(r"^(.*)\{(.+?)\s*=>\s*(.+?)\}(.*)$", part)
        if m:
            for side in (m.group(2).strip(), m.group(3).strip()):
                if side:
                    paths.append(m.group(1) + side + m.group(4))
        elif re.search(r"\s*=>\s*", part):
            paths += [x.strip() for x in re.split(r"\s*=>\s*", part) if x.strip()]
        else:
            paths.append(part)
    for p in paths:
        for i in range(len(bundles_)):
            pre = sets_[i] if i < len(sets_) else []
            if pre:
                hit = any(p == q.rstrip("/") or p.startswith(q.rstrip("/") + "/") for q in pre if q.rstrip("/"))
            else:
                hit = assign(p, bundles_) == i
            if hit:
                out.add(i)
    return sorted(out)


for c in corpus["rowMatch"]:
    if owners(c["field"], corpus["expected"]["prefixSets"], bundles) != c["expected"]:
        failures.append("row %r" % c["field"])
for c in corpus["pathMatch"]:
    pre = c["prefixSets"][c["index"]]
    hit = any(c["path"] == q or c["path"].startswith(q + "/") for q in pre) if pre else assign(c["path"], bundles) == c["index"]
    if hit != c["expected"]:
        failures.append("path match")
''' + lib.PY_TAIL
lib.write_unit(UP, corpus_p, schema_p, ts_p, oracle_p)
