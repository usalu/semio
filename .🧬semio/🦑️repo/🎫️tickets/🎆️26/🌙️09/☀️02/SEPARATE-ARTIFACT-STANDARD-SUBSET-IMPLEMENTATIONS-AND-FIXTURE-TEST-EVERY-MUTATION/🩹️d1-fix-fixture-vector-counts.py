import pickle, json

candidates = pickle.load(open('/tmp/d1_candidates.pkl','rb'))
fixed = []
for f, art, oid in candidates:
    d = json.load(open(f))
    oracle = next(o for o in d['oracles'] if o['id'] == oid)
    manifest = next(m for m in d['mutationManifests'] if m.get('artifact') == art)
    n = len(manifest['mutations'])
    evidence = oracle.get('nativeSecondImplementation')
    assert evidence is not None, (f, oid)
    old = evidence['fixtureCoverage']['vectors']
    evidence['fixtureCoverage']['vectors'] = n
    if old != n:
        fixed.append((art, oid, old, n))
    with open(f, 'w', encoding='utf-8') as fh:
        json.dump(d, fh, indent=2, ensure_ascii=False)
        fh.write("\n")

print(f"corrected {len(fixed)} of {len(candidates)}")
for row in fixed:
    print(row)
