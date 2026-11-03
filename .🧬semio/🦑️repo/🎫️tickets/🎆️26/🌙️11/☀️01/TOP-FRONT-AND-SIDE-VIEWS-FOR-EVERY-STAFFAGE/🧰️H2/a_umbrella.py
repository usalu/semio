import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from human import *
AID='person-with-umbrella'
reg=dict(face='head-skin',hair='hair',hand='hand-skin',coat='coat',trousers='leg-clothing',shoe='shoes',regions500=['coat','hand-skin'],hips='leg-clothing')
h=Human(AID,reg,arm='upper-arm',neck=False)
c=Conv(AID)
YU=0.0
SOLID={'kind':'revolved','axisBone':'umbrella','axisX':0,'axisZ':YU,'mirror':True}
def canopy(sc,front,plan):
    el={s['id']:s for s in h.A['representations'][str(sc)]['geometry']['elevation']}
    pens=h.A['representations'][str(sc)]['pens']
    for view,lst in (('front',front),('plan',plan)):
        s=dict(el['canopy-front']); s['id']='canopy'; s['z']=6 if view=='front' else 5
        s['solid']=dict(SOLID); lst.append(s)
        if sc in (50,100):
            r=dict(el['rim-tip-front']); r['id']='rim-tip'; r['solid']=dict(SOLID); lst.append(r)
    plan.append(ell('hub','rib-front',c.px('rib-front',0.3),YU,0.012 if sc<=100 else 0.02,0.012 if sc<=100 else 0.02,z=9.5,tone=0.3,region='umbrella-frame',role='detail' if sc<=100 else 'body'))
def build(sc):
    front=front_view(h,sc); plan=plan_view(h,sc)
    U='umbrella'
    w={500:0.025,200:0.012,100:0.009,50:0.0075}[sc]
    pen=0.1
    fy=lambda y: c.fyf(U,y)
    ext=lambda y: c.t_rest(U)[1]+(y-c.d[U]['t'][1])
    x0,x1=c.pcap(U,0.3,0.3)
    plan.append(cap('shaft',U,x0,YU,x1,YU,w,w,z=9,tone=0.2,region='umbrella-frame',pen=pen))
    front.append(poly('shaft',U,[(YU-w,fy(1.2068)),(YU+w,fy(1.2068)),(YU+w,fy(1.9868)),(YU-w,fy(1.9868))],follow=True,z=9,tone=0.2,region='umbrella-frame',pen=pen))
    rt=c.t_rest(U)[1]
    ln={500:0.1,200:0.1,100:0.1,50:0.1}[sc]
    fw=max(w,0.009 if sc==100 else 0.006)
    front.append(poly('ferrule',U,[(YU-fw,rt),(YU+fw,rt),(YU+fw*0.6,rt-ln),(YU-fw*0.6,rt-ln)],z=9.2,tone=0.1,region='umbrella-frame',role='body',pen=0.1))
    canopy(sc,front,plan)
    return front,plan,{'silhouette':joined(sc,"The umbrella shaft is a strip along its bone in front and a capsule in plan, so it follows every tilt; the canopy is a solid of revolution about the hub whose width follows the furling ribs.","Der Schirmstock ist in der Vorderansicht ein Streifen entlang seines Knochens und in der Draufsicht eine Kapsel und folgt so jeder Neigung; die Bespannung ist ein Rotationskörper um die Nabe, dessen Breite dem Einklappen der Rippen folgt.")}
if __name__=='__main__':
    from apply import apply
    apply(AID,build)
