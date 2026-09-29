"""🧯️ S20: static census of every refusal code the app crates can raise (coordinator decision 2026-09-29 08:4x) — per
plugin: distinct literal `FaultCode::new("…")` codes, `Fault::from(…)` sites (they answer the untyped `app.message`),
and computed codes (`FaultCode::new(<expr>)`), non-test sources only. Usage: python3 s20-fault-census.py <out.json>
"""
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PLUGINS = ROOT / "✏️s/🔌️plugins"
LITERAL = re.compile(r'FaultCode::new\(\s*"([^"]+)"\s*\)')
COMPUTED = re.compile(r"FaultCode::new\(\s*(?!\")")
FROM = re.compile(r"\bFault::from\(")
census: dict[str, dict] = defaultdict(lambda: {"codes": defaultdict(int), "faultFromSites": 0, "computedCodeSites": 0, "files": 0})
for path in PLUGINS.rglob("*.rs"):
    rel = path.relative_to(PLUGINS)
    if "🧪️tests" in rel.parts or "target" in rel.parts:
        continue
    text = path.read_text(errors="replace")
    text = re.sub(r"#\[cfg\(test\)\][^\n]*\n\s*mod [^;{]+;", "", text)
    plugin = rel.parts[0]
    entry = census[plugin]
    literal = LITERAL.findall(text)
    computed = len(COMPUTED.findall(text))
    froms = len(FROM.findall(text))
    if literal or computed or froms:
        entry["files"] += 1
    for code in literal:
        entry["codes"][code] += 1
    entry["faultFromSites"] += froms
    entry["computedCodeSites"] += computed
out = {plugin: {"distinctCodes": len(data["codes"]), "codes": dict(sorted(data["codes"].items())), "faultFromSites": data["faultFromSites"], "computedCodeSites": data["computedCodeSites"], "files": data["files"]} for plugin, data in sorted(census.items())}
totals = {"plugins": len(out), "distinctCodes": len({code for data in out.values() for code in data["codes"]}), "faultFromSites": sum(d["faultFromSites"] for d in out.values()), "computedCodeSites": sum(d["computedCodeSites"] for d in out.values())}
Path(sys.argv[1]).write_text(json.dumps({"totals": totals, "plugins": out}, ensure_ascii=False, indent=1))
print(json.dumps(totals))
for plugin, data in out.items():
    print(f"{plugin}: codes={data['distinctCodes']} fromSites={data['faultFromSites']} computed={data['computedCodeSites']}")
