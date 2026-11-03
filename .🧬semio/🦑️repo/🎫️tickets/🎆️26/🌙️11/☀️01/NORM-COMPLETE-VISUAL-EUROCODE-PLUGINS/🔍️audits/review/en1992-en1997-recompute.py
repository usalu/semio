import math
def hdr(t): print("\n== "+t)

# ---- EN 1992 spec examples (DE NA) ----
fck=30; fcd=0.85*fck/1.5; fyd=500/1.15
hdr("E-01")
mu=250e6/(300*450**2*fcd); om=1-math.sqrt(1-2*mu); z=450*(1-om/2)
print(round(fcd,2),round(fyd,2),round(mu,4),round(om,4),round(z,1),round(250e6/(z*fyd),0))
hdr("E-03 shear with links (DE)")
bw,d,z=300,450,405
vrdcc=0.5*0.48*fck**(1/3)*bw*z
ved=200e3
cot=min(3.0,1.2/(1-vrdcc/ved)); nu1=0.75*min(1.1-fck/500,1.0)
print(round(vrdcc/1e3,1),round(cot,3), round(bw*z*nu1*fcd/(cot+1/cot)/1e3,1), round(ved/(z*fyd*cot)*1e3,1))
hdr("E-04 VRd,c (DE)")
k=1+math.sqrt(200/450); rho=1257/(300*450)
v=0.15/1.5*k*(100*rho*fck)**(1/3); vmin=0.0525/1.5*k**1.5*math.sqrt(fck)
print(round(k,3),round(rho,5),round(v,4),round(vmin,4),round(max(v,vmin)*300*450/1e3,1))
hdr("E-05 punching interior")
d=210;u1=4*400+2*math.pi*2*d; ved=1.10*700e3/(u1*d)
k=1+math.sqrt(200/d); vrdc=0.18/1.5*k*(100*0.01*fck)**(1/3)
print(round(u1,1),round(ved,4),round(vrdc,4),round(ved/vrdc,3))
fywd_ef=250+0.25*d; asw=(ved-0.75*vrdc)*u1*d/(1.5*(d/(0.75*d))*fywd_ef)
print("asw",round(asw,1),"vmax DE 1.4*vrdc",round(1.4*vrdc,4))
uout=1.1*700e3/(vrdc*d); print("uout",round(uout),"rout",round((uout-1600)/(2*math.pi)))
hdr("E-06 torsion")
A=150000;u=1600;tef=A/u;Ak=(300-tef)*(500-tef);uk=2*((300-tef)+(500-tef))
print(tef,round(Ak),uk, round(2*0.75*fcd*Ak*tef*0.5/1e6,1))
hdr("E-07 anchorage/lap")
fctd=0.85*2.0/1.5; fbd=2.25*fctd; lb=20/4*fyd/fbd; a2=1-0.15*(45-20)/20
a6=min(1.5,math.sqrt(50/25)); print(round(fctd,3),round(fbd,3),round(lb,1),a2,round(a2*lb,0),round(a2*a6*lb,0))
hdr("E-09 crack width")
b,h,dd=300,500,450; As=1257; ae=200/33
# x from b x^2/2 = ae As (d-x)
import math
A_=b/2; B_=ae*As; C_=-ae*As*dd
x=(-B_+math.sqrt(B_*B_-4*A_*C_))/(2*A_)
I2=b*x**3/3+ae*As*(dd-x)**2; ss=ae*150e6*(dd-x)/I2
hc=min(2.5*(h-dd),(h-x)/3,h/2); rp=As/(b*hc)
eps=max((ss-0.4*2.9/rp*(1+ae*rp))/200000,0.6*ss/200000); sr=3.4*40+0.8*0.5*0.425*20/rp
print(round(x,1),round(ss,1),round(hc,1),round(rp,5),round(eps*1e3,3),round(sr,1),round(sr*eps,3))
hdr("E-09b l/d")
s=math.sqrt(fck);r0=s*1e-3;r=0.0033
print(round(11+1.5*s*r0/r+3.2*s*(r0/r-1)**1.5,2), round(11+1.5*s*r0/0.010,2))
hdr("E-10 creep/shrink")
fcm=38;a1=(35/fcm)**0.7;a2_=(35/fcm)**0.2;h0=187.5
phirh=(1+(1-0.5)/(0.1*h0**(1/3))*a1)*a2_; phi0=phirh*16.8/math.sqrt(fcm)/(0.1+28**0.2)
print(round(phirh,4),round(phi0,3))
brh=1.55*(1-0.5**3);ecd0=0.85*(220+110*4)*math.exp(-0.12*fcm/10)*1e-6*brh
kh=1-(h0-100)/100*0.15; print(round(ecd0*1e6,1),round(kh,3),round(kh*ecd0*1e6,1),2.5*(30-10))
hdr("E-11 column")
Ac=400*400;As=4*490.87;N0=Ac*fcd+As*fyd;print(round(N0/1e3,0),round(-As*fyd/1e3,1))
n=1500e3/N0; a=1+0.5*(n-0.1)/0.6; print(round(n,4),round(a,3),round((150/235.6)**a+(120/235.6)**a,3))
lam=4000/(400/math.sqrt(12));nn=1500e3/(Ac*fcd);om_=As*fyd/(Ac*fcd)
lim=20*0.7*math.sqrt(1+2*om_)*0.7/math.sqrt(nn);print(round(lam,1),round(nn,4),round(om_,4),round(lim,2))
eyd=fyd/2e5;dd2=350;r0=eyd/(0.45*dd2);nu=1+om_;Kr=(nu-nn)/(nu-0.4);e2=Kr*r0*4000**2/math.pi**2
print(r0,round(Kr,3),round(e2,2),round(1500*(40+10+e2)/1e3,1))
hdr("E-13 prestress")
print(round(math.exp(-0.19*(0.15+0.005*20)),4), round(0.66*2.5*math.exp(9.1*0.7)*(500000/1000)**(0.75*0.3)*1e-5,4))
hdr("E-14 fatigue")
rd=162.5/1.15;print(round(rd,1),round(120/rd,3),round(1e6*(rd/200)**5),round(1e5/(1e6*(rd/200)**5),3))
hdr("E-15 anchor cone")
n0=8.9*math.sqrt(30)*100**1.5; ratio=(100+150)*300/90000; ps=0.7+0.3*100/150
print(round(n0/1e3,2),round(ratio,4),ps,round(n0*ratio*ps/1e3,2),round(n0*ratio*ps/1.5/1e3,2),round(25/(n0*ratio*ps/1.5/1e3),3))
hdr("E-16 partial area; E-17 interface")
print(0.0)
fctd=0.85*2.0/1.5
nu=0.6*(1-30/250); print(round(0.40*fctd+0.002*fyd*0.7,3), round(0.5*nu*fcd,2))
hdr("E-12 fire (spec labels Table 5.5 col.2 but values are Table 5.6 continuous col.1)")
SS=[[(30,80,25),(60,120,40),(90,150,55),(120,200,65),(180,240,80),(240,280,90)],
    [(30,120,20),(60,160,35),(90,200,45),(120,240,60),(180,300,70),(240,350,80)],
    [(30,160,15),(60,200,30),(90,300,40),(120,300,55),(180,400,65),(240,500,75)],
    [(30,200,15),(60,300,25),(90,400,35),(120,500,50),(180,600,60),(240,700,70)]]
