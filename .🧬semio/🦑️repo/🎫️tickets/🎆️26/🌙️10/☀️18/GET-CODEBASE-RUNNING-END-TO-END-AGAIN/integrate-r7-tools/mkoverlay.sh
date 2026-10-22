#!/bin/sh
# usage: mkoverlay.sh <checkpoint dir> <dest>   clone of an integrate3 checkpoint with the two round-6 overlay hotfixes integrate3 has fixed in its makers (loader imports, caller import), the stale bun-repo composite removed
set -eu
src=$1; dest=$2
rm -rf "$dest"
python3 /private/tmp/integrate-r7/clone.py "$src" "$dest" | tail -1
python3 - "$dest" <<'P'
import sys,os
R=sys.argv[1]+"/"
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
f=R+B+"📥️source/🔏️closure/🍞️bun/🟦️.ts"
if os.path.exists(f):
    s=open(f,encoding='utf8').read()
    if "capturedSourceImportDiagnosticV1}from" not in s:
        s='import{capturedSourceImportDiagnosticV1}from"../🧾️diagnostics/🟦️.ts";\nimport{closureRefusalMessagesV1,closureRefusalTextV1,closureTargetSchemesV1}from"../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";\n'+s
        open(f,'w',encoding='utf8').write(s)
f=R+B+"📥️caller/🟦️.ts"
s=open(f,encoding='utf8').read()
if "declaredCapabilitiesV1(" in s and "import {declaredCapabilitiesV1}" not in s and "import{declaredCapabilitiesV1}" not in s:
    a='import { resolve } from "node:path";\n'
    assert a in s
    open(f,'w',encoding='utf8').write(s.replace(a,a+'import {declaredCapabilitiesV1} from "./🛰️service/🟦️.ts";\n',1))
b=R+".github/actions/bun-repo/action.yml"
if os.path.exists(b): os.remove(b)
P
