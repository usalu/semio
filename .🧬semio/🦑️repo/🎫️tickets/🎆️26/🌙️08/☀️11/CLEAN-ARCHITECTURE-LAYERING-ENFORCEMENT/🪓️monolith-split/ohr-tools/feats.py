"""🧷 OHR feature-args extractor: prepares a family once and stores the rustc `--cfg` args of lib and test in GEN/ohr-<family>-feats.json so sandbox reruns skip the 60 s prepare.
usage: feats.py <family>"""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/pos")
from poslib import *
import posunion as U
fam = sys.argv[1]
spec = json.load(open(f"{HERE}/families/{fam}.json", encoding="utf-8"))
mans = Manifests(Source(None))
mans.build_index()
crates = U.prepare(Source(None), mans, spec["crates"], False)
out = {}
for cfg in ("lib", "test"):
    en = U.feature_closure(crates, cfg == "test", mans)
    for c in crates:
        c.features = en[c.pkg]
    out[cfg] = U.cfg_args(crates)
log_write(f"{GEN}/{PREFIX}-{fam}-feats.json", json.dumps(out, ensure_ascii=False))
print({k: len(v) for k, v in out.items()})
