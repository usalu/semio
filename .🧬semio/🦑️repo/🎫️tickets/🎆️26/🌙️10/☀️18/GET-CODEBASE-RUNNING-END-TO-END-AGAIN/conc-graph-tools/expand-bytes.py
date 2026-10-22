import os, sys, json
ROOT = "/Users/ueli/Documents/semio"
if "--root" in sys.argv: ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
def resolve(rel):
    cur = ROOT
    for part in rel.split("/"):
        hits = [n for n in os.listdir(cur) if n.replace("️", "") == part.replace("️", "")]
        assert len(hits) == 1, (part, hits); cur = os.path.join(cur, hits[0])
    return cur
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/"
G = resolve(B + "🟦️.ts"); PJ = resolve(B + "🧪️tests/🕰️generation-protocol/🔣️.json")
OLD = "expand:async(projects:Record<string,unknown>,control:ResourceLeaseControl)=>{const expandChunkV1=64,names=Object.keys(projects),result:Record<string,unknown>={};for(let at=0;at<names.length;at+=expandChunkV1){const chunk:Record<string,unknown>={};for(const name of names.slice(at,at+expandChunkV1))chunk[name]=projects[name];Object.assign(result,await portableInputs(root,chunk,invocation,control) as Record<string,unknown>);}return result;},"
NEW = ("expand:async(projects:Record<string,unknown>,control:ResourceLeaseControl)=>{"
 "const expandChunkV1=32,expandChunkBytesV1=16777216,names=Object.keys(projects),result:Record<string,unknown>={};"
 "const pass=async(group:string[]):Promise<void>=>{const chunk:Record<string,unknown>={};for(const name of group)chunk[name]=projects[name];"
 "try{Object.assign(result,await portableInputs(root,chunk,invocation,control) as Record<string,unknown>);}"
 "catch(error){if(group.length>1&&/capacity exceeded/u.test((error as Error).message)){const half=group.length>>1;await pass(group.slice(0,half));await pass(group.slice(half));return;}throw error;}};"
 "let group:string[]=[],bytes=0;"
 "for(const name of names){const size=JSON.stringify(projects[name]).length;if(group.length&&(group.length>=expandChunkV1||bytes+size>expandChunkBytesV1)){await pass(group);group=[];bytes=0;}group.push(name);bytes+=size;}"
 "if(group.length)await pass(group);return result;},")
t = open(G, encoding="utf8", newline="").read()
if NEW in t: print("applied G")
else:
    assert t.count(OLD) == 1; print("pending G")
    if "--write" in sys.argv: open(G, "w", encoding="utf8", newline="").write(t.replace(OLD, NEW))
raw = open(PJ, encoding="utf8").read(); d = json.loads(raw)
for r in d["present"]:
    if r["name"] == "the host expands the compact inputs in bounded chunks":
        if r["text"] == "const expandChunkV1=32,expandChunkBytesV1=16777216": print("applied law")
        else:
            print("pending law"); r["text"] = "const expandChunkV1=32,expandChunkBytesV1=16777216"
            if "--write" in sys.argv: open(PJ, "w", encoding="utf8", newline="").write(json.dumps(d, ensure_ascii=False, indent=1) + "\n")
