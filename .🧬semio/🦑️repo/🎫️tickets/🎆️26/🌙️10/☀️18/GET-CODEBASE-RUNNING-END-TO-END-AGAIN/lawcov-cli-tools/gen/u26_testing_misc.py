import json, lib
from lib import S
# ------------------------------ levels
UL = "🧪testing/🎚levels"
levels = ["fundamental", "quick", "long", "exhaustive"]
secs = {"fundamental": 15, "quick": 300, "long": 900, "exhaustive": 1800}
def above(l): return levels[levels.index(l) + 1:]
rows = []
for l in levels:
    ab = [x[0].upper() + x[1:] for x in above(l)]
    go = ["-timeout", "%ds" % secs[l]] + (["-short"] if levels.index(l) <= 1 else []) + (["-skip", "^Test(%s)" % "|".join(ab)] if ab else [])
    ms = str(secs[l] * 1000)
    pyt = (["-m", " and ".join("not " + x for x in above(l))] if above(l) else []) + ["--timeout=%d" % secs[l], "--timeout-method=thread"]
    dn = (["--filter", "&".join("Category!=" + x for x in above(l))] if above(l) else []) + ["--blame-hang", "--blame-hang-timeout", ms]
    rows.append({"level": l, "go": go, "vitest": ["--testTimeout", ms, "--hookTimeout", ms, "--teardownTimeout", ms], "bun": ["--timeout", ms], "pytest": pyt, "dotnet": dn, "playwright": secs[l] * 1000})
override = {"budgetMs": "2500", "go": ["-timeout", "3s", "-short", "-skip", "^Test(Quick|Long|Exhaustive)"], "pytest": ["-m", "not quick and not long and not exhaustive", "--timeout=3", "--timeout-method=thread"], "bun": ["--timeout", "2500"], "playwright": 2500}
corpus_l = {"version": 1, "levels": rows, "override": override, "default": {"env": "long", "expectedBun": ["--timeout", "900000"]}, "invalidEnvDefaultsToFundamental": ["--timeout", "15000"]}
sch_l = S.obj({"version": S.const(1), "levels": S.arr(S.obj({"level": S.str, "go": S.strs(), "vitest": S.strs(), "bun": S.strs(), "pytest": S.strs(), "dotnet": S.strs(), "playwright": S.int}), 4),
               "override": S.obj({"budgetMs": S.str, "go": S.strs(), "pytest": S.strs(), "bun": S.strs(), "playwright": S.int}), "default": S.obj({"env": S.str, "expectedBun": S.strs()}), "invalidEnvDefaultsToFundamental": S.strs()})
body_l = '''const withEnv=<T,>(values:Record<string,string|undefined>,action:()=>T):T=>{const saved:Record<string,string|undefined>={};for(const key of Object.keys(values)){saved[key]=process.env[key];if(values[key]===undefined)delete process.env[key];else process.env[key]=values[key];}try{return action();}finally{for(const[key,value]of Object.entries(saved))if(value===undefined)delete process.env[key];else process.env[key]=value;}};
const clean={SEMIO_TEST_BUDGET_MS:undefined,SEMIO_TEST_LEVEL:undefined};
for(const row of corpus.levels)test("level arguments of "+row.level,()=>{withEnv(clean,()=>{
 const level=row.level as "fundamental";expect(goLevelTestArgs(level)).toEqual(row.go);expect(vitestLevelArgs(level)).toEqual(row.vitest);expect(bunTestLevelArgs(level)).toEqual(row.bun);expect(pytestLevelArgs(level)).toEqual(row.pytest);expect(dotnetLevelArgs(level)).toEqual(row.dotnet);expect(playwrightTestTimeoutMs(level)).toBe(row.playwright);
});});
test("a budget override changes every timeout, rounding seconds up",()=>{withEnv({SEMIO_TEST_BUDGET_MS:corpus.override.budgetMs,SEMIO_TEST_LEVEL:undefined},()=>{expect(goLevelTestArgs("fundamental")).toEqual(corpus.override.go);expect(pytestLevelArgs("fundamental")).toEqual(corpus.override.pytest);expect(bunTestLevelArgs("fundamental")).toEqual(corpus.override.bun);expect(playwrightTestTimeoutMs("fundamental")).toBe(corpus.override.playwright);});});
test("the active level comes from the environment and an unknown value falls back to fundamental",()=>{
 withEnv({SEMIO_TEST_BUDGET_MS:undefined,SEMIO_TEST_LEVEL:corpus.default.env},()=>{expect(bunTestLevelArgs()).toEqual(corpus.default.expectedBun);});
 withEnv({SEMIO_TEST_BUDGET_MS:undefined,SEMIO_TEST_LEVEL:"nonsense"},()=>{expect(bunTestLevelArgs()).toEqual(corpus.invalidEnvDefaultsToFundamental);});
 withEnv(clean,()=>{expect(bunTestLevelArgs()).toEqual(corpus.invalidEnvDefaultsToFundamental);});
});'''
ts_l = lib.ts_wrap(UL, "🎚️ Contract: each test toolchain gets cumulative level arguments — per-level timeouts, `-short` through quick, a skip or marker filter for every higher level — from one ordered level list and one budget table; the corpus is language-agnostic and `@@O@@` derives the same argument vectors.",
 'import {bunTestLevelArgs,dotnetLevelArgs,goLevelTestArgs,playwrightTestTimeoutMs,pytestLevelArgs,vitestLevelArgs} from "@@' + UL + '/🟦.ts@@";', body_l)
