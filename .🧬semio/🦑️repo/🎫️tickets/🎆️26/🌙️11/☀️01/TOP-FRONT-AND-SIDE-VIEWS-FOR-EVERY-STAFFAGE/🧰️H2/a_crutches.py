import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from human import *
AID='person-on-crutches'
reg=dict(face='face-skin',hair='hair',hand='hands',coat='hoodie',trousers='legs-skin',shoe='boot',regions500=['hoodie','hands'],hips='shorts')
reg['trousers-l']='legs-skin'; reg['trousers-r']='legs-skin'
h=Human(AID,reg,shoe={'l':'boot','r':'sneaker'},td=0.14)
c=Conv(AID)
def build(sc):
    front=front_view(h,sc); plan=plan_view(h,sc)
    k=h.k
    for sd,sg in SIDES:
        xc=sg*h.hw
        if sc==500:
            for s in front:
                if s['id']=='sil-leg-'+sd:
                    s.pop('region'); s['regions']=['legs-skin','shorts']
        else:
            front.append(poly('shorts-'+sd,'thigh-'+sd,limb(xc,h.hipY+0.02*k,h.hipY-0.2*k,h.rTh[0]+0.012*k,h.rTh[0]*0.97,eb=0.0),follow=True,**h.kw('shorts',1.5)))
    plan.append(ell('hood','torso',-0.1,0,0.09,0.1,z=3.5,tone=0.34,region='hoodie'))
    # crutches
    topy=0.8935; hl=0.2
    for sd,sg in SIDES:
        y0={'l':0.8935,'r':0.8881}[sd]
        b='crutch-'+sd
        fy=lambda y: c.fyf(b,y)
        lat_tip=sg*(0.325-1.6*{500:0.026,200:0.015,100:0.0125,50:0.011}[sc])
        lat_top=sg*(h.sw+0.015)
        lat=lambda y: lat_tip+(lat_top-lat_tip)*(y/y0)
        w={500:0.026,200:0.015,100:0.0125,50:0.011}[sc]
        pen=0.1
        x0,x1=c.pcap(b,{'l':0.3083,'r':0.296}[sd],{'l':0.2,'r':0.15}[sd])
        plan.append(cap('crutch-tube-'+sd,b,x0,lat_top,x1,lat_tip,w,w,z=9 if sd=='r' else 0.8,tone=0.62 if sd=='r' else 0.5,region='crutch-frame',pen=pen))
        front.append(poly('crutch-tube-'+sd,b,[(lat(y0-0.03)-w,fy(y0-0.03)),(lat(y0-0.03)+w,fy(y0-0.03)),(lat(0.07)+w,fy(0.07)),(lat(0.07)-w,fy(0.07))],follow=True,z=9 if sd=='r' else 0.8,tone=0.62 if sd=='r' else 0.5,region='crutch-frame',pen=pen))
        if sc<=100:
            front.append(poly('crutch-tip-'+sd,b,[(lat(0.07)-w*1.4,fy(0.07)),(lat(0.07)+w*1.4,fy(0.07)),(lat(0.0)+w*1.6,fy(0.0)),(lat(0.0)-w*1.6,fy(0.0))],follow=True,z=9.1,tone=0.1,region='crutch-tips',pen=pen))
            front.append(ell('crutch-grip-'+sd,'hand-'+sd,sg*(h.sw+0.015),h.wriY-0.01,0.03,0.016,z=9.2,tone=0.12,region='crutch-grips',pen=pen))
            front.append(rect('crutch-cuff-'+sd,'forearm-'+sd,sg*h.sw-0.045,h.elbY+0.07,0.09,0.05,r=0.015,z=9.25,tone=0.15,region='crutch-cuffs'))
            plan.append(ell('crutch-cuff-'+sd,'forearm-'+sd,c.px('forearm-'+sd,0.13),sg*h.sw,0.022,0.07,z=9.25,tone=0.15,region='crutch-cuffs'))
        if sc==50:
            plan.append(ell('crutch-grip-'+sd,'hand-'+sd,c.px('hand-'+sd,0.29),sg*(h.sw+0.015),0.045,0.016,z=9.2,tone=0.12,region='crutch-grips',pen=pen))
    return front,plan,{'silhouette':joined(sc,"Each crutch is a slanted strip in front, from the hand grip out to the planted tip at 0.325 m from the centre line, and a capsule from grip to tip in plan, both on the crutch bone so the tripod and every swing-through reads; bare thighs show below the shorts, and the left boot and right sneaker keep their own regions.","Jede Krücke ist in der Vorderansicht ein schräger Streifen vom Handgriff zur aufgesetzten Spitze 0,325 m neben der Mittellinie und in der Draufsicht eine Kapsel von Griff zu Spitze, beide am Krückenknochen, sodass Dreipunktstand und jedes Durchschwingen lesbar bleiben; unter den Shorts sind die nackten Oberschenkel sichtbar, und linker Stiefel und rechter Turnschuh behalten ihre eigenen Regionen.")}
if __name__=='__main__':
    from apply import apply
    apply(AID,build)
