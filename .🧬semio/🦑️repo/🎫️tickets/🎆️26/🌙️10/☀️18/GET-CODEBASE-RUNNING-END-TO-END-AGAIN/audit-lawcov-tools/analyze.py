#!/usr/bin/env python3
# 🔎 Read-only coverage audit of the build library: units, schema, tests, corpus, oracle.
import subprocess, re, os, sys, json, collections, datetime

ROOT = "/Users/ueli/Documents/semio"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
TICKET = sys.argv[1]
SCRATCH = sys.argv[2]
REPORT = os.path.join(TICKET, "audit-lawcov.md")


def N(s):
    return s.replace("️", "")


raw = subprocess.run(["git", "-c", "core.quotepath=off", "ls-files", "-z", "--", LIB],
                     cwd=ROOT, capture_output=True, check=True).stdout.decode("utf8").split("\0")
raw = [f for f in raw if f]
rel_abs = {N(f[len(LIB) + 1:]): os.path.join(ROOT, f) for f in raw}
files = sorted(rel_abs)
fileset = set(files)

PROD = {"🟦.ts", "🟨.mjs", "🦀.rs", "🐹.go", "🐍.py"}
EXCL = {"🧪tests", "🧫fixtures"}
TESTEXT = (".ts", ".mjs", ".mts", ".cjs", ".cts", ".js", ".rs", ".go", ".py")
DATAEXT = (".json", ".feature", ".yaml", ".yml", ".toml", ".csv")
RUNNERS = {"vitest", "@jest/globals", "jest", "mocha", "chai", "@playwright/test"}
SQLITE = {"bun:sqlite", "node:sqlite", "sqlite3", "better-sqlite3", "sql.js"}
BUILTIN = {"fs", "path", "crypto", "child_process", "os", "url", "assert", "util", "events",
           "module", "vm", "stream", "worker_threads", "http", "net", "zlib", "readline",
           "timers", "async_hooks", "tty", "process", "buffer", "string_decoder", "https", "dns"}

prod = [r for r in files if r.split("/")[-1] in PROD and not (EXCL & set(r.split("/")[:-1]))]
units = collections.defaultdict(list)
for r in prod:
    units[os.path.dirname(r)].append(r)

testdirs = set()
for r in files:
    parts = r.split("/")
    for i, p in enumerate(parts[:-1]):
        if p == "🧪tests":
            testdirs.add("/".join(parts[:i]))


def nearest_scope(U):
    parts = U.split("/") if U else []
    for i in range(len(parts), 0, -1):
        P = "/".join(parts[:i])
        if P in testdirs:
            return P
    return "" if "" in testdirs else None


def read_text(r):
    try:
        with open(rel_abs[r], "rb") as fh:
            return N(fh.read().decode("utf8", "replace"))
    except OSError:
        return ""


IMP = re.compile(
    r"""(?:^|[\s;])(?:import|export)\b[^'"`;]*?\bfrom\s*['"]([^'"]+)['"]"""
    r"""|(?:^|[\s;])import\s*['"]([^'"]+)['"]"""
    r"""|\bimport\(\s*['"]([^'"]+)['"]\s*\)"""
    r"""|\brequire\(\s*['"]([^'"]+)['"]\s*\)""")
BASENAME = re.compile(r"""([^\s'"`/\\()\[\],]+\.(?:json|feature|yaml|yml|toml|csv))""")


def is_schema_like(base):
    return "schema" in base or base.startswith("🧬") or base in {"package.json", "project.json", "tsconfig.json"}


def third_party(spec):
    if spec.startswith((".", "/", "data:")) or "${" in spec:
        return None
    if spec.startswith("node:"):
        return "sqlite" if spec == "node:sqlite" else None
    if spec.startswith("bun:"):
        return "sqlite" if spec == "bun:sqlite" else None
    if spec.startswith("@semio-tech/"):
        return None
    if spec in RUNNERS or spec.split("/")[0] in RUNNERS:
        return None
    if spec in BUILTIN:
        return None
    if spec in SQLITE:
        return "sqlite"
    return spec


evcache = {}