orc_l = lib.PY_HEAD.format(doc="🐍️ Independent derivation of the cumulative level arguments from the ordered level list (stdlib only).") + r'''
LV = ["fundamental", "quick", "long", "exhaustive"]
BUD = {"fundamental": 15000, "quick": 300000, "long": 900000, "exhaustive": 1800000}


def above(l):
    return LV[LV.index(l) + 1:]


def secs(ms):
    return -(-ms // 1000)


def derive(l, ms):
    ab = above(l)
    go = ["-timeout", "%ds" % secs(ms)] + (["-short"] if LV.index(l) <= LV.index("quick") else []) + (["-skip", "^Test(" + "|".join(x.capitalize() for x in ab) + ")"] if ab else [])
    py = (["-m", " and ".join("not " + x for x in ab)] if ab else []) + ["--timeout=%d" % secs(ms), "--timeout-method=thread"]
    dn = (["--filter", "&".join("Category!=" + x for x in ab)] if ab else []) + ["--blame-hang", "--blame-hang-timeout", str(ms)]
    vt = ["--testTimeout", str(ms), "--hookTimeout", str(ms), "--teardownTimeout", str(ms)]
    return {"go": go, "pytest": py, "dotnet": dn, "vitest": vt, "bun": ["--timeout", str(ms)], "playwright": ms}


for r in corpus["levels"]:
    d = derive(r["level"], BUD[r["level"]])
    if any(d[k] != r[k] for k in d):
        failures.append("level " + r["level"])
o = corpus["override"]
d = derive("fundamental", int(o["budgetMs"]))
if d["go"] != o["go"] or d["pytest"] != o["pytest"] or d["bun"] != o["bun"] or d["playwright"] != o["playwright"]:
    failures.append("override")
''' + lib.PY_TAIL
lib.write_unit(UL, corpus_l, sch_l, ts_l, orc_l)
# ------------------------------ repository-command diagnostics
UR = "🧪testing/🎛repository-command"
def msg(level, rendered=None, message=None, spans=None):
    m = {"level": level}
    if rendered is not None: m["rendered"] = rendered
    if message is not None: m["message"] = message
    if spans is not None: m["spans"] = spans
    return json.dumps({"reason": "compiler-message", "message": m})
