# 👁️ Print the converted text of one method of a WP5 crate, optionally only the lines around a needle.
import sys, re, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, deps
crate, name = sys.argv[1], sys.argv[2]; needle = sys.argv[3] if len(sys.argv) > 3 else None
for n in deps.load(cfg.NEW):
    if n.kind == "method" and n.crate == crate and n.name == name:
        L = n.text.split("\n")
        if needle:
            for i, l in enumerate(L):
                if needle in l: print("\n".join(f"{k+1:4d} {x[:200]}" for k, x in enumerate(L[max(0, i - 8):i + 6], max(0, i - 8))))
        else: print("\n".join(f"{k+1:4d} {x[:200]}" for k, x in enumerate(L)))
