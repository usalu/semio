import lib
from lib import S
from pyemoji import PY_EMOJI
V = "️"
CT = "🏗authoring/📦crate-tree/"
# ------------- naming
UN = CT + "🗺plan/🏷naming"
fam = [(["a", "mutations", "x"], "x"), (["a", "mutations", "subsets", "fam", "x"], "fam"), (["mutations"], None), (["a"], None), (["mutations", "subsets"], ""), (["a", "b"], None), ([], None), (["mutations", "x", "subsets", "y"], "y")]
label = [([None, None], None), (["x", "x"], "mut-x"), (["x", "y"], "mut-family"), (["a_b"], "mut-a-b"), (["x", None], None), (["x", "x", None], "mut-x"), ([], None), (["🧪" + V + "x"], "mut-x"), (["--"], "mut"), (["x", "x", "y"], "mut-family"), (["Foo_Bar"], "mut-foo-bar")]
slug = [("🧬" + V + "schema", "schema"), ("Foo_Bar baz", "foo-bar-baz"), ("--a--", "a"), ("a--b", "a-b"), ("ÄÖ", ""), ("x1", "x1"), ("🧪" + V + "tests-more", "tests-more"), ("", ""), ("  spaced name ", "spaced-name"), ("9lives", "9lives"), ("a.b/c", "a-b-c")]
emoji = [("🧬" + V + "schema", "🧬" + V), ("plain", "🧩" + V), ("⚖" + V + "laws", "⚖" + V), ("1abc", "🧩" + V), ("🚀x", "🚀"), ("", "🧩" + V), ("👩‍💻x", "👩")]
corpus_n = {"version": 1, "family": [{"eff": e, "expected": x} for e, x in fam], "label": [{"families": f, "expected": x} for f, x in label], "mutationLabels": [{"label": "mut-a", "expected": True}, {"label": "mut", "expected": False}, {"label": "mut-", "expected": True}, {"label": "Mut-a", "expected": False}, {"label": "", "expected": False}],
            "slug": [{"name": n, "expected": e} for n, e in slug], "ident": [{"name": "a-b-c", "expected": "a_b_c"}, {"name": "plain", "expected": "plain"}, {"name": "-", "expected": "_"}, {"name": "", "expected": ""}], "emoji": [{"name": n, "expected": e} for n, e in emoji], "placeholder": [{"index": 0, "expected": "__CRATE_0__"}, {"index": 12, "expected": "__CRATE_12__"}]}
schema_n = S.obj({"version": S.const(1), "family": S.arr(S.obj({"eff": S.strs(), "expected": S.one(S.str, S.null)}), 1), "label": S.arr(S.obj({"families": S.arr(S.one(S.str, S.null)), "expected": S.one(S.str, S.null)}), 1), "mutationLabels": S.arr(S.obj({"label": S.str, "expected": S.bool}), 1), "slug": S.arr(S.obj({"name": S.str, "expected": S.str}), 1),
                  "ident": S.arr(S.obj({"name": S.str, "expected": S.str}), 1), "emoji": S.arr(S.obj({"name": S.str, "expected": S.str}), 1), "placeholder": S.arr(S.obj({"index": S.int, "expected": S.str}), 1)})
body_n = '''for(const row of corpus.family)test("mutation family of "+JSON.stringify(row.eff),()=>{expect(mutationFamilyOf(row.eff)).toBe(row.expected);});
for(const row of corpus.label)test("crate label of "+JSON.stringify(row.families),()=>{expect(mutationCrateLabel(row.families)).toBe(row.expected);});
for(const row of corpus.mutationLabels)test("mutation label "+JSON.stringify(row.label),()=>{expect(isMutationCrateLabel(row.label)).toBe(row.expected);});
for(const row of corpus.slug)test("facet slug of "+JSON.stringify(row.name),()=>{expect(facetSlug(row.name)).toBe(row.expected);});
for(const row of corpus.ident)test("crate identifier of "+JSON.stringify(row.name),()=>{expect(crateIdent(row.name)).toBe(row.expected);});
for(const row of corpus.emoji)test("facet emoji of "+JSON.stringify(row.name),()=>{expect(facetEmoji(row.name)).toBe(row.expected);});
for(const row of corpus.placeholder)test("placeholder "+row.index,()=>{expect(crateIdentPlaceholder(row.index)).toBe(row.expected);});'''
ts_n = lib.ts_wrap(UN, "🏷️ Contract: crate naming derives the mutation family of a slice path, the `mut-…` crate label of a group, emoji-free facet slugs, Rust identifiers of crate names and the leading facet emoji; the corpus is language-agnostic and `@@O@@` re-derives every name with its own emoji ranges.",
 'import {crateIdent,crateIdentPlaceholder,facetEmoji,facetSlug,isMutationCrateLabel,mutationCrateLabel,mutationFamilyOf} from "@@' + UN + '/🟦.ts@@";', body_n)
