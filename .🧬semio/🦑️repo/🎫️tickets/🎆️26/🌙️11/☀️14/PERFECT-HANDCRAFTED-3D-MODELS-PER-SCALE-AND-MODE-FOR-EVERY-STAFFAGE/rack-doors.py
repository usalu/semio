"""🗄️ The folded, punched doors, ventilated enclosure and four wheel contacts of the 42U recording rack."""
from copy import deepcopy
from pathlib import Path
import importlib.util
import json
import math
import os
from runpy import run_path

T=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location("rack_kit",T/"r3-server-rack-kit.py")
K=importlib.util.module_from_spec(spec);spec.loader.exec_module(K)
CAT=next((T.parents[6]/"🎓️teaching").glob("*architecture/*staffage/*catalog/*/server-rack/*.json"))
SOURCE=T/"volumes/appliances/server-rack.json"
G=.0015
H=(.10,1.967)
AXES={"door-hinge":(-.2735,-.4935),"rear-door-a":(-.2735,.4935),"rear-door-b":(.2735,.4935)}
PUNCH={"50":(8,30,4),"100":(8,24,4),"200":(3,10,2),"500":(2,4,1)}
def words(en,de):return {"en":en,"de":de}

def rear_local(solid,bone):
    """🧭 Transform the world-authored rear fitting to the upright hinge on its folded return."""
    x,z=AXES[bone];solid["bone"]=bone
    trans=solid.setdefault("transform",{});old=trans.get("translation",[0,0,0])
    trans["translation"]=[round(a+b,7) for a,b in zip(old,[-.10,x,-z])]
    return solid

def panels(d,bone,span,front=False):
    """🕳️ Preserve gross clear area, exterior dimensions and the unpunched latch stile at every resolution."""
    x0,x1=span;y0,y1=H;z=-.50 if front else .50
    nx,ny,nrear=PUNCH[d];nx=nx if front else nrear
    inset0,inset1=(.018,.044) if front else ((.011,.016) if bone.endswith("a") else (.016,.011))
    fx0,fx1=x0+inset0,x1-inset1;fy0,fy1=y0+.022,y1-.022
    pitch=( (fx1-fx0)/nx,(fy1-fy0)/ny )
    factor=math.sqrt(.70*(x1-x0)*(y1-y0)/((fx1-fx0)*(fy1-fy0)))
    holes=[]
    for j in range(ny):
        for i in range(nx):
            cx,cy=fx0+(i+.5)*pitch[0],fy0+(j+.5)*pitch[1];a,b=pitch[0]*factor/2,pitch[1]*factor/2
            holes.append(K.rect(cx-a,cx+a,cy-b,cy+b))
    base="door" if front else "frame";name="door-field" if front else bone
    plate=K.deep(name,base,"mesh-steel",K.rect(x0+G,x1-G,y0+G,y1-G),(z,z+G) if front else (z-G,z),holes=holes)
    folded=K.deep("door-return" if front else bone+"-return",base,"mesh-steel",K.rect(x0,x1,y0,y1),
                  (z,z+.025) if front else (z-.025,z),holes=[K.rect(x0+G,x1-G,y0+G,y1-G)],tone=.25)
    return [plate,folded] if front else [rear_local(s,bone) for s in (plate,folded)]

