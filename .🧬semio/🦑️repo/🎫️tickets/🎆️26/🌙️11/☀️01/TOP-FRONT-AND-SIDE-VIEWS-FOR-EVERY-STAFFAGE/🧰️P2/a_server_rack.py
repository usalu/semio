from lib2 import *
A="server-rack"
RS="rack-steel"
def slots(x0,x1,ys,h):
    return " ".join(f"M {x0} {y} L {x1} {y} L {x1} {y+h} L {x0} {y+h} Z" for y in ys)
def build(sc,asset):
    F=[];P=[]
    if sc==50:
        F.append(rc("side-panel","frame",-0.5,0.1,0.5,1.967,z=3,tone=0.5,region="cabinet",material=RS))
        F.append(rc("roof","frame",-0.5,1.967,0.5,2.0,z=3.1,tone=0.45,region="cabinet",material=RS))
        F.append(rc("base-rail","frame",-0.5,0.07,0.5,0.1,z=3.1,tone=0.45,region="cabinet",material=RS))
        F.append(rc("panel-border","frame",-0.46,0.14,0.46,1.93,z=3.2,tone=0.35,role="detail",filled=False,r=0.01))
        F.append(line("panel-seam","frame",[(0,0.14),(0,1.93)],z=3.2,tone=0.35))
        F.append(pth("vents","frame",slots(0.22,0.42,[1.62+0.05*i for i in range(7)],0.012),z=3.3,tone=0.3,role="detail"))
        F.append(rc("door-edge","door",-0.5,0.1,-0.475,1.967,z=3.4,tone=0.18,region="door",material="mesh-steel"))
        for id_,x in (("caster-left",0.4),("caster-right",-0.4)): F.append(rc(id_,"feet",x-0.015,0,x+0.015,0.07,z=3,tone=0.2,region="feet",material="caster-rubber",r=0.012))
        P.append(rc("roof","frame",-0.3,-0.475,0.3,0.5,z=3,tone=0.45,region="cabinet",material=RS,r=0.01))
        P.append(rc("roof-seam","frame",-0.27,-0.445,0.27,0.47,z=3.1,tone=0.35,role="detail",filled=False,r=0.01))
        for i,lat in enumerate((0.1,0.32)): P.append(pl(f"fan-opening-{i+1}","frame",ring(0,lat,0.08,28),z=3.2,tone=0.15,role="detail"))
        P.append(rc("cable-slot","frame",-0.15,0.435,0.15,0.465,z=3.2,tone=0.2,role="detail",r=0.01))
        P.append(rc("door-field","door",-0.27,-0.5,0.27,-0.475,z=3.4,tone=0.18,region="door",material="mesh-steel"))
        dec={"silhouette":("Front view: the right-hand side panel across the 1 m depth, framed by roof and base rail, with a centre seam, a rear vent group, the 25 mm door edge on the right and the two front casters. Top view: the 0.6 m by 1 m roof with two fan openings, a cable slot and the door edge as a strip along the right-hand side.",
                           "Vorderansicht: die rechte Seitenwand über die Tiefe von 1 m, von Dach und Sockelschiene gerahmt, mit Mittelfuge, einer hinteren Lüftungsgruppe, der 25 mm starken Türkante rechts und den zwei vorderen Rollen. Draufsicht: das Dach von 0,6 m mal 1 m mit zwei Lüfteröffnungen, einem Kabelschlitz und der Türkante als Streifen an der rechten Seite."),
             "omitted":("Front and top view omit the servers, switch and blanking panels, which sit behind the side panel and the roof, and the rear casters.",
                        "Vorder- und Draufsicht lassen Server, Switch und Blenden weg, die hinter Seitenwand und Dach sitzen, außerdem die hinteren Rollen.")}
    elif sc==100:
        F.append(rc("side-panel","frame",-0.5,0.104,0.5,1.96,z=3,tone=0.5,region="cabinet",material=RS))
        F.append(rc("roof","frame",-0.5,1.96,0.5,2.0,z=3.1,tone=0.45,region="cabinet",material=RS))
        F.append(rc("base-rail","frame",-0.5,0.07,0.5,0.104,z=3.1,tone=0.45,region="cabinet",material=RS))
        F.append(line("panel-seam","frame",[(0,0.15),(0,1.9)],z=3.2,tone=0.35))
        F.append(rc("door-edge","door",-0.5,0.104,-0.47,1.96,z=3.4,tone=0.18,region="door",material="mesh-steel"))
        for id_,x in (("caster-left",0.4),("caster-right",-0.4)): F.append(rc(id_,"feet",x-0.02,0,x+0.02,0.07,z=3,tone=0.2,region="feet",material="caster-rubber",r=0.012))
        P.append(rc("roof","frame",-0.3,-0.47,0.3,0.5,z=3,tone=0.45,region="cabinet",material=RS,r=0.01))
        P.append(rc("cable-slot","frame",-0.15,0.43,0.15,0.47,z=3.2,tone=0.2,role="detail",r=0.01))
        P.append(rc("door-field","door",-0.266,-0.5,0.266,-0.47,z=3.4,tone=0.18,region="door",material="mesh-steel"))
        dec={"silhouette":("Front view: the side panel with roof, base rail, seam, the 30 mm door edge and two casters. Top view: the roof with a cable slot and the door strip on the right.",
                           "Vorderansicht: die Seitenwand mit Dach, Sockelschiene, Fuge, der 30 mm starken Türkante und zwei Rollen. Draufsicht: das Dach mit einem Kabelschlitz und der Türleiste rechts."),
             "omitted":("Front and top view omit panel border, vents and fan openings; the equipment is hidden behind the panel.",
                        "Vorder- und Draufsicht lassen Wandrahmen, Lüftung und Lüfteröffnungen weg; die Geräte sind hinter der Wand verdeckt.")}
    else:
        F.append(rc("side-panel","frame",-0.5,0.07,0.5,1.94,z=3,tone=0.5,region="cabinet",material=RS))
        F.append(rc("roof","frame",-0.5,1.94,0.5,2.0,z=3.1,tone=0.45,region="cabinet",material=RS))
        F.append(rc("door-edge","door",-0.5,0.12,-0.45,1.94,z=3.4,tone=0.18,region="door",material="mesh-steel"))
        for id_,x in (("caster-left",0.4),("caster-right",-0.4)): F.append(rc(id_,"feet",x-0.025,0,x+0.025,0.07,z=3,tone=0.2,region="feet",material="caster-rubber"))
        P.append(rc("roof","frame",-0.3,-0.45,0.3,0.5,z=3,tone=0.45,region="cabinet",material=RS))
        P.append(rc("door-field","door",-0.25,-0.5,0.25,-0.45,z=3.4,tone=0.18,region="door",material="mesh-steel"))
        dec={"silhouette":("Front view: the side panel block with roof band, a 50 mm door edge and two casters. Top view: the roof with the door strip on the right.",
                           "Vorderansicht: der Seitenwandblock mit Dachband, einer 50 mm starken Türkante und zwei Rollen. Draufsicht: das Dach mit der Türleiste rechts."),
             "omitted":("Front and top view omit all fine detail; the equipment is hidden.",
                        "Vorder- und Draufsicht lassen alle Feinheiten weg; die Geräte sind verdeckt.")}
    return F,P,dec
if __name__=="__main__": apply2(A,build)
