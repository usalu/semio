import re, lib
from lib import S, resolve, LIBN
from pyemoji import PY_EMOJI
V = "️"
DATE = lambda yy, mm, dd: "🎆" + V + yy + "🌙" + V + mm + "☀" + V + dd
D21, D22 = DATE("26", "10", "21"), DATE("26", "10", "22")
SPLIT, FLAG, METRIC, TOTAL = "🔀" + V, "🚩" + V, "📊" + V + "metric", "🔢" + V
SC1, SC2 = "🏘" + V + "compose✍" + V + "sketchpad", "🥅" + V + "framework"
# ---------------- body
UB = "🎛commands/📝commit/📜body"
valid = "# comment\n" + SC1 + "\n" + D21 + "\n🧪" + V + "add tests\n🚀ship it\n" + D22 + METRIC + "📃uloc💯" + V + "1k\n🔥hot fix\n\n" + SC2 + "\n" + D21 + "\n🧰" + V + "tool box\n" + METRIC + "📃uloc\nSigned-off-by: someone\n"
clean = SC1 + "\n" + D21 + "\n🧪" + V + "add tests\n🚀ship it\n" + D22 + "\n🔥hot fix\n\n" + SC2 + "\n" + D21 + "\n🧰" + V + "tool box\n"
parsed = [{"label": SC1, "dates": [{"dateLine": D21, "bullets": ["🧪" + V + "add tests", "🚀ship it"]}, {"dateLine": D22, "bullets": ["🔥hot fix"]}]}, {"label": SC2, "dates": [{"dateLine": D21, "bullets": ["🧰" + V + "tool box"]}]}]
errors = [
 ("empty text", "", "at least one emoji bundle scope is required"), ("only comments", "# a\n\n# b", "at least one emoji bundle scope is required"), ("a date before any scope", D21 + "\n🚀x", "date section before bundle scope"),
 ("a bullet before a date section", SC1 + "\n🚀x", "expected " + D21[:0] + "🎆" + V + "YY🌙" + V + "MM☀" + V + "DD date section before bullet"), ("a non-scope first line", "plain text", "expected emoji bundle scope"),
 ("a bundle without a date section", SC1, "needs at least one date section"), ("a date section without bullets", SC1 + "\n" + D21, "needs at least one bullet"), ("a bullet with a space after a plain emoji", SC1 + "\n" + D21 + "\n🚀 x", "bullet must start with {emoji} then description"),
 ("a reserved bullet emoji", SC1 + "\n" + D21 + "\n" + TOTAL + "x", "commit: " + TOTAL + " is reserved"), ("a path where a scope belongs", SC1 + "\n" + D21 + "\n🚀a\na/b", "bullet must start with"), ("a scope with a reserved emoji", SC1 + SPLIT + "\n" + D21 + "\n🚀x", "bundle scope must not include"),
 ("a second bundle without bullets", SC1 + "\n" + D21 + "\n🚀x\n" + SC2, "needs at least one date section"), ("a date-like line is read as a new scope, leaving the day empty", SC1 + "\n" + D21 + "\n" + D22 + "x", "needs at least one bullet"),
]
body_err = [
 ("a bundle subject", "🐙" + V + "ueli" + D21 + SPLIT + "\nrest", "must not include the bundle subject"), ("a micro-commit subject", "🐙" + V + "ueli" + D21 + FLAG + "5\nrest", "must not include micro-commit subject lines"), ("a metric footer", SC1 + "\n" + METRIC + "📃uloc", "must not include the " + METRIC + " footer"),
 ("a per-day metric", SC1 + "\n" + D21 + METRIC + "📃uloc", "per-day"), ("a clean body", clean, None), ("an empty body", "", None),
]
corpus_b = {"version": 1, "valid": {"text": valid, "expected": parsed}, "errors": [{"name": n, "text": t, "message": m} for n, t, m in errors], "bodyErrors": [{"name": n, "text": t, "message": m} for n, t, m in body_err],
            "defects": [{"name": "a one-word bullet is a bullet, not a bundle scope", "text": SC1 + "\n" + D21 + "\n🧪" + V + "tests\n", "expected": [{"label": SC1, "dates": [{"dateLine": D21, "bullets": ["🧪" + V + "tests"]}]}], "message": "", "defect": "📜️body/🟦️.ts:parseCommitBundleBody tests isBundleScopeLine before validating a bullet, so a bullet that is one emoji plus one lowercase word (`🧪️tests`, `🔧️fix`) is read as a new bundle scope and the day fails with \"needs at least one bullet\""}, {"name": "a bullet with a space after an emoji carrying a variation selector is refused", "text": SC1 + "\n" + D21 + "\n🧪" + V + " add tests", "message": "bullet must start with {emoji} then description", "defect": "📜️body/🟦️.ts:validateBulletLine relies on MICRO_COMMIT_BULLET_RE, whose `\\S` can match the variation selector, so `🧪️ add tests` (space after the emoji) is admitted"}]}
