"""🖨️ Draw the real hardware, canvas and carpentry at each print band's pen width."""
from pathlib import Path
from runpy import run_path
import json,sys
T=Path(__file__).resolve().parent
catalog=next(T.parents[6].glob('🎓*/*/*staffage/🗂*catalog'))
def author(a):
    slug=a['id']
    for d,r in a['representations'].items():
        if r['status']!='authored':continue
        for view,shapes in r['geometry'].items():
            if slug=='folding-deck-chair' and d=='100':
                shapes[:]=[p for p in shapes if not ('-stripe-' in p['id'] and p['id'].endswith(('-1','-3')))]
            for p in shapes:
                id=p['id']
                if slug=='vacuum-cleaner' and d=='50' and id=='hose-end':p['penMm']=.20
                if slug=='chalkboard':
                    if d=='50' and (id.startswith('frame-corner-') or id in ('end-cleat','end-hanger','end-buffer','hanger-a','hanger-b')):p['penMm']=.20
                    if d in ('200','500') and id in ('end-frame','end-leaf','top-tray'):p['penMm']=.20
                if slug=='retractable-bollard' and d=='50':
                    if id in ('seal-l','seal-r','frame-bolt-l','frame-bolt-r','unit-lid-bolt-1','cable-gland'):p['penMm']=.13
                if slug=='hot-water-cylinder' and d=='500' and id=='feet':p['penMm']=.20
                if slug=='cargo-bike' and id in {
                    '50':{'chainstay-bridge','crate-upper','cup-r-top','stand-cross','stand-foot-l','stand-foot-r'},
                    '100':{'box-floor','box-rail-3','crate-upper','steerer-cap-f'},
                    '200':{'box-floor','box-rear','box-side-l','box-side-r','crate-lower','pedal-f'}
                }.get(d,()):p['penMm']=.20
                if slug=='city-bicycle' and id in {
                    '50':{'axle-r','hub-r','pedal-block-f-1','pedal-block-f-2','pedal-block-n-1','pedal-plate-n-1','tail-lamp','tail-lamp-lens'},
                    '100':{'fender-f','rack-hoop','tail-lamp','stand-pivot','hub-f','hub-r'},'200':{'rack-hoop','rack-leg-l','rack-leg-r'}
                }.get(d,()):p['penMm']=.20
                if slug=='panel-radiator' and view=='front':
                    if d=='100' and id in ('pipe-left','pipe-right'):
                        p.update(penMm=.1,x=-.4765 if id=='pipe-left' else -.4185)
                    if d=='50' and id.startswith('connection-nut-'):
                        p['penMm']=.1
                        if 'outer' in id or 'inner' in id:
                            offset=-.002 if 'outer' in id else .002
                            if 'x' in p:p['x']+=offset
                            if 'points' in p:p['points']=[[round(x+offset,7),y] for x,y in p['points']]
                if slug=='panel-radiator' and view=='elevation' and d=='50' and id=='connection-nut-upper':
                    p['penMm']=.10
                    p['points']=[[x,round(-.05+(y+.05)*.76,7)] for x,y in p['points']]
                if slug=='shopping-trolley' and d=='500' and id=='gate':p['penMm']=.20
                if slug=='sorption-wheel' and d=='100' and id in ('foot-left','foot-right'):p['penMm']=.20
                if slug=='city-car' and d=='100' and id=='exhaust-pipe':p['penMm']=.20
                if slug=='external-venetian-blind' and d=='200' and id in ('guide','bottom-rail'):p['penMm']=.20
                if slug=='bookshelf' and id.startswith('pin-') and d=='50':p['penMm']=.20
                if slug=='electricity-meter' and id in ('mark','eye','screw-terminal','disc','rim'):p['penMm']=.20
                if slug=='thermostatic-radiator-valve' and id in ('pointer','index'):p['penMm']=.20
                if slug=='operating-table' and id in ('head-lens','head-rod-l','head-rod-r','head-clamp','castor-foot-r-b','castor-foot-l-b','castor-head-r-b','castor-head-l-b','frame-leg-r','frame-leg-l','head-plate','stand-castor-near'):p['penMm']=.20
                if slug=='park-bench' and d=='100' and id in ('back-slat-2','back-slat-3'):
                    p['penMm']=.20 if view=='plan' else .15
                    if view=='elevation':
                        p['h']=.071
                        if id=='back-slat-3':p['y']=.0996
                    if view=='front':
                        high=sorted(range(4),key=lambda i:p['points'][i][1])
                        for k in (high[2:] if id=='back-slat-2' else high[:2]):
                            p['points'][k][1]+=(-.0131 if id=='back-slat-2' else .0131)
                if slug=='park-bench' and d=='200' and view=='elevation' and id.startswith('back-board-'):
                    p['penMm']=.15
                    if id=='back-board-1':p['h']=.071
                    if id=='back-board-2':p['y']=-.0113;p['h']=.1819
                if slug=='bar-stool' and d=='200' and id in ('leg-r','leg-l','leg-m'):p['penMm']=.13
                if slug=='folding-deck-chair' and id.startswith('canvas-'):
                    p['penMm']=.2
if __name__=='__main__':
    for slug in sys.argv[1:]:
        path=next(catalog.glob('*/'+slug+'/🔣️.json'));doc=json.loads(path.read_text());author(doc['asset'])
        (T/'🗑️generated'/('digital-print-'+slug+'.json')).write_text(json.dumps(doc,ensure_ascii=False))
