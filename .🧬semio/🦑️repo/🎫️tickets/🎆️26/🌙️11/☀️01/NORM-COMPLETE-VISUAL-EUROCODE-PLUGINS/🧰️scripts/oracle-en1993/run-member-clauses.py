import importlib.util, json, sys, pathlib, traceback
O = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles")
def load(p, name):
    spec = importlib.util.spec_from_file_location(name, p); m = importlib.util.module_from_spec(spec); sys.modules[name] = m; spec.loader.exec_module(m); return m
core = load(O / "🚦️check-outcome" / "🐍️.py", "core")
mod = load(O / "🧱️member-clauses" / "🐍️.py", "memberclauses")
only = sys.argv[1:]
fns = {"grade": lambda: mod.self_check, }
import time
for name in only or ["bow_cross_check","ltb_cross_check","cf_cross_check","frequency_cross_check","toughness_cross_check","stable_length_cross_check","interaction_cross_check","torsional","hybrid","classes"]:
    t=time.time()
    try:
        if name=="torsional":
            docs=[json.loads(p.read_text()) for p in sorted((O.parent/"🧪️tests"/"🧫️fixtures"/"📄️oracle-documents").glob("*/🔣️.json"))]
            sec=next(s for d in docs for s in d["sections"] if s["designation"]=="HEB 240"); mat=docs[0]["materials"][0]
            r=mod.torsional_cross_check(sec, mat, 4.0)
        elif name=="hybrid": r=mod.hybrid_plastic_moment(0.5,0.2,0.008,0.012,460e6,235e6)
        elif name=="classes":
            docs=[json.loads(p.read_text()) for p in sorted((O.parent/"🧪️tests"/"🧫️fixtures"/"📄️oracle-documents").glob("*/🔣️.json"))]
            r=mod.class_cross_check(core, docs)
        else:
            f=getattr(mod,name); r=f(core) if f.__code__.co_argcount else f()
        print(name, round(time.time()-t,1),"s", json.dumps(r)[:600])
    except Exception:
        print(name,"FAILED"); traceback.print_exc(limit=3)