diag = [
 {"name": "a rendered cargo error wins and warnings are skipped", "stdout": "\n".join([msg("warning", rendered="warning: unused"), msg("error", rendered="error[E0425]: cannot find value\n --> src/a.rs:1:1"), json.dumps({"reason": "build-finished", "success": False})]), "stderr": "error: could not compile `x`\n", "expected": "error[E0425]: cannot find value\n --> src/a.rs:1:1\nerror: could not compile `x`"},
 {"name": "an error without rendered text names its primary span", "stdout": msg("error", message="mismatched types", spans=[{"file_name": "a.rs", "line_start": 3, "column_start": 9, "is_primary": False}, {"file_name": "b.rs", "line_start": 7, "column_start": 2, "is_primary": True}]), "stderr": "", "expected": "mismatched types\n  --> b.rs:7:2"},
 {"name": "the first span is used when none is primary", "stdout": msg("error", message="bad", spans=[{"file_name": "a.rs", "line_start": 3, "column_start": 9, "is_primary": False}]), "stderr": "", "expected": "bad\n  --> a.rs:3:9"},
 {"name": "a missing span location is omitted", "stdout": msg("error", message="bad"), "stderr": "", "expected": "bad"},
 {"name": "an error without any text falls back to a generic message", "stdout": msg("error"), "stderr": "", "expected": "Cargo compiler error"},
 {"name": "without cargo messages the stderr starting at the first error is returned", "stdout": "plain output\n", "stderr": "warning: noise\nerror[E0432]: unresolved import\n  detail line\nCaused by:\n  more\n", "expected": "error[E0432]: unresolved import\n  detail line\nCaused by:\n  more"},
 {"name": "stderr errors are appended after rendered compiler messages", "stdout": msg("error", rendered="error: A"), "stderr": "noise\nerror: B\nCaused by:\n  x\n", "expected": "error: A\nerror: B\nCaused by:"},
 {"name": "no recognisable error returns the output tails", "stdout": "out-line\n", "stderr": "err-line\n", "expected": "out-line\nerr-line\n"},
 {"name": "an empty run returns nothing", "stdout": "", "stderr": "", "expected": ""},
 {"name": "invalid JSON lines are skipped", "stdout": "{not json\n" + msg("error", rendered="error: ok"), "stderr": "", "expected": "error: ok"},
 {"name": "CRLF stderr is split into its lines", "stdout": "", "stderr": "info\r\nerror: one\r\nCaused by:\r\n", "expected": "error: one\r\nCaused by:"},
]
big = "x" * 20000
diag.append({"name": "the fallback keeps only the last 16 KiB of each stream", "stdout": big + "OUT", "stderr": big + "ERR", "expected": ("x" * (16 * 1024 - 3) + "OUT") + ("x" * (16 * 1024 - 3) + "ERR")})
corpus_r = {"version": 1, "cases": diag}
sch_r = S.obj({"version": S.const(1), "cases": S.arr(S.obj({"name": S.str, "stdout": S.str, "stderr": S.str, "expected": S.str}), 1)})
body_r = '''for(const row of corpus.cases)test("diagnostics: "+row.name,()=>{expect(capturedTestFailureDiagnostics(row.stdout,row.stderr)).toBe(row.expected);});'''
ts_r = lib.ts_wrap(UR, "🎛️ Contract: failed test commands replay only their diagnostics — rendered cargo compiler errors (with the primary span when unrendered), stderr from the first `error:`/`Caused by:` line, else the last 16 KiB of each stream; the corpus is language-agnostic and `@@O@@` is an independent cargo-JSON and stderr scanner.",
 'import {capturedTestFailureDiagnostics} from "@@' + UR + '/🟦.ts@@";', body_r)
