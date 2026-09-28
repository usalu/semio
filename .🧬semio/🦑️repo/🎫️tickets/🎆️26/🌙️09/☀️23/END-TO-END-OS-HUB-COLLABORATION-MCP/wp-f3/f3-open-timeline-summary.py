#!/usr/bin/env python3
"""⏱️ F3 — phase summary of an `open-timeline-<tag>.json` (f3-open-timeline.ts): per open, the first ms of each stage
(install band, window, execution-target stage) and the heaviest resources. usage: python3 f3-open-timeline-summary.py <tag>…"""
import json, re, sys
for tag in sys.argv[1:]:
    d = json.load(open(f"/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated/open-timeline-{tag}.json"))
    for k in ("first", "warm"):
        o = d.get(k)
        if not o:
            print(tag, k, "missing")
            continue
        print(f"== {tag} {k} mounted {o['mountedMs']} ms, {o['resourceTotals']}")
        prev = None
        for e in o["events"]:
            stage = " ; ".join(re.sub(r"install \S+", "install", x) for x in re.findall(r"(?:install|target) \S+(?: \d+/\d+)?|windows \S*", e["what"]))
            key = re.sub(r" \d+/\d+", "", stage)
            if key != prev:
                print(f"   {e['ms']:>7} {stage[:160]}")
                prev = key
        for r in o["resources"][:5]:
            print(f"   R start {r['start']} {r['ms']} ms {r['transfer'] / 2**20:.1f} MB {r['name'][:100]}")
