"""Item-level identity proof (svm): every top-level item of the pristine monolith (fn, struct, enum, impl, trait, const, macro call) must
reappear byte-identical in exactly one new file, modulo the `pub(super) ` visibility prefix. usage: itemdiff.py <plan.py> [--stage]"""
import collections, hashlib, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from split import load_plan, read, ROOT, F, WORK, REGION
from rslex import outline


def items_of(text):
    lines = text.split("\n")
    out = []
    for it in outline(text):
        if it["kind"] in ("use", "mod"):
            continue
        body = [l for l in lines[it["start"]:it["end"] + 1] if not REGION.match(l)]
        t = "\n".join(body).replace("pub(super) ", "").strip("\n")
        out.append((it["name"] or it["head"][:60], hashlib.sha256(t.encode()).hexdigest()[:16], len(body)))
    return out


def main():
    plan = load_plan(sys.argv[1])
    stage = "--stage" in sys.argv
    base = f"{WORK}/{plan['name']}/stage/" if stage else ROOT + "/"
    real_dir = os.path.dirname(ROOT + "/" + plan["real"])
    files = [plan["real"]] + [os.path.relpath(f"{real_dir}/{c['folder']}/🦀️.rs", ROOT) for c in plan["children"]]
    orig = items_of(read(F + "/" + plan["orig"]))
    new = []
    for rel in files:
        new.extend(items_of(read(base + rel)))
    a = collections.Counter((h) for _, h, _ in orig)
    b = collections.Counter((h) for _, h, _ in new)
    missing = a - b
    extra = b - a
    names = {h: n for n, h, _ in orig + new}
    print(f"[{plan['name']}] orig items={len(orig)} new items={len(new)} identical-multiset={'YES' if not missing and not extra else 'NO'}")
    for h, n in missing.items():
        print("   missing in new:", names[h])
    for h, n in extra.items():
        print("   extra in new:", names[h])


if __name__ == "__main__":
    main()
