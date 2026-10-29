"""🏗️ Plot actual cellar projections and record their independent clearance diagnostics."""
import importlib
import json
import math
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon as Patch
from shapely.geometry import Polygon,LineString
from shapely.ops import unary_union

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model=api.load_model()
root=Path(__file__).parent
output=root/"🗑️generated"
case=json.loads((Path(api.__file__).parent/"🧪️tests/🏗️services.json").read_text(encoding="utf-8"))
geometry=api.building_geometry(model)
fig,axes=plt.subplots(2,2,figsize=(15,13),layout="constrained")

def patch(ax,ring,color,alpha=1.,edge=None):
    if len(ring)>=3:
        ax.add_patch(Patch(ring,facecolor=color,edgecolor=edge or color,alpha=alpha,linewidth=.8))

for ax,z in zip(axes[0],(-1.2,.05)):
    for wall in geometry["walls"]:
        for part in wall["parts"]:
            if part["z0"]<=z<part["z1"]:
                patch(ax,api.part_outline_at(part,z),"#40474d")
    for tag in ("CELLAR-HOIST-PIT","CELLAR-COAL-CHUTE","HOIST-JIB"):
        for part in api.solid_geometry(tag,model)["parts"]:
            if part["z0"]<=z<part["z1"]:
                patch(ax,api.part_outline_at(part,z),"#b48247")
    for column in geometry["pilotis"]:
        active=[p for p in column["parts"] if p["z0"]<=z<p["z1"]]
        for p in active:
            patch(ax,p["outline"],"#9c3443")
        if active and 2<column["x"]<15.5 and 5<column["y"]<18:
            ax.text(column["x"],column["y"],column["tag"],fontsize=7,ha="center")
    for key in ("hoist","coal"):
        ring=api.element(case[key]["opening"],model)["geometry"]["outline"]
        patch(ax,ring,"#5db8dd",.28,"#087da5")
        ax.text(sum(p[0] for p in ring)/4,sum(p[1] for p in ring)/4,key.upper(),fontsize=8,ha="center")
    ax.set(xlim=(1.8,15.8),ylim=(5,18),xlabel="X / m",ylabel="Y / m")
    ax.set_aspect("equal");ax.grid(alpha=.15)
axes[0,0].set_title("Cellar cut −1.20 m / Kellerschnitt")
axes[0,1].set_title("Ground cut +0.05 m, covers hidden / Erdgeschoss, Deckel ausgeblendet")
parking=json.loads((Path(api.__file__).parent/"🧪️tests/🚗️parking.json").read_text(encoding="utf-8"))
car_length=model["car"]["length"]["value"]
car_width=model["car"]["width"]["value"]
control=[]
for pose in parking["verified_control"]["poses"]:
    angle=math.radians(pose["angle_degrees"])
    c,s=math.cos(angle),math.sin(angle)
    cx,cy=pose["center"]
    ring=[[cx+c*x-s*y,cy+s*x+c*y] for x,y in ((-car_length/2,-car_width/2),(car_length/2,-car_width/2),(car_length/2,car_width/2),(-car_length/2,car_width/2))]
    patch(axes[0,1],ring,"#cc7777",.16,"#aa4444")
    hatch=Polygon(api.element("OP-HOIST-HATCH",model)["geometry"]["outline"])
    jib=unary_union([Polygon(api.part_footprint(p)) for p in api.solid_geometry("HOIST-JIB",model)["parts"] if p["z0"]<model["car"]["height"]["value"] and p["z1"]>0])
    control.append({"pose":pose,"hatch_overlap_m2":Polygon(ring).intersection(hatch).area,"jib_overlap_m2":Polygon(ring).intersection(jib).area})

for ax,key,y in ((axes[1,0],"coal",15.95),(axes[1,1],"hoist",11.85)):
    for tag in (case[key]["assembly"],"HOIST-JIB"):
        for part in api.solid_geometry(tag,model)["parts"]:
            ring=api.part_section(part,y)
            patch(ax,ring,"#b48247")
    for tag in ("SL-CELLAR","SL-GROUND-CELLAR",case[key]["cover"]):
        for p in api.represent(tag,"3d",model)["primitives"]:
            line=Polygon(p["polygon"]).intersection(LineString([[-100,y],[100,y]]))
            for segment in ([line] if line.geom_type=="LineString" else getattr(line,"geoms",())):
                if segment.geom_type=="LineString" and segment.length>1e-10:
                    xs=[v[0] for v in segment.coords]
                    patch(ax,[[min(xs),p["z0"]],[max(xs),p["z0"]],[max(xs),p["z1"]],[min(xs),p["z1"]]],"#39778a" if tag.startswith("COVER") else "#909aa0")
    for wall in geometry["walls"]:
        if wall["storey"]!="cellar":
            continue
        for p in wall["parts"]:
            line=Polygon(p["outline"]).intersection(LineString([[-100,y],[100,y]]))
            for segment in ([line] if line.geom_type=="LineString" else getattr(line,"geoms",())):
                if segment.geom_type=="LineString" and segment.length>1e-10:
                    xs=[v[0] for v in segment.coords]
                    patch(ax,[[min(xs),p["z0"]],[max(xs),p["z0"]],[max(xs),p["z1"]],[min(xs),p["z1"]]],"#40474d")
    ax.axhline(-2.42,color="#777777",linestyle=":",linewidth=.6)
    ax.set(xlim=(2.2,5.3) if key=="coal" else (10.4,12.7),ylim=(-2.8,.25) if key=="coal" else (-2.8,2.2),xlabel="X / m",ylabel="Z / m")
    ax.set_aspect("equal");ax.grid(alpha=.15)
    ax.set_title(("Coal chute / Kohlerutsche" if key=="coal" else "Hoist shaft and inferred jib / Heberschacht und abgeleiteter Ausleger")+f" (Y={y:.2f}m)")
fig.suptitle("Actual physical cellar services / Physische Kellerdienste — inferred construction dimensions",fontsize=14)
fig.savefig(output/"cellar-services-r21.png",dpi=140)
plt.close(fig)
print(json.dumps({"elements":len(model["elements"]),"parking_control":control,"volumes":{tag:api.quantities(tag,model)["volume"] for tag in case["new_tags"]},"native_hatch_areas":{case[k]["opening"]:api.quantities(case[k]["opening"],model)["area"] for k in ("hoist","coal")}},ensure_ascii=False))
