import importlib.util
s=importlib.util.spec_from_file_location("l","animals-appliances-components-b-lib.py");l=importlib.util.module_from_spec(s);s.loader.exec_module(l)
d=l.doc("pig")
l.shape(d,"50","front","hindbody",cy=-0.0324,ry=0.2534,rx=0.2075)
l.shape(d,"50","front","chest",cy=-0.036,ry=0.2506,rx=0.2085)
d.commit()
