"""Wires `index_selector` kinds into one keyed-diff artifact: group crates, registration (Cargo.toml, Cargo.lock, script.ts, build-owners), root enum/KINDS/twins, per-kind twins, edit rules, text and binary codecs, harness, oracle, taxonomy, fixtures. Flavours (`Art.flavour`) cover the two layouts met so far: en1992 (external-tagged payloads, `print_op/parse_op` text codec, rs/py/feature harness) and en1990 (`mutation`-tagged payloads, `OpText` codec, oracle-only harness)."""
import copy, glob, json, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import index_selector as I

MAXKINDS = 8


class Art:
    def __init__(self, short, emoji, style, flavour, anchor_dir, anchor_crate, template_kind, text_ctx):
        self.short, self.emoji, self.style, self.flavour = short, emoji, style, flavour
        self.name = emoji + short
        self.root = NORM + "/🗿️artifacts/" + self.name
        self.any = self.root + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
        self.mut = self.any + "/🧬️schema/🧬️mutations"
        self.fix = self.any + "/🧪️tests/🧫️fixtures/🧬️mutations"
        self.P = short.replace("en", "En", 1)
        self.crate = "semio-s-artifact-norm-%s-mut-" % short
        self.anchor_dir, self.anchor_crate = anchor_dir, anchor_crate
        self.template_kind = template_kind
        self.text_ctx = text_ctx
        self.rel = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
        self.harness = glob.glob(self.any + "/🧪️tests/*mutate-%s-1" % short)[0] if glob.glob(self.any + "/🧪️tests/*mutate-%s-1" % short) else None

    @property
    def external(self):
        return not self.style.tagged


def slug_crate(art, key, taken):
    name = "-".join(x for x in key if x)
    return name


def group_kinds(art, kinds, existing_crates):
    """Chunks the kinds into crates of at most MAXKINDS: `<entity group>[-<family>]`, further chunks of the same key named by the chunk's first leaf (no numeric suffixes)."""
    by, order = {}, []
    for k in kinds:
        key = (k.group, getattr(k, "family", ""))
        if key not in by:
            by[key] = []
            order.append(key)
        by[key].append(k)
    groups = {}
    for key in order:
        ks = by[key]
        name = "-".join(x for x in key if x)
        assert len(ks) <= MAXKINDS, "crate %s would hold %d kinds (max %d): split its family" % (name, len(ks), MAXKINDS)
        assert name not in groups, "duplicate group: " + name
        groups[name] = ks
    return groups


def crate_toml(art, name, base_toml):
    src = rd(base_toml)
    head = src.split("[dependencies]")[0].rstrip()
    deps = re.search(r"\[dependencies\]\n(.*?)(?:\n\[|\Z)", src, re.S).group(1).strip()
    dev = re.search(r"\[dev-dependencies\]\n.*", src, re.S).group(0).strip()
    head = re.sub(r'name = "[^"]+"', 'name = "%s"' % name, head, 1)
    head = re.sub(r'description = "[^"]+"', 'description = "EN %s mutation leaves: %s"' % (art.short[2:], name.split("-mut-")[1].replace("-", " ")), head, 1)
    return head + "\n\n[dependencies]\n" + deps + "\n\n" + dev + "\n"


MOUNT = re.compile(r'#\[path = "\.\./([^/"]+)/🦀️\.rs"\]\s*\n(?:pub )?mod (\w+);')


def mounted_dirs(art, F):
    """Leaf directories mounted by any crate root of the mutation tree."""
    return {d for p in glob.glob(art.mut + "/*/🦀️.rs") for d, _ in MOUNT.findall(F.read(p))}


def derived_mounts(art, F, root, ks):
    """Mounts a group root must carry, derived from the leaf directories on disk: every mount the root already has plus every unmounted leaf directory of the group's verb-entity prefixes, so a regeneration never drops a mount."""
    out = MOUNT.findall(F.read(root)) if F.exists(root) else []
    mounted = mounted_dirs(art, F)
    prefixes = {"-".join(k.kind.split("-")[:2]) for k in ks}
    for p in sorted(glob.glob(art.mut + "/*/🦀️.rs")):
        dn = os.path.basename(os.path.dirname(p))
        bare = re.sub(r"^[^a-z]+", "", dn)
        if dn in mounted or not os.path.isdir(os.path.dirname(p) + "/↩️inverse") or not any(bare == q or bare.startswith(q + "-") for q in prefixes):
            continue
        out.append((dn, bare.replace("-", "_")))
    return out


