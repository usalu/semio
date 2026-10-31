"""📏️ Check conservative source-space gallery bounds before expensive tower painting."""
import importlib
import json
from pathlib import Path
import numpy as np

V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
TICKET=Path(__file__).resolve().parent
scene=V.NakaginDrawing()
items=scene._tower_items(lit=None,tint=None,types=False,capital=True,podium=True,where=None,core=True,ground=True,segments=2,capsules={},cores={},capitals={},base=V.CAPSULE,towers=(0,1),storeys=None)
vertices=np.vstack([item.verts for item in items])
centre=(vertices.min(axis=0)+vertices.max(axis=0))/2
for item in items:
    if item.tier<0:
        low,high=item.verts.min(axis=0),item.verts.max(axis=0)
        vertices=np.vstack((vertices,[(x,y,0) for x in (low[0],high[0]) for y in (low[1],high[1])]))
boundaries=(0,495,960,1425,1920)
rows=[]
for index,(x,azimuth) in enumerate(zip((-5.8,-1.95,1.95,5.8),(45,135,225,315))):
    view=V.make_view((azimuth,24),(x,0,0),.092,centre)
    points=V._screen(view,vertices)
    limits=np.asarray(boundaries[index:index+2])*16/1920-8
    bounds=(points.min(axis=0),points.max(axis=0))
    basis,_=V._basis(view)
    local=(vertices-centre)@basis.T
    available=min(x-limits[0],limits[1]-x)-.03
    suggested=min(.092,available/np.max(np.abs(local[:,0])))
    rows.append({'azimuth':azimuth,'scale':.092,'panel_x':limits.tolist(),'source_x':[float(bounds[0][0]),float(bounds[1][0])],'fits':bool(bounds[0][0]>=limits[0] and bounds[1][0]<=limits[1]),'conservative_scale':float(suggested)})
record={'items':len(items),'source_bounds':[vertices.min(axis=0).tolist(),vertices.max(axis=0).tolist()],'views':rows}
(TICKET/'🗑️generated/exterior-layout-preflight.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] Conservative exterior layout '+json.dumps(rows),flush=True)
