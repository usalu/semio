import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from human import *
AID='parent-with-stroller'
reg=dict(face='head-skin',hair='hair',hand='hand-skin',coat='torso-clothing',sleeve='arm-clothing',trousers='leg-clothing',shoe='shoes',regions500=['torso-clothing','arm-clothing','hand-skin'],hips='leg-clothing')
h=Human(AID,reg,arm='upper-arm',neck=False)
c=Conv(AID)
def build(sc):
    front=front_view(h,sc); plan=plan_view(h,sc)
    P=c.px; F=c.fy
    pen={500:0.1,200:0.1,100:0.12,50:0.14}[sc]
    ST='stroller'
    ALL=['stroller-frame','stroller-fabric','tyres','child-clothing','child-skin']
    def X(view,lst,shape,z0,z1):
        lst.append(solid(shape,z0,z1))
    def both(shape,z0,z1,zf,zp):
        a=dict(shape); a['z']=zf; front.append(solid(a,z0,z1))
        b=dict(shape); b['z']=zp; plan.append(solid(b,z0,z1))
    for sd,sg in SIDES:
        both(ell('tyre-rear-'+sd,'wheel-rear',0,0,0.1592,0.1592,tone=0.1,region='tyres'),sg*0.28-0.02,sg*0.28+0.02,7.2,7.1) if sg>0 else both(ell('tyre-rear-'+sd,'wheel-rear',0,0,0.1592,0.1592,tone=0.1,region='tyres'),sg*0.28-0.02,sg*0.28+0.02,7.2,7.1)
        both(ell('tyre-front-'+sd,'wheel-front',0,0,0.0796,0.0796,tone=0.1,region='tyres'),sg*0.2-0.02,sg*0.2+0.02,7.5,7.1)
    for s in (front,plan):
        for e in s:
            if 'solid' in e and 'z0' in e['solid'] and e['solid']['z0']>e['solid']['z1']: e['solid']['z0'],e['solid']['z1']=e['solid']['z1'],e['solid']['z0']
    if sc==500:
        both(poly('sil-body',ST,[(0.05,-0.05),(0.2,0.1),(0.55,0.1),(0.66,-0.1),(0.8,-0.62),(0.22,-0.58)],tone=0.4,regions=ALL),-0.26,0.26,8,8)
    else:
        for sd,sg in SIDES:
            both(poly('chassis-'+sd,ST,[(0.22,-0.567),(0.79,-0.607),(0.79,-0.633),(0.22,-0.593)],tone=0.5,region='stroller-frame',pen=pen),sg*0.24-0.013,sg*0.24+0.013,7.6,7.0)
            both(poly('push-bar-'+sd,ST,[(0.02,0.0),(0.045,0.0),(0.235,-0.58),(0.21,-0.58)],tone=0.5,region='stroller-frame',pen=pen),sg*0.24-0.013,sg*0.24+0.013,7.41,7.01)
            if sc<=100:
                both(poly('fork-'+sd,ST,[(0.76,-0.62),(0.79,-0.62),(0.82,-0.92),(0.79,-0.92)],tone=0.5,region='stroller-frame',pen=pen),sg*0.2-0.01,sg*0.2+0.01,7.42,7.02)
        both(ell('handle-grip',ST,0,0,0.017,0.017,tone=0.5,region='stroller-frame',pen=pen),-0.25,0.25,7.9,4.0)
        both(poly('hood','stroller-hood',[(0,0),(0.01,0.1),(0.08,0.2),(0.2,0.23),(0.34,0.18),(0.44,0.08),(0.46,0)],tone=0.45,region='stroller-fabric'),-0.26,0.26,8.0,7.6)
        both(rect('child-body','child',-0.075,0.03,0.165,0.3,r=0.03,tone=0.3,region='child-clothing'),-0.11,0.11,8.2,7.3)
        both(ell('child-head','child',-0.01,0.34,0.085,0.085,tone=0.88,region='child-skin'),-0.08,0.08,8.3,7.4)
        both(rect('child-legs','child',0,0.0,0.34,0.045,r=0.015,tone=0.3,region='child-clothing'),-0.085,0.085,8.25,7.3)
        if sc<=100:
            both(ell('child-hat','child',-0.01,0.395,0.085,0.045,tone=0.3,region='child-clothing'),-0.08,0.08,8.4,7.5)
            both(rect('footrest',ST,0.6,-0.67,0.14,0.05,tone=0.5,region='stroller-frame',pen=pen),-0.12,0.12,7.7,7.2)
        if sc<=200:
            both(rect('basket',ST,0.3,-0.8,0.38,0.1,tone=0.4,region='stroller-fabric'),-0.2,0.2,7.45,6.9)
    return front,plan,{'silhouette':joined(sc,"The stroller is drawn from its own bones: in front the wheels, forks, chassis, child and hood stand before the parent and hide the legs behind them; in plan the rails run from the push handle over the rear and front wheel bones to the front, the hood covers the child's upper body and the parent's hands cover the handle. The rails are split over the handle, rear-wheel and front-wheel bones with overlaps, so a tipped kerb pose still reads.","Der Kinderwagen ist aus eigenen Knochen gezeichnet: in der Vorderansicht stehen Räder, Gabeln, Fahrgestell, Kind und Verdeck vor dem Elternteil und verdecken dahinterliegende Beine; in der Draufsicht laufen die Rohre vom Schiebegriff nach vorn, das Verdeck bedeckt den Oberkörper des Kindes, und die Hände des Elternteils bedecken den Griff. Die Rohre hängen in drei Abschnitten an Griff-, Hinter- und Vorderradknochen, sodass auch die Bordstein-Kippstellung lesbar bleibt.")}
if __name__=='__main__':
    from apply import apply
    apply(AID,build)
