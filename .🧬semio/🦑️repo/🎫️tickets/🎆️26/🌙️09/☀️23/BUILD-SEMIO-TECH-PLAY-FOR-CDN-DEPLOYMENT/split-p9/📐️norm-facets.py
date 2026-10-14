#!/usr/bin/env python3
"""📐️ Per norm artifact: lines/bytes/files of each taxonomy facet of the native lib closure (from 📏️unit-files.ts --files)."""
import sys, glob, re, collections, os
FACETS = [("root", r"^[^/]+/🦀️\.rs$"), ("schema", r"🧬️schema/[^/]+$"), ("snapshot", r"🧬️schema/📸️snapshot/"), ("diff", r"🧬️schema/🔺️diff/"),
          ("mutroot", r"🧬️schema/🧬️mutations/([^/]+|🧭️edit-rules/.*)$"), ("mutkinds", r"🧬️schema/🧬️mutations/"), ("infer", r"🧬️schema/💡️inferences/"),
          ("io", r"🚪️io/"), ("editor", r"✏️editor/"), ("viewer", r"👁️viewer/"), ("examples", r"(📚️examples|🖼️assets)/"), ("other", r".")]
print(f"{'artifact':12} " + " ".join(f"{n:>17}" for n, _ in FACETS) + "  kinds")
for path in sorted(glob.glob(sys.argv[1] + "/norm-*-native.tsv")):
    art = re.search(r"norm-(.+)-native", path).group(1)
    roll = collections.defaultdict(lambda: [0, 0, 0]); kinds = set()
    for line in open(path, encoding="utf-8"):
        p = line.rstrip("\n").split("\t")
        if p[0] != "F": continue
        f = p[4]; m = re.search(r"🗿️artifacts/(.+)$", f); rel = m.group(1) if m else "~" + f
        rel = re.sub(r"🏅️standards/🔖️[^/]+/🪆️subsets/[^/]+/", "", rel)
        rel = rel.split("/", 1)[1] if "/" in rel else rel
        rel = art + "/" + rel if re.match(r"^🦀️\.rs$", rel) else rel
        km = re.match(r"🧬️schema/🧬️mutations/([^/]+)/", rel)
        if km and km.group(1) != "🧭️edit-rules": kinds.add(km.group(1))
        for n, r in FACETS:
            if re.search(r, rel): roll[n][0] += int(p[1]); roll[n][1] += int(p[2]); roll[n][2] += 1; break
    print(f"{art[-8:]:12} " + " ".join(f"{roll[n][0]:5}L{roll[n][1]//1024:4}K{roll[n][2]:4}F" for n, _ in FACETS) + f"  {len(kinds)}")
