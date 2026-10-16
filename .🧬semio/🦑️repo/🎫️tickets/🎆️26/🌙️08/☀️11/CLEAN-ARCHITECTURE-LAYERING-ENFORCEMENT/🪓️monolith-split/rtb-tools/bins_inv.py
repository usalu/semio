"""Inventory of the test files the existing Shell test bins mount."""
import re, os
from lib import *
def bins():
    out = {}
    for d in sorted(os.listdir(T)):
        if "shell-tests-" not in d:
            continue
        root = T + "/" + d + "/🦀️.rs"
        text = read(root)
        mounts = []
        for m in re.finditer(r'((?:#\[cfg[^\n]*\]\n)*)#\[path = "([^"\n]+)"\]\n(pub )?mod ([A-Za-z_0-9]+);', text):
            mounts.append(dict(cfg=m.group(1).strip(), target=os.path.normpath(os.path.join(os.path.dirname(root), m.group(2))), name=m.group(4)))
        out[d] = dict(root=root, mounts=mounts, manifest_dir=T + "/" + d + "/📦️packages/🦀️rust")
    return out
if __name__ == "__main__":
    for d, b in bins().items():
        print(d, len(b["mounts"]))
