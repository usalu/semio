#!/usr/bin/env python3
"""📋 Staged patch 3 of lawcov-build (integrate-r7): registers the 13 test files of the lawcov-tests executor (their sets are already live) in the lane list of 🏗build and the lane law-coverage-tests of ⚡caching. Dry run unless --apply."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from engine import Patch

patch = Patch("apply-tests", "--apply" in sys.argv)
patch.register("tests")
patch.finish()
