"""Silhouette repairs round 3 (excavator 1:500: the slew ring is drawn again, as role body so that it is a pen-bearing shape: its 0.14 m closes the gap between the undercarriage and the house that the volume has)."""
import importlib.util, json, re
spec = importlib.util.spec_from_file_location('s', 'sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
slug = '🏗️machinery/excavator'
h = s.head_text(slug); hr = s.parse(h)
d = s.Doc(slug)
for view in ('elevation', 'front'):
    hn = s.walk(hr, ['asset', 'representations', '500', 'geometry', view])
    first = hn['items'][0]
    text = h[first['s']:first['e']]
    assert json.loads(text)['id'] == 'slew-ring'
    text = text.replace('"role": "detail"', '"role": "body"')
    ln = s.walk(d.root, ['asset', 'representations', '500', 'geometry', view])
    f = ln['items'][0]
    before = d.t[:f['s']]
    indent = re.search(r'\n([ ]*)$', before).group(1)
    d.rep({'s': f['s'], 'e': f['s']}, text + ',\n' + indent)
d.commit()
