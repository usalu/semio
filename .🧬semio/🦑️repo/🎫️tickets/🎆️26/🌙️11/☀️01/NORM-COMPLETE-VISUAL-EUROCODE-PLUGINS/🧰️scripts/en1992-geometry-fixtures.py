import json, pathlib
A = pathlib.Path('/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any')
MS = ['placementX', 'placementY', 'placementZ', 'placementYaw']
AS = ['headDiameter'] + MS
def walk(o, full):
    if isinstance(o, dict):
        if 'labelEn' in o and 'relativeHumidity' in o and 'placementX' not in o:
            for k in MS: o[k] = 0.5 if full else 0.0
            o['sectionPoints'] = [{'id': 'p1', 'y': 0.5, 'z': 0.5}] if full else []
            o['bars'] = [{'id': 'b1', 'layerId': 'l1', 'y': 0.5, 'z': 0.5, 'diameter': 0.5}] if full else []
            o['tendonPoints'] = [{'id': 't1', 'x': 0.5, 'y': 0.5, 'z': 0.5}] if full else []
            o['studRings'] = [{'id': 'r1', 'radius': 0.5, 'count': 4294967295, 'diameter': 0.5}] if full else []
        if 'aggregateSize' in o and 'anchorageLength' in o and 'lapShare' not in o: o['lapShare'] = 0.5 if full else 1.0
        if 'hEf' in o and 'fCk' in o and 'headDiameter' not in o:
            for k in AS: o[k] = 0.5 if full else 0.0
        for v in o.values(): walk(v, full)
    elif isinstance(o, list):
        for v in o: walk(v, full)
n = 0
files = list((A / '🧪️tests/🧫️fixtures').rglob('*.json')) + [A / '🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json']
for p in files:
    s = p.read_text()
    if 'relativeHumidity' not in s: continue
    d = json.loads(s)
    assert json.dumps(d, indent=2, ensure_ascii=False) + '\n' == s or p.name == '🔣️.json' and 'sqlite' in str(p), p
    walk(d, 'sqlite' in str(p))
    p.write_text(json.dumps(d, indent=2, ensure_ascii=False) + '\n'); n += 1
print('fixtures', n)
