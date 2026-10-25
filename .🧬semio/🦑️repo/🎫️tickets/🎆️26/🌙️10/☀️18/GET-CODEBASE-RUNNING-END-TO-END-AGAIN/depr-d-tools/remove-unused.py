import json,sys
R="/Users/ueli/Documents/semio/"
edits={
 "package.json":['    "color": "4.2.3",\n','    "npm-run-path": "^6.0.0",\n'],
 "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json":['    "terser": "5.49.1",\n'],
 "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/package.json":['    "terser": "5.49.1",\n'],
}
for path,lines in edits.items():
    s=open(R+path,encoding="utf8").read()
    for l in lines:
        if s.count(l)!=1: sys.exit(f"{path}: {l!r} count {s.count(l)}")
    for l in lines: s=s.replace(l,"")
    json.loads(s)
    open(R+path,"w",encoding="utf8").write(s)
    print("edited",path[-60:],len(lines))
