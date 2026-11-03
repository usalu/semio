import importlib.util, sys, unittest
p = "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️staffage_only.py"
s = importlib.util.spec_from_file_location("so_t", p)
m = importlib.util.module_from_spec(s); sys.modules["so_t"] = m; s.loader.exec_module(m)
unittest.main(module=m, argv=["x"], exit=False)
