"""Tour guide 1:500 elevation: the second part of the waving forearm swings out of the drawing plane, so it gets the extruded solid of its first part at the forearm's lateral position."""
import importlib.util
spec=importlib.util.spec_from_file_location('e','humans-animals-edit.py');e=importlib.util.module_from_spec(spec);spec.loader.exec_module(e)
S='👤️humans/tour-guide'
orig,doc=e.loadpair(S)
for s in doc['asset']['representations']['500']['geometry']['elevation']:
    if s['id']=='forearm-r-part-2': s['solid']={'kind':'extruded','sweep':'w','z0':e.N('-0.195'),'z1':e.N('-0.185')}
e.save(S,doc,orig)
