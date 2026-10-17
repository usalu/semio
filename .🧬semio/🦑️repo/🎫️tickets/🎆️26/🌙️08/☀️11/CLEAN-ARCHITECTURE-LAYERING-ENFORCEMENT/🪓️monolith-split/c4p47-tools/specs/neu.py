import json, os
from specs.lib import *
_S = json.load(open(os.path.join(T, 'specs-frag', 'neu.json'), encoding='utf-8'))
SPEC = dict(orig='neu.rs', file=_S['file'], children=[dict(folder=c['folder']) for c in _S['chunks']], frag=True, external=[])
