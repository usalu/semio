"""🧭 Audit four exterior directions and shared capsule, core and podium sections."""
import importlib
import argparse
import math
from pathlib import Path
import sys
import numpy as np
parser=argparse.ArgumentParser()
mode=parser.add_mutually_exclusive_group(required=True)
mode.add_argument('--direction-index',type=int,choices=range(4))
mode.add_argument('--sections-only',action='store_true')
args=parser.parse_args()
sys.path.insert(0, str(Path.cwd()))
V = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
M, E = V._model, V._engine
output = Path(__file__).parent / "🗑️generated"
output.mkdir(exist_ok=True)
E.config.pixel_width, E.config.pixel_height = 1920, 1080
E.config.frame_width, E.config.frame_height = 16, 9
scene, drafting = V.NakaginDrawing(), V._d.Drafting()

def text(label, at):
    return drafting.text(label, at, size=22, color=V.INK)

def save(name, objects):
    camera = E.Camera(background_color=V.BACKGROUND)
    camera.capture_mobjects(objects)
    camera.get_image().save(output / name)
    print("[DEBUG] saved " + name, flush=True)

if args.direction_index is not None:
    index=args.direction_index
    x,azimuth=(-5.8,-1.95,1.95,5.8)[index],(45,135,225,315)[index]
    drawing = scene.tower_axon(at=(x, 0, 0), scale=.085, view=(azimuth, 24), types=False, ground=True)
    assert len(drawing.capsules) == 140
    direction = drawing.view.towards
    label = ("N" if direction[1] > 0 else "S") + " / " + ("E" if direction[0] > 0 else "W")
    camera=E.Camera(background_color=V.BACKGROUND)
    annotation=text(label,(x,3.55,0))
    left,right=(0,495,960,1425,1920)[index:index+2]
    limits=np.array([left,right])*16/1920-8
    assert drawing.get_left()[0]>=limits[0] and drawing.get_right()[0]<=limits[1]
    camera.capture_mobjects([drawing,annotation])
    camera.get_image().save(output/f'exterior-direction-{index}.png')
    print("[DEBUG] exterior " + label, flush=True)
    raise SystemExit(0)

def clipped(mesh, axis, limit, less):
    """✂️ Retain the production source-plane cut without inferring section caps."""
    normal=np.zeros(3)
    normal[axis]=1 if less else -1
    result=V._clip_mesh(mesh,((normal,float(normal[axis]*limit)),))
    return result if result.faces else None


def section(parts, at, scale):
    groups = {}
    items = V._assembly_items(parts, base=V.CONCRETE, groups=groups)
    vertices = np.vstack([item.verts for item in items])
    centre = (vertices.min(axis=0) + vertices.max(axis=0)) / 2
    view = V.make_view((45, 28), at, scale, centre)
    drawing = V.NakaginDrawing3D(view)
    drawing.add(*V._paint_ordered(V._section_order(items,view),view))
    return drawing

lower, upper = M.podium_height(), M.podium_height() + 2 * M.value("core.storey_height").value
core_parts = []
for part, mesh in V._parts(M.core_assembly(0, detail="full")):
    if part.role in ("roof", "vent") or part.key.startswith(("wall-0-", "wall-1-", "lift-shaft-north", "lift-shaft-east")):
        continue
    for axis, limit, less in ((2, lower, False), (2, upper, True)):
        mesh = clipped(mesh, axis, limit, less) if mesh is not None else None
    if mesh is not None:
        core_parts.append((part, mesh))
core = section(core_parts, (0, 0, 0), .42)
roles = {shape.semantic_role for shape in core}
assert {"stair_tread", "landing", "lift_shaft", "service_riser"} <= roles
podium_parts = V._parts(M.podium_assembly())
all_vertices = np.vstack([mesh.V for _, mesh in podium_parts])
cut_y = (all_vertices[:, 1].min() + all_vertices[:, 1].max()) / 2
podium_parts = [(part, cut) for part, mesh in podium_parts if (cut := clipped(mesh, 1, cut_y, True)) is not None]
podium = section(podium_parts, (5.1, 0, 0), .14)
capsule = scene.capsule_axon(M.capsule_type("A"), at=(-5.1, 0, 0), scale=.8, cutaway=True, interior="reference")
save("nakagin-representative-sections.png", [capsule, core, podium,
     text("Capsule / Kapsel", (-5.1, 3.5, 0)), text("Core / Kern", (0, 3.5, 0)), text("Podium / Sockel", (5.1, 3.5, 0)),
     text("Open cuts / Offene Schnitte", (0, -3.9, 0)),
     text(f"Z {lower:.2f}–{upper:.2f} m", (0, -3.5, 0))])
