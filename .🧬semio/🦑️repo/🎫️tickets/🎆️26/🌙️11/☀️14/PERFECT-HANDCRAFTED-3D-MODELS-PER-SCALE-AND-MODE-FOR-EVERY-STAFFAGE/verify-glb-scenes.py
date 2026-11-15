"""🔎️ Lists the glTF scenes, material variants and the scene names of the model JSON of every exported GLB, and checks them against the design (s<scale>-<mode>, seven variants). usage: python3 verify-glb-scenes.py <models dir>"""
import glob, json, os, re, struct, sys

root = sys.argv[1]
bad = 0
for glb in sorted(glob.glob(os.path.join(root, "*", "*.glb"))):
    data = open(glb, "rb").read()
    length = struct.unpack_from("<I", data, 8)[0]
    clen, ctype = struct.unpack_from("<II", data, 12)
    doc = json.loads(data[20:20 + clen])
    scenes = [s.get("name") for s in doc.get("scenes", [])]
    variants = [v["name"] for v in doc.get("extensions", {}).get("KHR_materials_variants", {}).get("variants", [])]
    ok = all(re.fullmatch(r"s(50|100|200|500)-(body|thermal|structure)", s or "") for s in scenes) and len(variants) == 7 and doc.get("scene") == 0
    bad += 0 if ok else 1
    print(("ok  " if ok else "BAD ") + os.path.basename(glb), len(scenes), "scenes", scenes[:3], "...", scenes[-1], "| variants", variants)
for js in sorted(glob.glob(os.path.join(root, "*", "*.pose.json")))[:1] + sorted(glob.glob(os.path.join(root, "*", "*.clip.json")))[:1]:
    model = json.load(open(js))
    print(os.path.basename(js), model.get("schema") or model.get("version"), [s["id"] for s in model.get("scenes", [])])
names = sorted(os.path.basename(f) for f in glob.glob(os.path.join(root, "*", "*")))
pattern = re.compile(r"^[a-z0-9-]+\.[a-z0-9-]+\.(glb|pose\.json|clip\.json)$|^[a-z0-9-]+\.[a-z0-9-]+\.s(50|100|200|500)\.(dark-solid|light-solid|dark-line|light-line|thermal|forces|moments)\.fbx$|^[a-z0-9-]+\.zip$")
odd = [n for n in names if not pattern.match(n)]
print("file names outside the design pattern:", odd)
print("BAD" if bad or odd else "ALL OK", bad)
