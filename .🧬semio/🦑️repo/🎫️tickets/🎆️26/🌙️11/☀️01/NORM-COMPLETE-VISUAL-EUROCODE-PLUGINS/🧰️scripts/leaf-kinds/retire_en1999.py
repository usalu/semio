"""Retires the 8 whole-collection kinds of en1999 through every layer (idempotent)."""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import glob, json, os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import en1999 as E
import wire as W

art = E.art
KINDS = E.WHOLE
MODS = [k.replace("-", "_") for k in KINDS]
RECS = [pascal(k) for k in KINDS]
CAML = [r[:1].lower() + r[1:] for r in RECS]


def sub_lines(t, pred):
    return "".join(l for l in t.splitlines(True) if not pred(l))


def has(l, names):
    return any(re.search(r"(?<![A-Za-z0-9_-])%s(?![A-Za-z0-9_-])" % re.escape(n), l) for n in names)


def rw(path, fn):
    t = rd(path)
    u = fn(t)
    if u != t:
        open(path, "w", encoding="utf-8").write(u)


def main():
    F = lambda p: p
    for d in glob.glob(art.mut + "/*change-*") + glob.glob(art.fix + "/*change-*"):
        if re.sub(r"^[^a-z]+", "", os.path.basename(d)) in KINDS:
            shutil.rmtree(d)
    rw(art.mut + "/🧾️inputs/🦀️.rs", lambda t: re.sub(r'#\[path = "\.\./[^"]*change-(?:%s)/🦀️\.rs"\]\npub mod \w+;\n' % "|".join(k[len("change-"):] for k in KINDS), "", t))
    def root(t):
        t = sub_lines(t, lambda l: l.startswith("use ") and has(l, MODS))
        t = sub_lines(t, lambda l: re.match(r"    (%s)\(" % "|".join(RECS), l))
        return sub_lines(t, lambda l: re.match(r'    "(%s)",$' % "|".join(KINDS), l))
    rw(art.mut + "/🦀️.rs", root)
    rw(art.mut + "/🟦️.ts", lambda t: sub_lines(t, lambda l: has(l, RECS) and (l.startswith("import") or l.lstrip().startswith("|")) or re.match(r"  (%s):" % "|".join(CAML), l)))
    def tsfix(t):
        return re.sub(r"(  \| [A-Za-z]+)\n(;)", r"\1;", t)
    def js(t):
        d = json.loads(t)
        d["oneOf"] = [x for x in d["oneOf"] if not any(x["$ref"].endswith("/%s/schema.json" % k) for k in KINDS)]
        return "{\n" + ",\n".join("  %s:%s" % (json.dumps(k), json.dumps(v, ensure_ascii=False, separators=(",", ":")) if k != "oneOf" else "[\n" + ",\n".join("    " + json.dumps(x, separators=(",", ":")) for x in v) + "\n  ]") for k, v in d.items()) + "\n}\n"
    rw(art.mut + "/🔣️.json", js)
    def gql(t):
        t = sub_lines(t, lambda l: re.match(r"  \| (%s)$" % "|".join(RECS), l))
        t = re.sub(r"(# KINDS: )([^\n]*)", lambda m: m.group(1) + ", ".join(k for k in m.group(2).split(", ") if k not in KINDS), t)
        return re.sub(r"type (?:%s) \{.*?\n\}\n\n" % "|".join(RECS), "", t, flags=re.S)
    rw(art.mut + "/🔗️.graphql", gql)
    def proto(t):
        t = re.sub(r"(?:// kind: [^\n]*\n)?message (?:%s) \{.*?\n\}\n\n" % "|".join(RECS), "", t, flags=re.S)
        t = sub_lines(t, lambda l: re.match(r"    (%s) " % "|".join(RECS), l))
        n = [0]
        def num(m):
            n[0] += 1
            return "%s = %d;" % (m.group(1), n[0])
        head, tail = t.split("oneof payload {", 1)
        body, rest = tail.split("  }\n}", 1)
        return head + "oneof payload {" + re.sub(r"(    \w+ \w+) = \d+;", num, body) + "  }\n}" + rest
    rw(art.mut + "/🛰️.proto", proto)
    tp = art.any + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    def text(t):
        t = re.sub(r"(?m)^    (?:%s) \{[^\n]*\},\n" % "|".join(RECS), "", t)
        t = sub_lines(t, lambda l: re.match(r"        (?:%s)Mutation(?:Dsl)?::(%s)\b" % ("En1999", "|".join(RECS)), l))
        t = sub_lines(t, lambda l: re.match(r"        En1999MutationDsl::(%s)\b" % "|".join(RECS), l))
        t = sub_lines(t, lambda l: re.match(r"        En1999Mutation::(%s)\(" % "|".join(RECS), l))
        for m in MODS:
            t = re.sub(r"\b%s, " % m, "", t)
            t = re.sub(r", %s\b" % m, "", t)
        return sub_lines(t, lambda l: l.startswith("use ") and has(l, MODS))
    rw(tp, text)
    rw(art.any + "/🚪️io/💾️binary/🧬️mutations/🦀️.rs", lambda t: [t := re.sub(r"\b%s, " % m, "", re.sub(r", %s\b" % m, "", t)) for m in MODS][-1])
    h = art.harness
    def hrs(t):
        t = re.sub(r'        "(?:%s)" => \(\n(?:            [^\n]*\n)+        \),\n' % "|".join(KINDS), "", t)
        return sub_lines(t, lambda l: re.match(r'    "(%s)",$' % "|".join(KINDS), l))
    rw(h + "/🦀️.rs", hrs)
    rw(h + "/🐍️.py", lambda t: sub_lines(t, lambda l: re.match(r'    "(%s)"[:,]' % "|".join(KINDS), l)))
    rw(h + "/🥒️.feature", lambda t: sub_lines(t, lambda l: re.match(r"      \| (%s) +\|" % "|".join(KINDS), l)))
    def orc(t):
        o = json.loads(t)
        c, m = o["mutationCatalogs"][0], o["mutationManifests"][0]
        c["vectors"] = [v for v in c["vectors"] if v["mutationId"] not in KINDS]
        c["kinds"] = [k for k in c["kinds"] if k not in KINDS]
        m["mutations"] = [x for x in m["mutations"] if x["id"] not in KINDS]
        return dumps(o)
    rw(art.any + "/🔮️oracles/🔣️.json", orc)
    def tax(t):
        d = json.loads(t)
        d["rows"] = [r for r in d["rows"] if not (r["artifact"] == art.name and r["kind"] in KINDS)]
        return dumps(d)
    rw(TAXONOMY, tax)
    def difrs(t):
        return re.sub(r"\nimpl En1999\w+Rows \{\n\s+/// 🧮️ The delta of a whole-list setter.*?\n    \}\n", "\n", t, flags=re.S)
    rw(art.any + "/🧬️schema/🔺️diff/🦀️.rs", difrs)
    def dtest(t):
        t = t.replace("use semio_s_artifact_norm_en1999_mut_inputs::change_members::ChangeMembers;\n", "use semio_s_artifact_norm_en1999_mut_member::change_member_length::ChangeMemberLength;\n")
        i = t.index("async fn a_list_setter_names_every_row_and_inverts")
        j = t.index("\n}\n", i) + 3
        new = '''async fn a_typed_leaf_names_one_row_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = En1999Mutation::ChangeMemberLength(ChangeMemberLength { member_id: base.members[0].id.clone(), new_length: base.members[0].length + 1.0 });
    let forward = diff_of(&mutation, &base);
    let after = protocol::apply_diff_cold(&forward, &base).expect("apply");
    assert_eq!(after.members[0].length, base.members[0].length + 1.0);
    assert_eq!(protocol::apply_diff_cold(&forward.inverse(&base), &after).expect("inverse"), base);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}
'''
        return t[:i] + new + t[j:]
    rw(art.any + "/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs", dtest)
    dev = art.any + "/🧬️schema/🔺️diff/📦️packages/🦀️rust/Cargo.toml"
    rw(dev, lambda t: t.replace("[dev-dependencies]\n", "[dev-dependencies]\nsemio-s-artifact-norm-en1999-mut-member = { workspace = true }\n", 1) if "mut-member =" not in t else t)
    kinds = re.findall(r'^    "([a-z0-9-]+)",$', rd(art.mut + "/🦀️.rs"), re.M)
    p = art.any + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio"
    t = rd(p)
    open(p, "w", encoding="utf-8").write(t[: t.index("record ")] + "".join("record %s tag=%d\nfield payload bytes\n" % (k, i) for i, k in enumerate(kinds)))
    print("kinds left", len(kinds))


main()
