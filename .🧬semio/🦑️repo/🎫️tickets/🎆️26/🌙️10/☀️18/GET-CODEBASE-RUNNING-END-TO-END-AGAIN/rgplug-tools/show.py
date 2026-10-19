import json,sys,os,subprocess
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
m=json.load(open(C+"/rgplug-tools/manifests.json"))
for n in sys.argv[1:]:
    k="semio-s-artifact-"+n
    subprocess.run(["bun",C+"/rgplug-tools/allclosure.ts","/tmp/x.json",m[k]],cwd="/Users/ueli/Documents/semio",capture_output=True,text=True)
    d=json.load(open("/tmp/x.json"))[m[k]]
    base=os.path.dirname(os.path.dirname(os.path.dirname(m[k])))
    print("=====",k,"base=",base[-90:])
    for x in d: print(x["l"],x["b"],x["p"].replace(base,"~"))
