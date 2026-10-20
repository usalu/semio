#!/usr/bin/env python3
# 🧪 Mirrors the staged precision files under /private/tmp/ip/run/p at their destination layout; relative specifiers that leave the precision directory are re-pointed to the live files.
import os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from layout import NEW, P
RUN = "/private/tmp/ip/run"
S = HERE + "/staged2"
SPEC = re.compile(r"""(from\s*|import\s*\(\s*|import\s*)(["'])(\.{1,2}/[^"']+)\2""")
def main():
    shutil.rmtree(RUN, ignore_errors=True)
    for flat, dest in NEW.items():
        src = S + "/" + flat
        if not os.path.exists(src): continue
        out = RUN + "/p" + dest[len(P):]
        os.makedirs(os.path.dirname(out), exist_ok=True)
        text = open(src, encoding="utf8").read()
        def fix(m):
            target = os.path.normpath(os.path.join(os.path.dirname(dest), m.group(3)))
            return m.group(0) if target.startswith(P) else m.group(1) + m.group(2) + target + m.group(2)
        if flat.endswith(".ts"):
            text = re.sub(r"""from\s+"(ajv|minimatch)\"""", lambda m: 'from "/Users/ueli/Documents/semio/node_modules/%s"' % m.group(1), SPEC.sub(fix, text))
        open(out, "w", encoding="utf8").write(text)
    print("prepared", RUN + "/p")
main()
