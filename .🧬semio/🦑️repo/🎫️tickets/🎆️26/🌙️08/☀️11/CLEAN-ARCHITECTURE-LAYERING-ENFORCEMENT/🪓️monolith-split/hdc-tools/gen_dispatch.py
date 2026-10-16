"""Regroups the closed-enum dispatch by capability trait (stage only)."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *
from portparse import parse_members

ORIG = SC + "/hdc-orig/🌎️hub/📇️directory/🔀️dispatch/🦀️.rs"
OUT = SC + "/hdc-stage/🔀️dispatch/🦀️.rs"


def main():
    text = read(ORIG)
    lines = text.split("\n")
    a = lines.index("impl HubDirectory for HubDirectories {")
    b = max(i for i, l in enumerate(lines) if l == "}" and i > a)
    assert lines[b + 1] == "//#endregion 🔖️Dispatch", lines[b + 1]
    items = parse_members(lines[a + 1:b])
    mem = {}
    for it in items:
        if it["kind"] == "member":
            assert it["name"] not in mem
            mem[it["name"]] = it
    implemented = [m for m in mem]
    expect = {m for sec in SECTIONS.values() for t, ms in sec for m in ms if m not in TRAIT_DEFAULTED}
    assert set(mem) == expect, (set(mem) ^ expect)
    out = lines[:a]
    first = True
    for sec, traits in SECTIONS.items():
        for t, names in traits:
            if not first:
                out.append("")
            first = False
            out.append("impl %s for HubDirectories {" % t)
            req = [n for n in names if n not in TRAIT_DEFAULTED]
            for k, n in enumerate(req):
                if k:
                    out.append("")
                out += mem[n]["doc"] + mem[n]["lines"]
            out.append("}")
    out += lines[b + 1:]
    new = "\n".join(out)
    alltr = sorted(t for sec in SECTIONS.values() for t, _ in sec)
    ol = new.split("\n")
    hit = [i for i, l in enumerate(ol) if l.startswith("use semio_hub_directory::{") and "HubDirectory" in l]
    assert len(hit) == 1
    ol[hit[0]] = merge_use_list(ol[hit[0]], {"HubDirectory"}, alltr)
    new = "\n".join(ol)
    new = new.replace("match-delegating capability trait impls", "match-delegating impl of each capability trait")
    new = new.replace("dispatches HubDirectory over", "dispatches the hub directory capability traits over")
    new = new.replace("every `HubDirectory` backend", "every hub directory backend")
    new = new.replace("match-delegating `impl HubDirectory`", "match-delegating capability trait impls")
    write(OUT, new)
    print("dispatch staged", len(new.split("\n")), "lines")


if __name__ == "__main__":
    main()
