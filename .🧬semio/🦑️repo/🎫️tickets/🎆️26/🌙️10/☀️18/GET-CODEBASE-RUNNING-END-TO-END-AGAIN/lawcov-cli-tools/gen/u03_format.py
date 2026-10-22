import re, lib, sys
from lib import S, resolve, LIBN
from spec_metrics import loc, size, ratio
UNIT = "🎛commands/📝commit/📊metrics/🎨format"
spec_src = open("spec_metrics.py", encoding="utf8").read()
H = "📊️metric"
SYM = {"net": "➕️", "edit": "✏️", "rem": "➖️", "sum": "\U0001f7f0️", "bloc": "\U0001f4af️", "up": "\U0001f4c8️", "down": "\U0001f4c9️", "ratio": "➗️"}
TOK = {"uloc": "📃uloc", "size": "💾size"}
counts = [0, -5, 1, 7, 99, 100, 999, 999.4, 999.5, 1000, 1001, 1234, 1500, 9999, 10000, 12345, 99949, 99950, 123456, 422377, 999499, 999500, 999999, 1000000, 1300000, 12345678, 999999999, 1000000000, 2500000000, 0.5, 0.25, 0.0004, 3.14159, 42.4242]
ratios = [0, -1, 0.5, 1, 2, 0.12345, 0.01234, 0.001, 0.0005, 0.00001234, 0.000001, 0.0000001, 0.9999, 0.00999, 0.00099]
slugs = {"TypeScript": "typescript", "C#": "csharp", "Bourne Shell": "shell", "Bourne Again Shell": "shell", "Shell": "shell", "Docker": "docker", "Dockerfile": "dockerfile", "Ruby": "ruby", "C++": "c", "Objective-C": "objectivec", "": "", "A Very Long Language Name That Exceeds": "averylonglanguagenametha", "Rust": "rust", "TeX": "tex", "Visual Basic .NET": "visualbasicnet", "日本語": ""}

def body(kind, code, added, edited, removed, le=None, ls=None):
    f = loc if kind == "uloc" else size
    line = H
    if le and ls:
        line += le + ls
    line += TOK[kind]
    bloc = max(0, int(code + 0.5))
    if bloc > 0:
        line += SYM["bloc"] + f(bloc)
    net = added - removed
    if net != 0:
        line += (SYM["up"] if net > 0 else SYM["down"]) + f(abs(net))
        prev = bloc - net
        if prev > 0:
            line += SYM["ratio"] + ratio(abs(net) / prev)
    if added > 0: line += SYM["net"] + f(added)
    if edited > 0: line += SYM["edit"] + f(edited)
    if removed > 0: line += SYM["rem"] + f(removed)
    s = added + edited + removed
    if s > 0: line += SYM["sum"] + f(s)
    return line

bodies = []
for kind, code, a, e, r, le, ls in [("uloc", 1000, 100, 20, 10, None, None), ("uloc", 0, 0, 0, 0, None, None), ("size", 2048, 512, 0, 0, "🟦️", "typescript"), ("uloc", 500, 0, 0, 40, "🦀️", "rust"), ("uloc", 100, 100, 0, 0, None, None), ("uloc", 90, 10, 5, 10, None, None), ("size", 1500000, 0, 300, 0, None, None), ("uloc", 12, 3, 0, 15, "🐍️", "python"), ("uloc", 0.4, 1, 0, 0, None, None), ("uloc", 50, 0, 7, 0, "🧾️", None)]:
    bodies.append({"kind": kind, "code": code, "added": a, "edited": e, "removed": r, "langEmoji": le, "langSlug": ls, "expected": body(kind, code, a, e, r, le, ls)})
bodies = [{k: v for k, v in b.items() if v is not None} for b in bodies]

def suffix(d):
    s = ""
    if d["added"] > 0: s += SYM["net"] + loc(d["added"])
    if d["edited"] > 0: s += SYM["edit"] + loc(d["edited"])
    if d["removed"] > 0: s += SYM["rem"] + loc(d["removed"])
    t = d["added"] + d["edited"] + d["removed"]
    if t > 0: s += SYM["sum"] + loc(t)
    return s
deltas = [{"added": a, "edited": e, "removed": r} for a, e, r in [(0, 0, 0), (5, 0, 0), (0, 3, 0), (0, 0, 9), (1200, 30, 4000), (0, 1000, 0)]]
suffixes = [{"line": "base", "delta": d, "expected": "base" + suffix(d)} for d in deltas]
bundle = []
for ud, sd, ub, sb, le, ls in [({"added": 10, "edited": 2, "removed": 1}, {"added": 4096, "edited": 0, "removed": 0}, 800, 120000, None, None), ({"added": 0, "edited": 0, "removed": 0}, {"added": 0, "edited": 0, "removed": 0}, 0, 0, "🟦️", "typescript")]:
    one = {"ulocDelta": ud, "sizeDelta": sd, "ulocBloc": ub, "sizeBloc": sb, "expected": body("uloc", ub, ud["added"], ud["edited"], ud["removed"], le, ls) + body("size", sb, sd["added"], sd["edited"], sd["removed"], le, ls)}
    if le: one.update(langEmoji=le, langSlug=ls)
    bundle.append(one)
