"""🏷️ Round 2, concern 2: callback parameters that the JSON typing used to infer are annotated now that the schema is materialized at run time."""
def declare(edit, paths):
    D = paths.DEPENDENCIES
    S = D + "/💾️storage/🟦️.ts"
    edit(S, "types-prefix", "storage2", "recipe.clearEnvironmentPrefixes.some(prefix=>upper.startsWith(prefix))", "recipe.clearEnvironmentPrefixes.some((prefix:string)=>upper.startsWith(prefix))")
    edit(S, "types-ancestor", "storage2", "protocol.ancestorFields.map(key=>String(row[key as keyof BigIntStats]))", "protocol.ancestorFields.map((key:string)=>String(row[key as keyof BigIntStats]))")
    edit(S, "types-selected-identity", "storage2", "protocol.selectedFields.map(key=>String(row[key as keyof ArtifactReadFileIdentityV1]))", "protocol.selectedFields.map((key:string)=>String(row[key as keyof ArtifactReadFileIdentityV1]))")
    edit(S, "types-selected-facts", "storage2", "limits.selectedFields.map(key=>String(facts[key as keyof BigIntStats]))", "limits.selectedFields.map((key:string)=>String(facts[key as keyof BigIntStats]))")
    G = D + "/🧾️generation/💾️storage/🟦️.ts"
    edit(G, "types-store-roles", "generation2", "for(const[role,names]of Object.entries(policy.storeRoleFiles))for(const name of names)", "for(const[role,names]of Object.entries(policy.storeRoleFiles)as [string,string[]][])for(const name of names)")
    edit(G, "types-environment-prefix", "generation2", "policy.environmentPrefixes.some(prefix=>upper.startsWith(prefix))", "policy.environmentPrefixes.some((prefix:string)=>upper.startsWith(prefix))")
    I = D + "/🧾️incremental/💾️storage/🟦️.ts"
    edit(I, "types-selected-name", "storage2", "policy.selectedFields.map(name=>String(facts[name as keyof typeof facts]))", "policy.selectedFields.map((name:string)=>String(facts[name as keyof typeof facts]))")
    edit(I, "types-ancestor-name", "storage2", "policy.ancestorFields.map(name=>String(facts[name as keyof typeof facts]))", "policy.ancestorFields.map((name:string)=>String(facts[name as keyof typeof facts]))")
    P = paths.LIBRARY + "/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts"
    edit(P, "types-stamp-fields", "storage2", "SelectedRetirementCaptureProtocol.const.stampFields.map(key=>String(value[key as keyof ArtifactReadFileIdentityV1]))", "SelectedRetirementCaptureProtocol.const.stampFields.map((key:string)=>String(value[key as keyof ArtifactReadFileIdentityV1]))")
    edit(S, "types-home-config", "storage2", ".map(name=>path.join(home,name))", ".map((name:string)=>path.join(home,name))")
    edit(S, "types-config-config", "storage2", ".map(name=>path.join(config,name))", ".map((name:string)=>path.join(config,name))")
