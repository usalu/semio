import os,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
from lib import *

class Human:
    """Rest-pose front and plan shapes of a standing person on the common human rig."""
    def __init__(s,aid,reg,arm='arm',neck=True,sw=None,hw=None,headRX=None,headRY=None,headH=None,tw=None,fw=None,rtop=None,hat=None,shoe=None,k=None,hair_cap=0.55,td=None,**extra):
        s.aid=aid; s.A=load(aid)['asset']; s.w=rest(s.A); w=s.w
        s.H=s.A['dimensions']['heightM']; s.k=k or s.H/1.75
        s.arm=arm; s.neck=neck; s.reg=reg; s.hat=hat; s.shoe=shoe or {}
        k=s.k
        s.sw=sw or 0.11*s.H; s.hw=hw or 0.092*k; s.tw=tw or s.sw+0.02; s.fw=fw or 0.045*k
        s.hairCap=hair_cap; s.td=td or 0.115*k
        s.hipY=w['thigh-l'][1]; s.kneeY=w['shank-l'][1]; s.ankleY=w['foot-l'][1]
        s.groundY=tip(w,'toe-l')[1]
        s.toeBaseX=tip(w,'foot-l')[0]; s.toeTipX=tip(w,'toe-l')[0]
        s.waistY=w['torso'][1]; s.neckY=tip(w,'torso')[1]; s.pelvY=w['pelvis'][1]
        s.shY=w[arm+'-l'][1]; s.elbY=w['forearm-l'][1]; s.wriY=w['hand-l'][1]; s.fingY=tip(w,'hand-l')[1]
        s.headO=(w['head'][0],w['head'][1]); s.headTop=tip(w,'head')[1]
        s.headH=headH or ((s.headTop-s.headO[1]) if neck else 0.225*k)
        s.headCY=s.headTop-s.headH/2
        s.headRY=headRY or s.headH/2+0.004
        s.headRX=headRX or 0.075*k
        s.headDX=s.headRX*1.2
        s.headCX=s.headO[0]+(0.012 if neck else 0.012)
        s.tone={}
        for sc in s.A['representations'].values():
            if sc.get('status')!='authored': continue
            for sh in sc['geometry']['elevation']:
                if sh.get('region') and sh['region'] not in s.tone: s.tone[sh['region']]=sh['tone']
        s.mat={}
        for sc in s.A['representations'].values():
            if sc.get('status')!='authored': continue
            for sh in sc['geometry']['elevation']:
                if sh.get('region') and sh.get('material'): s.mat[sh['region']]=sh['material']
        s.rTh=(0.085*k, 0.062*k); s.rSh=(0.058*k,0.040*k); s.rUA=(0.052*k,0.044*k); s.rFA=(0.044*k,0.034*k)
    def t(s,r,d=None):
        return s.tone.get(r,0.5) if d is None else d
    def kw(s,region,z,tone=None,**o):
        d=dict(z=z,tone=s.t(region,tone),region=region)
        m=s.mat.get(region)
        if m: d['material']=m
        d.update(o); return d
    def kwm(s,regions,z,tone,**o):
        d=dict(z=z,tone=tone,regions=regions)
        d.update(o); return d
    def shoe_reg(s,side): return s.shoe.get(side,s.reg['shoe'])
    def trousers(s,side): return s.reg.get('trousers-'+side,s.reg['trousers'])

import math
SIDES=(('l',1),('r',-1))

def arc(cx,cy,rx,ry,a0,a1,n):
    return [(cx+rx*math.cos(math.radians(a0+(a1-a0)*i/n)),cy+ry*math.sin(math.radians(a0+(a1-a0)*i/n))) for i in range(n+1)]

