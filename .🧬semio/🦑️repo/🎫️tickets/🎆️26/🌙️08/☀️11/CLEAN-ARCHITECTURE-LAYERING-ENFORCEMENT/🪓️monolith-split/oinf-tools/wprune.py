"""Collect unused-import names from the stitched oracle runs (must be unused in every config)."""
import json, re, sys, os, collections
SCR = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/oinf"
def unused(tag):
    out = collections.defaultdict(set)
    for ln in open(f"{SCR}/err-sb-w2-{tag}.json"):
        if not ln.startswith('{'): continue
        d = json.loads(ln)
        if d.get('level') != 'warning' or not d['message'].startswith('unused import'): continue
        sp = [s for s in d['spans'] if s['is_primary']]
        if not sp: continue
        f = sp[0]['file_name']
        if '/🌍️world/' not in f: continue
        key = f.split('/🌍️world/')[1]
        names = re.findall(r'`([^`]+)`', d['message'])
        for n in names: out[key].add(n)
    return out
def main(tags):
    runs = [unused(t) for t in tags]
    res = {}
    for f in set().union(*[set(r) for r in runs]):
        sets = [r.get(f, set()) for r in runs]
        inter = set.intersection(*sets)
        if inter: res[f] = sorted(inter)
    return res
if __name__ == '__main__':
    res = main(sys.argv[1:])
    json.dump(res, open(SCR + '/prune.json', 'w'), ensure_ascii=False, indent=1)
    for f, n in res.items(): print(f, len(n), n[:8])
