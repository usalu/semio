import re, lib
from lib import S, resolve, LIBN
from pyemoji import PY_EMOJI
from spec_metrics import body_line
V = "️"
DATE = lambda yy, mm, dd: "🎆" + V + yy + "🌙" + V + mm + "☀" + V + dd
D21, D22 = DATE("26", "10", "21"), DATE("26", "10", "22")
RES = {"split": "🔀" + V, "flag": "🚩" + V, "metric": "📊" + V + "metric", "total": "🔢" + V}
# ---------------- labels
UL = "🎛commands/📝commit/🏷labels"
lead = [("🧪" + V + "🔥x", 2), ("🧪" + V + " x", 1), ("x🧪" + V, 0), ("  🚀🚀🚀 y", 3), ("👩‍💻🚀", 2), ("", 0), ("🏘" + V + "compose✍" + V + "sketchpad", 1)]
anywhere = [("a🧪" + V + "b🚀c", 2), ("no", 0), ("🧪" + V + "🧪" + V, 2), ("👩‍💻 and 🚀", 2), ("", 0)]
norm = [("  🏘" + V + "compose" + RES["metric"] + "📃uloc💯" + V + "1k", "🏘" + V + "compose"), ("🔀" + V + "🏘" + V + "x" + RES["flag"], "🏘" + V + "x"), ("plain", "plain"), ("  spaced  ", "spaced"), ("a" + RES["split"] + "b" + RES["flag"] + "c", "abc"), ("🔀x", "🔀x")]
tokens = [("🏘" + V + "compose✍" + V + "sketchpad", ["compose", "sketchpad"]), ("🥅" + V + "framework", ["framework"]), ("🖱" + V + "ui⚛" + V + "react", ["ui", "react"]), ("🏗" + V + "a-b c2", ["c2"]), ("🏘" + V + "Compose", ["compose"]), ("🏘" + V + "café", ["caf"]), ("🏘" + V + "the repo and uloc keep", ["keep"]),
          ("🏘" + V + "Repo" + RES["metric"] + "📃uloc", ["repo"][:0]), ("", []), ("🏘" + V + "x1 y_z", ["x1"])]
scope_errors = [
 ("a path with a slash", "🏘" + V + "compose/x", "commit: bundle scope must be emoji + area name only — no paths or `|`"), ("a pipe", "🏘" + V + "a|b", "commit: bundle scope must be emoji + area name only — no paths or `|`"),
 ("the bundle subject emoji", "🏘" + V + "compose" + RES["split"], "commit: bundle scope must not include"), ("the tag emoji", "🏘" + V + "compose" + RES["flag"], "commit: bundle scope must not include"),
 ("a metric footer", "🏘" + V + "compose" + RES["metric"], "commit: bundle scope must not include"), ("the total emoji", "🔢" + V + "x", "commit: bundle scope must not include"),
 ("an empty line", "   ", "commit: empty bundle scope after removing reserved emojis"), ("only emojis", "🏘" + V + "🥅" + V, "commit: bundle scope needs an area name after emojis"),
 ("a single emoji with an uppercase slug", "🥅" + V + "Framework", "commit: invalid bundle scope"),
 ("a single emoji with a too short slug", "🥅" + V + "ab", "commit: invalid bundle scope: 🥅" + V + "ab"), ("a single emoji with two words", "🥅" + V + "framework x", "commit: invalid bundle scope"),
 ("a hyphenated slug", "🥅" + V + "frame-work", "commit: invalid bundle scope"),
 ("two emojis and an area", "🏘" + V + "compose✍" + V + "sketchpad", None), ("one emoji and a lowercase slug", "🥅" + V + "framework", None), ("three characters slug", "🥅" + V + "abc", None), ("a 25 character slug", "🥅" + V + "a" * 25, None), ("a 26 character slug", "🥅" + V + "a" * 26, "commit: invalid bundle scope"),
]
scope_line = [("🏘" + V + "compose✍" + V + "sketchpad", True), ("🥅" + V + "framework", True), ("🥅" + V + "abc", True), ("🥅" + V + "a" * 25, True), ("🥅" + V + "a" * 26, False), ("🥅" + V + "ab", False), ("🥅" + V + "Framework", False), ("🥅" + V + "framework x", False), ("🥅" + V + "frame-work", False),
              ("🏘" + V + "compose" + RES["split"], True), ("🏘" + V + "compose" + RES["flag"], True), ("🏘" + V + "compose" + RES["metric"], True), ("🏘" + V + "🥅" + V, True), (RES["total"] + "x", False), (D21, False), ("a/b", False), ("plain words", False), ("", False), ("🏘" + V + "a/b✍" + V + "c", False), ("🏘" + V + "a|b✍" + V + "c", False)]
