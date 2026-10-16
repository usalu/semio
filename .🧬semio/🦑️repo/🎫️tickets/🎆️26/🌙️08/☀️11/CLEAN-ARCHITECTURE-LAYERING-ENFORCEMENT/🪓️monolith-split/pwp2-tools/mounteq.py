#!/usr/bin/env python3
# 🔗️ Mount equivalence of hub 2: every module name the snapshot hub (and the two root-resident units) defined is still defined by the new hub text (extern crate, re-export or thin module), with the same cfg attributes and region markers.
import re, sys, os
ARGS = list(sys.argv[1:])
from common import *
import model as M
import engine as E
import hubgen
import modmap
from rs import blank

def names_old():
    txt = read(SNAP + '/' + hubgen.HUB_REL)
    out = {}
    for m in modmap.mounts_of(txt, '🪝️mounts/📜️wp2-declaration'):
        out[m['name']] = m['cfg']
    for n in ('app_emission', 'tool_contract'):
        out[n] = ''
    return out

def names_new(hub):
    clean = blank(hub)
    out = set()
    for m in re.finditer(r'(?m)^(?:#\[[^\]\n]*\]\s*\n)*(?:pub(?:\([^)]*\))?\s+)?(?:mod\s+(\w+)\s*[;{]|extern\s+crate\s+\w+\s+as\s+(\w+)\s*;|use\s+[\w:]+::(\w+)\s*;)', clean):
        out.add(m.group(1) or m.group(2) or m.group(3))
    return out

def regions(text):
    return re.findall(r'//#(?:end)?region\s+(\S+)', text)

def main():
    E.set_input(PLUG)
    hub = hubgen.build_hub().replace('//#endregion 📜️DeclarationFacet', hubgen.extra_units() + '\n//#endregion 📜️DeclarationFacet', 1)
    old = names_old()
    new = names_new(hub)
    missing = [n for n in old if n not in new]
    print('old modules:', len(old), '; defined by the new hub:', len(old) - len(missing), '; missing:', missing)
    ro, rn = regions(read(SNAP + '/' + hubgen.HUB_REL)), regions(hub)
    print('region markers old/new:', len(ro), len(rn), '; every old marker kept in order:', [x for x in ro if x not in rn] == [])
    return 1 if missing else 0

if __name__ == '__main__':
    sys.exit(main())
