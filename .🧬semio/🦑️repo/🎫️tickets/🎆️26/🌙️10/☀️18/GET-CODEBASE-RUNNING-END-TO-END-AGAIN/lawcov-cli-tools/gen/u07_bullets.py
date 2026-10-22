import lib
from lib import S
from pyemoji import PY_EMOJI
UNIT = "🎛commands/📝commit/🔸bullets"
V = "️"
lead = [("🧪" + V + "tests", "🧪" + V), ("  🚀 deploy", "🚀"), ("plain", ""), ("👩‍💻x", "👩‍💻"), ("🧪" + V + "🔥x", "🧪" + V + "🔥"), ("✅" + V + "ok", "✅" + V), ("-🧪" + V + "x", ""), ("", ""), ("1️⃣x", "")]
fmt = [("- 🧪" + V + " add tests", "🧪" + V + "add tests"), ("--🚀  x", "🚀x"), ("🚀x", "🚀x"), ("plain text", "plain text"), ("🚀", "🚀"), ("   ", ""), ("-  - 🚀 y", "- 🚀 y"), ("🧪" + V + "\tz", "🧪" + V + "z")]
norm_text = "# heading\n\n- 🧪" + V + " add tests\n📊" + V + "metric📃uloc💯" + V + "1k\n x \n🚀 ship\n   \n✅" + V + "ok\n"
norm_expected = ["🧪" + V + "add tests", "🚀ship", "✅" + V + "ok"]
many = "\n".join("🚀item%d" % i for i in range(11))
reserved = {"🎆" + V: "🎆" + V + "x", "📊" + V: "📊" + V + "x", "🔢" + V: "🔢" + V + "x", "🚩" + V: "🚩" + V + "x"}
valid = [(["🧪" + V + "ok", "🚀fine"], None), ([], None), (["🎆" + V + "26🌙" + V + "10☀" + V + "21"], "reserved"), (["🧪" + V + "ok", "📊" + V + "metric"], "reserved"), (["🔢" + V + "total"], "reserved"), (["🚩" + V + "x"], "reserved")]
tokens = [("src/Some_File-name.test.ts", ["some", "file", "name", "test"]), ("a/b.c", []), ("framework/ui/index.ts", ["framework", "index"]), ("UPPER/lower.ts", ["upper", "lower"]), ("dup/dups.dups", ["dups"])]
STAGED = [
 ([], ["x"], ["other staged paths"]), (["🚀a"], [], []), (["🚀a"], ["x/micro-commit.ts", "y/index.test.ts", "z/SKILL.md"], []),
 (["🚀 plan updated"], [".cursor/plans/foo.md"], []), (["🚀 other"], [".cursor/plans/foo.md"], [".cursor/plans"]), (["🚀 CURSOR changed"], [".cursor/plans/foo-bar-baz.md"], []),
 (["🚀 skill docs"], [".agents/skills/a.md"], []), (["🚀 other"], [".agents/skills/a.md"], [".agents"]), (["🚀 hook fix"], ["tools/repo/x.ts"], []), (["🚀 nothing"], ["tools/repo/x.ts"], ["repo"]),
 (["🚀 devcontainer tweak"], [".devcontainer/a.json"], []), (["🚀 other"], [".devcontainer/a.json"], [".devcontainer"]),
 (["🚀 framework work"], ["framework/x/y.ts"], []), (["🚀 other"], ["framework/x/y.ts"], ["product"]), (["🚀 readme edit"], ["docs/readme.md"], []), (["🚀 other"], ["docs/readme.md"], ["other staged paths"]),
 (["🚀 plan"], [".cursor/plans/a.md", "docs/readme.md"], ["other staged paths"]), (["🚀 other"], [".cursor/plans/a.md", ".agents/x.md", "docs/readme.md"], [".cursor/plans", ".agents", "other staged paths"]),
 (["🚀 ui work"], ["x/ui/y.ts"], ["product"]), (["🚀 other"], ["s/y.ts"], ["product"]),
]
staged_defect = [(["🛠" + V + "framework tweak"], ["🧰" + V + "framework/x/y.ts"], [])]
ticket = {"staged": {".🧬semio/🦑️repo/🎫️tickets/a/🎫️ticket.json": '{"emoji":"🧪️","title":"First"}', ".🧬semio/🦑️repo/🎫️tickets/b/🎫️ticket.json": '{"emoji":"🚀","title":"Second"}', ".🧬semio/🦑️repo/🎫️tickets/c/🎫️ticket.json": "{broken", ".🧬semio/🦑️repo/🎫️tickets/d/🎫️ticket.json": '{"title":"NoEmoji"}', ".🧬semio/🦑️repo/🎫️tickets/e/other.json": '{"emoji":"🧪️","title":"Other"}'}, "expected": ["🧪️First", "🚀Second"]}
prepared = [["🧪" + V + "a", "🚀b"], []]
corpus = {"version": 1, "lead": [{"line": a, "expected": b} for a, b in lead], "format": [{"line": a, "expected": b} for a, b in fmt],
 "normalize": {"text": norm_text, "expected": norm_expected, "capText": many, "capExpected": ["🚀item%d" % i for i in range(8)]},
 "reserved": [{"lead": k, "bullet": v} for k, v in reserved.items()], "validate": [{"bullets": b, "reserved": e == "reserved"} for b, e in valid],
 "tokens": [{"path": p, "expected": e} for p, e in tokens], "staged": [{"bullets": b, "staged": s, "expected": e} for b, s, e in STAGED],
 "stagedDefects": [{"bullets": b, "staged": s, "expected": e, "defect": "🔸️bullets/🟦️.ts:pathTokensForBulletCoverage keeps the leading emoji of an emoji-named directory in the token (`🧰️framework`), so a bullet that names the area in words never covers the product area in this repository"} for b, s, e in staged_defect],
 "tickets": ticket, "prepared": prepared}
