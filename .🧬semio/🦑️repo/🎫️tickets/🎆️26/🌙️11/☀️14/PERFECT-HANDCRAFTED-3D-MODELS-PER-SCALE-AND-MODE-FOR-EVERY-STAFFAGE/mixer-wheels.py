"""🛞 Author the mixer’s moulded tyre channels and its eight ten-stud hub fittings."""
from math import cos, pi, sin, sqrt


def wheel_nut(part):
    hand=1 if "-l" in part["id"] else -1
    z=hand*(1.138 if part["id"].split("-")[2] in ("1","2") else .934)
    vertices=[]
    faces=[]
    for height,corner,flat in [(0,.022,.022),(.001,.023,.023),(.0045,.023,.023),(.006,.032/sqrt(3),.016),(.025,.032/sqrt(3),.016),(.030,.0168,.01455)]:
        for i in range(12):
            angle=i*pi/6
            r=flat if i%2 else corner
            vertices.append([round(.1675+r*cos(angle),8),round(r*sin(angle),8),round(z+hand*height,8)])
        start=len(vertices)-12
        if start:
            for j in range(12):
                k=(j+1)%12
                faces.extend([[start-12+j,start-12+k,start+j],[start-12+k,start+k,start+j]])
    lower=len(vertices)
    vertices.extend([[.1675,0,z],[.1675,0,z+hand*.030]])
    for i in range(12):
        k=(i+1)%12
        faces.extend([[lower,k,i],[lower+1,60+i,60+k]])
    for key in ("origin","axis","profile"):part.pop(key,None)
    part.update(kind="polyhedron",material="galvanised-steel",vertices=vertices,triangles=faces if hand>0 else [f[::-1] for f in faces])
    part["repeat"]["rotationRad"]=2*pi/10


def update_wheels(volumes):
    fine=volumes["50"]
    fine["body"]=[p for p in fine["body"] if not p["id"].startswith("tread-groove-")]
    for tyre in fine["body"]:
        if tyre["id"].startswith("wheel-nuts-"):
            wheel_nut(tyre)
        if not tyre["id"].startswith("tyre-"):continue
        steer=tyre["id"].split("-")[1] in ("1","2")
        mouth,floor,depth=(.014,.008,.0105) if steer else (.018,.009,.0135)
        profile=tyre["profile"]
        start=next(i for i,p in enumerate(profile) if p==[-.118,.52])
        end=next(i for i,p in enumerate(profile) if p==[.118,.52])
        grooves=[[round(c+x,6),r] for c in (-.066,0,.066) for x,r in ((-mouth/2,.52),(-floor/2,.52-depth),(floor/2,.52-depth),(mouth/2,.52))]
        tyre["profile"]=profile[:start+1]+grooves+profile[end:]
    note={"en":"The four steer tyres use three 14 mm circumferential grooves, 10.5 mm deep. The eight tyres on the driven tandem have wider 18 mm grooves, 13.5 mm deep. Each external wheel face carries ten flanged hexagonal nuts, 32 mm across flats, on a 335 mm bolt circle.","de":"Die vier Lenkreifen haben drei 14 mm breite, 10,5 mm tiefe Umfangsrillen. In die acht Reifen des angetriebenen Doppelachsaggregats sind 18 mm breite und 13,5 mm tiefe Rillen geformt. Jede äußere Radschüssel trägt zehn Sechskantbundmuttern mit 32 mm Schlüsselweite auf einem Lochkreis von 335 mm."}
    if note not in fine["decisions"]:fine["decisions"].append(note)
