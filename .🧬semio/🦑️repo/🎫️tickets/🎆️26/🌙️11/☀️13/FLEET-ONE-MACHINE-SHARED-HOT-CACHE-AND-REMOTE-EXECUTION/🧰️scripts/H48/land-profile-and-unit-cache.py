import os,sys
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
def edit(path,pairs):
    s=open(path,encoding='utf8').read()
    for a,b in pairs:
        if s.count(a)!=1: sys.exit("anchor count %d in %s: %s"%(s.count(a),path,a[:90]))
        s=s.replace(a,b)
    if os.environ.get("H48_DRY"): print("dry ok",path[-60:]); return
    open(path+".h48tmp","w",encoding='utf8').write(s); os.replace(path+".h48tmp",path)
SCHEMA="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧬️schema/🔣️.json"
IMP='import{cargoProfileConstraintsForOwnerV1,CARGO_PROFILE_CONSTRAINT_SOURCE_V1}from"./📋️constraints/🟦️.ts";'
edit(R+"🏗️build/🎚️mode/🦀️profile/🟦️.ts",[
 (IMP,IMP+'\n/** 📚️ The checkout-relative paths of the Sources every profile policy declares (constraints, then schema); a private repository adopts exactly these so the policy capture reads them back below its own root. */\nexport const CARGO_PROFILE_POLICY_SOURCE_PATHS_V1:readonly [string,string]=Object.freeze([CARGO_PROFILE_CONSTRAINT_SOURCE_V1.path,"%s"] as const);'%SCHEMA),
 ('Object.freeze({owner:"profile-schema",path:"%s",digest:hash(JSON.stringify(schema))})'%SCHEMA,
  'Object.freeze({owner:"profile-schema",path:CARGO_PROFILE_POLICY_SOURCE_PATHS_V1[1],digest:hash(JSON.stringify(schema))})')])
A=R+"🧪️testing/🧫️private-repository/🧰️adoption/🟦️.ts"
edit(A,[
 ('import {createOwnerRegistryTextV1}','import {CARGO_PROFILE_POLICY_SOURCE_PATHS_V1} from "../../../🏗️build/🎚️mode/🦀️profile/🟦️.ts";\nimport {createOwnerRegistryTextV1}'),
 ('/** 📦️ Gives one private repository the captured Source closure',
  '/** 🎚️ Gives one private repository the declared Sources of the Cargo profile policy (constraints and schema), byte for byte as the checkout holds them, which the profile capture reads back below the private root; an existing file is left as it is. */\nexport function adoptCargoProfilePolicySourcesForV1(workspace:string,checkout:string):void{\n for(const path of CARGO_PROFILE_POLICY_SOURCE_PATHS_V1){const target=join(resolve(workspace),path);mkdirSync(dirname(target),{recursive:true});try{copyFileSync(join(resolve(checkout),path),target,constants.COPYFILE_EXCL);}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}}\n}\n/** 📦️ Gives one private repository the captured Source closure')])
P=R+"🧪️testing/🧫️private-repository/🟦️.ts"
edit(P,[
 ('adoptAuthoredCargoConfigurationForV1} from "./🧰️adoption/🟦️.ts";','adoptAuthoredCargoConfigurationForV1,adoptCargoProfilePolicySourcesForV1} from "./🧰️adoption/🟦️.ts";'),
 ('adoptBuildOwnerRegistryForV1(root,testCheckoutRootV1());','adoptBuildOwnerRegistryForV1(root,testCheckoutRootV1());adoptCargoProfilePolicySourcesForV1(root,testCheckoutRootV1());')])
U=R+"⚡️caching/🦀️cargo/🗃️unit-cache/"
ORIGIN='physicalSourceOriginV1(import.meta.url,(import.meta as PhysicalSourceModuleMetaV1).semioPhysicalSourceURL)'
edit(U+"🚪️launch/🟦️.ts",[
 ('import {fileURLToPath} from "node:url";','import {fileURLToPath} from "node:url";\nimport {physicalSourceOriginV1,type PhysicalSourceModuleMetaV1} from "../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";'),
 ('fileURLToPath(new URL("../📜️script.ts",import.meta.url))','fileURLToPath(new URL("../📜️script.ts",%s))'%ORIGIN),
 ('fileURLToPath(new URL("../🏁️entry.ts",import.meta.url))','fileURLToPath(new URL("../🏁️entry.ts",%s))'%ORIGIN)])
edit(U+"🏗️buildscript/🟦️.ts",[
 ('import {fileURLToPath} from "node:url";','import {fileURLToPath} from "node:url";\nimport {physicalSourceOriginV1,type PhysicalSourceModuleMetaV1} from "../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";'),
 ('fileURLToPath(new URL("./🟦️.ts",import.meta.url))','fileURLToPath(new URL("./🟦️.ts",%s))'%ORIGIN)])
edit(U+"🛰️fleet/📤️uploader/🟦️.ts",[
 ('import {fileURLToPath} from "node:url";','import {fileURLToPath} from "node:url";\nimport {physicalSourceOriginV1,type PhysicalSourceModuleMetaV1} from "../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";'),
 ('fileURLToPath(new URL("../../📜️script.ts",import.meta.url))','fileURLToPath(new URL("../../📜️script.ts",%s))'%ORIGIN)])
print("ok")
