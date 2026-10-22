import lib
from lib import S
UNIT = "🎛commands/📝commit/🔢counter"
V = "️"
Y, M, D, F, R, B = "🎆" + V, "🌙" + V, "☀" + V, "🚩" + V, "🐙" + V, ""
def base(prefix, yy, mm, dd):
    return "%s%s%s%s%s%s%s" % (prefix, Y, yy, M, mm, D, dd)
def subj(prefix, yy, mm, dd, n):
    return base(prefix, yy, mm, dd) + F + str(n)
me = {"alias": "ueli", "emoji": "🐙" + V, "name": "Ueli", "email": "u@example.com"}
other = {"alias": "bob", "emoji": "🦑" + V, "name": "Bob", "email": "b@example.com"}
P = me["emoji"] + me["alias"]
PO = other["emoji"] + other["alias"]
extract = [(subj(P, "26", "06", "04", 700), {"nnn": 700, "line1Base": base(P, "26", "06", "04")}), ("  " + subj(P, "26", "06", "04", 7) + "  ", {"nnn": 7, "line1Base": base(P, "26", "06", "04")}), (base(P, "26", "06", "04") + F, None), ("plain subject", None), (subj(P, "26", "6", "04", 1), None), (subj(P, "26", "06", "04", "x"), None), ("", None), (subj("", "26", "06", "04", 5), None), (subj(P, "26", "06", "04", 0), {"nnn": 0, "line1Base": base(P, "26", "06", "04")})]
numeric = [("152", 152), (" 299 ", 299), ("0", None), ("-5", None), ("12a", None), ("", None), ("007", 7)]
tags = [(base(P, "26", "06", "04") + F, base(P, "26", "06", "04")), (" " + base(P, "26", "06", "04") + F + " ", base(P, "26", "06", "04")), (base(P, "26", "06", "04"), None), (base(P, "26", "06", "04") + F + "1", None), ("v1.0.0", None)]
bump = [
 {"name": "no history starts a fresh epoch at counter 001", "subjects": [], "wip": None, "contributor": me, "now": [2026, 10, 21], "expected": {"line1Base": base(P, "26", "10", "21"), "nnn": "001"}},
 {"name": "the newest formatted subject keeps its epoch and the maximum counter wins", "subjects": [subj(P, "26", "06", "04", 700), subj(P, "26", "06", "04", 699)], "wip": None, "contributor": me, "now": [2026, 10, 21], "expected": {"line1Base": base(P, "26", "06", "04"), "nnn": "701"}},
 {"name": "the maximum over the whole history counts, not the newest", "subjects": [subj(P, "26", "06", "04", 5), subj(P, "26", "06", "04", 12), subj(P, "26", "06", "04", 9)], "wip": None, "contributor": me, "now": [2026, 10, 21], "expected": {"line1Base": base(P, "26", "06", "04"), "nnn": "013"}},
 {"name": "another contributor's counters are ignored", "subjects": [subj(PO, "26", "06", "04", 900), subj(P, "26", "06", "04", 3)], "wip": None, "contributor": me, "now": [2026, 10, 21], "expected": {"line1Base": base(P, "26", "06", "04"), "nnn": "004"}},
 {"name": "only another contributor's counters leave the fresh epoch at 001", "subjects": [subj(PO, "26", "06", "04", 900)], "wip": None, "contributor": me, "now": [2026, 10, 21], "expected": {"line1Base": base(P, "26", "10", "21"), "nnn": "001"}},
 {"name": "numeric GitKraken subjects continue the counter on the fresh epoch", "subjects": ["152", "299", "17"], "wip": None, "contributor": me, "now": [2026, 1, 2], "expected": {"line1Base": base(P, "26", "01", "02"), "nnn": "300"}},
 {"name": "a persisted WIP epoch replaces the fresh epoch", "subjects": ["152"], "wip": base(P, "25", "01", "01"), "contributor": me, "now": [2026, 10, 21], "expected": {"line1Base": base(P, "25", "01", "01"), "nnn": "153"}},
 {"name": "formatted history wins over the persisted WIP epoch", "subjects": [subj(P, "26", "06", "04", 1)], "wip": base(P, "25", "01", "01"), "contributor": me, "now": [2026, 10, 21], "expected": {"line1Base": base(P, "26", "06", "04"), "nnn": "002"}},
 {"name": "counters of 1000 and above keep growing past three digits", "subjects": [subj(P, "26", "06", "04", 999)], "wip": None, "contributor": me, "now": [2026, 10, 21], "expected": {"line1Base": base(P, "26", "06", "04"), "nnn": "1000"}},
]
bump_errors = [{"name": "a prior subject that looks like a counter but does not parse refuses to reset to 001", "subjects": [subj(P, "26", "06", "4", 7)], "contributor": me, "message": "refusing to reset counter to 001"}]
bump_defects = [
 {"name": "subjects without emoji presentation selectors continue the counter", "subjects": [subj("🐙ueli", "26", "06", "04", 700).replace(V, "")], "wip": None, "contributor": me, "now": [2026, 10, 21], "expected": {"line1Base": "🐙ueli🎆26🌙06☀04", "nnn": "701"},
  "defect": "🔢️counter/🟦️.ts:5 COUNTER_RE spells each emoji with its selector and then an optional selector, so a subject without selectors never parses and bumpCounterFromHistory throws instead of continuing"}]
