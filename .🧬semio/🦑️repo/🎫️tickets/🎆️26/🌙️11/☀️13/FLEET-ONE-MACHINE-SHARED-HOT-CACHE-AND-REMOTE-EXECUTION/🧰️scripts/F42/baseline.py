import json,subprocess,sys
root="/Users/ueli/Documents/semio"
out=subprocess.run(["git","grep","-cE",r"process\.kill\([^)]*, *0 *\)","--","*.ts","*.mjs","*.js"],cwd=root,capture_output=True,text=True).stdout
sites={}
for line in out.splitlines():
    path,_,count=line.rpartition(":")
    if "🧪️tests" in path or "🎫️tickets" in path or path.startswith("🧰️framework/🔨️modules/🏃️process/🔖️identity/"):continue
    sites[path]={"count":int(count)}
doc={"description":"Production files that still judge a process by pid alone (process.kill(pid, 0)); the law test lets this list only shrink. Convert a file to the identity module (processHolderAliveV1 with a persisted {pid, startMs}) and lower or remove its entry.","sites":dict(sorted(sites.items()))}
target=root+"/🧰️framework/🔨️modules/🏃️process/🔖️identity/🧪️tests/🔒️law/🔣️.json"
tmp=target+".tmp"
open(tmp,"w",encoding="utf8").write(json.dumps(doc,ensure_ascii=False,indent=1)+"\n")
import os;os.rename(tmp,target)
print(len(sites))
