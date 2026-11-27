import os,uuid
f="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/🧰️adoption/🟦️.ts"
s=open(f,encoding="utf8").read()
def sub(a,b):
    global s
    assert s.count(a)==1,a
    s=s.replace(a,b)
sub('try{copyFileSync(join(resolve(checkout),"📋️build-owners.json"),join(resolve(workspace),"📋️build-owners.json"),constants.COPYFILE_EXCL);}catch(error){const code=(error as NodeJS.ErrnoException).code;if(code!=="EEXIST"&&code!=="ENOENT")throw error;}',
    'try{createOwnerRegistryTextV1(join(resolve(workspace),"📋️build-owners.json"),readFileSync(join(resolve(checkout),"📋️build-owners.json"),"utf8"));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}')
sub('import {captureNxAuthorizerSourcesV1,','import {createOwnerRegistryTextV1} from "../../../🏗️build/🕸️graph/🧾️registry-consistency/🔏️writer/🟦️.ts";\nimport {captureNxAuthorizerSourcesV1,')
sub('an existing file or a checkout without one is left as it is. */\nexport function adoptBuildOwnerRegistryForV1','an existing file or a checkout without one is left as it is, and a registry that `readBuildOwnerRegistryV1` refuses is refused here instead of copied. */\nexport function adoptBuildOwnerRegistryForV1')
t=f+"."+uuid.uuid4().hex+".tmp"
open(t,"w",encoding="utf8").write(s);os.replace(t,f);print("landed")
