"""📋 Author 17: compact listing of a source (id, kind, bone, material, the few numbers that state the size) to measure a model against its reference table. Usage: python3 rework-17-list.py <slug> [scale ...]"""
import json, sys
slug = sys.argv[1]
d = json.load(open('volumes/appliances/%s.json' % slug))['volumes']


def size(s):
    k = s.get('kind')
    t = s.get('transform', {}).get('translation')
    if k == 'box':
        return 'size %s at %s' % (s['size'], t)
    if k == 'ellipsoid':
        return 'radii %s at %s' % (s['radii'], t)
    if k == 'capsule':
        return '%s -> %s r %s' % (s['p0'], s['p1'], s['r0'])
    if k == 'revolve':
        a = [p[0] for p in s['profile']]
        r = [p[1] for p in s['profile']]
        return 'axis %s o %s a %.4g..%.4g r %.4g..%.4g n%d' % (s['axis'], s['origin'], min(a), max(a), min(r), max(r), len(a))
    if k == 'extrude':
        a = [p[0] for p in s['profile']]
        b = [p[1] for p in s['profile']]
        return '%s %.4g..%.4g x %.4g..%.4g along %.4g..%.4g n%d holes %d' % (s['plane'], min(a), max(a), min(b), max(b), s['from'], s['to'], len(a), len(s.get('holes', [])))
    if k == 'loft':
        return 'loft %d sections %s .. %s rx %.4g ry %.4g' % (len(s['sections']), s['sections'][0]['at'], s['sections'][-1]['at'], max(x['rx'] for x in s['sections']), max(x['ry'] for x in s['sections']))
    if k == 'polyhedron':
        return 'POLYHEDRON %d vertices' % len(s['vertices'])
    return 'of ' + s.get('of', '?')


for sc in (sys.argv[2:] or ['50']):
    for lst in ('body', 'thermal', 'structure'):
        print('== 1:%s %s (%d)' % (sc, lst, len(d[sc][lst])))
        for s in d[sc][lst]:
            extra = ' '.join('%s=%s' % (k, s[k]) for k in ('region', 'layer', 'member', 'part', 'repeat', 'texture', 'slender') if k in s)
            print('  %-22s %-9s %-8s %-16s %s | %s' % (s['id'], s.get('kind', 'of'), s.get('bone', ''), s.get('material', ''), size(s), extra[:110]))
