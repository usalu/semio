"""EN 1993 wiring: group crates, row kinds cloned from the crane template, externally tagged root twins, edit rules, codecs, harness, oracle, taxonomy, fixtures, retirement of the `update-*-inputs` kinds."""
import copy, glob, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import common, leaf as L, delta as D, wire as W
import en1993 as X

art = X.art
ROOT_REL = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/" + art.name + "/" + art.rel
CRANE = {"insert": "➕️insert-crane-runway", "remove": "➖️remove-crane-runway"}
PREAMBLE = "#![allow(async_fn_in_trait)]\n\nextern crate semio_framework_os_kernel as dsl;\nextern crate semio_framework_os_kernel as protocol;\nextern crate semio_framework_os_kernel as store;\nextern crate semio_framework_schema as framework_schema;\nextern crate semio_framework_value_derive as value_derive;\n\n"


def us(c):
    return c.replace("-", "_")


def kind_dir(kind):
    for d in os.listdir(art.mut):
        if re.sub(r"^[^a-z]+", "", d) == kind and os.path.isdir(art.mut + "/" + d):
            return d
    return None


def clone_row_kind(F, c, df, verb):
    """➕ Clones the crane-runway insert/remove kind onto the new collection `df`."""
    e = c.by_def[df]
    k = X.def_kebab(df)
    kind = "%s-%s" % (verb, k)
    src = art.mut + "/" + CRANE[verb]
    dst = art.mut + "/" + emoji_dir("➕️" if verb == "insert" else "➖️", kind)
    patch = c.patches[df]
    delta = c.deltas[patch]
    item = c.items[df]
    toks = [("En1993CraneRunwayDelta", delta), ("Crane runway beam", e.en), ("crane runway beam", e.en.lower()), ("Kranbahnträger", e.de), ("Crane runway", e.en), ("crane runway", e.en.lower()), ("Kranbahn", e.de), ("crane_runways", e.chain[0][0]), ("crane_runway", snake(item)), ("craneRunway", item), ("crane-runway", k), ("CraneRunway", df)]
    for dirpath, _, files in os.walk(src):
        for f in files:
            rel = os.path.relpath(dirpath + "/" + f, src)
            t = rd(src + "/" + rel)
            for a, b in toks:
                t = t.replace(a, b)
            if rel == "🔣️.json":
                j = json.loads(t)
                j["owner"] = ROOT_REL + "/" + os.path.basename(dst)
                j["binaryTag"] = c.tags[kind]
                t = dumps(j)
            F.write(dst + "/" + rel, t)
    return dst


def write_root(F, path, body):
    mods = re.findall(r'#\[path = "\.\./([^"]+)/🦀️\.rs"\]\npub mod (\w+);', body)
    F.write(path, mount_into(F.read(path), mods) if F.exists(path) else body)


def create_crates(F, c):
    base = art.mut + "/🪝️cranes/📦️packages/🦀️rust/Cargo.toml"
    names = {}
    for g, ls in c.groups.items():
        ent = ls[0].ent
        cname = art.crate + g
        d = art.mut + "/" + emoji_dir(ent.emoji, g)
        body = "//! %s EN 1993 %s mutation leaves: %s.\n\n" % (ent.emoji, ent.en.lower(), ", ".join(l.field.replace("_", " ") for l in ls[:3]) + (" and more" if len(ls) > 3 else "")) + PREAMBLE + "//#region 🪢️Mounts\n" + "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (l.dirname, l.module) for l in ls) + "//#endregion 🪢️Mounts\n"
        write_root(F, d + "/🦀️.rs", body)
        if not F.exists(d + "/📦️packages/🦀️rust/Cargo.toml"):
            F.write(d + "/📦️packages/🦀️rust/Cargo.toml", W.crate_toml(art, cname, base))
        names[g] = (cname, d)
    for g, emoji, defs_ in X.ROW_CRATES:
        present = [df for df in defs_ if df in c.by_def]
        if not present or not any("insert-" + X.def_kebab(df) in c.row_kinds for df in present):
            continue
        cname = art.crate + g
        d = art.mut + "/" + emoji_dir(emoji, g)
        mods = [(v + "-" + X.def_kebab(df)) for df in present for v in ("insert", "remove")]
        body = "//! %s EN 1993 row mutation leaves: %s.\n\n" % (emoji, ", ".join(X.def_kebab(df) for df in present)) + PREAMBLE + "//#region 🪢️Mounts\n" + "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (emoji_dir("➕️" if m.startswith("insert") else "➖️", m), us(m)) for m in mods) + "//#endregion 🪢️Mounts\n"
        write_root(F, d + "/🦀️.rs", body)
        if not F.exists(d + "/📦️packages/🦀️rust/Cargo.toml"):
            F.write(d + "/📦️packages/🦀️rust/Cargo.toml", W.crate_toml(art, cname, base))
        names[g] = (cname, d)
        c.row_mods.update({m: cname for m in mods})
    return names


