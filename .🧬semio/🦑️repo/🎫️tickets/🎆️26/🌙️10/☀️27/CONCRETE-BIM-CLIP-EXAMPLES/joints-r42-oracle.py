"""🏗️ Decode authored source polygons independently of the model API and sweep all beam/column joints."""
import json,math,hashlib
from pathlib import Path
import numpy as np
from shapely.geometry import Polygon
from scipy.spatial import ConvexHull
TICKET=Path(__file__).parent
ROOT=next(p for p in TICKET.parents if (p/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json").is_file())
MODEL=ROOT/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model"
raw=(MODEL/"🔣️.json").read_bytes()
source=globals().get("source_override") or json.loads(raw)
objects={o["tag"]:o for field in ("elements","types","materials","storeys") for o in source[field]}
levels={o["tag"]:o["elevation"]["value"] for o in source["storeys"]}
def elevation(value):
    return levels[value] if isinstance(value,str) else value
def ring(profile,axis):
    shape=profile["shape"]
    if shape in ("circle","ellipse"):
        a=profile["diameter"]["value"]/2 if shape=="circle" else profile["width"]["value"]/2
        b=a if shape=="circle" else profile["depth"]["value"]/2
        angle=np.arange(96)*2*np.pi/96
        points=np.column_stack((a*np.cos(angle),b*np.sin(angle)))
    elif shape=="rectangle":
        a,b=profile["width"]["value"]/2,profile["depth"]["value"]/2
        points=np.array([[-a,-b],[a,-b],[a,b],[-a,b]])
    else:
        assert shape=="polygon"
        points=np.asarray(profile["outline"])
    r=math.radians(profile.get("rotation",0))
    rotation=np.array([[math.cos(r),-math.sin(r)],[math.sin(r),math.cos(r)]])
    outline=points@rotation.T+np.asarray(axis)
    holes=[]
    for h in profile.get("voids",[]):
        angle=np.arange(96)*2*np.pi/96
        local=np.column_stack((np.cos(angle),np.sin(angle)))*h["diameter"]["value"]/2+np.asarray(h["offset"])
        holes.append(local@rotation.T+np.asarray(axis))
    area=lambda p:abs(np.dot(p[:,0],np.roll(p[:,1],-1))-np.dot(p[:,1],np.roll(p[:,0],-1)))/2
    independent_area=area(outline)-sum(area(h) for h in holes)
    polygon=Polygon(outline,holes)
    assert abs(polygon.area-independent_area)<1e-10
    return polygon
columns=[]
for obj in source["elements"]:
    if obj["class"]!="IfcColumn":
        continue
    g=obj["geometry"]
    sections=g.get("sections",[{"profile":objects[obj["type"]]["profile"],"base_level":g["base_level"],"top_level":g["top_level"]}])
    for index,s in enumerate(sections):
        columns.append(dict(column=obj["tag"],section=index,polygon=ring(s["profile"],s.get("axis",g["axis"])),z0=elevation(s["base_level"]),z1=elevation(s["top_level"])))
pairs=[]
longitudinal=[]
beams=[e for e in source["elements"] if e["class"]=="IfcBeam" and "section_sweep" not in e["geometry"]]
for obj in beams:
    g=obj["geometry"]
    base=elevation(g["base_level"])
    for index,p in enumerate(g["parts"]):
        assert "mesh" not in p and "section" not in p
        polygon=Polygon(p["outline"],p.get("holes",[]))
        for column in columns:
            lo,hi=max(base+p["z0"],column["z0"]),min(base+p["z1"],column["z1"])
            if hi-lo<=1e-10:
                continue
            common=polygon.intersection(column["polygon"])
            volume=common.area*(hi-lo)
            if volume<=1e-10:
                continue
            if common.geom_type=="Polygon" and not common.interiors:
                vertices=np.array([[x,y,z] for x,y in list(common.exterior.coords)[:-1] for z in (lo,hi)])
                assert abs(ConvexHull(vertices).volume-common.convex_hull.area*(hi-lo))<1e-9
            pairs.append(dict(column=column["column"],beam=obj["tag"],part=index,section=column["section"],z0=lo,z1=hi,area=common.area,volume=volume))
fixture=json.loads((MODEL/"🧪️tests/🏗️joints.json").read_text())
key=lambda p:(p["column"],p["beam"],p["part"])
actual={}
for p in pairs:
    actual[key(p)]=actual.get(key(p),0)+p["volume"]
expected={key(p):p["volume"] for p in fixture["pairs"]}
new=[{**dict(zip(("column","beam","part"),k)),"volume":v} for k,v in actual.items() if k not in expected]
removed=[dict(zip(("column","beam","part"),k)) for k in expected if k not in actual]
changed=[{**dict(zip(("column","beam","part"),k)),"expected":expected[k],"actual":v} for k,v in actual.items() if k in expected and abs(v-expected[k])>1e-9]
owner_differences=[]
for obj in beams:
    owners={p["column"] for p in pairs if p["beam"]==obj["tag"]}
    if owners!=set(obj["geometry"].get("joint_owners",[])):
        owner_differences.append(dict(beam=obj["tag"],actual=sorted(owners),registered=obj["geometry"].get("joint_owners",[])))
result=dict(candidate=bool(globals().get("source_override")),sha256=hashlib.sha256(raw).hexdigest(),elements=len(source["elements"]),beam_products=len(beams),column_products=len({c["column"] for c in columns}),column_sections=len(columns),section_pairs=len(pairs),registered_pairs=len(actual),total=sum(actual.values()),fixture_total=fixture["pairwise_volume"],new=new,removed=removed,changed=changed,owner_differences=owner_differences,pairs=pairs,continued_columns=[{k:c[k] for k in ("column","section","z0","z1")} for c in columns if c["column"] in ("10-FIRST","15-FIRST")])
(TICKET/"🗑️generated/joints-r42-oracle.json").write_text(json.dumps(result,indent=2))
print({k:v for k,v in result.items() if k!="pairs"})
