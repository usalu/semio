from pathlib import Path
import json,hashlib,subprocess,os
root=Path.cwd();ticket=root/".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END"
command=json.loads((ticket/"drawing-native-owned-owning-command-10.json").read_text());packet=json.loads((ticket/command["inputPacket"]).read_text());rows=packet["inputs"]
def check():
 return [{"path":r["path"],"before":r["sha256"],"after":hashlib.sha256((root/r["path"]).read_bytes()).hexdigest()}for r in rows if hashlib.sha256((root/r["path"]).read_bytes()).hexdigest()!=r["sha256"]]
pre=check()
if pre:raise RuntimeError("Fresh precheck refused: "+json.dumps(pre,ensure_ascii=False))
print("[DEBUG] all "+str(len(rows))+" inputs exact; starting canonical NativeOwned all-targets",flush=True)
env=os.environ.copy();env.update(command["environment"])
with open(command["log"],"wb")as log:exit_code=subprocess.run(command["argv"],stdout=log,stderr=subprocess.STDOUT,env=env).returncode
post=check();result={"exit":exit_code,"preMatched":len(rows),"postMatched":len(rows)-len(post),"total":len(rows),"drift":post,"docsLaunched":False}
if exit_code==0 and not post:
 print("[DEBUG] all-targets terminal0/current exact; starting canonical docs",flush=True)
 with open(command["docsLog"],"wb")as log:result["docsExit"]=subprocess.run(command["docsArgv"],stdout=log,stderr=subprocess.STDOUT,env=env).returncode
 result["docsLaunched"]=True;result["docsDrift"]=check()
Path(command["log"]).with_name("result.json").write_text(json.dumps(result,ensure_ascii=False,indent=2)+"\n")
print(json.dumps(result,ensure_ascii=False),flush=True)
