import sys, os, json
sys.path.insert(0,'.')
from model7 import PLUG, GEN, PLUGP
ROOTS={'instance-stores':'🧵️instance/🏪️stores','instance-history':'🧵️instance/📜️history','instance-children':'🧵️instance/🪆️children','instance-jobs':'🧵️instance/👷️jobs','instance-docs':'🧵️instance/📄️docs',
 'instance-extension':'🧵️instance/🧩️extensions','instance-render':'🧵️instance/🖼️render','instance-dispatch':'🧵️instance/📨️dispatch','instance-typed-ops':'🧵️instance/🧾️typed-operations','time-travel-turns':'⏪️time-travel/🧵️instance','tool-run-turns':'⏯️tool-run/🧵️instance','instance-commands':'🧵️instance/🧷️command-routes','instance-ladders':'🪜️maintenance-ladder','instance-app':'🧵️instance'}
CLASS_A={'instance-stores','instance-history','instance-children','instance-jobs','instance-docs','instance-dispatch'}
ENTRY_B={'instance-extension':'🧵️instance/🧩️extensions/🦀️.rs','instance-render':'🧵️instance/🖼️render/🦀️.rs','instance-typed-ops':'🧵️instance/🧾️typed-operations/🦀️.rs','instance-commands':'🧵️instance/🧷️command-routes/🦀️.rs','time-travel-turns':'⏪️time-travel/🧵️instance/🦀️.rs','tool-run-turns':'⏯️tool-run/🧵️instance/🦀️.rs','instance-ladders':'🪜️maintenance-ladder/🦀️.rs','instance-app':'🧵️instance/🦀️.rs'}
def staging(c):
    d=PLUG+ROOTS[c]+'/📥️staging'; out=[]
    if os.path.isdir(d):
        for dp,dn,fn in os.walk(d):
            for f in fn:
                if f.endswith('.rs'): out.append(os.path.relpath(os.path.join(dp,f),PLUG))
    return sorted(out)
own={};entry={};mounts={};extra={}
for c in ROOTS:
    sf=staging(c)
    if c in CLASS_A:
        entry[c]=[ROOTS[c]+'/🦀️.rs','A']; own[c]=[ROOTS[c]+'/📥️staging/']; mounts[c]=sf
    else:
        e=ENTRY_B[c]; entry[c]=[e,'B']; own[c]=[e,ROOTS[c]+'/📥️staging/']
        if c=='instance-app': own[c]+=['🧵️instance/🔌️plugin-app-impl/']
        mounts[c]=[] if c=='instance-app' else sf
    extra[c]=[]
json.dump(dict(own=own,entry=entry,mounts=mounts,extra=extra),open(GEN+'/pwp7-crates7.json','w'),ensure_ascii=False,indent=1)
print({c:len(mounts[c]) for c in ROOTS})
