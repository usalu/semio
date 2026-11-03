import math
r=math.radians
def hdr(t): print("\n== "+t)
def nq(p): return math.exp(math.pi*math.tan(r(p)))*math.tan(math.pi/4+r(p)/2)**2
hdr("EX97-01/03/04")
p=30;NQ=nq(p);NC=(NQ-1)/math.tan(r(p));NG=2*(NQ-1)*math.tan(r(p));print(round(NQ,3),round(NC,3),round(NG,3))
Rk=18*NQ+0.5*18*2*NG;print(round(Rk,2),"R_d/m",round(Rk*2/1.4,1),"mu",round(555/(Rk*2/1.4),4))
Bp,Lp=1.9,3.2;ra=Bp/Lp;sq=1+ra*math.sin(r(p));sg=1-0.3*ra;sc=(sq*NQ-1)/(NQ-1)
q=19*1.2;R=5*NC*sc+q*NQ*sq+0.5*19*Bp*NG*sg;print(round(sq,5),round(sg,5),round(sc,5),round(R,2),round(R*Bp*Lp,1))
A=Bp*Lp;t=1-200/(1500+A*5/math.tan(r(p)));m=(2+ra)/(1+ra)
iq=t**m;ig=t**(m+1);ic=iq-(1-iq)/(NC*math.tan(r(p)));print(round(t,5),round(m,4),round(iq,5),round(ig,5),round(ic,5))
R=5*NC*sc*ic+q*NQ*sq*iq+0.5*19*Bp*NG*sg*ig;print(round(R,2),round(R*A,1))
hdr("EX97-11 Coulomb Ka")
def ka(phi,d,a,b):
    p,d,a,b=map(r,(phi,d,a,b))
    root=math.sqrt(math.sin(p+d)*math.sin(p-b)/(math.cos(a+d)*math.cos(a-b)))
    return math.cos(p-a)**2/(math.cos(a)**2*math.cos(a+d)*(1+root)**2)
k=ka(30,20,0,0);print(round(k,5),round(.5*19*25*k+k*10*5,2),round((.5*19*25*k+k*10*5)*math.cos(r(20)),2),round((.5*19*25*k+k*10*5)*math.sin(r(20)),2))
hdr("Kp plane Coulomb vs log-spiral claim (delta=phi=30): plane Coulomb is unsafe above ~phi/3")
def kp(phi,d):
    p,d=map(r,(phi,d)); root=math.sqrt(math.sin(p+d)*math.sin(p)/math.cos(d)); return math.cos(p)**2/(math.cos(d)*(1-root)**2)
print([round(kp(30,x),2) for x in (0,10,20,30)])
print("Caquot-Kerisel/EC7 Annex C (phi=30, delta/phi=1, beta=0) Kp ~ 6.4-6.5 ; code log-spiral 6.93 (+8%) ; DIN 4085 limits |delta_p|<=2/3 phi")
hdr("Newmark / Boussinesq")
def corner(m,n):
    s=m*m+n*n+1;pp=m*m*n*n
    return (2*m*n*math.sqrt(s)/(s+pp)*(s+1)/s+math.atan2(2*m*n*math.sqrt(s),s-pp))/(4*math.pi)
print("square B=L=2, z=2 centre factor",round(4*corner(0.5,0.5),4)," (textbook 0.336 at z/B=1)")
print("circle a=1.5,z=1:",round(200*(1-(1/(1+(1.5/1)**2))**1.5),2))
hdr("Settlement influence flexible rectangle centre vs table (L/B=1: s=1.12 pB/E for nu=0)")
m=1;I=(math.log((1+math.sqrt(1+m*m))/m)+m*math.log((m+math.sqrt(1+m*m))/1))
print(round(4*0.5*I/math.pi,4),"-> 2*I/pi for square centre (nu=0): 1.122")
hdr("Bishop pore-water bias (project-level submerged weights in drive)")
gam,gsat,gw=19,20,9.81
print("driving weight used (gamma'):",gsat-gw," vs total:",gsat," -> drive underestimated by",round(100*(1-(gsat-gw)/gsat),1),"% in wet part")
hdr("UPL fallback inverted")
for gwl in (0,2,5): print("groundwater depth",gwl,"m ->code pore",9.81*gwl,"kPa; correct head above base 5 m: ",9.81*max(5-gwl,0))
hdr("EN1997 Outcome::utilization zero capacity")
print("capacity=0, demand=100 -> util 0.0 (pass) in en1997 catalogue; en1992 returns inf")
hdr("pile R1 type factors EN Tables A.6-A.8")
print("driven R1 b/s/t=1.0/1.0/1.25(tension); bored R1 b=1.25 s=1.0 t=1.25 ; CFA R1 b=1.2 s=1.0 t=1.25 ; code R1 st=1.0, b=s=1.0")
