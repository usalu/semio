#!/usr/bin/env python3
"""Merged copy of the rcache-server set on top of cache-gc staged2: the `cache` operation's flag list gained `--deep`, so the three catalogue edits that cut `cache serve|remote` carry it."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/rcache-server-tools", "rcache-server")
patch(T + "/edits.py", '"--json", "--dry-run", "--area", "--tasks"]', '"--json", "--dry-run", "--area", "--deep", "--tasks"]', count=3)
print("rcache-server merged copy:", T)
