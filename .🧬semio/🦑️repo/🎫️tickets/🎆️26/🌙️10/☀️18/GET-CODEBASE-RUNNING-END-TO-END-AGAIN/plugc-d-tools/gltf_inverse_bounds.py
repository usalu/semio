#!/usr/bin/env python3
"""usage: gltf_inverse_bounds.py [--dry]  Repairs the gltf mutation leaves whose `inverse<Op>` helper bound names RemoveUsedExtensionMutation instead of the mutations its body builds, and adds those bounds to the ColdMutationKind impl."""
import re,sys,os,subprocess
G="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations"
dry="--dry" in sys.argv
files=subprocess.run(["rg","-l","--glob","🦀️.rs","pub fn inverse<Op>",G],capture_output=True,text=True).stdout.split("\n")
files=[f for f in files if f]
MUT=re.compile(r"pub fn mutation<Op>\([^)]*\)\s*->\s*Op\s*where\s*Op:\s*From<([\w:]+)>",re.S)
own={}
text={}
for f in files:
    t=open(f).read(); text[f]=t
    m=MUT.search(t)
    if m:
        full=m.group(1); own[f]=full; own[full.rsplit("::",1)[0]]=full
HELP=re.compile(r"(pub fn inverse<Op>\([^)]*\)\s*->\s*Vec<Op>\s*)where\s*\n\s*Op:\s*From<[\w:]+>,\s*\n(\{.*?\n\})",re.S)
changed=0;skipped=[]
for f in files:
    t=text[f]
    hm=HELP.search(t)
    if not hm: skipped.append((f,"no helper match")); continue
    body=hm.group(2)
    needed=[]
    for pm in re.finditer(r"((?:crate|super)(?:::\w+)+)::mutation\(",body):
        full=own.get(pm.group(1))
        if full is None: skipped.append((f,"unknown module "+pm.group(1))); full=None; break
        if full not in needed: needed.append(full)
    else:
        if re.search(r"(?<![:\w])mutation\(",body):
            if f not in own: skipped.append((f,"own mutation type unknown")); continue
            if own[f] not in needed: needed.append(own[f])
        bound=" + ".join("From<%s>"%n for n in needed)
        if needed:
            newh=hm.group(1)+"where\n    Op: "+bound+",\n"+body
        else:
            newh=hm.group(1)+body
        nt=t[:hm.start()]+newh+t[hm.end():]
        if needed:
            def addimpl(m2):
                blk=m2.group(0)
                for n in needed:
                    if ("From<%s>"%n) not in blk and not blk.rstrip().endswith("From<%s>,"%n.split("::")[-1]):
                        short=n.split("::")[-1]
                        if ("From<%s>"%short) in blk: continue
                        blk=re.sub(r"(Op:\s*protocol::ColdMutation<[^>]*>[^\n]*?)(,\s*\n\{)",lambda mm:mm.group(1)+" + From<%s>"%n+mm.group(2),blk,count=1)
                return blk
            nt=re.sub(r"impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for \w+\s*\nwhere\s*\n\s*Op:[^\n]*\n\{",addimpl,nt,count=1)
        if nt!=t:
            changed+=1
            if not dry: open(f,"w").write(nt)
print("files",len(files),"changed",changed,"skipped",len(skipped))
for s in skipped[:40]: print("  skip",s[0][-90:],s[1])