def create_crates(art, F, groups):
    base_dir = art.mut + "/" + art.anchor_dir
    preamble = rd(base_dir + "/🦀️.rs")
    pre = preamble[preamble.index("#![allow"): preamble.index("//#region 🪢️Mounts")]
    names = {}
    for g, ks in groups.items():
        cname = art.crate + g
        d = art.mut + "/" + emoji_dir(ks[0].ent.emoji, g)
        words = ", ".join(sorted({k.kind.split("-", 1)[1].replace("-", " ") for k in ks})[:3]) + (" and more" if len(ks) > 3 else "")
        root, toml = d + "/🦀️.rs", d + "/📦️packages/🦀️rust/Cargo.toml"
        mods = list(dict.fromkeys([(k.dirname, k.module) for k in ks] + derived_mounts(art, F, root, ks)))
        if F.exists(root):
            F.write(root, mount_into(F.read(root), mods))
        else:
            F.write(root, "//! %s EN %s %s mutation leaves: %s.\n\n" % (ks[0].ent.emoji, art.short[2:], g.replace("-", " "), words) + pre + "//#region 🪢️Mounts\n" + mount_into("//#endregion 🪢️Mounts\n", mods))
        if not F.exists(toml):
            F.write(toml, crate_toml(art, cname, base_dir + "/📦️packages/🦀️rust/Cargo.toml"))
        names[g] = (cname, d)
    return names


def lock_register(F, art, names):
    p = art.root + "/Cargo.lock"
    s = F.read(p)
    parts = re.split(r"(?m)^(?=\[\[package\]\]$)", s)
    head, blocks = parts[0], [x.rstrip("\n") for x in parts[1:]]
    name_of = lambda b: re.search(r'name = "([^"]+)"', b).group(1)
    tmpl = next(b for b in blocks if name_of(b) == art.anchor_crate)
    for g, (cname, d) in names.items():
        if any(name_of(b) == cname for b in blocks):
            continue
        new = tmpl.replace(art.anchor_crate, cname)
        at = next((i for i, b in enumerate(blocks) if name_of(b) > cname), len(blocks))
        blocks.insert(at, new)
    out = []
    for b in blocks:
        m = re.search(r"dependencies = \[\n(.*?)\n\]", b, re.S)
        if m and ' "%s",' % art.anchor_crate in m.group(1).split("\n") and name_of(b) != art.anchor_crate and not name_of(b).startswith(art.crate):
            deps = m.group(1).split("\n")
            deps += [' "%s",' % c for c, _ in names.values() if ' "%s",' % c not in deps]
            deps.sort(key=lambda x: x.strip().strip('",'))
            b = b.replace(m.group(1), "\n".join(deps), 1)
        out.append(b)
    F.write(p, head + "\n\n".join(out) + "\n")


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
    anchor = '"%s",' % art.anchor_crate
    F.write(f, rep(s, anchor, anchor + "".join('"%s",' % c for c, _ in names.values() if '"%s"' % c not in s)))
    s = F.read(OWNERS)
    m = re.search(r'    \{\n      "name": "%s",\n.*?\n    \},\n' % re.escape(art.anchor_crate), s, re.S)
    ent, new = m.group(0), ""
    for g, (cname, d) in names.items():
        if '"name": "%s"' % cname in s:
            continue
        new += ent.replace(art.anchor_crate, cname).replace(os.path.relpath(art.mut + "/" + art.anchor_dir + "/📦️packages/🦀️rust", SEMIO), os.path.relpath(d + "/📦️packages/🦀️rust", SEMIO))
    F.write(OWNERS, s.replace(ent, ent + new, 1))
    consumers = glob.glob(art.any + "/**/Cargo.toml", recursive=True) + [art.mut + "/📦️packages/🦀️rust/Cargo.toml"]
    for pkg in dict.fromkeys(consumers):
        s = F.read(pkg)
        a = "%s = { workspace = true }\n" % art.anchor_crate
        if a in s:
            F.write(pkg, s.replace(a, a + "".join("%s = { workspace = true }\n" % c for c, _ in names.values() if (c + " =") not in s), 1))
    lock_register(F, art, names)


