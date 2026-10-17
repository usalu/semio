import json, os
from specs.lib import *
_d = json.load(open(os.path.join(T, 'specs-frag', 'cut.json'), encoding='utf-8'))
SPEC = dict(orig='cut.rs', file=_d['file'], children=[dict(folder=c['folder'], mod=None) for c in _d['chunks']], frag=True)
