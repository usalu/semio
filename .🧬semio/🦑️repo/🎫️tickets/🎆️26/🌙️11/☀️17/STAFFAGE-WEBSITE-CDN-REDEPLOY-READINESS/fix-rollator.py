"""Person with rollator 1:500: grips and tyres were widened for legibility around their true centres and grew outward past the declared 0.62 m overall width; keep their outer faces at 0.31 m, where the 1:100 tyres end, and widen them inward."""
import importlib.util
spec=importlib.util.spec_from_file_location('e','humans-animals-edit.py');e=importlib.util.module_from_spec(spec);spec.loader.exec_module(e)
S='👤️humans/person-with-rollator'
orig,doc=e.loadpair(S)
g=doc['asset']['representations']['500']['geometry']
for s in g['front']:
    if s['id']=='grip-l': s['x']=e.N('0.233')
    if s['id']=='grip-r': s['x']=e.N('-0.31')
    if s['id']=='tyre-l': s['solid']['z0'],s['solid']['z1']=e.N('0.233'),e.N('0.31')
    if s['id']=='tyre-r': s['solid']['z0'],s['solid']['z1']=e.N('-0.31'),e.N('-0.233')
for s in g['plan']:
    if s['id']=='handlebar': s['y0'],s['y1']=e.N('-0.2775'),e.N('0.2775')
    if s['id']=='grip-l': s['y0']=s['y1']=e.N('0.2715')
    if s['id']=='grip-r': s['y0']=s['y1']=e.N('-0.2715')
    if s['id'].startswith('tyre') and s['id'].endswith('-l'): s['solid']['z0'],s['solid']['z1']=e.N('0.233'),e.N('0.31')
    if s['id'].startswith('tyre') and s['id'].endswith('-r'): s['solid']['z0'],s['solid']['z1']=e.N('-0.31'),e.N('-0.233')
e.save(S,doc,orig)
