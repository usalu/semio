#!/usr/bin/env python3
"""replay.py <slug> reverse|forward : replays a stored conversion plan (moves only, no registry)."""
import json, os, sys
G = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/gocanon-logs'
R = '/Users/ueli/Documents/semio/'
slug, way = sys.argv[1], sys.argv[2]
plan = json.load(open(f'{G}/{slug}-convert-plan.json'))
for item in plan['items']:
    a, b = R + item['from'], R + item['to']
    src, dst = (b, a) if way == 'reverse' else (a, b)
    assert os.path.exists(src) and not os.path.exists(dst), (src, dst)
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    os.rename(src, dst)
    if way == 'reverse':
        d = os.path.dirname(src)
        if not os.listdir(d):
            os.rmdir(d)
print(slug, way, len(plan['items']))
