import json, os, subprocess, sys
from lib import *
from crates import *
import overlay
overlay.build(SP + "/S3")
spec = [[cname(k), BY_KEY[k][2]] for k, *_ in CRATES]
spec.append(["semio-framework-pack", "PACKAGE_ROOT"])
env = dict(os.environ, CRATES=json.dumps([s for s in spec if s[1] != "PACKAGE_ROOT"]))
here = os.path.dirname(os.path.abspath(__file__))
print(subprocess.run(["bun", here + "/census_stage.ts"], env=env, capture_output=True, text=True).stdout)