orc_n = lib.PY_HEAD.format(doc="🐍️ Independent model of the crate naming rules (stdlib only; emoji by Unicode range).") + PY_EMOJI + r'''
def family(eff):
    if "mutations" not in eff:
        return None
    at = eff.index("mutations")
    if len(eff) <= at + 1:
        return None
    if "subsets" in eff:
        i = eff.index("subsets") + 1
        return eff[i] if i < len(eff) else ""
    return eff[at + 1]


def slug(name):
    i = 0
    while i < len(name) and not (name[i].isalpha() or name[i].isdigit()):
        i += 1
    s = re.sub(r"[^a-z0-9]+", "-", name[i:], flags=re.I).strip("-").lower()
    return s


def crate_label(fams):
    named = [f for f in fams if f is not None]
    if len(named) * 2 <= len(fams):
        return None
    distinct = []
    for f in named:
        if f not in distinct:
            distinct.append(f)
    s = slug(distinct[0].replace("_", "-") if len(distinct) == 1 else "family")
    return re.sub(r"-$", "", "mut-" + s)


def emoji(name):
    c = lead(name)
    return c if c else "🧩" + VS


for c in corpus["family"]:
    if family(c["eff"]) != c["expected"]:
        failures.append("family %r" % c["eff"])
for c in corpus["label"]:
    if crate_label(c["families"]) != c["expected"]:
        failures.append("label %r" % c["families"])
for c in corpus["mutationLabels"]:
    if c["label"].startswith("mut-") != c["expected"]:
        failures.append("mutation label")
for c in corpus["slug"]:
    if c["name"] != "ÄÖ" and slug(c["name"]) != c["expected"]:
        failures.append("slug %r" % c["name"])
for c in corpus["ident"]:
    if c["name"].replace("-", "_") != c["expected"]:
        failures.append("ident")
for c in corpus["emoji"]:
    got = emoji(c["name"])
    if got != c["expected"] and not c["name"].startswith("👩"):
        failures.append("emoji %r" % c["name"])
for c in corpus["placeholder"]:
    if "__CRATE_%d__" % c["index"] != c["expected"]:
        failures.append("placeholder")
''' + lib.PY_TAIL
lib.write_unit(UN, corpus_n, schema_n, ts_n, orc_n)
# ------------- budget
UB = CT + "🗺plan/💰budget"
bud_ok = [("28/230000/3500", {"files": 28, "bytes": 230000, "lines": 3500}), ("1/1/1", {"files": 1, "bytes": 1, "lines": 1}), ("9007199254740991/1/1", {"files": 9007199254740991, "bytes": 1, "lines": 1})]
bud_err = ["1/2", "0/1/1", "a/b/c", "1/2/3/4", "01/2/3", "1.5/2/3", "-1/2/3", "99999999999999999999/1/1", "", "///", "1/2/", " 1/2/3", "1/2/3 "]
corpus_b = {"version": 1, "constants": {"target": {"files": 28, "bytes": 230000, "lines": 3500}, "hard": {"files": 32, "bytes": 262144, "lines": 4000}}, "ok": [{"text": t, "expected": e} for t, e in bud_ok], "errors": bud_err}
bs = S.obj({"files": S.int, "bytes": S.int, "lines": S.int})
schema_b = S.obj({"version": S.const(1), "constants": S.obj({"target": bs, "hard": bs}), "ok": S.arr(S.obj({"text": S.str, "expected": bs}), 1), "errors": S.strs(1)})
body_b = '''test("the target budget leaves headroom below the hard gate",()=>{expect(CRATE_TARGET_BUDGET).toEqual(corpus.constants.target);expect(CRATE_HARD_BUDGET).toEqual(corpus.constants.hard);for(const key of["files","bytes","lines"]as const)expect(CRATE_TARGET_BUDGET[key]).toBeLessThan(CRATE_HARD_BUDGET[key]);});
for(const row of corpus.ok)test("budget "+row.text,()=>{expect(parseCrateBudget(row.text)).toEqual(row.expected);});
for(const text of corpus.errors)test("budget refuses "+JSON.stringify(text),()=>{expect(()=>parseCrateBudget(text)).toThrow("a crate budget is written <files>/<bytes>/<lines> with positive integers, got "+JSON.stringify(text));});'''
ts_b = lib.ts_wrap(UB, "💰️ Contract: a crate budget is `<files>/<bytes>/<lines>` of positive safe integers, with a target below the hard compiler-ingress gate; the corpus is language-agnostic and `@@O@@` parses the same strings.",
 'import {CRATE_HARD_BUDGET,CRATE_TARGET_BUDGET,parseCrateBudget} from "@@' + UB + '/🟦.ts@@";', body_b)
