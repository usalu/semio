#!/usr/bin/env python3
"""📦 Staged patch 1 of lawcov-build: copies the staged test sets of excluded units (leases command, graph retry, graph generation-files) and registers every new test of 🏗build and ⚡caching in the lanes that run them. Dry run unless --apply."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from engine import Patch

patch = Patch("apply", "--apply" in sys.argv)
patch.copy_sets("main", False)
patch.register("main")
patch.finish()
