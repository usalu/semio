from lib2 import *
A="pendant-lamp"
def build(sc,asset):
    el=asset["representations"][str(sc)]["geometry"]["elevation"]
    F=[to_world(asset,s) for s in el]
    P=[]
    if sc==50:
        P.append(pl("plan-shade","shade",ring(0,0,0.133,48),z=3.4,tone=0.4,region="shade",material="enamel-white"))
        P.append(pl("plan-shade-rim","shade",ring(0,0,0.1235,44),role="detail",z=3.45,tone=0.7,filled=False))
        P.append(pl("plan-rose","rose",ring(0,0,0.05,32),z=3.6,tone=0.7,region="rose",material="rose-polymer"))
        P.append(pl("plan-rose-face","rose",ring(0,0,0.032,28),role="detail",z=3.7,tone=0.45,filled=False))
        dec={"silhouette":("Front view: the lamp is turned about its vertical axis, so the front view repeats the side view, redrawn from the same rotationally symmetric parts. Top view: the 0.266 m shade disc under the 0.1 m ceiling rose, which hides the cord, the lampholder and the bulb.",
                           "Vorderansicht: die Leuchte ist um ihre senkrechte Achse drehsymmetrisch, die Vorderansicht wiederholt daher die Seitenansicht, aus denselben drehsymmetrischen Teilen neu gezeichnet. Draufsicht: die Schirmscheibe von 0,266 m unter der Deckenrosette von 0,1 m, die Kabel, Fassung und Lampe verdeckt."),
             "omitted":("The top view omits cord, lampholder and bulb, which hide under the rose and the shade.",
                        "Die Draufsicht lässt Kabel, Fassung und Lampe weg, die unter Rosette und Schirm verdeckt sind.")}
    elif sc==100:
        P.append(pl("plan-shade","shade",ring(0,0,0.133,40),z=3.4,tone=0.4,region="shade",material="enamel-white"))
        P.append(pl("plan-rose","rose",ring(0,0,0.05,28),z=3.6,tone=0.7,region="rose",material="rose-polymer"))
        dec={"silhouette":("Front view: the rotationally symmetric side view redrawn. Top view: the 0.266 m shade disc under the 0.1 m rose.",
                           "Vorderansicht: die drehsymmetrische Seitenansicht neu gezeichnet. Draufsicht: die Schirmscheibe von 0,266 m unter der Rosette von 0,1 m."),
             "omitted":("The top view omits the rim line, cord, lampholder and bulb.",
                        "Die Draufsicht lässt Randlinie, Kabel, Fassung und Lampe weg.")}
    else:
        P.append(pl("plan-shade","shade",ring(0,0,0.133,32),z=3.4,tone=0.4,regions=["shade","lampholder"],material="enamel-white"))
        P.append(pl("plan-rose","rose",ring(0,0,0.05,24),z=3.6,tone=0.7,region="rose",material="rose-polymer"))
        dec={"silhouette":("Front view: the rotationally symmetric side view redrawn. Top view: the 0.266 m shade disc under the 0.1 m rose.",
                           "Vorderansicht: die drehsymmetrische Seitenansicht neu gezeichnet. Draufsicht: die Schirmscheibe von 0,266 m unter der Rosette von 0,1 m."),
             "omitted":("The top view omits cord and bulb.",
                        "Die Draufsicht lässt Kabel und Lampe weg.")}
    return F,P,dec
if __name__=="__main__": apply2(A,build)
