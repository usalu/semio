#!/usr/bin/env python3
"""One-shot: delete the dead en1992 `FireSpec.column_method` in every layer (model, twins, diff lane, leaf `change-member-fire-column-method`, kind registrations, codecs, editor, fixtures, assets).
Each replacement asserts its match count; a second run finds nothing left to do. Usage: python3 -I en1992_remove_column_method.py [--dry]"""
import json, os, pathlib, re, shutil, sys
ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any")
S = ROOT / "🧬️schema"
DRY = "--dry" in sys.argv
changed = set()

def edit(path, fn, must=True):
    p = ROOT / path
    t = p.read_text(encoding="utf-8")
    n = fn(t)
    if n == t:
        assert not must or not re.search(r"column_method|columnMethod|ColumnMethod|column-method|COLUMN_METHOD", t), f"{path}: nothing replaced but field still present"
        return
    changed.add(path)
    if not DRY:
        p.write_text(n, encoding="utf-8")

def rm(t, pattern, count=None, flags=0):
    n, c = re.subn(pattern, "", t, flags=flags)
    assert count is None or c == count, f"expected {count} of {pattern!r}, removed {c}"
    return n

def rep(t, old, new, count=1):
    c = t.count(old)
    assert c in (0, count), f"expected {count} of {old[:60]!r}, found {c}"
    return t.replace(old, new)

# model + literals
edit("🧬️schema/📸️snapshot/🦀️.rs", lambda t: rm(rm(t, r'column_method: "[^"]*"\.into\(\), ', 6), r"    pub column_method: String,\n", 1))
for facet in ("📸️snapshot/", ""):
    edit(f"🧬️schema/{facet}🟦️.ts", lambda t: rm(rep(t, "  columnMethod: string;\n", ""), r"columnMethod: normWireRequired\(normWireString\), ", 1))
    edit(f"🧬️schema/{facet}🔗️.graphql", lambda t: rep(t, "columnMethod: String! ", ""))
    edit(f"🧬️schema/{facet}🛰️.proto", lambda t: rep(rep(t, "string column_method = 3; ", ""), "slab_system = 4;", "slab_system = 3;"))
    def js(t):
        o = json.loads(t)
        for d in (o, o.get("$defs", {}).get("FireSpec", {})):
            pass
        fs = o["$defs"]["FireSpec"]
        fs["properties"].pop("columnMethod", None)
        fs["required"] = [x for x in fs.get("required", []) if x != "columnMethod"]
        style = json.dumps(o, indent=2, ensure_ascii=False) + "\n"
        assert style.count("columnMethod") == 0
        return style
    p = ROOT / f"🧬️schema/{facet}🔣️.json"
    raw = p.read_text(encoding="utf-8")
    assert raw == json.dumps(json.loads(raw), indent=2, ensure_ascii=False) + "\n", "json style"
    edit(f"🧬️schema/{facet}🔣️.json", js)

# diff lane
edit("🧬️schema/🔺️diff/🦀️.rs", lambda t: rep(t, "fire_column_method => fire.column_method: String, ", ""))
edit("🧬️schema/🔺️diff/🟦️.ts", lambda t: rm(rm(t, r"  fireColumnMethod\??: [^\n]*\n", 1), r"fireColumnMethod: normWireOptional\(normWireString\), ", 1))
edit("🧬️schema/🔺️diff/🔗️.graphql", lambda t: rm(t, r"  fireColumnMethod:[^\n]*\n", 1))
def dproto(t):
    t = rm(t, r"  optional string fire_column_method = \d+;\n", 1)
    return t
edit("🧬️schema/🔺️diff/🛰️.proto", dproto)
def djson(t):
    o = json.loads(t)
    pt = o["$defs"]["En1992MembersPatch"]
    pt["properties"].pop("fireColumnMethod")
    pt["required"] = [x for x in pt["required"] if x != "fireColumnMethod"]
    return t.replace(json.dumps(json.loads(t)["$defs"]["En1992MembersPatch"], separators=(",", ":"), ensure_ascii=False), json.dumps(pt, separators=(",", ":"), ensure_ascii=False))