schema = S.obj({"version": S.const(1), "lead": S.arr(S.obj({"line": S.str, "expected": S.str}), 1), "format": S.arr(S.obj({"line": S.str, "expected": S.str}), 1),
 "normalize": S.obj({"text": S.str, "expected": S.strs(), "capText": S.str, "capExpected": S.strs()}), "reserved": S.arr(S.obj({"lead": S.str, "bullet": S.str}), 1),
 "validate": S.arr(S.obj({"bullets": S.strs(), "reserved": S.bool}), 1), "tokens": S.arr(S.obj({"path": S.str, "expected": S.strs()}), 1),
 "staged": S.arr(S.obj({"bullets": S.strs(), "staged": S.strs(), "expected": S.strs()}), 1), "stagedDefects": S.arr(S.obj({"bullets": S.strs(), "staged": S.strs(), "expected": S.strs(), "defect": S.str}), 1),
 "tickets": S.obj({"staged": S.map(S.str), "expected": S.strs()}), "prepared": S.arr(S.strs(), 1)})
bodyts = '''const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const scratch=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"bullets-")));git(root,"init","-q");return root;};
for(const row of corpus.lead)test("lead emoji of "+JSON.stringify(row.line),()=>{expect(bulletLeadEmoji(row.line)).toBe(row.expected);});
for(const row of corpus.format)test("bullet format "+JSON.stringify(row.line),()=>{expect(formatMicroCommitBulletLine(row.line)).toBe(row.expected);});
test("bullet text normalizes to emoji-first lines without headings, metrics and one-character lines",()=>{expect(normalizeBulletLines(corpus.normalize.text)).toEqual(corpus.normalize.expected);expect(normalizeBulletLines(corpus.normalize.capText)).toEqual(corpus.normalize.capExpected);});
test("the bullet pattern wants an emoji directly followed by text",()=>{for(const text of["🧪️x","🚀y","👩\\u200d💻z"])expect(MICRO_COMMIT_BULLET_RE.test(text)).toBe(true);for(const text of["plain","-🚀x","🚀"])expect(MICRO_COMMIT_BULLET_RE.test(text)).toBe(false);});
test.failing("a space after an emoji with a variation selector is refused",()=>{expect(MICRO_COMMIT_BULLET_RE.test("🧪️ x")).toBe(false);});
for(const row of corpus.reserved)test("reserved lead emoji "+JSON.stringify(row.lead),()=>{expect(bulletEmojiValidationError([row.bullet])).toContain(row.lead);});
for(const[index,row]of corpus.validate.entries())test("emoji validation "+index,()=>{const error=bulletEmojiValidationError(row.bullets);if(row.reserved)expect(error).toContain("reserved");else expect(error).toBeNull();});
for(const row of corpus.tokens)test("coverage tokens of "+row.path,()=>{expect(pathTokensForBulletCoverage(row.path)).toEqual(row.expected);});
for(const[index,row]of corpus.staged.entries())test("uncovered areas "+index+": "+JSON.stringify(row.staged),()=>{expect(uncoveredStagedAreas(row.bullets,row.staged)).toEqual(row.expected);});
for(const[index,row]of corpus.stagedDefects.entries())test.failing("uncovered areas defect "+index,()=>{expect(uncoveredStagedAreas(row.bullets,row.staged)).toEqual(row.expected);});
test("prepared bullets round-trip through the git directory and normalize on read",()=>{
 const root=scratch();
 try{
  expect(readPreparedBullets(root)).toEqual([]);
  for(const bullets of corpus.prepared){writePreparedBullets(root,bullets);expect(readPreparedBullets(root)).toEqual(bullets);}
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("staged added ticket files become emoji-title bullets",()=>{
 const root=scratch();
 try{
  for(const[path,text]of Object.entries(corpus.tickets.staged)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
  git(root,"add","-A");expect(listCachedPaths(root).length).toBe(Object.keys(corpus.tickets.staged).length);
  expect(listAddedTicketPaths(root).sort()).toEqual(Object.keys(corpus.tickets.staged).filter(path=>path.endsWith("ticket.json")).sort());
  expect(ticketBullets(root).sort()).toEqual([...corpus.tickets.expected].sort());
 }finally{rmSync(root,{recursive:true,force:true});}
});'''
ts = lib.ts_wrap(UNIT, "🔸️ Contract: commit bullets keep their leading emoji, normalize authored text, refuse reserved emojis, cover the staged areas and travel through the git directory; the corpus is language-agnostic and `@@O@@` classifies emoji by Unicode range and recomputes every verdict.",
 'import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {MICRO_COMMIT_BULLET_RE,bulletEmojiValidationError,bulletLeadEmoji,formatMicroCommitBulletLine,listAddedTicketPaths,listCachedPaths,normalizeBulletLines,pathTokensForBulletCoverage,readPreparedBullets,ticketBullets,uncoveredStagedAreas,writePreparedBullets} from "@@' + UNIT + '/🟦.ts@@";', bodyts)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent model of the commit bullets: emoji clusters by Unicode range, bullet normalisation, reserved lead emojis and staged-area coverage (stdlib only).") + PY_EMOJI + r'''
