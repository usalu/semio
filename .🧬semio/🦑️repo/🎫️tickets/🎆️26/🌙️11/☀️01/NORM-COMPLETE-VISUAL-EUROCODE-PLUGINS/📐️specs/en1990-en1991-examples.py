import math
from statistics import NormalDist
N=NormalDist()
L10=math.log10
out=[]
def ex(i,t,*r):out.append((i,t,r))
# ---- EN1990
G=50;Q1=20;W=10;S=8
ex("E90-01","STR (6.10) office lead, DE psi: 1.35*50+1.5*20+1.5*0.6*10+1.5*0.5*8",1.35*G+1.5*Q1+1.5*.6*W+1.5*.5*S)
ex("E90-02","STR (6.10) wind lead",1.35*G+1.5*W+1.5*.7*Q1+1.5*.5*S)
ex("E90-03","STR (6.10) snow lead",1.35*G+1.5*S+1.5*.7*Q1+1.5*.6*W)
ex("E90-04","(6.10a) 1.35G+1.5*0.7*Q1+1.5*0.6W+1.5*0.5S [EN only]",1.35*G+1.5*.7*Q1+1.5*.6*W+1.5*.5*S)
ex("E90-05","(6.10b) 0.85*1.35G+1.5Q1+1.5*0.6W+1.5*0.5S [EN only]",.85*1.35*G+1.5*Q1+1.5*.6*W+1.5*.5*S)
# EQU
Gstb=400;Gdst=120;Qdst=60
ex("E90-06","EQU stabilising 0.9*400, destabilising 1.1*120+1.5*60",0.9*Gstb,1.1*Gdst+1.5*Qdst)
ex("E90-07","Char (office lead) 50+20+.6*10+.5*8 ; Freq (office lead) 50+.5*20+0 ; QP 50+.3*20",G+Q1+.6*W+.5*S, G+.5*Q1, G+.3*Q1)
ex("E90-08","Accidental Ad=100: 50+100+psi11*20 (office .5)+psi2 wind 0",G+100+.5*Q1)
ex("E90-09","Seismic AEd=40: 50+40+.3*20",G+40+.3*Q1)
for b,t in((3.3,50),(3.8,50),(4.3,50),(4.2,1),(4.7,1),(5.2,1)):ex("E90-10",f"pf=Phi(-{b})",N.cdf(-b))
ex("E90-11","KFI RC3: 1.1*1.5, 1.1*1.35",1.1*1.5,1.1*1.35)
V=.10;b=3.8;aR=.8
sig=math.sqrt(math.log(1+V*V));Rd_over_m=math.exp(-aR*b*sig)/math.sqrt(1+V*V)
Rk_over_m=math.exp(-1.645*sig)/math.sqrt(1+V*V)
ex("E90-12","lognormal V=.1 beta 3.8 aR .8: Rd/mean, Rk/mean, gammaR",Rd_over_m,Rk_over_m,Rk_over_m/Rd_over_m)
ex("E90-13","Annex D n=6, Vx unknown kn=2.18, mean 100 sd 8: Xk=m(1-kn V)",100*(1-2.18*.08))
ex("E90-14","Deflection 5qL^4/384EI q=10kN/m L=6 EI=1.5e4 kNm2 ; L/250",5*10*6**4/(384*15000)*1000,6000/250)
ex("E90-15","Bridge gr1a TS+UDL: 1.35*1000+1.35*(300+150)+1.5*.6*80",1.35*1000+1.35*450+1.5*.6*80)
# ---- EN1991-1-1
ex("E11-01","slab 0.20 m RC 25 kN/m3 + screed 0.06 m 22 kN/m3",.2*25,.06*22,.2*25+.06*22)
ex("E11-02","steel 77.0 vs 78.5 kN/m3 plate 10 mm kN/m2",.01*77,.01*78.5)
psi0=.7
ex("E11-03","alphaA=5/7*psi0+A0/A, A=40,A0=10",5/7*psi0+10/40)
ex("E11-04","alphaA A=10 -> capped 1.0; A=200 -> C-D floor 0.6",min(1,5/7*psi0+1),max(.6,5/7*psi0+10/200))
ex("E11-05","alphan n=5 psi0 .7",(2+(5-2)*psi0)/5)
wall=14*.115*2.6+0.4
ex("E11-06","wall line load: 14*0.115*2.6 + plaster 0.4 kN/m ; DE addition 1.2(<=5)",wall)
ex("E11-07","wheel 10 kN on 0.2x0.2 -> kN/m2",10/.04)
ex("E11-08","Balcony DE Z: 4.0 kN/m2*3.0 m depth*1 m",4*3)
ex("E11-09","Office B1 DE 2.0+partition 0.8 / 1.2 / 1.6 (g<=3/5/6)",2.0+.8,2+1.2,2+1.6)
ex("E11-10","fill soil 0.5 m x 20",.5*20)
# ---- EN1991-1-3 DE
def sk(zone,A):
    t=((A+140)/760)**2
    return {"1":max(.65,.19+.91*t),"2":max(.85,.25+1.91*t),"3":max(1.1,.31+2.91*t)}[zone[0]]*(1.25 if zone.endswith('a') else 1)
