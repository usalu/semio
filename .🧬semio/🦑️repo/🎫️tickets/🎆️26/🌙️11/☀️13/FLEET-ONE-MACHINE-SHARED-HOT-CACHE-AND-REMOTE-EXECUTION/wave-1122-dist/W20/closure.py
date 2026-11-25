import json,os,re,sys
lib="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
files=json.load(open(lib+"🏡️resident/🧬️schema/🔣️.json"))["definitions"]["HostPolicy"]["const"]["serviceRevisionFiles"]
rx=re.compile(r'''(?:from\s*|import\s*\(\s*|import\s*)["'](\.{1,2}/[^"']+)["']''')
def closure(entry):
    seen=set();todo=[os.path.normpath(entry)]
    while todo:
        p=todo.pop()
        if p in seen or not os.path.isfile(p):continue
        seen.add(p)
        if not p.endswith((".ts",".mjs",".js")):continue
        for s in rx.findall(open(p,encoding="utf8").read()):
            todo.append(os.path.normpath(os.path.join(os.path.dirname(p),s)))
    return seen
listed={os.path.normpath(lib+f) for f in files}
allc=set()
for f in files:
    c=closure(lib+f);allc|=c;print(len(c),f)
print("union",len(allc),"listed",len(listed))
for p in sorted(allc-listed):
    if "host-capacity" in p: print("unlisted:",p.replace("/Users/ueli/Documents/semio/",""))