bl = S.obj({"label": S.str, "dates": S.arr(S.obj({"dateLine": S.str, "bullets": S.strs()}))})
cs = lambda extra=None: S.obj({"name": S.str, "text": S.str, "message": S.one(S.str, S.null)})
sch_b = S.obj({"version": S.const(1), "valid": S.obj({"text": S.str, "expected": S.arr(bl)}), "errors": S.arr(S.obj({"name": S.str, "text": S.str, "message": S.str}), 1), "bodyErrors": S.arr(cs(), 1), "defects": S.arr(S.obj({"name": S.str, "text": S.str, "message": S.str, "defect": S.str, "expected": S.arr(bl)}, required=["name", "text", "message", "defect"]), 1)})
body_ts = '''test("a full bundle body is parsed into scopes, days and bullets",()=>{expect(parseCommitBundleBody(corpus.valid.text)).toEqual(corpus.valid.expected);});
for(const row of corpus.errors)test("parse refuses "+row.name,()=>{expect(()=>parseCommitBundleBody(row.text)).toThrow(row.message);});
for(const row of corpus.defects)test.failing("parse defect: "+row.name,()=>{if((row as {expected?:unknown}).expected!==undefined)expect(parseCommitBundleBody(row.text)).toEqual((row as {expected:unknown}).expected);else expect(()=>parseCommitBundleBody(row.text)).toThrow(row.message);});
for(const row of corpus.bodyErrors)test("body error: "+row.name,()=>{const error=commitBundleBodyError(row.text);if(row.message===null)expect(error).toBeNull();else expect(error).toContain(row.message);});
test("every parsed bundle has a date and every day a bullet",()=>{for(const bundle of parseCommitBundleBody(corpus.valid.text)){expect(bundle.dates.length).toBeGreaterThan(0);for(const date of bundle.dates)expect(date.bullets.length).toBeGreaterThan(0);}});'''
ts_b = lib.ts_wrap(UB, "📜️ Contract: the commit bundle body parser reads emoji scope lines, `🎆️YY🌙️MM☀️DD` day sections and emoji bullets, ignores comments, metrics and sign-offs, and refuses a stray subject, footer, path or malformed bullet with a fixed message; the corpus is language-agnostic and `@@O@@` is a line-state machine over the same text.",
 'import {commitBundleBodyError,parseCommitBundleBody} from "@@' + UB + '/🟦.ts@@";', body_ts)
