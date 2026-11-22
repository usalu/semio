import sys, json
sys.path.insert(0, ".")
import lib
d = lib.load_base(); lib.strip_box(d["asset"]); p = lib.save_draft(d, "probe"); print(p)
