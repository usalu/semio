import json, lib
from lib import S
# ---------------- contract
UC = "🧬schema/🔍field-discovery/🧱contract"
states = [(" PERSISTED_SHARED ", "persisted-shared"), ("a_b-C", "a-b-c"), ("", ""), ("EPHEMERAL", "ephemeral"), ("x__y", "x--y")]
scalars = [("String", "string"), ("string", "string"), ("bool", "bool"), ("boolean", "bool"), ("Boolean", "bool"), ("i32", "int32"), ("u32", "uint32"), ("i64", "int64"), ("u64", "uint64"), ("f32", "float32"), ("f64", "float64"), ("Int", "int32"), ("Float", "float64"), ("bytes", "bytes"), ("Vec<u8>", "bytes"), (" Vec < u8 > ", "bytes"),
           ("Option<String>", "Option<String>"), ("", ""), ("Widget", "Widget"), ("Str ing", "string"), ("double", "double"), ("number", "number")]
snake = [("display_name", "displayName"), ("a_b_c", "aBC"), ("_x", "X"), ("already", "already"), ("a__b", "aB"), ("", ""), ("x_1_y", "x1Y"), ("trailing_", "trailing"), ("ID_value", "IDValue")]
find = [
 {"name": "no expected name never matches", "text": "pub struct A {", "pattern": "\\bpub\\s+struct\\s+([A-Za-z_][A-Za-z0-9_]*)\\s*\\{", "flags": "", "expected": None, "found": None},
 {"name": "the named declaration is found and the body starts after the brace", "text": "pub struct A { x }\npub struct B {  y }", "pattern": "\\bpub\\s+struct\\s+([A-Za-z_][A-Za-z0-9_]*)\\s*\\{", "flags": "", "expected": "B", "found": {"typeName": "B", "bodyStart": 29}},
 {"name": "an absent name is not found", "text": "pub struct A {", "pattern": "\\bpub\\s+struct\\s+([A-Za-z_][A-Za-z0-9_]*)\\s*\\{", "flags": "", "expected": "C", "found": None},
 {"name": "a global pattern is accepted as is", "text": "message M {", "pattern": "\\bmessage\\s+([A-Za-z_]\\w*)\\s*\\{", "flags": "g", "expected": "M", "found": {"typeName": "M", "bodyStart": 11}},
 {"name": "a prefix of the name does not match", "text": "type Widgets {", "pattern": "\\btype\\s+([A-Za-z_]\\w*)\\s*\\{", "flags": "", "expected": "Widget", "found": None},
]
find[1]["found"]["bodyStart"] = len("pub struct A { x }\npub struct B {")
find[3]["found"]["bodyStart"] = len("message M {")
corpus_c = {"version": 1, "scalarDefects": [{"raw": r, "expected": r, "defect": "🧱️contract/🟦️.ts:policyCanonicalScalar indexes a plain object literal, so a token that names an Object.prototype member (`constructor`, `toString`, `hasOwnProperty`, `valueOf`) returns the inherited function instead of the token"} for r in ("constructor", "toString", "hasOwnProperty", "valueOf")], "state": [{"raw": a, "expected": b} for a, b in states], "scalar": [{"raw": a, "expected": b} for a, b in scalars], "snake": [{"name": a, "expected": b} for a, b in snake], "find": find}
sch_c = S.obj({"version": S.const(1), "scalarDefects": S.arr(S.obj({"raw": S.str, "expected": S.str, "defect": S.str}), 1), "state": S.arr(S.obj({"raw": S.str, "expected": S.str}), 1), "scalar": S.arr(S.obj({"raw": S.str, "expected": S.str}), 1), "snake": S.arr(S.obj({"name": S.str, "expected": S.str}), 1),
               "find": S.arr(S.obj({"name": S.str, "text": S.str, "pattern": S.str, "flags": S.str, "expected": S.one(S.str, S.null), "found": S.one(S.obj({"typeName": S.str, "bodyStart": S.int}), S.null)}), 1)})
