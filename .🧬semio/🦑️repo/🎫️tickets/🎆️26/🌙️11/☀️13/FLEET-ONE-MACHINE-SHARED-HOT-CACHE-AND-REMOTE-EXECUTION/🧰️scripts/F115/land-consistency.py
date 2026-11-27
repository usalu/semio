import os,subprocess,sys,uuid
root="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency"
stage=os.path.join(os.path.dirname(os.path.abspath(__file__)),"stage.py")
tmp=os.path.join(root,"."+uuid.uuid4().hex+".tmp")
subprocess.check_call([sys.executable,stage,"consistency",tmp])
s=open(tmp,encoding="utf8").read()
def sub(a,b):
    global s
    assert s.count(a)==1,a
    s=s.replace(a,b)
sub('import { createHash, randomUUID } from "node:crypto";\n','')
sub('import {ownProcessIdentityV1,processRecordAliveV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";\n','')
sub('import { existsSync, lstatSync, mkdirSync, readdirSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";\nimport { tmpdir } from "node:os";\n','import { existsSync, lstatSync, readdirSync, readFileSync } from "node:fs";\n')
sub('import { readBuildOwnerRegistryV1, type BuildOwnerRegistryV1 } from "../🚪️admission/🟦️.ts";\n','import { readBuildOwnerRegistryV1, type BuildOwnerRegistryV1 } from "../🚪️admission/🟦️.ts";\nimport { updateBuildOwnerRegistryV1 } from "./🔏️writer/🟦️.ts";\n')
open(tmp,"w",encoding="utf8").write(s)
os.replace(tmp,os.path.join(root,"🟦️.ts"))
print("landed")
