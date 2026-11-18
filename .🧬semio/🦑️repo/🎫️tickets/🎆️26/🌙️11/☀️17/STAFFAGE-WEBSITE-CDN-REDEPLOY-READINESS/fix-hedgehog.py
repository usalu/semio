import importlib.util,sys
spec=importlib.util.spec_from_file_location('e','humans-animals-edit.py');e=importlib.util.module_from_spec(spec);spec.loader.exec_module(e)
S='🐕️animals/hedgehog'
orig,doc=e.loadpair(S)
Y=float(sys.argv[1])
for s in doc['asset']['representations']['100']['geometry']['front']:
    if s['id'] in('near-fore','far-fore'):
        for p in s['points']:
            if p[1]<-0.06: p[1]=e.N(repr(Y))
e.save(S,doc,orig)