def gql_ty(t):
    return {"f64": "Float!", "u32": "Int!", "u8": "Int!", "bool": "Boolean!", "String": "String!", "index": "Int!", "optindex": "Int", "ref": "String!"}.get(t, "String!")


def proto_ty(t):
    return {"f64": "double", "u32": "uint32", "u8": "uint32", "bool": "bool", "String": "string", "index": "uint32", "optindex": "uint32", "ref": "string"}.get(t, "string")


def leaf_files(art, F, kinds, defs, tags):
    st = art.style
    for k in kinds:
        d = art.mut + "/" + k.dirname
        F.write(d + "/🦀️.rs", k.leaf_rs(st))
        F.write(d + "/🔺️diff/🦀️.rs", k.diff_rs(st))
        F.write(d + "/↩️inverse/🦀️.rs", k.inverse_rs(st))
        F.write(d + "/🧬️schema/🔣️.json", dumps(k.schema_json(st, defs, art.short)))
        F.write(d + "/🧬️schema/🟦️.ts", k.schema_ts(st))
        if art.flavour == "en1992":
            fl = k.wire_fields
            F.write(d + "/🧬️schema/🔗️.graphql", "type %s {\n%s}\n" % (k.record, "".join("  %s: %s\n" % (camel(n), gql_ty(t)) for n, t, c in fl)))
            F.write(d + "/🧬️schema/🛰️.proto", 'syntax = "proto3";\nmessage %s {\n%s}\n' % (k.record, "".join("  %s %s = %d;\n" % (proto_ty(t), camel(n), i + 1) for i, (n, t, c) in enumerate(fl))))
        F.write(d + "/🔣️.json", dumps(catalog_json(art, k, tags[k.kind])))


def catalog_json(art, k, tag):
    return {"schemaVersion": 1, "owner": "✏️s/🔌️plugins/📕️norm/🗿️artifacts/" + art.name + "/" + art.rel + "/" + k.dirname, "semanticKind": k.kind, "displayName": " ".join(w.capitalize() for w in k.kind.split("-")), "emoji": k.ent.emoji, "aggregateVariant": k.record, "payloadSchema": "🧬️schema/🔣️.json", "textOpcode": None, "binaryTag": tag, "invertibility": "explicit-mutation", "diffParticipation": "detect", "outcomeClasses": ["applied", "no-op"], "composition": "atomic", "requiredLanguageSurfaces": ["rust", "json-schema", "text", "binary"]}


def kinds_of(art, F):
    s = F.read(art.mut + "/🦀️.rs")
    body = s[s.index("pub const KINDS"):]
    return re.findall(r'^    "([a-z0-9-]+)",$', body[: body.index("\n];")], re.M)


