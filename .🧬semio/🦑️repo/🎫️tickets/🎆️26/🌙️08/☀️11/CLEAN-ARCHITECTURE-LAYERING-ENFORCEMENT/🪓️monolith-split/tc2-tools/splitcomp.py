"""tc2: move a subset of the files of an applied companion into a new sibling companion.
usage: splitcomp.py <specA.json> <newspec-name> <newdir> <emoji> <summary> <fileA-substring-for-B>...   (files of A whose path contains any substring go to B)"""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mkcomp, modtree, fixrt

def main():
    a_path, bname, bdir, bemoji, bsummary = sys.argv[1:6]
    subs = sys.argv[6:]
    A = json.load(open(a_path, encoding="utf-8"))
    cm = modtree.build(mkcomp.abs_(A["dir"]) + "/🦀️.rs")
    rootp = os.path.normpath(mkcomp.abs_(A["dir"]) + "/🦀️.rs")
    chain_of = {os.path.normpath(m.file): m.chain for m in cm if not m.inline}
    top = {os.path.normpath(m.file) for m in cm if not m.inline and os.path.normpath(m.parent_file) == rootp}
    bfiles = [f for f in A["files"] if any(s in f for s in subs)]
    afiles = [f for f in A["files"] if f not in bfiles]
    old_root = open(mkcomp.abs_(A["dir"]) + "/🦀️.rs", encoding="utf-8").read()
    ext = [l for l in old_root.split("\n") if l.startswith("extern crate")]
    def tree_of(files):
        tree = {}
        for f in files:
            if os.path.normpath(mkcomp.abs_(f)) not in top:
                continue
            ch = chain_of[os.path.normpath(mkcomp.abs_(f))]
            node = tree
            for part in ch[:-1]:
                node = node.setdefault(part, {})
            node[ch[-1]] = mkcomp.abs_(f)
        return tree
    def write_root(spec, files):
        d = mkcomp.abs_(spec["dir"])
        os.makedirs(d + "/📦️packages/🦀️rust", exist_ok=True)
        lines = [f"//! {spec['emoji']} Companion test crate of `{spec['parent']}`: {spec['summary']} The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.", "", "#![cfg(test)]", ""] + ext + ([""] if ext else []) + mkcomp.emit_tree(tree_of(files), d)
        open(d + "/🦀️.rs", "w", encoding="utf-8").write("\n".join(lines) + "\n")
    B = dict(A, name=bname, dir=bdir, emoji=bemoji, summary=bsummary, files=bfiles, description=f"Companion test crate of {A['parent']}: {bsummary.rstrip('.')}")
    write_root(B, bfiles)
    A["files"] = afiles
    write_root(A, afiles)
    json.dump(A, open(a_path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    bp = os.path.join(os.path.dirname(os.path.abspath(a_path)), bname.replace("semio-s-artifact-", "") + ".json")
    json.dump(B, open(bp, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("A files", len(afiles), "B files", len(bfiles), "->", bp)
    for sp in (A, B):
        mkcomp.retransform(sp)
        p = mkcomp.plan(dict(sp, post=True, retransform=True))
        open(p["root"], "w", encoding="utf-8").write(mkcomp.render_root(p, sp))

main()
