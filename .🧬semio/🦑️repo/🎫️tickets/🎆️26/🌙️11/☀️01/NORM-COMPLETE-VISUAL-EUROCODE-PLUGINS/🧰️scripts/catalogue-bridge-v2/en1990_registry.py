import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import registry_ext as R
A = Art("en1990")
R.oracle(A, "change-accidental-ad")
R.taxonomy(A)
R.feature(A, A.any + "/🧪️tests/⚖️mutate-en1990-1", "change-accidental-ad")
print(len(LOG))
