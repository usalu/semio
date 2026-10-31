"""🌉 Inspect native bridge geometry without copying repository source assets."""
import json
from pathlib import Path
import rhino3dm as rh

TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
ASSETS=ROOT/'🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/🌉️bridge'
result={}
for path in ASSETS.glob('*.3dm'):
    model=rh.File3dm.Read(str(path))
    objects=[]
    for i,obj in enumerate(model.Objects):
        g=obj.Geometry
        box=g.GetBoundingBox()
        row={'index':i,'kind':type(g).__name__,'layer':model.Layers[obj.Attributes.LayerIndex].Name,'name':obj.Attributes.Name,'bounds':[[box.Min.X,box.Min.Y,box.Min.Z],[box.Max.X,box.Max.Y,box.Max.Z]]}
        if isinstance(g,rh.Curve):
            n=g.ToNurbsCurve()
            row['points']=[[p.X/p.W,p.Y/p.W,p.Z/p.W] for p in n.Points]
        if isinstance(g,rh.Mesh):
            row['vertices']=[[v.X,v.Y,v.Z] for v in g.Vertices]
            row['faces']=[list(f) for f in g.Faces]
        objects.append(row)
    result[path.name]={'units':str(model.Settings.ModelUnitSystem),'objects':objects}
out=TICKET/'🗑️generated/bridge-cad.json'
out.write_text(json.dumps(result,indent=2),encoding='utf8')
print('[DEBUG] Native bridge metadata saved',flush=True)
               