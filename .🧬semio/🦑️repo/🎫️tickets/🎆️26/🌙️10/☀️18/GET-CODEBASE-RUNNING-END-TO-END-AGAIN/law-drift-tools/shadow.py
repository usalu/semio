#!/usr/bin/env python3
import os,sys,shutil,subprocess,json,tempfile
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
SH=C+"/🗑️generated/law-drift-run/shadow"
test=sys.argv[1]
overlays={}
for a in sys.argv[2:]:
    if "=" in a and a.split("=",1)[0].endswith(".ts") or a.split("=",1)[0].endswith(".json") : k,v=a.split("=",1); overlays[k]=v
    else: overlays[test]=a
if os.path.exists(SH): shutil.rmtree(SH)
os.makedirs(SH)
realdirs={"",".cache"}
for rel in list(overlays)+[test]:
    p=rel.split("/")[:-1]
    for i in range(len(p)+1): realdirs.add("/".join(p[:i]))
def build(relpath):
    real=ROOT+("/"+relpath if relpath else ""); shadow=SH+("/"+relpath if relpath else "")
    os.makedirs(shadow,exist_ok=True)
    for name in os.listdir(real):
        child=(relpath+"/"+name) if relpath else name
        if child in realdirs: build(child)
        elif child in overlays: shutil.copyfile(overlays[child],shadow+"/"+name)
        elif child==test: shutil.copyfile(real+"/"+name,shadow+"/"+name)
        else: os.symlink(real+"/"+name,shadow+"/"+name)
build("")
env=dict(os.environ,NO_COLOR="1",REPO_ROOT=SH,SEMIO_REPO_ROOT=SH,SEMIO_TICKET_DIR=C,SEMIO_BUILD_MODE="prototype")
wire=json.loads(subprocess.run(["bun",C+"/law-drift-tools/envwire.ts"],cwd=ROOT,capture_output=True,text=True).stdout)
env.update(wire)
ta=os.environ.get("TA") or (C+"/🗑️generated/l")
os.makedirs(ta,exist_ok=True)
env["SEMIO_TEST_ARTIFACT_DIR"]=tempfile.mkdtemp(dir=ta)
p=subprocess.run(["perl","-e","alarm 280; exec @ARGV","--","bun","test","--timeout","120000","./"+test],cwd=SH,env=env,capture_output=True,text=True)
sys.stdout.write((p.stdout+p.stderr)[-60000:])
shutil.rmtree(env["SEMIO_TEST_ARTIFACT_DIR"],ignore_errors=True)