def register(F, c, names):
    f = art.root + "/Cargo.toml"
    s = F.read(f)
    for g, (cname, d) in names.items():
        rel = os.path.relpath(d + "/📦️packages/🦀️rust", art.root)
        if '"%s"' % rel not in s:
            s = s.replace("members = [\n", 'members = [\n    "%s",\n' % rel, 1)
        if "[workspace.dependencies.%s]" % cname not in s:
            s = s.replace("[workspace.lints.rust]", '[workspace.dependencies.%s]\npath = "%s"\n\n[workspace.lints.rust]' % (cname, rel), 1)
    F.write(f, s)
    f = art.root + "/📦️packages/🦀️rust/📜️script.ts"
    s = F.read(f)
    anchor = '"semio-s-artifact-norm-en1993-mut-general",'
    F.write(f, rep(s, anchor, anchor + "".join('"%s",' % n for n, _ in names.values() if '"%s"' % n not in s)))
    s = F.read(OWNERS)
    m = re.search(r'    \{\n      "name": "%s",\n.*?\n    \},\n' % re.escape(art.anchor_crate), s, re.S)
    ent = m.group(0)
    new = ""
    for cname, d in names.values():
        if '"name": "%s"' % cname in s:
            continue
        new += ent.replace(art.anchor_crate, cname).replace(os.path.relpath(art.mut + "/" + art.anchor_dir + "/📦️packages/🦀️rust", SEMIO), os.path.relpath(d + "/📦️packages/🦀️rust", SEMIO))
    F.write(OWNERS, s.replace(ent, ent + new, 1))
    anchor = "semio-s-artifact-norm-en1993-mut-towers = { workspace = true }\n"
    for pkg in [art.mut + "/📦️packages/🦀️rust/Cargo.toml", art.any + "/🚪️io/📦️packages/🦀️rust/Cargo.toml"]:
        s = F.read(pkg)
        F.write(pkg, rep(s, anchor, anchor + "".join("%s = { workspace = true }\n" % n for n, _ in names.values() if (n + " =") not in s)))


def leaf_files(F, c):
    for l in c.new_leaves:
        d = art.mut + "/" + l.dirname
        patch = c.patches[l.ent.chain[0][1]]
        F.write(d + "/🦀️.rs", L.leaf_rs(l))
        F.write(d + "/🔺️diff/🦀️.rs", D.diff_rs(l, c.deltas[patch], patch))
        F.write(d + "/↩️inverse/🦀️.rs", L.inverse_rs(l))
        F.write(d + "/🧬️schema/🔣️.json", dumps(D.schema_json(l, c.defs)))
        F.write(d + "/🧬️schema/🟦️.ts", D.schema_ts(l))
        F.write(d + "/🔣️.json", dumps(L.catalog_json(l, c.tags[l.kind], ROOT_REL)))
    for df in X.ROW_ORDER:
        if df in c.by_def:
            for v in ("insert", "remove"):
                if "%s-%s" % (v, X.def_kebab(df)) in c.row_kinds:
                    clone_row_kind(F, c, df, v)


def rec_of(kind):
    return pascal(kind)


