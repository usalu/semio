"""🌬️ Bind the coarse canopy clusters to the boughs beneath them while preserving their surveyed rest locations."""
import json, math, runpy
from pathlib import Path

h=runpy.run_path(str(Path(__file__).with_name("living-articulation.py")))
frames=json.loads((Path(__file__).parent/"🗑️generated/living-frames.json").read_text())

def dot(a,b):return sum(x*y for x,y in zip(a,b))
def multiply(a,b):
    x,y,z,w=a
    i,j,k,r=b
    return (w*i+x*r+y*k-z*j,w*j-x*k+y*r+z*i,w*k+x*j-y*i+z*r,w*r-x*i-y*j-z*k)
def quaternion(rows):
    d=[rows[0][0],rows[1][1],rows[2][2]]
    q=[0.0]*4
    pivot=max(range(4),key=lambda i:1+2*d[i]-sum(d) if i<3 else 1+sum(d))
    if pivot==3:
        q[3]=math.sqrt(1+sum(d))/2
        for i in range(3):q[i]=(rows[(i+2)%3][(i+1)%3]-rows[(i+1)%3][(i+2)%3])/(4*q[3])
    else:
        i=pivot;j=(i+1)%3;k=(i+2)%3
        q[i]=math.sqrt(1+2*d[i]-sum(d))/2
        q[j]=(rows[i][j]+rows[j][i])/(4*q[i]);q[k]=(rows[i][k]+rows[k][i])/(4*q[i])
        q[3]=(rows[k][j]-rows[j][k])/(4*q[i])
    return q
def rebind(f,s,target):
    if s["bone"]==target:return
    old,new=f[s["bone"]],f[target]
    axes=("u","v","w")
    R=[[dot(new[a],old[b]) for b in axes] for a in axes]
    pose=s.get("transform",{})
    t=pose.get("translation",[0,0,0])
    displacement=[i-j for i,j in zip(old["o"],new["o"])]
    transformed=[dot(row,t)+dot(new[axis],displacement) for row,axis in zip(R,axes)]
    q=multiply(quaternion(R),pose.get("rotation",[0,0,0,1]))
    assert abs(sum(x*x for x in q)-1)<0.001
    s["transform"]={**pose,"translation":[round(n,8) for n in transformed],"rotation":[round(n/math.sqrt(dot(q,q)),8) for n in q]}
    s["bone"]=target

