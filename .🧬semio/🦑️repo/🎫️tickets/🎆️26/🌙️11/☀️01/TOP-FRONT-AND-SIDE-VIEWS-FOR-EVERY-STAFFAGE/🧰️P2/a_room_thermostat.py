from lib2 import *
A="room-thermostat"
def build(sc,asset):
    F=[];P=[]
    if sc==50:
        t0,t1,d1=-0.0015,0.0135,-0.0135
        F.append(rc("housing","housing",t0,-0.045,t1,0.045,z=3,tone=0.9,region="casing",material="abs-plastic",r=0.004))
        F.append(ext(asset,sc,"dial",d1,t0)); P.append(ext(asset,sc,"dial",d1,t0))
        P.append(rc("housing","housing",-0.045,t0,0.045,t1,z=3,tone=0.9,region="casing",material="abs-plastic",r=0.004))
        dec={"silhouette":("Front view: the 0.09 m high wall slab, 0.015 m thick, with the dial standing 0.012 m proud of its right-hand face; the front view of the wall side is on the left. Top view: the same slab 0.09 m wide with the dial 0.042 m wide.",
                           "Vorderansicht: die 0,09 m hohe Wandplatte von 0,015 m Dicke mit dem Drehknopf, der 0,012 m aus der rechten Fläche ragt; die Wandseite liegt links. Draufsicht: dieselbe Platte, 0,09 m breit, mit dem 0,042 m breiten Drehknopf."),
             "omitted":("Front and top view omit the display, the LED and the pointer, which lie on the right-hand face and are seen edge-on there.",
                        "Vorder- und Draufsicht lassen Anzeige, Leuchtdiode und Zeiger weg, die auf der rechten Fläche liegen und dort nur von der Kante zu sehen wären.")}
    else:
        t0,t1,d1=-0.0085,0.0135,-0.0305
        F.append(rc("housing","housing",t0,-0.045,t1,0.045,z=3,tone=0.9,regions=["casing","display","led"],material="abs-plastic",r=0.004))
        F.append(ext(asset,sc,"dial",d1,t0)); P.append(ext(asset,sc,"dial",d1,t0))
        P.append(rc("housing","housing",-0.045,t0,0.045,t1,z=3,tone=0.9,regions=["casing","display","led"],material="abs-plastic",r=0.004))
        dec={"silhouette":("Front view: the wall slab with the dial thickened to 0.022 m so it stays above the feature width. Top view: the slab 0.09 m wide with the 0.048 m dial in front of it.",
                           "Vorderansicht: die Wandplatte mit dem auf 0,022 m verdickten Drehknopf, damit er über der Mindestbreite bleibt. Draufsicht: die 0,09 m breite Platte mit dem 0,048 m breiten Drehknopf davor."),
             "omitted":("Front and top view show slab and dial only; the slab is 0.022 m thick instead of 0.015 m.",
                        "Vorder- und Draufsicht zeigen nur Platte und Drehknopf; die Platte ist 0,022 m statt 0,015 m dick.")}
    return F,P,dec
if __name__=="__main__": apply2(A,build)