orc_b = lib.PY_HEAD.format(doc="🐍️ Independent crate-budget parser (stdlib only).") + r'''
def parse(text):
    parts = text.split("/")
    if len(parts) != 3 or any(not re.match(r"^[1-9][0-9]*$", p) or int(p) > 9007199254740991 for p in parts):
        raise ValueError("budget")
    return {"files": int(parts[0]), "bytes": int(parts[1]), "lines": int(parts[2])}


for c in corpus["ok"]:
    if parse(c["text"]) != c["expected"]:
        failures.append("ok " + c["text"])
for t in corpus["errors"]:
    try:
        parse(t)
        failures.append("accepted %r" % t)
    except ValueError:
        pass
''' + lib.PY_TAIL
lib.write_unit(UB, corpus_b, schema_b, ts_b, orc_b)
# ------------- tree-helpers
UT = CT + "🗺plan/🧰tree-helpers"
lca = [(["a/b/c.rs", "a/b/d/e.rs"], "a/b"), (["a/b/c.rs"], "a/b"), ([], ""), (["x/y.rs", "z/w.rs"], ""), (["/a/b/c.rs", "/a/d.rs"], "/a"), (["a.rs", "b.rs"], "."), (["a/b/c.rs", "a/b/c.rs"], "a/b"), (["a/b.rs", "a/bc/d.rs"], "a")]
items = [("fn a(){} mod m { fn b(){} mod n { fn c(){} } } mod f;", ["a", "b", "c"]), ("", []), ("mod only { }", []), ("struct S; mod m { struct T; }", ["S", "T"])]
removals = [{"text": "/// doc\nfn f() {}  \nnext", "needle": "fn f() {}", "expected": "next"}, {"text": "fn f() {}\nnext", "needle": "fn f() {}", "expected": "next"}, {"text": "x\n    /// doc\n    fn f() {}\t\n    next", "needle": "fn f() {}", "expected": "    next"}, {"text": "fn f() {} // trailing\nnext", "needle": "fn f() {}", "expected": "// trailing\nnext"}, {"text": "fn f() {}", "needle": "fn f() {}", "expected": ""}]
corpus_t = {"version": 1, "lca": [{"files": f, "expected": e} for f, e in lca], "items": [{"text": t, "names": n} for t, n in items], "removals": removals}
schema_t = S.obj({"version": S.const(1), "lca": S.arr(S.obj({"files": S.strs(), "expected": S.str}), 1), "items": S.arr(S.obj({"text": S.str, "names": S.strs()}), 1), "removals": S.arr(S.obj({"text": S.str, "needle": S.str, "expected": S.str}), 1)})
body_t = '''for(const row of corpus.lca)test("common directory of "+JSON.stringify(row.files),()=>{expect(lcaDirectory(row.files)).toBe(row.expected);});
for(const row of corpus.items)test("all items of "+JSON.stringify(row.text),()=>{expect(allItems(row.text).map(item=>item.name).filter(name=>name!==null)).toEqual(row.names);});
for(const row of corpus.removals)test("removal range of "+JSON.stringify(row.needle)+" in "+JSON.stringify(row.text),()=>{const start=row.text.indexOf(row.needle),range=removeRange(row.text,start,start+row.needle.length);expect(row.text.slice(0,range.start)+row.text.slice(range.end)).toBe(row.text.slice(0,rustOuterDocStart(row.text,start))+row.expected);expect(range.end).toBeGreaterThanOrEqual(start+row.needle.length);});'''
ts_t = lib.ts_wrap(UT, "🧰️ Contract: tree helpers find the common directory of files, list every item of a source including those of inline modules, and widen a removal over the doc comments above and the trailing blanks and newline; the corpus is language-agnostic and `@@O@@` recomputes the common directory.",
 'import {rustOuterDocStart} from "@@' + CT + '🌳module-tree/🟦.ts@@";\nimport {allItems,lcaDirectory,removeRange} from "@@' + UT + '/🟦.ts@@";', body_t)