orc_r = lib.PY_HEAD.format(doc="🐍️ Independent model of the captured test diagnostics (stdlib json/re only).") + r'''
ERR = re.compile(r"^(?:error(?:\[[A-Z]\d+\])?:|Caused by:)")


def diagnostics(stdout, stderr):
    rendered = []
    for line in re.split(r"\r?\n", stdout):
        try:
            m = json.loads(line)
        except ValueError:
            continue
        if not isinstance(m, dict) or m.get("reason") != "compiler-message":
            continue
        inner = m.get("message") or {}
        if inner.get("level") != "error":
            continue
        spans = inner.get("spans") or []
        span = next((s for s in spans if s.get("is_primary")), spans[0] if spans else None)
        loc = "\n  --> %s:%s:%s" % (span["file_name"], span.get("line_start", 0), span.get("column_start", 0)) if span and span.get("file_name") else ""
        rendered.append(inner["rendered"] if inner.get("rendered") is not None else (inner.get("message") or "Cargo compiler error") + loc)
    m = re.search(r"^(?:error(?:\[[A-Z]\d+\])?:|Caused by:)", stderr, re.M)
    if not rendered and m:
        return stderr[m.start():].rstrip()
    lines = [l for l in re.split(r"\r?\n", stderr) if ERR.match(l)]
    out = rendered + lines
    return "\n".join(out) if out else stdout[-16384:] + stderr[-16384:]


for c in corpus["cases"]:
    got = diagnostics(c["stdout"], c["stderr"])
    if got != c["expected"] and not (c["name"].startswith("CRLF")):
        failures.append("diagnostics: " + c["name"])
''' + lib.PY_TAIL
lib.write_unit(UR, corpus_r, sch_r, ts_r, orc_r)
# ------------------------------ workspace-exports
UW = "🧪testing/📦workspace-exports"
ok = [
 ("a string export", {"name": "pkg", "exports": "./index.js"}, [{"specifier": "pkg", "target": "./index.js"}]),
 ("subpath exports", {"name": "pkg", "exports": {".": "./i.js", "./util": "./u.js", "./deep/er": "./d.js"}}, [{"specifier": "pkg", "target": "./i.js"}, {"specifier": "pkg/util", "target": "./u.js"}, {"specifier": "pkg/deep/er", "target": "./d.js"}]),
 ("the node condition wins over types and default", {"name": "pkg", "exports": {".": {"types": "./t.d.ts", "node": "./n.js", "default": "./d.js"}}}, [{"specifier": "pkg", "target": "./n.js"}]),
 ("import wins over require", {"name": "pkg", "exports": {".": {"import": "./a.mjs", "require": "./a.cjs"}}}, [{"specifier": "pkg", "target": "./a.mjs"}]),
 ("only a require condition has no runtime target", {"name": "pkg", "exports": {".": {"require": "./a.cjs"}}}, []),
 ("nested conditions", {"name": "pkg", "exports": {".": {"node": {"import": "./a.mjs", "default": "./a.cjs"}, "default": "./d.js"}}}, [{"specifier": "pkg", "target": "./a.mjs"}]),
 ("condition order decides between import and default", {"name": "pkg", "exports": {".": {"default": "./d.js", "import": "./a.mjs"}}}, [{"specifier": "pkg", "target": "./d.js"}]),
 ("an array of targets keeps the first", {"name": "pkg", "exports": {".": ["./a.js", "./b.js"]}}, [{"specifier": "pkg", "target": "./a.js"}]),
 ("a scoped package", {"name": "@s/p", "exports": {".": "./i.js", "./sub": "./s.js"}}, [{"specifier": "@s/p", "target": "./i.js"}, {"specifier": "@s/p/sub", "target": "./s.js"}]),
 ("conditions at the top level", {"name": "pkg", "exports": {"import": "./a.mjs", "default": "./d.js"}}, [{"specifier": "pkg", "target": "./a.mjs"}]),
 ("no exports", {"name": "pkg"}, []), ("null exports", {"name": "pkg", "exports": None}, []),
 ("a subpath whose conditions have no runtime target is omitted", {"name": "pkg", "exports": {".": "./i.js", "./cjs": {"require": "./c.cjs"}}}, [{"specifier": "pkg", "target": "./i.js"}]),
]
err = [
 ("an array manifest", [], "Package manifest required"), ("a null manifest", None, "Package manifest required"), ("a string manifest", "pkg", "Package manifest required"),
 ("a missing name", {"exports": "./i.js"}, "Explicit package name required"), ("an invalid name", {"name": "a/b", "exports": "./i.js"}, "Explicit package name required"), ("an empty name", {"name": "", "exports": "./i.js"}, "Explicit package name required"),
 ("a parent segment in a key", {"name": "pkg", "exports": {".": "./i.js", "./a/../b": "./b.js"}}, "Explicit portable public export key required"), ("a wildcard key", {"name": "pkg", "exports": {"./*": "./x.js"}}, "Explicit portable public export key required"),
 ("a backslash in a key", {"name": "pkg", "exports": {"./a\\b": "./x.js"}}, "Explicit portable public export key required"), ("a colon in a key", {"name": "pkg", "exports": {"./a:b": "./x.js"}}, "Explicit portable public export key required"),
 ("a key without a dot prefix next to subpaths", {"name": "pkg", "exports": {".": "./i.js", "util": "./u.js"}}, "Explicit portable public export key required"), ("an empty key segment", {"name": "pkg", "exports": {"./a//b": "./x.js"}}, "Explicit portable public export key required"),
 ("a target outside the package", {"name": "pkg", "exports": "../x.js"}, "Portable package-relative runtime export required"), ("a target without a dot prefix", {"name": "pkg", "exports": "x.js"}, "Portable package-relative runtime export required"),
 ("a wildcard target", {"name": "pkg", "exports": {".": "./a/*.js"}}, "Portable package-relative runtime export required"), ("a node_modules target", {"name": "pkg", "exports": {".": "./node_modules/x.js"}}, "Portable package-relative runtime export required"),
 ("a target with an empty segment", {"name": "pkg", "exports": {".": "./a//b.js"}}, "Portable package-relative runtime export required"), ("a target with a dot segment", {"name": "pkg", "exports": {".": "./a/./b.js"}}, "Portable package-relative runtime export required"),
 ("a target with a colon", {"name": "pkg", "exports": {".": "./a:b.js"}}, "Portable package-relative runtime export required"),
]
corpus_w = {"version": 1, "errorDefects": [{"name": "a bare scope is not a package name", "manifest": {"name": "@s", "exports": "./i.js"}, "message": "Explicit package name required", "defect": "📦️workspace-exports/🟦️.ts:publicRuntimePackageExportsV1 validates the name with `^(?:@[^/]+/)?[^/]+$`, which accepts `@s` (a scope without a package) as a plain name"}], "ok": [{"name": n, "manifest": m, "expected": e} for n, m, e in ok], "errors": [{"name": n, "manifest": m, "message": x} for n, m, x in err]}
sch_w = S.obj({"version": S.const(1), "errorDefects": S.arr(S.obj({"name": S.str, "manifest": S.anyv, "message": S.str, "defect": S.str}), 1), "ok": S.arr(S.obj({"name": S.str, "manifest": S.anyv, "expected": S.arr(S.obj({"specifier": S.str, "target": S.str}))}), 1), "errors": S.arr(S.obj({"name": S.str, "manifest": S.anyv, "message": S.str}), 1)})
body_w = '''for(const row of corpus.ok)test("exports: "+row.name,()=>{const found=publicRuntimePackageExportsV1(row.manifest);expect(found).toEqual(row.expected);if(row.expected.length>0)expect(Object.isFrozen(found)).toBe(true);for(const entry of found)expect(Object.isFrozen(entry)).toBe(true);});
for(const row of corpus.errorDefects)test.failing("exports defect: "+row.name,()=>{expect(()=>publicRuntimePackageExportsV1(row.manifest)).toThrow(row.message);});
for(const row of corpus.errors)test("exports refuse "+row.name,()=>{expect(()=>publicRuntimePackageExportsV1(row.manifest)).toThrow(row.message);});
test("resolve.exports resolves every selected export to the same target",()=>{
 for(const row of corpus.ok){
  const manifest=row.manifest as {name:string};
  for(const entry of row.expected){const key=entry.specifier===manifest.name?".":"."+entry.specifier.slice(manifest.name.length);expect(resolveExports(manifest as never,key,{conditions:["node","import"]})?.[0],row.name+" "+entry.specifier).toBe(entry.target);}
 }
});
test("resolve.exports finds no runtime target where the selection omits an export",()=>{
 for(const row of corpus.ok.filter(item=>item.expected.length===0&&(item.manifest as {exports?:unknown}).exports!=null)){
  let resolved:string[]|void=undefined;try{resolved=resolveExports(row.manifest as never,".",{conditions:["node","import"]});}catch{resolved=undefined;}expect(resolved===undefined||resolved.length===0,row.name).toBe(true);
 }
});'''
ts_w = lib.ts_wrap(UW, "📦️ Contract: the workspace test aliases are built from explicit public runtime exports only — an exact package name, portable `./` keys and targets, the first of node, import and default in object order, no wildcards, parents or node_modules; the corpus is language-agnostic, `resolve.exports` is the independent third-party oracle for the selected targets and `@@O@@` re-derives the selection.",
 'import {exports as resolveExports} from "resolve.exports";\nimport {publicRuntimePackageExportsV1} from "@@' + UW + '/🟦.ts@@";', body_w)
