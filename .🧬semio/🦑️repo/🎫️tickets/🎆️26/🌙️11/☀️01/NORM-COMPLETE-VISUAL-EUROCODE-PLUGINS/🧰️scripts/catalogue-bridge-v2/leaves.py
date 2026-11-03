"""Installs the seven design-case leaves of EN 1995 into another artifact: leaf directories, family crate `mut-design-cases`, workspace registration, root enum, KINDS, wire twins of the mutation root."""
import json, os, re, shutil
from lib import *
import crates as C


def subst(art, text, cases_path):
    text = text.replace("semio_s_artifact_norm_en1995_kernel::En1995Cases", cases_path)
    text = text.replace(T.emoji + "en1995", art.emoji + art.short)
    text = text.replace("en1995", art.short).replace("En1995", art.P).replace("EN 1995", "EN " + art.short[2:])
    return text


def install_leaf_dirs(art, cases_path, tags):
    made = []
    for k in KINDS:
        src = T.mut + "/" + LEAF_DIRS[k]
        dst = art.mut + "/" + LEAF_DIRS[k]
        assert not os.path.exists(dst), "leaf dir exists: " + dst
        for dp, dn, fn in os.walk(src):
            for f in fn:
                s = os.path.join(dp, f)
                d = os.path.join(dst, os.path.relpath(s, src))
                text = subst(art, rd(s), cases_path)
                if os.path.relpath(s, src) == "🔣️.json":
                    j = json.loads(text)
                    j["binaryTag"] = tags[k]
                    text = json.dumps(j, indent=2, ensure_ascii=False) + "\n"
                wr(d, text)
        made.append(dst)
    return made


def deps_of(toml):
    names = []
    for sec in ("dependencies", "dev-dependencies"):
        m = re.search(r"\[%s\]\n(.*?)(?:\n\[|\Z)" % sec, toml, re.S)
        if m:
            names += re.findall(r"^([A-Za-z0-9_-]+) =", m.group(1), re.M)
    return sorted(set(names))


def first_mut_crate(art):
    t = rd(art.mut + "/📦️packages/🦀️rust/Cargo.toml")
    return re.search(r"^(%smut-[a-z0-9-]+) = \{ workspace = true \}$" % re.escape(art.crate), t, re.M).group(1)


def family_crate(art, catalogue_deps):
    leaves = [(LEAF_DIRS[k], MODULE[k]) for k in KINDS]
    d = C.create(art, leaves, catalogue_deps, "🗂️ EN %s design-case mutation leaves: insert, remove, set number, set text, label and entry id." % art.short[2:], "EN %s design-case mutation leaves" % art.short[2:])
    cname = art.crate + "mut-" + C.FAMILY
    anchor = first_mut_crate(art)
    consumers = C.register(art, cname, d, deps_of(rd(d + "/📦️packages/🦀️rust/Cargo.toml")), anchor=anchor)
    return cname, consumers


def ident(name):
    return name.replace("-", "_")


def root_enum(art, cname):
    p = art.mut + "/🦀️.rs"
    s = rd(p)
    d = s.index("#[derive(Clone, Debug, PartialEq, dsl::Mutations")
    uses = list(re.finditer(r"^use [\w:{}, ]+;\n", s[:d], re.M))
    at = uses[-1].end()
    new = "".join("use %s::%s;\n" % (ident(cname), MODULE[k]) for k in KINDS)
    s = s[:at] + new + s[at:]
    s = rep1(s, "\n}\n\n", "\n" + "".join("    %s(%s::%s),\n" % (RECORD[k], MODULE[k], RECORD[k]) for k in KINDS).rstrip("\n") + "\n}\n\n", s.index("pub enum %sMutation" % art.P))
    i = s.index("pub const KINDS")
    j = s.index("\n];", i)
    s = s[:j] + "\n" + "".join('    "%s",\n' % k for k in KINDS).rstrip("\n") + s[j:]
    wr(p, s)


def rep1(s, old, new, start):
    i = s.index(old, start)
    return s[:i] + new + s[i + len(old):]


def root_json(art):
    p = art.mut + "/🔣️.json"
    j = json.loads(rd(p))
    external = "$ref" not in j["oneOf"][-1]
    for k in KINDS:
        ref = {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/%s/mutation/%s/schema.json" % (art.short, k)}
        j["oneOf"].append({"type": "object", "additionalProperties": False, "required": [RECORD[k]], "properties": {RECORD[k]: ref}} if external else ref)
    wr(p, json.dumps(j, indent=2, ensure_ascii=False) + "\n")
    return external


def root_ts(art, external):
    p = art.mut + "/🟦️.ts"
    s = rd(p)
    imports = "".join('import { type %s, parse%s } from "./%s/🧬️schema/🟦️.ts";\n' % (RECORD[k], RECORD[k], LEAF_DIRS[k]) for k in KINDS)
    i = s.index("\nexport type %sMutation =" % art.P)
    s = s[:i].rstrip("\n") + "\n" + imports + s[i:]
    j = s.index(";\n", s.index("export type %sMutation =" % art.P))
    s = s[:j] + "".join("\n  | %s" % ("{ %s: %s }" % (RECORD[k], RECORD[k]) if external else RECORD[k]) for k in KINDS) + s[j:]
    kk = s.rindex("\n});")
    s = s[:kk] + "".join("\n  %s: parse%s," % (RECORD[k] if external else RECORD[k][:1].lower() + RECORD[k][1:], RECORD[k]) for k in KINDS) + s[kk:]
    wr(p, s)


SET_RULES = """        SetFieldRule { path: "cases[].labelEn", kind: "change-case-label-en", selectors: &[SelectorKey::Id("caseId")], value: "newValue" },
        SetFieldRule { path: "cases[].labelDe", kind: "change-case-label-de", selectors: &[SelectorKey::Id("caseId")], value: "newValue" },
        SetFieldRule { path: "cases[].entryId", kind: "change-case-entry-id", selectors: &[SelectorKey::Id("caseId")], value: "newValue" },
        SetFieldRule { path: "cases[].numbers[].value", kind: "set-case-number", selectors: &[SelectorKey::Id("caseId"), SelectorKey::Id("key")], value: "value" },
        SetFieldRule { path: "cases[].texts[].value", kind: "set-case-text", selectors: &[SelectorKey::Id("caseId"), SelectorKey::Id("key")], value: "value" },
"""
INSERT_RULE = '        InsertItemRule { path: "cases", kind: "insert-case", selectors: &[], index: "index", item: "case" },\n'
REMOVE_RULE = '        RemoveItemRule { path: "cases", kind: "remove-case", selectors: &[], index: "index" },\n'


def edit_rules(art):
    p = art.mut + "/🧭️edit-rules/🦀️.rs"
    s = rd(p)
    assert "cases[]" not in s
    i = s.index("    set_field: &[\n")
    j = s.index("    ],\n", i)
    s = s[:j] + SET_RULES + s[j:]
    i = s.index("    insert_item: &[\n")
    j = s.index("    ],\n", i)
    s = s[:j] + INSERT_RULE + s[j:]
    i = s.index("    remove_item: &[\n")
    j = s.index("    ],\n", i)
    s = s[:j] + REMOVE_RULE + s[j:]
    wr(p, s)
