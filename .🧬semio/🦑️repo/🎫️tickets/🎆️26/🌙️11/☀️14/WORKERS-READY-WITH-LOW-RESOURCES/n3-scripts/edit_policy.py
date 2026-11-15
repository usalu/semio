import json,sys
base="/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🧰️toolchains/"
scr="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/n3/"
rows=json.load(open(scr+"rows.json"))
uvos={"darwin":"macos","linux":"linux","win32":"windows"}
uvarch={"arm64":"aarch64","x64":"x86_64"}
doc=json.load(open(base+"🔣️.json",encoding="utf8"))
series={}
for r in rows:
    s=series.setdefault(r["series"],{"version":r["version"],"distributions":[]})
    assert s["version"]==r["version"]
    os_,arch=r["host"].split("-")
    win=os_=="win32"
    s["distributions"].append({"host":r["host"],"archive":r["archive"],"format":"tar.gz","bytes":r["bytes"],"sha256":r["sha256"],"prefix":"python","binaryDirectory":"." if win else "bin","binaries":["python.exe"] if win else ["python"+r["series"]],"directory":f'cpython-{r["version"]}-{uvos[os_]}-{uvarch[arch]}-'+("gnu" if os_=="linux" else "none")})
for s in series.values(): s["distributions"].sort(key=lambda d:d["host"])
py=doc["python"]
new={}
for k,v in py.items():
    new[k]=v
    if k=="tool":
        new["cpython"]={"release":"https://github.com/astral-sh/python-build-standalone/releases/download/20260408","versionArguments":["--version"],"series":dict(sorted(series.items()))}
doc["python"]=new
doc["python"]["variables"]["UV_PYTHON_DOWNLOADS"]="never"
open(base+"🔣️.json","w",encoding="utf8").write(json.dumps(doc,indent=2,ensure_ascii=False)+"\n")
sc=json.load(open(base+"🧬️schema/🔣️.json",encoding="utf8"))
d=sc["definitions"]
d["Distribution"]["properties"]["archive"]["pattern"]="^[A-Za-z0-9._+-]+\\.(?:tar\\.gz|zip)$"
iprops=json.loads(json.dumps(d["Distribution"]["properties"]))
iprops["directory"]={"description":"The directory name below the interpreter directory that uv recognises as a managed installation.","type":"string","pattern":"^cpython-[0-9]+\\.[0-9]+\\.[0-9]+-(?:macos|linux|windows)-(?:aarch64|x86_64)-(?:none|gnu)$","maxLength":64}
d["InterpreterDistribution"]={"description":"One checksum-pinned CPython archive for one host and the uv installation directory it unpacks to.","type":"object","additionalProperties":False,"required":d["Distribution"]["required"]+["directory"],"properties":iprops}
d["Cpython"]={"description":"The checksum-pinned CPython builds of the python-build-standalone release: one entry per interpreter series, each with its full version and one archive per host.","type":"object","additionalProperties":False,"required":["release","versionArguments","series"],"properties":{"release":{"type":"string","pattern":"^https://github\\.com/astral-sh/python-build-standalone/releases/download/[0-9]{8}$"},"versionArguments":d["Tool"]["properties"]["versionArguments"],"series":{"type":"object","minProperties":1,"propertyNames":{"$ref":"#/definitions/Series"},"additionalProperties":{"type":"object","additionalProperties":False,"required":["version","distributions"],"properties":{"version":{"$ref":"#/definitions/Version"},"distributions":{"type":"array","minItems":1,"items":{"$ref":"#/definitions/InterpreterDistribution"}}}}}}}
pp=sc["definitions"]["Python"]
pp["required"]=[x for x in pp["required"]]
newp={}
for k,v in pp["properties"].items():
    newp[k]=v
    if k=="tool": newp["cpython"]={"$ref":"#/definitions/Cpython"}
pp["properties"]=newp
pp["required"].insert(pp["required"].index("tool")+1,"cpython")
# keep definitions order: insert after Distribution
nd={}
for k,v in d.items():
    nd[k]=v
    if k=="Distribution": nd["InterpreterDistribution"]=d["InterpreterDistribution"]
    if k=="Tool": nd["Cpython"]=d["Cpython"]
nd2={}
for k in nd:
    if k in("InterpreterDistribution","Cpython") and k in nd2: continue
    nd2[k]=nd[k]
sc["definitions"]=nd2
open(base+"🧬️schema/🔣️.json","w",encoding="utf8").write(json.dumps(sc,indent=2,ensure_ascii=False)+"\n")
print("ok")
