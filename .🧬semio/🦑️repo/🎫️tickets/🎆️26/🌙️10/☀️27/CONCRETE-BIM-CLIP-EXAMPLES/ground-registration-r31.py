"""📏 Register independent primary plan controls and plot source-bound ground deltas."""
import importlib,json
from pathlib import Path
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon as Patch
from shapely.geometry import LineString

TICKET=Path(__file__).parent
case=json.loads((TICKET/"ground-registration-r31.json").read_text())
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
model=api.load_model()

def register(controls):
    pixels=np.asarray([c["pixel"] for c in controls],float)
    world=np.asarray([c["world"] for c in controls],float)
    matrix=np.linalg.lstsq(np.c_[pixels,np.ones(len(pixels))],world,rcond=None)[0]
    residual=np.linalg.norm(np.c_[pixels,np.ones(len(pixels))]@matrix-world,axis=1)
    return matrix,residual

def apply(points,matrix):
    p=np.asarray(points,float)
    return np.c_[p,np.ones(len(p))]@matrix

matrix,residual=register(case["controls"])
flc_matrix,flc_residual=register(case["flc19414"]["controls"])
trace=apply(case["trace_pixels"],matrix)
flc_trace=apply(case["flc19414"]["curve_pixels"],flc_matrix)
diagonal=apply(case["garage_diagonal_pixels"],matrix)
envelope=apply(case["east_conceptual_envelope_pixels"],matrix)
uncertainty=float(np.linalg.norm(matrix[:2],axis=1).max()*case["pixel_uncertainty"])
fig,axes=plt.subplots(1,3,figsize=(16,8),layout="constrained")
ax=axes[0]
ax.imshow(plt.imread(TICKET/"🗑️generated"/case["source_image"]))
ax.plot(*np.asarray(case["trace_pixels"]).T,color="#b72c65",linewidth=1.5)
for control,error in zip(case["controls"],residual):
    x,y=control["pixel"]
    ax.plot(x,y,"+",color="#0a6f96")
    ax.annotate(control["id"],(x,y),fontsize=6,xytext=(3,3),textcoords="offset points")
ax.set(xlim=(735,980),ylim=(365,105),title="CMN primary pixels / Primärpixel\nBlue: homologous axes; pink: registered boundary")
ax.set_aspect("equal")
ax=axes[1]
ax.imshow(plt.imread(TICKET/"🗑️generated"/case["flc19414"]["source_image"]))
ax.plot(*np.asarray(case["flc19414"]["curve_pixels"]).T,color="#d48a13",linewidth=2)
for control in case["flc19414"]["controls"]:
    ax.plot(*control["pixel"],"+",color="#0a6f96")
ax.set(xlim=(50,730),ylim=(890,190),title="FLC19414 design phase / Entwurfsphase\nIndependent grid; no forced stair equivalence")
ax.set_aspect("equal")
ax=axes[2]
for element in model["elements"]:
    if element["storey"]!="ground":
        continue
    if element["class"]=="IfcColumn":
        geometry=api.column_geometry(model,element)
        for part in geometry["parts"]:
            if part["z0"]<=1.<part["z1"]:
                for region in api.part_regions(part,1.):
                    ax.add_patch(Patch(region,facecolor="#353535",edgecolor="none"))
        x,y=element["geometry"]["axis"]
        ax.annotate(element["tag"],(x,y),fontsize=5,xytext=(2,2),textcoords="offset points")
    elif element["class"]=="IfcWall":
        path=element["geometry"].get("path")
        if path and all(isinstance(p,list) for p in path):
            ax.plot(*np.asarray(path).T,color="#969696",linewidth=.8)
for part in api.filling_geometry("CW-1",model)["parts"]:
    if part["role"]=="glass":
        points=np.asarray(part["outline"]+[part["outline"][0]])
        ax.plot(*points.T,color="#11799b",linewidth=.8)
ax.plot(*trace.T,color="#b72c65",linewidth=2,label="CMN boundary registration")
for polygon in [LineString(trace).buffer(uncertainty)]:
    ax.add_patch(Patch(np.asarray(polygon.exterior.coords),facecolor="#b72c65",alpha=.12,edgecolor="none"))
ax.plot(*flc_trace.T,color="#d48a13",linestyle="--",label="FLC19414 design comparison")
ax.plot(*diagonal.T,color="#b72c65",linewidth=2,linestyle=":",label="CMN garage diagonal")
ax.plot(*envelope.T,color="#817787",linestyle="--",label="East conceptual trace; ownership unresolved")
ax.plot([],[],color="#11799b",label="Current native CW glass footprint")
ax.set(xlim=(-.7,19.7),ylim=(-.7,22.2),xlabel="X / m",ylabel="Y / m",title="Registered source vs current authored ground\nRegistrierte Quelle und heutiges Modell")
ax.set_aspect("equal");ax.grid(alpha=.15);ax.legend(fontsize=6,loc="upper left")
fig.suptitle("Primary-ground registration before pane authorship / Primärregistrierung vor Scheibenmodellierung",fontsize=13)
fig.savefig(TICKET/"🗑️generated"/"ground-registration-r31.png",dpi=150)
plt.close(fig)
result={"matrix":matrix.tolist(),"control_residuals_m":residual.tolist(),"maximum_control_residual_m":float(residual.max()),"trace_world":trace.tolist(),"garage_diagonal_world":diagonal.tolist(),"east_envelope_world":envelope.tolist(),"flc_design_trace_world":flc_trace.tolist(),"pixel_uncertainty_m":uncertainty,"model_elements":len(model["elements"]),"status":"inferred registration; no production edit"}
(TICKET/"🗑️generated"/"ground-registration-r31-result.json").write_text(json.dumps(result,indent=2))
print(json.dumps(result,indent=2))
                                                                                                                                                        