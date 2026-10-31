"""🔎 Inspect existing native capsule geometry names, layers and bounds without copying source assets."""
from pathlib import Path
import json
import rhino3dm as rh
import sys
sys.stdout.reconfigure(errors='backslashreplace')
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
source=root/'🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/↗️slash/📐️capsule_slash.3dm'
model=rh.File3dm.Read(str(source))
rows=[]
for index,item in enumerate(model.Objects):
    geometry=item.Geometry
    bounds=geometry.GetBoundingBox()
    rows.append({'index':index,'name':item.Attributes.Name,'layer':model.Layers[item.Attributes.LayerIndex].Name,'type':type(geometry).__name__,'bounds':[[bounds.Min.X,bounds.Min.Y,bounds.Min.Z],[bounds.Max.X,bounds.Max.Y,bounds.Max.Z]]})
folder=Path(__file__).parent
(folder/'🗑️generated'/'capsule-cad.json').write_text(json.dumps(rows,ensure_ascii=False,indent=2),encoding='utf8')
lines=['# Capsule Detail Evidence','', '## Existing Native CAD','',f'Read {len(rows)} objects from {source.relative_to(root)}. Source assets are not copied. Bounding coordinates remain native CAD coordinates; this is evidence of the repository design fixture, not a measured historic inventory.','', '| Object | Name | Layer | Type | Bounds |','|---:|---|---|---|---|']
lines.extend(f'| {r["index"]} | {r["name"]} | {r["layer"]} | {r["type"]} | {r["bounds"]} |' for r in rows)
(folder/'capsule-detail-evidence.md').write_text('\n'.join(lines)+'\n',encoding='utf8')
print('[DEBUG] '+json.dumps({'objects':len(rows),'layers':[layer.Name for layer in model.Layers],'types':sorted({r['type'] for r in rows})},ensure_ascii=False),flush=True)
