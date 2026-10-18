#!/usr/bin/env python3
"""📋 plugfix report: prints the real findings of sweep.py jsonl files, deduplicated per crate and config. usage: report.py [--suspect] file..."""
import json, sys, collections
args = sys.argv[1:]
show = "real"
if args and args[0] == "--suspect":
    show = "suspect"
    args = args[1:]
for f in args:
    for l in open(f, encoding="utf-8"):
        r = json.loads(l)
        s = r["mdir"].replace("✏️s/🔌️plugins/", "").replace("/📦️packages/🦀️rust", "")
        seen = collections.OrderedDict()
        for c in ("lib", "test"):
            for code, msg, where in r.get(c, {}).get(show, []):
                seen.setdefault((code, msg, where.split(":")[0].split("/")[-2] + "/" + where), []).append(c)
        if seen:
            print("##", s)
            for (code, msg, w), cs in seen.items():
                print("  ", ",".join(cs), code, msg[:110], "@", w.split("/", 1)[1][-90:])
