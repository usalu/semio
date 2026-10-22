import json, lib
from lib import S
V = "\ufe0f"
# ---------------------------- lcov
UL = "🧪testing/🗄lcov"
lcov_a = "TN:\nSF:src/a.ts\nFN:1,f\nDA:1,1\nDA:2,0\nDA:2,3\nLF:2\nLH:2\nend_of_record\nSF:src/b.ts\nDA:10,0\nend_of_record\n"
lcov_b = "SF:src/a.ts\r\nDA:1,4\r\nDA:5,1\r\nend_of_record\r\nDA:99,9\r\nSF:src/c.ts\r\n  DA:7,2  \r\nend_of_record\r\n"
parse_cases = [
 {"name": "records, repeated lines add up and bookkeeping is ignored", "text": lcov_a, "expected": [{"path": "src/a.ts", "lines": [[1, 1], [2, 3]]}, {"path": "src/b.ts", "lines": [[10, 0]]}]},
 {"name": "CRLF, stray lines outside a record and indentation", "text": lcov_b, "expected": [{"path": "src/a.ts", "lines": [[1, 4], [5, 1]]}, {"path": "src/c.ts", "lines": [[7, 2]]}]},
 {"name": "an empty text", "text": "", "expected": []},
 {"name": "a record without end_of_record keeps absorbing lines", "text": "SF:x\nDA:1,1\nSF:y\nDA:2,2\n", "expected": [{"path": "x", "lines": [[1, 1]]}, {"path": "y", "lines": [[2, 2]]}]},
 {"name": "a DA line with a malformed count counts NaN", "text": "SF:x\nDA:3,\nend_of_record\n", "expected": [{"path": "x", "lines": [[3, 0]]}]},
]
parse_cases[4]["expected"] = [{"path": "x", "lines": [[3, 0]]}]
merge = {"sets": [[{"path": "a", "lines": [[1, 1], [2, 0]]}, {"path": "b", "lines": [[5, 2]]}], [{"path": "a", "lines": [[2, 4], [3, 1]]}], []], "expected": [["a", [[1, 1], [2, 4], [3, 1]]], ["b", [[5, 2]]]]}
render = {"merged": [["z.ts", [[3, 1], [1, 0], [2, 7]]], ["a.ts", [[10, 2]]]], "expected": "SF:z.ts\nDA:1,0\nDA:2,7\nDA:3,1\nend_of_record\n\nSF:a.ts\nDA:10,2\nend_of_record\n", "empty": ""}
go = [
 {"name": "a statement range marks each of its lines", "text": "mode: atomic\nfile.go:3.10,5.2 2 1\nfile.go:10.1,10.9 1 0\nother.go:1.1,2.5 1 4\n", "expected": [{"path": "file.go", "lines": [[3, 1], [4, 1], [5, 1], [10, 0]]}, {"path": "other.go", "lines": [[1, 4], [2, 4]]}]},
 {"name": "overlapping ranges add their counts", "text": "mode: set\nf.go:1.1,2.1 1 1\nf.go:2.1,3.1 1 2\n", "expected": [{"path": "f.go", "lines": [[1, 1], [2, 3], [3, 2]]}]},
 {"name": "blank, header and malformed lines are skipped", "text": "mode: count\n\nnot a record\nf.go:1.1,1.5 1 x\n  f.go:2.1,2.5 1 3  \n", "expected": [{"path": "f.go", "lines": [[2, 3]]}]},
 {"name": "a path with a colon", "text": "mode: set\nC:/a/b.go:7.1,8.1 1 1\n", "expected": [{"path": "C:/a/b.go", "lines": [[7, 1], [8, 1]]}]},
 {"name": "an empty profile", "text": "", "expected": []},
]
corpus_l = {"version": 1, "parse": parse_cases, "merge": merge, "render": render, "go": go}
rec = S.obj({"path": S.str, "lines": S.arr(S.arr(S.int, 2))})
schema_l = S.obj({"version": S.const(1), "parse": S.arr(S.obj({"name": S.str, "text": S.str, "expected": S.arr(rec)}), 1), "merge": S.obj({"sets": S.arr(S.arr(rec)), "expected": S.arr(S.anyv)}), "render": S.obj({"merged": S.arr(S.anyv), "expected": S.str, "empty": S.str}), "go": S.arr(S.obj({"name": S.str, "text": S.str, "expected": S.arr(rec)}), 1)})
body_l = '''const flat=(records:{path:string;lines:Map<number,number>}[])=>records.map(record=>({path:record.path,lines:[...record.lines.entries()]}));
const asMap=(rows:[string,[number,number][]][]):Map<string,Map<number,number>>=>new Map(rows.map(([path,lines])=>[path,new Map(lines)]));
const fromRows=(rows:{path:string;lines:number[][]}[])=>rows.map(row=>({path:row.path,lines:new Map(row.lines.map(pair=>[pair[0]!,pair[1]!] as [number,number]))}));
for(const row of corpus.parse)test("parse: "+row.name,()=>{expect(flat(parseLcov(row.text))).toEqual(row.expected);});
test("merging sums the hits of the same line across runs",()=>{const merged=mergeLcov(corpus.merge.sets.map(fromRows));expect([...merged.entries()].map(([path,lines])=>[path,[...lines.entries()]])).toEqual(corpus.merge.expected);expect(mergeLcov([]).size).toBe(0);});
test("rendering sorts lines and keeps file order",()=>{expect(renderLcov(asMap(corpus.render.merged as [string,[number,number][]][]))).toBe(corpus.render.expected);expect(renderLcov(new Map())).toBe(corpus.render.empty);});
test("a rendered report parses back to the merged map",()=>{const merged=mergeLcov(corpus.merge.sets.map(fromRows));expect(mergeLcov([parseLcov(renderLcov(merged))])).toEqual(merged);});
for(const row of corpus.go)test("go profile: "+row.name,()=>{expect(flat(goProfileToLcov(row.text))).toEqual(row.expected);});'''
ts_l = lib.ts_wrap(UL, "🗄️ Contract: LCOV text is parsed into per-file line hits (only `SF:`, `DA:` and `end_of_record`), merged by summing hits, rendered with sorted lines, and Go cover profiles expand each statement range into per-line hits; the corpus is language-agnostic and `@@O@@` is an independent LCOV/Go-profile model.",
 'import {goProfileToLcov,mergeLcov,parseLcov,renderLcov} from "@@' + UL + '/🟦.ts@@";', body_l)
