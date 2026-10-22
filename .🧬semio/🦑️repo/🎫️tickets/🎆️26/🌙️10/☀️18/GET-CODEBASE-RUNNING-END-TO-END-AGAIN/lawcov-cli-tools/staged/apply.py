#!/usr/bin/env python3
# Verifies the lawcov-cli tests manifest. Dry-run by default; there is nothing to write (only new files were added).
import json, os, sys
ROOT = "/Users/ueli/Documents/semio"
manifest = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "tests-manifest.json"), encoding="utf8"))
missing = [p for p in manifest if not os.path.exists(os.path.join(ROOT, p))]
print("tests=%d missing=%d %s" % (len(manifest), len(missing), "(dry-run)" if "--write" not in sys.argv else "(write: nothing to apply)"))
sys.exit(1 if missing else 0)
