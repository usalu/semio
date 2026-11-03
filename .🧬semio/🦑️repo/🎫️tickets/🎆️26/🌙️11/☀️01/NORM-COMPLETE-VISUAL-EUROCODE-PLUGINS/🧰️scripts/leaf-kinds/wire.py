"""Wires generated kinds into one id-keyed artifact: group crates, root enum, twins, codecs, edit rules, harness, oracle, taxonomy, fixtures."""
import copy, glob, json, os, re
from common import *
import leaf as L
from diff import GQL, PROTO

MAXLEAVES = 7
FAMILY = {("connection", 1): "fasteners", ("connection-action", 1): "components", ("cold-formed-action", 1): "components", ("member", 1): "web-details", ("member-action", 1): "components", ("section-element", 1): "stiffeners", ("shell-action", 1): "components", ("weld-run", 1): "process"}


class Art:
    def __init__(self, short, emoji, group_emoji, anchor_mod, anchor_group):
        self.short = short
        self.emoji = emoji
        self.name = emoji + short
        self.root = NORM + "/🗿️artifacts/" + self.name
        self.any = self.root + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
        self.mut = self.any + "/🧬️schema/🧬️mutations"
        self.fix = self.any + "/🧪️tests/🧫️fixtures/🧬️mutations"
        self.harness = self.any + "/🧪️tests/%smutate-%s-1" % (emoji, short)
        self.P = short.replace("en", "En", 1)
        self.group_emoji = group_emoji
        self.anchor_group = anchor_group
        self.crate = "semio-s-artifact-norm-%s-mut-" % short
        self.rel = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"