def head_front(h,sc,out):
    r=h.reg; cx=0; cy=h.headCY; rx=h.headRX; ry=h.headRY; k=h.k
    if sc==500:
        if h.hat: cy+=0.008; ry+=0.012
        out.append(ell('head','head',0,cy,rx,ry,**h.kwm([r['hair'],r['face']],6,h.t(r['hair']),)))
        return
    if sc==200:
        if h.hat: cy+=0.008; ry+=0.012
        out.append(ell('head','head',0,cy,rx,ry,**h.kwm([r['hair'],r['face']],6,(h.t(r['hair'])+h.t(r['face']))/2)))
        return
    out.append(ell('head','head',0,cy,rx,ry,**h.kw(r['face'],5)))
    cap_f=h.hairCap
    pts=arc(0,cy,rx*1.04,ry*1.03,180,0,10)
    hl=cy+ry*(1-2*cap_f)
    pts+= [(rx*1.0,hl),(rx*0.7,hl-0.012*k),(0,hl-0.004*k),(-rx*0.7,hl-0.012*k),(-rx*1.0,hl)]
    if r.get('hair'): out.append(poly('hair','head',pts,**h.kw(r['hair'],5.5)))
    if sc==50:
        for sd,sg in SIDES:
            out.append(ell('ear-'+sd,'head',sg*(rx+0.004),cy-0.012*k,0.011*k,0.024*k,**h.kw(r['face'],4.9,detail=None) if False else h.kw(r['face'],4.9,role='detail')))
        out.append(line('nose','head',[(0,cy+0.004),(0.003*k,cy-0.045*k)],z=5.6,tone=0.1,region=r['face']))
        out.append(line('mouth','head',[(-0.016*k,cy-0.075*k),(0.016*k,cy-0.075*k)],z=5.6,tone=0.1,region=r['face']))
    hat=h.hat
    if hat:
        if hat['kind']=='beret':
            out.append(ell('hat','head',hat.get('dx',0.012),h.headTop-0.03*k,rx*1.32,0.05*k,rot=hat.get('rot',4),**h.kw(hat['region'],6.5)))
        elif hat['kind']=='beanie':
            hp=arc(0,cy+0.01*k,rx*1.07,ry*1.05,180,0,10)+[(rx*1.07,cy-0.03*k),(rx*1.0,cy-0.05*k),(-rx*1.0,cy-0.05*k),(-rx*1.07,cy-0.03*k)]
            out.append(poly('hat','head',hp,**h.kw(hat['region'],6.5)))


def blen(h,b): return [x for x in h.A['rig']['bones'] if x['id']==b][0]['lengthM']
def rev(h,id,bone,xc,length,r0,r1,e0,e1,**kw):
    pts=[(-e0,0),(-e0,r0*0.78),(0,r0),(length*0.5,(r0+r1)/2*1.04),(length,r1),(length+e1,r1*0.78),(length+e1,0)]
    s=poly(id,bone,pts,**kw)
    s['solid']={'kind':'revolved','axisBone':bone,'axisX':0,'axisZ':round(xc,4),'mirror':True}
    return s

