"""▶️ Regenerates the whole stage tree from the pristine base."""
import os, re, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import rcrates as C
import cut_wire, cut_mutation as CM, cut_misc, retire, rewrite, rewrite2, facade, rmanifests, law, hsplit, prune

CATALOG = None

def structure():
    shutil.rmtree(STAGE, ignore_errors=True)
    combinators = cut_wire.run()
    glue_msgs = CM.cut_mutation_root()
    CM.cut_canonical_fragments()
    CM.add_fragment_mounts()
    extra = CM.cut_canonical_retire()
    crypto = CM.cut_format_crypto()
    CM.wire_core_additions(crypto, combinators)
    global CATALOG
    CATALOG = CM.split_wire_tests()
    CM.add_ids_test_mount()
    CM.move_value_refusal_test()
    cut_misc.small_edits()
    cut_misc.glue_roots(glue_msgs)
    cut_misc.presence_mounts()
    cut_misc.append_allocator([C.libroot("history"), C.libroot("local-interaction")])
    return extra

def widen(nd, name):
    t = read_stage_or_base(nd.file)
    it = top_defs(t)[name]
    old = it["text"]
    vis = it["vis"]
    m = re.search(r"(?m)^(" + re.escape(vis) + r"\s*)?(pub(?:\([^)]*\))?\s+)?(async\s+|const\s+|unsafe\s+)*(fn|struct|enum|const|static|type|trait)\s+" + re.escape(name) + r"\b", old) if vis else None
    if vis:
        new = old.replace(vis, "pub", 1)
    else:
        m2 = re.search(r"(?m)^(\s*)((?:async\s+|const\s+|unsafe\s+)*(?:fn|struct|enum|const|static|type|trait)\s+" + re.escape(name) + r"\b)", old)
        assert m2, (name, old[:80])
        new = old[:m2.start(2)] + "pub " + old[m2.start(2):]
    assert t.count(old) == 1
    write(nd.file, t.replace(old, new))

def main():
    extra = structure()
    report = {}
    retire.distribute(extra, report)
    delete(retire.RET)
    print("retire:", {k: len(v) for k, v in report.items()})
    model = rewrite.Model(CM.MOVED)
    changed = 0
    for key in C.KEYS:
        for f in crawl(C.libroot(key)):
            t = read_stage_or_base(f)
            n = rewrite.rewrite(t, key, model)
            if n != t:
                write(f, n); changed += 1
    print("rewritten files:", changed)
    model.refresh()
    origin = {CM.EDIT: CM.MUT, cut_wire.PRESENCE: cut_wire.WIRE, cut_wire.INTERACTION: cut_wire.WIRE, CM.FRAG_IDS: CM.CANON_TEXT, CM.FRAG_FOREIGN: CM.CANON_TEXT, CM.FRAG_POLICY: CM.CANON_TEXT, CM.IDS_TEST: CM.TEST_WIRE}
    cut_files = {cut_wire.WIRE, CM.MUT, CM.CANON_TEXT, CM.CANON_MUT, CM.FORMAT, CM.WIRE_CORE, CM.TEST_WIRE}
    rep = []
    print("pass2 changed:", rewrite2.run(model, origin, cut_files, rep))
    for r in rep: print("  P2", *r)
    for (d, name), nd in model.needs_pub.items():
        widen(nd, name)
        print("WIDENED", d, name, nd.key.replace(R, ""))
    print("pruned:", prune.run([CM.CANON_TEXT, R + "🦀️.rs", cut_wire.PRESENCE, R + "⚙️codec/🦀️.rs", R + "🧾️wire/🧪️tests/🔬️unit/🦀️.rs"]))
    print("history split:", hsplit.run())
    model.refresh()
    fa = facade.Facade(model)
    write(FACADE_ROOT, fa.render().rstrip("\n") + "\n\n#[cfg(test)]\n#[path = \"../../🧪️tests/🔬️facade/🦀️.rs\"]\nmod facade_tests;\n")
    registry_use, cat = CATALOG
    conf = "\n\n".join(fa.conformance())
    write(R + "🧪️tests/🔬️facade/🦀️.rs", "//! 🧪️ Facade conformance: every pre-split public path still resolves, and the kernel inference catalog registers independently of the four-facet descriptor.\n\n"
          + registry_use + "\n" + conf + "\n\n#[test]\nfn every_pre_split_public_path_resolves() {}\n\n" + cat + "\n")
    for (d, name), nd in model.needs_pub.items(): print("FACADE-NEEDS-PUB", d, name)
    rep = rmanifests.run()
    print("FACADE DEPS", rmanifests.run_facade())
    law.run(rep)
    cut_wire_done = True

if __name__ == "__main__":
    main()
