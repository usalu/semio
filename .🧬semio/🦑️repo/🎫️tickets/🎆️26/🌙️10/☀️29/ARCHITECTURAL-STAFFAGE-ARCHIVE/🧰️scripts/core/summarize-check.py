import collections, json, sys
T = sys.argv[1]
data = json.load(open(f"{T}/🗑️generated/core/check-v2.json"))
codes = collections.defaultdict(list)
for r in data:
    for code in r.get("counts", {}):
        codes[code].append(r["asset"])
out = [f"\n## Summary\n\n{len(data)} assets, {sum(1 for r in data if r['ok'])} passing, {sum(1 for r in data if not r['ok'])} failing.\n"]
out.append("| code | assets | which |\n|---|---|---|")
for code, assets in sorted(codes.items(), key=lambda kv: -len(kv[1])):
    out.append(f"| {code} | {len(assets)} | {', '.join(assets) if len(assets) <= 12 else ', '.join(assets[:12]) + ', …'} |")
out.append("\n## Measurement\n\nGap = distance between the visible outline centrelines of two different shapes minus half of each pen; outlines that touch, cross or whose inked strokes overlap are joins (gap ≤ 0) and are never reported. A gap is reported only for 0 < gap < min gap. `negative-space` = white area inside the inked silhouette (fills plus pen-wide strokes) thinner than the min gap and larger than 3·w² (raster pitch ≤ min gap / 6 = 0.025 mm; its width is quantised to that pitch, so `0.025` means ‘about one raster pixel’). Feature = thickness of a filled shape (morphological opening). All measured over every named pose.\n")
out.append("## Sample violations with positions (paper mm from the model origin; x right, y down)\n")
for r in data:
    lines = []
    for s in r.get("scales", []):
        for v in (s.get("violations") or [])[:3]:
            lines.append(f"1:{s['d']} {v}")
    if lines:
        out.append(f"- **{r['asset']}**: " + "; ".join(lines[:6]))
open(f"{T}/check-v2-results.md", "a").write("\n".join(out) + "\n")