def front_view(h,sc):
    out=[]; r=h.reg; k=h.k; sw=h.sw; hw=h.hw
    cW=sw; wW=sw-0.035*k; hipW=hw+h.rTh[0]*0.95; crotch=h.hipY-0.06*k
    A=h.arm
    sh_pts=[(-wW,h.waistY),(wW,h.waistY),(cW,h.waistY+(h.shY-h.waistY)*0.55),(sw+0.02*k,h.shY-0.035*k),(sw*0.80,h.shY+0.012*k),(0.065*k,h.neckY),(-0.065*k,h.neckY),(-sw*0.80,h.shY+0.012*k),(-sw-0.02*k,h.shY-0.035*k),(-cW,h.waistY+(h.shY-h.waistY)*0.55)]
    if sc==500:
        sil=[(-wW,h.waistY),(wW,h.waistY),(sw+0.062*k,h.waistY+(h.shY-h.waistY)*0.4),(sw+0.062*k,h.wriY),(sw-0.045*k,h.wriY),(sw-0.045*k,h.elbY),(sw*0.55,h.elbY),(0.075*k,h.neckY),(-0.075*k,h.neckY),(-sw*0.55,h.elbY),(-sw+0.045*k,h.elbY),(-sw+0.045*k,h.wriY),(-sw-0.062*k,h.wriY),(-sw-0.062*k,h.waistY+(h.shY-h.waistY)*0.4)]
        sil=[(-wW,h.waistY),(wW,h.waistY),(sw+0.06*k,h.waistY+(h.shY-h.waistY)*0.35),(sw+0.06*k,h.wriY),(sw-0.04*k,h.wriY),(sw-0.04*k,h.shY-0.1*k),(sw*0.8,h.shY+0.01*k),(0.07*k,h.neckY+0.01*k),(-0.07*k,h.neckY+0.01*k),(-sw*0.8,h.shY+0.01*k),(-sw+0.04*k,h.shY-0.1*k),(-sw+0.04*k,h.wriY),(-sw-0.06*k,h.wriY),(-sw-0.06*k,h.waistY+(h.shY-h.waistY)*0.35)]
        out.append(poly('sil-torso','torso',sil,follow=True,**h.kwm(r.get('regions500',[r['coat'],r['hand']]),3.5,h.t(r['coat']))))
        head_front(h,500,out)
        for sd,sg in SIDES:
            lw=h.rTh[0]*0.95
            out.append(poly('sil-leg-'+sd,'thigh-'+sd,limb(sg*hw,h.hipY+0.02*k,h.kneeY,lw,0.058*k,eb=0.04*k),follow=True,**h.kw(h.trousers(sd),1)))
            out.append(poly('sil-shank-'+sd,'shank-'+sd,limb(sg*hw,h.kneeY,h.ankleY,0.052*k,0.042*k,et=0.03*k)[:6]+[(sg*hw+h.fw,h.groundY),(sg*hw-h.fw,h.groundY)]+limb(sg*hw,h.kneeY,h.ankleY,0.052*k,0.042*k,et=0.03*k)[7:],follow=True,**h.kwm([h.trousers(sd),h.shoe_reg(sd)],1.2,(h.t(h.trousers(sd))+h.t(h.shoe_reg(sd)))/2)))
        return out
    # legs
    for sd,sg in SIDES:
        xc=sg*hw
        out.append(rev(h,'thigh-'+sd,'thigh-'+sd,xc,blen(h,'thigh-'+sd),h.rTh[0],h.rTh[1],0.03*k,0.02*k,**h.kw(h.trousers(sd),1.0)))
        out.append(rev(h,'shank-'+sd,'shank-'+sd,xc,blen(h,'shank-'+sd),h.rSh[0],h.rSh[1],0.02*k,0.0,**h.kw(h.trousers(sd),1.02)))
        shoe=[(xc-h.fw*0.9,h.ankleY+0.02*k),(xc+h.fw*0.9,h.ankleY+0.02*k),(xc+h.fw*1.05,h.ankleY-0.025*k),(xc+h.fw*1.12,h.groundY+0.012*k),(xc+h.fw*1.0,h.groundY),(xc-h.fw*1.0,h.groundY),(xc-h.fw*1.12,h.groundY+0.012*k),(xc-h.fw*1.05,h.ankleY-0.025*k)]
        out.append(poly('shoe-'+sd,'foot-'+sd,shoe,follow=True,**h.kw(h.shoe_reg(sd),1.2)))
        if sc==50 and r.get('crease',True):
            out.append(line('crease-thigh-'+sd,'thigh-'+sd,[(xc,h.hipY-0.02*k),(xc,h.kneeY)],follow=True,z=1.05,tone=0.1,region=h.trousers(sd)))
            out.append(line('crease-shank-'+sd,'shank-'+sd,[(xc,h.kneeY),(xc,h.ankleY+0.03*k)],follow=True,z=1.07,tone=0.1,region=h.trousers(sd)))
    # hips and torso
    if sc<=100:
        out.append(poly('hips','pelvis',[(-hipW,h.waistY),(hipW,h.waistY),(hipW,crotch+0.03*k),(hipW*0.6,crotch),(-hipW*0.6,crotch),(-hipW,crotch+0.03*k)],**h.kw(r['hips'] if 'hips' in r else r['trousers'],2.0)))
    out.append(poly('torso','torso',sh_pts,follow=True,**h.kw(r['coat'],3.0)))
    if sc==50:
        out.append(line('placket','torso',[(0,h.neckY-0.01*k),(0,h.waistY)],follow=True,z=3.1,tone=0.1,region=r['coat']))
    # neck
    nb='neck' if h.neck else 'torso'
    if sc<=100:
        if h.neck:
            o=h.w['neck']; out.append(poly('neck','neck',limb(0,o[1],tip(h.w,'neck')[1],0.052*k,0.048*k,et=0.02*k,eb=0.03*k),follow=True,**h.kw(r['neckskin'] if 'neckskin' in r else r['face'],4.0)))
        else:
            out.append(poly('neck','torso',limb(0,h.neckY-0.015*k,h.neckY+0.045*k,0.052*k,0.048*k),follow=True,**h.kw(r['face'],4.0)))
    head_front(h,sc,out)
    # arms
    for sd,sg in SIDES:
        xc=sg*sw
        sl=r.get('sleeve',r['coat'])
        out.append(rev(h,'arm-'+sd,A+'-'+sd,xc,blen(h,A+'-'+sd),h.rUA[0],h.rUA[1],0.025*k,0.01*k,**h.kw(sl,3.5)))
        if sc==200:
            out.append(rev(h,'forearm-'+sd,'forearm-'+sd,xc,blen(h,'forearm-'+sd)+blen(h,'hand-'+sd),h.rFA[0],0.036*k,0.015*k,0.012*k,**h.kwm([sl,r['hand']],3.6,(h.t(sl)+h.t(r['hand']))/2)))
        else:
            out.append(rev(h,'forearm-'+sd,'forearm-'+sd,xc,blen(h,'forearm-'+sd),h.rFA[0],h.rFA[1],0.015*k,0.0,**h.kw(sl,3.6)))
            out.append(rev(h,'hand-'+sd,'hand-'+sd,xc,blen(h,'hand-'+sd),0.032*k,0.027*k,0.0,0.012*k,**h.kw(r['hand'],3.8)))
        if sc==50:
            out.append(line('cuff-'+sd,'forearm-'+sd,[(xc-h.rFA[1],h.wriY+0.035*k),(xc+h.rFA[1],h.wriY+0.035*k)],follow=True,z=3.65,tone=0.1,region=sl))
    return out

