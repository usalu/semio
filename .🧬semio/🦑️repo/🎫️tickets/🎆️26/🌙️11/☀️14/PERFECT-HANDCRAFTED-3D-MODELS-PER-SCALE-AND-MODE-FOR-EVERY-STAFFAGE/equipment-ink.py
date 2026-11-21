"""🖋️ Draw the small hydraulic and mechanical parts with individual coarse-band pen weights."""
from pathlib import Path
import runpy
import sys

ticket=Path(__file__).resolve().parent
helpers=runpy.run_path(str(ticket/"equipment-sections.py"))
weights={
    "evaporative-humidifier":{"water":.18},
    "finned-heat-exchanger":{"header-block":.2},
    "plate-heat-exchanger":{"damper":.16,"base-rail-rear":.2,"bypass-opening":.15},
}

def style_band(asset):
    """🖍️ Preserve nominal print eligibility and make the selected features legible across the drawn band."""
    band=asset["representations"]["200"]
    band["validRange"]["maxDenominator"]=350
    for shapes in band["geometry"].values():
        for shape in shapes:
            if shape["id"] in weights[asset["id"]]:
                shape["penMm"]=weights[asset["id"]][shape["id"]]


if __name__=="__main__":
    for name in weights:
        path,asset=helpers["document"](name)
        style_band(asset)
        band=asset["representations"]["200"]
        edits=[(["asset","representations","200","validRange"],band["validRange"])]
        for view,shapes in band["geometry"].items():
            for index,shape in enumerate(shapes):
                if shape["id"] in weights[name]:
                    edits.append((["asset","representations","200","geometry",view,index],shape))
        if "--apply" in sys.argv:
            helpers["write"](path,edits)
        else:
            candidate=ticket/"🗑️generated"/"equipment-check"/(name+"-band.json")
            candidate.write_text(path.read_text())
            helpers["write"](candidate,edits)
