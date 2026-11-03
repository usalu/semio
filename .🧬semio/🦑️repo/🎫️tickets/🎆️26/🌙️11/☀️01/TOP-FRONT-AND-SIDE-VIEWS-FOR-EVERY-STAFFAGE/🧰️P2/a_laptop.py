from lib2 import *
A="laptop"
EPS=1.75
Y0=0.0125
def hook(asset):
    drop_rise(asset)
SPAN=0.213*math.sin(math.radians(EPS))
def lid_rows(n,r,x0,x1,tone,id_prefix="plan-lid"):
    w=0.313
    sp=(w-2*r)/(n-1)
    return [cap(f"{id_prefix}-{j+1}","lid",x0,-w/2+r+j*sp,x1,-w/2+r+j*sp,r,z=3.4,tone=tone,filled=True,role="body",followTip=True,region="lid",material="alu-anodised") for j in range(n)]
def build(sc,asset):
    F=[];P=[]
    if sc==50:
        for id_,x in (("foot-left",0.11),("foot-right",-0.11)): F.append(rc(id_,"base",x-0.01,0,x+0.01,0.002,z=3,tone=0.15,region="underside",material="rubber-foot"))
        F.append(pl("base","base",[(-0.1565,0.001),(0.1565,0.001),(0.1565,0.0092),(0.14,0.0125),(-0.14,0.0125),(-0.1565,0.0105)],z=3,tone=0.6,regions=["deck","underside"],material="alu-anodised"))
        F.append(ext(asset,sc,"lid-shell",-0.1565,0.1565,z=2.9))
        P.append(ext(asset,sc,"lid-shell",-0.1565,0.1565,id="plan-lid-shell",z=3.4,**({"x":-0.005,"w":0.218} if sc==50 else {})))
        F.append(ext(asset,sc,"display",-0.145,0.145,z=2.95))
        P.append(rc("plan-deck","base",-0.109,-0.1565,0.109,0.1565,z=3,tone=0.62,region="deck",material="alu-anodised",r=0.008))
        for i in range(5): P.append(rc(f"plan-key-row-{i+1}","base",-0.068+0.021*i,-0.115,-0.051+0.021*i,0.115,z=3.2,tone=0.3,role="detail"))
        P.append(rc("plan-trackpad","base",0.056,-0.04,0.092,0.04,z=3.2,tone=0.55,role="detail",filled=False,r=0.002))
        dec={"silhouette":("Front view: the 0.313 m wide base with its two front feet and chamfered front edge as one block of deck and underside, and the lid as a slab behind it whose top edge rises with the lid angle; the dark display fills its face. Top view: the base with five key rows and trackpad, covered by the lid, which draws back over the hinge as it opens.",
                           "Vorderansicht: die 0,313 m breite Basis mit ihren zwei vorderen Füßen und der abgeschrägten Vorderkante als ein Block aus Deck und Unterseite und der Deckel als Platte dahinter, deren Oberkante mit dem Deckelwinkel steigt; das dunkle Display füllt seine Fläche. Draufsicht: die Basis mit fünf Tastenreihen und Touchpad, vom Deckel bedeckt, der sich beim Öffnen über das Scharnier zurückzieht."),
             "omitted":("Front and top view omit the keycap relief, hinge barrels and the rear feet.",
                        "Vorder- und Draufsicht lassen das Tastenrelief, die Scharnierwalzen und die hinteren Füße weg.")}
    elif sc==100:
        F.append(rc("base-lower","base",-0.1565,0,0.1565,0.0066,z=3,tone=0.5,region="underside",material="alu-anodised",r=0.0012))
        F.append(pl("base-upper","base",[(-0.1565,0.0066),(0.1565,0.0066),(0.1565,0.0092),(0.14,0.0125),(-0.14,0.0125),(-0.1565,0.0105)],z=3.1,tone=0.62,region="deck",material="alu-anodised"))
        F.append(ext(asset,sc,"lid-shell",-0.1565,0.1565,z=2.9))
        P.append(ext(asset,sc,"lid-shell",-0.1565,0.1565,id="plan-lid-shell",z=3.4))
        P.append(rc("plan-deck","base",-0.109,-0.1565,0.109,0.1565,z=3,tone=0.62,region="deck",material="alu-anodised",r=0.008))
        P.append(rc("plan-keys","base",-0.07,-0.115,0.05,0.115,z=3.2,tone=0.3,role="detail"))
        dec={"silhouette":("Front view: base and a lid slab whose top edge rises with the lid angle. Top view: the base with one key field, covered by the lid, which draws back over the hinge as it opens.",
                           "Vorderansicht: Basis und eine Deckelplatte, deren Oberkante mit dem Deckelwinkel steigt. Draufsicht: die Basis mit einem Tastenfeld, vom Deckel bedeckt, der sich beim Öffnen über das Scharnier zurückzieht."),
             "omitted":("Front and top view omit the feet, the display, the key rows and the trackpad.",
                        "Vorder- und Draufsicht lassen Füße, Display, Tastenreihen und Touchpad weg.")}
    else:
        F.append(shp("base","base","rect",z=3,tone=0.55,regions=["deck","underside"],material="alu-anodised",x=-0.1565,y=0,w=0.313,h=0.0125,r=0.002))
        F.append(ext(asset,sc,"lid",-0.1565,0.1565,z=2.9))
        P.append(ext(asset,sc,"lid",-0.1565,0.1565,id="plan-lid",z=3.4))
        P.append(rc("plan-base","base",-0.109,-0.1565,0.109,0.1565,z=3,tone=0.55,regions=["deck","underside"],material="alu-anodised",r=0.004))
        dec={"silhouette":("Front view: base block and a lid slab raised with the lid angle. Top view: the base covered by the lid, which draws back over the hinge as it opens.",
                           "Vorderansicht: Basisblock und eine mit dem Deckelwinkel gehobene Deckelplatte. Draufsicht: die vom Deckel bedeckte Basis, wobei sich der Deckel beim Öffnen über das Scharnier zurückzieht."),
             "omitted":("Front and top view carry no detail beyond the two plates.",
                        "Vorder- und Draufsicht tragen keine Einzelheiten außer den zwei Platten.")}
    return F,P,dec
if __name__=="__main__": apply2(A,build,hook)
