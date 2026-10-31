import sys, importlib
sys.path.insert(0, '.')
from common import *
from probe import *
for n in sys.argv[1:]:
    S = importlib.import_module('sp_' + n).build()
    doc, _ = assemble(S)
    import json
    print(n, doc['asset']['dimensions'])
    for sc in ['500', '200', '100', '50']:
        g = pose_gaps(S, S.shapes[sc], sc)
        print(' ', sc, len(g), g[:8])
    dump(S_ROOT + '/' + S.folder + '/🔣️.json', doc)