orc_l = lib.PY_HEAD.format(doc="🐍️ Independent model of LCOV parsing, merging, rendering and Go cover-profile expansion (stdlib only).") + r'''
def parse(text):
    recs, cur = [], None
    for raw in re.split(r"\r?\n", text):
        line = raw.strip()
        if line.startswith("SF:"):
            cur = {"path": line[3:], "lines": {}}
            recs.append(cur)
        elif line.startswith("DA:") and cur is not None:
            a, _, b = line[3:].partition(",")
            n, h = int(a), int(b) if b.strip().lstrip("-").isdigit() else 0
            cur["lines"][n] = cur["lines"].get(n, 0) + h
        elif line == "end_of_record":
            cur = None
    return [{"path": r["path"], "lines": [[k, v] for k, v in r["lines"].items()]} for r in recs]


for c in corpus["parse"]:
    if parse(c["text"]) != c["expected"]:
        failures.append("parse: " + c["name"])
merged = {}
for s in corpus["merge"]["sets"]:
    for r in s:
        t = merged.setdefault(r["path"], {})
        for n, h in r["lines"]:
            t[n] = t.get(n, 0) + h
if [[p, [[n, h] for n, h in l.items()]] for p, l in merged.items()] != corpus["merge"]["expected"]:
    failures.append("merge")
out = []
for p, l in corpus["render"]["merged"]:
    out.append("SF:" + p)
    for n, h in sorted(l):
        out.append("DA:%d,%d" % (n, h))
    out += ["end_of_record", ""]
if "\n".join(out) != corpus["render"]["expected"]:
    failures.append("render")
for c in corpus["go"]:
    files = {}
    for raw in re.split(r"\r?\n", c["text"]):
        line = raw.strip()
        if not line or line.startswith("mode:"):
            continue
        m = re.match(r"^(.+):(\d+)\.\d+,(\d+)\.\d+ \d+ (\d+)$", line)
        if not m:
            continue
        f, a, b, h = m.group(1), int(m.group(2)), int(m.group(3)), int(m.group(4))
        d = files.setdefault(f, {})
        for n in range(a, b + 1):
            d[n] = d.get(n, 0) + h
    got = [{"path": f, "lines": [[n, h] for n, h in d.items()]} for f, d in files.items()]
    if got != c["expected"]:
        failures.append("go: " + c["name"])
''' + lib.PY_TAIL
lib.write_unit(UL, corpus_l, schema_l, ts_l, orc_l)
# ---------------------------- coverage
UC = "🧪testing/📊coverage"
excl_true = ["a/generated/x.ts", "framework/module/asset/metabolism/icon/generated/t.rs", "pkg/foo/pkg/x.js", "ui/Button.stories.tsx", "ui/Button.stories.ts", "x/y.spec.ts", "x/y.test.ts", ".storybook/main.ts", "docs/m.tex", "gpu/s.wgsl", "img/i.svg", "elements/client/lib/geometry/topologic/a.cpp",
             "framework/product/os/module/renderer/wgpu/x.rs", "framework/module/ui/wgpu/y.rs", "app/dist/o.js", "app/node_modules/p/i.js", "crate/target/debug/x.rs", ".\\a\\generated\\x.ts", "./a/generated/x.ts", "a\\b\\c.test.ts"]