canon = [{"base": base(P, "26", "06", "04"), "contributor": me, "expected": base(P, "26", "06", "04")}, {"base": "no epoch here", "contributor": me, "expected": "no epoch here"}]
canon_defect = [{"base": base(PO, "26", "06", "04"), "contributor": me, "expected": base(P, "26", "06", "04"), "defect": "🔢️counter/🟦️.ts:canonicalWipLine1Base applies a selector-carrying pattern to text whose selectors were just stripped, so it never matches and never rewrites the contributor prefix"}]
second = [{"now": [2026, 10, 21, 5, 7, 9], "expected": "🎆️26🌙️10☀️21⏰️05⌚️07⏱️09"}, {"now": [2026, 1, 2, 23, 59, 59], "expected": "🎆️26🌙️01☀️02⏰️23⌚️59⏱️59"}, {"now": [2105, 12, 31, 0, 0, 0], "expected": "🎆️05🌙️12☀️31⏰️00⌚️00⏱️00"}]
repo = {"commits": [subj(P, "26", "06", "04", 41), "unrelated", subj(PO, "26", "06", "04", 90)], "tags": [base(P, "25", "05", "05") + F], "expectedNext": {"line1Base": base(P, "26", "06", "04"), "nnn": "042"}, "expectedWip": base(P, "26", "06", "04"),
        "tagOnly": {"commits": ["unrelated"], "tags": [base(P, "25", "05", "05") + F, "v1"], "expectedWip": base(P, "25", "05", "05")}, "none": {"commits": ["unrelated"], "tags": []}}
corpus = {"version": 1, "contributor": me, "extract": [{"subject": a, "expected": b} for a, b in extract], "numeric": [{"subject": a, "expected": b} for a, b in numeric], "tag": [{"tag": a, "expected": b} for a, b in tags],
 "bump": bump, "bumpErrors": bump_errors, "bumpDefects": bump_defects, "canonical": canon, "canonicalDefects": canon_defect, "second": second, "pad": [{"n": n, "expected": e} for n, e in [(0, "00"), (5, "05"), (12, "12"), (123, "123")]], "repo": repo}
cs = S.obj({"alias": S.str, "emoji": S.str, "name": S.str, "email": S.str})
res = S.obj({"line1Base": S.str, "nnn": S.str})
dt = S.arr(S.int, 3)
bumpc = S.obj({"name": S.str, "subjects": S.strs(), "wip": S.one(S.str, S.null), "contributor": cs, "now": dt, "expected": res})
schema = S.obj({"version": S.const(1), "contributor": cs, "extract": S.arr(S.obj({"subject": S.str, "expected": S.one(S.obj({"nnn": S.int, "line1Base": S.str}), S.null)}), 1),
 "numeric": S.arr(S.obj({"subject": S.str, "expected": S.one(S.int, S.null)}), 1), "tag": S.arr(S.obj({"tag": S.str, "expected": S.one(S.str, S.null)}), 1),
 "bump": S.arr(bumpc, 1), "bumpErrors": S.arr(S.obj({"name": S.str, "subjects": S.strs(), "contributor": cs, "message": S.str}), 1),
 "bumpDefects": S.arr(S.obj({"name": S.str, "subjects": S.strs(), "wip": S.one(S.str, S.null), "contributor": cs, "now": dt, "expected": res, "defect": S.str}), 1),
 "canonical": S.arr(S.obj({"base": S.str, "contributor": cs, "expected": S.str}), 1), "canonicalDefects": S.arr(S.obj({"base": S.str, "contributor": cs, "expected": S.str, "defect": S.str}), 1),
 "second": S.arr(S.obj({"now": S.arr(S.int, 6), "expected": S.str}), 1), "pad": S.arr(S.obj({"n": S.int, "expected": S.str}), 1),
 "repo": S.obj({"commits": S.strs(), "tags": S.strs(), "expectedNext": res, "expectedWip": S.str, "tagOnly": S.obj({"commits": S.strs(), "tags": S.strs(), "expectedWip": S.str}), "none": S.obj({"commits": S.strs(), "tags": S.strs()})})})
