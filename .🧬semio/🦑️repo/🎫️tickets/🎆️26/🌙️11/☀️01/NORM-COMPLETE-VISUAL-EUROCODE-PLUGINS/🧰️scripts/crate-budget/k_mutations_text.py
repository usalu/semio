"""🪚 Crate-budget K: the en1998 mutation text codec leaves `mutations` for one `-mutations-text-<family>` crate per mutation leaf crate plus `-mutations-text-kit`; the dispatcher keeps the keyword routing. Usage: k_mutations_text.py (idempotent)."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from d_lib import *

ART = NORM + "/🗿️artifacts/🫨️en1998"
S = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
TEXT_DIR = S + "/🚪️io/📝️text/🧬️mutations"
TEXT = TEXT_DIR + "/🦀️.rs"
PARENT = "semio-s-artifact-norm-en1998-mutations"
PARENT_TOML = S + "/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml"
MP = "semio_s_artifact_norm_en1998_"
EMOJI = {"buildings": "🏢️", "members": "🧱️", "bridges_towers": "🗼️", "silos_tanks": "🏭️", "foundations": "🏗️", "tie_beams": "🔗️", "seismic_slopes": "⛰️", "placements": "📍️"}
KIT = "semio-s-artifact-norm-en1998-mutations-text-kit"
PACK = "semio-framework-pack-json"


def main():
    src = rd(TEXT)
    if "fn print_en1998_mutation" not in src or "text_buildings" in src:
        return print("done")
    ty_fam, ty_use = {}, {}
    for m in re.finditer(r"(?m)^use %smut_(\w+?)::(\w+)::(\w+);$" % MP, src):
        ty_fam[m.group(3)] = m.group(1)
        ty_use[m.group(3)] = m.group(0)
    pa = src.index("fn print_en1998_mutation")
    pb = src.index("fn parse_en1998_mutation")
    pc = src.index("impl protocol::OpText")
    prints = {}
    for m in re.finditer(r'(?m)^        En1998Mutation::(\w+)\(p\) => (format!\("([a-z0-9-]+)[ "].*),$', src[pa:pb]):
        prints[m.group(1)] = (m.group(3), m.group(2))
    parses = {}
    for m in re.finditer(r'(?m)^        "([a-z0-9-]+)" => Ok\(En1998Mutation::(\w+)\((\w+) (\{.*\})\)\),$', src[pb:pc]):
        parses[m.group(2)] = (m.group(1), m.group(3), m.group(4))
    order = re.findall(r"(?m)^        En1998Mutation::(\w+)\(p\)", src[pa:pb])
    assert len(prints) == len(order) == len(parses) == len(ty_fam), (len(prints), len(order), len(parses), len(ty_fam))
    for v in order:
        assert prints[v][0] == parses[v][0] and parses[v][1] == v and v in ty_fam, v
    fams = {}
    for v in order:
        fams.setdefault(ty_fam[v], []).append(v)
    kit_root = TEXT_DIR + "/🧰️kit"
    kit_body = src[src.index("fn enc_str"):src.index("fn print_en1998_mutation")]
    kit_body = re.sub(r"(?m)^fn ", "pub fn ", kit_body)
    kit_body = kit_body.replace("std::collections::BTreeMap<String, String>", "Args")
    kit = '//! 🧰️ EN 1998 mutation text kit: the quoting, JSON-atom and `key=value` argument helpers every family codec shares.\n\n/// 🧰️ The decoded `key=value` arguments of one mutation line.\npub type Args = std::collections::BTreeMap<String, String>;\n\n/// 🔑 One required argument of `keyword`, or the missing-argument refusal.\npub fn required(args: &Args, keyword: &str, key: &str) -> Result<String, String> {\n    args.get(key).cloned().ok_or_else(|| format!("en1998 mutation: missing arg \'{key}\' for \'{keyword}\'"))\n}\n\n' + kit_body.rstrip() + "\n"
    kit = kit.replace("/// 📍 An insert's optional `index=` argument: absent appends.\n", "/// 📍 An insert's optional `index=` argument: absent appends.\n")
    wr(kit_root + "/🦀️.rs", kit)
    kit_pkg = kit_root + "/📦️packages/🦀️rust"
    pack_abs = os.path.normpath(os.path.join(os.path.dirname(PARENT_TOML), re.search(r'%s = \{ path = "([^"]+)" \}' % PACK, rd(PARENT_TOML)).group(1)))
    wr(kit_pkg + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN 1998 mutation text kit"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n%s = { path = "%s" }\nsemio-framework-value = { workspace = true }\n' % (relp(ART, kit_pkg), KIT, PACK, relp(pack_abs, kit_pkg)))
    register_workspace(ART, KIT, kit_pkg)
    script_register(ART, PARENT, KIT)
    lock_register(PARENT, KIT, new_deps=[PACK, "semio-framework-value"])
    alias_uses, print_arms, parse_arms = [], {}, {}
    parent_deps = [KIT]
    for fam, vs in sorted(fams.items()):
        name = "semio-s-artifact-norm-en1998-mutations-text-" + fam.replace("_", "-")
        root = TEXT_DIR + "/" + EMOJI[fam] + fam.replace("_", "-")
        body = "//! %s EN 1998 %s mutation text codec: one print and one parse function per mutation kind, routed by keyword from the `mutations` dispatcher.\n\n#![allow(unused_imports)]\n\nuse %s::{dec_json, enc_json, opt_index, required, Args};\n" % (EMOJI[fam], fam.replace("_", " "), KIT.replace("-", "_"))
        body += "".join(ty_use[v] + "\n" for v in vs)
        for v in vs:
            kw, t, fields = parses[v]
            fn = kw.replace("-", "_")
            fields = fields.replace("(&args)", "(args)")
            body += '\n/// ✍️ Prints `%s` as an op-text line.\npub fn print_%s(p: &%s) -> String {\n    %s\n}\n' % (kw, fn, v, prints[v][1])
            body += '\n/// 📖 Parses the arguments of `%s`.\npub fn parse_%s(args: &Args) -> Result<%s, String> {\n    let arg = |k: &str| required(args, "%s", k);\n    Ok(%s %s)\n}\n' % (kw, fn, v, kw, v, fields)
            print_arms[v] = "        En1998Mutation::%s(p) => text_%s::print_%s(p),\n" % (v, fam, fn)
            parse_arms[v] = '        "%s" => text_%s::parse_%s(&args).map(En1998Mutation::%s),\n' % (kw, fam, fn, v)
        wr(root + "/🦀️.rs", body)
        pkg = root + "/📦️packages/🦀️rust"
        mut = "semio-s-artifact-norm-en1998-mut-" + fam.replace("_", "-")
        wr(pkg + "/Cargo.toml", '[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "EN 1998 %s mutation text codec"\n\n[package.metadata.semio]\nrole = "library"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "../../🦀️.rs"\n\n[dependencies]\n%s = { workspace = true }\n%s = { workspace = true }\n' % (relp(ART, pkg), name, fam.replace("_", " "), KIT, mut))
        register_workspace(ART, name, pkg)
        script_register(ART, PARENT, name)
        lock_register(PARENT, name, new_deps=[KIT, mut])
        parent_deps.append(name)
        alias_uses.append("use %s as text_%s;\n" % (name.replace("-", "_"), fam))
    head = src[:src.index("use semio_s_artifact_norm_en1998_mut_")]
    mid = src[src.index("pub const COMPONENT_GRAMMAR_SEMIO"):src.index("fn enc_str")]
    codec = "use %s::parse_args;\n" % KIT.replace("-", "_")
    pr = "fn print_en1998_mutation(mutation: &En1998Mutation) -> String {\n    match mutation {\n" + "".join(print_arms[v] for v in order) + "    }\n}\n\n"
    pp = 'fn parse_en1998_mutation(line: &str) -> Result<En1998Mutation, String> {\n    let (keyword, rest) = line.split_once(\' \').unwrap_or((line, ""));\n    let args = parse_args(rest)?;\n    match keyword {\n' + "".join(parse_arms[v] for v in order) + '        other => Err(format!("en1998 mutation: unknown keyword {other:?}")),\n    }\n}\n\n'
    tail = src[pc:]
    a = tail.index("#[allow(unused_imports)]\nmod mutations_codec {")
    b = tail.index("/// 📥️ Decodes one committed")
    tail = tail[:a] + "mod mutations_codec {\nuse crate::En1998Mutation;\n\n" + tail[b:]
    wr(TEXT, head + "".join(alias_uses) + codec + "\n" + mid + pr + pp + tail)
    t = rd(PARENT_TOML)
    for d in parent_deps:
        if d + " =" not in t:
            t = t.replace("[dependencies]\n", "[dependencies]\n%s = { workspace = true }\n" % d, 1)
    wr(PARENT_TOML, t)
    print("families", {k: len(v) for k, v in fams.items()})


main()
