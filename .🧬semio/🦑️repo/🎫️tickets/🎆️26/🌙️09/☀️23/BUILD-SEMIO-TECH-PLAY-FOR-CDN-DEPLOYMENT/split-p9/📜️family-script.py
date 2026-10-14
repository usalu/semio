#!/usr/bin/env python3
"""📜️ Points an artifact's `📜️script.ts` router at its whole crate family (`packages` = every family package, aggregator
last), so `check`/`test` cover the family as Lane C's semio script does.

usage: 📜️family-script.py <config.json>
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
cfg = json.load(open(sys.argv[1], encoding="utf-8"))
script = os.path.join(os.path.dirname(os.path.join(REPO, cfg["root"])), "📦️packages", "🦀️rust", "📜️script.ts")
text = open(script, encoding="utf-8").read()
packages = sorted(c["package"] for k, c in cfg["crates"].items() if k != "agg") + [cfg["crates"]["agg"]["package"]]
listed = "packages:[" + ",".join(json.dumps(p) for p in packages) + "]"
m = re.search(r'runArtifactRustPackageMain\(([^,()]+),\s*([^,()]+?)\s*(?:,\s*\{(.*?)\})?\s*\);', text, re.S)
body = (m.group(3) or "").strip()
body = re.sub(r"packages:\[[^\]]*\],?", "", body).strip().strip(",")
new = f'runArtifactRustPackageMain({m.group(1).strip()}, {m.group(2).strip()}, {{{listed}{"," + body if body else ""}}});'
open(script, "w", encoding="utf-8").write(text[:m.start()] + new + text[m.end():])
print(new[:300])
