#!/usr/bin/env python3
"""usage: reloc_all.py [--apply]  For every stdio semio subset whose root crate carries retirement/member-owner macros for types that live in carve crates, report (and with --apply move) them."""
import os,re,sys,subprocess
os.chdir("/Users/ueli/Documents/semio")
DOIT="--apply" in sys.argv
sys.argv=[sys.argv[0],"/dev/null"]
exec(open(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/plug-mz-c-tools/modscan2.py").read().split("tot=0")[0])
S="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/"
for sub in sorted(os.listdir(S)):
    root=S+sub+"/🦀️.rs"
    if not os.path.isfile(root): continue
    r=open(root).read()
    lines=[l for l in r.split("\n") if re.match(r"semio_framework_value::artifact_retire_(leaf|struct)!\(schema::snapshot::|semio_s_artifact_stdio_semio_base::semio_(snapshot_retirement|member_owner)!",l)]
    if not lines: continue
    snapfile=os.path.realpath(S+sub+"/🧬️schema/📸️snapshot/🦀️.rs"); mutfile=os.path.realpath(S+sub+"/🧬️schema/🧬️mutations/🦀️.rs")
    owners={}
    for cf in subprocess.run(["rg","--files","-g","Cargo.toml",S+sub],capture_output=True,text=True).stdout.split("\n"):
        if not cf: continue
        t=open(cf).read()
        if "[package]" not in t: continue
        seen=set(); walk(targets(cf,t)[0][1],seen); seen={os.path.realpath(x) for x in seen}
        nm=re.search(r'name\s*=\s*"([^"]+)"',t).group(1)
        for f,k in ((snapfile,"snap"),(mutfile,"mut")):
            if f in seen: owners.setdefault(k,[]).append((nm,os.path.dirname(os.path.dirname(cf)) if False else os.path.dirname(cf)))
    rootname=re.search(r'name\s*=\s*"([^"]+)"',open(S+sub+"/📦️packages/🦀️rust/Cargo.toml").read()).group(1)
    sn=[o for o in owners.get("snap",[]) if o[0]!=rootname]; mu=[o for o in owners.get("mut",[]) if o[0]!=rootname]
    rootowns=[k for k in ("snap","mut") if any(o[0]==rootname for o in owners.get(k,[]))]
    print(sub,"macros",len(lines),"snap owners",[o[0][-25:] for o in sn],"mut owners",[o[0][-25:] for o in mu],"root owns",rootowns)
    if DOIT and sn and mu and not rootowns and not any("open =" in l for l in lines) and len(sn)==1 and len(mu)==1:
        sd=os.path.relpath(sn[0][1],S).rsplit("/📦️packages/🦀️rust",1)[0]; md=os.path.relpath(mu[0][1],S).rsplit("/📦️packages/🦀️rust",1)[0]
        print("  ->",sd,md)
        subprocess.run(["python3",".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/plug-mz-c-tools/relocate_semio.py",sub,sd,md])
