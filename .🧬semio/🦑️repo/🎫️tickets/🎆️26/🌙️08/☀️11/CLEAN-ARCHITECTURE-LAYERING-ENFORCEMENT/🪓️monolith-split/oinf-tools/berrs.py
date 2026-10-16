"""Show new errors (not in baseline) with source locations: python3 berrs.py <tag> <cfg> [pattern]"""
import sys, json, re, collections
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
tag, cfg = sys.argv[1], sys.argv[2]
pat = sys.argv[3] if len(sys.argv) > 3 else None
import woracle
woracle.EXTFILTER = True
def load(path):
    out = []
    for ln in open(path, encoding='utf-8', errors='replace'):
        if not ln.startswith('{'): continue
        try: d = json.loads(ln)
        except Exception: continue
        if d.get('level') != 'error': continue
        out.append(d)
    return out
def key(d):
    spans = [s for s in d.get('spans', []) if s.get('is_primary')]
    txt = lab = ''
    if spans:
        s = spans[0]; txt = ' '.join(' '.join(t['text'] for t in s.get('text', [])).split()); lab = s.get('label') or ''
    code = (d.get('code') or {}).get('code') or ''
    msg = re.sub(r'\s+', ' ', d['message']); msg = re.sub(re.escape(SC) + r'/sb-[a-z0-9]+/', 'ROOT/', msg)
    return (code, msg, '', lab)
base = collections.Counter(key(d) for d in load(f"{SC}/err-sb-baseB-{cfg}.json"))
new = load(f"{SC}/err-sb-{tag}-{cfg}.json")
cnt = collections.Counter(key(d) for d in new)
seen = collections.Counter()
for d in new:
    k = key(d)
    if cnt[k] <= base.get(k, 0): continue
    if pat and not re.search(pat, k[1]): continue
    sp = [s for s in d.get('spans', []) if s.get('is_primary')]
    loc = f"{sp[0]['file_name'].split('/♾️infinite/')[-1].replace('📦️packages/🦀️rust/../../','')[-70:]}:{sp[0]['line_start']}" if sp else '?'
    seen[k] += 1
    if seen[k] <= 3: print(k[0], k[1][:100], '|', loc)
