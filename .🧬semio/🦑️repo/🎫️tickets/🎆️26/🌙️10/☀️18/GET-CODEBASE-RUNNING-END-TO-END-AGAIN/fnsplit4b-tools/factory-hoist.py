import re, os, shutil, sys
V = "️"
C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
F = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🔧️pipeline/🏭️factory/🏗️pipeline-resources-construction/🦀️.rs"
D = os.path.dirname(F)
for child in os.listdir(D):
    if os.path.isdir(D + "/" + child):
        shutil.rmtree(D + "/" + child)
shutil.copyfile(C + "/fnsplit4b-tools/orig/factory-construction.rs", F)
lines = open(F, encoding="utf8").read().split("\n")
ranges = [(229, 238), (239, 251), (252, 253), (254, 255), (454, 467), (468, 474)]
docs = ["🧊 Vertex buffer layout of the world3d mesh vertex stream.", "🪞 Instance buffer layout of the world3d mesh instance stream.", "🏷 Vertex buffer layout of the authored surface vertex stream.", "🔗 Instance buffer layout of the authored surface instance stream.", "🎉 Instance buffer layout of the celebration instance stream.", "🧱 Depth stencil state of the material passes: depth write or the depth biased read only variant."]
def conv(block, doc):
    m = re.match(r"\s*let (\w+)\s*=\s*\|(\w*)\|\s*(.*)$", block[0])
    name, arg, rest = m.group(1), m.group(2), m.group(3)
    ret = "gpu_device::VertexBufferLayout<'static>" if not arg else "gpu_device::DepthStencilState"
    body = [rest] + block[1:]
    assert body[-1].rstrip().endswith(";")
    body[-1] = body[-1].rstrip()[:-1]
    body = [(l[4:] if l.startswith("        ") else l) for l in body]
    return ["/// " + doc, "pub(super) fn %s(%s) -> %s {" % (name, "write: bool" if arg else "", ret)] + ["    " + l.strip() if i == 0 else l for i, l in enumerate(body)] + ["}"]
items = [conv(lines[a - 1:b], d) for (a, b), d in zip(ranges, docs)]
cdir = D + "/📏" + V + "vertex-layouts-and-material-depth"
os.makedirs(cdir)
body = ["//! 📏" + V + " Vertex layouts and material depth: the vertex and instance buffer layouts and the depth stencil state shared by the world pipelines. See [`super`].", "", "use super::*;", ""]
body += "\n\n".join("\n".join(o) for o in items).split("\n") + [""]
open(cdir + "/🦀️.rs", "w", encoding="utf8").write("\n".join(body))
for a, b in sorted(ranges, reverse=True):
    del lines[a - 1:b]
i = lines.index("use super::*;")
lines[i + 1:i + 1] = ["", '#[path = "📏' + V + 'vertex-layouts-and-material-depth/🦀️.rs"]', "mod vertex_layouts_and_material_depth;", "use vertex_layouts_and_material_depth::{world_vertex_layout, world_instance_layout, world_surface_vertex_layout, world_surface_instance_layout, celebration_instance_layout, material_depth};"]
open(F, "w", encoding="utf8").write("\n".join(lines))