def root_wire(art, F, kinds, groups_of):
    P = art.P
    p = art.mut + "/🦀️.rs"
    s = F.read(p)
    d = s.index("#[derive(Clone, Debug, PartialEq, dsl::Mutations")
    uses = list(re.finditer(r"^use [\w:]+;\n", s[:d], re.M))
    at = uses[-1].end()
    new = "".join("use %s::%s;\n" % (groups_of[k.kind].replace("-", "_"), k.module) for k in kinds)
    s = s[:at] + "".join(u for u in new.splitlines(True) if u not in s) + s[at:]
    s = rep(s, "\n}\n\npub const KINDS", "\n" + "".join("    %s(%s::%s),\n" % (k.record, k.module, k.record) for k in kinds).rstrip("\n") + "\n}\n\npub const KINDS", 1)
    i = s.index("pub const KINDS")
    j = s.index("\n];", i)
    s = s[:j] + "\n" + "".join('    "%s",\n' % k.kind for k in kinds).rstrip("\n") + s[j:]
    F.write(p, s)
    p = art.mut + "/🟦️.ts"
    s = F.read(p)
    imports = "".join('import { type %s, parse%s } from "./%s/🧬️schema/🟦️.ts";\n' % (k.record, k.record, k.dirname) for k in kinds)
    i = s.index("\nexport type %sMutation =" % P)
    s = s[:i].rstrip("\n") + "\n" + imports + s[i:]
    j = s.index(";\n", s.index("export type %sMutation =" % P))
    s = s[:j] + "".join("\n  | %s" % ("{ %s: %s }" % (k.record, k.record) if art.external else k.record) for k in kinds) + s[j:]
    kk = s.rindex("\n});")
    s = s[:kk] + "".join("\n  %s: parse%s," % (k.record if art.external else k.record[:1].lower() + k.record[1:], k.record) for k in kinds) + s[kk:]
    F.write(p, s)
    p = art.mut + "/🔣️.json"
    j = json.loads(F.read(p))
    for k in kinds:
        ref = {"$ref": "https://json.schemas.assets.semio-tech.com/s/norm/%s/mutation/%s/schema.json" % (art.short, k.kind)}
        j["oneOf"].append({"type": "object", "additionalProperties": False, "required": [k.record], "properties": {k.record: ref}} if art.external else ref)
    F.write(p, dumps(j))
    if art.flavour == "en1990":
        p = art.mut + "/🔗️.graphql"
        s = F.read(p)
        e = s.index("\n}\n", s.index("enum %sMutationKind" % P))
        F.write(p, s[:e] + "".join("\n  %s" % k.kind.upper().replace("-", "_") for k in kinds) + s[e:])


def edit_rules(art, F, kinds):
    p = art.mut + "/🧭️edit-rules/🦀️.rs"
    s = F.read(p)
    sets, ins, rem = "", "", ""
    key = lambda l: 'SelectorKey::%s("%s")' % ("Index" if I.is_index(l.sel) else "Id", camel(l.sel))
    for k in kinds:
        links = k.ent.links
        base = ".".join(camel(l.coll) + "[]" for l in links[:-1]) + ("." if links[:-1] else "")
        if isinstance(k, I.TopKind):
            sets += '        SetFieldRule { path: "%s", kind: "%s", selectors: &[], value: "%s" },\n' % (camel(k.leaf.field), k.kind, k.leaf.newc)
        elif isinstance(k, I.SetKind):
            l = k.leaf
            sets += '        SetFieldRule { path: "%s", kind: "%s", selectors: &[%s], value: "%s" },\n' % (base + camel(links[-1].coll) + "[]." + ".".join(camel(x) for x in l.path), k.kind, ", ".join(key(x) for x in links), l.newc)
        elif isinstance(k, I.InsertKind):
            ins += '        InsertItemRule { path: "%s", kind: "%s", selectors: &[%s], index: "index", item: "%s" },\n' % (base + camel(links[-1].coll), k.kind, ", ".join(key(x) for x in links[:-1]), k.item_c)
        else:
            rem += '        RemoveItemRule { path: "%s", kind: "%s", selectors: &[%s], index: "index" },\n' % (base + camel(links[-1].coll), k.kind, ", ".join(key(x) for x in links[:-1]))
    s = rep(s, "    ],\n    insert_item:", sets + "    ],\n    insert_item:", 1)
    s = rep(s, "    ],\n    remove_item:", ins + "    ],\n    remove_item:", 1)
    i = s.index("    remove_item: &[")
    j = s.index("    ],\n};", i)
    s = s[:j] + rem + s[j:]
    s = re.sub(r"use semio_s_plugin_norm_app_surface::\{([^}]*)\};", lambda m: "use semio_s_plugin_norm_app_surface::{%s};" % ", ".join(sorted(set(x.strip() for x in m.group(1).split(",")) | ({"RemoveItemRule"} if rem else set()) | ({"SelectorKey"} if "SelectorKey::" in s else set()))), s, 1)
    F.write(p, s)


def enc(art, t, expr):
    if art.flavour == "en1990" or t in ("String", "ref", "optindex") or t.startswith(("enum:", "row:")):
        return "enc_json(&%s)" % expr
    return expr


