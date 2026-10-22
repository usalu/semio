#!/usr/bin/env python3
"""🔧 Staged patch 2 of lawcov-build: the production fixes of five defects (identity key framing, pool double close, snapshot base64 pattern, patch hunk counts) with their three dependent oracles in other tests, plus the corrected versions of the author's own earlier test sets that expect the fixed behaviour, the test sets that need the injected host parameter of package acquisition and the exported restore of the pristine SDK, and their registration. Atomic: refuses everything on any drift. Apply after apply.py. Dry run unless --apply."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from engine import Patch

patch = Patch("apply-fixes", "--apply" in sys.argv)
patch.edits("main")
patch.copy_sets("main", True)
patch.copy_sets("main", False, "fixes")
patch.register("fixes")
patch.finish()
