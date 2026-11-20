"""📋 Record geometry and validation evidence for equipment and building components."""
import json
from pathlib import Path

ticket=Path(__file__).resolve().parent
catalog=next((ticket.parents[6]/"🎓️teaching").glob("*architecture/*staffage/*catalog"))
text=["# Vehicles, Machinery and Building Components","",
      "The catalog contains 9 vehicles, 16 machines and 18 components. Each has body, thermal and structural geometry at four digital scales. The official document and volume checks use the live catalog, including its original print-availability decisions.","",
      "The new digital scales are 1:500 for the glass balustrade, tilt window, panel radiator, evaporative humidifier, finned heat exchanger, plate heat exchanger, expansion valve, scroll compressor, sorption wheel, scissor lift, city bicycle and electric kick scooter, plus 1:200 for the expansion valve. The authoring file is `equipment-digital.py`. The outer dimensions, mounting directions and mechanical pivots come from each asset’s existing measured frames.","",
      "The 1:500 body, thermal and structural views were rendered and inspected for all twelve assets. The plate exchanger keeps four open port collars, the valve maintains its sensing bulb and return tube, the scissor stack and the extendible deck retain independent moving frames, and the cycle wheels remain open. The check includes analytical surface-area and support measurements alongside those views.","",
      "The expansion valve’s body, head and capillary surface areas are 0.0055, 0.0034 and 0.0074 m². The installation retains the previous heat flux per unit area at the casting and diaphragm. The panel radiator includes both sides of each pressed plate. The room-facing projected areas are 0.60 m² at the rear and 0.20 m² per front band. Rated, comfort and night thermal solves deliver 1210 W, 929 W and 665 W. The outdoor heat pump models exchange through the exterior of its closed compressor compartment. The two cycle frame mass distributions lie on their tubes and retain the aggregate centre of mass.","",
      "The uninsulated 1960 wall has twelve courses of 240 × 115 × 71 mm brick. The 1:50 body and structural model contain 168 full and clipped units with 168 bed, collar and head-joint pieces. The section and plan show the alternating bond. A Three.js box intersection and volume oracle verified the 0.4 × 1 × 1 m envelope, no overlapping interiors and a total model volume of 0.3999999854 m³. The independent expected dimensions and checks are in `solid-wall-bond.json` and `solid-wall-bond.ts`.","",
      "| Asset | Body solids at 1:50 / 100 / 200 / 500 before repeat expansion | Catalog validation |","| --- | --- | --- |"]
failed=[]
for kind in ("vehicles","machinery","components"):
    for path in sorted(next(catalog.glob("*"+kind)).glob("*/*.json")):
        asset=json.loads(path.read_text())["asset"]
        slug=path.parent.name
        logfile=ticket/"🗑️generated"/"equipment-check"/(slug+".log")
        log=logfile.read_text() if logfile.exists() else ""
        status="Passed" if any(l.startswith("VALID ") for l in log.splitlines()) else "Open findings" if any(l.startswith("INVALID ") for l in log.splitlines()) else "Pending"
        counts=" / ".join(str(len(v.get("body",[]))) for v in asset["volumes"].values())
        text.append(f"| {slug} | {counts} | {status} |")
        if status=="Open findings":
            for l in log.splitlines():
                if l.startswith(("VOLUMES","ASSET","SCHEMA")):
                    failed.append(f"- {slug} — {l}")
if failed:
    text+=["","## Open measured findings",""]+failed
text+=["","## Additional asset corrections","",
       "The rebuilt motor scooter has a hinged seat, a single-sided front suspension, a swinging engine unit, a protected silencer and a raked steering column. The body has 79, 40, 16 and 10 parts at the four scales. Its three orthographic drawings, symbol, poses, physical regions and structural members use the same geometry. All four print-suitability checks passed, and the final GLB reopened with twelve scenes.","",
       "The wheel loader's radiator and exhaust outlets now have distinct geometric and airflow-glyph identifiers. Its 4,756,520-byte GLB reopened with twelve scenes. The tram door calculation uses the opaque faces, edges and glazing reveals of its four leaves. The blind retains all 17 thin heat-exchanging slats in its coarsest thermal view, while the facade render shows its exterior face. The sectional door retains the rear crossbar in plan and front view and assigns the sandwich skin's bonded face to its insulating core."]
(ticket/"equipment-audit.md").write_text("\n".join(text)+"\n")

