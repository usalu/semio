#!/usr/bin/env python3
"""🧹 Staged patch for cache-gc, tests only: the runner and command test sets that expect the runner to refuse a file where a container is expected and the command to refuse empty --area items, and their registration. The production edits themselves live in the cache-gc staged sets (cache-gc-tools staged3/gc-runner.ts, staged4/gc-command.ts, staged4/ledger-texts.json) and must be applied first: this patch refuses until they are. staged/fixes/cache-gc.json only drives the overlay verification. Dry run unless --apply."""
import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from engine import Patch

patch = Patch("apply-cache-gc", "--apply" in sys.argv)
patch.require_edited("cache-gc")
patch.copy_sets("cache-gc", False)
patch.register("cache-gc")
patch.finish()
