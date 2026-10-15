import os,sys,shutil
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from engine import *
T=os.path.dirname(os.path.abspath(__file__))
STAGE=os.environ.get('STAGE','/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/dirinf-ws/stage')
src=read_lines(T+'/orig-directory.rs.txt')
if src[-1]=='':pass
U='use super::*;'
C=[
 dict(name='model',folder='🧾️model',mod='model',doc='//! 🧾️ Directory records: the DTOs every backend and the service exchange. Independent of the artifact authority; it reaches back only to the four capability types it embeds.',
      ranges=[(45,675)],drop=[46,674],dedent=(47,673,4),uses=[],reexport=False,modvis='pub'),
 dict(name='capabilities',folder='🎟️capabilities',mod='capabilities',doc='//! 🎟️ Capabilities, identity assertions, local bootstrap transports, auth preflight and audit validators.',
      ranges=[(814,1621)],uses=[U]),
 dict(name='wire',folder='🔤️wire',mod='wire',doc='//! 🔤️ Wire conversions, descriptor and checkpoint validation, and the in-memory artifact projection.',
      ranges=[(1623,2049)],uses=[U]),
 dict(name='events',folder='🕰️events',mod='events',doc='//! 🕰️ The hybrid logical clock and the decided-but-unpersisted directory event. A leaf: it depends on neither the artifact authority nor any sibling module.',
      ranges=[(2051,2100)],uses=['use directory::os_directory::{DirectoryActor, DirectoryEventBody, Hlc};'],modvis='pub(crate)'),
 dict(name='decider',folder='🧭️decider',mod='decider',doc='//! 🧭️ The directory command decider: one command in, events and a result out.',
      ranges=[(2102,2341)],uses=[U]),
 dict(name='service',folder='🛠️service',mod='service',doc='//! 🛠️ DirectoryService: the serialized writer, projection rebuild, artifact CAS sweep and the verified checkpoint publisher adapter.',
      ranges=[(2343,3200)],uses=[U]),
 dict(name='port',folder='🔌️port',mod='port',doc='//! 🔌️ The HubDirectory port every persistence backend implements.',
      ranges=[(3202,3525)],uses=[U]),
 dict(name='dispatch',folder='🔀️dispatch',mod='dispatch',doc='//! 🔀️ HubDirectories: the closed enum that dispatches HubDirectory over the sqlite, postgres and neo4j backends.',
      ranges=[(3542,4341)],uses=[U]),
]
def wire(c):
    l=['#[path = "%s/🦀️.rs"]'%c['folder']]
    l.append('%smod %s;'%((c.get('modvis','')+' ') if c.get('modvis') else '',c['mod']))
    if c.get('reexport',True):l.append('@@REEXPORT:%s:%s@@'%(c['mod'],c['folder']))
    l.append('')
    return l
ptxt,files=assemble(src,C,wire)
d=STAGE+'/dir'
shutil.rmtree(d,ignore_errors=True)
os.makedirs(d)
open(d+'/🦀️.rs','w',encoding='utf-8').write(ptxt+('\n' if not ptxt.endswith('\n') else ''))
for f,t in files.items():
    os.makedirs(d+'/'+f)
    open(d+'/'+f+'/🦀️.rs','w',encoding='utf-8').write(t)
print('ok',{f:t.count('\n') for f,t in files.items()},ptxt.count('\n'))
