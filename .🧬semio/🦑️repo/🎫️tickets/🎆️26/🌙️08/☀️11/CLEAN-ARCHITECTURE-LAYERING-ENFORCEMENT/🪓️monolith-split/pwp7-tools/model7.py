import sys, os, re, json, collections
P6='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/pwp6-tools'
sys.path.insert(0,P6)
_cwd=os.getcwd(); os.chdir(P6)
from model6 import parse_file, mask, PLUGP, ROOT
os.chdir(_cwd)
MS=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GEN=MS+'/🗑️generated'
PLUG=ROOT+PLUGP
FIELDS=['app','mounted_policy','constructor','stores','roster','ingress','interaction','history','host_events','txn','child_composition','pending','tools','media','ingest','maintenance','close','tool_runs','time_travel','tool_machines']
TIER2_FILES='''🧵️instance/🧷️command-routes/🦀️.rs
🧵️instance/🧷️command-routes/📥️staging/🎟️lease/🦀️.rs
🧵️instance/🧷️command-routes/📥️staging/📨️dispatch/🦀️.rs
🧵️instance/🧷️command-routes/📥️staging/🚪️ingress/🦀️.rs
🧵️instance/🧷️command-routes/📥️staging/🤝️group/🦀️.rs
🧵️instance/🧾️typed-operations/🦀️.rs
🧵️instance/🧾️typed-operations/📥️staging/🛠️tool-machine/🦀️.rs
🧵️instance/🖼️render/🦀️.rs
🪜️maintenance-ladder/🦀️.rs
🪜️maintenance-ladder/📥️staging/🏪️lifecycle-stores/🦀️.rs
🪜️maintenance-ladder/📥️staging/🔁️document-replacement/🦀️.rs
🪜️maintenance-ladder/📥️staging/🔄️full-operation/🦀️.rs
🪜️maintenance-ladder/📥️staging/🔒️close-ladder/🦀️.rs
🪜️maintenance-ladder/📥️staging/🗝️envelope-ladder/🦀️.rs
🪜️maintenance-ladder/📥️staging/🚪️ingress/🦀️.rs
⏪️time-travel/🧵️instance/🦀️.rs
⏯️tool-run/🧵️instance/🦀️.rs
🧵️instance/🔌️plugin-app-impl/🦀️.rs
🧵️instance/📥️staging/📨️dispatch/🦀️.rs
🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs
🧵️instance/📥️staging/🚪️ingress/🦀️.rs
🧵️instance/📥️staging/🧬️composer-api/🦀️.rs
🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs'''.split('\n')
def load_methods():
    out=[]
    for rel in TIER2_FILES:
        f=parse_file(rel)
        for pi,p in enumerate(f['pieces']):
            if p.kind!='vcsimpl': continue
            for mi,m in enumerate(p.members):
                txt='\n'.join(f['R'][m.a:m.e])
                out.append(dict(file=rel,piece=pi,idx=mi,name=m.name,trait=p.trait,vis=m.vis,a=m.a,e=m.e,text=txt,masked=mask(txt)))
    return out
