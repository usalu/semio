"""🫁 Keep the breath outlet surfaces on the actual nares and lips at each scale."""
import copy
import json
from pathlib import Path

catalog=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
for slug in ('pigeon','pig','goat','sheep'):
    path=next(catalog.glob(f'*/{slug}/*.json'))
    raw=path.read_text()
    a=json.loads(raw)['asset']
    v=a['volumes']
    breath=[s for s in v['50']['thermal'] if s['id'].startswith('th-breath-')]
    if slug=='pigeon':
        left=copy.deepcopy(breath[0])
        left['id']='th-breath-nostril'
        left['transform']['translation']=[0.037,0.0036,-0.0021]
        right=copy.deepcopy(left)
        right['id']='th-breath-nostril-f'
        right['transform']['translation'][2]=0.0021
        breath=[left,right]
    for scale in ('50','100','200','500'):
        thermal=v[scale]['thermal']
        first=next(i for i,s in enumerate(thermal) if s['id'].startswith('th-breath-'))
        outlets=copy.deepcopy(breath)
        for s in outlets:
            s['role']='detail'
            nose='mouth' not in s['id']
            if slug=='pigeon':
                s['radii']=[0.001,0.00055,0.00032]
                s['region']='cere'
            if slug=='pig':
                s['radii']=[0.0025,0.0065,0.008] if nose else [0.003,0.015,0.0035]
            if slug=='sheep':
                s['radii']=[0.002,0.0046,0.0055] if nose else [0.0022,0.010,0.003]
            if slug=='goat':
                s['radii']=[0.0021,0.004,0.005] if nose else [0.002,0.009,0.0028]
            if slug=='goat' and not nose:
                s['radii']=[0.002,0.0028,0.009]
            if scale!='50':
                rim={
                    '100':[(1,0),(.88,.47),(.48,.86),(0,1),(-.52,.81),(-.90,.43),(-1,0),(-.83,-.48),(-.46,-.91),(0,-1),(.51,-.81),(.89,-.42)],
                    '200':[(1,0),(.69,.70),(0,1),(-.71,.62),(-1,0),(-.68,-.69),(0,-1),(.74,-.62)],
                    '500':[(1,0),(.56,.77),(0,1),(-1,0),(0,-1),(.55,-.75)]
                }[scale]
                rx,ry,rz=s.pop('radii')
                depth,width,height=(rz,rx,ry) if slug=='pigeon' else (rx,ry,rz)
                s.update(kind='extrude',plane='xy' if slug=='pigeon' else 'yz',profile=[[round(x*width,7),round(y*height,7)] for x,y in rim])
                s['from']=-depth
                s['to']=depth
            s.pop('texture',None)
        others=[s for s in thermal if not s['id'].startswith('th-breath-')]
        v[scale]['thermal']=others[:first]+outlets+others[first:]
    start=raw.index('{',raw.index('"volumes":'))
    old,n=json.JSONDecoder().raw_decode(raw[start:])
    if old!=v:
        if path.read_text()!=raw:raise SystemExit(f'{slug} changed concurrently')
        path.write_text(raw[:start]+json.dumps(v,ensure_ascii=False,indent=2).replace('\n','\n    ')+raw[start+n:])
        print('[DEBUG]',slug)

