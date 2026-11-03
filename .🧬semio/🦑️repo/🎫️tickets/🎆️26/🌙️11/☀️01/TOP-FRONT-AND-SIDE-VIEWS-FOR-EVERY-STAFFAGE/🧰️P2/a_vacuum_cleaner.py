from lib2 import *
A="vacuum-cleaner"
HE=6
WE=4
Y0=0.125
def hook(asset):
    drop_rise(asset)
def at(el,id_):
    s=el[id_]; return {k:s[k] for k in ("region","regions","material") if k in s}
def hoses(asset,sc,r):
    F=[ext(asset,sc,f"hose-{i}-tube",-r,r) for i in range(1,11)]
    return F,[ext(asset,sc,f"hose-{i}-tube",-r,r) for i in range(1,11)]
def build(sc,asset):
    el={s["id"]:s for s in asset["representations"][str(sc)]["geometry"]["elevation"]}
    F,P=hoses(asset,sc,0.024 if sc==200 else 0.019)
    rw=0.024 if sc==200 else 0.019
    gr=0.03 if sc==200 else 0.026
    wr=0.024 if sc==200 else 0.0175
    F.append(ext(asset,sc,"wand-tube",-wr,wr,z=2.6)); P.append(ext(asset,sc,"wand-tube",-wr,wr,z=2.6))
    F.append(ext(asset,sc,"grip-sleeve",-gr,gr)); P.append(ext(asset,sc,"grip-sleeve",-gr,gr))
    F.append(ext(asset,sc,"nozzle",-0.14,0.14,z=2.4)); P.append(ext(asset,sc,"nozzle",-0.14,0.14,z=2.4))
    for z0,z1,side in ((0.12,0.15,"left"),(-0.15,-0.12,"right")):
        F.append(ext(asset,sc,"rear-wheel",z0,z1,id=f"wheel-{side}")); P.append(ext(asset,sc,"rear-wheel",z0,z1,id=f"wheel-{side}"))
    if sc<200:
        for z0,z1,side in ((0.04,0.06,"left"),(-0.06,-0.04,"right")): F.append(ext(asset,sc,"caster-wheel",z0,z1,id=f"caster-{side}"))
    if sc<200: F.append(ext(asset,sc,"brush",-0.14,0.14))
    if sc<200: F.append(ext(asset,sc,"cuff",-0.026,0.026)); P.append(ext(asset,sc,"cuff",-0.026,0.026))
    wheel_w=(0.12,0.15)
    if sc==50:
        P.append(pl("swivel","nozzle",ring(2.15,0,0.028,24),z=2.7,tone=0.5,region="nozzle",role="body"))
        P.append(line("nozzle-slot","nozzle",[(2.21,0),(2.32,0)],z=2.7,tone=0.1,region="nozzle"))
        S=el["shell"]; sh=at(el,"shell"); lid=at(el,"lid")
        F.append(pth("shell","body",rr(-0.13,0.045,0.13,0.2,0.03,0.03,0.05,0.05),role="body",z=1,tone=0.5,**sh))
        F.append(pth("lid","body","M -0.12 0.2 C -0.12 0.24 -0.07 0.268 0 0.268 C 0.07 0.268 0.12 0.24 0.12 0.2 Z",role="body",z=1.1,tone=0.62,**lid))
        F.append(pth("handle","body",rr(-0.04,0.266,0.04,0.305,0.012,0.012),role="body",z=1,tone=0.62,**lid))
        F.append(line("shell-seam","body",[(-0.125,0.12),(0.125,0.12)],z=1.2,tone=0.3,region="body-shell"))
        F.append(rc("front-bumper","body",-0.05,0.06,0.05,0.16,z=1.3,tone=0.25,region="body-shell",material="abs-grey",r=0.008))
        P.append(rc("shell","body",-0.252,-0.13,0.262,0.13,role="body",z=1,tone=0.5,r=0.06,**sh))
        P.append(rc("lid","body",-0.24,-0.118,0.258,0.118,role="body",z=1.1,tone=0.62,r=0.05,**lid))
        P.append(rc("handle","body",-0.058,-0.04,0.078,0.04,role="body",z=1.2,tone=0.62,r=0.015,**lid))
        P.append(shp("rewind-button","body","ellipse",z=1.3,tone=0.8,**{"region":"lid"},cx=0.175,cy=0,rx=0.014,ry=0.014))
        P.append(rc("front-bumper","body",0.245,-0.05,0.265,0.05,z=1.3,tone=0.25,region="body-shell",material="abs-grey",r=0.008))
        dec={"silhouette":("Front view: the 0.26 m body with its domed lid and handle between two 0.03 m wheels, the two small casters, then in front of it the hose segments as stretched bars that follow the hose joints, the wand bar between grip and nozzle, the round grip sleeve and the 0.28 m nozzle with its brush strip. Top view: the rounded body with lid, handle, rewind button and front bumper beside the two wheel strips, and the hose, wand and grip as bars along the centre line, ending in the 0.28 m by 0.28 m nozzle with its swivel and slot.",
                           "Vorderansicht: der 0,26 m breite Körper mit gewölbtem Deckel und Griff zwischen zwei 0,03 m breiten Rädern, die zwei kleinen Lenkrollen und davor die Schlauchstücke als gestreckte Balken, die den Schlauchgelenken folgen, der Rohrbalken zwischen Griff und Düse, die runde Griffhülse und die 0,28 m breite Düse mit ihrem Bürstenstreifen. Draufsicht: der abgerundete Körper mit Deckel, Griff, Aufrolltaste und vorderem Stoßfänger neben den zwei Radstreifen und Schlauch, Rohr und Griff als Balken auf der Mittellinie, bis zur 0,28 m mal 0,28 m großen Düse mit Drehgelenk und Schlitz."),
             "omitted":("Front and top view leave out the hose ribs and the wand joint (they cannot follow the bending hose across views), the air slide, the exhaust grille on the rear side face and the brush underside; the hose runs in the symmetry plane.",
                        "Vorder- und Draufsicht lassen die Schlauchrippen und die Rohrfuge (sie könnten dem gebogenen Schlauch nicht über die Ansichten folgen), den Nebenluftschieber, das Ausblasgitter an der hinteren Seitenfläche und die Bürstenunterseite weg; der Schlauch verläuft in der Symmetrieebene.")}
    elif sc==100:
        sh=at(el,"shell"); lid=at(el,"lid")
        F.append(pth("shell","body",rr(-0.13,0.045,0.13,0.2,0.03,0.03,0.05,0.05),role="body",z=1,tone=0.5,**sh))
        F.append(pth("lid","body","M -0.12 0.2 C -0.12 0.24 -0.07 0.268 0 0.268 C 0.07 0.268 0.12 0.24 0.12 0.2 Z",role="body",z=1.1,tone=0.62,**lid))
        F.append(pth("handle","body",rr(-0.04,0.266,0.04,0.305,0.012,0.012),role="body",z=1,tone=0.62,**lid))
        P.append(rc("shell","body",-0.252,-0.13,0.262,0.13,role="body",z=1,tone=0.5,r=0.06,**sh))
        P.append(rc("lid","body",-0.24,-0.118,0.258,0.118,role="body",z=1.1,tone=0.62,r=0.05,**lid))
        P.append(rc("handle","body",-0.058,-0.04,0.078,0.04,role="body",z=1.2,tone=0.62,r=0.015,**lid))
        dec={"silhouette":("Front view: body with lid and handle between the two wheels, casters, hose bars, wand bar, grip disc and the 0.28 m nozzle with brush strip. Top view: body with lid and handle, wheel strips, centre-line hose, wand and grip bars and the nozzle.",
                           "Vorderansicht: Körper mit Deckel und Griff zwischen den zwei Rädern, Lenkrollen, Schlauchbalken, Rohrbalken, Griffscheibe und die 0,28 m breite Düse mit Bürstenstreifen. Draufsicht: Körper mit Deckel und Griff, Radstreifen, Schlauch, Rohr- und Griffbalken auf der Mittellinie und die Düse."),
             "omitted":("Front and top view omit seam, bumper, rewind button, swivel, slot and the rear grille.",
                        "Vorder- und Draufsicht lassen Fuge, Stoßfänger, Aufrolltaste, Drehgelenk, Schlitz und das hintere Gitter weg.")}
    else:
        sh=at(el,"shell")
        F.append(pth("shell","body",rr(-0.13,0.045,0.13,0.268,0.08,0.08,0.05,0.05),role="body",z=1,tone=0.5,**sh))
        P.append(rc("shell","body",-0.252,-0.13,0.262,0.13,role="body",z=1,tone=0.5,r=0.06,**sh))
        dec={"silhouette":("Front view: one domed body block between two wheels, then hose bars, wand bar, grip disc and the 0.28 m nozzle block. Top view: the body, two wheel strips, the centre-line hose, wand and grip bars and the nozzle block.",
                           "Vorderansicht: ein gewölbter Körperblock zwischen zwei Rädern, davor Schlauchbalken, Rohrbalken, Griffscheibe und der 0,28 m breite Düsenblock. Draufsicht: der Körper, zwei Radstreifen, Schlauch, Rohr- und Griffbalken auf der Mittellinie und der Düsenblock."),
             "omitted":("Front and top view omit lid, handle, casters, cuff and every detail below 0.04 m.",
                        "Vorder- und Draufsicht lassen Deckel, Griff, Lenkrollen, Manschette und jede Einzelheit unter 0,04 m weg.")}
    return F,P,dec
if __name__=="__main__": apply2(A,build,hook)
