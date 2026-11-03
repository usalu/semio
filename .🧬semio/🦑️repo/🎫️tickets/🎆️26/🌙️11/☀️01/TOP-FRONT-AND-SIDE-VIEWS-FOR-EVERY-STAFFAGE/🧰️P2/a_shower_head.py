from lib2 import *
A="shower-head"
LEV_EPS=5
BULGE=0.13
def hook(asset):
    drop_rise(asset)
def hose(asset,sc,n,view):
    el={s["id"]:s for s in asset["representations"][str(sc)]["geometry"]["elevation"]}
    h=copy.deepcopy(el["hose"]); h.pop("d")
    pts=hose_samples(n); N=len(pts)
    out=[]
    for i,(x,y) in enumerate(pts):
        lat=-BULGE*math.sin(math.pi*i/(N-1))**1.5
        X=x+0.03; Y=y+0.04
        out.append([rnd(lat),rnd(Y)] if view=="front" else [rnd(X),rnd(lat)])
    h.update({"kind":"poly","points":out,"closed":False})
    h={k:h[k] for k in ("id","bone","role","z","filled","tone","kind","points","closed","region","material","penMm") if k in h}
    return h
def build(sc,asset):
    F=[hose(asset,sc,{50:12,100:8,200:5}[sc],"front")]
    P=[hose(asset,sc,{50:12,100:8,200:5}[sc],"plan")]
    F.append(ext(asset,sc,"handle",-{50:0.0165,100:0.02,200:0.026}[sc],{50:0.0165,100:0.02,200:0.026}[sc]))
    P.append(ext(asset,sc,"handle",-{50:0.0165,100:0.02,200:0.026}[sc],{50:0.0165,100:0.02,200:0.026}[sc]))
    F.append(ext(asset,sc,"head",-0.0685,0.0685))
    P.append(ext(asset,sc,"head",-0.0685,0.0685))
    if sc==50:
        F.append(ext(asset,sc,"lever",-0.041,-0.031))
        P.append(ext(asset,sc,"lever",-0.041,-0.031))
        F.append(rc("bracket-low","rail",-0.025,0.08,0.025,0.115,z=3.1,tone=0.6,region="rail",material="chrome"))
        F.append(rc("bracket-top","rail",-0.025,0.775,0.025,0.81,z=3.1,tone=0.6,region="rail",material="chrome"))
        F.append(rc("tube","rail",-0.011,0.1,0.011,0.79,z=3.2,tone=0.8,region="rail",material="chrome",r=0.008))
        F.append(rc("elbow","elbow",-0.03,0,0.03,0.08,z=3.3,tone=0.7,region="valve",material="chrome"))
        F.append(rc("holder","holder",-0.028,0.59,0.028,0.65,z=3.3,tone=0.7,region="rail",material="chrome",r=0.008))
        P.append(rc("bracket-top","rail",0,-0.025,0.05,0.025,z=3.1,tone=0.6,region="rail",material="chrome"))
        P.append(pl("tube","rail",ring(0.06,0,0.011,16),z=3.2,tone=0.8,region="rail",material="chrome"))
        P.append(rc("elbow","elbow",0,-0.03,0.06,0.03,z=3.3,tone=0.7,region="valve",material="chrome"))
        P.append(rc("holder","holder",0.062,-0.028,0.118,0.028,z=3.3,tone=0.7,region="rail",material="chrome",r=0.008))
        dec={"silhouette":("Front view: the rail with both brackets, the 0.06 m valve block with its lever as a short bar that rises with the lever angle, the holder, the handle and the 0.137 m wide shower head seen from the front, and the hose hanging as a loop that bulges 0.13 m to the right. Top view: the same parts from above, the hose bulging sideways as in the front view.",
                           "Vorderansicht: die Stange mit beiden Haltern, der 0,06 m breite Ventilblock mit seinem Hebel als kurzer Balken, der mit dem Hebelwinkel steigt, der Halter, der Griff und der 0,137 m breite Brausekopf von vorn und der als Schlaufe hängende Schlauch, der 0,13 m nach rechts ausbaucht. Draufsicht: dieselben Teile von oben, der Schlauch baucht wie in der Vorderansicht zur Seite aus."),
             "omitted":("Front and top view omit the lower bracket (hidden under the upper one in the top view), the lever tick and the spray face.",
                        "Vorder- und Draufsicht lassen den unteren Halter (in der Draufsicht unter dem oberen verdeckt), die Hebelmarke und die Sprühfläche weg.")}
    elif sc==100:
        F.append(rc("bracket-low","rail",-0.0275,0.075,0.0275,0.12,z=3.1,tone=0.6,region="rail",material="chrome"))
        F.append(rc("bracket-top","rail",-0.0275,0.765,0.0275,0.81,z=3.1,tone=0.6,region="rail",material="chrome"))
        F.append(rc("tube","rail",-0.015,0.1,0.015,0.79,z=3.2,tone=0.8,region="rail",material="chrome",r=0.01))
        F.append(rc("elbow","elbow",-0.033,0,0.033,0.085,z=3.3,tone=0.7,region="valve",material="chrome"))
        F.append(rc("holder","holder",-0.03,0.585,0.03,0.655,z=3.3,tone=0.7,region="rail",material="chrome",r=0.008))
        P.append(rc("bracket-top","rail",0,-0.0275,0.055,0.0275,z=3.1,tone=0.6,region="rail",material="chrome"))
        P.append(pl("tube","rail",ring(0.06,0,0.015,14),z=3.2,tone=0.8,region="rail",material="chrome"))
        P.append(rc("elbow","elbow",0,-0.033,0.065,0.033,z=3.3,tone=0.7,region="valve",material="chrome"))
        P.append(rc("holder","holder",0.06,-0.03,0.12,0.03,z=3.3,tone=0.7,region="rail",material="chrome",r=0.008))
        dec={"silhouette":("Front view: rail with brackets, valve block, holder, handle, the 0.137 m shower head and the sideways hose loop. Top view: the same parts from above.",
                           "Vorderansicht: Stange mit Haltern, Ventilblock, Halter, Griff, der 0,137 m breite Brausekopf und die seitliche Schlauchschlaufe. Draufsicht: dieselben Teile von oben."),
             "omitted":("Front and top view omit the lever, as in the side view, and the lower bracket in the top view.",
                        "Vorder- und Draufsicht lassen wie die Seitenansicht den Hebel weg und in der Draufsicht den unteren Halter.")}
    else:
        F.append(rc("tube","rail",-0.02,0,0.02,0.81,z=3.2,tone=0.8,regions=["rail","valve"],material="chrome"))
        P.append(rc("tube","rail",0.04,-0.02,0.08,0.02,z=3.2,tone=0.8,regions=["rail","valve"],material="chrome"))
        dec={"silhouette":("Front view: one 0.04 m rail column, the handle, the 0.137 m shower head and the sideways hose loop. Top view: the rail square, handle, head and hose loop.",
                           "Vorderansicht: eine 0,04 m breite Stangensäule, der Griff, der 0,137 m breite Brausekopf und die seitliche Schlauchschlaufe. Draufsicht: das Stangenquadrat, Griff, Kopf und Schlauchschlaufe."),
             "omitted":("Front and top view omit brackets, valve, holder and lever as in the side view.",
                        "Vorder- und Draufsicht lassen wie die Seitenansicht Halter, Ventil, Schiene und Hebel weg.")}
    return F,P,dec
if __name__=="__main__": apply2(A,build,hook)
