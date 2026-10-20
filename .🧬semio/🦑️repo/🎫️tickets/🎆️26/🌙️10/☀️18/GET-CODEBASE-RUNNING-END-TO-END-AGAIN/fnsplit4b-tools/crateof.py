#!/usr/bin/env python3
"""📦 Finds the crate manifest owning a Rust source file. usage: crateof.py <file.rs>... -> repo-relative Cargo.toml paths (one per line)"""
import os, sys
ROOT = "/Users/ueli/Documents/semio/"
for f in sys.argv[1:]:
    d = os.path.dirname(os.path.abspath(f))
    found = None
    while d.startswith(ROOT.rstrip("/")):
        for cand in (os.path.join(d, "📦️packages", "🦀️rust", "Cargo.toml"), os.path.join(d, "Cargo.toml")):
            if os.path.exists(cand):
                found = cand; break
        if found: break
        d = os.path.dirname(d)
    print(os.path.relpath(found, ROOT) if found else "?")