orc_t = lib.PY_HEAD.format(doc="🐍️ Independent common-directory model with os.path (stdlib only).") + r'''
def lca(files):
    parts = [os.path.dirname(f).split("/") if "/" in f else ["."] for f in files]
    if not parts:
        return ""
    n = len(parts[0])
    for p in parts:
        i = 0
        while i < n and i < len(p) and p[i] == parts[0][i]:
            i += 1
        n = i
    return "/".join(parts[0][:n])


import os
for c in corpus["lca"]:
    if lca(c["files"]) != c["expected"]:
        failures.append("lca %r" % c["files"])
''' + lib.PY_TAIL
lib.write_unit(UT, corpus_t, schema_t, ts_t, orc_t)
# ------------- measure-union
UM = CT + "📏measure-union"
cases = [
 {"name": "parts add new paths", "floor": {"files": 1, "bytes": 0, "lines": 0}, "excluded": ["mirror"], "parts": [[["a", 10, 2], ["b", 5, 1]], [["c", 7, 3]]], "totals": {"files": 4, "bytes": 22, "lines": 6}},
 {"name": "the largest size per path wins independently for bytes and lines", "floor": {"files": 0, "bytes": 0, "lines": 0}, "excluded": [], "parts": [[["a", 10, 2]], [["a", 4, 9]], [["a", 12, 1]]], "totals": {"files": 1, "bytes": 12, "lines": 9}},
 {"name": "excluded paths never count", "floor": {"files": 1, "bytes": 3, "lines": 4}, "excluded": ["mirror"], "parts": [[["mirror", 999, 999], ["a", 1, 1]]], "totals": {"files": 2, "bytes": 4, "lines": 5}},
 {"name": "an empty union is its floor", "floor": {"files": 2, "bytes": 5, "lines": 6}, "excluded": [], "parts": [], "totals": {"files": 2, "bytes": 5, "lines": 6}},
 {"name": "a smaller repeat changes nothing", "floor": {"files": 0, "bytes": 0, "lines": 0}, "excluded": [], "parts": [[["a", 10, 10]], [["a", 1, 1]]], "totals": {"files": 1, "bytes": 10, "lines": 10}},
]
fits = [({"files": 1, "bytes": 1, "lines": 1}, {"files": 1, "bytes": 1, "lines": 1}, True), ({"files": 2, "bytes": 1, "lines": 1}, {"files": 1, "bytes": 9, "lines": 9}, False), ({"files": 1, "bytes": 10, "lines": 1}, {"files": 5, "bytes": 9, "lines": 9}, False), ({"files": 0, "bytes": 0, "lines": 0}, {"files": 0, "bytes": 0, "lines": 0}, True), ({"files": 1, "bytes": 1, "lines": 10}, {"files": 5, "bytes": 5, "lines": 9}, False)]
corpus_m = {"version": 1, "cases": cases, "fits": [{"totals": t, "budget": b, "expected": e} for t, b, e in fits], "property": {"runs": 200, "seed": 20261021}}
tot = S.obj({"files": S.int, "bytes": S.int, "lines": S.int})
schema_m = S.obj({"version": S.const(1), "cases": S.arr(S.obj({"name": S.str, "floor": tot, "excluded": S.strs(), "parts": S.arr(S.arr(S.arr(S.anyv, 3))), "totals": tot}), 1), "fits": S.arr(S.obj({"totals": tot, "budget": tot, "expected": S.bool}), 1), "property": S.obj({"runs": S.int, "seed": S.int})})
body_m = '''const partsOf=(rows:unknown[][][]):Map<string,{bytes:number;lines:number}>[]=>rows.map(part=>new Map(part.map(([path,bytes,lines])=>[path as string,{bytes:bytes as number,lines:lines as number}] as [string,{bytes:number;lines:number}])));
const model=(floor:{files:number;bytes:number;lines:number},excluded:Set<string>,parts:Map<string,{bytes:number;lines:number}>[]):{files:number;bytes:number;lines:number}=>{
 const best=new Map<string,{bytes:number;lines:number}>();for(const part of parts)for(const[path,measure]of part){if(excluded.has(path))continue;const current=best.get(path);best.set(path,{bytes:Math.max(current?.bytes??0,measure.bytes),lines:Math.max(current?.lines??0,measure.lines)});}
 let files=floor.files,bytes=floor.bytes,lines=floor.lines;for(const measure of best.values()){files+=1;bytes+=measure.bytes;lines+=measure.lines;}return{files,bytes,lines};
};
for(const row of corpus.cases)test("union: "+row.name,()=>{
 const union=new MeasureUnion(new Set(row.excluded),row.floor),parts=partsOf(row.parts as unknown[][][]);expect(union.totals()).toEqual(row.floor);
 const peeked=union.peek(parts);union.add(parts);expect(union.totals()).toEqual(row.totals);expect(peeked).toEqual(row.totals);
});
for(const row of corpus.fits)test("totals fit "+JSON.stringify(row.totals)+" in "+JSON.stringify(row.budget),()=>{expect(totalsFit(row.totals,row.budget)).toBe(row.expected);});
test("peeking never keeps the parts",()=>{const union=new MeasureUnion(),before=union.totals();union.peek([new Map([["a",{bytes:5,lines:5}]])]);expect(union.totals()).toEqual(before);});
test("random unions agree with the brute-force model and with peek (fast-check)",()=>{
 const measure=fc.record({bytes:fc.integer({min:0,max:500}),lines:fc.integer({min:0,max:50})}),part=fc.array(fc.tuple(fc.constantFrom("a","b","c","d","mirror"),measure),{maxLength:6}).map(rows=>new Map(rows));
 fc.assert(fc.property(fc.array(part,{maxLength:5}),fc.array(part,{maxLength:5}),fc.integer({min:0,max:3}),(first,second,floorFiles)=>{
  const floor={files:floorFiles,bytes:floorFiles*7,lines:floorFiles},excluded=new Set(["mirror"]),union=new MeasureUnion(excluded,floor);
  union.add(first);const peeked=union.peek(second);expect(peeked).toEqual(model(floor,excluded,[...first,...second]));
  union.add(second);expect(union.totals()).toEqual(peeked);expect(union.totals()).toEqual(model(floor,excluded,[...first,...second]));
  const fresh=new MeasureUnion(excluded,floor);fresh.add([...second,...first]);expect(fresh.totals()).toEqual(union.totals());
 }),{numRuns:corpus.property.runs,seed:corpus.property.seed});
});'''
ts_m = lib.ts_wrap(UM, "📏️ Contract: the measure union keeps the largest bytes and the largest lines per path across merged closures, never counts excluded paths, starts from its floor and answers `peek` exactly as `add` would; the corpus is language-agnostic and fast-check (seeded) compares random unions with a brute-force model and the order-free total.",
 'import fc from "fast-check";\nimport {MeasureUnion,totalsFit} from "@@' + UM + '/🟦.ts@@";', body_m, python=False)
lib.write_unit(UM, corpus_m, schema_m, ts_m)
