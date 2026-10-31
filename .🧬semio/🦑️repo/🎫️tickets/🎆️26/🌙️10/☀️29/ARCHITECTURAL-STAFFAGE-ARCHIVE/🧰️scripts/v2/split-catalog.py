"""Splits the v1 category documents into one v2 asset document per asset and reclassifies furniture and machinery (one-off restructure)."""
import json, os, sys
S = sys.argv[1]
C = os.path.join(S, '🗂️catalog')
FOLDERS = {'humans': '👤️humans', 'animals': '🐕️animals', 'vegetation': '🌳️vegetation', 'furniture': '🛋️furniture', 'objects': '🪑️objects', 'vehicles': '🚜️vehicles', 'machinery': '🏗️machinery', 'components': '🧱️components'}
MOVE = {'park-bench': 'furniture', 'cafe-chair': 'furniture', 'office-swivel-chair': 'furniture', 'height-adjustable-desk': 'furniture', 'excavator': 'machinery', 'tower-crane': 'machinery', 'forklift': 'machinery'}
for old in ['👤️humans', '🐕️animals', '🌳️vegetation', '🪑️objects', '🚜️vehicles', '🧱️components']:
    path = os.path.join(C, old, '🔣️.json')
    doc = json.load(open(path, encoding='utf-8'))
    for a in doc['assets']:
        a['category'] = MOVE.get(a['id'], a['category'])
        d = os.path.join(C, FOLDERS[a['category']], a['id'])
        os.makedirs(d, exist_ok=True)
        out = {'$schema': '../../../🧬️schema/🔣️.json#/$defs/AssetDocument', 'version': 'semio.staffage/v2', 'asset': a}
        with open(os.path.join(d, '🔣️.json'), 'w', encoding='utf-8') as f:
            json.dump(out, f, ensure_ascii=False, indent=2); f.write('\n')
    os.remove(path)
m = json.load(open(os.path.join(C, '🔣️.json'), encoding='utf-8'))
for e in m['entries']:
    e['category'] = MOVE.get(e['id'], e['category'])
m['version'] = 'semio.staffage/v2'
TITLES = {'furniture': ({'en': 'Furniture', 'de': 'Möbel'}, {'en': 'Seating, tables, storage and beds for interiors and public space.', 'de': 'Sitzmöbel, Tische, Stauraum und Betten für Innenräume und öffentlichen Raum.'}),
          'machinery': ({'en': 'Machinery', 'de': 'Maschinen'}, {'en': 'Construction and building-services machines with operating states.', 'de': 'Bau- und Haustechnikmaschinen mit Betriebszuständen.'})}
cats = {c['id']: c for c in m['categories']}
new = []
for cid in ['humans', 'animals', 'vegetation', 'furniture', 'objects', 'vehicles', 'machinery', 'components']:
    c = cats.get(cid) or {'id': cid, 'emoji': FOLDERS[cid].split('️')[0], 'title': TITLES[cid][0], 'description': TITLES[cid][1]}
    c.pop('file', None); c['folder'] = FOLDERS[cid]
    new.append(c)
m['categories'] = new
json.dump(m, open(os.path.join(C, '🔣️.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=2)
print('ok', sum(1 for _ in m['entries']))