def dir_of(c, kind):
    if kind in c.leaf_by_kind:
        return c.leaf_by_kind[kind].dirname
    if kind.startswith(("insert-", "remove-")) and kind in c.row_kinds:
        return emoji_dir("➕️" if kind.startswith("insert") else "➖️", kind)
    return kind_dir(kind)


def root_rs(F, c):
    p = art.mut + "/🦀️.rs"
    s = F.read(p)
    ret = c.retired
    mods = {k.replace("-", "_") for k in ret}
    recs = {pascal(k) for k in ret}
    s = "".join(l for l in s.splitlines(True) if not (l.startswith("use ") and any(re.search(r"::%s;" % m, l) for m in mods)))
    s = "".join(l for l in s.splitlines(True) if not re.match(r"    (%s)\(" % "|".join(recs), l))
    s = "".join(l for l in s.splitlines(True) if not re.match(r'    "(%s)",$' % "|".join(ret), l))
    uses = ""
    for k in c.row_kinds + [l.kind for l in c.new_leaves]:
        crate = c.row_mods.get(k) or c.crate_of[k]
        uses += "use %s::%s;\n" % (us(crate), us(k))
    s = rep(s, "//#endregion 🔖️Leaves", uses + "//#endregion 🔖️Leaves")
    variants = "".join("    %s(%s::%s),\n" % (pascal(k), us(k), pascal(k)) for k in c.row_kinds + [l.kind for l in c.new_leaves])
    s = rep(s, "\n}\n\npub const KINDS", "\n" + variants.rstrip("\n") + "\n}\n\npub const KINDS")
    s = rep(s, "\n];\n//#endregion 🔖️Mutations", "\n" + "".join('    "%s",\n' % k for k in c.row_kinds + [l.kind for l in c.new_leaves]).rstrip("\n") + "\n];\n//#endregion 🔖️Mutations")
    F.write(p, s)


def root_ts(F, c):
    p = art.mut + "/🟦️.ts"
    s = F.read(p)
    recs = {pascal(k) for k in c.retired}
    pat = "|".join(recs)
    s = s if not recs else "".join(l for l in s.splitlines(True) if not (re.search(r"\b(?:type |parse)(%s)\b" % pat, l) and l.startswith("import")) and not re.match(r"  \| \{ (%s): " % pat, l) and not re.match(r"  (%s): " % pat, l))
    new = c.row_kinds + [l.kind for l in c.new_leaves]
    imports = "".join('import { type %s, parse%s } from "./%s/🧬️schema/🟦️.ts";\n' % (pascal(k), pascal(k), dir_of(c, k)) for k in new)
    i = s.index("\nexport type En1993Mutation =")
    s = s[:i].rstrip("\n") + "\n" + imports + s[i:]
    j = s.index(";\n\nexport const parseEn1993Mutation")
    s = s[:j] + "".join("\n  | { %s: %s }" % (pascal(k), pascal(k)) for k in new) + s[j:]
    k = s.index("\n});", s.index("normWireExternal"))
    s = s[:k] + "".join("\n  %s: parse%s," % (pascal(x), pascal(x)) for x in new) + s[k:]
    F.write(p, s)