orc_b = lib.PY_HEAD.format(doc="🐍️ Independent model of the commit bundle body grammar: a three-state line machine (scope, day, bullet) with emoji classification by Unicode range (stdlib only).") + PY_EMOJI + r'''
DAY = "🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2}"
RES = ["🔀" + VS, "🚩" + VS, "📊" + VS + "metric", "🔢" + VS]
RESERVED_LEAD = {"🎆" + VS, "📊" + VS, "🔢" + VS, "🚩" + VS}


def normalize_scope(line):
    s = line.strip()
    k = s.find("📊" + VS + "metric")
    s = s[:k] if k >= 0 else s
    return s.replace("🔀" + VS, "").replace("🚩" + VS, "").strip()


def clusters(t):
    n, i = 0, 0
    while i < len(t):
        c = lead(t[i:])
        if c:
            n += 1
            i += len(c)
        else:
            i += 1
    return n


def is_scope(line):
    t = normalize_scope(line)
    if not t or re.match("^" + DAY + "$", t) or "/" in t or "|" in t or any(r in t for r in RES):
        return False
    n = clusters(t)
    if n >= 2:
        return True
    return n == 1 and re.match(r"^[a-z][a-z0-9]{2,24}$", t[len(lead(t)):]) is not None


def scope_error(line):
    raw = line.strip()
    if "|" in raw or "/" in raw or any(r in raw for r in RES):
        return True
    n = normalize_scope(raw)
    tokens = [x for x in re.split(r"[^a-z0-9]+", "".join(" " if pict(c) else c for c in n).strip().lower()) if len(x) >= 2 and x not in {"uloc", "repo", "the", "and"}]
    return not n or not tokens or not is_scope(n)


def bullet_ok(b):
    cl = lead(b)
    return bool(cl) and len(b) > len(cl) and not b[len(cl)].isspace() and True


def validate_bullet(b):
    if not bullet_ok(b):
        raise ValueError("bullet")
    if lead_cluster_seq(b.strip()) in RESERVED_LEAD:
        raise ValueError("reserved")
    if re.match("^" + DAY + "$", b.strip()):
        raise ValueError("date bullet")


def parse(text):
    bundles, cur, ds = [], None, None
    for raw in text.split("\n"):
        line = raw.strip()
        if not line or line.startswith("#") or line.startswith("📊" + VS + "metric") or line.startswith("Signed-off-by:"):
            continue
        cand = normalize_scope(line) if False else (lambda s: (s[:s.find("📊" + VS + "metric")] if s.find("📊" + VS + "metric") >= 0 else s).strip())(line)
        if re.match("^" + DAY + "$", cand):
            if cur is None:
                raise ValueError("date before scope")
            if ds:
                cur["dates"].append(ds)
            ds = {"dateLine": cand, "bullets": []}
            continue
        if ds is not None:
            if is_scope(line):
                if scope_error(line):
                    raise ValueError("scope")
                if cur:
                    cur["dates"].append(ds)
                    bundles.append(cur)
                cur, ds = {"label": normalize_scope(line), "dates": []}, None
                continue
            validate_bullet(line)
            ds["bullets"].append(line)
            continue
        if is_scope(line):
            if scope_error(line):
                raise ValueError("scope")
            if cur:
                bundles.append(cur)
            cur = {"label": normalize_scope(line), "dates": []}
            continue
        raise ValueError("expected")
    if cur:
        if ds:
            cur["dates"].append(ds)
        bundles.append(cur)
    if not bundles:
        raise ValueError("none")
    for b in bundles:
        if not b["dates"]:
            raise ValueError("nodate")
        for d in b["dates"]:
            if not d["bullets"]:
                raise ValueError("nobullet")
    return bundles


if parse(corpus["valid"]["text"]) != corpus["valid"]["expected"]:
    failures.append("valid")
for c in corpus["errors"]:
    try:
        parse(c["text"])
        failures.append("accepted " + c["name"])
    except ValueError:
        pass
for c in corpus["defects"]:
    if "expected" in c:
        continue
    try:
        parse(c["text"])
        failures.append("model accepted " + c["name"])
    except ValueError:
        pass


def body_error(text):
    first = next((l.strip() for l in text.strip().split("\n") if l.strip()), "")
    if re.match("^.+" + DAY + "🔀" + VS + "$", first):
        return "subject"
    if re.match("^(🐙" + VS + "|🧑" + VS + ")", first) and "🚩" + VS in first:
        return "micro"
    if re.search("^📊" + VS + "metric", text, re.M):
        return "footer"
    if re.search("^" + DAY + "📊" + VS + "metric", text, re.M):
        return "per-day"
    return None


KIND = {"must not include the bundle subject": "subject", "must not include micro-commit": "micro", "footer": "footer", "per-day": "per-day"}
for c in corpus["bodyErrors"]:
    want = None if c["message"] is None else next(v for k, v in KIND.items() if k in c["message"])
    if body_error(c["text"]) != want:
        failures.append("body error " + c["name"])
''' + lib.PY_TAIL
lib.write_unit(UB, corpus_b, sch_b, ts_b, orc_b)
# ---------------- tags
UT = "🎛commands/📝commit/🔖tags"
me = {"alias": "ueli", "emoji": "🐙" + V, "name": "U", "email": "u@e.x"}
tag_cases = [{"now": [2026, 10, 21], "expectedTag": me["emoji"] + "ueli" + D21 + FLAG, "expectedSubject": me["emoji"] + "ueli" + D21 + SPLIT}, {"now": [2105, 1, 2], "expectedTag": me["emoji"] + "ueli" + DATE("05", "01", "02") + FLAG, "expectedSubject": me["emoji"] + "ueli" + DATE("05", "01", "02") + SPLIT}]
repo = {"commits": ["start", me["emoji"] + "ueli" + DATE("26", "10", "20") + SPLIT, "middle", me["emoji"] + "ueli" + D21 + SPLIT, "latest"], "expectedIndex": 3, "none": ["a", "b"], "lookback": {"wipAt": 0}}
corpus_t = {"version": 1, "contributor": me, "names": tag_cases, "repo": repo}
cn = S.obj({"alias": S.str, "emoji": S.str, "name": S.str, "email": S.str})
sch_t = S.obj({"version": S.const(1), "contributor": cn, "names": S.arr(S.obj({"now": S.arr(S.int, 3), "expectedTag": S.str, "expectedSubject": S.str}), 1), "repo": S.obj({"commits": S.strs(), "expectedIndex": S.int, "none": S.strs(), "lookback": S.obj({"wipAt": S.int})})})
body_t = '''const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const history=(subjects:string[]):{root:string;shas:string[]}=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"tags-")));git(root,"init","-q");const shas:string[]=[];for(const subject of subjects){git(root,"commit","-q","--allow-empty","-m",subject);shas.push(git(root,"rev-parse","HEAD"));}return{root,shas};};
for(const[index,row]of corpus.names.entries())test("tag and subject names "+index,()=>{const now=new Date(row.now[0]!,row.now[1]!-1,row.now[2]!);expect(formatBundleTagName(corpus.contributor,now)).toBe(row.expectedTag);expect(formatBundleSubject(corpus.contributor,now)).toBe(row.expectedSubject);});
test("the newest bundle subject is found in the log",()=>{
 const{root,shas}=history(corpus.repo.commits),empty=history(corpus.repo.none);
 try{
  const found=findLastBundleWipCommit(root);expect(found).not.toBeNull();expect(found!.sha).toBe(shas[corpus.repo.expectedIndex]);expect(found!.subject).toBe(corpus.repo.commits[corpus.repo.expectedIndex]);
  expect(findLastBundleWipCommit(empty.root)).toBeNull();expect(findLastBundleWipCommit(join(empty.root,"missing"))).toBeNull();
 }finally{rmSync(root,{recursive:true,force:true});rmSync(empty.root,{recursive:true,force:true});}
});'''
ts_t = lib.ts_wrap(UT, "🔖️ Contract: the bundle tag and subject are `<emoji><alias>🎆️YY🌙️MM☀️DD` plus the tag or squash marker, and the newest squash subject in the log is found by its own pattern; the corpus is language-agnostic and `@@O@@` rebuilds the names and replays the log with the git CLI.",
 'import {mkdtempSync,realpathSync,rmSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {findLastBundleWipCommit,formatBundleSubject,formatBundleTagName} from "@@' + UT + '/🟦.ts@@";', body_t)
