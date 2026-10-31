"""🔎 Inspect reference fixture meshes using the existing NumPy and Pillow test dependencies."""
import importlib.util
from pathlib import Path
import sys
import numpy as np
from PIL import Image, ImageDraw

TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
spec=importlib.util.spec_from_file_location('capsule_detail',ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin/🐍️.py')
model=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=model
spec.loader.exec_module(model)
parts=model.capsule_assembly({'key':'B','width':2.5,'length':4.,'height':2.5},layout='Backslash').parts
image=Image.new('RGB',(1600,900),'#f1f1ee')
draw=ImageDraw.Draw(image)
sets=(('Reference wetroom: open sanitary cavities',{'bathtub','washbasin','toilet','tap','shower','drain','plumbing'},(2,-3,3)),('Reference media and telephone',{'tv','television_control','audio_console','audio_control','speaker','speaker_grille','telephone','shelf','vent'},(-3,-2,1.4)))
for index,(title,roles,eye) in enumerate(sets):
    towards=np.asarray(eye,dtype=float)
    towards/=np.linalg.norm(towards)
    right=np.cross([0,0,1],towards)
    right/=np.linalg.norm(right)
    up=np.cross(towards,right)
    chosen=[p for p in parts if p.role in roles]
    points=np.concatenate([np.asarray(p.solid.vertices) for p in chosen])
    projected=np.column_stack((points@right,points@up))
    lower,upper=projected.min(0),projected.max(0)
    scale=min(700/(upper[0]-lower[0]),720/(upper[1]-lower[1]))
    center=(lower+upper)/2
    faces=[]
    for part in chosen:
        vertices=np.asarray(part.solid.vertices)
        color={'ceramic':'#e8e4da','dark':'#32383d','metal':'#8d9699','shell':'#f8f6ef','wood':'#d7bd99'}.get(part.material,'#ddd')
        for face in part.solid.faces:
            v=vertices[list(face)]
            xy=np.column_stack((v@right,v@up))
            pixel=(xy-center)*[scale,-scale]+[400+index*800,480]
            faces.append((float(np.mean(v@towards)),[tuple(p) for p in pixel],color))
    for depth,polygon,color in sorted(faces,key=lambda p:p[0]):
        draw.polygon(polygon,fill=color,outline='#777777',width=1)
    draw.text((35+index*800,35),title,fill='#202020',font_size=24)
    draw.text((35+index*800,75),'Source-informed reconstruction; dimensions inferred',fill='#444444',font_size=18)
out=TICKET/'🗑️generated/capsule-detail.png'
out.parent.mkdir(exist_ok=True)
image.save(out)
print('[DEBUG] Reference detail inspection saved: capsule-detail.png',flush=True)
