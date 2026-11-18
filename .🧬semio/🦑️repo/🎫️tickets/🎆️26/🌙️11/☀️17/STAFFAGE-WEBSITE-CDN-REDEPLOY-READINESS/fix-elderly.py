"""Elderly with cane: the 1:200 and 1:500 cane capsules are thicker than the 1:50 shaft for print legibility and grew forward past the true ferrule/shaft edge, so the forward length exceeded the declared 0.54 m; keep the legibility growth behind the true front edge by offsetting the capsule centre line (the 1:50 front edge is the shaft radius 0.0105 m)."""
import importlib.util,sys
spec=importlib.util.spec_from_file_location('e','humans-animals-edit.py');e=importlib.util.module_from_spec(spec);spec.loader.exec_module(e)
S='👤️humans/elderly-with-cane'
orig,doc=e.loadpair(S)
sign=float(sys.argv[1])
for d,x1,r in (('200','0.86',0.018),('500','0.86',0.0325)):
    for s in doc['asset']['representations'][d]['geometry']['elevation']:
        if s['id']=='cane':
            s['x1']=e.N(x1)
            off=sign*(r-0.0105)
            s['y0']=e.N(repr(round(off,5))); s['y1']=e.N(repr(round(off,5)))
e.save(S,doc,orig)
