import importlib.util,sys,os
s=importlib.util.spec_from_file_location("r","animals-appliances-components-b-retrace.py");r=importlib.util.module_from_spec(s);s.loader.exec_module(r)
d=r.l.doc("pig")
os.environ["CLOSE"]="1"
for sid,so in {"hoof-h-f":"far-hind-hoof","hoof-f-f":"far-fore-hoof","hoof-h-n":"near-hind-hoof","hoof-f-n":"near-fore-hoof"}.items(): r.retrace(d,"pig","50","front",sid,so)
d.commit()