def cabinet(d,vol):
    """🧱 The coarse open carcass carries a punched roof, real side sheets and four wheels."""
    body=vol["body"];lookup={s["id"]:s for s in body}
    for s in body:
        if s["id"]=="roof" and s["kind"]=="extrude":s["from"]=round(s["to"]-.0012,5)
    if d in ("200","500"):
        body[:]=[s for s in body if s["id"] not in ("cabinet","roof","roof-skirt","side-a","side-b","casters") and not (d=="500" and s["id"] in ("base-frame","caster-f","caster-r"))]
        count=12 if d=="200" else 8
        holes=[K.circle(0,z,.08,count) for z in (.10,.32)]
        if d=="200":holes.append(K.rect(-.15,.15,.41,.43))
        body.append(K.up("roof","frame","rack-steel",K.rect(-.3,.3,-.475,.475),(1.9988,2),holes=holes))
        body.append(K.up("roof-skirt","frame","rack-steel",K.rect(-.3,.3,-.475,.475),(1.967,1.9988),holes=[K.rect(-.2988,.2988,-.4738,.4738)]))
        for side,(a,b) in (("a",(-.3,-.2988)),("b",(.2988,.3))):
            body.append(K.across("side-"+side,"frame","rack-steel",K.rect(.07,1.967,-.475,.475),(a,b)))
        if d=="500":
            body.append(K.up("base-frame","frame","rack-steel",K.rect(-.28,.28,-.475,.475),(.07,.10),holes=[K.rect(-.25,.25,-.445,.445)]))
            for name in ("server-block","top-group"):
                s=lookup[name];s["size"][1]=.4826;s["size"][2]=.72 if name=="server-block" else .43
                s["transform"]["translation"][2]=round(-.408+s["size"][2]/2,5)
            for end,z in (("f",-.45),("r",.45)):
                outline=[(.004,z-.014),(0,z-.006),(0,z+.006),(.004,z+.014),(.016,z+.025),(.048,z+.023),(.07,z+.018),(.07,z-.018),(.048,z-.023),(.016,z-.025)]
                body.append(K.repeat(K.across("caster-"+end,"feet","caster-rubber",outline,(-.262,-.238),role="ground"),2,dx=.50))
    skirt=next((s for s in body if s["id"]=="roof-skirt"),None)
    if skirt:
        skirt.update(K.up("roof-skirt","frame","rack-steel",K.rect(-.3,.3,-.475,.475),(1.967,1.9988),holes=[K.rect(-.2988,.2988,-.4738,.4738)]))
    return body

def covers(d,vol):
    """🌡️ Bind heat geometry to the actual ventilated roof, three pierced doors and thin skin of the enclosure."""
    th=vol["thermal"];body={s["id"]:s for s in vol["body"]}
    th[:]=[s for s in th if not ("door" in s["id"] or s["id"] in ("th-roof","th-casters") or d=="500" and s["id"] in ("th-caster-f","th-caster-r"))]
    for s in th:
        if s.get("layer")=="cabinet-sheet" and s.get("kind")=="box":
            axis=1 if s["id"].startswith("th-side") else 0
            coord=s["transform"]["translation"];face= .30 if s["id"].startswith("th-side-a") else -.30 if s["id"].startswith("th-side-b") else .07
            coord[axis]=face + (.0006 if face in (-.30,.07) else -.0006);s["size"][axis]=.0012
        if s["id"]=="th-exhaust-air":
            s.clear();s.update(K.repeat(K.turned("th-exhaust-air","frame","rack-steel",[(0,.077),(.025,.077)],"y",(0,1.975,.10),role="detail"),2,dz=.22))
            s.update(region="cabinet",tags=["emitter","exhaust-air"])
        if s["id"] in ("th-servers","th-blanks","th-switch") and d in ("200","500"):
            s["size"][1]=.4826;s["transform"]["translation"][2]=-.408-s["size"][2]/2
    th.append({"id":"th-roof","of":"roof","region":"cabinet","layer":"cabinet-sheet","sectionFaces":["-u"]})
    for id_ in ("door-field","door-return","rear-door-a","rear-door-a-return","rear-door-b","rear-door-b-return"):
        front=id_.startswith("door-")
        th.append({"id":"th-"+id_,"of":id_,"region":"door" if front else "rear-doors","layer":"door-sheet" if front else "rear-sheet"})
    if d=="500":
        th.extend({"id":"th-"+id_,"of":id_,"region":"feet"} for id_ in ("caster-f","caster-r"))
    for s in body.values():
        if s['id']=='gland-plate' and s.get('plane')=='yz':s['to']=round(s['from']+.0012,5)

def structure(d,vol):
    st=vol["structure"]
    st[:]=[s for s in st if not s.get("anchor","").startswith(("base-contact","wheel-")) and s["id"] not in ("member-door","member-casters","member-rear-door-a","member-rear-door-b","joint-rear-door-a","joint-rear-door-b","member-hinge-pin") and not (d=="500" and s["id"] in ("member-caster-f","member-caster-r"))]
    st.append({"id":"member-door","of":"door-return","member":"door","part":"member"})
    for id_ in ("rear-door-a","rear-door-b"):
        st.extend([{"id":"member-"+id_,"of":id_+"-return","member":id_,"part":"member"},
                   {"id":"joint-"+id_,"of":id_+"-hinge","member":id_,"part":"joint"}])
    if d=="500":
        st.extend({"id":"member-caster-"+end,"of":"caster-"+end,"member":"feet","part":"member"} for end in ("f","r"))
    st.append({"id":"member-hinge-pin","of":"door-hinge-pin","member":"door-hinge","part":"member"})
    for end,z in (("f",-.45),("r",.45)):
        for side,x in (("left",-.25),("right",.25)):
            name="wheel-"+end+"-"+side
            pad=K.box("support-"+name,"feet","caster-rubber",(x-.010,x+.010),(-.001,0),(z-.004,z+.004),role="ground")
            st.append(dict(pad,member="feet",part="support",anchor=name))


