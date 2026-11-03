from lib2 import *
A="toaster"
CH="chrome-steel"
def build(sc,asset):
    F=[];P=[]
    if sc==50:
        for id_,x in (("foot-left",0.06),("foot-right",-0.06)): F.append(rc(id_,"base",x-0.025,0,x+0.025,0.012,z=2.8,tone=0.15,region="feet",material="rubber-foot"))
        F.append(pth("slice-rear","slices",rr(0.0235,0.085,0.0365,0.196,tl=0.0065,tr=0.0065),z=2.4,tone=0.7,region="bread",material="bread-crust"))
        F.append(pth("slice-front","slices",rr(-0.0365,0.085,-0.0235,0.196,tl=0.0065,tr=0.0065),z=2.5,tone=0.78,region="bread",material="bread-crust"))
        F.append(pth("casing","base",rr(-0.085,0.012,0.085,0.188,0.022,0.022,0.007,0.007),z=3,tone=0.82,region="casing",material=CH))
        F.append(pl("top-plate","base",[(-0.071,0.172),(0.071,0.172),(0.057,0.188),(-0.057,0.188)],z=3.2,tone=0.55,region="top-plate",material="mica-steel"))
        F.append(line("seam-top","base",[(-0.071,0.172),(0.071,0.172)],z=3.3,tone=0.3,region="top-plate"))
        P.append(rc("casing","base",-0.15,-0.085,0.15,0.085,z=3,tone=0.82,region="casing",material=CH,r=0.022))
        P.append(rc("top-plate","base",-0.114,-0.057,0.114,0.057,z=3.2,tone=0.55,region="top-plate",material="mica-steel",r=0.01))
        P.append(rc("seam-top","base",-0.128,-0.071,0.128,0.071,z=3.3,tone=0.3,role="detail",filled=False,r=0.012))
        for i,lat in enumerate((0.03,-0.03)):
            P.append(rc(f"slot-{i+1}","base",-0.06,lat-0.0145,0.06,lat+0.0145,z=3.3,tone=0.12,region="top-plate",material="mica-steel",r=0.006))
        P.append(rc("slice-rear","slices",-0.049,0.0235,0.049,0.0365,z=3.4,tone=0.7,region="bread",material="bread-crust",r=0.004))
        P.append(rc("slice-front","slices",-0.052,-0.0365,0.052,-0.0235,z=3.45,tone=0.78,region="bread",material="bread-crust",r=0.004))
        dec={"silhouette":("Front view: the 0.17 m wide rounded body on two front feet, the bevelled top plate and the two bread slices, which stand 13 mm thick behind the top edge and rise with the carriage. Top view: the 0.3 m by 0.17 m body with the bevelled top plate and two slots in which the bread tops show.",
                           "Vorderansicht: der 0,17 m breite, gerundete Korpus auf zwei vorderen Füßen, die abgeschrägte Deckplatte und die zwei Brotscheiben, die 13 mm dick hinter der Oberkante stehen und mit dem Schlitten steigen. Draufsicht: der Korpus von 0,3 m mal 0,17 m mit der abgeschrägten Deckplatte und zwei Schlitzen, in denen die Brotoberseiten zu sehen sind."),
             "omitted":("Front and top view omit the lever, the dial and the cancel button, which sit on the right-hand face and are seen edge-on there, and the rear feet.",
                        "Vorder- und Draufsicht lassen Hebel, Drehknopf und Abbruchtaste weg, die auf der rechten Seitenfläche sitzen und dort nur von der Kante zu sehen wären, außerdem die hinteren Füße.")}
    elif sc==100:
        for id_,x in (("foot-left",0.06),("foot-right",-0.06)): F.append(rc(id_,"base",x-0.025,0,x+0.025,0.02,z=2.8,tone=0.15,region="feet",material="rubber-foot"))
        for id_,lat,z in (("slice-rear",0.03,2.4),("slice-front",-0.03,2.5)): F.append(pth(id_,"slices",rr(lat-0.011,0.085,lat+0.011,0.196,tl=0.011,tr=0.011),z=z,tone=0.78,region="bread",material="bread-crust"))
        F.append(pth("casing","base",rr(-0.085,0.02,0.085,0.188,0.022,0.022,0.008,0.008),z=3,tone=0.82,region="casing",material=CH))
        F.append(line("top-bevel","base",[(-0.071,0.172),(0.071,0.172)],z=3.3,tone=0.3,region="top-plate"))
        P.append(rc("casing","base",-0.15,-0.085,0.15,0.085,z=3,tone=0.82,region="casing",material=CH,r=0.022))
        P.append(rc("top-bevel","base",-0.128,-0.071,0.128,0.071,z=3.3,tone=0.3,role="detail",filled=False,r=0.012,region="top-plate"))
        for id_,lat,z in (("slice-rear",0.03,3.4),("slice-front",-0.03,3.45)): P.append(rc(id_,"slices",-0.052,lat-0.011,0.052,lat+0.011,z=z,tone=0.78,region="bread",material="bread-crust",r=0.005))
        dec={"silhouette":("Front view: the rounded body on two feet and the two bread slices thickened to 22 mm. Top view: the body with the bevel outline and the two bread tops.",
                           "Vorderansicht: der gerundete Korpus auf zwei Füßen und die zwei auf 22 mm verdickten Brotscheiben. Draufsicht: der Korpus mit der Fasenkontur und den zwei Brotoberseiten."),
             "omitted":("Front and top view omit the slots, the controls and the rear feet.",
                        "Vorder- und Draufsicht lassen die Schlitze, die Bedienelemente und die hinteren Füße weg.")}
    else:
        F.append(pth("slice","slices",rr(-0.061,0.1,0.061,0.196,tl=0.02,tr=0.02),z=2.5,tone=0.78,region="bread",material="bread-crust"))
        F.append(pth("casing","base",rr(-0.085,0,0.085,0.188,0.022,0.022,0,0),z=3,tone=0.82,regions=["casing","top-plate","control","feet"],material=CH))
        P.append(rc("casing","base",-0.15,-0.085,0.15,0.085,z=3,tone=0.82,regions=["casing","top-plate","control","feet"],material=CH,r=0.022))
        P.append(rc("slice","slices",-0.052,-0.045,0.052,0.045,z=3.4,tone=0.78,region="bread",material="bread-crust",r=0.01))
        dec={"silhouette":("Front view: one body block with the bread block rising above it as one 0.12 m wide mass. Top view: the body with one bread patch for both slices.",
                           "Vorderansicht: ein Korpusblock, über dem der Brotblock als eine 0,12 m breite Masse aufragt. Draufsicht: der Korpus mit einem Brotfeld für beide Scheiben."),
             "omitted":("Front and top view carry no slots, feet or controls.",
                        "Vorder- und Draufsicht tragen weder Schlitze noch Füße oder Bedienelemente.")}
    return F,P,dec
if __name__=="__main__": apply2(A,build)
