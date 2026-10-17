#!/usr/bin/env python3
"""🔑️ Multiset difference of two stored oracle key files: keydiff.py <tag> <config> (A = pi2-oracle-<tag>-full-<cfg>, B = ...-fullB-<cfg>)"""
import json, sys, collections
from lib import *
tag, cfg = sys.argv[1], sys.argv[2]
a = collections.Counter({tuple(k): v for k, v in json.load(open(f"{GEN}/pi2-oracle-{tag}-full-{cfg}.keys.json"))})
b = collections.Counter({tuple(k): v for k, v in json.load(open(f"{GEN}/pi2-oracle-{tag}-fullB-{cfg}.keys.json"))})
for k, v in sorted((a - b).items(), key=lambda kv: -kv[1]):
    print("+new", v, k[0], k[1][:200], "|", k[2][:60])
for k, v in sorted((b - a).items(), key=lambda kv: -kv[1]):
    print("-old", v, k[0], k[1][:200], "|", k[2][:60])