def fit(d,vol):
    body=cabinet(d,vol)
    names={"door-return","door-field","door-face","rear-door","rear-door-a","rear-door-b","rear-door-a-return","rear-door-b-return"}
    body[:]=[s for s in body if s["id"] not in names and not s["id"].startswith("rear-door-") and s["id"]!="door-hinge-pin"]
    body.extend(panels(d,"door",(-.27,.27),True))
    for suffix,span in (("a",(-.27,-.001)),("b",(.001,.27))):
        name="rear-door-"+suffix;x,z=AXES[name];body.extend(panels(d,name,span))
        pin=K.repeat(K.turned(name+"-hinge","frame","mesh-steel",[(0,.006),(.06,.006)],"y",(x,.27,z),role="detail"),3,dy=.72)
        pin["slender"]=False;body.append(rear_local(pin,name))
        handle=next((s for s in body if s["id"]=="rear-handle-"+suffix),None)
        if handle and handle["bone"]=="frame":rear_local(handle,name)
    for name,(hx,hz) in AXES.items():
        if d not in ("50","100"):continue
        side=1 if hz<0 else -1
        root="front" if name=="door-hinge" else name[-1]+"-rear"
        for label,y,h in (("lower",.240,.029),("upper",.331,.029)):
            prefix="socket-"+root+"-"+label
            body[:]=[s for s in body if s["id"] not in (prefix,prefix+"-strap")]
            ring=[(0,.0068),(h,.0068),(h,.0032),(0,.0032),(0,.0068)]
            body.append(K.repeat(K.turned(prefix,"frame","rack-steel",ring,"y",(hx,y,hz),role="detail"),3,dy=.72))
            x0,x1=sorted((hx-.004 if hx<0 else hx+.004,-.2535 if hx<0 else .2535))
            z0,z1=sorted((hz+side*.005,hz+side*.036))
            body.append(K.repeat(K.box(prefix+"-strap","frame","rack-steel",(x0,x1),(y,y+h),(z0,z1),role="detail"),3,dy=.72))
        hinge=next((s for s in body if s["id"]==("door-hinge" if name=="door-hinge" else name+"-hinge")),None)
        if hinge:
            hinge["profile"]=[[0,.006],[.06,.006],[.06,.0032],[0,.0032],[0,.006]]
    pin=K.repeat(K.turned("door-hinge-pin","frame","rack-steel",[(0,.003),(.124,.003)],"y",(-.2735,.238,-.4935),role="detail",slender=True),3,dy=.72)
    body.append(rear_local(pin,"door-hinge"));covers(d,vol);structure(d,vol)
    vol["decisions"]=[words("The 42U enclosure keeps its 1.2 mm cover gauge. Its three 1.5 mm folded door sheets have 70% free gross face area. The front opens on its left pin; the two rear wings turn outward on opposite pins. Punched aperture count falls with drawing scale. The structural contact is four wheel prints.","Das 42-HE-Gehäuse behält seine Abdeckbleche von 1,2 mm. Die drei gekanteten Türbleche sind 1,5 mm stark und haben 70 % freien Gesamtquerschnitt. Die Fronttür öffnet am linken Bolzen, die beiden hinteren Flügel an gegenüberliegenden Bolzen nach außen. Die Zahl der dargestellten Stanzöffnungen sinkt mit dem Maßstab. Vier Radaufstandsflächen tragen den Schrank.")]

