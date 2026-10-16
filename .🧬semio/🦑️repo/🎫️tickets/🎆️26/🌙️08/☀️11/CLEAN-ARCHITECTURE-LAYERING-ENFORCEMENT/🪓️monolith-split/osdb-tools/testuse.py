"""🧪 Which non-pub top-level items of a module does a test file use? Usage: testuse.py module_file test_file"""
import sys,re
from outline import outline
from rs import strip,idents
mod,test=sys.argv[1:3]
src=open(mod,encoding='utf-8').read(); lines=src.split('\n')
priv={};pub={}
for kind,nm,s,e,h,vis in outline(mod):
    if kind in('use','impl'):continue
    m=re.match(r'\s*([A-Za-z_][A-Za-z0-9_]*)',nm or '')
    if not m: continue
    (pub if vis.startswith('pub') and '(' not in vis else priv)[m.group(1)]=(kind,vis.strip())
tt=idents(strip(open(test,encoding='utf-8').read()))
up=sorted(k for k in priv if k in tt)
print('test idents hitting private/crate items:',len(up))
for k in up: print('  ',priv[k][0],k,priv[k][1])
print('pub items used:',len([k for k in pub if k in tt]))
