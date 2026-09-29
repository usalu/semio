"""🔎️ FH6 class review: prints BD work-list entries with their owner-scoped raise sites (census, overlay lines).
Usage: python3 fh6-show.py <from> <to> [before] [after] [--code substring]"""
import json
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
OVERLAY = ROOT / ".🧬semio/🌐hub/s14-s20-overlay-faults"
CENSUS = OVERLAY / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🤖️generated/🎯️acceptance/🧯️fault-census.json"
WORK = ROOT / ".🧬semio/🌐hub/s14-s20-sets/class/family-BD.json"
args = [a for a in sys.argv[1:] if not a.startswith("--")]
needle = sys.argv[sys.argv.index("--code") + 1] if "--code" in sys.argv else None
if needle:
    args = [a for a in args if a != needle]
lo, hi = int(args[0]), int(args[1])
before = int(args[2]) if len(args) > 2 else 6
after = int(args[3]) if len(args) > 3 else 2
census = json.loads(CENSUS.read_text())
entries = json.loads(WORK.read_text())
cache: dict[str, list[str]] = {}
for index, entry in enumerate(entries):
    if not lo <= index <= hi or (needle and needle not in entry["code"]):
        continue
    print(f"##### {index} {entry['code']} | {entry['class']} | {entry['rule']}\n    en: {entry['en']}")
    sites = [r for r in census["raises"] if r["code"] == entry["code"] and f"/{entry['owner']}/" in r["path"]]
    for site in sites[:int(__import__("os").environ.get("N","4"))]:
        lines = cache.setdefault(site["path"], (OVERLAY / site["path"]).read_text().split("\n"))
        print(f"  --- {site['path'].split('🔌️plugins/')[-1][-110:]}:{site['line']}")
        for i in range(max(0, site["line"] - 1 - before), min(len(lines), site["line"] + after)):
            print(f"  {i + 1:5d}| {lines[i].strip()[:200]}")
    if len(sites) > int(__import__("os").environ.get("N","4")):
        print(f"  (+{len(sites) - 4} more sites)")
