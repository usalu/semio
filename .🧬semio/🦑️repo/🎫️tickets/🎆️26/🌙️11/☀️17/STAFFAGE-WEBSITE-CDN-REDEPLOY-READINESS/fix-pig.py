"""Pig 1:200 and 1:500: the merged hindbody carried the tail but stopped at the rump; pull its rear outline back so the drawn length matches the 1:100 tail (about 1.24 m)."""
import importlib.util,sys
spec=importlib.util.spec_from_file_location('e','humans-animals-edit.py');e=importlib.util.module_from_spec(spec);spec.loader.exec_module(e)
S='🐕️animals/pig'
orig,doc=e.loadpair(S)
for d in ('200','500'):
    for s in doc['asset']['representations'][d]['geometry']['plan']:
        if s['id']=='hindbody':
            for p in s['points']:
                if float(p[0])<-0.125: p[0]=e.num(float(p[0])-0.028)
e.save(S,doc,orig)
