"""Group sibling top-level `use` statements that share a parent path into one use-tree. usage: groupuse.py [--apply] <file>..."""
import sys, re, os
RT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sys.path.insert(0, RT + "/rgplug-tools")
from rgp import *

USE = re.compile(r'^(pub(?:\([^)]*\))?\s+)?use\s+([^;{}]+?)(?:::\{([^{}]*)\})?\s*;\s*$')

def parse_use(line):
    m = re.match(r'^(pub(?:\([^)]*\))?\s+)?use\s+(.+?);\s*$', line)
    if not m:
        return None
    vis, body = m.group(1) or "", m.group(2).strip()
    if "{" in body and body.count("{") > 1:
        return None
    if body.endswith("*") or " as " in body and "{" in body:
        pass
    if "{" in body:
        mm = re.match(r'^(.+?)::\{(.*)\}$', body)
        if not mm:
            return None
        return vis, mm.group(1).strip(), [x.strip() for x in mm.group(2).split(",") if x.strip()]
    if "::" not in body:
        return None
    head, leaf = body.rsplit("::", 1)
    return vis, head.strip(), [leaf.strip()]

def regroup(text):
    lines = text.split("\n")
    out = []
    i = 0
    groups = {}
    order = []
    block = []
    def flush():
        nonlocal block
        if not block:
            return
        grp = {}
        order2 = []
        for kind, payload in block:
            if kind == "other":
                order2.append(("other", payload)); continue
            vis, head, leaves = payload
            key = (vis, head)
            if key not in grp:
                grp[key] = []
                order2.append(("grp", key))
            for l in leaves:
                if l not in grp[key]:
                    grp[key].append(l)
        for kind, payload in order2:
            if kind == "other":
                out.append(payload)
            else:
                vis, head = payload
                ls = grp[payload]
                if len(ls) == 1:
                    out.append("%suse %s::%s;" % (vis, head, ls[0]))
                else:
                    out.append("%suse %s::{%s};" % (vis, head, ", ".join(ls)))
        block = []
    for line in lines:
        if re.match(r'^(pub(\([^)]*\))?\s+)?use\s', line) and line.rstrip().endswith(";") and "\n" not in line:
            p = parse_use(line)
            if p and "*" not in p[2] and not any("{" in x for x in p[2]):
                block.append(("use", p)); continue
        if line.strip() == "" and block:
            block.append(("other", line)); continue
        flush()
        out.append(line)
    flush()
    return "\n".join(out)

if __name__ == "__main__":
    apply_ = "--apply" in sys.argv
    tot = 0
    for f in [a for a in sys.argv[1:] if not a.startswith("--")]:
        t = rd(f)
        n = regroup(t)
        d = len(t.encode()) - len(n.encode())
        tot += d
        print(d, f[-70:])
        if apply_ and n != t:
            wr(f, n)
    print("total saved", tot)