bodyts = '''const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const history=(commits:string[],tags:string[]):string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"counter-")));git(root,"init","-q");for(const subject of commits)git(root,"commit","-q","--allow-empty","-m",subject);for(const tag of tags)git(root,"tag",tag);return root;};
const when=(parts:number[]):Date=>new Date(parts[0]!,parts[1]!-1,parts[2]!,parts[3]??0,parts[4]??0,parts[5]??0);
for(const row of corpus.extract)test("extract counter of "+JSON.stringify(row.subject),()=>{expect(extractCounterFromSubject(row.subject)).toEqual(row.expected);});
for(const row of corpus.numeric)test("numeric counter of "+JSON.stringify(row.subject),()=>{expect(extractNumericCounterFromSubject(row.subject)).toBe(row.expected);});
for(const row of corpus.tag)test("bundle tag epoch of "+JSON.stringify(row.tag),()=>{expect(line1BaseFromBundleTag(row.tag)).toBe(row.expected);});
for(const row of corpus.bump)test("bump: "+row.name,()=>{expect(bumpCounterFromHistory(row.subjects,row.contributor,when(row.now),row.wip)).toEqual(row.expected);});
for(const row of corpus.bumpErrors)test("bump fails: "+row.name,()=>{expect(()=>bumpCounterFromHistory(row.subjects,row.contributor,when([2026,10,21]))).toThrow(row.message);});
for(const row of corpus.bumpDefects)test.failing("bump defect: "+row.name,()=>{expect(bumpCounterFromHistory(row.subjects,row.contributor,when(row.now),row.wip)).toEqual(row.expected);});
for(const row of corpus.canonical)test("canonical epoch of "+JSON.stringify(row.base),()=>{expect(canonicalWipLine1Base(row.base,row.contributor)).toBe(row.expected);});
for(const row of corpus.canonicalDefects)test.failing("canonical epoch defect "+JSON.stringify(row.base),()=>{expect(canonicalWipLine1Base(row.base,row.contributor)).toBe(row.expected);});
for(const row of corpus.second)test("second stamp "+row.expected,()=>{expect(formatSecond(when(row.now))).toBe(row.expected);});
for(const row of corpus.pad)test("pad "+row.n,()=>{expect(pad2(row.n)).toBe(row.expected);});
test("a single-subject bump equals the history bump",()=>{const row=corpus.bump[1]!;expect(bumpCounterFromSubject(row.subjects[0]!,row.contributor,when(row.now))).toEqual(bumpCounterFromHistory([row.subjects[0]!],row.contributor,when(row.now)));});
test("the repository log and tags drive the persisted epoch and the next counter",()=>{
 const found=history(corpus.repo.commits,corpus.repo.tags),tagged=history(corpus.repo.tagOnly.commits,corpus.repo.tagOnly.tags),empty=history(corpus.repo.none.commits,corpus.repo.none.tags);
 try{
  expect(contributorWipLine1Base(found,corpus.contributor)).toBe(corpus.repo.expectedWip);expect(nextCounter(found,corpus.contributor)).toEqual(corpus.repo.expectedNext);
  expect(contributorWipLine1Base(tagged,corpus.contributor)).toBe(corpus.repo.tagOnly.expectedWip);expect(contributorWipLine1Base(empty,corpus.contributor)).toBeNull();
 }finally{for(const root of[found,tagged,empty])rmSync(root,{recursive:true,force:true});}
});'''
ts = lib.ts_wrap(UNIT, "🔢️ Contract: the micro-commit counter reads `…🚩️NNN` subjects and bundle tags, continues the highest counter of the contributor on the right epoch and refuses to reset on an unparsable prior; the corpus is language-agnostic and `@@O@@` recomputes every verdict on selector-stripped text.",
 'import {mkdtempSync,realpathSync,rmSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {bumpCounterFromHistory,bumpCounterFromSubject,canonicalWipLine1Base,contributorWipLine1Base,extractCounterFromSubject,extractNumericCounterFromSubject,formatSecond,line1BaseFromBundleTag,nextCounter,pad2} from "@@' + UNIT + '/🟦.ts@@";', bodyts)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent model of the micro-commit counter: parses on selector-stripped text and maps the match back to the original string (stdlib only).") + r'''