best=0
for col in SS:
    for r,bm,am in col:
        if 300>=bm and 38>=am: best=max(best,r)
print("code beam_ss R_achieved for b=300,a=38:",best,"(spec expects 90 -> continuous-beam table)")

# ---- code defects reproduced numerically ----
hdr("DEFECT: shear_capacity_n asw unit (x1e-3 instead of x1e3)")
asw_per_s=2*math.pi*4**2/1e6/0.15   # m2/m, 2-leg phi8 s=150 -> m
asw_mm=asw_per_s*1e3                 # correct mm2/mm
asw_code=asw_per_s*1e-3
zm=405;fywd=434.78;cot=2.5
print("correct Vrd,s kN",round(asw_mm*zm*fywd*cot/1e3,1)," code Vrd,s N",round(asw_code*zm*fywd*cot,3))
hdr("DEFECT: decompression sigma_c always >= 0 (|e|)")
P=1000e3;e=-100;Ac=300*500;I=300*500**3/12
print("code sigma_c (|e|)",P/Ac+P*abs(e)*250/I," top fibre true",P/Ac-P*abs(e)*250/I)
hdr("DEFECT: M-09 sign: -(fpd+...) vs signum on increment")
fpd=1600;fpt=1760;ep=195000;eps=-25; eud=20
ey=fpd/ep*1000
code=fpd+(fpt/1.0-fpd)*((min(abs(eps),eud)/1000-fpd/ep)/(eud/1000-fpd/ep))*(-1)
right=-(fpd+(fpt-fpd)*((min(abs(eps),eud)/1000-fpd/ep)/(eud/1000-fpd/ep)))
print("code",round(code,1),"correct",round(right,1))
hdr("Psi values EN 1990 A1.1")
print("storage (E) psi0/1/2 = 1.0/0.9/0.8 ; code 0.7/0.7/0.6; C congregation 0.7/0.7/0.6 ; code 0.7/0.5/0.3; snow>1000m 0.7/0.5/0.2")
hdr("Design effects: second order min ecc etc skipped")
