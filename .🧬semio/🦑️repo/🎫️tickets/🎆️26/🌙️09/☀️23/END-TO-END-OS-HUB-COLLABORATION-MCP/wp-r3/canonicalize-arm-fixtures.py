import json, os
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧫️fixtures"
def edit(folder, fn):
    for name in ("⬅️before.json", "➡️after.json", "🦠️mutation.json"):
        path = os.path.join(ROOT, folder, name)
        value = json.load(open(path, encoding="utf-8"))
        value = fn(name, value)
        open(path, "w", encoding="utf-8").write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")
def walk(value, visit):
    if isinstance(value, dict):
        value = visit(value)
        return {k: walk(v, visit) for k, v in value.items()}
    if isinstance(value, list):
        return [walk(v, visit) for v in value]
    return value
def document(name, value):
    def visit(obj):
        if obj.get("kind") in ("paragraph", "heading", "image"):
            obj = {("style_id" if k == "styleId" else "image_id" if k == "imageId" else k): v for k, v in obj.items()}
        return obj
    return walk(value, visit)
def video(name, value):
    def visit(obj):
        if "codec" in obj and obj.get("kind") in ("V", "A"):
            obj = dict(obj, kind={"V": "video", "A": "audio"}[obj["kind"]])
        if "pts" in obj and isinstance(obj.get("data"), str):
            obj = dict(obj, data=list(bytes.fromhex(obj["data"])))
        return obj
    return walk(value, visit)
def brep(name, value):
    def visit(obj):
        if "point" in obj and "id" in obj and "tol" not in obj and name != "🦠️mutation.json":
            obj = dict(obj, tol=1e-7)
        return obj
    return walk(value, visit)
def obj(name, value):
    def visit(o):
        if "childId" in o and "target" in o:
            o = dict(o, childId=o["target"]["artifactId"])
        return o
    return walk(value, visit)
edit("📑️apply-document-applied", document)
edit("🎬️apply-video-applied", video)
edit("🧊️apply-brep-applied", brep)
edit("📦️apply-object-applied", obj)
print("ok")
