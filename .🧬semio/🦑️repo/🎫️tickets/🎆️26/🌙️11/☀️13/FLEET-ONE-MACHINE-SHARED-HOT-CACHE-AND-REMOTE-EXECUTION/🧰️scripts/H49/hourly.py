import json,datetime,collections,re,sys
path=sys.argv[1] if len(sys.argv)>1 else '/Users/ueli/Documents/semio/.cache/b/p/m/fleet/cache-server/fleet/divergences.json'
since=sys.argv[2] if len(sys.argv)>2 else '2026-11-27T00:00'
d=json.load(open(path))
tz=datetime.timezone(datetime.timedelta(hours=1))
start=datetime.datetime.fromisoformat(since).replace(tzinfo=tz)
def kind(diff):
    def k(p):
        if p.startswith('⟨'): return 'stream'
        n=p.split('/')[-1]
        if re.search(r'\.rmeta(\.|$)',n): return 'rmeta'
        if n.endswith('.rlib'): return 'rlib'
        if n.endswith('.d'): return 'dep-info'
        if re.search(r'\.(dylib|so|dll)$',n): return 'dylib'
        return 'other' if '.' in n else 'binary'
    return '+'.join(sorted(set(map(k,diff)))) or 'none'
hours=collections.defaultdict(collections.Counter)
for r in d['rows']:
    t=datetime.datetime.fromtimestamp(r['atMs']/1000,tz)
    if t>=start: hours[t.strftime('%m-%d %H:00 CET')][kind(r['differing'])]+=1
for h in sorted(hours): print(h,sum(hours[h].values()),dict(hours[h]))
print('lifetime total',d['total'])
