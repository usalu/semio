"""Lists the layers of a DXF with entity counts and extents (ezdxf), and audits it."""
import sys, collections
import ezdxf
from ezdxf import colors
doc = ezdxf.readfile(sys.argv[1])
aud = doc.audit()
print("errors", len(aud.errors), "fixes", len(aud.fixes), "note", doc.header.get("$PROJECTNAME"))
n = collections.Counter(e.dxf.layer for e in doc.modelspace())
for layer in doc.layers:
    name = layer.dxf.name
    pts = [p for e in doc.modelspace().query(f'LWPOLYLINE[layer=="{name}"]') for p in e.get_points("xy")]
    box = (round(min(p[0] for p in pts), 3), round(min(p[1] for p in pts), 3), round(max(p[0] for p in pts), 3), round(max(p[1] for p in pts), 3)) if pts else None
    print(name, "aci", layer.color, colors.aci2rgb(layer.color) if 0 < layer.color < 256 else None, "lw", layer.dxf.lineweight, "n", n[name], box)
