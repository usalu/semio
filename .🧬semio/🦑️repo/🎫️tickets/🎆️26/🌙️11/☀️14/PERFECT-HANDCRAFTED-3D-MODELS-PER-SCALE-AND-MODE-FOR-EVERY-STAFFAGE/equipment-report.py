"""📋 Record geometry and validation evidence for equipment and building components."""
import json
from datetime import datetime
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
        source=ticket/"volumes"/kind/(slug+".json")
        same_source=json.loads(source.read_text())["volumes"]==asset["volumes"]
        candidates=[logfile,ticket/"🗑️generated"/("r5-"+slug)/"validate-catalog.txt"]
        if same_source:
            candidates.append(ticket/"🗑️generated"/("r5-"+slug)/"validate.txt")
        completed=[]
        for proof in candidates:
            if not proof.exists():
                continue
            contents=proof.read_text(errors="replace").replace("\0","\n")
            lines=[line for line in contents.splitlines() if line.startswith(("VALID ","INVALID ")) and "checkVolumes not run" not in line]
            if lines:
                completed.append((proof.stat().st_mtime,lines[-1],contents))
        status="Pending"
        if completed:
            stamp,line,log=max(completed)
            live=path.stat().st_mtime<=stamp
            status=("Passed" if line.startswith("VALID ") else "Open findings") if live else "Changed since "+("pass" if line.startswith("VALID ") else "findings")
            status+=f" ({datetime.fromtimestamp(stamp):%b %d %H:%M})"
        else:
            log=""
        counts=" / ".join(str(len(v.get("body",[]))) for v in asset["volumes"].values())
        text.append(f"| {slug} | {counts} | {status} |")
        if status.startswith("Open findings"):
            for l in log.splitlines():
                if l.startswith(("VOLUMES","ASSET","SCHEMA","PRINT")):
                    failed.append(f"- {slug} — {l}")
if failed:
    text+=["","## Open measured findings",""]+failed
text+=["","## Additional asset corrections","",
       "The rebuilt motor scooter has a hinged seat, a single-sided front suspension, a swinging engine unit, a protected silencer and a raked steering column. The first published model had 79, 40, 16 and 10 body parts. The current revision has 84, 42, 16 and 11. Its three orthographic drawings, symbol, poses, physical regions and structural members use the same geometry. All four print-suitability checks passed, and the final GLB reopened with twelve scenes.","",
       "The wheel loader's radiator and exhaust outlets now have distinct geometric and airflow-glyph identifiers. Its 4,756,520-byte GLB reopened with twelve scenes. The tram door calculation uses the opaque faces, edges and glazing reveals of its four leaves. The blind retains all 17 thin heat-exchanging slats in its coarsest thermal view, while the facade render shows its exterior face. The sectional door retains its rear crossbar in plan and front view."]
text+=["","## November 21 verification","",
       "The four 1:500 digital models for the tilt window, evaporative humidifier and both heat exchangers were restored after an overnight source rebuild. Each recovered level, its authoring function and its ticket source compare equal to the catalog JSON. The active builders now call those same handcrafted definitions. The 1:200 drawings keep the humidifier water level, finned-coil header and plate exchanger’s damper, rear base rail and bypass opening legible with individual pen weights. All three drawings passed both range endpoints, including 1:350.","",
       "The masonry wall with lintel has 248 mm blocks on a 250 mm module. The sliding door’s two track brushes leave a 1 mm working slit. The sectional door’s spring turns use 7.8 mm axial wire sections on an 8 mm pitch. All three GLBs round-tripped twelve scenes with zero geometric topology findings. The GLB sizes were 2,795,480, 1,488,036 and 12,559,244 bytes respectively.","",
       "The spiral stair’s 43 balusters, two end posts, post flanges and helical rail yield 2.592 m² of exposed steel from their lengths and diameters. The prior declaration of 1.1917 m² omitted part of the rail assembly. A new complete run is required for that change. The steel column, tilt window, wheelchair ramp, evaporative humidifier and both heat exchangers passed current full catalog validation.","",
       "The table uses the latest completed full product check for each asset and marks a file edited since that check. It also accepts fleet merged-source checks where all four ticket volumes exactly match the catalog. Older runs used earlier validator versions. All 172 equipment volumes now match their author sources after resolving the city bicycle brake cables and the exposed steel/glass balustrade handrail. The catalog now includes the delivery van’s lofted mirror housings, proud bumper skin and three pressed arch lips. The sliding-door track occupies the fourth arch. The vintage saloon’s volumes now match the revised wheelbase, suspension and hand-drawn period equipment, including its steering dampers, bonnet mascot and vacuum wiper. For these, only a catalog-only result counts as product evidence.","",
       "The tower crane’s 1:50 elevation draws the true cross-sections of the mast, boom and counter-jib chords and places two 72 mm diameter axial spigots inside the heel chords. Its elevation measured 1.875 mm p95 and 0.99825 IoU against the candidate body. The two joint pins at each of the three finer scales lie entirely inside their 120 mm chords. The 1:20 lifting-load print check passed. The live model and author source match. A complete product run is still pending."]
text += ["", "The later per-document schema checks, scoped alternative resolution, wheel-loader cab, mixer barrel and analytical shaft evidence are recorded in [Equipment revisions and scoped catalog validation](equipment-scoped.md). The dedicated physiological outlet rule and oracle are recorded in [Breath-aperture validation](breath-apertures.md)."]
(ticket/"equipment-audit.md").write_text("\n".join(text)+"\n")

