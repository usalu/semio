#!/usr/bin/env python3
"""⏳️ T14 blocking wait (one call ≤ 10 min): returns as soon as `<file>` contains `<text>` (or exists, text empty), else
after `<seconds>`; prints the matching tail. usage: wait-for.py <seconds> <file> [text]"""
import os
import sys
import time

limit, path, text = float(sys.argv[1]), sys.argv[2], sys.argv[3] if len(sys.argv) > 3 else ""
deadline = time.time() + limit
while time.time() < deadline:
    if os.path.exists(path):
        content = open(path, encoding="utf-8", errors="replace").read()
        if not text or text in content:
            print("MATCH", time.strftime("%H:%M:%S"))
            print(content[-1500:])
            sys.exit(0)
    time.sleep(10)
print("TIMEOUT", time.strftime("%H:%M:%S"))
if os.path.exists(path):
    print(open(path, encoding="utf-8", errors="replace").read()[-800:])
