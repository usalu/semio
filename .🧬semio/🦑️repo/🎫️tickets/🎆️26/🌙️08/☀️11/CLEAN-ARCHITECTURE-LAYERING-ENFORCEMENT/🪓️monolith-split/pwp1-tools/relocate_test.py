#!/usr/bin/env python3
# 🧪️ Hub-hosts a test file that names layers above its crate: explicit imports replace `use super::*`, the mount moves into hub 1 as a `#[cfg(test)]` module of the facade.
# usage: relocate_test.py <test file rel to PLUG> <snapshot parent file rel to PLUG> <monolith alias path e.g. crate::window_kits> <host module name> [--unmount '<old mount text>' <file rel to PLUG holding it>]
import os, re, sys, json
from carvelib import *
import explode_glob

LEDGER = TOOLS + "/relocated-tests.json"


def relocate(test_rel, parent_rel, alias, host, unmount=None):
    T = PLUG + "/" + test_rel
    P = SNAP + "/" + PLUG_REL + "/" + parent_rel
    text = read(T)
    imports, items, un = explode_glob.explode(text, read(P), None, parent_alias=alias)
    if re.search(r"(?m)^use super::\*;", text):
        text = re.sub(r"(?m)^use super::\*;", lambda m: imports, text, count=1)
        how = "replaced"
    else:
        text = imports + "\n" + text
        how = "prepended"
    write(T, text)
    if unmount:
        old, rel = unmount
        edit(PLUG + "/" + rel, old, "")
    mount = f'#[cfg(test)]\n#[path = "../../{test_rel}"]\nmod {host};\n'
    h = read(HUB)
    marker = "//#endregion 🏛️OwnershipFacet\n"
    assert h.count(marker) == 1
    assert f"mod {host};" not in h
    write(HUB, h.replace(marker, mount + marker))
    led = json.load(open(LEDGER)) if os.path.exists(LEDGER) else []
    led.append({"test": test_rel, "host": host, "alias": alias, "imports": how, "items": items, "unresolved": un})
    json.dump(led, open(LEDGER, "w"), ensure_ascii=False, indent=1)
    return how, items, un

if __name__ == "__main__":
    args = sys.argv[1:]
    print(relocate(args[0], args[1], args[2], args[3]))
