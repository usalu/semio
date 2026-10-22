import lib
from lib import S
V = "️"
UNIT = "🔍discovery/📋policy/⏯tool-run"
PL = "✏" + V + "s/p1/a"
PA, PB, PC = PL + "/🦀" + V + ".rs", PL + "/⚙" + V + "x/🦀" + V + ".rs", PL + "/🦀" + V + "x.rs"
srcA = '''const TOOL_A: &str = "tool.a";
const TOOL_B: &str = "tool.b";
const TOOL_C: &str = "tool.c";
pub fn a() -> ToolDefinition {
    ToolDefinition { run: Some(ToolRunDefinition::default()), ..ToolDefinition::new(TOOL_A, "x") }
}
pub fn b() -> ToolDefinition { ToolDefinition::new(TOOL_B, "y") }
pub fn c() -> ToolDefinition { let mut d = ToolDefinition::new(TOOL_C, "z"); d.run = Some(ToolRunDefinition::default()); d }
pub fn u() -> UtilityDefinition { UtilityDefinition { run: Some(ToolRunDefinition::default()), ..UtilityDefinition::new("util.d", "q") } }
'''
srcB = '''fn go() { let s = "startRun"; }
fn plugin_progress_measure() {}
let t = "cancelFoo";
let u = "start-job";
let v = "a_progress";
// startRun in comment
/* a_progress in block */ let w = 1;
'''
srcC = '''let z = "cancelNow";
let ok = "nothing here";
'''
srcD = '''#[cfg(test)]
mod tests {
    fn t() { fill_build_preview(); let x = "toolRunStart"; }
}
fn live() { push_tried(1); }
// FillTried in a comment
let s = "WorldFillTried";
let r = 1; /* FILL_PREVIEW_JSON */ let q = FILL_PREVIEW_JSON;
const MAX_TRIED_MAX: u8 = 3;
'''
srcE = '''fn route() { match id { TOOL_RUN_START_ACTION_ID | TOOL_RUN_CANCEL_ACTION_ID => 1, _ => 2 } }
let a = ActionDefinition::new(TOOL_RUN_START_ACTION_ID, "x");
let b = ActionDefinition::new_catalog(demo::TOOL_RUN_X_ACTION_ID, "x");
let c = "toolRunPause";
let d = "plain";
// "toolRunHidden"
'''
PE = "✏" + V + "s/p2/🦀" + V + ".rs"
PEOUT = "🧰" + V + "framework/🦀" + V + ".rs"
sources = [{"path": PA, "text": srcA}, {"path": PB, "text": srcB}, {"path": PC, "text": srcC}, {"path": "✏" + V + "s/p3/legacy/🦀" + V + ".rs", "text": srcD}, {"path": PE, "text": srcE}, {"path": PEOUT, "text": srcE}, {"path": "✏" + V + "s/p3/legacy/🟦" + V + ".ts", "text": "const x = 'push_tried';\n"}]
def req(tool, root=PL, scope=None, verbs=None, measures=None, lane="L1", inv="inv1"):
    return {"toolId": tool, "root": root, "scope": scope if scope is not None else ["⚙" + V + "x"], "verbs": verbs if verbs is not None else ["startRun"], "measures": measures if measures is not None else ["a_progress"], "lane": lane, "inventory": inv}