corpus_l = {"version": 1, "lead": [{"line": a, "expected": b} for a, b in lead], "anywhere": [{"line": a, "expected": b} for a, b in anywhere], "normalize": [{"line": a, "expected": b} for a, b in norm], "tokens": [{"label": a, "expected": b} for a, b in tokens],
            "scopeErrors": [{"name": n, "line": l, "message": m} for n, l, m in scope_errors], "scopeLine": [{"line": a, "expected": b} for a, b in scope_line]}
sch_l = S.obj({"version": S.const(1), "lead": S.arr(S.obj({"line": S.str, "expected": S.int}), 1), "anywhere": S.arr(S.obj({"line": S.str, "expected": S.int}), 1), "normalize": S.arr(S.obj({"line": S.str, "expected": S.str}), 1), "tokens": S.arr(S.obj({"label": S.str, "expected": S.strs()}), 1),
               "scopeErrors": S.arr(S.obj({"name": S.str, "line": S.str, "message": S.one(S.str, S.null)}), 1), "scopeLine": S.arr(S.obj({"line": S.str, "expected": S.bool}), 1)})
body_l = '''for(const row of corpus.lead)test("leading emoji clusters of "+JSON.stringify(row.line),()=>{expect(leadingEmojiClusterCount(row.line)).toBe(row.expected);});
for(const row of corpus.anywhere)test("emoji clusters anywhere in "+JSON.stringify(row.line),()=>{expect(emojiClusterCountInLine(row.line)).toBe(row.expected);});
for(const row of corpus.normalize)test("scope label normalisation of "+JSON.stringify(row.line),()=>{expect(normalizeBundleScopeLabel(row.line)).toBe(row.expected);});
for(const row of corpus.tokens)test("path tokens of "+JSON.stringify(row.label),()=>{expect(labelPathTokens(row.label)).toEqual(row.expected);});
for(const row of corpus.scopeErrors)test("scope label error: "+row.name,()=>{const error=bundleScopeLabelError(row.line);if(row.message===null)expect(error).toBeNull();else expect(error).toContain(row.message);});
for(const row of corpus.scopeLine)test("scope line "+JSON.stringify(row.line),()=>{expect(isBundleScopeLine(row.line)).toBe(row.expected);});'''
ts_l = lib.ts_wrap(UL, "🏷️ Contract: bundle scope labels are emoji clusters plus a lowercase area name; the label module counts clusters, strips reserved emojis, extracts ASCII path tokens and refuses paths, reserved emojis and malformed names with a fixed message; the corpus is language-agnostic and `@@O@@` classifies emoji by Unicode range and recomputes each verdict.",
 'import {bundleScopeLabelError,emojiClusterCountInLine,isBundleScopeLine,labelPathTokens,leadingEmojiClusterCount,normalizeBundleScopeLabel} from "@@' + UL + '/🟦.ts@@";', body_l)