def group_leaves(art, leaves):
    groups, order = {}, []
    for ent in dict.fromkeys(l.ent for l in leaves):
        ls = [l for l in leaves if l.ent is ent and not l.covered]
        for i in range(0, len(ls), MAXLEAVES):
            n = ent.name if i == 0 else "%s-%s" % (ent.name, FAMILY.get((ent.name, i // MAXLEAVES), kebab(ls[i].field)))
            groups[n] = ls[i: i + MAXLEAVES]
            order.append(n)
    return groups, order


def crate_toml(art, name, base_toml):
    src = rd(base_toml)
    deps = re.search(r"\[dependencies\]\n(.*?)(?:\n\[|\Z)", src, re.S).group(1).strip()
    dev = re.search(r"\[dev-dependencies\]\n.*", src, re.S).group(0).strip()
    return re.sub(r'name = "[^"]+"', 'name = "%s"' % name, re.sub(r'description = "[^"]+"', 'description = "%s"' % ("EN %s mutation leaves: %s" % (art.short[2:], name.split("-mut-")[1])), src.split("[dependencies]")[0].rstrip() + "\n")) + "\n[dependencies]\n" + deps + "\n\n" + dev + "\n"


def create_crates(art, F, groups, base_dir):
    preamble = rd(base_dir + "/🦀️.rs")
    pre = preamble[preamble.index("#![allow"): preamble.index("//#region 🪢️Mounts")]
    names = {}
    for g, ls in groups.items():
        ent = ls[0].ent
        cname = art.crate + g
        d = art.mut + "/" + emoji_dir(ent.emoji, g)
        mods = [(l.dirname, l.module) for l in ls]
        root, toml = d + "/🦀️.rs", d + "/📦️packages/🦀️rust/Cargo.toml"
        if F.exists(root):
            F.write(root, mount_into(F.read(root), mods))
        else:
            F.write(root, "//! %s EN %s %s mutation leaves: %s.\n\n" % (ent.emoji, art.short[2:], ent.en.lower(), ", ".join(l.field.replace("_", " ") for l in ls[:3]) + (" and more" if len(ls) > 3 else "")) + pre + "//#region 🪢️Mounts\n" + mount_into("//#endregion 🪢️Mounts\n", mods))
        if not F.exists(toml):
            F.write(toml, crate_toml(art, cname, base_dir + "/📦️packages/🦀️rust/Cargo.toml"))
        names[g] = (cname, d)
    return names


def register(art, F, names):
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
    anchor = '"semio-s-artifact-norm-%s-mut-members",' % art.short
    add = "".join('"%s",' % c for c, _ in names.values() if '"%s"' % c not in s)
    F.write(f, rep(s, anchor, anchor + add))
    s = F.read(OWNERS)
    m = re.search(r'    \{\n      "name": "%s",\n.*?\n    \},\n' % re.escape(art.anchor_crate), s, re.S)
    ent = m.group(0)
    new = ""
    for g, (cname, d) in names.items():
        if '"name": "%s"' % cname in s:
            continue
        root = os.path.relpath(d + "/📦️packages/🦀️rust", SEMIO)
        new += ent.replace(art.anchor_crate, cname).replace(os.path.relpath(art.mut + "/" + art.anchor_dir + "/📦️packages/🦀️rust", SEMIO), root)
    F.write(OWNERS, s.replace(ent, ent + new, 1))
    for pkg in [art.mut + "/📦️packages/🦀️rust/Cargo.toml", art.any + "/🚪️io/📦️packages/🦀️rust/Cargo.toml", art.any + "/🚪️io/📝️text/📦️packages/🦀️rust/Cargo.toml"]:
        s = F.read(pkg)
        a = "%s = { workspace = true }\n" % art.anchor_crate
        add = "".join("%s = { workspace = true }\n" % c for c, _ in names.values() if (c + " =") not in s)
        F.write(pkg, rep(s, a, a + add))


def leaf_files(art, F, leaves, defs, tags):
    for l in leaves:
        d = art.mut + "/" + l.dirname
        F.write(d + "/🦀️.rs", L.leaf_rs(l))
        F.write(d + "/🔺️diff/🦀️.rs", L.diff_rs(l))
        F.write(d + "/↩️inverse/🦀️.rs", L.inverse_rs(l))
        F.write(d + "/🧬️schema/🔣️.json", compact_obj(L.schema_json(l, defs)))
        F.write(d + "/🧬️schema/🟦️.ts", L.schema_ts(l))
        F.write(d + "/🔣️.json", dumps(L.catalog_json(l, tags[l.kind], "✏️s/🔌️plugins/📕️norm/🗿️artifacts/" + art.name + "/" + art.rel)))


def compact_obj(d):
    return dumps(d)


def mutation_twins(art, F, leaves, groups_of):
    P = art.P
    s = F.read(art.mut + "/🦀️.rs")
    uses = "".join("use %s::%s;\n" % (groups_of[l.kind].replace("-", "_"), l.module) for l in leaves)
    m = re.search(r"\nuse [\w:]+;\n\n//#region 🔖️Mutations", s)
    s = s[: m.start() + 1] + "".join(u for u in uses.splitlines(True) if u not in s) + s[m.start() + 1:]
    variants = "".join("    %s(%s::%s),\n" % (l.record, l.module, l.record) for l in leaves)
    s = rep(s, "\n}\n\npub const KINDS", "\n" + variants.rstrip("\n") + "\n}\n\npub const KINDS", 1)
    s = rep(s, '\n];\n//#endregion 🔖️Mutations', "\n" + "".join('    "%s",\n' % l.kind for l in leaves).rstrip("\n") + '\n];\n//#endregion 🔖️Mutations', 1)
    F.write(art.mut + "/🦀️.rs", s)
    p = art.mut + "/🟦️.ts"
    s = F.read(p)
    imports = "".join('import { type %s, parse%s } from "./%s/🧬️schema/🟦️.ts";\n' % (l.record, l.record, l.dirname) for l in leaves)
    i = s.index("\nexport type %sMutation =" % P)
    s = s[:i].rstrip("\n") + "\n" + imports + s[i:]
    j = s.index(";\n", s.index("export type %sMutation =" % P))
    s = s[:j] + "".join("\n  | %s" % l.record for l in leaves) + s[j:]
    k = s.index("\n});", s.index("normWireTagged"))
    s = s[:k] + "".join("\n  %s: parse%s," % (l.record[:1].lower() + l.record[1:], l.record) for l in leaves) + s[k:]
    F.write(p, s)
    p = art.mut + "/🔣️.json"
    j = json.loads(F.read(p))
    for l in leaves:
        j["oneOf"].append({"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/%s/mutation/%s/schema.json" % (art.short, l.kind)})
    F.write(p, "{\n" + ",\n".join('  %s:%s' % (json.dumps(k), json.dumps(v, ensure_ascii=False, separators=(",", ":")) if k != "oneOf" else "[\n" + ",\n".join("    " + json.dumps(x, separators=(",", ":")) for x in v) + "\n  ]") for k, v in j.items()) + "\n}\n")
    p = art.mut + "/🔗️.graphql"
    s = F.read(p)
    u = s.index("\n\n# KINDS:")
    s = s[:u] + "".join("\n  | %s" % l.record for l in leaves) + s[u:]
    s = re.sub(r"(# KINDS: [^\n]*)", lambda m: m.group(1) + "".join(", " + l.kind for l in leaves), s, 1)
    blocks = ""
    for l in leaves:
        flds = "".join("  %s: String! @state(class: ARTIFACT)\n" % camel(x) for x in l.ent.sels) + "  %s: %s! @state(class: ARTIFACT)\n" % (l.newc, L.gql_ty(l))
        blocks += 'type %s {\n  # kind: "%s"\n%s}\n\n' % (l.record, l.kind, flds)
    F.write(p, s.rstrip("\n") + "\n\n" + blocks.rstrip("\n") + "\n")
    p = art.mut + "/🛰️.proto"
    s = F.read(p)
    n = max(int(x) for x in re.findall(r"= (\d+);\n  \}\n\}", s)[-1:] or [0])
    blocks = ""
    oneof = ""
    for l in leaves:
        flds = ""
        for i, x in enumerate(l.ent.sels + [l.new]):
            ty = "string" if x != l.new else ("string" if l.ty.startswith("enum:") else L.proto_ty(l))
            flds += "  // @state artifact\n  %s %s = %d;\n" % (ty, x, i + 1)
        blocks += "// kind: %s\nmessage %s {\n%s}\n\n" % (l.kind, l.record, flds)
        n += 1
        oneof += "    %s %s = %d;\n" % (l.record, l.kind.replace("-", "_"), n)
    i = s.index("message %sMutation {" % P)
    s = s[:i] + blocks + s[i:]
    j = s.index("  }\n}", i)
    s = s[:j] + oneof + s[j:]
    F.write(p, s)


def edit_rules(art, F, leaves, rule_path_fn, rule_removals):
    p = art.mut + "/🧭️edit-rules/🦀️.rs"
    s = F.read(p)
    for kind in rule_removals:
        s = re.sub(r"        SetFieldRule \{ path: \"[^\"]*\", kind: \"%s\"[^\n]*\n" % kind, "", s)
    rules = "".join('        SetFieldRule { path: "%s", kind: "%s", selectors: &[%s], value: "%s" },\n' % (rule_path_fn(l), l.kind, ", ".join('SelectorKey::Id("%s")' % camel(x) for x in l.ent.sels), l.newc) for l in leaves)
    s = rep(s, "    ],\n    insert_item:", rules + "    ],\n    insert_item:", 1)
    F.write(p, s)


def text_codec(art, F, leaves, groups_of):
    p = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    s = F.read(p)
    crates = sorted({groups_of[l.kind] for l in leaves})
    uses = ""
    for c in crates:
        mods = [l.module for l in leaves if groups_of[l.kind] == c]
        uses += "use %s::{%s};\n" % (c.replace("-", "_"), ", ".join(sorted(mods)))
    s = rep(s, "use semio_s_artifact_norm_%s_model::{" % art.short, uses + "use semio_s_artifact_norm_%s_model::{" % art.short, 1)
    enums = sorted({l.rust_ty for l in leaves if l.ty.startswith("enum:")})
    if enums:
        s = re.sub(r"(use semio_s_artifact_norm_%s_model::\{)" % art.short, lambda m: m.group(1) + ", ".join(enums) + ", ", s, 1)
    variants, to_dsl, from_dsl = "", "", ""
    for l in leaves:
        f = [(x, "String") for x in l.ent.sels] + [(l.new, l.rust_ty)]
        variants += "    %s { %s },\n" % (l.record, ", ".join("%s: %s" % x for x in f))
        to_dsl += "        %sMutation::%s(p) => %sMutationDsl::%s { %s },\n" % (art.P, l.record, art.P, l.record, ", ".join("%s: p.%s%s" % (n, n, "" if (t != "String") else ".clone()") for n, t in f))
        from_dsl += "        %sMutationDsl::%s { %s } => %sMutation::%s(%s::%s { %s }),\n" % (art.P, l.record, ", ".join(n for n, _ in f), art.P, l.record, l.module, l.record, ", ".join(n for n, _ in f))
    s = rep(s, "\n}\n\nimpl OpText for %sMutationDsl" % art.P, "\n" + variants.rstrip("\n") + "\n}\n\nimpl OpText for %sMutationDsl" % art.P, 1)
    a = "    }\n}\n\npub fn %s_mutation_from_dsl" % art.short
    s = rep(s, a, to_dsl + a, 1)
    a = "    }\n}\n\nimpl OpText for %sMutation {" % art.P
    s = rep(s, a, from_dsl + a, 1)
    F.write(p, s)


def binary_protocol(art, F, kinds):
    p = art.any + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = F.read(p)
    F.write(p, protocol_merge(s, kinds))


def append_list(text, head, lines, close="\n];"):
    i = text.index(head)
    j = text.index(close, i)
    return text[: j + 1] + lines + text[j + 1:]


def harness(art, F, leaves, vec_row):
    p = art.harness + "/🦀️.rs"
    s = F.read(p)
    s = append_list(s, "const KINDS: &[&str] = &[", "".join('    "%s",\n' % l.kind for l in leaves))
    arms = ""
    for l in leaves:
        arms += '        "%s" => (\n' % l.kind + "".join('            include_str!("../🧫️fixtures/🧬️mutations/%s/✅apply/🔣️.%s.json"),\n' % (l.dirname, x) for x in ["before", "mutation", "after", "outcome"]) + "        ),\n"
    a = '        other => panic!("mutate-%s-1: no committed fixture' % art.short
    s = rep(s, a, arms + a, 1)
    s = re.sub(r"current \d+-kind", "current %d-kind" % (len(re.findall(r'^    "[a-z0-9-]+",$', s.split("//#region 🔖️Kinds")[1].split("];")[0], re.M))), s, 1)
    F.write(p, s)
    p = art.harness + "/🐍️.py"
    s = F.read(p)
    s = append_list(s, "\nKINDS = [", "".join('    "%s",\n' % l.kind for l in leaves), "\n]")
    s = rep(s, '    "change-bolt-count": ("🔩change-bolt-count", "✅apply"),\n', '    "change-bolt-count": ("🔩change-bolt-count", "✅apply"),\n' + "".join('    "%s": ("%s", "✅apply"),\n' % (l.kind, l.dirname) for l in leaves), 1)
    F.write(p, s)
    p = art.harness + "/🥒️.feature"
    s = F.read(p)
    rows = "".join("      | %s | %s | ✅apply  |\n" % (l.kind, l.dirname) for l in leaves)
    s = re.sub(r"(      \| change-bolt-count[^\n]*\n)", lambda m: m.group(1) + rows, s)
    F.write(p, s)


def oracle(art, F, leaves):
    p = art.any + "/🔮️oracles/🔣️.json"
    o = json.loads(F.read(p))
    cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
    vt = [v for v in cat["vectors"] if v["mutationId"] == "change-bolt-count"][0]
    mt = [m for m in man["mutations"] if m["id"] == "change-bolt-count"][0]
    for l in leaves:
        v = copy.deepcopy(vt)
        v["mutationId"] = l.kind
        v["sourceMutationDirectoryName"] = v["mutationDirectoryName"] = l.dirname
        cat["vectors"].append(v)
        cat["kinds"].append(l.kind)
        m = copy.deepcopy(mt)
        m["id"] = l.kind
        m["productionDispatch"]["operation"] = l.kind
        m["productionDispatch"]["variant"] = l.record
        man["mutations"].append(m)
    man["mutations"] = man["mutations"]
    F.write(p, dumps(o))


def taxonomy(art, F, leaves):
    t = json.loads(F.read(TAXONOMY))
    name = art.name
    last = max(i for i, x in enumerate(t["rows"]) if x["artifact"] == name)
    new = [{"aggregateVariant": l.record, "artifact": name, "entity": l.ent.name + "-" + kebab(l.field), "kind": l.kind, "module": l.dirname, "physicalLayout": "direct", "record": "Changed" + l.record[len("Change"):], "source": "%s/%s/%s/🦀️.rs" % (name, art.rel, l.dirname), "standard": "🔖️1", "subset": "✳️any", "type": l.record, "verb": "change"} for l in leaves]
    new.sort(key=lambda r: r["module"])
    have = {r["kind"] for r in t["rows"] if r["artifact"] == name}
    t["rows"][last + 1: last + 1] = [r for r in new if r["kind"] not in have]
    F.write(TAXONOMY, dumps(t))