orc_t = lib.PY_HEAD.format(doc="🐍️ Independent model of the bundle names and the newest-squash search, replaying the commits with the git CLI (stdlib + git).") + r'''import os, subprocess, tempfile, shutil

who = corpus["contributor"]
for c in corpus["names"]:
    y, m, d = c["now"]
    base = "%s%s🎆%s%02d🌙%s%02d☀%s%02d" % (who["emoji"], who["alias"], VS, y % 100, VS, m, VS, d)
    if base + "🚩" + VS != c["expectedTag"] or base + "🔀" + VS != c["expectedSubject"]:
        failures.append("names %r" % c["now"])


def git(cwd, *a):
    env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "-c", "user.name=t", "-c", "user.email=t@e.x"] + list(a), cwd=cwd, env=env, check=True, capture_output=True, text=True).stdout.strip()


root = tempfile.mkdtemp(prefix="tags-oracle-")
try:
    git(root, "init", "-q")
    for s in corpus["repo"]["commits"]:
        git(root, "commit", "-q", "--allow-empty", "-m", s)
    pattern = re.compile("^(.+🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2})🔀" + VS + "$")
    rows = git(root, "log", "--format=%H%x09%s").split("\n")
    first = next((r.split("\t") for r in rows if pattern.match(r.split("\t", 1)[1])), None)
    if first is None or first[1] != corpus["repo"]["commits"][corpus["repo"]["expectedIndex"]]:
        failures.append("newest squash")
finally:
    shutil.rmtree(root, ignore_errors=True)
''' + lib.PY_TAIL
lib.write_unit(UT, corpus_t, sch_t, ts_t, orc_t)
# ---------------- steps
US = "🎛commands/📝commit/🪜steps"
src = open(resolve(LIBN + "/" + US + "/🟦.ts"), encoding="utf8").read()
devs = re.search(r'"(🧑[^"]*devs)"', src).group(1)
parse_ok = [(["ct"], {"tag": True, "squash": False, "push": False}), (["cs"], {"tag": False, "squash": True, "push": False}), (["cp"], {"tag": False, "squash": False, "push": True}), (["CT", "CS"], {"tag": True, "squash": True, "push": False}), (["ctag"], {"tag": True, "squash": False, "push": False}), (["csquash", "cpush"], {"tag": False, "squash": True, "push": True}),
            ([], {"tag": False, "squash": False, "push": False}), (["x", "contact"], {"tag": False, "squash": False, "push": False}), (["ct,cs"], {"tag": True, "squash": True, "push": False}), (["a", "ct"], {"tag": True, "squash": False, "push": False}), (["tag!x"], {"tag": True, "squash": False, "push": False}), (["a+squash"], {"tag": False, "squash": True, "push": False})]
