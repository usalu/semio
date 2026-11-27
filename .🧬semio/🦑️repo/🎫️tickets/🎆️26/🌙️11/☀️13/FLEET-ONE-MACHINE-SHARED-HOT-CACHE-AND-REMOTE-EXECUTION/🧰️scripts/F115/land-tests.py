import os,uuid
d="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🧪️tests"
f=os.path.join(d,"🟦️.ts")
s=open(f,encoding="utf8").read()
a='withOwnerRegistryLockV1, checkBuildOwnerRegistry'
assert s.count(a)==1
s=s.replace(a,'checkBuildOwnerRegistry')
b='import { readBuildOwnerRegistryV1 } from "../../🚪️admission/🟦️.ts";\n'
assert s.count(b)==1
s=s.replace(b,b+'import { createOwnerRegistryTextV1, updateBuildOwnerRegistryV1, withOwnerRegistryLockV1 } from "../🔏️writer/🟦️.ts";\n')
add=open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"tests-append.ts.txt"),encoding="utf8").read()
if not s.endswith("\n"): s+="\n"
s+=add
t=f+"."+uuid.uuid4().hex+".tmp"
open(t,"w",encoding="utf8").write(s);os.replace(t,f);print("landed")