body_c = '''for(const row of corpus.state)test("canonical state of "+JSON.stringify(row.raw),()=>{expect(policyCanonicalState(row.raw)).toBe(row.expected);});
for(const row of corpus.scalar)test("canonical scalar of "+JSON.stringify(row.raw),()=>{expect(policyCanonicalScalar(row.raw)).toBe(row.expected);});
for(const row of corpus.scalarDefects)test.failing("canonical scalar defect "+row.raw,()=>{expect(policyCanonicalScalar(row.raw)).toBe(row.expected);});
for(const row of corpus.snake)test("camel case of "+JSON.stringify(row.name),()=>{expect(policySnakeToCamel(row.name)).toBe(row.expected);});
for(const row of corpus.find)test("declaration lookup: "+row.name,()=>{expect(policyFindSchemaDeclaration(row.text,new RegExp(row.pattern,row.flags),row.expected)).toEqual(row.found);});
test("a lookup with a global pattern is repeatable",()=>{const pattern=/\\bmessage\\s+([A-Za-z_]\\w*)\\s*\\{/g;expect(policyFindSchemaDeclaration("message M {",pattern,"M")).not.toBeNull();expect(policyFindSchemaDeclaration("message M {",pattern,"M")).not.toBeNull();});'''
ts_c = lib.ts_wrap(UC, "🧱️ Contract: the field-discovery contract normalises state classes, scalar spellings and snake-case names, and finds a schema declaration only by its declared name; the corpus is language-agnostic and `@@O@@` recomputes each verdict.",
 'import {policyCanonicalScalar,policyCanonicalState,policyFindSchemaDeclaration,policySnakeToCamel} from "@@' + UC + '/🟦.ts@@";', body_c)
orc_c = lib.PY_HEAD.format(doc="🐍️ Independent model of the field-discovery contract (stdlib only).") + r'''
SC = {"String": "string", "string": "string", "bool": "bool", "boolean": "bool", "Boolean": "bool", "i32": "int32", "u32": "uint32", "i64": "int64", "u64": "uint64", "f32": "float32", "f64": "float64", "Int": "int32", "Float": "float64", "bytes": "bytes", "Vec<u8>": "bytes"}
for c in corpus["state"]:
    if c["raw"].strip().lower().replace("_", "-") != c["expected"]:
        failures.append("state %r" % c["raw"])
for c in corpus["scalar"]:
    t = re.sub(r"\s+", "", c["raw"])
    if SC.get(t, t) != c["expected"]:
        failures.append("scalar %r" % c["raw"])
for c in corpus["snake"]:
    p = c["name"].split("_")
    if p[0] + "".join(x[:1].upper() + x[1:] for x in p[1:]) != c["expected"]:
        failures.append("snake %r" % c["name"])
for c in corpus["find"]:
    found = None
    if c["expected"]:
        for m in re.finditer(c["pattern"].replace("\\w", "[A-Za-z0-9_]"), c["text"]):
            if m.group(1) == c["expected"]:
                found = {"typeName": m.group(1), "bodyStart": m.end()}
                break
    if found != c["found"]:
        failures.append("find: " + c["name"])
''' + lib.PY_TAIL
lib.write_unit(UC, corpus_c, sch_c, ts_c, orc_c)
# ---------------- comparison
UK = "🧬schema/🔍field-discovery/⚖comparison"
diffs = [([], [], [], []), (["a", "b"], ["b", "a"], [], []), (["a", "b", "c"], ["b"], ["a", "c"], []), (["a"], ["a", "z", "y"], [], ["y", "z"]), (["b", "a"], ["c", "d"], ["a", "b"], ["c", "d"]), (["a", "a"], ["a"], [], []), (["Z", "a"], ["z"], ["Z", "a"], ["z"])]
corpus_k = {"version": 1, "cases": [{"reference": r, "candidate": c, "missing": m, "extra": e} for r, c, m, e in diffs]}
sch_k = S.obj({"version": S.const(1), "cases": S.arr(S.obj({"reference": S.strs(), "candidate": S.strs(), "missing": S.strs(), "extra": S.strs()}), 1)})
body_k = '''for(const[index,row]of corpus.cases.entries())test("field differences "+index+": "+JSON.stringify(row.reference)+" vs "+JSON.stringify(row.candidate),()=>{expect(policySchemaFieldDifferences(row.reference,row.candidate)).toEqual({missing:row.missing,extra:row.extra});});'''
ts_k = lib.ts_wrap(UK, "⚖️ Contract: schema field sets are compared by name without regard to order or duplicates, reporting sorted missing and extra names; the corpus is language-agnostic and `@@O@@` computes the same set differences.",
 'import {policySchemaFieldDifferences} from "@@' + UK + '/🟦.ts@@";', body_k)
orc_k = lib.PY_HEAD.format(doc="🐍️ Independent set-difference model of the field comparison (stdlib only).") + r'''
for i, c in enumerate(corpus["cases"]):
    r, a = set(c["reference"]), set(c["candidate"])
    if sorted(r - a) != c["missing"] or sorted(a - r) != c["extra"]:
        failures.append("case %d" % i)
''' + lib.PY_TAIL
lib.write_unit(UK, corpus_k, sch_k, ts_k, orc_k)
