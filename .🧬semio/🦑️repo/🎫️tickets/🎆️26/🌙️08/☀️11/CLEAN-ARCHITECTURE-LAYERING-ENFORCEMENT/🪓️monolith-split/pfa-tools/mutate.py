import os,sys,re,importlib,subprocess
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import pfa
tag=sys.argv[1]; kind=sys.argv[2]; idx=int(sys.argv[3]) if len(sys.argv)>3 else 0
sp=importlib.import_module('specs.'+tag).SPEC
S=pfa.SCR
rows=[l.rstrip('\n').split('\t') for l in open(S+'/stage/'+tag+'.widen.tsv',encoding='utf-8')]
sel=[r for r in rows if r[1]==kind and (kind!='item' or r[2]=='fn')]
r=sel[idx]
child=r[0]
base=os.path.join(S,'sbx',tag,'new',os.path.dirname(sp['file']),child,'🦀️.rs')
t=open(base,encoding='utf-8').read()
name=r[3]
if kind=='item': pat=re.compile(r'pub\(super\) (fn '+re.escape(name)+r'\b)')
elif kind=='field': pat=re.compile(r'pub\(super\) ('+re.escape(name)+r'\s*:)')
elif kind=='member': pat=re.compile(r'pub\(super\) (fn '+re.escape(name)+r'\b)')
m=pat.search(t)
print('mutate',kind,child,r[2],name,bool(m))
t2=pat.sub(r'\1',t,count=1)
assert t2!=t
open(base,'w',encoding='utf-8').write(t2)
