import sys
p=sys.argv[1]
s=open(p,encoding='utf8').read()
start=s.index(' const rootFiles=limits.overlay.rootFiles.includes("*")')
end=s.index(' for(const entry of changeset.entries){\n  const path=resolve(overlay,entry.path);')
block=s[start:end]
call=' await landLinkWorkspaceV1(root,overlay,clone,ports.signal,limits);\n'
s=s[:start]+call+s[end:]
fn='''/** 🔗️ Links what an overlay (or a source snapshot) shares with the live tree: the root files it lacks, the dependency folder bound to the overlay, the mirrored cache folders, the heavy members of the Rust roots and the other declared links; whatever already exists is kept, so the call is repeatable. */
export async function landLinkWorkspaceV1(root:string,overlay:string,clone:(source:string,target:string,signal?:AbortSignal)=>Promise<void>=landCloneTreeV1,signal?:AbortSignal,limits:LandPolicyV1=readLandPolicyV1()):Promise<void>{
'''+block.replace('ports.signal','signal')+''' for(const name of limits.overlay.rustRoots)if(existsSync(join(overlay,name))&&existsSync(join(root,name)))for(const child of limits.overlay.heavyMembers)if(existsSync(join(root,name,child))&&!existsSync(join(overlay,name,child)))symlinkSync(join(realpathSync(join(root,name)),child),join(overlay,name,child),"junction");
}

'''
open('/tmp/.h95-unused','w').close() if False else None
marker='/** 🧱️ Builds the scratch overlay of the live tree'
i=s.index(marker)
s=s[:i]+fn+s[i:]
open(p,'w',encoding='utf8').write(s)
