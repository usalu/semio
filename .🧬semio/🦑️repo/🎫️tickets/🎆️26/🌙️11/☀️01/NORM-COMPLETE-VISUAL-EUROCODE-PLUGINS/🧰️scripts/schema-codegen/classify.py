#!/usr/bin/env python3
"""Classifies every content difference between the committed twins (BEFORE tree) and the generated twins (AFTER tree).
Usage: classify.py <before-norm-root> <after-norm-root> <before-check.txt> <before-importcheck.txt>
A twin differs in content when its normalised reading (declaration/import order, comments, whitespace dropped) differs. Per file the class says why the committed twin is stale, broken or differently conventional; the independent checker (check.py) output of the BEFORE tree and its import check are the evidence."""
import collections, os, re, sys, difflib

before, after, check_txt, import_txt = sys.argv[1:5]
TW = {"🟦️.ts": "ts", "🛰️.proto": "proto", "🔗️.graphql": "graphql", "🦀️.rs": "rust"}


def flagged(path):
    out = collections.defaultdict(list)
    for line in open(path, encoding="utf8"):
        m = re.match(r"^(\w[\w-]*)\s+(\S+?):(\S*)\s+(\S+) \[([\w-]+)\]: (.*)$", line)
        if m: out[(m.group(2), m.group(3), m.group(5).split("-")[0])].append((m.group(1), m.group(4), m.group(6)[:90]))
    return out


FLAGS = flagged(check_txt)
IMPORTS = [l.rstrip("\n").split("\t") for l in open(import_txt, encoding="utf8") if "\t" in l] if os.path.exists(import_txt) else []
BROKEN = {line[0] for line in IMPORTS}


def norm_ts(t):
    t = re.sub(r"/\*[\s\S]*?\*/", "", t)
    t = re.sub(r"^\s*//.*$", "", t, flags=re.M)
    body = re.sub(r"^import .*$", "", t, flags=re.M)
    out = {}
    for s in re.split(r"\n(?=export )", body):
        s = re.sub(r"\s+", "", s).replace("(string)|null", "string|null")
        m = re.match(r"export(?:interface|type|const)(\w+)", s)
        if m: out[m.group(1)] = s
    imps = set()
    for m in re.finditer(r'^import \{ (.*) \} from ("[^"]+");', t, re.M):
        imps |= {(m.group(2), n.strip().replace("type ", "")) for n in m.group(1).split(",") if re.search(r"\b" + n.strip().replace("type ", "") + r"\b", body)}
    return out, imps


def norm_blocks(t, comment):
    t = re.sub(comment, "", t, flags=re.M)
    items = re.split(r"\n(?=(?:message|enum|type|input|union|scalar|interface) )", t)
    out = {}
    for it in items:
        it = re.sub(r"\s+", "", it)
        m = re.match(r"(?:message|enum|type|input|union|scalar|interface)(\w+)", it)
        if m: out[m.group(1)] = it
    return out


def unit_of(rel):
    m = re.search(r"🗿️artifacts/(\S+?)/🏅️standards/🔖️1/🪆️subsets/✳️any/(.*)/[^/]+$", rel)
    if m: return m.group(1)[1:].replace("️", ""), m.group(2)
    return "shared", rel.rsplit("/", 1)[0]


def reasons_ts(a, b):
    (sa, ia), (sb, ib) = norm_ts(a), norm_ts(b)
    r = set()
    for k in set(sa) | set(sb):
        if k in sa and k in sb and sa[k] != sb[k]:
            x, y = sa[k], sb[k]
            if "normWireDefault" in x and "normWireDefault" not in y: r.add("reader defaults a field the schema leaves optional")
            if re.sub(r"normWireRange\(normWireInteger,\{[^}]*\}\)", "normWireInteger", y) == x or "normWireRange" in y and "normWireRange" not in x: r.add("committed reader lacks the integer bounds of the schema")
            if "?:" in y and "?:" not in x: r.add("schema marks the field optional/nullable, committed twin requires it")
            if "normWireMap" in x or "normWireJson" in x: r.add("committed reader keeps a map/untyped value where the schema names a record, enum or union")
            if not r: r.add("declaration differs: " + k)
        elif k in sa and k not in sb: r.add("committed declares a name the schema no longer produces (renamed/removed type)")
        elif k in sb and k not in sa: r.add("schema produces a declaration the committed twin lacks (stale: new type)")
    if {i for i in ia if not i[0].startswith('"@')} != {i for i in ib if not i[0].startswith('"@')} and not r: r.add("sibling import names differ (committed imports stale names)")
    return r or {"import list only"}


