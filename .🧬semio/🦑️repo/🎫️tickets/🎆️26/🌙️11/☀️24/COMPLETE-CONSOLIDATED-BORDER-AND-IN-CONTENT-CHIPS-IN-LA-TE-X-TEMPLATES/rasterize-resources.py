"""🖼️ Rasterizes the SVG resources of an assignment's resource plan (`icons.json`) into a scratch copy of its sources,
so the document can be compiled with its real images outside the Print pipeline.

usage: python rasterize-resources.py <assignment dir relative to repo> <name>
"""
import hashlib, io, json, os, shutil, sys
import pymupdf

repo = "C:/semio"
source, name = sys.argv[1], sys.argv[2]
copy = os.path.join(os.environ["SEMIO_PRINT_SCRATCH"], "sources", name)
shutil.rmtree(copy, ignore_errors=True)
shutil.copytree(os.path.join(repo, source), copy)
plan = json.load(io.open(os.path.join(copy, "icons.json"), encoding="utf-8"))
assert plan["format"] == "svg-to-raster-png" and plan["sourceRoot"] == "workspace"
for resource in plan["resources"]:
    data = open(os.path.join(repo, resource["source"]), "rb").read()
    assert hashlib.sha256(data).hexdigest() == resource["sha256"], resource["source"]
    page = pymupdf.open(stream=data, filetype="svg")[0]
    scale = resource["width"] / page.rect.width
    target = os.path.join(copy, resource["output"])
    os.makedirs(os.path.dirname(target), exist_ok=True)
    page.get_pixmap(matrix=pymupdf.Matrix(scale, scale), alpha=True).save(target)
print(len(plan["resources"]), "resources rasterized into", copy)