def rig(a):
    bones=a["rig"]["bones"];front=next(b for b in bones if b["id"]=="door")
    front.update(parent="door-hinge",x=0,y=-.2735,z=.4935)
    next(b for b in bones if b["id"]=="frame")["mass"]["kg"]=85
    bones[:]=[b for b in bones if b["id"] not in AXES]
    for name,(x,z) in AXES.items():
        rear=name.startswith("rear");angle=95 if name.endswith("a") else -95 if rear else -100
        b=dict(id=name,parent="frame",title=words({"door-hinge":"Front hinge axis","rear-door-a":"Left rear leaf","rear-door-b":"Right rear leaf"}[name],{"door-hinge":"Frontscharnierachse","rear-door-a":"Linker hinterer Türflügel","rear-door-b":"Rechter hinterer Türflügel"}[name]),x=.1,y=-x,z=z,angleDeg=0,lengthM=1.867,joint={"type":"revolute","minDeg":0,"maxDeg":0,"twistMinDeg":min(angle,0),"twistMaxDeg":max(angle,0)})
        if rear:b["mass"]={"kg":5,"comX":.9335,"comY":-.138 if name.endswith("a") else .138,"source":"authored-estimate","distribution":"point"}
        bones.insert(bones.index(front),b)
    a["poses"][:]=[p for p in a["poses"] if p["id"]!="service-open"]
    p=deepcopy(next(p for p in a["poses"] if p["id"]=="off"));p.update(id="service-open",title=words("Service access","Wartungszugang"),description=words("Power isolated. The front door opens 100° and the divided rear doors open 95° to expose the rails and cable side.","Spannung abgeschaltet. Die Fronttür öffnet 100°, die hinteren Flügel öffnen 95° und geben Schienen und Verkabelung frei."),bones={n:{"twistDeg":v} for n,v in zip(AXES,(-100,95,-95))});a["poses"].append(p)
    a["clips"][:]=[c for c in a["clips"] if c["id"]!="service-access"]
    c=dict(id="service-access",title=words("Open for maintenance","Zur Wartung öffnen"),description=words("Open the three doors, inspect the de-energised rack, then close it.","Die drei Türen öffnen, den spannungsfreien Schrank prüfen und wieder schließen."),durationS=8,fps=12,loop=True,rootMotion="none",keys=[{"t":s,"pose":p,"ease":"smooth"} for s,p in ((0,"off"),(3,"service-open"),(5,"service-open"),(8,"off"))],contacts=[{"anchor":"base-contact","fromS":0,"toS":8}],events=[],phases=[{"id":"open","fromS":0,"toS":3,"title":words("Open doors","Türen öffnen")},{"id":"inspect","fromS":3,"toS":5,"title":words("Inspect","Prüfen")},{"id":"close","fromS":5,"toS":8,"title":words("Close doors","Türen schließen")}],tracks=[])
    a["clips"].append(c)

def contacts(a):
    fresh=[];ids=[]
    for end,z in (("f",-.45),("r",.45)):
        for side,x in (("left",-.25),("right",.25)):
            id_="wheel-"+end+"-"+side;ids.append(id_)
            fresh.append(dict(id=id_,bone="feet",x=0,y=-x,z=z,kind="contact",support="contact",footprintLengthM=.020,footprintWidthM=.008,frictionCoefficient=.6,title=words(("Front " if end=="f" else "Rear ")+side+" tire contact",("Vordere " if end=="f" else "Hintere ")+("linke" if side=="left" else "rechte")+" Radaufstandsfläche")))
    a["anchors"]=[s for s in a["anchors"] if s["kind"]!="contact"]+fresh
    a["structure"]["supports"]=ids
    for p in a["poses"]:p["contacts"]=ids[:]
    for c in a["clips"]:
        c["contacts"]=[{"anchor":i,"fromS":0,"toS":c["durationS"]} for i in ids]

def leaf_properties(width,rear=False):
    height=H[1]-H[0];outer=width*height;inner=(width-2*G)*(height-2*G)
    sheet=(inner-.70*outer)*G+(outer-inner)*.025
    nx,ny=(4,30) if rear else (8,30)
    width_free=width-(.027 if rear else .062);height_free=height-.044
    fraction=math.sqrt(.70*outer/(width_free*height_free))
    hole_edge=2*nx*ny*fraction*(width_free/nx+height_free/ny)
    surface=2*(inner-.70*outer)+2*(outer-inner)+2*(width+height)*.025+2*(width+height-4*G)*(.025-G)+hole_edge*G
    return round(sheet*7850,6),round(surface,6)