def dec(art, name, t, arg):
    if art.flavour == "en1990":
        return "dec_json(arg(\"%s\")?)?" % name if t != "optindex" else "opt_index(&args)?"
    if t in ("String", "ref") or t.startswith(("enum:", "row:")):
        return "dec_json(&get(\"%s\")?)?" % name
    err = {"f64": "ParseFloatError", "u32": "ParseIntError", "u8": "ParseIntError", "index": "ParseIntError", "bool": "ParseBoolError"}[t]
    return 'get("%s")?.parse().map_err(|e: std::num::%s| e.to_string())?' % (name, err)


def text_codec(art, F, kinds, groups_of):
    p = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    s = F.read(p)
    P = art.P
    anchor = re.search(r"^use (?:semio_s_artifact_norm_\w+_mutations|crate)::%sMutation;\n" % P, s, re.M)
    uses = "".join("use %s::%s;\n" % (groups_of[k.kind].replace("-", "_"), k.module) for k in kinds)
    s = s[: anchor.end()] + "".join(u for u in uses.splitlines(True) if u not in s) + s[anchor.end():]
    prints, parses = "", ""
    for k in kinds:
        flds = k.wire_fields
        fmt = " ".join("%s={}" % kebab(n) for n, t, c in flds)
        args = ", ".join(enc(art, t, "p.%s" % n) for n, t, c in flds)
        prints += '        %sMutation::%s(p) => format!("%s %s", %s),\n' % (P, k.record, k.kind, fmt, args)
        parses += '        "%s" => Ok(%sMutation::%s(%s::%s { %s })),\n' % (k.kind, P, k.record, k.module, k.record, ", ".join("%s: %s" % (n, dec(art, kebab(n), t, n)) for n, t, c in flds))
    pa = "    }\n}\n\n/// 📖️ Parse a single op line" if art.flavour == "en1992" else "    }\n}\n\nfn parse_%s_mutation" % art.short
    s = rep(s, pa, prints + pa, 1)
    pb = '        other => Err('
    s = rep(s, pb, parses + pb, 1)
    F.write(p, s)


def binary_protocol(art, F, kinds_all):
    p = art.any + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = F.read(p)
    assert len(kinds_all) <= 256, "u8 tag overflow: %d kinds" % len(kinds_all)
    F.write(p, protocol_merge(s, kinds_all))


def harness(art, F, kinds):
    if art.flavour == "en1992":
        harness_rs_py(art, F, kinds)
    p = art.harness + "/🥒️.feature"
    s = F.read(p)
    rows = "".join("      | %s | %s | ✅apply  |\n" % (k.kind, k.dirname) for k in kinds)
    s = re.sub(r"(      \| %s [^\n]*\n)" % re.escape(art.template_kind), lambda m: m.group(1) + rows, s)
    F.write(p, s)


def harness_rs_py(art, F, kinds):
    p = art.harness + "/🦀️.rs"
    s = F.read(p)
    i = s.index("const KINDS")
    j = s.index("\n];", i)
    s = s[:j] + "\n" + "".join('    "%s",\n' % k.kind for k in kinds).rstrip("\n") + s[j:]
    arms = ""
    for k in kinds:
        arms += '        "%s" => (\n' % k.kind + "".join('            include_str!("../🧫️fixtures/🧬️mutations/%s/✅apply/🔣️.%s.json"),\n' % (k.dirname, x) for x in ["before", "mutation", "after", "outcome"]) + "        ),\n"
    s = rep(s, '        other => panic!("mutate-%s-1: no committed fixture' % art.short, arms + '        other => panic!("mutate-%s-1: no committed fixture' % art.short, 1)
    s = re.sub(r"current \d+-kind", "current %d-kind" % len(re.findall(r'^    "[a-z0-9-]+",$', s[s.index("const KINDS"): s.index("];", s.index("const KINDS"))], re.M)), s, 1)
    F.write(p, s)
    p = art.harness + "/🐍️.py"
    s = F.read(p)
    i = s.index("KINDS = [")
    j = s.index("\n]", i)
    s = s[:j] + "\n" + "".join('    "%s",\n' % k.kind for k in kinds).rstrip("\n") + s[j:]
    i = s.index("VECTORS = {")
    j = s.index("\n}", i)
    last_apply = max(m.start() for m in re.finditer(r'^    "[a-z0-9-]+": \("[^"]+", "✅apply"\),$', s[:j], re.M))
    k2 = s.index("\n", last_apply) + 1
    s = s[:k2] + "".join('    "%s": ("%s", "✅apply"),\n' % (k.kind, k.dirname) for k in kinds) + s[k2:]
    F.write(p, s)


