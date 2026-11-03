import sys, importlib
sys.path.insert(0, '.')
from lib import *
for name in sys.argv[1:]:
    m = importlib.import_module(name.replace('-', '_'))
    doc = load(m.ID)
    save(m.ID, m.build(doc))
    print('built', m.ID)