ex("E13-01","Zone 2 A=300: sk, s=0.8*sk",sk("2",300),.8*sk("2",300))
ex("E13-02","Zone 3 A=800",sk("3",800))
ex("E13-03","Zone 1a A=50 ; sAd=2.3*sk(zone 1, NGL)",sk("1a",50),2.3*sk("1",50))
ex("E13-04","Zone 1 A=900 ; zone 2a A=100",sk("1",900),sk("2a",100))
ex("E13-05","mu1(40deg)",.8*(60-40)/30)
ex("E13-06","multi-span alpha=30: mu2",.8+.8*30/30)
ex("E13-07","obstruction h=1 sk=.85: mu2 raw, capped, ls",2*1/.85,min(2.0,max(.8,2*1/.85)),max(5,min(15,2*1)))
d=.9/3;k=min(3/d,d*3)
ex("E13-08","overhang s=.9: d=s/3, k=min(3/d,d*gamma), se=k s^2/gamma",d,k,k*.9**2/3)
ex("E13-09","cylindrical b=12 h=2 mu3",min(2.0,.2+10*2/12))
V=.4;P=.01
ex("E13-10","Annex D V=.4 n=100",(1-V*(.45+.78*math.log(-math.log(1-P))))/(1+2.59*V))
ex("E13-11","snow guard Fs=s*b*sin(alpha) s=1.0 b=4 alpha=30",1.0*4*math.sin(math.radians(30)))
ex("E13-12","Ce windswept .8 Ct 1 mu1 .8 sk .85",.8*.8*1*.85)
# ---- EN1991-1-4
rho=1.25
def qb(vb):return .5*rho*vb**2
def qp_mix_bin(z,qb0):
    return 1.5*qb0 if z<=7 else 1.7*qb0*(z/10)**.37 if z<=50 else 2.1*qb0*(z/10)**.24
WZ={1:22.5,2:25,3:27.5,4:30}
ex("E14-01","DE vb0 WZ1..4 ; qb0 kN/m2",*[round(qb(WZ[i])/1000,4) for i in WZ])
ex("E14-02","Binnenland qp WZ2 z=5,10,30,100 kN/m2",*[round(qp_mix_bin(z,qb(25)/1000),4) for z in(5,10,30,100)])
def prof(z0,vb,z,co=1.0,kI=1.0):
    kr=.19*(z0/.05)**.07
    zmin={.003:1,.01:1,.05:2,.3:5,1.0:10}[z0]
    zz=max(z,zmin);cr=kr*math.log(zz/z0);vm=cr*co*vb
    Iv=kI/(co*math.log(zz/z0))
    return kr,cr,vm,Iv,(1+7*Iv)*.5*rho*vm**2/1000
