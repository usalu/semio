from lib import *
A="hot-water-cylinder"
ZONES=[("zone-4",0.06,3,0.9),("zone-3",0.39,3.01,0.88),("zone-2",0.72,3.02,0.86),("zone-1",1.05,3.03,0.84)]
def dome(n,r=0.3,h=0.1,y0=1.38):
    return [(r*math.cos(math.pi*i/n),y0+h*math.sin(math.pi*i/n)) for i in range(n+1)]
def build(sc,asset):
    F=[];P=[]
    if sc==50:
        for x,id_ in [(0.18,"foot-l"),(-0.25,"foot-r")]: F.append(shp(id_,"base","rect",z=2.9,tone=0.35,x=x,y=0,w=0.07,h=0.06))
        F.append(shp("base-ring","base","rect",z=2.95,tone=0.4,x=-0.28,y=0.04,w=0.56,h=0.025))
        for id_,y0,z,t in ZONES: F.append(shp(id_,id_,"rect",z=z,tone=t,region=id_,x=-0.3,y=y0,w=0.6,h=0.33))
        F.append(shp("cap","cap","poly",z=3.1,tone=0.78,region="top-cap",material="cap-plastic",points=dome(24)))
        F.append(shp("cap-seam","cap","poly",role="detail",z=3.15,tone=0.4,filled=False,closed=False,points=[(0.2476,1.4365),(0.1809,1.4598),(0.0955,1.4748),(0,1.48),(-0.0955,1.4748),(-0.1809,1.4598),(-0.2476,1.4365)]))
        F.append(shp("zip","zone-3","poly",role="detail",z=3.3,tone=0.4,filled=False,closed=False,points=[(-0.204,0.42),(-0.204,0.69)]))
        for i,y in ((3,0.9107),(4,0.4982)):
            F.append(shp(f"nut-{i}","connections","poly",role="detail",z=3.25,tone=0.45,region="connections",points=ring(0,y+0.012,0.019,6,math.pi/6)))
            F.append(shp(f"stub-{i}","connections","poly",role="detail",z=3.3,tone=0.7,region="connections",points=ring(0,y+0.012,0.012,16)))
        F.append(shp("hot-out","connections","rect",role="detail",z=3.2,tone=0.7,region="connections",x=-0.015,y=1.43,w=0.03,h=0.048))
        P.append(shp("plan-cap","cap","poly",z=3.1,tone=0.78,region="top-cap",material="cap-plastic",points=ring(0,0,0.3,48)))
        P.append(shp("plan-cap-seam","cap","poly",role="detail",z=3.15,tone=0.4,filled=False,points=ring(0,0,0.2547,40)))
        for i,x,nx in ((0,-0.33,-0.33),(3,0.3,0.318)):
            P.append(shp(f"stub-{i}","connections","rect",role="detail",z=3.0,tone=0.7,region="connections",x=x,y=-0.012,w=0.03,h=0.024))
            P.append(shp(f"nut-{i}","connections","rect",role="detail",z=3.05,tone=0.45,region="connections",x=nx,y=-0.017,w=0.012,h=0.034))
        P.append(shp("hot-out","connections","poly",role="detail",z=3.3,tone=0.7,region="connections",points=ring(0.2039,0,0.015,16)))
        dec={"silhouette":("Front view: the 0.6 m cylinder with its two feet, four stacked shells, the dome with its seam, the zip seam on the right-hand side and the two front stubs with union nuts seen end-on on the axis. Top view: the 0.6 m dome with its seam ring, the hot-water stub and the rear and the front stub with union nuts.",
                           "Vorderansicht: der 0,6 m breite Zylinder mit seinen zwei Füßen, den vier übereinanderliegenden Schalen, der Wölbung mit ihrer Fuge, der Reißverschlussnaht auf der rechten Seite und den beiden vorderen Stutzen mit Überwurfmuttern in der Stirnansicht auf der Achse. Draufsicht: die 0,6 m große Wölbung mit ihrem Fugenring, der Warmwasserstutzen sowie der hintere und der vordere Stutzen mit Überwurfmuttern."),
             "omitted":("Front and top view leave out the thermometer and the label, which sit on the right side face and are seen edge-on there, and the rear stubs behind the shells; the top view also omits the feet and shells below the dome.",
                        "Vorder- und Draufsicht lassen Thermometer und Schild weg, die auf der rechten Seitenfläche sitzen und dort nur von der Kante zu sehen wären, außerdem die hinteren Stutzen hinter den Schalen; die Draufsicht lässt zudem Füße und Schalen unter der Wölbung weg.")}
    elif sc==100:
        F.append(shp("feet","base","rect",z=2.9,tone=0.35,x=-0.25,y=0,w=0.5,h=0.06))
        for id_,y0,z,t in ZONES: F.append(shp(id_,id_,"rect",z=z,tone=t,region=id_,x=-0.3,y=y0,w=0.6,h=0.33))
        F.append(shp("cap","cap","poly",z=3.1,tone=0.78,region="top-cap",material="cap-plastic",points=dome(18)))
        F.append(shp("stub-3","connections","rect",role="detail",z=3.2,tone=0.7,region="connections",x=-0.02,y=0.9107,w=0.04,h=0.024))
        F.append(shp("stub-4","connections","rect",role="detail",z=3.2,tone=0.7,region="connections",x=-0.02,y=0.4982,w=0.04,h=0.024))
        F.append(shp("hot-out","connections","rect",role="detail",z=3.2,tone=0.7,region="connections",x=-0.02,y=1.43,w=0.04,h=0.048))
        P.append(shp("plan-cap","cap","poly",z=3.1,tone=0.78,region="top-cap",material="cap-plastic",points=ring(0,0,0.3,36)))
        P.append(shp("stub-0","connections","rect",role="detail",z=3.0,tone=0.7,region="connections",x=-0.33,y=-0.02,w=0.04,h=0.04))
        P.append(shp("stub-3","connections","rect",role="detail",z=3.0,tone=0.7,region="connections",x=0.29,y=-0.02,w=0.04,h=0.04))
        P.append(shp("hot-out","connections","poly",role="detail",z=3.3,tone=0.7,region="connections",points=ring(0.2039,0,0.02,12)))
        dec={"silhouette":("Front view: the same four shells, dome and foot bar across 0.6 m with the two front stubs and the hot-water stub seen end-on. Top view: the 0.6 m dome disc with a rear and a front stub and the hot-water stub.",
                           "Vorderansicht: dieselben vier Schalen, die Wölbung und der Fußbalken über 0,6 m, mit den zwei vorderen Stutzen und dem Warmwasserstutzen in der Stirnansicht. Draufsicht: die 0,6 m große Wölbungsscheibe mit einem hinteren und einem vorderen Stutzen und dem Warmwasserstutzen."),
             "omitted":("Front and top view omit the seams, the dial and the rear stubs; the stubs are thickened to 40 mm as in the side view.",
                        "Vorder- und Draufsicht lassen Fugen, Thermometer und hintere Stutzen weg; die Stutzen sind wie in der Seitenansicht auf 40 mm verdickt.")}
    else:
        F.append(shp("feet","base","rect",z=2.9,tone=0.35,x=-0.25,y=0,w=0.5,h=0.06))
        F.append(shp("lower","zone-4","rect",z=3,tone=0.9,regions=["zone-4","zone-3"],x=-0.3,y=0.06,w=0.6,h=0.66))
        F.append(shp("upper","zone-2","rect",z=3.01,tone=0.86,regions=["zone-2","zone-1"],x=-0.3,y=0.72,w=0.6,h=0.66))
        F.append(shp("cap","cap","poly",z=3.1,tone=0.78,region="top-cap",material="cap-plastic",points=dome(14)))
        F.append(shp("hot-out","connections","rect",role="detail",z=3.2,tone=0.7,region="connections",x=-0.025,y=1.43,w=0.05,h=0.05))
        F.append(shp("side-r","connections","rect",role="detail",z=3.2,tone=0.7,region="connections",x=-0.025,y=0.8988,w=0.05,h=0.05))
        P.append(shp("plan-cap","cap","poly",z=3.1,tone=0.78,region="top-cap",material="cap-plastic",points=ring(0,0,0.3,28)))
        P.append(shp("side-l","connections","rect",role="detail",z=3.2,tone=0.7,region="connections",x=-0.33,y=-0.025,w=0.04,h=0.05))
        P.append(shp("side-r","connections","rect",role="detail",z=3.2,tone=0.7,region="connections",x=0.29,y=-0.025,w=0.04,h=0.05))
        P.append(shp("hot-out","connections","rect",role="detail",z=3.3,tone=0.7,region="connections",x=0.1778,y=-0.025,w=0.05,h=0.05))
        dec={"silhouette":("Front view: two stacked halves and a dome across 0.6 m on the foot bar, with one thickened front stub and the hot-water stub. Top view: the 0.6 m dome disc with two thickened stubs and the hot-water stub.",
                           "Vorderansicht: zwei übereinanderliegende Hälften und eine Wölbung über 0,6 m auf dem Fußbalken, mit einem verdickten vorderen Stutzen und dem Warmwasserstutzen. Draufsicht: die 0,6 m große Wölbungsscheibe mit zwei verdickten Stutzen und dem Warmwasserstutzen."),
             "omitted":("Front and top view omit the other stubs, the seams, the dial and the base ring as in the side view.",
                        "Vorder- und Draufsicht lassen wie die Seitenansicht die übrigen Stutzen, die Fugen, das Thermometer und den Bodenring weg.")}
    if sc>50:
        for s in F+P:
            if s['role']=='detail': s['role']='body'
    return F,P,dec
if __name__=="__main__": apply(A,build)