SPACE = " \t\n\r\x0b\x0c   ﻿"


def fmt(line):
    body = line.strip()
    body = re.sub(r"^-+\s*", "", body)
    lead_ = lead_cluster_seq(body)
    if lead_ and len(body) > len(lead_) and body[len(lead_)].isspace():
        return lead_ + body[len(lead_):].lstrip()
    return body


def normalize(text):
    out = []
    for l in text.split("\n"):
        l = l.strip()
        if l and not l.startswith("#") and not l.startswith("📊" + VS + "metric"):
            out.append(fmt(l))
    return [l for l in out if len(l.encode("utf-16-le")) // 2 > 1][:8]


for c in corpus["lead"]:
    if lead_cluster_seq(c["line"].strip()) != c["expected"]:
        failures.append("lead %r" % c["line"])
for c in corpus["format"]:
    if fmt(c["line"]) != c["expected"]:
        failures.append("format %r" % c["line"])
n = corpus["normalize"]
if normalize(n["text"]) != n["expected"] or normalize(n["capText"]) != n["capExpected"]:
    failures.append("normalize")
RESERVED = {"🎆" + VS, "📊" + VS, "🔢" + VS, "🚩" + VS}
for c in corpus["validate"]:
    got = any(lead_cluster_seq(b.strip()) in RESERVED for b in c["bullets"])
    if got != c["reserved"]:
        failures.append("validate %r" % c["bullets"])
for c in corpus["tokens"]:
    seen, toks = set(), []
    for t in re.split(r"[/._-]+", c["path"].lower()):
        if len(t) >= 4 and t not in seen:
            seen.add(t)
            toks.append(t)
    if toks != c["expected"]:
        failures.append("tokens %s" % c["path"])
AREAS = [(".cursor/plans", lambda p: ".cursor/plans" in p, ["plan"]), (".agents", lambda p: ".agents" in p and not p.endswith("SKILL.md"), ["skill", "agent"]), ("repo", lambda p: "repo/" in p, ["hook", "micro-commit"]), (".devcontainer", lambda p: ".devcontainer" in p, ["devcontainer"]), ("product", lambda p: re.search(r"(^|/)(framework|puzzle|compose|cad|ui|mathematical|infinite|elements|coda|reuse|s)/", re.sub(r"^[^\w./-]+", "", p)) is not None, [])]


def tokens_of(p):
    return {t for t in re.split(r"[/._-]+", p.lower()) if len(t) >= 4}


def uncovered(bullets, staged):
    sig = [p for p in staged if not (re.search(r"/micro-commit\.ts$", p) or re.search(r"/index\.test\.ts$", p) or p.endswith("SKILL.md"))]
    if not sig:
        return []
    text = "\n".join(bullets).lower()
    missed, matched = [], set()
    for ident, match, kws in AREAS:
        files = [p for p in sig if match(p)]
        matched.update(files)
        if files and not (any(k in text for k in kws) or any(t in text for p in files for t in tokens_of(p))):
            missed.append(ident)
    other = [p for p in sig if p not in matched]
    if other and not any(t in text for p in other for t in tokens_of(p)):
        missed.append("other staged paths")
    return missed


for c in corpus["staged"] + corpus["stagedDefects"]:
    got = uncovered(c["bullets"], c["staged"])
    if "defect" in c:
        continue
    if got != c["expected"]:
        failures.append("staged %r" % c["staged"])
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, oracle)
