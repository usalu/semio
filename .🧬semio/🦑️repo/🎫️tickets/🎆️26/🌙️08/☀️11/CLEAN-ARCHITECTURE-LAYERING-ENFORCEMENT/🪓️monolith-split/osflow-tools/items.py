"""📋️ Top-level item outline of one Rust file. usage: items.py <file>"""
import sys,os
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from rlex2 import lex2
from wsan import top_items,classify
p=sys.argv[1]
t=open(p,encoding='utf-8').read()
L=[l+'\n' for l in t.split('\n')][:-1]
I=lex2(L)
for it in top_items(L,I):
    classify(it)
    print(f"{it['start']+1}-{it['end']+1}\t{it['end']-it['start']+1}\t{it['kind']}\t{it['text'][:110]}")