def head_plan(h,sc,out):
    r=h.reg; k=h.k; cx=h.headCX; rx=h.headDX; ry=h.headRX
    if sc in (500,200):
        out.append(ell('head','head',cx,0,rx,ry,**h.kwm([r['hair'],r['face']],6,h.t(r['hair']))))
        return
    out.append(ell('head','head',cx,0,rx,ry,**h.kw(r['face'],6.0)))
    out.append(ell('hair','head',cx-0.014*k,0,rx*0.9,ry*0.98,**h.kw(r['hair'],6.1)))
    if sc==50:
        for sd,sg in SIDES:
            out.append(ell('ear-'+sd,'head',cx-0.004,sg*(ry+0.002),0.016*k,0.008*k,**h.kw(r['face'],5.9)))
        out.append(ell('nose','head',cx+rx+0.006*k,0,0.012*k,0.009*k,**h.kw(r['face'],6.0)))
    hat=h.hat
    if hat:
        if hat['kind']=='beret':
            out.append(ell('hat','head',cx-0.006,0.0,rx*1.18,ry*1.22,**h.kw(hat['region'],6.5)))
        elif hat['kind']=='beanie':
            out.append(ell('hat','head',cx-0.004,0,rx*1.04,ry*1.06,**h.kw(hat['region'],6.5)))

