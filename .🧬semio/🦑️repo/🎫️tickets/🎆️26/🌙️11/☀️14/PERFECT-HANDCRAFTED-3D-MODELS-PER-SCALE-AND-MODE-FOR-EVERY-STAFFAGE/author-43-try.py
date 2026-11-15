import sys, importlib.util, json
sys.path.insert(0, ".")
spec = importlib.util.spec_from_file_location("cb", "author-43-city-bicycle.py")
cb = importlib.util.module_from_spec(spec); spec.loader.exec_module(cb)
from author_43_lib import *
def sil(den, mutate=None, rules="volumes-silhouette"):
    B = {50: cb.build50, 100: cb.build100}[den]()
    if mutate: mutate(B)
    cb.struct_coarse(B, den) if den != 50 else cb.struct50(B)
    B.thermal = [{"id": "th-tmp", "of": B.body[0]["id"], "region": "tyres"}]
    B.dec("t", "t")
    write_scale(cb.SLUG, den, B)
    out = run_check(cb.SLUG, den, rules)
    return [l[:330] for l in out.splitlines() if l.startswith(("ok", "FAIL"))]
