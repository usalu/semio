import sys, importlib
sys.path.insert(0, ".")
for n in sys.argv[1:]:
    m = importlib.import_module("a_" + n.replace("-", "_"))
    m.apply(m.ID, m.SPEC, m.TEXT)
    print("applied", m.ID)