ex("E14-03","EN cat III z0=.3 vb=25 z=10: kr,cr,vm,Iv,qp",*prof(.3,25,10))
ex("E14-04","EN cat II z0=.05 vb=25 z=10",*prof(.05,25,10))
ex("E14-05","EN cat I,0,IV z=10 vb=25 qp",prof(.01,25,10)[4],prof(.003,25,10)[4],prof(1.0,25,10)[4])
ex("E14-06","orography co=1+2*s*Phi, s=1 Phi=.15 ; qp cat II z=10",1+2*1*.15,prof(.05,25,10,co=1.3)[4])
h,d,b=12,20,30
e=min(b,2*h)
ex("E14-07","walls e=min(b,2h), A=e/5, B=d-e/5 (e>=d)",e,e/5,d-e/5)
hd=h/d
def interp(x,x0,x1,y0,y1):return y0+(y1-y0)*(x-x0)/(x1-x0)
cD=interp(hd,.25,1,.7,.8);cE=interp(hd,.25,1,-.3,-.5)
qz=prof(.05,25,12)[4]
ex("E14-08","h/d=.6 cpeD,cpeE ; qp(12) cat II ; we_D, we_E",cD,cE,qz,qz*cD,qz*cE)
ex("E14-09","flat roof e=24: F=e/4*e/10, G=(b-2*e/4)*e/10, H=(e/2-e/10)*b, I rest",e/4*e/10, (b-2*e/4)*e/10,(e/2-e/10)*b, b*d-2*(e/4*e/10)-(b-2*e/4)*e/10-(e/2-e/10)*b)
cp1,cp10=-2.5,-1.8
ex("E14-10","size effect F A=3 m2: cpe",cp1-(cp1-cp10)*L10(3),cp1-(cp1-cp10)*L10(1),cp1-(cp1-cp10)*L10(10))
ex("E14-11","cpi dominant: .75*cpe, .90*cpe cpe=-1.2",.75*-1.2,.9*-1.2)
# cscd
hh=40;bb=30;z0=.3;zs=max(.6*hh,5);vb=25
kr,cr,vm,Iv,_=prof(z0,vb,zs)
al=.67+.05*math.log(z0);Li=300*(zs/200)**al
B2=1/(1+.9*((bb+hh)/Li)**.63)
n1=46/hh;fL=n1*Li/vm;SL=6.8*fL/(1+10.2*fL)**(5/3)
def Rh(eta):return 1/eta-1/(2*eta**2)*(1-math.exp(-2*eta))
dl=.1;eh=4.6*hh/Li*fL*SL/fL if False else 4.6*hh/Li*n1*SL/n1
eh=4.6*hh/Li*fL*SL;eb=4.6*bb/Li*fL*SL
R2=math.pi**2/(2*dl)*SL*Rh(eh)*Rh(eb)
nu=max(n1*math.sqrt(R2/(B2+R2)),.08);T=600
kp=max(math.sqrt(2*math.log(nu*T))+.6/math.sqrt(2*math.log(nu*T)),3)
cscd=(1+2*kp*Iv*math.sqrt(B2+R2))/(1+7*Iv)
ex("E14-12","Annex B h=40 b=30 cat III vb 25 delta .1 : zs,Li,B2,n1,SL,R2,nu,kp,cscd",zs,Li,B2,n1,SL,R2,nu,kp,cscd)
def cprob(p,K=.2,n=.5):return ((1-K*math.log(-math.log(1-p)))/(1-K*math.log(-math.log(.98))))**n
ex("E14-13","cprob p=0.2 (5 yr),p=.1 (10 yr), p=.5 (2 yr)",cprob(.2),cprob(.1),cprob(.5))
bd=.5;n=1.5
ex("E14-14","vortex vcrit=b n/St, St=.18 ; Sc=2*ds*me/(rho b^2) ds=.05 me=300 kg/m",bd*n/.18,2*.05*300/(rho*bd**2))
ex("E14-15","Bridge FW=.5 rho vb^2 C Aref C=ce*cfx=2.0*1.3? vb=25 Aref=3*40 : kN",.5*rho*25**2*(2.0*1.3)*120/1000)
nu_air=15e-6;v=math.sqrt(2*qz*1000/rho)
ex("E14-16","Re=b v/nu b=.5 v(ze) from qp(12)",bd*v/nu_air)
# ---- EN1991-1-5
Tmin=-24;Tmax=37;T0=10
for ty,(dn,dx) in{"Type1 steel":(-3,16),"Type2 comp":(4,4),"Type3 conc":(8,2)}.items():
    a=12e-6 if ty!="Type3 conc" else 10e-6
    ex("E15-01",f"{ty}: Te,min Te,max dTcon dTexp dL100(mm) con,exp",Tmin+dn,Tmax+dx,T0-(Tmin+dn),Tmax+dx-T0,a*(T0-(Tmin+dn))*100000,a*(Tmax+dx-T0)*100000)
