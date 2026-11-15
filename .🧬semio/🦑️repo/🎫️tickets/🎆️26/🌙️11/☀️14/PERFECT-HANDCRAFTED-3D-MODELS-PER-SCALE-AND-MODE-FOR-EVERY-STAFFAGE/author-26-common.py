import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib04", os.path.join(here, "author-04-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
lib.GEN = os.path.join(here, "🗑️generated", "author-26")
Volume = lib.Volume
EOF_MARK = None