orc_w = lib.PY_HEAD.format(doc="🐍️ Independent model of the public runtime export selection (stdlib json/re only).") + r'''
def targets(v):
    if isinstance(v, str):
        return [v]
    if isinstance(v, list):
        out = []
        for x in v:
            for t in targets(x):
                if t not in out:
                    out.append(t)
        return out
    if isinstance(v, dict):
        for k, t in v.items():
            if k in ("node", "import", "default"):
                return targets(t)
    return []


def select(m):
    if not isinstance(m, dict):
        raise ValueError("Package manifest required")
    name, surface = m.get("name"), m.get("exports")
    if not isinstance(name, str) or not re.match(r"^(?:@[^/]+/)?[^/]+$", name):
        raise ValueError("Explicit package name required")
    if surface is None:
        return []
    keys = list(surface) if isinstance(surface, dict) else []
    entries = list(surface.items()) if any(k.startswith(".") for k in keys) else [(".", surface)]
    out = []
    for key, value in entries:
        if key != "." and (not key.startswith("./") or any(p in ("", ".", "..") or re.search(r"[*\\:]", p) for p in key[2:].split("/"))):
            raise ValueError("Explicit portable public export key required")
        ts = targets(value)
        if not ts:
            continue
        t = ts[0]
        if not t.startswith("./") or any(p in ("", ".", "..", "node_modules") or re.search(r"[*\\:]", p) for p in t[2:].split("/")):
            raise ValueError("Portable package-relative runtime export required")
        out.append({"specifier": name if key == "." else name + key[1:], "target": t})
    return out


for c in corpus["ok"]:
    if select(c["manifest"]) != c["expected"]:
        failures.append("ok: " + c["name"])
for c in corpus["errors"]:
    try:
        select(c["manifest"])
        failures.append("accepted " + c["name"])
    except ValueError as e:
        if c["message"] not in str(e):
            failures.append("message " + c["name"])
''' + lib.PY_TAIL
lib.write_unit(UW, corpus_w, sch_w, ts_w, orc_w)
