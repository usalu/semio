"""⚖️ Compares two runs of `site-loading-findings.ts` (previous and current `hypot`) document by document: digests, the dumped findings themselves, and CPU time.
Usage: python3 site-loading-findings-compare.py <previous.jsonl> <current.jsonl> <previous.json> <current.json>"""
import json, sys
def rows(path): return [json.loads(line) for line in open(path) if line.startswith("{")]
previous, current = rows(sys.argv[1]), rows(sys.argv[2])
assert [row["document"] for row in previous] == [row["document"] for row in current]
print("| document | findings | sha256 (first 12) | identical | CPU ms previous | CPU ms current |")
print("|---|---:|---|---|---:|---:|")
for a, b in zip(previous, current):
    print(f'| {a["document"]} | {a["findings"]} | {a["sha256"][:12]} | {"yes" if a["sha256"] == b["sha256"] and a["findings"] == b["findings"] else "NO"} | {a["cpuMs"]} | {b["cpuMs"]} |')
same = sum(a["sha256"] == b["sha256"] for a, b in zip(previous, current))
dumps = json.load(open(sys.argv[3])) == json.load(open(sys.argv[4]))
print(f'\n{same} of {len(previous)} documents have identical findings digests; dumped findings deep-equal: {dumps}; findings compared: {sum(a["findings"] for a in previous)}; CPU previous {sum(a["cpuMs"] for a in previous)} ms, current {sum(b["cpuMs"] for b in current)} ms.')
