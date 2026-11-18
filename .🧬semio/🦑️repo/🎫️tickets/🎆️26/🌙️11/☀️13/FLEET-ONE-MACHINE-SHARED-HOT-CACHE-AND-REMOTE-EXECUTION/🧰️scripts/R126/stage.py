import json,os,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R126/"
H=os.path.dirname(os.path.abspath(__file__))+"/"
files=json.load(open(H+"files.json",encoding="utf8"))
os.makedirs(S+"tests",exist_ok=True)
shim=S+"tests/shim.ts"
open(shim,"w",encoding="utf8").write('''import{readFileSync as real}from"node:fs";
const map=new Map<string,string>();for(const l of (process.env.R126_OVER??"").split("\\n").filter(Boolean)){const at=l.indexOf("=");map.set("/Users/ueli/Documents/semio/"+l.slice(0,at),l.slice(at+1));}
export const readFileSync=((path:any,...rest:any[])=>real(typeof path==="string"?map.get(path)??path:path instanceof URL?map.get(decodeURIComponent(path.pathname))??path:path,...rest))as typeof real;
''')
for live,name in files["mods"]:
    if name.startswith("t-"):
        text=open(S+"stage/"+name,encoding="utf8").read().replace("import{mkdtempSync,readFileSync,rmSync}from\"node:fs\";","import{mkdtempSync,rmSync}from\"node:fs\";\nimport{readFileSync}from\"%s\";"%shim)
        open(S+"tests/"+name,"w",encoding="utf8").write(text)
for final,name,host in files["newTests"]:
    text=open(S+"stage/"+name,encoding="utf8").read().replace('import{readFileSync}from"node:fs";','import{readFileSync}from"%s";'%shim).replace('import{readFileSync as readSource}from"node:fs";','import{readFileSync as readSource}from"%s";'%shim)
    open(S+"tests/"+name,"w",encoding="utf8").write(text)
if len(sys.argv)>1:
    mode=sys.argv[1]
    if mode in("overlay","tests"):
        if mode=="overlay":
            for live,n in files["mods"]: print(live+"="+S+("tests/" if n.startswith("t-") else "stage/")+n)
        for final,name,host in files["newTests"]: print(host+"="+S+"tests/"+name)
    elif mode=="host":
        for final,name,host in files["newTests"]:
            if name==sys.argv[2]: print(host)
