#!/usr/bin/env python3
"""Dry-run of the staged-paths edits against another root: SEMIO_ROOT=<root> try-bzl-paths.py (reads only)."""
import os, sys
sys.dont_write_bytecode = True
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(C, "bzl-hermetic-tools", "staged-paths"))
import edits
try:
    out = edits.build()
    print("ok", len(out), "edited files re-derive on", edits.ROOT, "runner at", edits.RUNNER[-30:])
except Exception as e:
    print("REFUSED", type(e).__name__, str(e)[:500])