excl_false = ["app/.storybook/main.ts", "src/a.ts", "src/generatedness/x.ts", "src/pkgs/x.ts", "src/stories/x.ts", "src/a.test.tsx", "src/a.tsx", "framework/module/ui/wgpuish/x.rs", "framework/module/ui/wgpu.rs", "src/target.rs", "docs/m.texx", "src/i.svg.ts", "elements/client/lib/geometry/x.cpp"]
excl_defects = ["generated/x.ts", "dist/x.js", "target/debug/x.rs", "node_modules/p/i.js", "x.test.ts", "y.spec.ts", "Button.stories.ts", "s.wgsl", "pkg/x.js"]
merged = [["src/a.ts", [[1, 1], [2, 0], [3, 2]]], ["src/b.ts", [[1, 0], [2, 0]]], ["x/generated/g.ts", [[1, 1]]], ["src/empty.ts", []], ["src/z.ts", [[1, 1]]]]
summary = {"linesFound": 6, "linesHit": 3, "pct": 50.0, "perFile": [{"path": "src/b.ts", "linesFound": 2, "linesHit": 0, "pct": 0.0}, {"path": "src/a.ts", "linesFound": 3, "linesHit": 2, "pct": 66.66666666666666}, {"path": "src/z.ts", "linesFound": 1, "linesHit": 1, "pct": 100.0}, {"path": "src/empty.ts", "linesFound": 0, "linesHit": 0, "pct": 100.0}]}
summary["perFile"] = sorted(summary["perFile"], key=lambda r: (r["pct"], -r["linesFound"]))
corpus_c = {"version": 1, "excluded": [{"path": p, "expected": True} for p in excl_true] + [{"path": p, "expected": False} for p in excl_false], "excludedDefects": [{"path": p, "expected": True, "defect": "📊️coverage/🟦️.ts:coverageGlobToRegExp turns `**/x/**` into `.*/x/.*`, which needs a leading path segment: an entry at the repository root (`generated/…`, `dist/…`, `target/…`, `x.test.ts`) is not excluded although `**/` also matches zero directories"} for p in excl_defects],
            "reasonKeys": True, "summary": {"merged": merged, "expected": summary}, "threshold": {"pct": 50.0, "passing": 50.0, "failing": 80.0, "worst": 2}, "slug": [["my module/1", "my_module_1"], ["ok-name_1", "ok-name_1"], ["a..b", "a_b"], ["", ""]]}
schema_c = S.obj({"version": S.const(1), "excluded": S.arr(S.obj({"path": S.str, "expected": S.bool}), 1), "excludedDefects": S.arr(S.obj({"path": S.str, "expected": S.bool, "defect": S.str}), 1), "reasonKeys": S.bool,
                  "summary": S.obj({"merged": S.arr(S.anyv), "expected": S.obj({"linesFound": S.int, "linesHit": S.int, "pct": S.num, "perFile": S.arr(S.obj({"path": S.str, "linesFound": S.int, "linesHit": S.int, "pct": S.num}))})}),
                  "threshold": S.obj({"pct": S.num, "passing": S.num, "failing": S.num, "worst": S.int}), "slug": S.arr(S.arr(S.str, 2))})
