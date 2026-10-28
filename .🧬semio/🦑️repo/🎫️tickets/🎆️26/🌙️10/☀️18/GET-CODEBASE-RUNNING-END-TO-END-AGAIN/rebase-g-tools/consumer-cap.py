#!/usr/bin/env python3
"""rebase-g: the graph-consumer captures of `💾️storage/🔧️custody/🟨️.mjs` admit the first consumer file up to 8 MiB and every other one up to 65536 bytes (the planner refuses a bigger file: "Bounded physical Source revision required"). This reads the consumer list from that file and prints each size; exit 1 when a file of the given root exceeds its bound. usage: consumer-cap.py [root]  (a mirror root after a batch was applied; default the live tree)"""
import os, re, sys
root = (sys.argv[1] if len(sys.argv) > 1 else "/Users/ueli/Documents/semio").rstrip("/")
custody = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🔧️custody/🟨️.mjs"
line = open(custody, encoding="utf-8").read().split("\n")[17]
segment = line[line.index("graphConsumerURLsV1=["):line.index("],graphConsumerCapturesV1")]
bad = 0
for index, url in enumerate(re.findall(r'new URL\("([^"]+)"', segment)):
    if not url.startswith("."): continue
    path = os.path.normpath(os.path.join(os.path.dirname(custody), url)); size = os.path.getsize(path); bound = 8388608 if index == 0 else 65536
    flag = "OVER" if size > bound else "near" if size > bound * 0.9 else ""
    bad += size > bound
    if flag: print("%-5s %8d / %d  %s" % (flag, size, bound, "/".join(path.split("/")[-4:])))
print("graph consumer files over their bound:", bad); sys.exit(1 if bad else 0)
