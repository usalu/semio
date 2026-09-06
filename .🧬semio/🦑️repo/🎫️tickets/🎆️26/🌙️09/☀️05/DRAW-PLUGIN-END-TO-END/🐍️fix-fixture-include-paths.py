#!/usr/bin/env python3
"""🧷 Repoint stale include_str!/include_bytes! literals under the draw plugin to the renamed
fixture directories on disk (friendly slug → hash-suffixed slug). Dry run by default; --apply writes."""
import os, re, sys
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw"
APPLY = "--apply" in sys.argv
PAT = re.compile(r'include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)')
fixed = missing = ok = 0
for dp, dn, fn in os.walk(ROOT):
    if "/target" in dp or "node_modules" in dp: continue
    for f in fn:
        if not f.endswith(".rs"): continue
        p = os.path.join(dp, f)
        src = open(p, encoding="utf-8").read()
        new = src
        for lit in set(PAT.findall(src)):
            full = os.path.normpath(os.path.join(dp, lit))
            if os.path.exists(full): ok += 1; continue
            parts = lit.split("/")
            # find the first missing segment and try a unique prefix match on disk
            cur = dp; repl = []
            for i, seg in enumerate(parts):
                cand = os.path.normpath(os.path.join(cur, seg))
                if os.path.exists(cand): repl.append(seg); cur = cand; continue
                stem = re.sub(r"-[0-9a-f]{6}$", "", seg)
                # try progressively shorter dash-prefixes of the stem
                pieces = stem.split("-")
                match = None
                for k in range(len(pieces), 0, -1):
                    pre = "-".join(pieces[:k])
                    hits = [d for d in os.listdir(cur) if d.startswith(pre + "-") or d == pre]
                    if len(hits) == 1: match = hits[0]; break
                    if len(hits) > 1: break
                if match is None: repl = None; break
                repl.append(match); cur = os.path.normpath(os.path.join(cur, match))
            if repl is None or not os.path.exists(cur):
                missing += 1; print(f"MISSING {p}:{lit}"); continue
            newlit = "/".join(repl)
            print(f"FIX {os.path.relpath(p, ROOT)}\n    {lit}\n -> {newlit}")
            new = new.replace(f'"{lit}"', f'"{newlit}"'); fixed += 1
        if APPLY and new != src: open(p, "w", encoding="utf-8").write(new)
print(f"ok={ok} fixed={fixed} missing={missing} apply={APPLY}")
