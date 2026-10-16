# ⚖️ Losslessness of the WP5 conversion: the multiset of normalized non-blank lines of the WP5 staging files (and the Shell tests) before and after.
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg

def norm(l):
    s = l.strip()
    s = re.sub(r"impl<'a> \w+Ctx<'a>", "impl ShellState", s)
    s = re.sub(r"\bself\.as_\w+\(\)", "self", s)
    s = re.sub(r"\b(?:pub\(super\)|pub\(crate\)|pub)\s+", "", s)
    s = re.sub(r"&mut \*?\b(\w+)\b", r"\1", s)
    s = re.sub(r"\b(?:\w+Ctx)::of\(([^()]*)\)", r"\1", s)
    s = re.sub(r"\bSelf::|\bShellState::(?=[a-z])", "", s)
    s = re.sub(r"&mut self\b|&self\b", "SELF", s)
    return s

def bag(root, dirs, extra=()):
    c = collections.Counter()
    for d in dirs:
        base = cfg.stg(root, d)
        if not os.path.isdir(base): continue
        for dp, dn, fn in os.walk(base):
            for f in fn:
                if f.endswith(".rs") and os.path.basename(dp) != "🧺️bundle":
                    for l in open(os.path.join(dp, f), encoding="utf-8").read().split("\n"):
                        if l.strip() and not l.strip().startswith("//!"): c[norm(l)] += 1
    return c

if __name__ == "__main__":
    dirs = [d for d, _, _ in cfg.CRATES]
    a, b = bag(cfg.ORIG, dirs), bag(cfg.NEW, dirs)
    lost = a - b; gained = b - a
    print("lines before", sum(a.values()), "after", sum(b.values()), "lost", sum(lost.values()), "gained", sum(gained.values()))
    for k, v in lost.most_common(25): print("  -", v, k[:150])
    for k, v in gained.most_common(25): print("  +", v, k[:150])
