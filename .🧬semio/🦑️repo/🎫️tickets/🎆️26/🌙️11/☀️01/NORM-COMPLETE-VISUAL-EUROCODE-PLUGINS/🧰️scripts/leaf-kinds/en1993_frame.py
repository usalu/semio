"""EN 1993 additive kinds for entities added after the first leaf pass (FrameNode, FrameBar, Placement, materials[].rambergOsgoodN): typed `change-` leaf kinds plus `insert-`/`remove-` row kinds. Only kinds missing from the root enum are generated; rerun is idempotent. python -I en1993_frame.py [--apply]"""
import os, re, shutil, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *
import common, leaf as L, en1993 as X, en1993_wire as Wr, en1993_arrays as Ar

art = X.art


def append_graphql(F, c):
    p = art.mut + "/🔗️.graphql"
    t = F.read(p).rstrip("\n") + "\n"
    t += "".join("type %s { index: Int @state(class: ARTIFACT) }\n" % pascal(k) for k in c.row_kinds if k.startswith("insert"))
    t += "".join("type %s { index: Int! @state(class: ARTIFACT) }\n" % pascal(k) for k in c.row_kinds if k.startswith("remove"))
    F.write(p, t + Wr.gql_blocks(c))


def harness(F, c):
    new = c.row_kinds + [l.kind for l in c.new_leaves]
    h = art.harness
    p = h + "/🦀️.rs"
    s = F.read(p)
    i = s.index("const KINDS: &[&str] = &[")
    j = s.index("\n];", i)
    s = s[:j] + "\n" + "".join('    "%s",\n' % k for k in new).rstrip("\n") + s[j:]
    arms = "".join('    ("%s", fixture!("%s")),\n' % (k, Wr.dir_of(c, k)) for k in new)
    s = Wr.rep(s, "];\n\n/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector", arms + "];\n\n/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector")
    total = len(re.findall(r'^    "[a-z0-9-]+",$', s[i:s.index("\n];", i)], re.M))
    s = re.sub(r"current \d+-kind", "current %d-kind" % total, s, count=1)
    F.write(p, s)
    p = h + "/🐍️.py"
    s = F.read(p)
    s = Wr.rep(s, "\n]\n\n#: 🧫️ The committed specification vectors", "\n" + "".join('    "%s",\n' % k for k in new).rstrip("\n") + "\n]\n\n#: 🧫️ The committed specification vectors")
    s = Wr.rep(s, '    "change-annex": ("🌍️change-annex", "✅apply"),\n', '    "change-annex": ("🌍️change-annex", "✅apply"),\n' + "".join('    "%s": ("%s", "✅apply"),\n' % (k, Wr.dir_of(c, k)) for k in new))
    F.write(p, s)
    p = h + "/🥒️.feature"
    s = F.read(p)
    rows = "".join("      | %s | %s | ✅apply  |\n" % (k, Wr.dir_of(c, k)) for k in new)
    s = re.sub(r"(      \| remove-crane-runway +\|[^\n]*\n)", lambda m: m.group(1) + rows, s)
    s = re.sub(r"The \d+ `✅apply` vectors", "The %d `✅apply` vectors" % total, s, count=1)
    F.write(p, s)


def run():
    c = X.main()
    F = c.F
    common.STAGE = X.STAGE93
    c.leaf_by_kind = {l.kind: l for l in c.new_leaves}
    c.row_mods = {}
    shutil.rmtree(X.STAGE93, ignore_errors=True)
    names = Wr.create_crates(F, c)
    c.crate_of = {l.kind: names[g][0] for g, ls in c.groups.items() for l in ls}
    Wr.register(F, c, names)
    Wr.leaf_files(F, c)
    Wr.root_rs(F, c)
    Wr.root_ts(F, c)
    Wr.root_json(F, c)
    append_graphql(F, c)
    Wr.root_proto(F, c)
    Wr.edit_rules(F, c)
    Wr.text_codec(F, c)
    Wr.binary_codec(F, c)
    harness(F, c)
    Wr.oracle(F, c)
    Wr.taxonomy(F, c)
    Wr.fixtures(F, c)
    Wr.schema_index(F, c)
    print(c.stats, "files", len(F.written))
    if "--apply" in sys.argv:
        print("applied", F.apply())


if __name__ == "__main__":
    run()
