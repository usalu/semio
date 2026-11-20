"""🚪 Align the sectional door's rear brace in the 1:100 drawing and solid, and keep the external sandwich skin outside its foam core."""
from pathlib import Path
import runpy

ticket=Path(__file__).resolve().parent
h=runpy.run_path(str(ticket/"equipment-sections.py"))
path,asset=h["document"]("sectional-garage-door")
v=asset["volumes"]
bar=next(p for p in v["50"]["body"] if p["id"]=="track-end-bar")
body=[p for p in v["100"]["body"] if p["id"]!=bar["id"]]+[bar]
thermal={k:[p.copy() for p in v[k]["thermal"]] for k in ("50","100","200")}
for scale,regions in thermal.items():
    for patch in regions:
        if patch["region"]=="panel-sandwich" and patch.get("layer")=="skin-outer":
            patch["sectionFaces"]=["-v"]
thermal["100"]=[p for p in thermal["100"] if p["id"]!="th-track-end-bar"]+[{"id":"th-track-end-bar","of":"track-end-bar","region":"track-steel","layer":"track-wall"}]
body200=[p for p in v["200"]["body"] if p["id"]!=bar["id"]]+[{**bar,"size":[.03,.04,2.42],"transform":{"translation":[2.585,2.5,0]}}]
thermal["200"]=[p for p in thermal["200"] if p["id"]!="th-track-end-bar"]+[{"id":"th-track-end-bar","of":"track-end-bar","region":"track-steel","layer":"track-wall"}]
edits=[(["100","body"],body),(["200","body"],body200)]+[([k,"thermal"],th) for k,th in thermal.items()]
for file,prefix in [(path,["asset","volumes"]),(ticket/"volumes/components/sectional-garage-door.json",["volumes"])]:
    h["write"](file,[(prefix+keys,val) for keys,val in edits])
regions=asset["thermal"]["regions"]
eds=[(["asset","thermal","regions",next(i for i,r in enumerate(regions) if r["id"]==name),"areaM2"],val) for name,val in [("hardware-steel",.43),("spring-assembly",1.03)]]
for view in ("plan","front"):
    drawing=asset["representations"]["100"]["geometry"][view]
    brace=next(p for p in asset["representations"]["50"]["geometry"][view] if p["id"]=="track-end-bar")
    for k in ("100","200"):
        drawing=asset["representations"][k]["geometry"][view]
        eds.append((["asset","representations",k,"geometry",view],[p for p in drawing if p["id"]!="track-end-bar"]+[brace]))
h["write"](path,eds)

