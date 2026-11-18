import sys,os
LIB=os.environ.get("R148_LIB","/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")
mode=sys.argv[1]
NEW='''/** 🔎️ Classifies exact literal module and URL references as portable source facts through the own string scanner; the compiler package is only the test oracle.
 * @see ./🏗️build/🧮️task-model/🔎️module-references/🧬️schema/🔣️.json */
export function nxBuildEngineReferencesV1(source){return classifyModuleReferencesV1(source).map(({kind,specifier})=>({kind,specifier}));}

'''
IMPORT='import {classifyModuleReferencesV1} from "./🏗️build/🧮️task-model/🔎️module-references/🟦️.ts";\n'
CR='import {createRequire} from "node:module";\n'
if mode=="apply":
    s=open(LIB,encoding="utf8").read()
    a="/** 🔎️ Classifies exact literal module and URL references as portable source facts. */"
    b="/** 📄️ Resolves actual engine modules"
    assert s.count(a)==1 and s.count(b)==1 and s.count(CR)==1 and s.count("createRequire(")==1
    open(sys.argv[2],"w",encoding="utf8").write(s)
    s=s[:s.index(a)]+NEW+s[s.index(b):]
    s=s.replace(CR,IMPORT,1)
    open(LIB,"w",encoding="utf8").write(s)
else:
    open(LIB,"w",encoding="utf8").write(open(sys.argv[2],encoding="utf8").read())
