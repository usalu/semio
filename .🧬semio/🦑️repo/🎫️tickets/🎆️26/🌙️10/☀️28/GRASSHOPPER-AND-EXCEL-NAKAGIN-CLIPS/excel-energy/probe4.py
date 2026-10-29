import importlib, sys
sys.path.insert(0, "C:/git/semio")
n = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
c = n.climate()
print({k: (v if k != "months" else "...") for k, v in c.items()})