ex("E15-02","linear heating 18 K ksur .7 ; cooling 13 ksur 1.1(waterproof)",18*.7,13*1.1)
ex("E15-03","full restraint sigma=E a dT steel 210000*12e-6*40 ; conc 33000*10e-6*30",210000*12e-6*40,33000*10e-6*30)
ex("E15-04","combination dTM + .35 dTN ; dTN + .75 dTM  (dTM=18,dTN=43)",18+.35*43,43+.75*18)
ex("E15-05","restrained bowing M=E I a dTM/h  E=210000 MPa I=1e9 mm4 h=1000 mm dT=18 kNm",210000*1e9*12e-6*18/1000/1e6)
ex("E15-06","joint movement 100 m steel dTN exp 43 ,psi: ",12e-6*43*100000)
ex("E15-07","altitude correction Tmin 0.5K/100m @ 800m [V]",Tmin-.5*8)
ex("E15-08","K to C, T0 defaults",273.15+10)
# ---- EN1991-1-6
ex("E16-01","Qca 1.0 kN/m2 on 3x3",1.0*9)
ex("E16-02","fresh concrete hydrostatic 25*0.6 ; + Qca .75",25*.6,25*.6+.75)
ex("E16-03","cprob 3 months ~ 5 yr; wind vb scaling",25*cprob(.2),qb(25*cprob(.2))/1000)
ex("E16-04","Qcb storage 0.2 kN/m2 *area 50",.2*50)
ex("E16-05","stack of 4 pallets 1.2*... bricks 2.0 kN/m2*10 m2",2.0*10)
ex("E16-06","combination Qc,sum = 1.0*(Qca)+1.0*(Qcb)",1.0+.2)
ex("E16-07","Qcf temporary partial structure self weight 150 kN * psi 1",150)
ex("E16-08","2yr snow factor via Annex D V=.4 P=.5",(1-.4*(.45+.78*math.log(-math.log(1-.5))))/(1+2.59*.4))
# ---- EN1991-1-7
ex("E17-01","Impact urban 750/375 cars 50/25",750,375,50,25)
s,Lh=6.0,7.2;g=5;q=3
ex("E17-02","ties peripheral .4(g+.3q)sL ; internal .8(...)sL ; 75 cap least",.4*(g+.3*q)*s*Lh,.8*(g+.3*q)*s*Lh,min(.4*(g+.3*q)*s*Lh,75),min(.8*(g+.3*q)*s*Lh,75))
ex("E17-03","notional removal area min(15% of 400, 100)",min(.15*400,100))
ex("E17-04","dynamic impact F=v sqrt(k m) v=15 m/s k=3e5 N/m m=1500 kg kN",15*math.sqrt(3e5*1500)/1000)
pv=5;AvV=.04
ex("E17-05","explosion pd D1 3+pv ; D2 3+pv/2+.04/(Av/V)^2",3+pv,3+pv/2+.04/AvV**2)
ex("E17-06","key element Ad=34 kN/m2 * 3 m *6 m",34*3*6)
ex("E17-07","impact on deck 500/250; pier 1000/500 [V]",500,250)
ex("E17-08","impact energy .5 m v^2 : 1500 kg 15 m/s kJ",.5*1500*15**2/1000)
# ---- EN1991-1-2
iso=lambda t:20+345*L10(8*t+1)
ex("E12-01","ISO 834 t=15,30,60,90,120",*[round(iso(t),1) for t in(15,30,60,90,120)])
hc=lambda t:20+1080*(1-.325*math.exp(-.167*t)-.675*math.exp(-2.5*t))
ex("E12-02","hydrocarbon t=5,30,60",*[round(hc(t),1) for t in(5,30,60)])
ext=lambda t:20+660*(1-.687*math.exp(-.32*t)-.313*math.exp(-3.8*t))
ex("E12-03","external t=5,30,60",*[round(ext(t),1) for t in(5,30,60)])
Af=50;Av=5;heq=2.0;At=2*50+4*(5*3)-0;At=2*Af+ (4*7.07*3)
Af=50;Av=5;heq=2.0;At=2*50+4*7.07*3
O=Av*math.sqrt(heq)/At
bb_=1160;Gam=(O/bb_)**2/(.04/1160)**2
qfd=420*.8*1.0*1.0*1.0*1.0  # placeholder
qt=qfd*Af/At
tlim=20/60
tmax=max(.2e-3*qt*1000/O/60*60/60 if False else .2e-3*qt/O,tlim)
ts=lambda t:t*Gam
gp=lambda t:20+1325*(1-.324*math.exp(-.2*ts(t))-.204*math.exp(-1.7*ts(t))-.472*math.exp(-19*ts(t)))
ex("E12-04","parametric Af=50 Av=5 heq=2 At=2*50+4*7.07*3 b=1160: O,Gamma,qtd,tmax(h),theta_max",O,Gam,qt,tmax,gp(tmax))
Qfk=420;m=.8;dq1=1.5;dq2=1.0;dn=1.0
ex("E12-05","qfd=qfk*m*dq1*dq2*dn office 250 m2",Qfk*m*dq1*dq2*dn)
hnet=lambda tg,tm:25*(tg-tm)+1.0*.8*1.0*5.67e-8*((tg+273)**4-(tm+273)**4)
ex("E12-06","hnet ISO 30min tg=841.8 tm=500 W/m2",hnet(iso(30),500))
ex("E12-07","eta_fi Gk=100 Qk=50 psi11=.5",(100+.5*50)/(1.35*100+1.5*50))
af=50;wf=lambda H,av,ah:max(.5,(6/H)**.3*(.62+90*(.4-av)**4/(1+(max(10,12.5*(1+10*av-av**2)))*ah)))
av=.1;ah=0
ex("E12-08","te,d=qfd*kb*wf  kb=.07 H=3 av=.1 ah=0",wf(3,av,ah),Qfk*m*dq1*dq2*dn*.07*wf(3,av,ah))
Dd=1.0;Qr=2e6
Lf=-1.02*Dd+.0148*Qr**.4;z0=-1.02*Dd+.00524*Qr**.4
ex("E12-09","Heskestad D=1 m Q=2e6 W: Lf,z0,theta(z=3 m)",Lf,z0,min(900,20+.25*(.8*Qr)**(2/3)*(3-z0)**(-5/3)))
# ---- EN1991-2
def lanes(w):
    if w<5.4:return 1,3.0,w-3
    if w<6:return 2,w/2,0
    n=int(w//3);return n,3.0,w-3*n
ex("E2-01","lanes w=5.0,5.5,6.0,10.5",lanes(5.0),lanes(5.5),lanes(6.0),lanes(10.5))
ex("E2-02","LM1 3 lanes: tandems 2*(300+200+100); UDL kN/m for 3 m lanes",2*(300+200+100),9*3,2.5*3,2.5*3,"rest 1.5 m: 2.5*1.5=",2.5*1.5)
L=40
ex("E2-03","braking Qlk=.6*2*300+.1*9*3*L  [180,900]",.6*2*300+.1*9*3*L)
Qv=2*(300+200+100)
ex("E2-04","centrifugal 40Qv/r r=300 ; 0.2Qv r<200 r=150",40*Qv/300,.2*Qv)
ex("E2-05","footbridge qfk=2+120/(L+30) L=50 ; clamps L=5,300",2+120/80,min(5,max(2.5,2+120/35)),min(5,max(2.5,2+120/330)))
LP=20
ex("E2-06","LM71 phi2,phi3 L_phi=20",1.44/(math.sqrt(LP)-.2)+.82,2.16/(math.sqrt(LP)-.2)+.73)
v=160;Lf=30;ff=1-(v-120)/1000*((814/v)+1.75)*(1-math.sqrt(2.88/Lf))
Qvk=1000
ex("E2-07","rail centrifugal v=160 r=1000 Lf=30: f,Qtk=v^2/(127r)*f*Qvk",ff,v**2/(127*1000)*ff*Qvk)
ex("E2-08","traction 33*30 ; braking 20*30 (LM71)",33*30,20*30)
ex("E2-09","SW/0 total 2 spans 15 m: 133*15 ; LM71 UDL 80 kN/m",133*15,80)
ex("E2-10","road gr1a 1.35*(TS 2*(300)... ) crowd 5 kN/m2 *3 m wide",5*3)
ex("E2-11","LM2 400 kN axle wheel 0.35x0.6: pressure kN/m2 per wheel 200 kN",200/(.35*.6))
ex("E2-12","LM1 wheel 150 kN on .4x.4 pressure",150/.16)
# ---- EN1991-3
phi2=lambda mn,b2,vh:mn+b2*vh
ex("E3-01","phi2 HC1..4 vh=.25: min+b2*vh",phi2(1.05,.17,.25),phi2(1.10,.34,.25),phi2(1.15,.51,.25),phi2(1.20,.68,.25))
ex("E3-02","phi3 = 1-dm/m(1+b3) dm/m=.5 b3=.5(grab)=?, b3=1.0 ",1-.5*(1+.5),1-.5*(1+1.0))
ex("E3-03","hoist load Q_H=100 kN * phi2(HC2)",100*phi2(1.10,.34,.25))
ex("E3-04","self weight phi1=1.1 *300 kN ; 0.9*300",1.1*300,.9*300)
ex("E3-05","drive K=mu*SumQr 0.2*400 ; HL=phi5*K/nr=1.5*80/2",.2*400,1.5*.2*400/2)
ex("E3-06","skew f=.3(1-exp(-250a)) a=.015",.3*(1-math.exp(-250*.015)))
ex("E3-07","buffer HB=phi7*v1*sqrt(mc*SB) phi7=1.25 v1=.7 mc=20000kg SB=2e6 N/m kN",1.25*.7*math.sqrt(20000*2e6)/1000)
ex("E3-08","transverse HT=xi*M/a",.1*500/ 2)
ex("E3-09","wheel Qr,max group1 300*1.1(phi1)+ (Q_H share) ",1.1*300/2+ 100*1.185/2)
# ---- EN1991-4
gm=9;phi=30;mu=.4;Kj=1.1*(1-math.sin(math.radians(phi)));dd=6;AU=dd/4
z0j=AU/(Kj*mu);pho=gm*Kj*z0j
for z in(2,10,20):
    Yj=1-math.exp(-z/z0j);phf=pho*Yj
    ex("E4-01",f"Janssen z={z}: K,z0,pho,Yj,phf,pwf,pvf",Kj,z0j,pho,Yj,phf,mu*phf,phf/Kj)
ex("E4-02","slenderness hc/dc 12/6=2 -> slender (>=2); 8/6 intermediate; 4/6 squat; <.4 retaining",12/6,8/6,4/6)
ex("E4-03","Rankine active K=(1-sin)/(1+sin)",(1-math.sin(math.radians(30)))/(1+math.sin(math.radians(30))))
ex("E4-04","mass held pi/4 d^2 h gamma/g ->t : d=6 h=12 gamma=9",math.pi/4*36*12*9/9.81)
ex("E4-05","AAC by capacity t : 100 t, 10000 t thresholds",100,10000)
ex("E4-06","wall friction angle phiw=.75*phi ; mu=tan",math.tan(math.radians(.75*phi)))
ex("E4-07","hydrostatic tank gamma 10 z 5",50)
ex("E4-08","wall friction per m perimeter at z=10: mu*pho*(z-z0(1-e^(-z/z0))) == gamma*A/U*z - A/U*pvf",mu*pho*(10-z0j*(1-math.exp(-10/z0j))),9*AU*10-AU*(pho*(1-math.exp(-10/z0j))/Kj))
def fm(v):
    if isinstance(v,float):return f"{v:.6g}"
    if isinstance(v,tuple):return "("+", ".join(fm(i) for i in v)+")"
    return str(v)
for i,t,r in out:print(f"| {i} | {t} | "+"; ".join(fm(x) for x in r)+" |")