corpus = {"version": 1, "constants": {"header": H, "tokens": TOK, "symbols": SYM},
 "count": [{"n": n, "loc": loc(n), "size": size(n)} for n in counts], "ratio": [{"n": n, "expected": ratio(n)} for n in ratios],
 "slug": [{"lang": k, "expected": v} for k, v in slugs.items()], "round": [{"n": n, "digits": d, "expected": float(x)} for n, d, x in [(0, 3, 0), (1234.5678, 3, 1230), (-1234.5678, 3, -1230), (0.00123456, 2, 0.0012), (5, 1, 5), (99.95, 3, 100), (float("nan") if False else 1e-9, 3, 1e-9)]],
 "body": bodies, "suffix": suffixes, "bundle": bundle}
corpus["round"] = [{"n": n, "digits": d, "expected": x} for n, d, x in [(0, 3, 0), (1234.5678, 3, 1230), (-1234.5678, 3, -1230), (0.00123456, 2, 0.0012), (5, 1, 5), (99.95, 3, 100), (1e-9, 3, 1e-9), (123456, 2, 120000), (0.5, 1, 0.5), (2.5, 1, 3), (-2.5, 1, -3)]]
sd = S.obj({"added": S.int, "edited": S.int, "removed": S.int})
schema = S.obj({"version": S.const(1), "constants": S.obj({"header": S.str, "tokens": S.obj({"uloc": S.str, "size": S.str}), "symbols": S.map(S.str)}),
 "count": S.arr(S.obj({"n": S.num, "loc": S.str, "size": S.str}), 1), "ratio": S.arr(S.obj({"n": S.num, "expected": S.str}), 1),
 "slug": S.arr(S.obj({"lang": S.str, "expected": S.str}), 1), "round": S.arr(S.obj({"n": S.num, "digits": S.int, "expected": S.num}), 1),
 "body": S.arr(S.obj({"kind": S.enum("uloc", "size"), "code": S.num, "added": S.int, "edited": S.int, "removed": S.int, "langEmoji": S.str, "langSlug": S.str, "expected": S.str}, required=["kind", "code", "added", "edited", "removed", "expected"]), 1),
 "suffix": S.arr(S.obj({"line": S.str, "delta": sd, "expected": S.str}), 1),
 "bundle": S.arr(S.obj({"ulocDelta": sd, "sizeDelta": sd, "ulocBloc": S.num, "sizeBloc": S.num, "langEmoji": S.str, "langSlug": S.str, "expected": S.str}, required=["ulocDelta", "sizeDelta", "ulocBloc", "sizeBloc", "expected"]), 1)})
bodyts = '''for(const row of corpus.count)test("count "+row.n,()=>{expect(formatMetricLocCount(row.n)).toBe(row.loc);expect(formatMetricSizeCount(row.n)).toBe(row.size);});
for(const row of corpus.ratio)test("ratio "+row.n,()=>{expect(formatMetricRatio(row.n)).toBe(row.expected);});
for(const row of corpus.slug)test("slug "+JSON.stringify(row.lang),()=>{expect(langMetricsSlug(row.lang)).toBe(row.expected);});
for(const row of corpus.round)test("round "+row.n+" to "+row.digits,()=>{expect(roundSignificant(row.n,row.digits)).toBe(row.expected);});
test("non-finite numbers format as zero",()=>{for(const value of[Number.NaN,Number.POSITIVE_INFINITY,Number.NEGATIVE_INFINITY]){expect(roundSignificant(value)).toBe(0);expect(formatMetricLocCount(value)).toBe("0");expect(formatMetricSizeCount(value)).toBe("0");expect(formatMetricRatio(value)).toBe("0");}});
test("the constants match the corpus",()=>{expect(COMMIT_METRIC_HEADER).toBe(corpus.constants.header);expect(METRIC_KIND_ULOC.token).toBe(corpus.constants.tokens.uloc);expect(METRIC_KIND_SIZE.token).toBe(corpus.constants.tokens.size);expect(METRIC_KINDS.uloc).toBe(METRIC_KIND_ULOC);expect(METRIC_KINDS.size).toBe(METRIC_KIND_SIZE);expect(COMMIT_METRIC_TOTAL_EMOJI.length).toBeGreaterThan(0);});
for(const[index,row]of corpus.body.entries())test("body "+index,()=>{expect(formatMetricBody(row as any)).toBe(row.expected);if(row.kind==="uloc")expect(formatUlocMetricsBody(row as any)).toBe(row.expected);});
for(const row of corpus.suffix)test("suffix "+JSON.stringify(row.delta),()=>{expect(appendGitDeltaSuffix(row.line,row.delta)).toBe(row.expected);});
for(const[index,row]of corpus.bundle.entries())test("bundle suffixes "+index,()=>{expect(formatBundleMetricSuffixes(row.ulocDelta,row.sizeDelta,row.ulocBloc,row.sizeBloc,(row as any).langEmoji,(row as any).langSlug)).toBe(row.expected);});
test("the uloc bundle suffix is the uloc body",()=>{const row=corpus.body[0]!;expect(formatBundleUlocSuffix({added:row.added,edited:row.edited,removed:row.removed},row.code)).toBe(row.expected);});'''
ts = lib.ts_wrap(UNIT, "🎨️ Contract: the commit metric formatter renders counts with three significant figures (k/M and B/KB/MB/GB), ratios in percent, language slugs and the full metric line; the corpus is language-agnostic and `@@O@@` recomputes every string with exact decimal arithmetic.",
 'import {COMMIT_METRIC_HEADER,COMMIT_METRIC_TOTAL_EMOJI,METRIC_KINDS,METRIC_KIND_SIZE,METRIC_KIND_ULOC,appendGitDeltaSuffix,formatBundleMetricSuffixes,formatBundleUlocSuffix,formatMetricBody,formatMetricLocCount,formatMetricRatio,formatMetricSizeCount,formatUlocMetricsBody,langMetricsSlug,roundSignificant} from "@@' + UNIT + '/🟦.ts@@";', bodyts)
