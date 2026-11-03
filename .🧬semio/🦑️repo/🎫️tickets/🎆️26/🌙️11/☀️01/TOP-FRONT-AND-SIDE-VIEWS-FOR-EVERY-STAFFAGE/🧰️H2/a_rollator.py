import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from human import *
AID='person-with-rollator'
reg=dict(face='face-skin',hair='hair',hand='hands',coat='coat',trousers='trousers',shoe='shoes',regions500=['coat','hands'],hips='coat')
h=Human(AID,reg,hat=dict(kind='beret',region='beret'))
c=Conv(AID)
def build(sc):
    front=front_view(h,sc); plan=plan_view(h,sc)
    P=lambda b,x: c.px(b,x)
    F=lambda b,y: c.fy(b,y)
    pen={500:0.1,200:0.1,100:0.1,50:0.1}[sc]
    rail=0.27; wy=0.29
    fr='frame'
    # ---- plan
    if sc==500:
        plan.append(rect('frame','frame',P(fr,0.3),-0.31,0.72,0.62,z=1,tone=0.35,regions=['frame','seat-basket','tyres']))
    else:
        for sd,sg in SIDES:
            plan.append(cap('rail-'+sd,fr,P(fr,0.40),sg*rail,P(fr,0.88),sg*rail,0.0125 if sc<=100 else 0.02,0.0125 if sc<=100 else 0.02,follow=False,z=0.8,tone=0.4,region='frame',pen=pen))
            for nm,wb in (('rear','wheel-rear'),('front','wheel-front')):
                plan.append(solid(ell('tyre-%s-%s'%(nm,sd),wb,0,0,0.1,0.1,z=0.5,tone=0.12,region='tyres'),sg*wy-0.02,sg*wy+0.02))
        plan.append(cap('handlebar',fr,P(fr,0.395),-wy,P(fr,0.395),wy,0.0125,0.0125,follow=False,z=1.0,tone=0.4,region='frame',pen=pen))
        plan.append(cap('front-bar',fr,P(fr,0.88),-rail,P(fr,0.88),rail,0.0125,0.0125,follow=False,z=1.0,tone=0.4,region='frame',pen=pen))
        plan.append(rect('seat',fr,P(fr,0.44),-0.19,0.435,0.38,r=0.03,z=0.9,tone=0.35,region='seat-basket'))
        for sd,sg in SIDES:
            plan.append(cap('grip-'+sd,fr,P(fr,0.26),sg*wy,P(fr,0.40),sg*wy,0.017,0.017,follow=False,z=4.1,tone=0.15,region='grips',pen=0.1))
        if sc<=100:
            plan.append(rect('backrest',fr,P(fr,0.39),-0.2,0.05,0.4,r=0.01,z=1.05,tone=0.3,region='seat-basket'))
        if sc==50:
            for sd,sg in SIDES:
                plan.append(cap('brake-'+sd,fr,P(fr,0.27),sg*(wy-0.05),P(fr,0.33),sg*(wy-0.05),0.006,0.006,follow=False,z=4.2,tone=0.5,region='frame',role='detail',pen=0.1))
    # ---- front
    if sc==500:
        front.append(rect('frame','frame',-0.31,0,0.62,0.87,z=9,tone=0.35,regions=['frame','seat-basket','tyres']))
    else:
        for sd,sg in SIDES:
            front.append(solid(ell('tyre-'+sd,'wheel-front',0,0,0.1,0.1,z=9.3,tone=0.12,region='tyres'),sg*wy-0.02,sg*wy+0.02))
            front.append(rect('leg-'+sd,fr,sg*rail-0.0125,0.2,0.025,0.65,z=9.1,tone=0.4,region='frame',pen=pen))
        front.append(rect('handlebar',fr,-wy,0.84,2*wy,0.025,z=9.2,tone=0.4,region='frame',pen=pen))
        front.append(rect('seat',fr,-0.2,0.5,0.4,0.04,r=0.012,z=9.0,tone=0.35,region='seat-basket'))
        if sc<=100:
            front.append(rect('backrest',fr,-0.2,0.58,0.4,0.15,r=0.01,z=8.9,tone=0.3,region='seat-basket'))
            front.append(rect('basket',fr,-0.17,0.36,0.34,0.14,r=0.015,z=8.95,tone=0.62,region='seat-basket'))
            front.append(rect('lower-rail',fr,-rail,0.24,2*rail,0.025,z=9.05,tone=0.4,region='frame',pen=pen))
            for sd,sg in SIDES:
                front.append(rect('grip-'+sd,fr,sg*wy-0.0175,0.83,0.035,0.045,r=0.01,z=9.4,tone=0.15,region='grips',pen=0.1))
        if sc<=200:
            front.append(rect('bag',fr,-0.12,0.4,0.24,0.12,r=0.02,z=8.96,tone=0.78,region='bag'))
        if sc==50:
            for sd,sg in SIDES:
                front.append(rect('caster-fork-'+sd,'wheel-front',sg*rail-0.012,0.1,0.024,0.2,z=9.15,tone=0.7,region='frame',pen=pen))
                front.append(solid(ell('hub-'+sd,'wheel-front',0,0,0.02,0.02,z=9.4,tone=0.3,region='frame'),sg*wy+0.02,sg*wy+0.03))
    return front,plan,{'silhouette':joined(sc,"The rollator is drawn as seen: in front its two front wheels, legs, seat, backrest, basket and handlebar stand in front of the walker and hide her legs (the rear wheels are hidden behind the front ones); in plan the two side rails, the front and rear bars, the seat, backrest and four wheels lie below her hands, which cover the grips. All parts sit on the frame and wheel bones so the frame slides, the seat and wheel positions follow every pose.","Der Rollator ist so gezeichnet, wie er gesehen wird: in der Vorderansicht stehen die beiden Vorderräder, Beine, Sitz, Rückenlehne, Korb und Schiebegriff vor der Gehenden und verdecken ihre Beine (die Hinterräder liegen hinter den vorderen); in der Draufsicht liegen die beiden Seitenrohre, Front- und Hinterbügel, Sitz, Rückenlehne und vier Räder unter ihren Händen, die die Griffe bedecken. Alle Teile hängen an Rahmen- und Radknochen, sodass Rahmen, Sitz und Radlage jeder Pose folgen.")}
if __name__=='__main__':
    from apply import apply
    apply(AID,build)