def plan_view(h,sc):
    out=[]; r=h.reg; k=h.k; sw=h.sw; hw=h.hw; A=h.arm; w=h.w
    ax=w['foot-l'][0]; tox=w['toe-l'][0]
    for sd,sg in SIDES:
        yc=sg*hw
        shr=h.shoe_reg(sd)
        if sc==500:
            out.append(cap('shoe-'+sd,'foot-'+sd,ax-0.04*k,yc,h.toeTipX,yc,0.04*k,0.046*k,**h.kw(shr,1.2)))
        elif sc==200:
            out.append(cap('foot-'+sd,'foot-'+sd,ax-0.04*k,yc,h.toeTipX,yc,0.04*k,0.046*k,**h.kw(shr,1.2)))
        else:
            out.append(cap('thigh-'+sd,'thigh-'+sd,w['thigh-l'][0],yc,tip(w,'thigh-l')[0],yc,h.rTh[0],h.rTh[1],**h.kw(h.trousers(sd),1.05)))
            out.append(cap('shank-'+sd,'shank-'+sd,w['shank-l'][0],yc,tip(w,'shank-l')[0],yc,h.rSh[0],h.rSh[1],**h.kw(h.trousers(sd),1.1)))
            out.append(cap('shoe-heel-'+sd,'foot-'+sd,ax-0.04*k,yc,tox,yc,0.04*k,0.047*k,**h.kw(shr,1.2)))
            out.append(cap('shoe-toe-'+sd,'toe-'+sd,tox,yc,h.toeTipX,yc,0.047*k,0.038*k,**h.kw(shr,1.21)))
    if sc==500:
        out.append(ell('shoulders','neck' if h.neck else 'torso',0,0,h.td*1.05,sw+0.045*k,**h.kwm(r.get('regions500',[r['coat'],r['hand']]),4,h.t(r['coat']))))
        head_plan(h,500,out); return out
    nb='neck' if h.neck else 'torso'
    if sc<=100:
        out.append(ell('hips','pelvis',0,0,h.td*0.95,hw+h.rTh[0]*1.0,**h.kw(r.get('hips',r['trousers']),2.0)))
    out.append(ell('trunk','torso',0,0,h.td,sw+0.02*k,**h.kw(r['coat'],3.0)))
    out.append(ell('shoulders',nb,0,0,h.td*1.04,sw+0.04*k,**h.kw(r['coat'],4.0)))
    sl=r.get('sleeve',r['coat'])
    for sd,sg in SIDES:
        yc=sg*sw
        out.append(cap('arm-'+sd,A+'-'+sd,w[A+'-l'][0],yc,tip(w,A+'-l')[0],yc,h.rUA[0],h.rUA[1],**h.kw(sl,4.5)))
        if sc==200:
            out.append(cap('forearm-'+sd,'forearm-'+sd,w['forearm-l'][0],yc,tip(w,'forearm-l')[0],yc,h.rFA[0],0.043*k,**h.kwm([sl,r['hand']],4.6,(h.t(sl)+h.t(r['hand']))/2)))
        else:
            out.append(cap('forearm-'+sd,'forearm-'+sd,w['forearm-l'][0],yc,tip(w,'forearm-l')[0],yc,h.rFA[0],h.rFA[1],**h.kw(sl,4.6)))
            out.append(cap('hand-'+sd,'hand-'+sd,w['hand-l'][0],yc,tip(w,'hand-l')[0],yc,0.030*k,0.022*k,**h.kw(r['hand'],4.7)))
            if sc==50:
                out.append(cap('thumb-'+sd,'hand-'+sd,w['hand-l'][0]+0.03*k,yc-sg*0.03*k,tip(w,'hand-l')[0]-0.06*k,yc-sg*0.024*k,0.011*k,0.008*k,**h.kw(r['hand'],4.72)))
    head_plan(h,sc,out)
    return out

