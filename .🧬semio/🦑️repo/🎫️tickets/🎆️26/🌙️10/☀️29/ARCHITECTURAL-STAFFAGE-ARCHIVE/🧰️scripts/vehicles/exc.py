import math
R=math.radians
def rot(a,x,y): c,s=math.cos(R(a)),math.sin(R(a)); return (c*x-s*y, s*x+c*y)
P0=(1.0,1.65); BOOM_REST=40.0; BOOM_L=5.7; STICK_REST=-90.0; STICK_L=2.9; BUCK_REST=30.0; BUCK_L=1.35
B1=(0.45,1.10); E1=(1.8,-0.45)
S1=(3.0,0.95); S2=(0.9,0.38)
K1=(1.25,0.36); K2=(0.15,0.36)
def fk(db,ds,dk):
    ab=BOOM_REST+db; asx=ab+STICK_REST+ds; ak=asx+BUCK_REST+dk
    P1=tuple(map(sum,zip(P0,rot(ab,BOOM_L,0)))); P2=tuple(map(sum,zip(P1,rot(asx,STICK_L,0)))); tip=tuple(map(sum,zip(P2,rot(ak,BUCK_L,0))))
    return ab,asx,ak,P1,P2,tip
def cyl(base,eye):
    dx,dy=eye[0]-base[0],eye[1]-base[1]; return math.hypot(dx,dy), math.degrees(math.atan2(dy,dx))
def solve(db,ds,dk):
    ab,asx,ak,P1,P2,tip=fk(db,ds,dk)
    e1=tuple(map(sum,zip(P0,rot(ab,*E1)))); d1,a1=cyl(B1,e1)
    s1=tuple(map(sum,zip(P0,rot(ab,*S1)))); s2=tuple(map(sum,zip(P1,rot(asx,*S2)))); d2,a2=cyl(s1,s2)
    k1=tuple(map(sum,zip(P1,rot(asx,*K1)))); k2=tuple(map(sum,zip(P2,rot(ak,*K2)))); d3,a3=cyl(k1,k2)
    return dict(tip=tip,P1=P1,P2=P2,d=(d1,d2,d3),barrel=(a1-0,a2-ab,a3-asx),absang=(a1,a2,a3),ab=ab,asx=asx,ak=ak)
if __name__=="__main__":
    import sys
    for db in (-75,-40,0,45):
        for ds in (-50,0,70):
            for dk in (-100,-30,65):
                r=solve(db,ds,dk); print(db,ds,dk,"tip(%.2f,%.2f)"%r["tip"],"d=%.2f %.2f %.2f"%r["d"])
