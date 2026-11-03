import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from human import *
AID='person-with-white-cane'
h=Human(AID,dict(face='face-skin',hair='hair',hand='hands',coat='jacket',trousers='jeans',shoe='boots',regions500=['jacket','hands'],hips='jacket'),sw=0.15,hw=0.085,td=0.11)
c=Conv(AID)
CY=-0.15
def build(sc):
    front=front_view(h,sc); plan=plan_view(h,sc)
    k=h.k
    cx0,cx1=0.1469,0.8511
    r={500:0.026,200:0.014,100:0.0095,50:0.0085}[sc]
    pen={500:0.1,200:0.1,100:0.12,50:0.1}[sc]
    x0,x1=c.pcap('cane',cx0,cx1)
    yl={}
    # cane in plan
    plan.append(cap('cane-shaft','cane',x0,CY,x1,CY,r,r,z=9,tone=0.9,region='cane-shaft',pen=pen))
    if sc<=100: pass
    # cane in front: vertical strip from hand to tip
    y_hand,y_gnd=1.0928,0.0
    def fx(y): return c.fyf('cane',y)
    wid=r
    front.append(poly('cane-shaft','cane',[(CY-wid,fx(y_hand-0.02)),(CY+wid,fx(y_hand-0.02)),(CY+wid,fx(0.02)),(CY-wid,fx(0.02))],follow=True,z=9,tone=0.9,region='cane-shaft',pen=pen))
    if sc<=100:
        front.append(poly('cane-grip','cane',[(CY-0.016,fx(y_hand+0.02)),(CY+0.016,fx(y_hand+0.02)),(CY+0.016,fx(y_hand-0.09)),(CY-0.016,fx(y_hand-0.09))],follow=True,z=9.2,tone=0.12,region='cane-grip',pen=0.1))
        front.append(poly('cane-tip','cane',[(CY-0.014,fx(0.05)),(CY+0.014,fx(0.05)),(CY+0.02,fx(-0.0)),(CY-0.02,fx(-0.0))],follow=True,z=9.2,tone=0.7,region='cane-tip',pen=0.1))
    # bag
    by=0.16
    plan.append(ell('bag','pelvis',-0.115 if sc==500 else -0.1,by,0.115 if sc==500 else 0.1,0.055,**h.kw('bag',2.5)))
    front.append(poly('bag','pelvis',[(by-0.055,0.80),(by+0.055,0.80),(by+0.06,1.0),(by-0.06,1.0)] if False else [(by-0.055,h.pelvY-0.1),(by+0.055,h.pelvY-0.1),(by+0.06,h.pelvY+0.12),(by-0.06,h.pelvY+0.12)],z=2.5,tone=0.3,region='bag'))
    if sc<=100:
        front.append(poly('bag-strap','torso',[(h.sw*0.7,h.neckY),(by,h.pelvY+0.1)],closed=False,follow=True,z=2.6,tone=0.2,region='bag',role='detail',filled=False))
    if sc<=100:
        for sd,sg in SIDES:
            front.append(ell('lens-'+sd,'head',sg*0.036,h.headCY+0.012,0.028,0.015,z=6.7,tone=0.05,region='sunglasses',role='detail' if sc==100 else 'body'))
        front.append(line('bridge','head',[(-0.01,h.headCY+0.016),(0.01,h.headCY+0.016)],z=6.7,tone=0.05,region='sunglasses'))
    return front,plan,{'silhouette':joined(sc,"The white cane is a strip along its bone in front and a capsule from the hand to the roller tip in plan, so it follows every sweep; the shoulder bag sits at the left hip.","Der Langstock ist in der Vorderansicht ein Streifen entlang seines Knochens und in der Draufsicht eine Kapsel von der Hand bis zur Rollspitze und folgt so jedem Pendeln; die Umhängetasche sitzt an der linken Hüfte.")}
if __name__=='__main__':
    from apply import apply
    apply(AID,build)