edit("🧬️schema/🔺️diff/🔣️.json", djson)

# leaf and its registrations
LEAF = "🏛️change-member-fire-column-method"
if (S / "🧬️mutations" / LEAF).exists() and not DRY:
    shutil.rmtree(S / "🧬️mutations" / LEAF)
fx = ROOT / "🧪️tests/🧫️fixtures/🧬️mutations" / LEAF
if fx.exists() and not DRY:
    shutil.rmtree(fx)
edit("🧬️schema/🧬️mutations/🏛️member-prestress-fire/🦀️.rs", lambda t: rm(t, r'#\[path = "\.\./🏛️change-member-fire-column-method/🦀️\.rs"\]\npub mod change_member_fire_column_method;\n', 1))
M = "🧬️schema/🧬️mutations/"
edit(M + "🦀️.rs", lambda t: rm(rm(rm(t, r"use [\w:]+::change_member_fire_column_method;\n", 1), r"    ChangeMemberFireColumnMethod\([^\n]*\n", 1), r'    "change-member-fire-column-method",\n', 1))
edit(M + "🟦️.ts", lambda t: rm(rm(rm(t, r"import \{ type ChangeMemberFireColumnMethod[^\n]*\n", 1), r"  \| \{ ChangeMemberFireColumnMethod: [^\n]*\n", 1), r"  ChangeMemberFireColumnMethod: parseChangeMemberFireColumnMethod,\n", 1))
def mjson(t):
    blk = re.compile(r'    \{\n      "type": "object",\n      "additionalProperties": false,\n      "required": \[\n        "ChangeMemberFireColumnMethod"\n      \],\n      "properties": \{\n        "ChangeMemberFireColumnMethod": \{\n          "\$ref": "[^"]*"\n        \}\n      \}\n    \},\n')
    return rm(t, blk.pattern, 1)
edit(M + "🔣️.json", mjson)
edit(M + "🧭️edit-rules/🦀️.rs", lambda t: rm(t, r'        SetFieldRule \{ path: "members\[\]\.fire\.columnMethod"[^\n]*\n', 1))
TEXT = "🚪️io/📝️text/🧬️mutations/🦀️.rs"
edit(TEXT, lambda t: rm(rm(rm(t, r"use [\w:]+::change_member_fire_column_method;\n", 1), r"        En1992Mutation::ChangeMemberFireColumnMethod\(p\) => [^\n]*\n", 1), r'        "change-member-fire-column-method" => [^\n]*\n', 1))
def proto(t):
    m = re.search(r"record change-member-fire-column-method tag=(\d+)\nfield payload bytes\n", t)
    if not m:
        return t
    tag = int(m.group(1))
    t = t.replace(m.group(0), "")
    return re.sub(r"(record [\w-]+ tag=)(\d+)", lambda x: x.group(1) + str(int(x.group(2)) - 1 if int(x.group(2)) > tag else int(x.group(2))), t)
edit("🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio", proto, must=False)

# oracles + tests
def oracles(t):
    t = rm(t, r'        \{\n          "mutationId": "change-member-fire-column-method",\n(?:.*\n)*?        \},\n', 1)
    t = rm(t, r'        "change-member-fire-column-method",\n', 1)
    t = rm(t, r'        \{\n          "id": "change-member-fire-column-method",\n(?:.*\n)*?        \},\n', 1)
    return t
edit("🔮️oracles/🔣️.json", oracles)
T = "🧪️tests/🏛️mutate-en1992-1/"
edit(T + "🥒️.feature", lambda t: rm(t, r"      \| change-member-fire-column-method \|[^\n]*\n", 2))
edit(T + "🐍️.py", lambda t: rm(rm(t, r'    "change-member-fire-column-method",\n', 1), r'    "change-member-fire-column-method": \([^\n]*\n', 1))
edit(T + "🦀️.rs", lambda t: rm(rm(t, r'    "change-member-fire-column-method",\n', 1), r'        "change-member-fire-column-method" => \(\n(?:            include_str![^\n]*\n){4}        \),\n', 1))
edit("🧬️schema/🧪️tests/⚖️compliance/🦀️.rs", lambda t: rep(t, '"columnMethod", "slabSystem"]', '"slabSystem"]'))