def oracle(art, F, kinds):
    p = art.any + "/🔮️oracles/🔣️.json"
    o = json.loads(F.read(p))
    cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
    vt = [v for v in cat["vectors"] if v["mutationId"] == art.template_kind][0]
    mt = [m for m in man["mutations"] if m["id"] == art.template_kind][0]
    have = {v["mutationId"] for v in cat["vectors"]}
    for k in kinds:
        if k.kind in have:
            continue
        v = copy.deepcopy(vt)
        v["mutationId"] = k.kind
        v["sourceMutationDirectoryName"] = v["mutationDirectoryName"] = k.dirname
        cat["vectors"].append(v)
        cat["kinds"].append(k.kind)
        m = copy.deepcopy(mt)
        m["id"] = k.kind
        m["productionDispatch"]["operation"] = k.kind
        m["productionDispatch"]["variant"] = k.record
        man["mutations"].append(m)
    F.write(p, json.dumps(o, indent=2, ensure_ascii=False) + "\n")


def taxonomy(art, F, kinds):
    t = json.loads(F.read(TAXONOMY))
    name = art.name
    last = max(i for i, x in enumerate(t["rows"]) if x["artifact"] == name)
    new = [{"aggregateVariant": k.record, "artifact": name, "entity": k.kind.split("-", 1)[1], "kind": k.kind, "module": k.dirname, "physicalLayout": "direct", "record": {"change": "Changed", "insert": "Inserted", "remove": "Removed"}[k.kind.split("-")[0]] + k.record[len(k.kind.split("-")[0]):], "source": "%s/%s/%s/🦀️.rs" % (name, art.rel, k.dirname), "standard": "🔖️1", "subset": "✳️any", "type": k.record, "verb": k.kind.split("-")[0]} for k in kinds]
    new.sort(key=lambda r: r["module"])
    have = {r["kind"] for r in t["rows"] if r["artifact"] == name}
    t["rows"][last + 1: last + 1] = [r for r in new if r["kind"] not in have]
    F.write(TAXONOMY, dumps(t))


class KFiles(Files):
    """Staged writer that also removes directories when applied."""

    def __init__(self, stage):
        super().__init__(stage)
        self.removed = []

    def remove_dir(self, path):
        self.removed.append(path)

    def apply(self):
        n = super().apply()
        if self.removed:
            print("kept %d retired kind directories: generators never delete (retire by hand)" % len(self.removed))
        return n


def binary_protocol_keep(art, F, new_kinds, retired):
    """Keeps the wire tag of every surviving kind, drops retired records, appends new kinds at the next free tags; returns {kind: tag}."""
    p = art.any + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    s = F.read(p)
    head = s[: re.search(r"^record ", s, re.M).start()]
    recs = [(m.group(1), int(m.group(2))) for m in re.finditer(r"^record (\S+) tag=(\d+)$", s, re.M)]
    recs = [r for r in recs if r[0] not in retired]
    used = {t for _, t in recs}
    nxt = max(used) + 1
    for k in new_kinds:
        if k.kind not in {n for n, _ in recs}:
            recs.append((k.kind, nxt))
            nxt += 1
    assert nxt <= 256 or "field tag varint" in head, "u8 tag overflow"
    F.write(p, head + "".join("record %s tag=%d\nfield payload bytes\n" % r for r in recs))
    return dict(recs)


def drop_lines(text, pattern):
    return re.sub(pattern, "", text, flags=re.M)