parse_def = [(["tag!"], {"tag": True, "squash": False, "push": False}), (["+squash"], {"tag": False, "squash": True, "push": False}), (["push!"], {"tag": False, "squash": False, "push": True}), (["+tag"], {"tag": True, "squash": False, "push": False}), (["x", "squash!"], {"tag": False, "squash": True, "push": False})]
prep = [([], True), (["cprepare"], True), (["cprepare", "ct"], True), (["ct"], False), (["cs"], False), (["x"], True), (["prepare!x"], True), (["cprepare", "cp"], True)]
prep_def = [(["c.", "ct"], True), (["prepare!", "ct"], True), (["+prepare", "cs"], True)]
levels = [("prepare-only", {"tag": False, "squash": False, "push": False}), ("prepare-and-tag", {"tag": True, "squash": False, "push": False}), ("prepare-and-tag-and-squash", {"tag": True, "squash": True, "push": False}), ("prepare-and-tag-and-squash-and-push", {"tag": True, "squash": True, "push": True}), ("unknown-level", {"tag": False, "squash": False, "push": False}), (None, {"tag": False, "squash": False, "push": False})]
corpus_s = {"version": 1, "devsDir": devs, "contributor": {"alias": "ueli", "emoji": "🐙" + V, "name": "U", "email": "u@e.x"}, "parse": [{"segments": a, "expected": b} for a, b in parse_ok], "parseDefects": [{"segments": a, "expected": b, "defect": "🪜️steps/🟦️.ts:parseCommitSteps: the alias spellings `tag!`, `squash!`, `push!`, `+tag`, `+squash`, `+push` can never match because `\\b` needs a word character on the side of the symbol, so only the `ct`/`cs`/`cp` and long names work"} for a, b in parse_def],
            "prepareOnly": [{"segments": a, "expected": b} for a, b in prep], "prepareOnlyDefects": [{"segments": a, "expected": b, "defect": "🪜️steps/🟦️.ts:isCommitPrepareOnly: `c.`, `prepare!` and `+prepare` can never match (the `\\b` after a symbol needs a following word character), so a prepare alias cannot override an explicit step"} for a, b in prep_def],
            "levels": [{"level": lvl, "expected": e} for lvl, e in levels]}
st = S.obj({"tag": S.bool, "squash": S.bool, "push": S.bool})
sch_s = S.obj({"version": S.const(1), "devsDir": S.str, "contributor": cn, "parse": S.arr(S.obj({"segments": S.strs(), "expected": st}), 1), "parseDefects": S.arr(S.obj({"segments": S.strs(), "expected": st, "defect": S.str}), 1),
               "prepareOnly": S.arr(S.obj({"segments": S.strs(), "expected": S.bool}), 1), "prepareOnlyDefects": S.arr(S.obj({"segments": S.strs(), "expected": S.bool, "defect": S.str}), 1), "levels": S.arr(S.obj({"level": S.one(S.str, S.null), "expected": st}), 1)})