def evidence(S):
    if S in evcache:
        return evcache[S]
    pref = (S + "/" if S else "") + "🧪tests/"
    tfiles = [r for r in files if r.startswith(pref) and r.endswith(TESTEXT)]
    corpus = [r for r in files if r.startswith(pref) and r.endswith(DATAEXT)
              and not is_schema_like(N(r.split("/")[-1]))]
    refs, scan_dirs, py_oracle, libs, sqlite = set(), [], 0, collections.Counter(), 0
    for t in tfiles:
        txt = read_text(t)
        refs.update(N(b) for b in BASENAME.findall(txt))
        if re.search(r"readdir|glob", txt):
            scan_dirs.append(os.path.dirname(t))
        if t.endswith(".py"):
            if "oracle" in t.split("/")[-1]:
                py_oracle += 1
            continue
        if t.endswith(".rs") or t.endswith(".go"):
            continue
        for m in IMP.finditer(txt):
            spec = next(g for g in m.groups() if g)
            lib = third_party(spec)
            if lib == "sqlite":
                sqlite += 1
            elif lib:
                libs[lib.split("/")[0] if not lib.startswith("@") else "/".join(lib.split("/")[:2])] += 1
    consumed = []
    for c in corpus:
        base = N(c.split("/")[-1])
        cdir = os.path.dirname(c)
        ok = base in refs or any(cdir == d or cdir.startswith(d + "/") for d in scan_dirs)
        consumed.append(ok)
    ev = {
        "tests": len(tfiles),
        "corpus": len(corpus),
        "consumed": sum(consumed),
        "py_oracle": py_oracle,
        "third_party": sum(libs.values()),
        "sqlite": sqlite,
        "libs": libs,
        "oracle": bool(py_oracle or sum(libs.values()) or sqlite),
    }
    evcache[S] = ev
    return ev


def lines_of(r):
    with open(rel_abs[r], "rb") as fh:
        return len(fh.read().splitlines())


today = datetime.date(2026, 10, 21)
NAMED = {
    "🏗build/🕸graph": "11:59 generationFilesV1 (graph/🟦)",
    "🏗build/📥inputs/💾storage": "12:28 storage/🟨 retireOptionalGraphHeadroomV1",
    "⚡caching/💾storage/🔒enforcement/📦budget/🕸graph/📏headroom/🧾closures": "14:47-16:15 closures read.mjs, closures schema, maximumExpandedBytes",
    "🏗build/🕸graph/🗂generation-files": "15:xx-16:14 storage-seam generation files moved",
    "🏗build/🧹cache-gc/🏃runner": "14:19 nativeImageDirectoryHostV1 removed (cache-gc)",
}
root_text = []
for r in files:
    if r.startswith("🧪tests/") and r.endswith(TESTEXT) and "🧫fixtures" not in r.split("/"):
        root_text.append(read_text(r))


def root_named(U):
    name = U.split("/")[-1] if U else ""
    return bool(name) and any(name in t for t in root_text)


rows = []
for U in sorted(units):
    S = nearest_scope(U)
    kind = "none" if S is None else ("root" if S == "" else ("own" if S == U else "ancestor"))
    own_ev = evidence(U) if U in testdirs else None
    if kind in ("own", "ancestor"):
        ev = evidence(S)
    else:
        ev = {"tests": 0, "corpus": 0, "consumed": 0, "py_oracle": 0, "third_party": 0,
              "sqlite": 0, "libs": {}, "oracle": False}
    lines = sum(lines_of(r) for r in units[U])
    schema = any(r.startswith((U + "/" if U else "") + "🧬schema/") and r.endswith(".json") for r in files)
    mt = [r for r in units[U] if datetime.date.fromtimestamp(os.path.getmtime(rel_abs[r])) == today]
    named = [NAMED[U]] if U in NAMED else []
    rn = root_named(U) if kind == "root" else False
    if ev["tests"] == 0:
        cls = "1-NOTEST"
    elif ev["consumed"] == 0:
        cls = "2-NOCORPUS"
    elif not ev["oracle"]:
        cls = "3-NOORACLE"
    else:
        cls = "4-OK"
    rows.append({
        "unit": U or "(library root)", "cls": cls, "lines": lines, "files": len(units[U]),
        "scope": kind, "scope_dir": S, "schema": schema,
        "tests": ev["tests"], "corpus": ev["corpus"], "consumed": ev["consumed"],
        "oracle": ev["oracle"], "py_oracle": ev["py_oracle"], "third_party": ev["third_party"],
        "sqlite": ev["sqlite"], "own_tests": bool(own_ev and own_ev["tests"]),
        "own_corpus": bool(own_ev and own_ev["consumed"]), "own_oracle": bool(own_ev and own_ev["oracle"]),
        "mtime_0921": len(mt), "named": named, "root_named": rn,
    })