LEVEL={500:0,200:1,100:2,50:3}
NOTES={
 500:("Front and plan are composed silhouettes. Front: head ellipse, one torso-and-arms outline and one outline per leg (thigh mass plus a shank-and-shoe mass), each following its bone. Plan: a shoulders-and-arms ellipse, the head ellipse and one shoe capsule per foot; hands, legs, hair parting and face are merged or dropped.",
       "Vorder- und Draufsicht sind komponierte Silhouetten. Vorderansicht: Kopfellipse, eine Kontur für Rumpf mit Armen und je eine Kontur pro Bein (Oberschenkelmasse und eine Masse aus Unterschenkel und Schuh), jeweils an ihrem Knochen. Draufsicht: eine Ellipse für Schultern und Arme, die Kopfellipse und je eine Schuhkapsel pro Fuß; Hände, Beine, Scheitel und Gesicht sind verschmolzen oder entfallen."),
 200:("Front and plan show the main masses. Front: head, torso, an arm and a forearm-with-hand per side, a thigh and a shank-with-shoe per side. Plan: head, shoulders, trunk, an arm and a forearm-with-hand capsule per side, and one foot capsule per side; the legs are hidden below the trunk and dropped.",
       "Vorder- und Draufsicht zeigen die Hauptmassen. Vorderansicht: Kopf, Rumpf, je Seite Oberarm und Unterarm mit Hand sowie Oberschenkel und Unterschenkel mit Schuh. Draufsicht: Kopf, Schultern, Rumpf, je Seite eine Kapsel für Oberarm und eine für Unterarm mit Hand sowie je eine Fußkapsel; die Beine liegen unter dem Rumpf verdeckt und entfallen."),
 100:("Front and plan are articulated. Front: head with hair, neck, torso, hips, an arm, forearm and mitten hand per side, thigh, shank and shoe per side, with a cuff-end ellipse that keeps a forward-raised forearm visible. Plan: head with hair crescent, shoulders, trunk, hips, arm, forearm and hand capsules, thigh and shank capsules and a heel and a toe capsule per shoe; thighs and shanks stretch with the stride.",
       "Vorder- und Draufsicht sind gegliedert. Vorderansicht: Kopf mit Haar, Hals, Rumpf, Hüfte, je Seite Oberarm, Unterarm und Fäustlingshand, Oberschenkel, Unterschenkel und Schuh, dazu eine Manschettenellipse, die einen nach vorn gehobenen Unterarm sichtbar hält. Draufsicht: Kopf mit Haarsichel, Schultern, Rumpf, Hüfte, Kapseln für Oberarm, Unterarm und Hand, Kapseln für Ober- und Unterschenkel sowie je Schuh eine Ferse und eine Spitze; Ober- und Unterschenkel dehnen sich mit dem Schritt."),
 50:("Front and plan are fully articulated. Front: face with nose and mouth line, hair cap, ears, neck, torso with placket, hips, arms with cuffs, mitten hands, thighs and shanks with trouser creases and shoes, all on their own bones. Plan: face, hair, ears and nose, shoulders, trunk, hips, arms, forearms, hands with thumbs, thigh and shank capsules and heel and toe capsules.",
      "Vorder- und Draufsicht sind voll gegliedert. Vorderansicht: Gesicht mit Nasen- und Mundlinie, Haarkappe, Ohren, Hals, Rumpf mit Knopfleiste, Hüfte, Arme mit Manschetten, Fäustlingshände, Ober- und Unterschenkel mit Bügelfalten und Schuhe, jeweils an eigenem Knochen. Draufsicht: Gesicht, Haar, Ohren und Nase, Schultern, Rumpf, Hüfte, Arme, Unterarme, Hände mit Daumen, Kapseln für Ober- und Unterschenkel sowie Fersen- und Zehenkapseln."),
}
def joined(sc,en,de):
    return (NOTES[sc][0]+' '+en).strip(),(NOTES[sc][1]+' '+de).strip()