SEL = "︎️"


def parse(subject):
    s = subject.strip()
    stripped = "".join(c for c in s if c not in SEL)
    m = re.match(r"^(.+🎆\d{2}🌙\d{2}☀\d{2})🚩(\d+)$", stripped)
    if not m:
        return None
    return {"nnn": int(m.group(2)), "stripped": m.group(1), "original": s}


def line1(subject):
    p = parse(subject)
    if not p:
        return None
    o = p["original"]
    cut = o.rfind("🚩")
    return o[:cut]


def numeric(subject):
    m = re.match(r"^(\d+)$", subject.strip())
    return int(m.group(1)) if m and int(m.group(1)) > 0 else None


def bump(subjects, who, now, wip):
    prefix = who["emoji"] + who["alias"]
    pn = "".join(c for c in prefix if c not in SEL)
    top, base = 0, None
    for s in subjects:
        p = parse(s)
        if p:
            if not p["stripped"].startswith(pn):
                continue
            top = max(top, p["nnn"])
            base = base or line1(s)
        elif numeric(s) is not None:
            top = max(top, numeric(s))
    yy, mm, dd = now
    fresh = "%s🎆️%02d🌙️%02d☀️%02d" % (prefix, yy % 100, mm, dd)
    epoch = base or wip or fresh
    if top > 0:
        return {"line1Base": epoch, "nnn": "%03d" % (top + 1)}
    for s in subjects:
        n = "".join(c for c in s.strip() if c not in SEL)
        if n.startswith(pn + "🎆") and re.search(r"🚩\d+$", n) and not parse(s):
            raise ValueError("refuse")
    return {"line1Base": epoch, "nnn": "001"}


for c in corpus["extract"]:
    p = parse(c["subject"])
    got = None if p is None else {"nnn": p["nnn"], "line1Base": line1(c["subject"])}
    if got != c["expected"]:
        failures.append("extract %r" % c["subject"])
for c in corpus["numeric"]:
    if numeric(c["subject"]) != c["expected"]:
        failures.append("numeric %r" % c["subject"])
for c in corpus["tag"]:
    n = "".join(ch for ch in c["tag"].strip() if ch not in SEL)
    m = re.match(r"^(.+🎆\d{2}🌙\d{2}☀\d{2})🚩$", n)
    got = c["tag"].strip()[: c["tag"].strip().rfind("🚩")] if m and c["tag"].strip().endswith("🚩" + VS) else None
    if got != c["expected"]:
        failures.append("tag %r" % c["tag"])
for c in corpus["bump"]:
    if bump(c["subjects"], c["contributor"], c["now"], c["wip"]) != c["expected"]:
        failures.append("bump: " + c["name"])
for c in corpus["bumpErrors"]:
    try:
        bump(c["subjects"], c["contributor"], [2026, 10, 21], None)
        failures.append("bump error: " + c["name"])
    except ValueError:
        pass
for c in corpus["second"]:
    y, mo, d, h, mi, s = c["now"]
    if "🎆️%02d🌙️%02d☀️%02d⏰️%02d⌚️%02d⏱️%02d" % (y % 100, mo, d, h, mi, s) != c["expected"]:
        failures.append("second %s" % c["expected"])
for c in corpus["pad"]:
    if str(c["n"]).rjust(2, "0") != c["expected"]:
        failures.append("pad %d" % c["n"])
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, oracle)