def reasons_blocks(a, b, comment):
    sa, sb = norm_blocks(a, comment), norm_blocks(b, comment)
    r = set()
    for k in set(sa) | set(sb):
        if k in sa and k in sb and sa[k] != sb[k]:
            x, y = sa[k], sb[k]
            if re.sub(r"uint(32|64)", "int64", x) == re.sub(r"uint(32|64)", "int64", y): r.add("integer width (uint32/uint64/int64 follows the schema bounds)")
            else: r.add("record body differs: typed record/list where the committed twin lowers to a string/JSON blob, or fields differ")
        elif k in sa and k not in sb: r.add("committed declares a type the schema no longer produces")
        elif k in sb and k not in sa: r.add("schema produces a type the committed twin lacks (nested record, payload message or enum)")
    return r or {"layout only"}


files = collections.defaultdict(list)
for dp, dn, fn in os.walk(before):
    dn[:] = [d for d in dn if d not in ("target", "node_modules", "🗑️generated")]
    for f in fn:
        if f in TW and os.path.exists(os.path.join(dp, "🔣️.json")) or (f == "🦀️.rs"):
            p = os.path.join(dp, f)
            q = os.path.join(after, os.path.relpath(p, before))
            if not os.path.exists(q): continue
            a, b = open(p, encoding="utf8").read(), open(q, encoding="utf8").read()
            if a == b or re.search("🤖️ Generated by|region 🚪️Parsers", a): continue
            rel = os.path.relpath(p, before)
            lang = TW[f]
            if lang == "ts": r = reasons_ts(a, b)
            elif lang == "proto": r = reasons_blocks(a, b, r"^\s*//.*$")
            elif lang == "graphql": r = reasons_blocks(a, b, r"^\s*#.*$")
            else:
                if "cfg_attr(test," in a and "cfg_attr(test," not in b and re.sub(r"\s+", "", a.replace('cfg_attr(test,', 'cfg_attr(any(test,feature="test-support"),')) .replace('any(test,feature="test-support"),derive', 'any(test,feature="test-support"),derive') is not None: r = {"serde derive gated on `test` only; project standard is any(test, feature = \"test-support\")"}
                else: r = {"rust text differs"} if re.sub(r"\s+", "", a) != re.sub(r"\s+", "", b) else {"layout only"}
            if lang == "ts" and rel in BROKEN: r = {"committed twin imports names its sibling twin does not export (broken import)"} | {x for x in r if not x.startswith("declaration differs") and not x.startswith("sibling import")}
            if r == {"layout only"} or r == {"import list only"}: continue
            art, facet = unit_of(rel)
            hit = FLAGS.get((art, facet, {"ts": "ts", "proto": "proto", "graphql": "graphql", "rust": "rust"}[lang]), [])
            if not hit and lang == "ts": hit = FLAGS.get((art, facet, "ts"), [])
            tag = "committed twin fails the independent checker" if hit else "not flagged by the checker"
            for reason in sorted(r): files[(lang, tag, reason)].append(rel)
total = len({f for v in files.values() for f in v})
print(f"content differences: {total} files\n")
for (lang, tag, reason), fs in sorted(files.items(), key=lambda kv: (-len(kv[1]), kv[0])):
    print(f"- {lang:8} {len(fs):4}  [{tag}] {reason}")
print(f"\nbroken imports in the committed tree (names the sibling twin does not export): {len(IMPORTS)} in {len(BROKEN)} files")
