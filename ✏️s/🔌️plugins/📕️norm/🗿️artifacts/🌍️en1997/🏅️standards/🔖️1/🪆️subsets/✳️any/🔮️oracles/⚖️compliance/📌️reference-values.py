import math as m
r=m.radians
def ka(phi,d,a=0,b=0):
    p,dd,al,be=r(phi),r(d),r(a),r(b)
    return m.cos(p-al)**2/(m.cos(al)**2*m.cos(al+dd)*(1+m.sqrt(m.sin(p+dd)*m.sin(p-be)/(m.cos(al+dd)*m.cos(al-be))))**2)
def kp(phi,d,a=0,b=0):
    p,dd,al,be=r(phi),r(d),r(a),r(b)
    return m.cos(p+al)**2/(m.cos(al)**2*m.cos(dd-al)*(1-m.sqrt(m.sin(p+dd)*m.sin(p+be)/(m.cos(dd-al)*m.cos(al-be))))**2)
nq=lambda f:m.exp(m.pi*m.tan(r(f)))*m.tan(r(45+f/2))**2
nc=lambda f:(nq(f)-1)/m.tan(r(f))
ng=lambda f:2*(nq(f)-1)*m.tan(r(f))
sq=1+m.sin(r(30)); print("Ka(30,20)=%.5f Ka(30,0)=%.5f"%(ka(30,20),ka(30,0)))
print("Nq,Nc,Ng %.3f %.3f %.3f"%(nq(30),nc(30),ng(30)))
print("sc(30,B/L=1)=%.4f old=%.4f"%((sq*nq(30)-1)/(nq(30)-1),(sq*nq(30)-1)/nc(30)))
R=0.5*19*5**2*ka(30,20)+ka(30,20)*10*5; print("Ea=%.2f Eah=%.2f Eav=%.2f"%(R,R*m.cos(r(20)),R*m.sin(r(20))))
print("EX97-01 mu=%.4f"%(555/((18*nq(30)+0.5*18*2*ng(30))*2/1.4)))
xi=(1533.333/1.15,1450/1.05); print("EX97-12 Rck=%.2f Rcd=%.2f"%(min(xi),min(xi)/1.1))
print("Kp Rankine 30 = %.3f; Coulomb d=-15 (unsafe) %.3f"%(kp(30,0),kp(30,-15)))
