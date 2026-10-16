import sys, os, json, collections
os.environ['SEMIO_ROOT'] = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/pwp6-snap/'
sys.path.insert(0, '.')
import s0plan as SP
P = SP.plan()
out = []
for m in P['recs']:
    out.append(dict(file=m['file'], name=m['name'], start=m['start'], a=m['a'], e=m['e'], vis=m['vis'], trait=m['trait'], lines=m['lines'], home=m['home'], home2=m['home2'], need2=m['need2'], crate=m['crate'], why=m['why'], dcomps=m['dcomps'], calls=m['calls']))
GEN = '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated'
old = {(o['name'], o['trait']): o for o in json.load(open(GEN + '/pwp6-s0-recs.json'))}
chg = [(m['name'], old[(m['name'], m['trait'])]['crate'], m['crate'], old[(m['name'], m['trait'])]['home2'], m['home2']) for m in P['recs'] if old[(m['name'], m['trait'])]['crate'] != m['crate'] or old[(m['name'], m['trait'])]['home2'] != m['home2']]
print(len(chg), 'changes vs previous recs'); [print(c) for c in chg]
json.dump(out, open(GEN + '/pwp6-s0-recs.json', 'w'), ensure_ascii=False)
tot = collections.Counter()
for m in P['recs']: tot[m['crate']] += m['lines']
print(dict(tot))
