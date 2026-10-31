import sys,copy
from body import *
from thermal import *
A1=load_v1("jogger")['asset']
s=0.4447/0.4447
pose0=next(p for p in A1['poses'] if p['id']=="initial-contact-l")
W0=fk(A1,pose0['bones'],pose_root(A1,pose0))
P=dict(s=1.0,W=W0,
 torso=dict(region="torso-clothing",tone=0.8),pelvis=dict(region="leg-clothing",tone=0.22),
 sleeve=dict(region="arm-clothing",tone=0.8,x1=0.1448),arm_skin="arm-skin",fsleeve=None,hand="hand-skin",
 thigh_g=dict(region="leg-clothing",tone=0.22,x1=0.2068),leg_skin="leg-skin",shank_g=None,sock=dict(region="leg-skin",tone=0.95,x0=0.3413),
 shoe=dict(region="shoes",tone=0.92),head="head-skin",hair=dict(region="hair",tone=0.15),
 reg_fig=["head-skin","hair","torso-clothing","leg-clothing"],reg_uarm=["arm-clothing","arm-skin"],reg_farm=["arm-skin","hand-skin"],reg_thigh=["leg-clothing","leg-skin"],reg_shank=["leg-skin"])
def reps():
    r={}
    for sc in (100,200):
        r[sc]=build_body(sc,P)
    r[500]=merged500(P,pose0['bones'],None)
    drop=[f"{p}-{sd}" for p in ("upper-arm","upper-arm-skin","forearm","thigh","thigh-skin","shank","sock") for sd in "lr"]+["torso","pelvis","armband","hair","hair-strands"]
    add=["torso","pelvis","hair"]+[f"{p}-{sd}" for p in ("upper-arm","sleeve","forearm","thigh","thigh-skin","shank","sock") for sd in "lr"]
    r[50]=v50(A1,P,set(drop),add)
    return r
if __name__=="__main__":
    R=reps(); A=copy.deepcopy(A1)
    A['representations']={}
    pens={500:PENS(0.15,None,None,None,0.2),200:PENS(0.18,0.13,None,None,0.25),100:PENS(0.2,0.13,0.13,0.13,0.3),50:PENS(0.25,0.18,0.13,0.13,0.35)}
    for k in R: A['representations'][str(k)]={"geometry":{"elevation":R[k]},"pens":pens[k],"hatches":[]}
    for pid in sys.argv[1:] or ["initial-contact-l","midstance-l","flight-l"]:
        p=next(p for p in A['poses'] if p['id']==pid)
        for k in (500,200,100,50):
            try: print(pid,k,[round(v,3) for v in bbox(A,str(k),p['bones'],pose_root(A,p))])
            except Exception as e: print(e)
            render(A,str(k),p['bones'],pose_root(A,p),f"{GEN}/jog-{pid}-{k}.png",pxmm={500:80,200:50,100:30,50:20}[k],crop=(-0.6,0.9,-0.05,1.95))