def retire(art, F, kinds, dirs, fixture_dirs):
    """(Staged deletions are dropped by the additive merge and directories are kept: retirement is a manual step.) Retires whole kinds from every layer: mounts, root enum/KINDS/uses/ts/json/graphql, text codec, protocol, oracle, taxonomy, feature, unit tests, kind and fixture directories."""
    P = art.P
    mods = {k: k.replace("-", "_") for k in kinds}
    recs = {k: pascal(k) for k in kinds}
    for p in glob.glob(art.mut + "/*/🦀️.rs"):
        s = rd(p) if p not in F.written else F.read(p)
        t = s
        for k in kinds:
            t = re.sub(r'#\[path = "\.\./[^"]+/🦀️\.rs"\]\npub mod %s;\n' % mods[k], "", t)
        if t != s:
            F.write(p, t)
    p = art.mut + "/🦀️.rs"
    s = F.read(p)
    for k in kinds:
        s = drop_lines(s, r"^use [\w:]+::%s;\n" % mods[k])
        s = drop_lines(s, r"^    %s\(%s::%s\),\n" % (recs[k], mods[k], recs[k]))
        s = drop_lines(s, r'^    "%s",\n' % k)
    F.write(p, s)
    p = art.mut + "/🟦️.ts"
    s = F.read(p)
    for k in kinds:
        s = drop_lines(s, r"^import \{[^\n]*%s[^\n]*\} from \"\./[^\n]*\n" % recs[k])
        s = drop_lines(s, r"^  \| (\{ %s: %s \}|%s)\n" % (recs[k], recs[k], recs[k]))
        s = drop_lines(s, r"^  (%s|%s): parse%s,\n" % (recs[k], recs[k][:1].lower() + recs[k][1:], recs[k]))
    F.write(p, s)
    p = art.mut + "/🔣️.json"
    j = json.loads(F.read(p))
    j["oneOf"] = [x for x in j["oneOf"] if not any(("/mutation/%s/" % k) in json.dumps(x) for k in kinds)]
    F.write(p, dumps(j))
    p = art.mut + "/🔗️.graphql"
    s = F.read(p)
    for k in kinds:
        s = drop_lines(s, r"^  %s\n" % k.upper().replace("-", "_"))
    F.write(p, s)
    p = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    s = F.read(p)
    for k in kinds:
        s = drop_lines(s, r"^use [\w:]+::%s;\n" % mods[k])
        s = drop_lines(s, r"^        %sMutation::%s\(p\) => format!\(\"%s [^\n]*\n" % (P, recs[k], k))
        s = drop_lines(s, r'^        "%s" => Ok\([^\n]*\n' % k)
        s = drop_lines(s, r"^        %sMutation::%s\(%s::[^\n]*\n" % (P, recs[k], mods[k]))
        s = re.sub(r"(use [\w:]+::\{)([^}]*)\};", lambda m: m.group(1) + ", ".join(x for x in (y.strip() for y in m.group(2).split(",")) if x != mods[k]) + "};", s)
    s = drop_lines(s, r"^use [\w:]+::\{\};\n")
    F.write(p, s)
    p = art.any + "/🔮️oracles/🔣️.json"
    o = json.loads(F.read(p))
    cat, man = o["mutationCatalogs"][0], o["mutationManifests"][0]
    cat["vectors"] = [v for v in cat["vectors"] if v["mutationId"] not in kinds]
    cat["kinds"] = [x for x in cat["kinds"] if x not in kinds]
    man["mutations"] = [m for m in man["mutations"] if m["id"] not in kinds]
    F.write(p, json.dumps(o, indent=2, ensure_ascii=False) + "\n")
    t = json.loads(F.read(TAXONOMY))
    t["rows"] = [r for r in t["rows"] if not (r["artifact"] == art.name and r["kind"] in kinds)]
    F.write(TAXONOMY, dumps(t))
    if art.harness:
        p = art.harness + "/🥒️.feature"
        s = F.read(p)
        for k in kinds:
            s = drop_lines(s, r"^      \| %s +\|[^\n]*\n" % re.escape(k))
        F.write(p, s)
    for d in dirs:
        F.remove_dir(art.mut + "/" + d)
    for d in fixture_dirs:
        F.remove_dir(art.fix + "/" + d)
