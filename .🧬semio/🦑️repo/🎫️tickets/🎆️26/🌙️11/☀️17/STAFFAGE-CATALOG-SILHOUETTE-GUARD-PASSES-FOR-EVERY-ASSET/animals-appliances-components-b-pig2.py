import importlib.util,sys
s=importlib.util.spec_from_file_location("r","animals-appliances-components-b-retrace.py");r=importlib.util.module_from_spec(s);s.loader.exec_module(r)
d=r.l.doc("pig")
M={"gaskin-f":"far-gaskin","cannon-h-f":"far-hind-cannon","hoof-h-f":"far-hind-hoof","forearm-f":"far-forearm","cannon-f-f":"far-fore-cannon","hoof-f-f":"far-fore-hoof","gaskin-n":"near-gaskin","cannon-h-n":"near-hind-cannon","hoof-h-n":"near-hind-hoof","forearm-n":"near-forearm","cannon-f-n":"near-fore-cannon","hoof-f-n":"near-fore-hoof"}
for sid,so in M.items(): r.retrace(d,"pig","50","front",sid,so)
d.commit()
