"""Parent with stroller 1:500: the legibility-widened tyres (0.075 m) grew outward past the declared 0.6 m track; keep their outer face where the other scales have it and widen them inward."""
import importlib.util
spec=importlib.util.spec_from_file_location('e','humans-animals-edit.py');e=importlib.util.module_from_spec(spec);spec.loader.exec_module(e)
S='👤️humans/parent-with-stroller'
orig,doc=e.loadpair(S)
new={'tyre-rear-l':('0.225','0.3'),'tyre-rear-r':('-0.3','-0.225'),'tyre-front-l':('0.145','0.22'),'tyre-front-r':('-0.22','-0.145')}
for v in ('front','plan'):
    for s in doc['asset']['representations']['500']['geometry'][v]:
        if s['id'] in new:
            s['solid']['z0'],s['solid']['z1']=(e.N(x) for x in new[s['id']])
e.save(S,doc,orig)