def thermal(a):
    next(s for s in a["thermal"]["emitters"] if s["id"]=="exhaust-air")["diameterM"]=round(math.sqrt(2)*.16,6)
    regions=a["thermal"]["regions"];front=next(s for s in regions if s["id"]=="door")
    steel=next(m for m in a["materials"] if m["id"]=="rack-steel")
    mesh=next(m for m in a["materials"] if m["id"]=="mesh-steel")
    for key in ("densityKgM3","conductivityWmK","specificHeatJKgK"):mesh[key]=steel[key]
    mf,af=leaf_properties(.54);mr,ar=leaf_properties(.269,True)
    front["areaM2"]=af;front["heatCapacityJM2K"]=round(mf*490/af,3)
    front["layers"][0]["heatCapacityJM2K"]=round(7850*490*G,3)
    bones={b["id"]:b for b in a["rig"]["bones"]}
    bones["door"]["mass"]["kg"]=round(mf+.35,4)
    for name in ("rear-door-a","rear-door-b"):bones[name]["mass"]["kg"]=round(mr+.16,4)
    bones["frame"]["mass"]["kg"]=round(234-sum(b["mass"]["kg"] for n,b in bones.items() if n!="frame" and "mass" in b),4)
    front["layers"][0]["title"]=words("1.5 mm perforated steel, 70% gross free area","Lochblech 1,5 mm, 70 % freier Gesamtquerschnitt")
    rear=deepcopy(front);rear.update(id="rear-doors",title=words("Split ventilated rear doors","Geteilte gelochte Rücktüren"),areaM2=round(2*ar,6),heatCapacityJM2K=round(mr*490/ar,3))
    rear["layers"][0]["id"]="rear-sheet"
    regions[:]=[s for s in regions if s["id"]!="rear-doors"];regions.append(rear)
    next(r for r in regions if r["id"]=="cabinet")["areaM2"]=4.72
    for p in a["poses"]:p["thermal"]["regions"]["rear-doors"]={"exposedFraction":1,"sunlitFraction":0}

def paper(a):
    for d,r in a["representations"].items():
        if "geometry" not in r:continue
        g=r["geometry"]
        for v,shapes in g.items():
            shapes[:]=[s for s in shapes if not s["id"].startswith("rear-service-")]
            if v in ("plan","front"):shapes[:]=[s for s in shapes if s["id"]!="rear-door-edge"]
        for id_,(x,z) in AXES.items():
            if id_=="door-hinge":continue
            a0,a1=(-.27,-.001) if id_.endswith("a") else (.001,.27)
            for view,coords in (("elevation",(0,-a1+x,1.867,a1-a0)),("plan",(a0,.475,a1-a0,.025)),("front",(.475,.10,.025,1.867))):
                g[view].insert(0,dict(id="rear-service-"+id_[-1],bone=id_,role="body",z=.75,filled=True,tone=.32,region="rear-doors",material="mesh-steel",penMm=.20,kind="rect",**dict(zip(("x","y","w","h"),coords))))
        inks={"50":{"door-hinge-1","door-hinge-2","door-hinge-3","door-lock"},"100":{"door-handle"},"200":{"post-right","post-left","base-rail","door-edge"},"500":{"top-group"}}
        for shapes in g.values():
            for shape in shapes:
                if shape["id"] in inks.get(d,set()):shape["penMm"]=.20
        if d=="500":
            for v in g.values():
                for s in v:
                    if s.get("regions")==["cabinet","door"]:s.pop("regions");s["region"]="cabinet"
            if not any(s["bone"]=="door" for s in g["elevation"]):
                for view,coords in (("elevation",(0,-.27,1.867,.54)),("plan",(-.27,-.50,.54,.025)),("front",(-.50,.10,.025,1.867))):
                    g[view].append(dict(id="door-field",bone="door",role="body",z=3.1,filled=False,tone=.18,region="door",material="mesh-steel",penMm=.2,kind="rect",**dict(zip(("x","y","w","h"),coords))))

