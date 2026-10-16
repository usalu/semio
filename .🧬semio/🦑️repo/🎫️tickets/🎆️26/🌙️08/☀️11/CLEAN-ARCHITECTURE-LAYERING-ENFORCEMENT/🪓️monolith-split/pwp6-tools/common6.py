import sys, os, re, json, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
MS = os.path.dirname(HERE)
os.environ.setdefault('CLOSURE', MS + '/🗑️generated/pwp0-closure.json')
import plan_tools as pt
ROOT = pt.ROOT
PLUGP = pt.P
PLUG = ROOT + PLUGP
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad'
GEN = MS + '/🗑️generated'
def plug_files():
    return [l.rstrip('\n') for l in open(SCR + '/plug-rs-files.txt', encoding='utf-8')]
def read(rel):
    return open(ROOT + rel if not rel.startswith('/') else rel, encoding='utf-8').read()
def struct_fields():
    f = PLUG + '🧵️instance/🦀️.rs'
    its, L, R = pt.outline(f)
    va = [it for it in its if it['kind']=='struct' and it['name'].startswith('VcsArtifactApp')][0]
    return va, L, R
