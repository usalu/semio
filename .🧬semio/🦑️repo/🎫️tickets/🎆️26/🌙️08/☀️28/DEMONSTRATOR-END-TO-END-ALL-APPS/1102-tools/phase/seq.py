import os,sys,glob,shutil,subprocess,re
sys.path.insert(0,".")
from ph import R,POL,OUT,policy_files
T="/private/tmp/claude-501/-Users-ueli-Documents-semio/2c8171ff-0be5-4b23-8c30-2258199d4040/scratchpad/seq"
shutil.rmtree(T,ignore_errors=True);os.makedirs(T)
order=open(OUT+"/order.txt").read().split()
sup={l.split()[0] for l in open(OUT+"/supersedes.txt") if l.strip()}
pol=[p for p in sorted(glob.glob(POL+"/*.patch")) if os.path.basename(p)[:-6] not in sup]
mine=[f"{OUT}/{c}.patch" for c in order]
allp=pol+mine
touched=set()
for p in allp:touched|=set(policy_files(p))
for rel in touched:
    os.makedirs(os.path.dirname(f"{T}/{rel}"),exist_ok=True);shutil.copy(f"{R}/{rel}",f"{T}/{rel}")
bad=0
for p in allp:
    r=subprocess.run(["git","apply","--unsafe-paths","-p1",p],cwd=T,capture_output=True,text=True)
    if r.returncode:bad+=1;print("FAIL",os.path.basename(p),r.stderr[:200])
print("applied",len(allp),"patches (policy",len(pol),"mine",len(mine),") failures",bad)
pat=re.compile(r'\.unwrap\(\)|\.expect\(|panic!\(|unreachable!')
for p in mine:
    for rel in policy_files(p):
        a=open(f"{R}/{rel}",encoding="utf8").read().split("\n");b=open(f"{T}/{rel}",encoding="utf8").read().split("\n")
        sa=set(a)
        for l in b:
            if l not in sa and pat.search(l):print("PANIC-ON-CHANGED-LINE",rel[-40:],pat.search(l).group(0),l[:100])
