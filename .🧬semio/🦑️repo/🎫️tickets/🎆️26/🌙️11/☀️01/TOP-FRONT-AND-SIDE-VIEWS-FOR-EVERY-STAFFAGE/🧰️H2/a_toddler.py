import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from human import *
AID='parent-carrying-toddler'
reg=dict(face='face',hair='beanie',hand='gloves',coat='coat-torso',sleeve='coat-sleeves',trousers='jeans',shoe='boots',neckskin='scarf',regions500=['coat-torso','coat-sleeves','gloves','scarf'],hips='coat-torso')
h=Human(AID,reg,arm='upper-arm',neck=False,td=0.135)
c=Conv(AID)
TC=0.07
def build(sc):
    front=front_view(h,sc); plan=plan_view(h,sc)
    P=c.px; F=c.fy
    TT='toddler-torso'; TH='toddler-head'
    kw=h.kw
    if sc==500:
        front.append(poly('sil-toddler',TT,[(TC-0.1,F(TT,0.80)),(TC+0.1,F(TT,0.80)),(TC+0.115,F(TT,1.22)),(TC+0.07,F(TT,1.45)),(TC-0.07,F(TT,1.45)),(TC-0.115,F(TT,1.22))],z=8,tone=0.4,regions=['toddler-suit','toddler-face','toddler-hat','toddler-mittens','toddler-boots'],material='padded-suit'))
        plan.append(ell('sil-toddler',TT,0.24,TC,0.2,0.115,z=8,tone=0.4,regions=['toddler-suit','toddler-face','toddler-hat','toddler-mittens','toddler-boots'],material='padded-suit'))
    else:
        # front: toddler faces forward, seen from the front
        front.append(poly('toddler-suit',TT,[(TC-0.085,c.fyf(TT,1.0)),(TC+0.085,c.fyf(TT,1.0)),(TC+0.1,c.fyf(TT,1.2)),(TC+0.1,c.fyf(TT,1.259)),(TC-0.1,c.fyf(TT,1.259)),(TC-0.1,c.fyf(TT,1.2))],follow=True,z=8.1,tone=0.35,region='toddler-suit',material='padded-suit'))
        front.append(ell('toddler-head',TH,TC,F(TH,1.355),0.074,0.09,z=8.6,tone=0.86,region='toddler-face',material='skin'))
        front.append(poly('toddler-hat',TH,[(x+TC,F(TH,y)) for x,y in arc(0,1.38,0.082,0.075,180,0,10)]+[(TC+0.082,F(TH,1.34)),(TC-0.082,F(TH,1.34))],z=8.7,tone=0.3,region='toddler-hat',material='knit-hat'))
        for sd,sg in SIDES:
            front.append(ell('toddler-thigh-'+sd,'toddler-thigh-'+('n' if sd=='l' else 'f'),TC+sg*0.05,F('toddler-thigh-'+('n' if sd=='l' else 'f'),0.985),0.042,0.04,z=8.2,tone=0.35,region='toddler-suit',material='padded-suit'))
            sh='toddler-shank-'+('n' if sd=='l' else 'f')
            yk=c.d[sh]['o'][1]; ya=c.d[sh]['t'][1]
            front.append(poly('toddler-shank-'+sd,sh,limb(TC+sg*0.05,c.fyf(sh,yk),c.fyf(sh,ya),0.036,0.03),follow=True,z=8.25,tone=0.35,region='toddler-suit',material='padded-suit'))
            front.append(poly('toddler-boot-'+sd,sh,[(TC+sg*0.05-0.034,c.fyf(sh,ya+0.03)),(TC+sg*0.05+0.034,c.fyf(sh,ya+0.03)),(TC+sg*0.05+0.04,c.fyf(sh,ya-0.045)),(TC+sg*0.05-0.04,c.fyf(sh,ya-0.045))],follow=True,z=8.3,tone=0.2,region='toddler-boots',material='rubber'))
        an='toddler-arm-n'; af='toddler-arm-f'
        front.append(ell('toddler-arm-n',an,TC+0.115,F(an,1.23),0.035,0.04,z=8.4,tone=0.35,region='toddler-suit',material='padded-suit'))
        front.append(poly('toddler-arm-f',af,limb(TC-0.115,c.fyf(af,1.23),c.fyf(af,1.04),0.032,0.026),follow=True,z=8.4,tone=0.35,region='toddler-suit',material='padded-suit'))
        front.append(ell('toddler-mitten',af,TC-0.115,F(af,1.045),0.03,0.03,z=8.45,tone=0.35,region='toddler-mittens'))
        # plan
        plan.append(ell('toddler-suit',TT,P(TT,0.19),TC,0.065,0.11,z=8.1,tone=0.35,region='toddler-suit',material='padded-suit'))
        plan.append(ell('toddler-head',TH,P(TH,0.24),TC,0.082,0.075,z=8.6,tone=0.86,region='toddler-face',material='skin'))
        plan.append(ell('toddler-hat',TH,P(TH,0.236),TC,0.07,0.07,z=8.7,tone=0.3,region='toddler-hat',material='knit-hat'))
        for sd,sg in SIDES:
            n='n' if sd=='l' else 'f'
            t='toddler-thigh-'+n; s='toddler-shank-'+n
            x0,x1=c.pcap(t,0.15,0.28); plan.append(cap('toddler-thigh-'+sd,t,x0,TC+sg*0.05,x1,TC+sg*0.05,0.04,0.034,z=8.2,tone=0.35,region='toddler-suit',material='padded-suit'))
            x0,x1=c.pcap(s,0.28,0.341); plan.append(cap('toddler-shank-'+sd,s,x0,TC+sg*0.05,x1,TC+sg*0.05,0.034,0.03,z=8.25,tone=0.35,region='toddler-suit',material='padded-suit'))
            plan.append(cap('toddler-boot-'+sd,s,P(s,0.345),TC+sg*0.05,P(s,0.42),TC+sg*0.05,0.034,0.034,follow=False,z=8.3,tone=0.2,region='toddler-boots',material='rubber'))
        x0,x1=c.pcap(an,0.2081,0.0201); plan.append(cap('toddler-arm-n',an,x0,TC+0.11,x1,TC+0.04,0.032,0.028,z=8.4,tone=0.35,region='toddler-suit',material='padded-suit'))
        x0,x1=c.pcap(af,0.2081,0.2632); plan.append(cap('toddler-arm-f',af,x0,TC-0.11,x1,TC-0.11,0.03,0.028,z=8.4,tone=0.35,region='toddler-suit',material='padded-suit'))
        if sc<=100:
            plan.append(ell('toddler-mitten',an,P(an,0.03),TC+0.04,0.03,0.03,z=8.45,tone=0.35,region='toddler-mittens'))
    return front,plan,{'silhouette':joined(sc,"The toddler sits on the parent's left forearm facing forward and is drawn before the parent: in front its padded suit, face and hat, thighs seen end-on and two shanks with boots cover the parent's chest; in plan its suit, hat, thigh and shank capsules (which stretch while it is hoisted) and the arm reaching back around the parent's neck are drawn above the parent.","Das Kleinkind sitzt auf dem linken Unterarm des Elternteils und blickt nach vorn; es ist vor dem Elternteil gezeichnet: in der Vorderansicht bedecken sein Schneeanzug, Gesicht und Mütze, die von vorn gesehenen Oberschenkel und zwei Unterschenkel mit Stiefeln die Brust des Elternteils; in der Draufsicht liegen sein Anzug, die Mütze, die Kapseln für Ober- und Unterschenkel (die sich beim Hochnehmen dehnen) und der Arm, der sich um den Hals des Elternteils legt, über dem Elternteil.")}
if __name__=='__main__':
    from apply import apply
    apply(AID,build)
