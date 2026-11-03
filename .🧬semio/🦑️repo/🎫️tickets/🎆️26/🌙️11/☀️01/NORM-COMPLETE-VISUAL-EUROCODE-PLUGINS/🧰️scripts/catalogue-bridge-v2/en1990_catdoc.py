import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys, json; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import catalogue_doc as D
A = Art("en1990")
C = A.any + "/🧬️schema/💡️inferences/⚖️checks/📇️catalogue"
ents = D.entries(rd(C + "/🦀️.rs"))
vocab = json.loads(rd(C + "/📚️inputs/🔣️.json"))
cases = D.seed_cases(vocab, ents, D.demo_id)
base = json.loads(rd(A.fixd + "/💥️change-accidental-ad/✅apply/🔣️.before.json"))
base["cases"] = cases
wr(A.any + "/🧪️tests/🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json", json.dumps(base, indent=2, ensure_ascii=False) + "\n")
print(len(ents), len(cases))