orc_l = lib.PY_HEAD.format(doc="🐍️ Independent model of the bundle scope labels (emoji clusters by Unicode range; stdlib only).") + PY_EMOJI + r'''
RESERVED = ["🔀" + VS, "🚩" + VS, "📊" + VS + "metric", "🔢" + VS]
BLOCK = {"uloc", "repo", "the", "and"}


def clusters(text):
    out, i = [], 0
    while i < len(text):
        c = lead(text[i:])
        if c:
            out.append(c)
            i += len(c)
        else:
            i += 1
    return out


def leading_count(line):
    rest, n = line.strip(), 0
    while True:
        c = lead(rest)
        if not c:
            return n
        n += 1
        rest = rest[len(c):]


def normalize(line):
    s = line.strip()
    k = s.find("📊" + VS + "metric")
    s = s[:k] if k >= 0 else s
    return s.replace("🔀" + VS, "").replace("🚩" + VS, "").strip()


def tokens(label):
    t = normalize(label)
    t = "".join(" " if pict(c) else c for c in t).strip().lower()
    return [x for x in re.split(r"[^a-z0-9]+", t) if len(x) >= 2 and x not in BLOCK]


DATE = re.compile("^🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2}$")


def is_scope(line):
    t = normalize(line)
    if not t or DATE.match(t) or "/" in t or "|" in t or any(r in t for r in RESERVED):
        return False
    n = len(clusters(t))
    if n >= 2:
        return True
    if n != 1:
        return False
    after = t[len(lead(t)):]
    return re.match(r"^[a-z][a-z0-9]{2,24}$", after) is not None


def error(line):
    raw = line.strip()
    if "|" in raw or "/" in raw:
        return "paths"
    if any(r in raw for r in RESERVED):
        return "reserved"
    n = normalize(raw)
    if not n:
        return "empty"
    if not tokens(n):
        return "area"
    if not is_scope(n):
        return "invalid"
    return None


for c in corpus["lead"]:
    if leading_count(c["line"]) != c["expected"]:
        failures.append("lead %r" % c["line"])
for c in corpus["anywhere"]:
    if len(clusters(c["line"])) != c["expected"]:
        failures.append("anywhere %r" % c["line"])
for c in corpus["normalize"]:
    if normalize(c["line"]) != c["expected"]:
        failures.append("normalize %r" % c["line"])
for c in corpus["tokens"]:
    if tokens(c["label"]) != c["expected"]:
        failures.append("tokens %r" % c["label"])
KINDS = {"commit: bundle scope must be emoji + area name only": "paths", "commit: bundle scope must not include": "reserved", "commit: empty bundle scope": "empty", "commit: bundle scope needs an area name": "area", "commit: invalid bundle scope": "invalid"}
for c in corpus["scopeErrors"]:
    want = None if c["message"] is None else next(v for k, v in KINDS.items() if c["message"].startswith(k))
    if error(c["line"]) != want:
        failures.append("error: " + c["name"])
for c in corpus["scopeLine"]:
    if is_scope(c["line"]) != c["expected"]:
        failures.append("scope line %r" % c["line"])
''' + lib.PY_TAIL
lib.write_unit(UL, corpus_l, sch_l, ts_l, orc_l)
# ---------------- dates + bundle-model
UD = "🎛commands/📝commit/📅dates"
subj = [("🐙" + V + "ueli" + D21 + RES["flag"] + "5", D21), ("  " + D21 + "  ", D21), ("no date", None), ("🐙ueli🎆26🌙10☀21", None), ("x" + D21 + "y" + D22, D21), (DATE("26", "1", "21"), None), ("", None)]
bodyd = [("line\n" + D21 + "⏰" + V + "10⌚" + V + "00⏱" + V + "00\nmore", D21), (D21 + "\n" + D22 + "⏰" + V + "1", D22), ("  " + D22 + "⏰" + V + "x  ", D22), ("🐙ueli" + D21 + "⏰" + V + "1", None), ("", None), ("a\nb", None), (D21 + "⏰" + V + "1\n" + D22 + "⏰" + V + "2", D21)]
from_commit = [("s " + D21, D22 + "⏰" + V + "1", D22), ("s " + D21, "none", D21), ("none", "none", None), ("none", D22 + "⏰" + V + "1", D22)]
dnorm = [(D21 + RES["metric"] + "📃uloc💯" + V + "1k  ", D21), ("  " + D21 + "  ", D21), ("plain", "plain"), (D21 + "x" + RES["metric"], D21 + "x")]
fmt = []
for line, ud, sd, ub, sb in [(D21, {"added": 10, "removed": 1, "edited": 2}, {"added": 4096, "removed": 0, "edited": 0}, 800, 120000), (D21 + RES["metric"] + "old", {"added": 0, "removed": 0, "edited": 0}, {"added": 0, "removed": 0, "edited": 0}, 0, 0), (D22, {"added": 0, "removed": 5, "edited": 0}, {"added": 0, "removed": 100, "edited": 0}, 50, 5000)]:
    fmt.append({"line": line, "ulocDelta": ud, "sizeDelta": sd, "ulocBloc": ub, "sizeBloc": sb, "expected": re.sub(r"📊.*$", "", line).strip() + body_line("uloc", ub, ud["added"], ud["edited"], ud["removed"]) + body_line("size", sb, sd["added"], sd["edited"], sd["removed"])})