def root_json(F, c):
    p = art.mut + "/🔣️.json"
    j = json.loads(F.read(p))
    ret = {pascal(k) for k in c.retired}
    j["oneOf"] = [x for x in j["oneOf"] if not (set(x["required"]) & ret)]
    for k in c.row_kinds + [l.kind for l in c.new_leaves]:
        j["oneOf"].append({"type": "object", "additionalProperties": False, "required": [pascal(k)], "properties": {pascal(k): {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/en1993/mutation/%s/schema.json" % k}}})
    F.write(p, "{\n" + ",\n".join("  %s:%s" % (json.dumps(k), json.dumps(v, ensure_ascii=False, separators=(",", ":")) if k != "oneOf" else "[\n" + ",\n".join("    " + json.dumps(x, ensure_ascii=False, separators=(",", ":")) for x in v) + "\n  ]") for k, v in j.items()) + "\n}\n")


def gql_blocks(c):
    out = ""
    for l in c.new_leaves:
        out += "type %s { id: String! @state(class: ARTIFACT) %s: %s! @state(class: ARTIFACT) }\n" % (l.record, l.newc, L.gql_ty(l))
    return out


def root_graphql(F, c):
    p = art.mut + "/🔗️.graphql"
    head = "# 🧬️ En1993 mutation schema — artifact-lane fields only.\n\ntype En1993Mutation {\n  annex: String! @state(class: ARTIFACT)\n}\n\n"
    rows = "".join("type %s { index: Int! @state(class: ARTIFACT) }\n" % pascal(k) for k in c.row_kinds if k.startswith("remove"))
    rows += "".join("type %s { index: Int @state(class: ARTIFACT) }\n" % pascal(k) for k in c.row_kinds if k.startswith("insert"))
    F.write(p, head + rows + gql_blocks(c).rstrip("\n") + "\n")


def root_proto(F, c):
    p = art.mut + "/🛰️.proto"
    s = F.read(p)
    ret = {pascal(k) for k in c.retired}
    pat = "|".join(ret)
    s = "".join(l for l in s.splitlines(True) if not re.match(r"message (%s) \{" % pat, l))
    head, tail = s.split("message En1993Mutation {", 1)
    body = ""
    for k in c.row_kinds:
        body += ("message %s { uint32 index = 1; bytes payload = 2; }\n" if k.startswith("insert") else "message %s { uint32 index = 1; }\n") % pascal(k)
    for l in c.new_leaves:
        body += "message %s { string id = 1; %s %s = 2; }\n" % (l.record, "string" if l.ty == "String" else L.proto_ty(l), l.new)
    head = head.rstrip("\n") + "\n" + body + "\n"
    kinds = [k for k in c.kinds_all]
    oneof = "".join("    %s %s = %d;\n" % (pascal(k), us(k), i + 1) for i, k in enumerate(kinds))
    F.write(p, head + "message En1993Mutation {\n  oneof payload {\n" + oneof + "  }\n}\n")


def edit_rules(F, c):
    p = art.mut + "/🧭️edit-rules/📚️table/🦀️.rs"
    s = F.read(p)
    rules = "".join('        SetFieldRule { path: "%s", kind: "%s", selectors: &[SelectorKey::Id("id")], value: "%s" },\n' % (X.rule_path(l), l.kind, l.newc) for l in c.new_leaves)
    s = rep(s, "    ],\n    insert_item:", rules + "    ],\n    insert_item:")
    ins = "".join('        InsertItemRule { path: "%s", kind: "insert-%s", selectors: &[], index: "index", item: "%s" },\n' % (camel(c.by_def[df].chain[0][0]), X.def_kebab(df), c.items[df]) for df in X.ROW_ORDER if df in c.by_def and "insert-" + X.def_kebab(df) in c.row_kinds)
    rem = "".join('        RemoveItemRule { path: "%s", kind: "remove-%s", selectors: &[], index: "index" },\n' % (camel(c.by_def[df].chain[0][0]), X.def_kebab(df)) for df in X.ROW_ORDER if df in c.by_def and "remove-" + X.def_kebab(df) in c.row_kinds)
    s = rep(s, "    ],\n    remove_item:", ins + "    ],\n    remove_item:")
    s = rep(s, "    ],\n};\n", rem + "    ],\n};\n")
    F.write(p, s)


def text_codec(F, c):
    p = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    s = F.read(p)
    mods = {k.replace("-", "_") for k in c.retired}
    s = "".join(l for l in s.splitlines(True) if not (l.startswith("use ") and any(re.search(r"::%s;" % m, l) for m in mods)))
    uses = "".join("use %s::%s;\n" % (us(c.row_mods.get(k) or c.crate_of[k]), us(k)) for k in c.row_kinds + [l.kind for l in c.new_leaves])
    F.write(p, rep(s, "\npub fn decode_en1993_mutation_json", uses + "\npub fn decode_en1993_mutation_json"))


def binary_codec(F, c):
    p = art.any + "/🚪️io/💾️binary/🧬️mutations/🦀️.rs"
    s = F.read(p)
    old = s[s.index("fn kind_of"): s.index("impl protocol::OpBinary")]
    new = '''fn kind_of(json: &str) -> Option<String> {
    let rest = json.trim_start().strip_prefix('{')?.trim_start().strip_prefix('"')?;
    let name = &rest[..rest.find('"')?];
    let mut kebab = String::new();
    for c in name.chars() {
        if c.is_uppercase() {
            if !kebab.is_empty() {
                kebab.push('-');
            }
            kebab.extend(c.to_lowercase());
        } else {
            kebab.push(c);
        }
    }
    Some(kebab)
}

'''
    F.write(p, s.replace(old, new))
    p = art.any + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = F.read(p)
    F.write(p, protocol_merge(s, c.kinds_all))


def extend_block(text, shape, anchor, lines):
    """➕ Appends `lines` after the last line of the contiguous run of `shape` lines that holds the first `anchor` line."""
    ls = text.splitlines(True)
    i = next(n for n, l in enumerate(ls) if anchor in l and re.fullmatch(shape, l))
    j = i
    while j < len(ls) and re.fullmatch(shape, ls[j]):
        j += 1
    return "".join(ls[:j]) + lines + "".join(ls[j:])


def harness(F, c):
    h = art.harness
    ret = "|".join(c.retired)
    new = c.row_kinds + [l.kind for l in c.new_leaves]
    p = h + "/🦀️.rs"
    s = F.read(p)
    s = re.sub(r'        "(?:%s)" => \(\n(?:            [^\n]*\n)+        \),\n' % ret, "", s)
    s = re.sub(r'    \("(?:%s)", \(\n(?:        [^\n]*\n)+    \)\),\n' % ret, "", s)
    s = "".join(l for l in s.splitlines(True) if not re.match(r'    "(%s)",$' % ret, l))
    s = extend_block(s, r'    "[a-z0-9-]+",\n', '"remove-placement"', "".join('    "%s",\n' % k for k in new))
    arms = "".join('    ("%s", fixture!("%s")),\n' % (k, dir_of(c, k)) for k in new)
    macro = "#[cfg(feature = \"sut\")]\nmacro_rules! fixture {\n    ($d:literal) => {\n        (\n" + "".join('            include_str!(concat!("../🧫️fixtures/🧬️mutations/", $d, "/✅apply/🔣️.%s.json")),\n' % x for x in ["before", "mutation", "after", "outcome"]) + "        )\n    };\n}\n\n"
    if "macro_rules! fixture" not in s:
        s = rep(s, "/// 🗃️ The committed before, mutation, after and outcome fixture texts", macro + "/// 🗃️ The committed before, mutation, after and outcome fixture texts")
    s = extend_block(s, r'    \("[a-z0-9-]+", fixture!\(".*"\)\),\n', '"remove-placement"', arms)
    total = len(re.findall(r'^    "[a-z0-9-]+",$', s.split("//#region 🔖️Kinds")[1].split("];")[0], re.M))
    s = re.sub(r"current \d+-kind", "current %d-kind" % total, s, 1)
    F.write(p, s)
    p = h + "/🐍️.py"
    s = F.read(p)
    s = "".join(l for l in s.splitlines(True) if not re.match(r'    "(%s)"[:,]' % ret, l))
    s = extend_block(s, r'    "[a-z0-9-]+",\n', '"remove-placement"', "".join('    "%s",\n' % k for k in new))
    s = extend_block(s, r'    "[a-z0-9-]+": \("[^"]+", "✅apply"\),\n', '"remove-placement"', "".join('    "%s": ("%s", "✅apply"),\n' % (k, dir_of(c, k)) for k in new))
    F.write(p, s)
    p = h + "/🥒️.feature"
    s = F.read(p)
    s = "".join(l for l in s.splitlines(True) if not re.match(r"      \| (%s) +\|" % ret, l))
    rows = "".join("      | %s | %s | ✅apply  |\n" % (k, dir_of(c, k)) for k in new)
    s = extend_block(s, r"      \| [a-z0-9-]+ +\|.*\n", "| remove-placement ", rows)
    s = re.sub(r"The \d+ `✅apply` vectors(\s+)cover every kind of the current vocabulary \([^)]*\)", lambda m: "The %d `✅apply` vectors%scover every kind of the current vocabulary (%d `change`, %d `insert`, %d `remove`)" % (total, m.group(1), 1 + len(c.new_leaves), 16 + len(c.row_kinds) // 2, 16 + len(c.row_kinds) // 2), s, 1)
    F.write(p, s)


def oracle(F, c):
    p = art.any + "/🔮️oracles/🔣️.json"
    o = json.loads(F.read(p))
    cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
    cat["vectors"] = [v for v in cat["vectors"] if v["mutationId"] not in c.retired]
    cat["kinds"] = [k for k in cat["kinds"] if k not in c.retired]
    man["mutations"] = [m for m in man["mutations"] if m["id"] not in c.retired]
    vt = [v for v in cat["vectors"] if v["mutationId"] == "change-annex"][0]
    mt = [m for m in man["mutations"] if m["id"] == "change-annex"][0]
    for k in c.row_kinds + [l.kind for l in c.new_leaves]:
        v = copy.deepcopy(vt)
        v["mutationId"] = k
        v["sourceMutationDirectoryName"] = v["mutationDirectoryName"] = dir_of(c, k)
        cat["vectors"].append(v)
        cat["kinds"].append(k)
        m = copy.deepcopy(mt)
        m["id"] = k
        m["productionDispatch"]["operation"] = k
        m["productionDispatch"]["variant"] = pascal(k)
        man["mutations"].append(m)
    F.write(p, dumps(o))


def taxonomy(F, c):
    t = json.loads(F.read(TAXONOMY))
    t["rows"] = [r for r in t["rows"] if not (r["artifact"] == art.name and r["kind"] in c.retired)]
    last = max(i for i, x in enumerate(t["rows"]) if x["artifact"] == art.name)
    tmpl = {r["kind"]: r for r in t["rows"] if r["artifact"] == art.name}
    new = []
    for k in c.row_kinds:
        r = copy.deepcopy(tmpl["%s-crane-runway" % k.split("-")[0]])
        df = [d for d in X.ROW_ORDER if k.endswith("-" + X.def_kebab(d))][0]
        r.update({"aggregateVariant": pascal(k), "entity": c.items[df], "kind": k, "module": dir_of(c, k), "record": ("Inserted" if k.startswith("insert") else "Removed") + df, "type": pascal(k)})
        r["source"] = "%s/%s/%s/🦀️.rs" % (art.name, art.rel, dir_of(c, k))
        new.append(r)
    for l in c.new_leaves:
        new.append({"aggregateVariant": l.record, "artifact": art.name, "entity": l.ent.name + "-" + kebab(l.field), "kind": l.kind, "module": l.dirname, "physicalLayout": "direct", "record": "Changed" + l.record[len("Change"):], "source": "%s/%s/%s/🦀️.rs" % (art.name, art.rel, l.dirname), "standard": "🔖️1", "subset": "✳️any", "type": l.record, "verb": "change"})
    have = {r["kind"] for r in t["rows"] if r["artifact"] == art.name}
    t["rows"][last + 1: last + 1] = [r for r in new if r["kind"] not in have]
    F.write(TAXONOMY, dumps(t))


def catalog_tags(F, c):
    for k in c.keep:
        d = dir_of(c, k)
        p = art.mut + "/" + d + "/🔣️.json"
        j = json.loads(F.read(p))
        j["binaryTag"] = c.tags[k]
        F.write(p, dumps(j))


def fixtures(F, c):
    base_p = art.fix + "/➕️insert-crane-runway/✅apply/🔣️.before.json"
    import en1993_base
    base = en1993_base.base_doc(art, c.defs)
    diff0 = json.loads(rd(art.fix + "/➕️insert-crane-runway/✅apply/🔣️.diff.json"))
    snap_rs = rd(art.any + "/🧬️schema/📸️snapshot/🦀️.rs")
    members = [r["id"] for r in base.get("members", [])]
    for e in c.ents:
        key = camel(e.chain[0][0])
        if ("pub %s: Vec<" % e.chain[0][0]) in snap_rs and not base.get(key):
            base[key] = [X.synth(c.defs, e.chain[0][1], e.slug + "-1", members)]
    lanes = lambda: {"removed": [], "inserted": [], "moved": [], "modified": []}
    top = ["annex"] + [camel(e.chain[0][0]) for e in c.ents]
    for k in diff0:
        if k not in top:
            top.append(k)
    mk = lambda: {k: (None if k == "annex" else lanes()) for k in top}
    for l in c.new_leaves:
        e = l.ent
        key = camel(e.chain[0][0])
        row = base[key][0]
        patch = c.patches[e.chain[0][1]]
        val = X.new_value(l, X.get(row, l))
        after = copy.deepcopy(base)
        X.put(after[key][0], l, val)
        field = camel(l.path[0])
        pv = after[key][0][field]
        patch_json = {camel(f): None for f in c.patch_fields[patch]}
        patch_json[field] = pv
        diff = mk()
        diff[key]["modified"] = [{"id": row["id"], "patch": patch_json}]
        out = art.fix + "/" + l.dirname + "/✅apply"
        F.write(out + "/🔣️.before.json", dumps(base))
        F.write(out + "/🔣️.mutation.json", dumps({l.record: {"id": row["id"], l.newc: val}}))
        F.write(out + "/🔣️.after.json", dumps(after))
        F.write(out + "/🔣️.outcome.json", dumps({"status": "applied"}))
        F.write(out + "/🔣️.diff.json", dumps(diff))
    for df in X.ROW_ORDER:
        if df not in c.by_def:
            continue
        e = c.by_def[df]
        key = camel(e.chain[0][0])
        k = X.def_kebab(df)
        if "insert-" + k not in c.row_kinds:
            continue
        first = base[key][0]
        extra = copy.deepcopy(first)
        extra["id"] = first["id"] + "-2"
        one = copy.deepcopy(base)
        two = copy.deepcopy(base)
        two[key] = [first, extra]
        for verb, before, after in (("insert", one, two), ("remove", two, one)):
            kind = "%s-%s" % (verb, k)
            diff = mk()
            if verb == "insert":
                diff[key]["inserted"] = [{"index": 1, "row": extra}]
                mut = {pascal(kind): {"index": 1, c.items[df]: extra}}
            else:
                diff[key]["removed"] = [{"id": extra["id"], "index": 1}]
                mut = {pascal(kind): {"index": 1}}
            out = art.fix + "/" + dir_of(c, kind) + "/✅apply"
            F.write(out + "/🔣️.before.json", dumps(before))
            F.write(out + "/🔣️.mutation.json", dumps(mut))
            F.write(out + "/🔣️.after.json", dumps(after))
            F.write(out + "/🔣️.outcome.json", dumps({"status": "applied"}))
            F.write(out + "/🔣️.diff.json", dumps(diff))


def unit_tests(F, c):
    p = art.mut + "/🧪️tests/🔬️unit/🦀️.rs"
    s = F.read(p)
    s = re.sub(r"#\[test\]\nfn update_\w+\(\) \{.*?\n\}\n\n?", "", s, flags=re.S)
    s = "".join(l for l in s.splitlines(True) if not (l.startswith("use semio_s_artifact_norm_en1993_mut_") and re.search(r"\bupdate_\w+", l) and "change_annex" not in l))
    s = s.replace("{change_annex, update_member_properties, update_stainless_inputs, update_through_thickness_inputs}", "change_annex")
    s = s.replace("hierarchical entity upserts", "typed leaf and row kinds")
    s = s.replace("fn every_special_part_leaf_resolves_to_its_whole_entity_update", "fn every_special_part_leaf_resolves_to_one_typed_leaf_kind")
    m = re.search(r"use semio_s_artifact_norm_en1993_model::\{([^}]*)\};\n", s)
    body = s.replace(m.group(0), "")
    used = [n.strip() for n in m.group(1).split(",") if n.strip() and re.search(r"\b%s\b" % n.strip(), body)]
    s = s.replace(m.group(0), ("use semio_s_artifact_norm_en1993_model::{%s};\n" % ", ".join(used)) if used else "")
    s += '''
#[test]
fn core_entity_leaves_resolve_to_one_typed_kind() {
    use semio_framework_value::ToValue;
    use semio_s_plugin_norm_app_surface::NormEdit;
    let before = base();
    let cases = [
        (format!("materials[id={}].fy", before.materials[0].id), 123.0e6_f64),
        (format!("sections[id={}].area", before.sections[0].id), 0.0123),
        (format!("members[id={}].length", before.members[0].id), 9.0),
    ];
    for (path, number) in cases {
        let mutations = crate::resolve(&before, &NormEdit::SetField { path: path.clone(), value: number.to_value() }).unwrap_or_else(|error| panic!("{path}: {error}"));
        assert_eq!(mutations.len(), 1, "{path}");
        let (after, _) = apply_en1993_mutation(&before, &mutations[0]).expect("apply");
        assert_ne!(after, before, "{path}");
    }
}
'''
    F.write(p, s)


def schema_index(F, c):
    p = SEMIO + "/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json"
    d = json.loads(F.read(p))
    m = d["roots"]["✏️s/🔌️plugins/📕️norm"]
    base = "https://json.schemas.assets.semio-tech.com/s/norm/en1993/mutation/%s/schema.json"
    for k in c.retired:
        m.pop(base % k, None)
    for k in c.row_kinds + [l.kind for l in c.new_leaves]:
        m[base % k] = ROOT_REL + "/" + dir_of(c, k) + "/🧬️schema/🔣️.json"
    d["roots"]["✏️s/🔌️plugins/📕️norm"] = dict(sorted(m.items()))
    F.write(p, dumps(d))


def retire(c):
    removed = []
    for k in c.retired:
        d = kind_dir(k)
        if d:
            shutil.rmtree(art.mut + "/" + d)
            removed.append(d)
        for fd in glob.glob(art.fix + "/*"):
            if re.sub(r"^[^a-z]+", "", os.path.basename(fd)) == k:
                shutil.rmtree(fd)
    mods = [k.replace("-", "_") for k in c.retired]
    for crate_rs in glob.glob(art.mut + "/*/🦀️.rs"):
        t = rd(crate_rs)
        u = t
        for m in mods:
            u = re.sub(r'#\[path = "\.\./[^"]*"\]\npub mod %s;\n' % m, "", u)
            u = re.sub(r"\b%s, " % m, "", u)
            u = re.sub(r", %s\b" % m, "", u)
        if u != t:
            open(crate_rs, "w", encoding="utf-8").write(u)
    sub = art.any.rsplit("/", 1)[0] + "/🔣️.json"
    t = rd(sub)
    u = t.replace("every mutation (change-annex, update-member-properties, update-fire-inputs, update-cold-formed-inputs, update-stainless-inputs, update-plated-inputs, …)", "every mutation (change-annex, one typed change-<entity>-<field> kind per leaf, insert-/remove-<row> per collection)")
    if u != t:
        open(sub, "w", encoding="utf-8").write(u)
    return removed


def run():
    c = X.main()
    F = c.F
    common.STAGE = X.STAGE93
    c.leaf_by_kind = {l.kind: l for l in c.new_leaves}
    c.row_mods = {}
    shutil.rmtree(X.STAGE93, ignore_errors=True)
    names = create_crates(F, c)
    c.crate_of = {l.kind: names[g][0] for g, ls in c.groups.items() for l in ls}
    register(F, c, names)
    leaf_files(F, c)
    catalog_tags(F, c)
    root_rs(F, c)
    root_ts(F, c)
    root_json(F, c)
    root_graphql(F, c)
    root_proto(F, c)
    edit_rules(F, c)
    text_codec(F, c)
    binary_codec(F, c)
    harness(F, c)
    oracle(F, c)
    taxonomy(F, c)
    fixtures(F, c)
    unit_tests(F, c)
    schema_index(F, c)
    print(c.stats, "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply(), "removed", "0 (generators never delete; retire by hand)")


if __name__ == "__main__":
    run()
