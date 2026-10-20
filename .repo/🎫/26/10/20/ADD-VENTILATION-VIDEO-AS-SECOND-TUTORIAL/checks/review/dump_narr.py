import importlib.util, sys, pathlib, inspect
root = pathlib.Path("tutorial/energy/demand/Ventilation")
for f in sorted(root.glob("*/scene_*.py")):
    spec = importlib.util.spec_from_file_location("m_"+f.parent.name, f)
    m = importlib.util.module_from_spec(spec); sys.path.insert(0, str(f.parent))
    spec.loader.exec_module(m)
    print("#####", f, m.TITLE_DE)
    for n, c in inspect.getmembers(m, inspect.isclass):
        if n.startswith("Beat") and hasattr(c, "NARRATION") and c.__module__ == m.__name__:
            print("== ", n)
            for k, en, de in c.NARRATION:
                print(f"  [{k}] {de}")