def projected_paper(a):
    for den,rep in a["representations"].items():
        g=rep.get("geometry")
        if not g:continue
        front=[s for s in g["elevation"] if s["bone"]=="door" and s["id"] not in ("door-perforation","door-field","door-frame")]
        for s in front:
            span=(-.515,-.5075) if s["id"]=="door-handle-lever" else (-.5075,-.500) if "handle" in s["id"] or "lock" in s["id"] else (-.50,-.487)
            s["solid"]={"kind":"extruded","sweep":"w","z0":span[0],"z1":span[1]}
        leaves=[]
        for bone,x,span,z,depth in (("door",0,(-.27,.27),-.5,-.475),("rear-door-a",-.2735,(-.27,-.001),.5,.475),("rear-door-b",.2735,(.001,.27),.5,.475)):
            lo,hi=span;u0,u1=0,1.867;v0,v1=x-hi,x-lo
            def pathline(ring):
                return "M "+" L ".join(f"{p:.6f} {q:.6f}" for p,q in ring)+" Z"
            leaf,brace=panels(den,bone,span,bone=="door")
            offset=leaf.get("transform",{}).get("translation",[0,0,0])
            def face(ring):return [[r[0]+offset[0],r[1]+offset[1]] for r in ring]
            outer=face(leaf["profile"]);holes=[face(r) for r in leaf["holes"]]
            common=dict(bone=bone,role="body",z=3.9 if bone=="door" else .75,filled=True,tone=.22,region="door" if bone=="door" else "rear-doors",material="mesh-steel",penMm=.20,kind="path")
            leaves.append(dict(common,id=bone+"-field" if bone!="door" else "door-field",d=" ".join([pathline(outer)]+[pathline(list(reversed(h))) for h in holes]),solid={"kind":"extruded","sweep":"w","z0":min(z,z+(.0015 if z<0 else -.0015)),"z1":max(z,z+(.0015 if z<0 else -.0015))}))
            leaves.append(dict(common,id=bone+"-frame",d=pathline(outer)+" "+pathline(list(reversed(face(brace["holes"][0])))),solid={"kind":"extruded","sweep":"w","z0":min(z,depth),"z1":max(z,depth)}))
        if den=="500" and any(s["id"]=="cabinet" for s in g["elevation"]):
            elevation=g["elevation"];elevation[:]=[s for s in elevation if s["id"] not in ("cabinet","casters")]
            for id_,bone,rect,material,region in (
                ("post-left","frame",(.07,.24,1.897,.06),"rack-steel","cabinet"),("post-right","frame",(.07,-.30,1.897,.06),"rack-steel","cabinet"),
                ("roof","frame",(1.967,-.30,.033,.60),"rack-steel","cabinet"),("base-rail","frame",(.07,-.28,.03,.56),"rack-steel","cabinet"),
                ("caster-left","feet",(0,.238,.07,.024),"caster-rubber","feet"),("caster-right","feet",(0,-.262,.07,.024),"caster-rubber","feet")):
                elevation.append(dict(id=id_,bone=bone,role="body",z=3,filled=True,tone=.4,region=region,material=material,penMm=.20,kind="rect",**dict(zip(("x","y","w","h"),rect))))
        for view,shapes in g.items():
            shapes[:]=[s for s in shapes if s["bone"] not in ("door","rear-door-a","rear-door-b")]
            shapes.extend(deepcopy(leaves+front))

def apply(a):
    rig(a);thermal(a);contacts(a);paper(a);projected_paper(a)
    for d,v in a["volumes"].items():fit(d,v)
    fine=next(s for s in a["volumes"]["50"]["body"] if s["id"]=="base-frame")
    coarse=next(s for s in a["volumes"]["500"]["body"] if s["id"]=="base-frame")
    coarse.clear();coarse.update(deepcopy(fine))
    for d,v in a["volumes"].items():
        if d in ("200","500"):
            v["body"][:]=[s for s in v["body"] if s["id"] not in ("post-a","post-b")]
            v["body"].extend(deepcopy(s) for s in a["volumes"]["50"]["body"] if s["id"] in ("post-a","post-b"))
        v["structure"][:]=[s for s in v["structure"] if s["id"] not in ("mass-frame","mass-door","mass-front-leaf","mass-rear-a","mass-rear-b","mass-post-a","mass-post-b") and not (d=="500" and s["id"] in ("joint-rear-door-a","joint-rear-door-b"))]
        for name in ("a","b"):
            v["structure"].append(dict(id="mass-post-"+name,of="post-"+name,member="frame",part="mass"))
        for name,bone,of in (("mass-front-leaf","door","door-field"),("mass-rear-a","rear-door-a","rear-door-a"),("mass-rear-b","rear-door-b","rear-door-b")):
            v["structure"].append(dict(id=name,of=of,member=bone,part="mass"))
    run_path(str(Path(__file__).with_name('rack-stocks.py')))['author'](a)

def write(path,doc,original):
    indent=next(i for i in (1,2,4) if original.startswith(b'{\n'+b' '*i+b'"'))
    tmp=path.with_suffix(".partial");tmp.write_text(json.dumps(doc,ensure_ascii=False,indent=indent)+("\n" if original.endswith(b"\n") else ""))
    assert path.read_bytes()==original;os.replace(tmp,path)

if __name__=="__main__":
    raw=CAT.read_bytes();sr=SOURCE.read_bytes();d=json.loads(raw);source=json.loads(sr)
    apply(d["asset"]);source["volumes"]=d["asset"]["volumes"]
    write(CAT,d,raw);write(SOURCE,source,sr)
