import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from human import *
AID='person-cooking'
h=Human(AID,dict(face='face-skin',hair='hair',hand='hand-skin',coat='hoodie',trousers='jeans',shoe='sneakers',regions500=['hoodie','hand-skin'],hips='jeans'))
c=Conv(AID)
PY=-0.14
def build(sc):
    front=front_view(h,sc); plan=plan_view(h,sc)
    T=lambda r: h.tone.get(r,0.5)
    # counter, hob, pot: authored in default = rest coordinates
    if sc==500:
        front.append(rect('counter','counter',-0.31,0,0.62,0.91,z=8,tone=0.4,regions=['counter','hob']))
        front.append(rect('pot','pot',PY-0.11,0.91,0.22,0.14,r=0.02,z=8.5,tone=0.5,region='pot'))
        plan.append(rect('counter','counter',0.32,-0.31,0.64,0.62,z=2,tone=0.4,regions=['counter','hob']))
        plan.append(ell('pot','pot',0.66,PY,0.11,0.11,z=2.5,tone=0.5,region='pot'))
        plan.append(cap('spoon','spoon',c.pcap('spoon',0.57,0.645)[0],PY,c.pcap('spoon',0.57,0.645)[1],PY,0.02,0.03,z=6.6,tone=0.45,region='spoon'))
    else:
        if sc==200:
            front.append(rect('counter-body','counter',-0.31,0,0.62,0.86,z=8,tone=0.55,region='counter'))
            front.append(rect('worktop','counter',-0.31,0.86,0.62,0.05,z=8.1,tone=0.4,regions=['counter','hob']))
        else:
            front.append(rect('counter-plinth','counter',-0.28,0,0.56,0.1,z=8,tone=0.25,region='counter'))
            front.append(rect('counter-body','counter',-0.31,0.1,0.62,0.76,z=8.02,tone=0.55,region='counter'))
            front.append(rect('counter-panel','counter',-0.27,0.15,0.54,0.66,z=8.04,tone=0.1,region='counter',role='detail',filled=False))
            front.append(rect('worktop','counter',-0.31,0.86,0.62,0.04,z=8.1,tone=0.4,region='counter'))
            front.append(rect('hob-plate','counter',-0.26,0.9,0.52,0.01,z=8.2,tone=0.2,region='hob'))
        front.append(rect('pot-body','pot',PY-0.11,0.91,0.22,0.138,r=0.02,z=8.5,tone=0.5,region='pot'))
        if sc<=100:
            front.append(rect('pot-rim','pot',PY-0.12,1.034,0.24,0.014,z=8.52,tone=0.65,region='pot',role='detail'))
        # plan
        plan.append(rect('worktop','counter',0.32,-0.31,0.64,0.62,z=2,tone=0.4,region='counter'))
        plan.append(rect('hob-plate','counter',0.40,-0.26,0.52,0.52,z=2.2,tone=0.2,region='hob'))
        if sc==50:
            plan.append(ell('burner-ring','counter',0.66,0.14,0.075,0.075,z=2.3,tone=0.1,region='hob',role='detail',filled=False))
            plan.append(ell('burner-ring-2','counter',0.52,0.14,0.05,0.05,z=2.3,tone=0.1,region='hob',role='detail',filled=False))
        if sc<=100:
            plan.append(cap('pot-handle','pot',0.455,PY,0.55,PY,0.014,0.014,follow=False,z=2.45,tone=0.4,region='pot'))
        plan.append(ell('pot-body','pot',0.66,PY,0.11,0.11,z=2.5,tone=0.5,region='pot'))
        if sc<=100:
            plan.append(ell('pot-rim','pot',0.66,PY,0.1,0.1,z=2.52,tone=0.65,region='pot',role='detail',filled=False))
        x0,x1=c.pcap('spoon',0.57,0.645)
        plan.append(cap('spoon','spoon',x0,PY,x1,PY,0.009 if sc<=100 else 0.014,0.028 if sc<=100 else 0.03,z=6.6,tone=0.45,region='spoon'))
    # spoon front
    fy=lambda y: c.fyf('spoon',y)
    if sc<=200:
        front.append(poly('spoon','spoon',[(PY-0.012,fy(1.16)),(PY+0.012,fy(1.16)),(PY+0.012,fy(1.0)),(PY+0.03,fy(1.0)),(PY+0.03,fy(0.953)),(PY-0.03,fy(0.953)),(PY-0.03,fy(1.0)),(PY-0.012,fy(1.0))],follow=True,z=9,tone=0.45,region='spoon'))
    return front,plan,{'silhouette':joined(sc,"The counter, hob plate and pot are rectangles and circles on their own bones; in front the counter hides everything of the cook below the worktop, in plan the pot sits left of the centre line where the right hand reaches it, and the spoon is a tapered capsule from the hand to the bowl so it follows the stirring.","Arbeitsplatte, Kochfeld und Topf sind Rechtecke und Kreise an eigenen Knochen; in der Vorderansicht verdeckt die Arbeitsplatte alles unterhalb der Platte, in der Draufsicht steht der Topf links der Mittellinie, wo die rechte Hand ihn erreicht, und der Löffel ist eine verjüngte Kapsel von der Hand bis zur Laffe und folgt so dem Rühren.")}
if __name__=='__main__':
    from apply import apply
    apply(AID,build)