order = {"1-NOTEST": 0, "2-NOCORPUS": 1, "3-NOORACLE": 2, "4-OK": 3}
rows.sort(key=lambda x: (order[x["cls"]], -x["lines"], x["unit"]))
json.dump(rows, open(os.path.join(SCRATCH, "units.json"), "w"), ensure_ascii=False, indent=1)

# totals
def agg(filt):
    sel = [x for x in rows if filt(x)]
    return len(sel), sum(x["lines"] for x in sel)

tot_units, tot_lines = agg(lambda x: True)
libs_total = collections.Counter()
for S in {r["scope_dir"] for r in rows if r["scope_dir"] is not None and r["scope"] != "root"}:
    libs_total.update(evidence(S)["libs"])
named_rows = [x for x in rows if x["named"]]
mt_rows = [x for x in rows if x["mtime_0921"]]

out = []
w = out.append
w("# Audit LAWCOV: language-agnostic test, corpus and oracle coverage of the build library")
w("")
w("Ticket: GET-CODEBASE-RUNNING-END-TO-END-AGAIN (C). Auditor: audit-lawcov (read-only). Generated by `audit-lawcov-tools/analyze.py`.")
w("")
w("Scope: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `git ls-files` only (3627 files). Feature unit = directory with direct production files `🟦.ts`, `🟨.mjs`, `🦀.rs`, `🐹.go`, `🐍.py`, excluding any path under `🧪tests` or `🧫fixtures`.")
w("")
w("Definitions (heuristics, see caveats):")
w("- **Schema**: `<unit>/🧬schema/*.json` exists (schema-first).")
w("- **Scope**: nearest unit-or-ancestor directory that has a `🧪tests` child; `own` = unit's own `🧪tests`, `ancestor` = inherited from a non-root ancestor, `root` = only the library-root `🧪tests` (not counted as unit test).")
w("- **Tests**: code files (ts/mjs/mts/cjs/js/rs/go/py) under the scope's `🧪tests` tree.")
w("- **Corpus**: data file (json/feature/yaml/toml/csv) under that tree, excluding schema-like names; counted as consumed when a test file names its basename or scans the directory (readdir/glob).")
w("- **Oracle**: a `*oracle*.py` file, or a test file importing a third-party npm package (not test runner, not `@semio-tech/*`, not node built-in) or SQLite (`bun:sqlite`, `node:sqlite`).")
w("- Classes (risk order): 1-NOTEST (no unit-scoped test), 2-NOCORPUS (tests, no consumed corpus), 3-NOORACLE (corpus, no oracle), 4-OK.")
w("")
w("## Totals")
w("")
w(f"- Tracked library files: {len(files)}; production files (excl. tests/fixtures): {len(prod)}; feature units: {tot_units}; production lines: {tot_lines}.")
for cls in ["1-NOTEST", "2-NOCORPUS", "3-NOORACLE", "4-OK"]:
    n_, l_ = agg(lambda x, c=cls: x["cls"] == c)
    w(f"- {cls}: {n_} units, {l_} production lines ({100*l_/max(tot_lines,1):.1f}%).")
