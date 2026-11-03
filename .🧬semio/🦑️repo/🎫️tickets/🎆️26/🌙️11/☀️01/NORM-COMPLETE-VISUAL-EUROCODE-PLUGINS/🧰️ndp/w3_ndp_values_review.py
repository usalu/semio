import sys, os, re, json
sys.path.insert(0, os.path.dirname(__file__))
import w3_ndp_values_scan as s
SP = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "📐️specs")
cache = {}
def spec(ref):
    m = re.match(r"([\w.\-]+\.md):(\d+)", ref)
    if not m: return ""
    f = m.group(1)
    if f not in cache: cache[f] = open(os.path.join(SP, f), encoding="utf-8").read().split("\n")
    n = int(m.group(2)); return cache[f][n - 1].strip() if 0 < n <= len(cache[f]) else ""
code = sys.argv[1]
reads, _ = s.scan(code)
for i in sorted(reads):
    ids = s.resolve(i)
    if not ids: continue
    for k in ids:
        e = s.E[k]
        if e["recommended"] is None or e["naDe"] is None:
            if len(ids) > 1 and k != ids[0]: continue
            print(f"{k} | en={e['recommended']} de={e['naDe']} | {e['clause']} | {e['symbol'][:40]} | AW: {e['asWritten'][:160]} | SP: {spec(e['specRef'])[:260]}")