body_s = '''const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const scratch=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"steps-")));roots.push(root);return root;};
const configure=(root:string,level:string|null):void=>{const folder=join(root,".\\u{1F9EC}semio","\\u{1F991}\\ufe0frepo",corpus.devsDir,corpus.contributor.alias);mkdirSync(folder,{recursive:true});writeFileSync(join(folder,"commit.json"),JSON.stringify(level===null?{}:{level}));};
for(const row of corpus.parse)test("steps of "+JSON.stringify(row.segments),()=>{expect(parseCommitSteps(row.segments)).toEqual(row.expected);});
for(const row of corpus.parseDefects)test.failing("steps defect "+JSON.stringify(row.segments),()=>{expect(parseCommitSteps(row.segments)).toEqual(row.expected);});
for(const row of corpus.prepareOnly)test("prepare-only of "+JSON.stringify(row.segments),()=>{expect(isCommitPrepareOnly(row.segments)).toBe(row.expected);});
for(const row of corpus.prepareOnlyDefects)test.failing("prepare-only defect "+JSON.stringify(row.segments),()=>{expect(isCommitPrepareOnly(row.segments)).toBe(row.expected);});
for(const row of corpus.levels)test("configured level "+row.level,()=>{const root=scratch();configure(root,row.level);expect(loadCommitSteps(root,corpus.contributor,[])).toEqual(row.expected);});
test("explicit steps override the configured level and a missing file means no steps",()=>{
 const root=scratch(),bare=scratch();configure(root,"prepare-and-tag-and-squash-and-push");
 expect(loadCommitSteps(root,corpus.contributor,["ct"])).toEqual({tag:true,squash:false,push:false});expect(loadCommitSteps(bare,corpus.contributor,[])).toEqual({tag:false,squash:false,push:false});expect(loadCommitSteps(bare,corpus.contributor,["cs","cp"])).toEqual({tag:false,squash:true,push:true});
});'''
ts_s = lib.ts_wrap(US, "🪜️ Contract: commit steps come from explicit `ct`/`cs`/`cp` words or, failing those, from the contributor's configured level, and prepare-only means no step at all or an explicit prepare word; the corpus is language-agnostic and `@@O@@` applies the same word-boundary grammar.",
 'import {afterAll} from "bun:test";\nimport {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {isCommitPrepareOnly,loadCommitSteps,parseCommitSteps} from "@@' + US + '/🟦.ts@@";', body_s)
orc_s = lib.PY_HEAD.format(doc="🐍️ Independent model of the commit step words with Python's own regex engine (same `\\b` semantics; stdlib only).") + r'''
def steps(seg):
    t = " ".join(seg).lower()
    return {"tag": re.search(r"\b(ct|ctag|tag!|\+tag)\b", t) is not None, "squash": re.search(r"\b(cs|csquash|squash!|\+squash)\b", t) is not None, "push": re.search(r"\b(cp|cpush|push!|\+push)\b", t) is not None}


def prepare_only(seg):
    t = " ".join(seg).lower()
    if re.search(r"\b(c\.|cprepare|prepare!|\+prepare)\b", t):
        return True
    s = steps(seg)
    return not (s["tag"] or s["squash"] or s["push"])


for c in corpus["parse"]:
    if steps(c["segments"]) != c["expected"]:
        failures.append("steps %r" % c["segments"])
for c in corpus["parseDefects"]:
    if steps(c["segments"]) == c["expected"]:
        failures.append("model fixed %r" % c["segments"])
for c in corpus["prepareOnly"]:
    if prepare_only(c["segments"]) != c["expected"]:
        failures.append("prepare %r" % c["segments"])
for c in corpus["prepareOnlyDefects"]:
    if prepare_only(c["segments"]) == c["expected"]:
        failures.append("model fixed prepare %r" % c["segments"])
''' + lib.PY_TAIL
lib.write_unit(US, corpus_s, sch_s, ts_s, orc_s)