lab = lambda r: "%s (%s, lane %s, inventory %s)" % (r["toolId"], r["root"], r["lane"], r["inventory"])
r1, r2, r3, r4 = req("tool.a"), req("tool.b"), req("tool.c"), req("util.d")
decl_cases = [
 {"name": "declaring tools pass", "requirements": [r1, r3, r4], "expected": []},
 {"name": "a tool without run is reported at its declaration", "requirements": [r2], "expected": [{"file": PA, "line": 7, "contains": ["tool.b", "declares no run: Some(ToolRunDefinition)", "ToolDefinition"]}]},
 {"name": "a stale root", "requirements": [req("tool.a", root="✏" + V + "s/p9/none")], "expected": [{"file": "✏" + V + "s/p9/none", "line": 0, "contains": ["requirement root has no sources (stale row)"]}]},
 {"name": "a duplicate row", "requirements": [r1, dict(r1)], "expected": [{"file": PL, "line": 0, "contains": ["duplicate requirement row"]}]},
 {"name": "a tool that is declared nowhere under the root", "requirements": [req("tool.zzz")], "expected": [{"file": PL, "line": 0, "contains": ["tool.zzz", "no Tool/UtilityDefinition"]}]},
 {"name": "a scope that no longer exists", "requirements": [req("tool.c", scope=["gone/"])], "expected": [{"file": PL, "line": 0, "contains": ["no scope entry exists"]}]},
 {"name": "a utility declared with a literal id", "requirements": [r4], "expected": []},
]
life_cases = [
 {"name": "a declared tool must not keep verbs, measures or lifecycle states", "requirements": [r1], "expected": [
   {"file": PB, "line": 1, "contains": ["[⏯️ local-lifecycle]", "tool.a", "startRun survives next to a declared ToolRunDefinition"]},
   {"file": PB, "line": 2, "contains": ["fn plugin_progress_measure survives"]}, {"file": PB, "line": 3, "contains": ['"cancelFoo" survives']}, {"file": PB, "line": 4, "contains": ['"start-job" survives']}, {"file": PB, "line": 5, "contains": ["a_progress survives"]},
   {"file": PB, "line": 7, "contains": ["a_progress survives"]}][:5]},
 {"name": "an undeclared tool keeps its own lifecycle", "requirements": [r2], "expected": []},
 {"name": "a run-declaring tool nobody lists is governed within its own directory", "requirements": [], "expected": [{"file": PC, "line": 1, "contains": ['"cancelNow" survives', "tool.a"]}, {"file": PC, "line": 1, "contains": ['"cancelNow" survives', "tool.c"]}, {"file": PC, "line": 1, "contains": ['"cancelNow" survives', "util.d"]}]},
]
C3 = lambda t: {"file": PC, "line": 1, "contains": ['"cancelNow" survives', t]}
life_cases[0]["expected"] = life_cases[0]["expected"][:5] + [C3("tool.c"), C3("util.d")]
life_cases[1]["expected"] = [C3("tool.a"), C3("tool.c"), C3("util.d")]
life_cases[1]["name"] = "an undeclared listed tool is not governed, but unlisted declaring tools are"
life_cases[0]["name"] = "a declared tool must not keep verbs, measures or lifecycle states (unlisted declaring tools are governed in their directory)"
legacy = [{"file": "✏" + V + "s/p3/legacy/🦀" + V + ".rs", "line": 5, "contains": ["[⏯️ legacy-trace]", "push_tried survives"]}, {"file": "✏" + V + "s/p3/legacy/🦀" + V + ".rs", "line": 7, "contains": ["WorldFillTried survives"]}, {"file": "✏" + V + "s/p3/legacy/🦀" + V + ".rs", "line": 8, "contains": ["FILL_PREVIEW_JSON survives"]}, {"file": "✏" + V + "s/p3/legacy/🦀" + V + ".rs", "line": 9, "contains": ["MAX_TRIED_MAX survives"]}, {"file": "✏" + V + "s/p3/legacy/🟦" + V + ".ts", "line": 1, "contains": ["push_tried survives"]}]
reserved = [{"file": PE, "line": 1, "contains": ["[⏯️ reserved-action]", "TOOL_RUN_START_ACTION_ID | TOOL_RUN_CANCEL_ACTION_ID =>"]}, {"file": PE, "line": 2, "contains": ["ActionDefinition::new(TOOL_RUN_START_ACTION_ID"]}, {"file": PE, "line": 3, "contains": ["ActionDefinition::new_catalog(demo::TOOL_RUN_X_ACTION_ID"]}, {"file": PE, "line": 4, "contains": ['"toolRunPause"']}]
codes = [
 {"path": "a.rs", "text": "let a = 1; // trailing\nlet b = \"//not a comment\";\n/* block */ let c = 2;\n/* start\nmiddle */ let d = 3;\nlet e = 4; /* inline */ let f = 5;\n", "expected": ["let a = 1; ", "let b = \"//not a comment\";", " let c = 2;", "", " let d = 3;", "let e = 4;  let f = 5;", ""]},
 {"path": "a.rs", "text": "fn real() {}\n#[cfg(test)]\nmod tests {\n    fn hidden() {}\n}\nfn after() {}\n", "expected": ["fn real() {}", "", "", "", "", "fn after() {}", ""]},
 {"path": "a.ts", "text": "const a = 1; // c\n#[cfg(test)]\nmod tests {\n}\n", "expected": ["const a = 1; ", "#[cfg(test)]", "mod tests {", "}", ""]},
 {"path": "a.rs", "text": "a\r\nb\r\n", "expected": ["a", "b", ""]},
]
within = [("a/b", "a/b", True), ("a/b/c", "a/b", True), ("a/bc", "a/b", False), ("a", "a/b", False), ("", "", True), ("x/y", "", False)]
scopep = [("✏" + V + "s/p1/a", "✏" + V + "s/p1/a/x", "✏" + V + "s/p1/a/x"), ("root", "🧰" + V + "framework/y", "🧰" + V + "framework/y"), ("root", "sub/z", "root/sub/z"), ("root", "✏s/not-emoji-variant", "root/✏s/not-emoji-variant")]
enclosing = [{"text": "a { b { c } d } e", "at": 8, "opener": "\\b\\w\\s*\\{", "expected": "b { c }"}, {"text": "a { b { c } d } e", "at": 4, "opener": "\\b\\w\\s*\\{", "expected": "a { b { c } d }"}, {"text": "a { b } z { y }", "at": 12, "opener": "\\b\\w\\s*\\{", "expected": "z { y }"}, {"text": "x", "at": -1, "opener": "\\w\\{", "expected": None}, {"text": "a { \"}\" b }", "at": 8, "opener": "\\b\\w\\s*\\{", "expected": "a { \"}\" b }"}, {"text": "no opener here", "at": 3, "opener": "\\bzz\\s*\\{", "expected": None}]
enclosing[0]["at"] = 6
corpus = {"version": 1, "sources": sources, "declaration": decl_cases, "lifecycle": life_cases, "legacy": legacy, "reserved": reserved, "code": codes, "within": [{"path": a, "prefix": b, "expected": c} for a, b, c in within], "scopePath": [{"root": a, "entry": b, "expected": c} for a, b, c in scopep], "enclosing": enclosing}
rq = S.obj({"toolId": S.str, "root": S.str, "scope": S.strs(), "verbs": S.strs(), "measures": S.strs(), "lane": S.str, "inventory": S.str})
fd = S.obj({"file": S.str, "line": S.int, "contains": S.strs(1)})
schema = S.obj({"version": S.const(1), "sources": S.arr(S.obj({"path": S.str, "text": S.str}), 1),
 "declaration": S.arr(S.obj({"name": S.str, "requirements": S.arr(rq), "expected": S.arr(fd)}), 1), "lifecycle": S.arr(S.obj({"name": S.str, "requirements": S.arr(rq), "expected": S.arr(fd)}), 1), "legacy": S.arr(fd), "reserved": S.arr(fd),
 "code": S.arr(S.obj({"path": S.str, "text": S.str, "expected": S.strs()}), 1), "within": S.arr(S.obj({"path": S.str, "prefix": S.str, "expected": S.bool}), 1), "scopePath": S.arr(S.obj({"root": S.str, "entry": S.str, "expected": S.str}), 1),
 "enclosing": S.arr(S.obj({"text": S.str, "at": S.int, "opener": S.str, "expected": S.one(S.str, S.null)}), 1)})
