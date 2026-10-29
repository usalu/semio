"""🕳️ Plot actual authored column material, bore air and pipe outlet geometry."""
import importlib
import math
from pathlib import Path
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon as Patch

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
m=api.load_model()
obj=next(e for e in m["elements"] if e["tag"]=="2")
x,y=obj["geometry"]["axis"]
columns=api.column_geometry(m,obj)["parts"]
pipes=[*api.service_geometry("DP-1",m)["parts"],*api.service_geometry("DP-1-OUTLET",m)["parts"]]
fig,axes=plt.subplots(2,2,figsize=(10,10),layout="constrained")

def draw(ax,ring,color,xy=True):
    if len(ring)>=3:
        points=[[a-x,b-y] for a,b in ring] if xy else [[a-x,z] for a,z in ring]
        ax.add_patch(Patch(points,facecolor=color,edgecolor="none"))

for ax,z,label in ((axes[0,0],1.,"Finished330mm envelope\nFertige330mm Hülle"),(axes[0,1],-.4,"Buried250mm square\nVerdecktes250mm Quadrat"),(axes[1,0],-.6,"Inferred side outlet\nAbgeleiteter Seitenaustritt")):
    for p in columns:
        if p["z0"]<=z<p["z1"]:
            draw(ax,p["outline"],"#727e89")
            for hole in p["holes"]:
                draw(ax,hole,"white")
    for p in pipes:
        if p["z0"]<=z<p["z1"]:
            draw(ax,api.part_outline_at(p,z),"#b83d4c")
    ax.set(xlim=(-.19,.35 if z==-.6 else .19),ylim=(-.19,.19),xlabel="LocalX / m",ylabel="LocalY / m",title=label+f"\nZ={z:.2f}m")
    ax.set_aspect("equal");ax.grid(alpha=.15)
ax=axes[1,1]
for p in columns:
    for ring in api.part_sections(p,y):
        draw(ax,ring,"#727e89",False)
for p in pipes:
    for ring in api.part_sections(p,y):
        draw(ax,ring,"#b83d4c",False)
for level in (-.65,-.6,-.55,-.3):
    ax.axhline(level,color="#777777",linestyle=":",linewidth=.5)
ax.set(xlim=(-.19,.35),ylim=(-1.05,.15),xlabel="LocalX / m",ylabel="Z / m",title="Actual pipe and clearance\nRohr und Freiraum")
ax.set_aspect("equal");ax.grid(alpha=.15)
fig.suptitle("Piloti2 (piloti4 analogous): authored finished envelope; RCcore/finish split unresolved\nStütze2 (Stütze4 analog): fertige Hülle; Aufteilung Betonkern/Putz ungeklärt",fontsize=11)
output=Path(__file__).with_name("🗑️generated")/"hollow-pilotis-r22.png"
fig.savefig(output,dpi=150)
plt.close(fig)
print("Hollow piloti diagnostic saved",output.name)
     