w(f"- Units with `🧬schema/*.json` (own): {sum(1 for x in rows if x['schema'])} of {tot_units} ({100*sum(1 for x in rows if x['schema'])/tot_units:.1f}%), {sum(x['lines'] for x in rows if x['schema'])} lines.")
w(f"- Strict own-scope only (ignoring ancestor inheritance): own `🧪tests` {sum(1 for x in rows if x['own_tests'])}; own consumed corpus {sum(1 for x in rows if x['own_corpus'])}; own oracle {sum(1 for x in rows if x['own_oracle'])}.")
w(f"- Units using root-only tests (no unit-scoped test at all): {sum(1 for x in rows if x['scope']=='root')}, {sum(x['lines'] for x in rows if x['scope']=='root')} lines; of these {sum(1 for x in rows if x['scope']=='root' and x['root_named'])} are named by a library-root test file (name match only).")
w(f"- Strict own-scope classes (own `🧪tests` only): NOTEST {sum(1 for x in rows if not x['own_tests'])} units, {sum(x['lines'] for x in rows if not x['own_tests'])} lines; own-tested units (any class): {sum(1 for x in rows if x['own_tests'])}; own-tested with consumed corpus and oracle: {sum(1 for x in rows if x['own_tests'] and x['own_corpus'] and x['own_oracle'])}.")
w(f"- Units in a scope with `🧪tests` but no consumed corpus: {agg(lambda x: x['cls']=='2-NOCORPUS')[0]}; corpus but no oracle: {agg(lambda x: x['cls']=='3-NOORACLE')[0]}; fully covered: {agg(lambda x: x['cls']=='4-OK')[0]}.")
w(f"- Third-party oracle libraries seen in non-root test scopes (top 12, import count): " + ", ".join(f"{k} {v}" for k, v in libs_total.most_common(12)) + ".")
w("")
w("## Touched live on 10-21 (coordination.md from 11:59)")
w("")
w("Units named by identifier or file in coordination.md entries from 11:59 (638, 646, 652, 654-660, 670-671; located by searching library source for the named identifiers/files). Not located in the library: `store.recheck()` (670, many sites), `installOffBrowserGlobals` (670), `⚙runtime/🏷identity` (643), `🧪fixture/🟨.mjs` (666).")
w("")
if named_rows:
    w("| Class | Unit | Prod lines | Schema | Scope | Corpus consumed | Oracle | Named by |")
    w("|---|---|---:|:-:|---|---:|:-:|---|")
    for x in named_rows:
        w(f"| {x['cls']} | `{x['unit']}` | {x['lines']} | {'y' if x['schema'] else 'n'} | {x['scope']} | {x['consumed']} | {'y' if x['oracle'] else 'n'} | {'; '.join(x['named'])} |")
else:
    w("No unit matched.")
w("")
w(f"Units with a production file modified on 2026-10-21 (mtime, any time of day): {len(mt_rows)} of {tot_units}; of these, class 1-NOTEST: {sum(1 for x in mt_rows if x['cls']=='1-NOTEST')}.")
w("")
w("## Top 60 units by risk")
w("")
w("| # | Class | Unit | Prod lines | Files | Schema | Scope | Tests | Root-named | Corpus consumed | Oracle | Touched 10-21 |")
w("|---:|---|---|---:|---:|:-:|---|---:|:-:|---:|:-:|:-:|")
for i, x in enumerate(rows[:60], 1):
    tch = ("coord" if x["named"] else "") + (" mtime" if x["mtime_0921"] else "")
    w(f"| {i} | {x['cls']} | `{x['unit']}` | {x['lines']} | {x['files']} | {'y' if x['schema'] else 'n'} | {x['scope']} | {x['tests']} | {'y' if x['root_named'] else 'n'} | {x['consumed']}/{x['corpus']} | {'y' if x['oracle'] else 'n'} | {tch.strip() or '-'} |")
w("")
w("## Caveats")
w("")
w("- Scope evidence for `ancestor` units is shared with every unit in that subtree; ancestor-based OK/NOORACLE ratings overstate per-unit coverage.")
w("- Corpus consumption is basename-level (a test naming `🔣.json` marks every `🔣.json` in scope as consumed); oracle is import-level: presence of a third-party import is not proof that output is compared.")
w("- Excluded from production by rule: `📜script.ts` (77), `🍎.ts` (2), `🟨.cjs` (6), `🟨.d.mts` (45) and `🔣.json` data files; library root tests (451 files, 213 fixtures) are not counted as unit tests.")
w("- Non-TS test languages (rs/go) are counted as tests but not analyzed for oracles.")
open(REPORT, "w", encoding="utf8").write("\n".join(out) + "\n")
print("units", tot_units, "lines", tot_lines, {c: agg(lambda x, c=c: x["cls"] == c) for c in order})