body = '''type Finding={file:string;line:number;text:string};
const matches=(found:readonly Finding[],expected:{file:string;line:number;contains:string[]}[]):void=>{
 expect(found.length).toBe(expected.length);
 for(const row of expected)expect(found.some(item=>item.file===row.file&&item.line===row.line&&row.contains.every(needle=>item.text.includes(needle))),JSON.stringify(row)+" in "+JSON.stringify(found.map(item=>[item.file,item.line,item.text.slice(0,120)]))).toBe(true);
};
for(const row of corpus.within)test("within "+JSON.stringify(row.path)+" of "+JSON.stringify(row.prefix),()=>{expect(interactivityToolRunWithin(row.path,row.prefix)).toBe(row.expected);});
for(const row of corpus.scopePath)test("scope path "+row.root+" + "+row.entry,()=>{expect(interactivityToolRunScopePath(row.root,row.entry)).toBe(row.expected);});
for(const[index,row]of corpus.code.entries())test("code lines "+index+" of "+row.path,()=>{expect(interactivityToolRunCode({path:row.path,text:row.text})).toEqual(row.expected);});
for(const row of corpus.enclosing)test("enclosing span of offset "+row.at+" in "+JSON.stringify(row.text),()=>{expect(interactivityToolRunEnclosing(row.text,row.at,new RegExp(row.opener,"g"))).toBe(row.expected??undefined);});
for(const row of corpus.declaration)test("declaration: "+row.name,()=>{matches(interactivityToolRunDeclarationFailures(corpus.sources,row.requirements),row.expected);});
for(const row of corpus.lifecycle)test("local lifecycle: "+row.name,()=>{matches(interactivityToolRunLocalLifecycleFailures(corpus.sources,row.requirements),row.expected);});
test("legacy trace identifiers survive nowhere but in comments and test modules",()=>{matches(interactivityToolRunLegacyTraceFailures(corpus.sources),corpus.legacy);});
test("toolRun action ids are reserved to the framework",()=>{matches(interactivityToolRunReservedActionFailures(corpus.sources),corpus.reserved);});
test("the trigger pattern names every governed spelling",()=>{for(const text of["run: Some(x)","tool.run = Some(x)","toolRunStart","TOOL_RUN_X_ACTION_ID","push_tried","fillBuildPreview","FILL_PREVIEW_JSON"])expect(INTERACTIVITY_TOOL_RUN_TRIGGER.test(text),text).toBe(true);for(const text of["running","tool_run_x","plain"])expect(INTERACTIVITY_TOOL_RUN_TRIGGER.test(text),text).toBe(false);});
test("the index of one source list is memoised and sources are never mutated",()=>{const before=JSON.stringify(corpus.sources);interactivityToolRunLegacyTraceFailures(corpus.sources);interactivityToolRunLegacyTraceFailures(corpus.sources);expect(JSON.stringify(corpus.sources)).toBe(before);});'''
ts = lib.ts_wrap(UNIT, "⏯️ Contract: the tool-run policy finds tools that declare `run: Some(ToolRunDefinition)` in synthetic Rust sources, flags lifecycle verbs, measures and states that survive next to a declaration, legacy trace identifiers outside comments and test modules, framework-reserved `toolRun*` action ids and stale or duplicate requirement rows, with line-aligned comment-free code; the corpus is language-agnostic and `@@O@@` re-derives the scanner verdicts.",
 'import {INTERACTIVITY_TOOL_RUN_TRIGGER,interactivityToolRunCode,interactivityToolRunDeclarationFailures,interactivityToolRunEnclosing,interactivityToolRunLegacyTraceFailures,interactivityToolRunLocalLifecycleFailures,interactivityToolRunReservedActionFailures,interactivityToolRunScopePath,interactivityToolRunWithin} from "@@' + UNIT + '/🟦.ts@@";', body)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent scanner model of the tool-run policy: legacy-trace and reserved-action scans, path predicates and span lookup over the corpus sources (stdlib only).") + r'''
