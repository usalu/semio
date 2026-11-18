"""Silhouette repairs round 4 (sliding-door 1:500: the plan is the section A-A at 1.0 m, so the head track seen from above gives way to the two jamb cuts of the volume, 0.15 x 0.1 m each)."""
import importlib.util, json, re
spec = importlib.util.spec_from_file_location('s', 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
d = s.Doc('🧱️components/sliding-door')
plan = s.walk(d.root, ['asset', 'representations', '500', 'geometry', 'plan'])
node = plan['items'][0]
text = d.t[node['s']:node['e']]
assert json.loads(text)['id'] == 'portal'
indent = re.search(r'\n([ ]*)$', d.t[:node['s']]).group(1)
def jamb(id_, x):
    t = text.replace('"id": "portal"', '"id": "%s"' % id_)
    t = re.sub(r'"regions": \[[^\]]*\]', '"regions": ["jamb-steel"]', t)
    t = re.sub(r'"x": -0.05', '"x": %s' % x, t)
    t = re.sub(r'"w": 3.1', '"w": 0.15', t)
    return t
d.rep(node, jamb('jamb-west', '-0.05') + ',\n' + indent + jamb('jamb-east', '2.9'))
d.commit()
d = s.Doc('🧱️components/sliding-door')
plan = s.walk(d.root, ['asset', 'representations', '500', 'geometry', 'plan'])
leaf = [n for n in plan['items'] if json.loads(d.t[s.child(n, 'id')['s']:s.child(n, 'id')['e']]) == 'leaf'][0]
d.rep(s.child(leaf, 'y'), '-0.0375'); d.rep(s.child(leaf, 'h'), '0.075')
d.commit()
