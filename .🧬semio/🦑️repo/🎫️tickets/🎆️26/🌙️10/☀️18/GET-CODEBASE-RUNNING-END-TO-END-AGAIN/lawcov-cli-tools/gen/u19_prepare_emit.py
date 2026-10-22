import re, lib
from lib import S
V = "️"
DATE = lambda yy, mm, dd: "🎆" + V + yy + "🌙" + V + mm + "☀" + V + dd
D21 = DATE("26", "10", "21")
SPLIT, FLAG = "🔀" + V, "🚩" + V
# ---------- prepare-commands
UP = "🎛commands/📝commit/📋prepare-commands"
tags = ["🐙" + V + "ueli" + D21 + FLAG, "plain", "it's quoted", "a b", "x$(y)`z`", "", "'", "\\ back"]
cases = [{"tagName": t, "wipSha": "abc1234", "messageFile": m} for t in tags[:3] for m in (None, ".git/my message.txt", "it's.txt")]
cases = [{k: v for k, v in c.items() if v is not None} for c in cases]
CODE = chr(96) * 3
def qq(s): return "'" + s.replace("'", "'\"'\"'") + "'"
def render(c):
    msg = c.get("messageFile", ".git/compose-commit-message")
    tag = "git tag -s -m %s %s HEAD" % (qq(c["tagName"]), qq(c["tagName"]))
    squash = "git reset --soft %s && git commit -S -F %s" % (c["wipSha"], qq(msg))
    allc = "%s && git reset --soft %s && git commit -S -F %s && git push --follow-tags" % (tag, c["wipSha"], qq(msg))
    return "\n\n".join("%s\n%s\n%s" % (CODE, x, CODE) for x in (tag, squash, "git push --follow-tags", allc)) + "\n"
for c in cases:
    c["expected"] = render(c)
corpus_p = {"version": 1, "tags": tags, "cases": cases, "reply": {"tagName": "  " + tags[0] + " ", "wipSha": "f00", "commitMessage": "line one\nline two\n\n", "messageFile": "m.txt"}}
schema_p = S.obj({"version": S.const(1), "tags": S.strs(1), "cases": S.arr(S.obj({"tagName": S.str, "wipSha": S.str, "messageFile": S.str, "expected": S.str}, required=["tagName", "wipSha", "expected"]), 1), "reply": S.obj({"tagName": S.str, "wipSha": S.str, "commitMessage": S.str, "messageFile": S.str})})
body_p = '''test("the signed tag command quotes the name as one shell word, twice",()=>{
 for(const tag of corpus.tags){const command=formatGitSignedTagCommand(tag);expect(command.startsWith("git tag -s -m ")).toBe(true);expect(command.endsWith(" HEAD")).toBe(true);}
 expect(formatGitSignedTagCommand("v1","abc")).toBe("git tag -s -m 'v1' 'v1' abc");expect(formatGitSignedTagCommand("it's")).toBe("git tag -s -m 'it'\\"'\\"'s' 'it'\\"'\\"'s' HEAD");
});
for(const[index,row]of corpus.cases.entries())test("prepare commands "+index+": "+JSON.stringify(row.tagName),()=>{
 const text=formatCommitPrepareCommands({tagName:row.tagName,wipSha:row.wipSha,messageFile:(row as {messageFile?:string}).messageFile}),blocks=text.trimEnd().split("\\n\\n");
 expect(text).toBe(row.expected);expect(blocks.length).toBe(4);expect(text.endsWith("\\n")).toBe(true);for(const block of blocks){expect(block.startsWith("```\\n")).toBe(true);expect(block.endsWith("\\n```")).toBe(true);}
 const lines=blocks.map(block=>block.slice(4,-4));expect(lines[0]).toBe(formatGitSignedTagCommand(row.tagName));expect(lines[1]!.startsWith("git reset --soft "+row.wipSha+" && git commit -S -F ")).toBe(true);expect(lines[2]).toBe("git push --follow-tags");
 expect(lines[3]).toBe(lines[0]+" && "+lines[1]+" && "+lines[2]);
});
test("the agent reply is the four blocks, the tag name block and the message block",()=>{
 const reply=formatCommitPrepareAgentReply(corpus.reply),commands=formatCommitPrepareCommands({tagName:corpus.reply.tagName,wipSha:corpus.reply.wipSha,messageFile:corpus.reply.messageFile});
 expect(reply.startsWith(commands)).toBe(true);expect(reply.slice(commands.length)).toBe("```\\n"+corpus.reply.tagName.trim()+"\\n```\\n\\n```\\nline one\\nline two\\n```\\n");
});'''
ts_p = lib.ts_wrap(UP, "📋️ Contract: the prepare commands render the signed tag, squash, push and all-in-one blocks with every operand single-quoted for the shell, and the agent reply appends the tag name and the message; the corpus is language-agnostic and `@@O@@` recovers the operands with `shlex` as the independent shell oracle.",
 'import {formatCommitPrepareAgentReply,formatCommitPrepareCommands,formatGitSignedTagCommand} from "@@' + UP + '/🟦.ts@@";', body_p)