apple={"e":"e1","n":"w2","w":"w1","s":"e2","ne":"e2","nw":"w2","sw":"w1","se":"e1","core":"core-low","top":"top"}
plane={"core":"core","top":"top","o-0":"e2","o-180":"w2","ol-90":"e1","ol-270":"w1","od-45":"e2","od-135":"w2","od-225":"w1","od-315":"e1","m-0":"e1","m-180":"w1","m-60":"e2","m-120":"w2","m-240":"w1","m-300":"e1","lo-0":"low-e","lo-180":"low-w","lo-90":"low-e","lo-270":"low-w"}
poplar=["r1","l1","r1","r2","r2","l2","l2","r3","l3","r3","r4","l4","r4","top"]
def crown(a,v):
    slug=a["id"]
    for s in v["body"]:
        name=s["id"]
        target=None
        if slug=="apple-tree" and name.startswith("crown-"):target="fol-"+apple[name[6:]]
        if slug=="english-oak" and name.startswith("crown-"):
            groups={"w-low":"w-low","wb-low":"w-low","w-up":"w-low","e-low":"e-low","ef-low":"e-low","e-up":"e-low","w-mid":"w-mid","wf-mid":"w-mid","e-mid":"e-mid","eb-mid":"e-mid","leader-w":"leader-w","leader-e":"leader-e","f-mid":"core","b-mid":"core","top":"top","core":"core","inner":"core"}
            target="fol-"+next(g for k,g in groups.items() if name[6:].startswith(k))
        if slug=="lombardy-poplar" and name.startswith("crown-swell-"):
            target="fol-"+poplar[int(name.split("-")[-1])-1]
        if slug=="london-plane-pollarded" and name.startswith("crown-"):
            target="fol-"+plane[name[6:]]
        if slug=="japanese-maple" and name.startswith(("t1-","t2-","t3-","top-","core-")):
            if "-x" in name:
                target="fol-"+name[:2]+("e" if name[-1] in "14" else "w")
            else:
                target=next(g["bone"] for scale in ("200","100","50") for g in a["volumes"][scale]["body"] if g["id"]==name)
        if target:rebind(frames[slug],s,target)
    note={"apple-tree":("Fruit-tree crown clusters follow their own fruiting scaffolds during sway.","Die Kronenpartien folgen beim Schwingen ihren fruchttragenden Gerüstästen."),"english-oak":("The lower boughs sway separately from the leader and its upper crown.","Die unteren Starkäste schwingen unabhängig vom Leittrieb und seiner oberen Krone."),"japanese-maple":("The three leaf tiers follow the eastern and western fans of each stem.","Die drei Blattetagen folgen den östlichen und westlichen Zweigfächern ihrer Stämme."),"lombardy-poplar":("The narrow crown follows eight ascending branch fans and the top shoot.","Die schmale Krone folgt acht aufsteigenden Astfächern und dem Gipfeltrieb."),"london-plane-pollarded":("The low leaf groups and upright regrowth follow the corresponding pollard heads.","Die niedrigen Blattgruppen und aufrechten Jungtriebe folgen den zugehörigen Kopfästen.")}[slug]
    if v["decisions"][-1]["en"]!=note[0]:v["decisions"].append({"en":note[0],"de":note[1]})
def other(a,v):
    slug=a["id"];src=a["volumes"]["200"]
    if slug=="hornbeam-hedge":
        v["body"]=[s for s in v["body"] if s["id"]!="hedge" and not s["id"].startswith("block-")]
        v["thermal"]=[s for s in v["thermal"] if s.get("of")!="hedge" and not s.get("of","").startswith("block-")]
        for s in src["body"]:
            if not s["id"].startswith("block-"):continue
            v["body"].append(h["shortened"](s,[0,2,4,5]))
            v["thermal"].append(next(t for t in src["thermal"] if t.get("of")==s["id"]))
        note=("Six overlapping clipped crowns follow their individual stools.","Sechs überlappende geschnittene Kronen folgen ihren einzelnen Stöcken.")
    elif slug=="date-palm":
        if "root-collar" in {s["id"] for s in v["body"]}:return
        v["body"].insert(0,h["shortened"](next(s for s in src["body"] if s["id"]=="root-collar"),[0,2,3]))
        v["thermal"].insert(0,next(s for s in src["thermal"] if s.get("of")=="root-collar"))
        note=("The basal root collar grounds the segmented stem beneath the radial fronds.","Der Wurzelhals trägt den gegliederten Stamm unter dem strahlenförmigen Wedelkranz.")
    else:
        if "tress-a01-arch" in {s["id"] for s in v["body"]}:return
        for name in ["tress-a01-arch","tress-a02-arch","tress-a04-arch"]:
            item=next(s for s in src["body"] if s["id"]==name)
            n=len(item["sections"])
            v["body"].append(h["shortened"](item,sorted({0,n//2,n-1})))
            v["thermal"].append(next(t for t in src["thermal"] if t.get("of")==name))
        note=("The eastern and western canopy arches carry pendulous shoots over both sides of the fork.","Östliche und westliche Kronenbögen tragen die Hängezweige an beiden Seiten der Gabel.")
    if v["decisions"][-1]["en"]!=note[0]:v["decisions"].append({"en":note[0],"de":note[1]})
for slug in frames:h["write_scale"](next(h["catalog"].glob("*/"+slug+"/*.json")),500,crown)
for slug in ["hornbeam-hedge","date-palm","weeping-willow"]:h["write_scale"](next(h["catalog"].glob("*/"+slug+"/*.json")),500,other)

