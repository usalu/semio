"""📐 Draws the five real blade planforms, support sections and the edge-on impeller section independently from the printed line weight."""
from pathlib import Path
from runpy import run_path
import math,json
t=Path(__file__).resolve().parent
def draw(a):
    hrv=a['id'].startswith('decentralised')
    fine=a['representations']['50']['geometry']
    poly=lambda pts:'M '+' L '.join(f'{x:.6f} {y:.6f}' for x,y in pts)+' Z'
    def apply(q,x,y,z):
        qx,qy,qz,qw=q
        return [x*(1-2*qy*qy-2*qz*qz)+y*(2*qx*qy-2*qz*qw)+z*(2*qx*qz+2*qy*qw),x*(2*qx*qy+2*qz*qw)+y*(1-2*qx*qx-2*qz*qz)+z*(2*qy*qz-2*qx*qw),x*(2*qx*qz-2*qy*qw)+y*(2*qy*qz+2*qx*qw)+z*(1-2*qx*qx-2*qy*qy)]
    for d in ['100','200']:
        rep=a['representations'][d];geom=rep['geometry'];b={q['id']:q for q in a['volumes'][d]['body']};older={q['id']:q for q in fine['elevation']}
        keep=[q for q in fine['elevation'] if q['id'] in (['plate','ring','opening','core-disc'] if hrv else ['plate','ring','opening'])]
        if hrv and d=='100':keep.extend(q for q in geom['elevation'] if q['id']=='core-cells')
        for q in fine['elevation']:
            if q['id'].startswith('strut-'):keep.append({**q,'penMm':.2})
        sections=b['blade-1']['sections']
        pts=[]
        for k in [1,-1]:
            for s in (sections if k==1 else reversed(sections)):
                xyz=apply(s['rotation'],k*s['rx'],0,0)
                pts.append((s['at'][0]+xyz[0],s['at'][1]+xyz[1]))
        for i in range(5):
            z=i*math.tau/5;c=math.cos(z);sn=math.sin(z)
            keep.append({**older['blade-'+str(i+1)],'d':poly([(x*c-y*sn,x*sn+y*c) for x,y in pts]),'penMm':.2})
        keep.append(older['hub']);geom['elevation']=keep
        for v in ['front','plan']:
            shapes=geom[v]
            for q in shapes:
                if q['id']=='plate':
                    if hrv:q.update(**({'x':-.1075,'w':.0075} if v=='front' else {'y':-.1075,'h':.0075}))
                    else:q.update(**({'x':-.045,'w':.012} if v=='front' else {'y':-.045,'h':.012}))
                    q['penMm']=.2
                if q['id']=='ring' and not hrv:q.update(**({'x':-.033,'w':.078} if v=='front' else {'y':-.033,'h':.078}))
            if hrv:
                base=next(q for q in shapes if q['id']=='plate')
                shapes.append({**base,'id':'tray-rim',**({'x':-.12,'w':.0125} if v=='front' else {'y':-.12,'h':.0125})})
            elif not any(q['id']=='lip' for q in shapes):shapes.append({**next(q for q in fine[v] if q['id']=='lip'),'penMm':.2})
            def rect(id,bone,reg,mat,x,y,w,h):
                shapes.append({'id':id,'bone':bone,'role':'detail','z':4,'filled':True,'tone':.42,'kind':'rect','x':x,'y':y,'w':w,'h':h,'region':reg,'material':mat,'penMm':.2})
            shaft=next(q for q in a['volumes']['50']['structure'] if q['id']=='member-shaft');zmin,zmax=shaft['profile'][0][0],shaft['profile'][-1][0]
            rect('shaft-section','rotor','hub',shaft['material'],*( (zmin,-.004,zmax-zmin,.008) if v=='front' else (-.004,zmin,.008,zmax-zmin)))
            if v=='front':
                s=sections[len(sections)//2];points=[]
                for j in range(12):
                    ang=j*math.tau/12;dxyz=apply(s['rotation'],s['rx']*math.cos(ang),s['ry']*math.sin(ang),0)
                    points.append([round(s['at'][2]+dxyz[2],6),round(s['at'][1]+dxyz[1],6)])
                shapes.append({'id':'blade-section','bone':'rotor','role':'detail','z':4.1,'filled':True,'tone':.65,'kind':'poly','closed':True,'points':points,'region':'blades','material':b['blade-1']['material'],'penMm':.2})
        key='detail' if 'detail' in rep['decisions'] else next(iter(rep['decisions']))
        rep['decisions'][key]={'en':'Five pitched blades remain distinct around the motor hub. The side view includes a radial blade section and the bearing shaft. The cut surfaces take a 0.20 mm paper line.','de':'Fünf angestellte Schaufeln bleiben um die Motornabe erkennbar. Die Seitenansicht zeigt einen radialen Schaufelschnitt und die Lagerwelle. Die Schnittflächen erhalten 0,20 mm Strichstärke im Druck.'}
if __name__=='__main__':
    for slug in ['duct-fan','decentralised-hrv-fan']:
        p=t/'🗑️generated'/'rotor'/(slug+'.json');a=json.loads(p.read_bytes());draw(a['asset']);p.write_text(json.dumps(a,ensure_ascii=False))