orc_p = lib.PY_HEAD.format(doc="🐍️ Shell oracle of the prepare commands: tokenises every rendered command with `shlex` and checks that the operands come back exactly (stdlib only; the Bun test passes the corpus, this model renders its own commands and parses both).") + r'''import shlex

CODE = chr(96) * 3


def q(s):
    return "'" + s.replace("'", "'\"'\"'") + "'"


def tag_command(name, head="HEAD"):
    return "git tag -s -m %s %s %s" % (q(name), q(name), head)


for c in corpus["cases"]:
    msg = c.get("messageFile", ".git/compose-commit-message")
    tag = tag_command(c["tagName"])
    squash = "git reset --soft %s && git commit -S -F %s" % (c["wipSha"], q(msg))
    allc = "%s && git reset --soft %s && git commit -S -F %s && git push --follow-tags" % (tag, c["wipSha"], q(msg))
    text = "\n\n".join("%s\n%s\n%s" % (CODE, x, CODE) for x in (tag, squash, "git push --follow-tags", allc)) + "\n"
    first = shlex.split(tag)
    if first != ["git", "tag", "-s", "-m", c["tagName"], c["tagName"], "HEAD"]:
        failures.append("shlex tag %r" % c["tagName"])
    if shlex.split(squash)[-1] != msg:
        failures.append("shlex message %r" % msg)
    if text != c["expected"]:
        failures.append("render %r" % c["tagName"])
    for line in c["expected"].split("\n"):
        if line.startswith("git tag") and shlex.split(line)[4] != c["tagName"]:
            failures.append("expected tag operand %r" % c["tagName"])
for t in corpus["tags"]:
    if shlex.split(tag_command(t))[4:6] != [t, t]:
        failures.append("roundtrip %r" % t)
''' + lib.PY_TAIL
lib.write_unit(UP, corpus_p, schema_p, ts_p, orc_p)
# ---------- emit
UE = "🎛commands/📝commit/📤emit"
comm = [
 {"subject": "start", "body": ""},
 {"subject": "🐙" + V + "ueli" + D21 + SPLIT, "body": ""},
 {"subject": "work one", "body": "🚀 shipped the first feature\n" + D21 + "⏰" + V + "10\nshort\n📊" + V + "metric📃uloc\nSigned-off-by: a <a@b.c>\n  Mixed   CASE   Line Here  \n"},
 {"subject": "work two", "body": "🧪 tests for the parser module\n"},
]
history = ["🚀 shipped the first feature", "mixed case line here", "🧪 tests for the parser module"]
excluded = ["short", D21 + "⏰" + V + "10", "📊" + V + "metric📃uloc", "Signed-off-by: a <a@b.c>", "🐙" + V + "ueli" + D21 + SPLIT]
bullets = [("🚀 shipped the first feature", True), ("  🚀   SHIPPED the   first Feature ", True), ("🚀 shipped the second feature", False), ("short", False), ("Signed-off-by: a <a@b.c>", False), ("", False)]
fresh = {"ok": [{"label": "🏘" + V + "x✍" + V + "y", "dates": [{"dateLine": D21, "bullets": ["🔥 brand new bullet text"]}]}], "copy": [{"label": "🏘" + V + "x✍" + V + "y", "dates": [{"dateLine": D21, "bullets": ["🔥 brand new bullet text", "🧪 TESTS for the   parser module"]}]}]}
corpus_e = {"version": 1, "commits": comm, "history": history, "excludedLines": excluded, "bullets": [{"bullet": b, "expected": e} for b, e in bullets], "fresh": fresh, "expectedRange": {"subject": comm[1]["subject"], "index": 1}, "noWip": ["a", "b"], "copyMessage": "commit: bullet copies a prior commit message line"}
bs = S.obj({"label": S.str, "dates": S.arr(S.obj({"dateLine": S.str, "bullets": S.strs()}))})
schema_e = S.obj({"version": S.const(1), "commits": S.arr(S.obj({"subject": S.str, "body": S.str}), 2), "history": S.strs(), "excludedLines": S.strs(), "bullets": S.arr(S.obj({"bullet": S.str, "expected": S.bool}), 1), "fresh": S.obj({"ok": S.arr(bs), "copy": S.arr(bs)}), "expectedRange": S.obj({"subject": S.str, "index": S.int}), "noWip": S.strs(), "copyMessage": S.str})
body_e = '''const git=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-c","commit.gpgsign=false","-c","core.hooksPath="+(process.platform==="win32"?"NUL":"/dev/null"),"-c","user.name=t","-c","user.email=t@example.com",...args],{cwd,env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"}});if(result.exitCode!==0)throw Error(args.join(" ")+": "+result.stderr.toString());return result.stdout.toString().trim();};
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const replay=(commits:{subject:string;body:string}[]):{root:string;shas:string[]}=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"emit-")));roots.push(root);git(root,"init","-q");const shas:string[]=[];for(const commit of commits){git(root,"commit","-q","--allow-empty","-m",commit.subject,...(commit.body?["-m",commit.body]:[]));shas.push(git(root,"rev-parse","HEAD"));}return{root,shas};};
const captured=(action:()=>void):string=>{const lines:string[]=[],original=console.error;console.error=(...parts:unknown[])=>{lines.push(parts.join(" "));};try{action();}finally{console.error=original;}return lines.join("\\n");};
const world=replay(corpus.commits);
test("the range starts at the newest bundle subject",()=>{
 const resolved=resolveCommitBundleRange(world.root);expect(resolved).not.toBeNull();expect(resolved!.wip.sha).toBe(world.shas[corpus.expectedRange.index]);expect(resolved!.wip.subject).toBe(corpus.expectedRange.subject);expect(resolved!.range).toBe(world.shas[corpus.expectedRange.index]+"..HEAD");
 const bare=replay(corpus.noWip.map(subject=>({subject,body:""})));expect(resolveCommitBundleRange(bare.root)).toBeNull();
});
test("prior commit bodies yield normalised comparable lines without bookkeeping lines",()=>{
 const lines=commitHistoryCompareLines(world.root,world.shas[0]!,"HEAD");expect([...lines].sort()).toEqual([...corpus.history].sort());
 for(const text of corpus.excludedLines)expect(lines.has(text.trim().replace(/\\s+/gu," ").toLowerCase())).toBe(false);
 expect(commitHistoryCompareLines(world.root,"no-such-revision","HEAD").size).toBe(0);
});
for(const row of corpus.bullets)test("copy detection of "+JSON.stringify(row.bullet),()=>{expect(bulletMatchesCommitHistory(row.bullet,new Set(corpus.history))).toBe(row.expected);});
test("fresh bullets pass and a copied bullet is refused",()=>{
 expect(()=>validateBundleBulletsFresh(world.root,world.shas[0]!,"HEAD",corpus.fresh.ok)).not.toThrow();expect(()=>validateBundleBulletsFresh(world.root,world.shas[0]!,"HEAD",corpus.fresh.copy)).toThrow(corpus.copyMessage);
});
test("the analysis streams name the last bundle, the range and the diff",()=>{
 const log=captured(()=>emitCommitLog(world.root));expect(log).toContain("Last bundle WIP: "+corpus.expectedRange.subject+" ("+world.shas[corpus.expectedRange.index]!.slice(0,7)+")");expect(log).toContain("Range: "+world.shas[corpus.expectedRange.index]+"..HEAD");expect(log).toContain("prior commit messages (context only");for(const line of corpus.history.slice(0,1))expect(log).toContain(line);
 const diff=captured(()=>emitCommitDiff(world.root));expect(diff).toContain("=== git diff --stat (overview) ===");expect(diff).toContain("=== git diff (write bullets from this");expect(diff).toContain("(empty diff)");
 const both=captured(()=>emitCommitAnalysis(world.root));expect(both).toContain("prior commit messages");expect(both).toContain("git diff --stat");
});
test("a large diff is truncated at the byte ceiling with a notice",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"emit-big-")));roots.push(root);git(root,"init","-q");git(root,"commit","-q","--allow-empty","-m",corpus.commits[1]!.subject);
 writeFileSync(join(root,"big.txt"),("x".repeat(99)+"\\n").repeat(16000));git(root,"add","-A");git(root,"commit","-q","-m","big");
 const text=captured(()=>emitCommitDiff(root));expect(text).toContain("[commit diff truncated at 1500000 bytes of ");
});'''
ts_e = lib.ts_wrap(UE, "📤️ Contract: the commit analysis resolves the range from the newest bundle subject, lists comparable lines of prior commit bodies (without subjects, days, metrics and sign-offs), refuses a bullet that copies one of them case- and whitespace-insensitively and prints log and diff to stderr with the truncation notice; the corpus is language-agnostic and `@@O@@` replays the commits with the git CLI and recomputes the comparable lines.",
 'import {afterAll} from "bun:test";\nimport {mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";\nimport {tmpdir} from "node:os";\nimport {bulletMatchesCommitHistory,commitHistoryCompareLines,emitCommitAnalysis,emitCommitDiff,emitCommitLog,resolveCommitBundleRange,validateBundleBulletsFresh} from "@@' + UE + '/🟦.ts@@";', body_e)
