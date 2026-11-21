"""🪟 Assemble the insulated glass from the room-side pane, sealed gap and weather-side pane."""
from pathlib import Path
from copy import deepcopy
from runpy import run_path
import json, math
T=Path(__file__).resolve().parent;CAT=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/external-venetian-blind/🔣️.json'))
def author(a):
    run_path(str(T/'digital-print-stock.py'))['author'](a)
    region=next(r for r in a['thermal']['regions']if r['id']=='glazing-behind')
    layers={q['id']:q for q in region['layers']};region['layers']=[layers[q] for q in ('pane-in','cavity','pane-out')]
    fine=a['volumes']['50']
    for d,v in a['volumes'].items():
        if d=='50':continue
        ids={'pane-in','edge-spacer','pane-out'}
        v['body'][:]=[s for s in v['body'] if s['id'] not in ids|{'glass'}]+[deepcopy(s) for s in fine['body']if s['id'] in ids]
        v['thermal'][:]=[s for s in v['thermal']if s['region']!='glazing-behind']+[deepcopy(s)for s in fine['thermal']if s['region']=='glazing-behind']
        for s in v['structure']:
            if s.get('of')=='glass':s['of']='pane-out'
        note={'en':'The sealed unit retains two 4 mm panes and the 16 mm cavity and edge spacer across its 1.13 × 1.18 m opening. The exterior heat field follows the weather-side pane.','de':'Die Isolierverglasung behält im 1,13 × 1,18 m großen Feld beide 4-mm-Scheiben, den 16-mm-Zwischenraum und den Randverbund. Das äußere Wärmefeld folgt der Wetterscheibe.'}
        if note not in v['decisions']:v['decisions'].append(note)
    for d,v in a['volumes'].items():
        quarter={'50':5,'100':3,'200':1,'500':0}[d]
        spacer=next(s for s in v['body'] if s['id']=='edge-spacer')
        gas=next(s for s in v['thermal'] if s['id']=='th-cavity')
        if not quarter:continue
        radius=.006
        ring=[]
        for y,z,angle in [(2.203-radius,.553-radius,0),(1.047+radius,.553-radius,90),(1.047+radius,-.553+radius,180),(2.203-radius,-.553+radius,270)]:
            for j in range(quarter+1):
                theta=math.radians(angle+90*j/quarter)
                ring.append([round(y+radius*math.cos(theta),7),round(z+radius*math.sin(theta),7)])
        spacer['holes']=[list(reversed(ring))]
        for key in ('size','transform'):gas.pop(key,None)
        gas.update(kind='extrude',profile=ring,plane='yz',**{'from':spacer['from'],'to':spacer['to']})
    master=next(s for s in fine['body'] if s['id']=='edge-spacer')
    for v in a['volumes'].values():
        next(s for s in v['body'] if s['id']=='edge-spacer')['holes']=deepcopy(master['holes'])
if __name__=='__main__':
    doc=json.loads(CAT.read_text());author(doc['asset']);(T/'🗑️generated/digital-blind-unit.json').write_text(json.dumps(doc,ensure_ascii=False))