# editor
def meta(t):
    t = rm(t, r"const COLUMN_METHOD: &\[NormFieldChoice\] = &\[\n(?:    choice[^\n]*\n)+\];\n", 1)
    return rm(t, r'    \("members\[\]\.fire\.columnMethod"[^\n]*\n', 1)
edit("✏️editor/🏷️field-meta/🦀️.rs", meta)

# sqlite
IO = "🚪️io/🪶️sqlite/📸️snapshot/"
edit(IO + "🗄️.sql", lambda t: rep(t, "column_method TEXT NOT NULL, ", ""))
edit(IO + "🦀️.rs", lambda t: rep(rep(rep(t, "Cell::Text(&a.column_method),", ""), "column_method:text(a,3,c)?,", ""), "read_slab_fire_system(text(a,4,c)?)", "read_slab_fire_system(text(a,3,c)?)"))
edit(IO + "📏️cells/🦀️.rs", lambda t: rep(rep(t, '("en1992_fire",7)', '("en1992_fire",6)'), '&[(0,Fire),(1,Float),(2,Text),(3,Text)]', '&[(0,Fire),(1,Float),(2,Text)]'))
def sts(t):
    t = rep(t, "columnMethod:string;", "")
    t = rep(t, '"axisDistance","columnMethod","slabSystem"', '"axisDistance","slabSystem"')
    t = rep(t, "a.columnMethod,", "")
    t = rep(t, "columnMethod:artifactSqliteText(f,3),", "")
    return rep(t, "slabSystem:choice(artifactSqliteText(f,4),SLAB_FIRE_SYSTEMS)", "slabSystem:choice(artifactSqliteText(f,3),SLAB_FIRE_SYSTEMS)")
edit(IO + "🟦️.ts", sts)
edit(IO + "🧪️tests/🦀️.rs", lambda t: rep(t, "UPDATE en1992_fire SET column_method='independent 日本'", "UPDATE en1992_member SET label_en='independent 日本'"))
edit(IO + "🧪️tests/🟦️.ts", lambda t: rep(rep(rep(t, "UPDATE en1992_fire SET column_method='independent 日本'", "UPDATE en1992_member SET label_en='independent 日本'"), 'expect(edited.members[0]!.fire!.columnMethod).toBe("independent 日本")', 'expect(edited.members[0]!.labelEn).toBe("independent 日本")'), 'expect(proto).toContain("string column_method");', ""))

# fixtures, assets, examples (text level; the field is never the last key of its object)
for p in ROOT.rglob("*"):
    if p.is_dir() or any(x in p.parts for x in ("target", "node_modules")) or p.suffix not in (".json", ".semio"):
        continue
    try:
        t = p.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        continue
    if not re.search(r"columnMethod|fireColumnMethod|column-method|column_method", t):
        continue
    n = t
    if p.suffix == ".json":
        n = re.sub(r'"(?:fire)?[cC]olumnMethod"\s*:\s*(?:null|"[^"]*"),\s*', "", n)
    else:
        n = re.sub(r'column-method=(?:"[^"]*"|\S+) ', "", n)
    if n != t:
        changed.add(str(p.relative_to(ROOT)))
        if not DRY:
            p.write_text(n, encoding="utf-8")
print(("dry " if DRY else "") + f"changed {len(changed)} files")
left = [str(p.relative_to(ROOT)) for p in ROOT.rglob("*") if p.is_file() and "target" not in p.parts and "node_modules" not in p.parts and p.suffix in (".rs", ".ts", ".json", ".semio", ".proto", ".graphql", ".sql", ".py", ".feature") and re.search(r"column_method|columnMethod|ColumnMethod|column-method|COLUMN_METHOD", p.read_text(encoding="utf-8", errors="ignore"))]
print("remaining references:", left)
