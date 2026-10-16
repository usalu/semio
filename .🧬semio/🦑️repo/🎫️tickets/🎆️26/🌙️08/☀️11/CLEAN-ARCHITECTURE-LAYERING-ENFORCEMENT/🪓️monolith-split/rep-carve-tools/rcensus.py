"""🧮 Builds the base+stage overlay and prints the census rows of every carved crate and the facade."""
import json, os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import rcrates as C, runion
dest = SP + "/S3"
runion.overlay(dest)
spec = [[C.cname(k), os.path.relpath(REPO + C.libroot(k), REPO), C.pkgdir(k)] for k in C.KEYS] + [["semio-framework-replication", FACADE_ROOT, FACADE_PKG]]
env = dict(os.environ, OVERLAY=dest, CRATES=json.dumps(spec))
here = os.path.dirname(os.path.abspath(__file__))
r = subprocess.run(["bun", here + "/rcensus.ts"], env=env, capture_output=True, text=True)
print(r.stdout, r.stderr[:500])
