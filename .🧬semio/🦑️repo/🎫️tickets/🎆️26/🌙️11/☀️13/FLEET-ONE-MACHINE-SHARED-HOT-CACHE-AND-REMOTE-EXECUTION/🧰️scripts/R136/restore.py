import json,os,shutil
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R136"
for e in json.load(open(SC+"/backup/manifest.json")):
    if e["backup"] is None:
        if os.path.exists(e["file"]):os.remove(e["file"])
        d=os.path.dirname(e["file"])
        try:os.rmdir(d)
        except OSError:pass
    else:shutil.copyfile(e["backup"],e["file"])
print("restored")
