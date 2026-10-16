import subprocess, sys, os
root=sys.argv[1]
bad=0; n=0
for dp,dn,fn in os.walk(root):
    for f in fn:
        if not f.endswith('.rs'): continue
        p=os.path.join(dp,f); n+=1
        r=subprocess.run(['rustfmt','+nightly-2026-07-20','--edition','2021','--emit','stdout','--config','skip_children=true,unstable_features=true',p],capture_output=True,text=True)
        if r.returncode!=0 or 'error' in r.stderr[:2000].lower():
            bad+=1; print('PARSE FAIL',p.replace(root,''),r.stderr[:600])
print('checked',n,'bad',bad)
