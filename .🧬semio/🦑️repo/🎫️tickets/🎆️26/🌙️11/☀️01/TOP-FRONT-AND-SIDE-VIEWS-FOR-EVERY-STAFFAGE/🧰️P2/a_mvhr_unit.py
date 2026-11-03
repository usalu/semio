from lib import *
A="mvhr-unit"
GALV="galvanised-steel"
CX=[-0.225,-0.075,0.075,0.225]
LAT=[0.07,-0.07,0.07,-0.07]
def build(sc,asset):
    F=[];P=[]
    LAT=[0.08,-0.08,0.08,-0.08] if sc==200 else [0.07,-0.07,0.07,-0.07]
    if sc==50:
        F.append(rc("casing","casing",-0.15,0,0.15,0.6,z=3,tone=0.82,region="casing",material="painted-steel",r=0.025))
        F.append(line("cover-seam","casing",[(-0.125,0.07),(-0.125,0.565),(0.125,0.565),(0.125,0.07),(-0.125,0.07)],z=3.3,tone=0.4))
        for i in (0,1):
            lat=LAT[i]
            F.append(rc(f"spigot-{i+1}","spigots",lat-0.049,0.6,lat+0.049,0.662,z=3.1,tone=0.66,region="ducts",material=GALV))
            F.append(rc(f"bead-{i+1}","spigots",lat-0.057,0.648,lat+0.057,0.662,z=3.2,tone=0.5,role="detail",region="ducts",material=GALV))
        P.append(rc("casing","casing",-0.3,-0.15,0.3,0.15,z=3,tone=0.82,region="casing",material="painted-steel",r=0.025))
        P.append(line("cover-seam","casing",[(-0.275,-0.125),(0.275,-0.125),(0.275,0.125),(-0.275,0.125),(-0.275,-0.125)],z=3.3,tone=0.4))
        for i in range(4):
            P.append(pl(f"spigot-{i+1}","spigots",ring(CX[i],LAT[i],0.049,28),z=3.1,tone=0.66,region="ducts",material=GALV))
            P.append(pl(f"bead-{i+1}","spigots",ring(CX[i],LAT[i],0.057,28),z=3.2,tone=0.5,role="detail",filled=False,region=None))
        dec={"silhouette":("Front view: the 0.3 m casing with its cover seam and the two spigots on the front side of the top, the other two hiding behind them. Top view: the 0.6 m by 0.3 m casing with the cover seam and the four round spigots with their beads in a zigzag, at a lateral offset of 0.07 m either side.",
                           "Vorderansicht: das 0,3 m tiefe Gehäuse mit seiner Deckelfuge und den zwei vorderen Stutzen oben, die beiden anderen verbergen sich dahinter. Draufsicht: das Gehäuse von 0,6 m mal 0,3 m mit der Deckelfuge und den vier runden Stutzen mit ihren Sicken im Zickzack, seitlich je 0,07 m versetzt."),
             "omitted":("Front and top view omit the impeller window, the LED and the handle, which sit on the side face and are seen edge-on there.",
                        "Vorder- und Draufsicht lassen das Laufradfenster, die Leuchtdiode und den Griff weg, die auf der Seitenfläche sitzen und dort nur von der Kante zu sehen wären.")}
    elif sc==100:
        F.append(rc("casing","casing",-0.15,0,0.15,0.6,z=3,tone=0.82,region="casing",material="painted-steel",r=0.025))
        for i in (0,1):
            lat=LAT[i]
            F.append(rc(f"spigot-{i+1}","spigots",lat-0.049,0.6,lat+0.049,0.662,z=3.1,tone=0.66,region="ducts",material=GALV))
        P.append(rc("casing","casing",-0.3,-0.15,0.3,0.15,z=3,tone=0.82,region="casing",material="painted-steel",r=0.025))
        for i in range(4):
            P.append(shp(f"spigot-{i+1}","spigots","ellipse",z=3.1,tone=0.66,region="ducts",material=GALV,cx=CX[i],cy=LAT[i],rx=0.049,ry=0.049))
        dec={"silhouette":("Front view: the 0.3 m casing with the two front spigots. Top view: the 0.6 m by 0.3 m casing with four round spigots in a zigzag.",
                           "Vorderansicht: das 0,3 m tiefe Gehäuse mit den zwei vorderen Stutzen. Draufsicht: das Gehäuse von 0,6 m mal 0,3 m mit vier runden Stutzen im Zickzack."),
             "omitted":("Front and top view omit the cover seam, the beads and, as in the side view, window, LED and handle.",
                        "Vorder- und Draufsicht lassen die Deckelfuge, die Sicken und wie in der Seitenansicht Fenster, Leuchtdiode und Griff weg.")}
    else:
        F.append(rc("casing","casing",-0.15,0,0.15,0.6,z=3,tone=0.82,region="casing",material="painted-steel",r=0.03))
        for i in (0,1):
            lat=LAT[i]
            F.append(rc(f"spigot-{i+1}","spigots",lat-0.045,0.6,lat+0.045,0.665,z=3.1,tone=0.62,region="ducts",material=GALV))
        P.append(rc("casing","casing",-0.3,-0.15,0.3,0.15,z=3,tone=0.82,region="casing",material="painted-steel",r=0.03))
        for i in range(4):
            P.append(rc(f"spigot-{i+1}","spigots",CX[i]-0.045,LAT[i]-0.045,CX[i]+0.045,LAT[i]+0.045,z=3.1,tone=0.62,region="ducts",material=GALV))
        dec={"silhouette":("Front view: a 0.3 m by 0.6 m block with two spigots on top. Top view: a 0.6 m by 0.3 m block with four square spigot marks.",
                           "Vorderansicht: ein Block von 0,3 m mal 0,6 m mit zwei Stutzen oben. Draufsicht: ein Block von 0,6 m mal 0,3 m mit vier quadratischen Stutzenmarken."),
             "omitted":("Front and top view omit beads, seams and the impeller as in the side view.",
                        "Vorder- und Draufsicht lassen wie die Seitenansicht Sicken, Fugen und Laufrad weg.")}
    if sc==100:
        for s in F:
            if s['role']=='detail': s['role']='body'
    return F,P,dec
if __name__=="__main__": apply(A,build)