oracle = lib.PY_HEAD.format(doc="🐍️ Independent exact-decimal model of the commit metric formatter: significant figures with half-up rounding via `decimal`, no shared code with the Bun test.") + spec_src.replace("import math\n", "import math\n").replace("count(n, units):\n    v", "count(n, units):\n    v") + r'''

C = corpus["constants"]
SYM, H, TOK = C["symbols"], C["header"], C["tokens"]
for c in corpus["count"]:
    if loc(c["n"]) != c["loc"] or size(c["n"]) != c["size"]:
        failures.append("count %s" % c["n"])
for c in corpus["ratio"]:
    if ratio(c["n"]) != c["expected"]:
        failures.append("ratio %s" % c["n"])
SLUG = {"TypeScript": "typescript", "JavaScript": "javascript", "C#": "csharp", "JSON": "json", "YAML": "yaml", "TOML": "toml", "Markdown": "markdown", "Dockerfile": "dockerfile", "Makefile": "makefile", "Bourne Shell": "shell", "Bourne Again Shell": "shell", "PowerShell": "powershell", "TeX": "tex", "HTML": "html", "CSS": "css", "SQL": "sql", "CSV": "csv", "XML": "xml", "Go": "go", "Python": "python", "Rust": "rust", "Shell": "shell", "Docker": "docker"}
for c in corpus["slug"]:
    got = SLUG.get(c["lang"]) or re.sub(r"[^a-z0-9]+", "", c["lang"].lower())[:24]
    if got != c["expected"]:
        failures.append("slug %s" % c["lang"])


def body(kind, code, added, edited, removed, le=None, ls=None):
    f = loc if kind == "uloc" else size
    line = H + (le + ls if le and ls else "") + TOK[kind]
    bloc = max(0, int(code + 0.5))
    line += SYM["bloc"] + f(bloc) if bloc > 0 else ""
    net = added - removed
    if net:
        line += (SYM["up"] if net > 0 else SYM["down"]) + f(abs(net))
        if bloc - net > 0:
            line += SYM["ratio"] + ratio(abs(net) / (bloc - net))
    for key, v in (("net", added), ("edit", edited), ("rem", removed)):
        line += SYM[key] + f(v) if v > 0 else ""
    total = added + edited + removed
    return line + (SYM["sum"] + f(total) if total > 0 else "")


for i, c in enumerate(corpus["body"]):
    if body(c["kind"], c["code"], c["added"], c["edited"], c["removed"], c.get("langEmoji"), c.get("langSlug")) != c["expected"]:
        failures.append("body %d" % i)
for i, c in enumerate(corpus["bundle"]):
    u, s = c["ulocDelta"], c["sizeDelta"]
    got = body("uloc", c["ulocBloc"], u["added"], u["edited"], u["removed"], c.get("langEmoji"), c.get("langSlug")) + body("size", c["sizeBloc"], s["added"], s["edited"], s["removed"], c.get("langEmoji"), c.get("langSlug"))
    if got != c["expected"]:
        failures.append("bundle %d" % i)
''' + lib.PY_TAIL
lib.write_unit(UNIT, corpus, schema, ts, oracle)
