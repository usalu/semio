# 🔎️ Who references name N (token level, production + test files of the whole plugin tree, from the closure listing)? usage: refs.py NAME...
import sys, re
from model4 import *
files, items, defs, owner, owner2, text, final1 = load()
import subprocess
for name in sys.argv[1:]:
    print('==', name)
    for it in items:
        if name in it['toks'] and (it['defname'] != name):
            print('  ', it['file'].replace('⚛️reactor/', 'R/'), it['s'], it['e'], it['kind'], it['defname'] or it.get('target') or it['head'][:40])
