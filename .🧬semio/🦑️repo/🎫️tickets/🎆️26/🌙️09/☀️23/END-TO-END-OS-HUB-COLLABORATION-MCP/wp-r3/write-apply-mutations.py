import json, os
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧫️fixtures"
def load(d, n): return json.load(open(os.path.join(ROOT, d, n)))
def arm(d): return load(d, "⬅️before.json")["subset"], load(d, "➡️after.json")["subset"]
def tail(b, a, key): assert len(a[key]) == len(b[key]) + 1 and a[key][:-1] == b[key]; return len(b[key]), a[key][-1]
M = {}
b, a = arm("🌊️apply-flow-applied"); i, n = tail(b, a, "nodes"); M["🌊️apply-flow-applied"] = ("flow", {"mutation": "insertNode", "node": n})
b, a = arm("🎞️apply-animation-applied"); i, t = tail(b, a, "timelines"); M["🎞️apply-animation-applied"] = ("animation", {"mutation": "insertTimeline", "index": i, "timeline": t})
b, a = arm("🎬️apply-video-applied"); i, s = tail(b, a, "streams"); M["🎬️apply-video-applied"] = ("video", {"mutation": "insertStream", "index": i, "stream": s})
b, a = arm("🏛️apply-model-applied"); i, n = tail(b, a, "spatial"); M["🏛️apply-model-applied"] = ("model", {"mutation": "insertSpatialNode", "node": n})
b, a = arm("📐️apply-cad-applied"); i, l = tail(b, a, "layers"); M["📐️apply-cad-applied"] = ("cad", {"mutation": "addLayer", "layer": l})
b, a = arm("📑️apply-document-applied"); i, im = tail(b, a, "images"); M["📑️apply-document-applied"] = ("document", {"mutation": "insertImage", "image": im})
b, a = arm("📦️apply-object-applied"); M["📦️apply-object-applied"] = ("object", {"MoveObject": {"translation": a["transform"]["translation"]}})
b, a = arm("📽️apply-presentation-applied"); i, sl = tail(b, a, "slides"); M["📽️apply-presentation-applied"] = ("presentation", {"mutation": "insertSlide", "index": i, "slide": sl})
b, a = arm("🔊️apply-audio-applied"); i, c = tail(b, a, "channels"); M["🔊️apply-audio-applied"] = ("audio", {"mutation": "insertChannel", "index": i, "channel": c})
b, a = arm("🔢️apply-value-applied")
bi, ai = b["root"]["entries"][2]["value"]["items"], a["root"]["entries"][2]["value"]["items"]
assert b["root"]["entries"][2]["key"] == "models" and ai[:-1] == bi
M["🔢️apply-value-applied"] = ("value", {"mutation": "insertListItem", "path": [{"kind": "key", "key": "models"}], "index": len(bi), "value": ai[-1]})
b, a = arm("🔤️apply-text-applied"); M["🔤️apply-text-applied"] = ("text", {"EditRun": {"index": 0, "new_content": a["runs"][0]["content"]}})
b, a = arm("🔺️apply-mesh-applied"); i, m = tail(b, a, "materials"); M["🔺️apply-mesh-applied"] = ("mesh", {"CreateMaterial": {"material": m}})
b, a = arm("🕸️apply-graph-applied"); M["🕸️apply-graph-applied"] = ("graph", {"ChangeNodeLabel": {"id": a["nodes"][0]["id"], "new_label": a["nodes"][0]["label"]}})
b, a = arm("🖊️apply-drawing-applied"); i, l = tail(b, a, "layers"); M["🖊️apply-drawing-applied"] = ("drawing", {"CreateLayer": {"index": i, "layer": l}})
b, a = arm("🖼️apply-image-applied"); M["🖼️apply-image-applied"] = ("image", {"mutation": "setDimensions", "width": a["width"], "height": a["height"]})
b, a = arm("🗂️apply-table-applied")
col = next(c["name"] for c in b["columns"])
M["🗂️apply-table-applied"] = ("table", {"EditCell": {"row_index": 0, "column_name": col, "new_value": a["rows"][0]["cells"][0]}})
b, a = arm("🧊️apply-brep-applied"); i, v = tail(b, a, "vertices"); M["🧊️apply-brep-applied"] = ("brep", {"CreateVertex": {"id": v["id"], "point": v["point"], "tol": 1e-7}})
b, a = arm("🧰️apply-kit-applied"); M["🧰️apply-kit-applied"] = ("kit", {"RenameType": {"id": a["types"][0]["id"], "new_name": a["types"][0]["name"]}})
for d, (tag, inner) in M.items():
    body = {"mutation": tag, "payload": {"mutation": inner}}
    open(os.path.join(ROOT, d, "🦠️mutation.json"), "w", encoding="utf-8").write(json.dumps(body, indent=2, ensure_ascii=False) + "\n")
    print(d, tag, json.dumps(inner)[:160])