corpus_d = {"version": 1, "subject": [{"subject": a, "expected": b} for a, b in subj], "body": [{"body": a, "expected": b} for a, b in bodyd], "commit": [{"subject": a, "body": b, "expected": c} for a, b, c in from_commit], "normalize": [{"line": a, "expected": b} for a, b in dnorm], "format": fmt,
            "bundleModel": {"wip": [{"subject": "🐙" + V + "ueli" + D21 + RES["split"], "expected": "🐙" + V + "ueli" + D21}, {"subject": D21 + RES["split"], "expected": None}, {"subject": "🐙" + V + "ueli" + D21 + RES["flag"], "expected": None}, {"subject": "x" + D21 + RES["split"] + " ", "expected": None}, {"subject": "a" + D21 + RES["split"], "expected": "a" + D21}],
                            "date": [{"line": D21, "expected": True}, {"line": D21 + "x", "expected": False}, {"line": " " + D21, "expected": False}, {"line": "🎆26🌙10☀21", "expected": False}, {"line": DATE("26", "10", "2"), "expected": False}]}}
dl = S.obj({"added": S.int, "removed": S.int, "edited": S.int})
sch_d = S.obj({"version": S.const(1), "subject": S.arr(S.obj({"subject": S.str, "expected": S.one(S.str, S.null)}), 1), "body": S.arr(S.obj({"body": S.str, "expected": S.one(S.str, S.null)}), 1), "commit": S.arr(S.obj({"subject": S.str, "body": S.str, "expected": S.one(S.str, S.null)}), 1),
               "normalize": S.arr(S.obj({"line": S.str, "expected": S.str}), 1), "format": S.arr(S.obj({"line": S.str, "ulocDelta": dl, "sizeDelta": dl, "ulocBloc": S.int, "sizeBloc": S.int, "expected": S.str}), 1),
               "bundleModel": S.obj({"wip": S.arr(S.obj({"subject": S.str, "expected": S.one(S.str, S.null)}), 1), "date": S.arr(S.obj({"line": S.str, "expected": S.bool}), 1)})})
