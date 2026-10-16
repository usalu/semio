"""Inventory of every test mount of the top crate: target file -> {cfg, module name, host file, kind}."""
import json, re, sys
from lib import *
import modtree
from reach import reach

MOUNT = re.compile(r"((?:[ \t]*#\[cfg[^\n]*\]\n)*)([ \t]*)#\[path\s*=\s*\"([^\"\n]+)\"\]\n[ \t]*(pub(?:\([^)]*\))? )?mod ([A-Za-z_0-9]+);\n")
INCL = re.compile(r"((?:[ \t]*#\[cfg[^\n]*\]\n)*)[ \t]*include!\(\"([^\"\n]+)\"\);\n")


def inventory():
    out = {}
    for f in reach():
        text = read(f)
        clean = blank(text)
        for m in MOUNT.finditer(text):
            tgt = os.path.normpath(os.path.join(os.path.dirname(f), m.group(3)))
            if "/🧪️tests/" in tgt or "test-hosts" in tgt or "worker-cell" in tgt:
                out.setdefault(tgt, []).append(dict(kind="mod", name=m.group(5), cfg=" ".join(l.strip() for l in m.group(1).split("\n") if l.strip()), host=f, text=m.group(0)))
        for m in INCL.finditer(text):
            tgt = os.path.normpath(os.path.join(os.path.dirname(f), m.group(2)))
            if "/🧪️tests/" in tgt:
                out.setdefault(tgt, []).append(dict(kind="include", name=None, cfg=" ".join(l.strip() for l in m.group(1).split("\n") if l.strip()), host=f, text=m.group(0)))
    return out


if __name__ == "__main__":
    inv = inventory()
    for t, rows in sorted(inv.items()):
        for r in rows:
            print(os.path.relpath(t, E)[-62:], "|", r["kind"], r["name"], "|", r["cfg"], "|", os.path.relpath(r["host"], E)[-50:])