orc_e = lib.PY_HEAD.format(doc="🐍️ Independent model of the commit history comparison: replays the commits with the git CLI and normalises the bodies (stdlib + git).") + r'''import os, subprocess, tempfile, shutil

DAY = "🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2}"
WIP = re.compile("^(.+" + DAY + ")🔀" + VS + "$")


def git(cwd, *a):
    env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "-c", "user.name=t", "-c", "user.email=t@e.x"] + list(a), cwd=cwd, env=env, check=True, capture_output=True, text=True).stdout


def norm(s):
    return re.sub(r"\s+", " ", s.strip()).lower()


root = tempfile.mkdtemp(prefix="emit-oracle-")
try:
    git(root, "init", "-q")
    shas = []
    for c in corpus["commits"]:
        a = ["commit", "-q", "--allow-empty", "-m", c["subject"]] + (["-m", c["body"]] if c["body"] else [])
        git(root, *a)
        shas.append(git(root, "rev-parse", "HEAD").strip())
    subjects = git(root, "log", "--format=%H%x09%s").strip().split("\n")
    wip = next(((h, s) for h, s in (x.split("\t", 1) for x in subjects) if WIP.match(s)), None)
    if wip is None or wip[0] != shas[corpus["expectedRange"]["index"]]:
        failures.append("range")
    lines = set()
    for body in git(root, "log", "--format=%B%x00", shas[0] + "..HEAD").split("\0"):
        for raw in body.split("\n"):
            l = raw.strip()
            if len(l) < 12 or WIP.match(l) or l.startswith("📊" + VS + "metric") or l.startswith("Signed-off-by:") or re.match("^" + DAY + "⏰" + VS, l):
                continue
            k = re.sub(r"^(" + DAY + ")", r"\1", l)
            if re.match("^" + DAY + "$", l.split("📊")[0].strip()):
                continue
            lines.add(norm(l))
    if sorted(lines) != sorted(corpus["history"]):
        failures.append("history")
    for c in corpus["bullets"]:
        if (norm(c["bullet"]) in set(corpus["history"])) != c["expected"]:
            failures.append("bullet %r" % c["bullet"])
    for kind, expect_copy in (("ok", False), ("copy", True)):
        hit = any(norm(b) in lines for bundle in corpus["fresh"][kind] for d in bundle["dates"] for b in d["bullets"])
        if hit != expect_copy:
            failures.append("fresh " + kind)
finally:
    shutil.rmtree(root, ignore_errors=True)
''' + lib.PY_TAIL
lib.write_unit(UE, corpus_e, schema_e, ts_e, orc_e)