body_d = '''for(const row of corpus.subject)test("subject day of "+JSON.stringify(row.subject),()=>{expect(extractBundleDateLineFromSubject(row.subject)).toBe(row.expected);});
for(const row of corpus.body)test("body day of "+JSON.stringify(row.body),()=>{expect(extractBundleDateLineFromCommitBody(row.body)).toBe(row.expected);});
for(const row of corpus.commit)test("commit day of "+JSON.stringify(row.subject)+" + "+JSON.stringify(row.body),()=>{expect(extractBundleDateLineFromCommit(row.subject,row.body)).toBe(row.expected);});
for(const row of corpus.normalize)test("date line normalisation of "+JSON.stringify(row.line),()=>{expect(normalizeBundleDateLine(row.line)).toBe(row.expected);});
for(const row of corpus.format)test("formatted date line "+JSON.stringify(row.line),()=>{expect(formatBundleDateLine(row.line,row.ulocDelta,row.sizeDelta,row.ulocBloc,row.sizeBloc)).toBe(row.expected);});
for(const row of corpus.bundleModel.wip)test("bundle subject pattern on "+JSON.stringify(row.subject),()=>{const match=BUNDLE_WIP_SUBJECT_RE.exec(row.subject);expect(match===null?null:match[1]).toBe(row.expected);});
for(const row of corpus.bundleModel.date)test("bundle date section pattern on "+JSON.stringify(row.line),()=>{expect(BUNDLE_DATE_SECTION_RE.test(row.line)).toBe(row.expected);});'''
ts_d = lib.ts_wrap(UD, "📅️ Contract: bundle day lines are read from the micro-commit body timestamp (else the subject), stripped of hand-written metrics and re-rendered with the computed metric suffixes, and the bundle subject and date patterns accept exactly their spelling; the corpus is language-agnostic and `@@O@@` recomputes each line with exact-decimal metric formatting.",
 'import {BUNDLE_DATE_SECTION_RE,BUNDLE_WIP_SUBJECT_RE} from "@@🎛commands/📝commit/🧾bundle-model/🟦.ts@@";\nimport {extractBundleDateLineFromCommit,extractBundleDateLineFromCommitBody,extractBundleDateLineFromSubject,formatBundleDateLine,normalizeBundleDateLine} from "@@' + UD + '/🟦.ts@@";', body_d)
orc_d = lib.PY_HEAD.format(doc="🐍️ Independent model of the bundle date lines: regex on the same literals, exact-decimal metric suffixes (stdlib only).") + open("spec_metrics.py", encoding="utf8").read() + r'''
DAY = "🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2}"


def from_subject(s):
    m = re.search(DAY, s.strip())
    return m.group(0) if m else None


def from_body(b):
    for raw in b.split("\n"):
        line = raw.strip()
        if re.match("^" + DAY + "⏰" + VS, line):
            return re.match("^" + DAY, line).group(0)
    return None


def normalize(line):
    s = line.strip()
    k = s.find("📊" + VS + "metric")
    return (s[:k] if k >= 0 else s).strip()


for c in corpus["subject"]:
    if from_subject(c["subject"]) != c["expected"]:
        failures.append("subject %r" % c["subject"])
for c in corpus["body"]:
    if from_body(c["body"]) != c["expected"]:
        failures.append("body %r" % c["body"])
for c in corpus["commit"]:
    if (from_body(c["body"]) or from_subject(c["subject"])) != c["expected"]:
        failures.append("commit")
for c in corpus["normalize"]:
    if normalize(c["line"]) != c["expected"]:
        failures.append("normalize %r" % c["line"])
for c in corpus["format"]:
    u, s = c["ulocDelta"], c["sizeDelta"]
    got = normalize(c["line"]) + body_line("uloc", c["ulocBloc"], u["added"], u["edited"], u["removed"]) + body_line("size", c["sizeBloc"], s["added"], s["edited"], s["removed"])
    if got != c["expected"]:
        failures.append("format %r" % c["line"])
for c in corpus["bundleModel"]["wip"]:
    m = re.match("^(.+" + DAY + ")🔀" + VS + "$", c["subject"])
    if (m.group(1) if m else None) != c["expected"]:
        failures.append("wip %r" % c["subject"])
for c in corpus["bundleModel"]["date"]:
    if (re.match("^" + DAY + "$", c["line"]) is not None) != c["expected"]:
        failures.append("date %r" % c["line"])
''' + lib.PY_TAIL
lib.write_unit(UD, corpus_d, sch_d, ts_d, orc_d)