body_c = '''const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const withCoverage=<T,>(value:string|undefined,action:()=>T):T=>{const saved=process.env.SEMIO_COVERAGE;if(value===undefined)delete process.env.SEMIO_COVERAGE;else process.env.SEMIO_COVERAGE=value;try{return action();}finally{if(saved===undefined)delete process.env.SEMIO_COVERAGE;else process.env.SEMIO_COVERAGE=saved;}};
class Exit extends Error{constructor(readonly code:number){super("exit "+code);}}
const captureExit=(action:()=>void):{lines:string[];errors:string[];code:number|null}=>{
 const lines:string[]=[],errors:string[]=[],log=console.log,error=console.error,exit=process.exit;let code:number|null=null;
 console.log=(...parts:unknown[])=>{lines.push(parts.join(" "));};console.error=(...parts:unknown[])=>{errors.push(parts.join(" "));};(process as unknown as {exit:(c?:number)=>never}).exit=(value?:number)=>{throw new Exit(value??0);};
 try{action();}catch(thrown){if(thrown instanceof Exit)code=thrown.code;else throw thrown;}finally{console.log=log;console.error=error;process.exit=exit;}
 return{lines,errors,code};
};
const merged=():Map<string,Map<number,number>>=>new Map((corpus.summary.merged as [string,[number,number][]][]).map(([path,lines])=>[path,new Map(lines)]));
const globModel=(path:string,glob:string):boolean=>minimatch(path,glob,{dot:true});
for(const row of corpus.excluded)test("excluded "+JSON.stringify(row.path),()=>{expect(isCoverageExcluded(row.path)).toBe(row.expected);});
for(const row of corpus.excludedDefects)test.failing("excluded defect "+JSON.stringify(row.path),()=>{expect(isCoverageExcluded(row.path)).toBe(row.expected);});
test("minimatch agrees on every exclusion the corpus expects except the root-level ones the generator misses",()=>{
 for(const row of corpus.excluded.filter(item=>!item.path.includes("\\\\")&&!item.path.startsWith("./")))expect(COVERAGE_EXCLUDE_GLOBS.some(glob=>globModel(row.path,glob)),row.path).toBe(row.expected);
 for(const row of corpus.excludedDefects)expect(COVERAGE_EXCLUDE_GLOBS.some(glob=>globModel(row.path,glob)),row.path).toBe(true);
});
test("every exclusion has an auditable reason",()=>{expect(Object.keys(COVERAGE_EXCLUDE_REASONS).sort()).toEqual([...COVERAGE_EXCLUDE_GLOBS].sort());for(const reason of Object.values(COVERAGE_EXCLUDE_REASONS))expect(reason.length).toBeGreaterThan(8);});
test("the summary drops excluded files, sorts worst first and treats an empty file as fully covered",()=>{expect(summarizeCoverage(merged())).toEqual(corpus.summary.expected);expect(summarizeCoverage(new Map())).toEqual({linesFound:0,linesHit:0,pct:0,perFile:[]});});
test("the threshold only fails below the percentage and lists the worst files",()=>{
 const summary=summarizeCoverage(merged()),ok=captureExit(()=>enforceCoverageThreshold(summary,corpus.threshold.passing));expect(ok.code).toBeNull();expect(ok.lines[0]).toContain("3/6 lines (50.00%), threshold 50%.");
 const bad=captureExit(()=>enforceCoverageThreshold(summary,corpus.threshold.failing,corpus.threshold.worst));expect(bad.code).toBe(1);expect(bad.lines.some(line=>line.includes("worst 2 files:"))).toBe(true);expect(bad.lines.filter(line=>line.startsWith("  ")).length).toBe(2);expect(bad.lines.filter(line=>line.startsWith("  "))[0]).toContain("src/b.ts");expect(bad.errors[0]).toContain("50.00% < 80% — exhaustive gate failed.");
});
test("coverage arguments exist only while coverage is enabled",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"coverage-")));roots.push(root);
 expect(withCoverage(undefined,()=>coverageEnabled())).toBe(false);expect(withCoverage("0",()=>coverageEnabled())).toBe(false);expect(withCoverage("1",()=>coverageEnabled())).toBe(true);
 withCoverage("0",()=>{expect(goCoverageArgs(root,"m")).toEqual([]);expect(pytestCoverageArgs(root,"m")).toEqual([]);expect(dotnetCoverageArgs(root,"m")).toEqual([]);});
 withCoverage("1",()=>{
  const go=goCoverageArgs(root,"my mod/1");expect(go.slice(0,2)).toEqual(["-covermode=atomic","-coverpkg=./..."]);expect(go[2]!.startsWith("-coverprofile=")).toBe(true);expect(go[2]!.endsWith("my_mod_1.cover")).toBe(true);
  const py=pytestCoverageArgs(root,"my mod/1");expect(py[0]).toBe("--cov");expect(py[1]!.startsWith("--cov-report=lcov:")).toBe(true);expect(py[1]!.endsWith("my_mod_1.lcov")).toBe(true);
  const dn=dotnetCoverageArgs(root,"my mod/1");expect(dn.slice(0,2)).toEqual(["--collect","XPlat Code Coverage;Format=lcov"]);expect(dn[2]).toBe("--results-directory");expect(dn[3]!.endsWith("my_mod_1")).toBe(true);expect(existsSync(dn[3]!)).toBe(true);
 });
});'''
ts_c = lib.ts_wrap(UC, "📊️ Contract: repository coverage excludes generated, vendored, GPU and test paths by glob, reduces a merged line map to a percentage with the worst files first, fails below the threshold and only builds toolchain coverage arguments while enabled; the corpus is language-agnostic and minimatch is the independent third-party oracle for the exclusion globs.",
 'import {afterAll} from "bun:test";\nimport {minimatch} from "minimatch";\nimport {existsSync,mkdtempSync,realpathSync,rmSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {join} from "node:path";\nimport {COVERAGE_EXCLUDE_GLOBS,COVERAGE_EXCLUDE_REASONS,coverageEnabled,dotnetCoverageArgs,enforceCoverageThreshold,goCoverageArgs,isCoverageExcluded,pytestCoverageArgs,summarizeCoverage} from "@@' + UC + '/🟦.ts@@";', body_c, python=False)
lib.write_unit(UC, corpus_c, schema_c, ts_c)
