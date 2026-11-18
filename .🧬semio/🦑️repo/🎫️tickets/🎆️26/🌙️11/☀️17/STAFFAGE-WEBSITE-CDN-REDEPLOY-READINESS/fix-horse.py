"""Horse 1:500 elevation: the tail segments swing out of the drawing plane, so they get the same extruded solids as their front and plan twins."""
import importlib.util
spec=importlib.util.spec_from_file_location('e','humans-animals-edit.py');e=importlib.util.module_from_spec(spec);spec.loader.exec_module(e)
S='🐕️animals/horse'
orig,doc=e.loadpair(S)
half={'tail-upper':'0.075','tail-lower':'0.0675'}
for s in doc['asset']['representations']['500']['geometry']['elevation']:
    if s['id'] in half:
        s['solid']={'kind':'extruded','sweep':'w','z0':e.N('-'+half[s['id']]),'z1':e.N(half[s['id']])}
e.save(S,doc,orig)
