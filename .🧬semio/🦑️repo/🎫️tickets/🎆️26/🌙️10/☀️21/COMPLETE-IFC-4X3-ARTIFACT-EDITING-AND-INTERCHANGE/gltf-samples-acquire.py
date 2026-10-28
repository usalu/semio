"""📚 Acquire immutable original Khronos sample bytes and license provenance."""
import hashlib
import json
from pathlib import Path
import urllib.error
import urllib.request

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated/gltf-samples"
output.mkdir(parents=True, exist_ok=True)
def fetch(url):
    return urllib.request.urlopen(urllib.request.Request(url, headers={"User-Agent":"semio-ifc-official-fixtures"})).read()

metadata = json.loads(fetch("https://api.github.com/repos/KhronosGroup/glTF-Sample-Assets/commits/15cd02d304e54cb42a4db4eefe719f76190f4b68"))
commit = metadata["sha"]
expected = {"Triangle":"6bf10f2d1e07643cbda968fc6941929d52d4dfd4250182c852103668997f706a","TriangleWithoutIndices":"ee329910d99e32017cc8462eb880a42496b3c2ea56692e7b7ee613af331d631d","BoxInterleaved":"b2ae631f118f1d13f829cdf9d9dc0fe7cb582de20b8c51d17f81f77a1cbf290c","NegativeScaleTest":"ea8c41fd0630f03adcf7a5c06bf86d7b850fa510b6f07cc51eb1fccc205a50f2"}
rows = []
for name in ["Triangle", "TriangleWithoutIndices", "BoxInterleaved", "NegativeScaleTest"]:
    base = f"https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/{commit}/Models/{name}"
    for form, filename in [("glTF-Binary",name+".glb"),("glTF-Embedded",name+".gltf")]:
        try:
            data = fetch(f"{base}/{form}/{filename}")
            break
        except urllib.error.HTTPError as failure:
            if failure.code != 404:
                raise
    else:
        raise RuntimeError("Original self-contained sample not found: "+name)
    license_data = fetch(base+"/README.md")
    assert hashlib.sha256(data).hexdigest() == expected[name]
    (output / filename).write_bytes(data)
    (output / (name+"-README.md")).write_bytes(license_data)
    rows.append({"name":name,"file":filename,"source":f"{base}/{form}/{filename}","bytes":len(data),"sha256":hashlib.sha256(data).hexdigest(),"readmeSource":base+"/README.md","readmeSha256":hashlib.sha256(license_data).hexdigest()})
report = {"repository":"https://github.com/KhronosGroup/glTF-Sample-Assets","commit":commit,"cases":rows}
(output / "provenance.json").write_text(json.dumps(report,indent=2)+"\n",encoding="utf-8")
print("[DEBUG] "+json.dumps(report))