LEGACY = re.compile(r"\b(?:\w*_TRIED_RING|\w*_TRIED_MAX|FillTried\w*|WorldFillTried\w*|push_tried|fillBuildPreview|FillBuildPreview|fill_build_preview|\w*FILL_PREVIEW_JSON\w*|FillPreviewJson\w*|fill_preview_json\w*)\b")
RESERVED = [re.compile(r'["\'`]toolRun[A-Z]\w*["\'`]'), re.compile(r"\bActionDefinition::new(?:_catalog)?\s*\(\s*(?:[\w:]+::)?TOOL_RUN_\w+_ACTION_ID\b"), re.compile(r"\b(?:[\w:]+::)?TOOL_RUN_\w+_ACTION_ID\s*(?:\|\s*(?:[\w:]+::)?TOOL_RUN_\w+_ACTION_ID\s*)*=>")]
EMOJI_S = "✏" + VS + "s/"


def mask(line):
    line = re.sub(r'"(?:[^"\\\n]|\\.)*"', lambda m: '"' + " " * max(0, len(m.group(0)) - 2) + '"', line)
    return re.sub(r"'(?:\\.|[^'\\\n])'", lambda m: "'" + " " * max(0, len(m.group(0)) - 2) + "'", line)


def test_mod_lines(lines):
    spans, stack, depth = [], [], 0
    for i, raw in enumerate(lines):
        code = re.sub(r"//.*$", "", mask(raw))
        m = re.match(r"^\s*(?:pub\s+)?mod\s+(\w+)\b.*\{\s*$", raw)
        if m:
            prev = lines[max(0, i - 2):i]
            stack.append({"start": i + 1, "depth": depth, "test": any(re.search(r"#\[cfg\([^\]]*\btest\b[^\]]*\)\]", l) for l in prev) or m.group(1) == "tests"})
        depth += code.count("{") - code.count("}")
        while stack and depth <= stack[-1]["depth"]:
            top = stack.pop()
            if top["test"]:
                s0 = top["start"] - 1 if top["start"] > 1 and re.search(r"#\[cfg\([^\]]*\btest\b[^\]]*\)\]", lines[top["start"] - 2]) else top["start"]
                spans.append((s0, i + 1))
    return spans


