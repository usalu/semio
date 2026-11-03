import sys, json, time
sys.path.insert(0, ".")
import grid_torsion as g
def rect(w, h): return [(-w/2,-h/2),(w/2,-h/2),(w/2,h/2),(-w/2,h/2)]
def ipoly(b,h,tf,tw):
    return [(-b/2,-h/2),(b/2,-h/2),(b/2,-h/2+tf),(tw/2,-h/2+tf),(tw/2,h/2-tf),(b/2,h/2-tf),(b/2,h/2),(-b/2,h/2),(-b/2,h/2-tf),(-tw/2,h/2-tf),(-tw/2,-h/2+tf),(-b/2,-h/2+tf)]
cases = {
 "solid70x140": (rect(0.07,0.14), []),
 "box100x60x4": (rect(0.1,0.06), [rect(0.092,0.052)]),
 "I120": (ipoly(0.1,0.12,0.008,0.006), []),
}
def main():
    for k,(o,h) in cases.items():
        t=time.time(); r=g.solve(o,h); print(k, {a:(float(f"{v:.5g}") if isinstance(v,float) else v) for a,v in r.items()}, round(time.time()-t,2),"s")
if __name__=='__main__': main()

def hexa():
    import math
    R=0.05; o=[(R*math.cos(math.pi/3*i+0.2),R*math.sin(math.pi/3*i+0.2)) for i in range(6)]
    r=0.043; i=[(r*math.cos(math.pi/3*k+0.2),r*math.sin(math.pi/3*k+0.2)) for k in range(6)]
    return o,[i[::-1]]
cases["hex"]=hexa()
if __name__=='__main__': 
    pass
