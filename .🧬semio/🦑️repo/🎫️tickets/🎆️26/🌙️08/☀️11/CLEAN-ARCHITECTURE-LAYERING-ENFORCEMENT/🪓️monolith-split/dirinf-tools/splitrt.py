import os,sys,shutil
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from engine import *
T=os.path.dirname(os.path.abspath(__file__))
STAGE=os.environ.get('STAGE','/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/dirinf-ws/stage')
src=read_lines(T+'/orig-runtime.rs.txt')
U=['use super::*;']
SUP=[(r'\bsuper::',r'super::super::')]
IMPL='impl RetainedGisMapApprovalCommitterV1 {'
FEAT='#[cfg(feature = "integration-fixtures")]'
C=[
 dict(name='contract',folder='🧾️approval-contract',mod='approval_contract',doc='//! 🧾️ The commit contract: route errors, approval requests, receipts, the ingress and write authority ports, the checkpoint publisher port and the unavailable committer.',ranges=[(36,448)],uses=U,sub=SUP),
 dict(name='gis',folder='🌍️gis-map-binding',mod='gis_map_binding',doc='//! 🌍️ The GIS Map domain binding: GIS, drawing and value store types, the commit identity, the native GIS inference and the preview decoder. These items name the GIS crates inside the OS-level hub (wrong direction); moving this child into a hub-side GIS plugin is the follow-up.',ranges=[(450,503),(3364,3385),(3474,3516)],uses=U,sub=SUP),
 dict(name='committer',folder='🔐️committer',mod='committer',doc='//! 🔐️ The retained GIS Map approval committer: state and turn types, request guards, maintenance tasks, construction and recovery.',ranges=[(505,1426)],uses=U,post=['}'],sub=SUP),
 dict(name='prep',folder='🧱️document-preparation',mod='document_preparation',doc='//! 🧱️ Retained document preparation: mounting the three-store assembly and restoring the stores.',ranges=[(1428,1710)],uses=U,pre=[IMPL],post=['}'],sub=SUP),
 dict(name='turns',folder='🔁️commit-turns',mod='commit_turns',doc='//! 🔁️ Commit turns: driving one request or one abandoned turn to its terminal state.',ranges=[(1712,2109)],uses=U,pre=[IMPL],post=['}'],sub=SUP),
 dict(name='verify',folder='🔎️checkpoint-verify',mod='checkpoint_verify',doc='//! 🔎️ Checkpoint verification: preflight completion, publication frontiers, checkpoint publication and verification.',ranges=[(2111,2353)],uses=U,pre=[IMPL],post=['}'],sub=SUP),
 dict(name='close',folder='📕️commit-and-close',mod='commit_and_close',doc='//! 📕️ The retained commit, the undo commit and the closing sequence.',ranges=[(2355,2720)],uses=U,pre=[IMPL],sub=SUP),
 dict(name='jobs',folder='🧭️job-types-and-port',mod='job_types_and_port',doc='//! 🧭️ The GisMapApprovalCommitterV1 port implementation and the retained inference job types.',ranges=[(2722,2909)],uses=U,sub=SUP),
 dict(name='gate',folder='🚧️checkpoint-test-gate',mod='checkpoint_test_gate',doc='//! 🚧️ The integration-fixtures checkpoint test gate: control frames and the inherited descriptor reader.',ranges=[(2911,3064)],uses=U,sub=SUP,cfg=FEAT),
 dict(name='runtime',folder='🏭️inference-runtime',mod='inference_runtime',doc='//! 🏭️ HubInferenceRuntimeV1: the hub inference authority and its frozen-identity helpers.',ranges=[(3066,3362),(3387,3454)],uses=U,sub=SUP),
 dict(name='routes',folder='🛣️routes',mod='routes',doc='//! 🛣️ The authenticated inference route handlers and their contexts.',ranges=[(3456,3472),(3518,3897)],uses=U,sub=SUP),
]
def wire(c):
    l=[]
    if c.get('cfg'):l.append(c['cfg'])
    l.append('#[path = "%s/🦀️.rs"]'%c['folder'])
    l.append('mod %s;'%c['mod'])
    l.append('@@REEXPORT:%s:%s@@'%(c['mod'],c['folder']))
    l.append('')
    return l
ptxt,files=assemble(src,C,wire)
d=STAGE+'/rt'
shutil.rmtree(d,ignore_errors=True)
os.makedirs(d)
open(d+'/🦀️.rs','w',encoding='utf-8').write(ptxt+('\n' if not ptxt.endswith('\n') else ''))
for f,t in files.items():
    os.makedirs(d+'/'+f)
    open(d+'/'+f+'/🦀️.rs','w',encoding='utf-8').write(t)
print('ok',{f:t.count('\n') for f,t in files.items()},ptxt.count('\n'))
