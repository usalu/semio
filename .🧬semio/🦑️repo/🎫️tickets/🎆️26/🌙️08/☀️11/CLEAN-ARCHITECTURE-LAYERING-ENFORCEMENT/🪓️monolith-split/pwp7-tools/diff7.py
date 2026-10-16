import sys, json, collections, os, re
sys.path.insert(0,'.')
import importlib.util
spec=importlib.util.spec_from_file_location('oracle7','oracle7.py'); o=importlib.util.module_from_spec(spec); spec.loader.exec_module(o)
tag,cfg=sys.argv[1],sys.argv[2]
sbA=o.SCR+'/pwp7-sb-'+tag+'/A'; sbB=o.SCR+'/pwp7-sb-'+tag+'/B'
ea=open(o.GEN+'/pwp7-oracle-%s-A-%s.jsonl'%(tag,cfg)).read(); eb=open(o.GEN+'/pwp7-oracle-%s-B-%s.jsonl'%(tag,cfg)).read()
a,b=o.parse(ea,sbA),o.parse(eb,sbB)
new,old=a-b,b-a
print('A',sum(a.values()),'B',sum(b.values()),'new',sum(new.values()),'old',sum(old.values()))
for k,v in sorted(new.items(),key=lambda kv:-kv[1]): print('+new',v,k)
for k,v in sorted(old.items(),key=lambda kv:-kv[1]): print('-old',v,k)