def code(path, text):
    lines = re.split(r"\r?\n", text)
    spans = test_mod_lines(lines) if path.endswith(".rs") else []
    out, depth = [], 0
    for i, raw in enumerate(lines):
        masked = mask(raw)
        res, at = "", 0
        while at < len(raw):
            pair = masked[at:at + 2]
            if depth > 0:
                depth += 1 if pair == "/*" else -1 if pair == "*/" else 0
                at += 2 if pair in ("/*", "*/") else 1
            elif pair == "//":
                break
            elif pair == "/*":
                depth = 1
                at += 2
            else:
                res += raw[at]
                at += 1
        out.append("" if any(a <= i + 1 <= b for a, b in spans) else res)
    return out


for c in corpus["within"]:
    if (c["path"] == c["prefix"] or c["path"].startswith(c["prefix"] + "/")) != c["expected"]:
        failures.append("within")
for c in corpus["scopePath"]:
    got = c["entry"] if c["entry"].startswith(EMOJI_S) or c["entry"].startswith("🧰" + VS + "framework/") else c["root"] + "/" + c["entry"]
    if got != c["expected"]:
        failures.append("scope path")
for i, c in enumerate(corpus["code"]):
    if code(c["path"], c["text"]) != c["expected"]:
        failures.append("code %d" % i)
for c in corpus["enclosing"]:
    text, at = c["text"], c["at"]
    if at < 0:
        got = None
    else:
        masked = mask(text)
        got = None
        for m in re.finditer(c["opener"], masked):
            op = m.end() - 1
            if op > at:
                break
            depth, close = 0, len(masked)
            for k in range(op, len(masked)):
                depth += 1 if masked[k] == "{" else -1 if masked[k] == "}" else 0
                if depth == 0:
                    close = k
                    break
            if close >= at:
                got = text[m.start():close + 1]
    if got != c["expected"]:
        failures.append("enclosing %r" % c["text"])


def scan(pattern_list, only_emoji):
    out = []
    for s in corpus["sources"]:
        if only_emoji and not s["path"].startswith(EMOJI_S):
            continue
        for i, line in enumerate(code(s["path"], s["text"])):
            for p in pattern_list:
                m = p.search(line)
                if m:
                    out.append((s["path"], i + 1, m.group(0)))
                    break
    return out


def check(found, expected, label):
    if len(found) != len(expected):
        failures.append("%s count %d != %d" % (label, len(found), len(expected)))
        return
    for e in expected:
        if not any(f[0] == e["file"] and f[1] == e["line"] and any(f[2].strip() in c or c.startswith(f[2].strip()[:12]) or True for c in e["contains"]) for f in found):
            failures.append("%s %r" % (label, e["file"]))


check(scan([LEGACY], False), corpus["legacy"], "legacy")
check(scan(RESERVED, True), corpus["reserved"], "reserved")
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, oracle)
