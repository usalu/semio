import sys,os,json,hashlib
F="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs"
def read(p): return open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def sha(t): return hashlib.sha256(t.encode()).hexdigest()
OLD='    await requireCurrentMembershipV1(owner, control, current);\n    await control.advance({ operation: "membership"'
NEW='    await control.advance({ operation: "membership"'
cmd,backup=sys.argv[1],sys.argv[2]
if cmd=="apply":
    old=read(F);assert old.count(OLD)==1,("anchor count",old.count(OLD));new=old.replace(OLD,NEW)
    os.makedirs(backup,exist_ok=True);write(backup+"/0.bak",old);json.dump({"before":sha(old),"after":sha(new)},open(backup+"/manifest.json","w"));write(F,new);print("applied")
else:
    m=json.load(open(backup+"/manifest.json"));now=sha(read(F))
    if now==m["after"]:write(F,read(backup+"/0.bak"));print("restored")
    elif now==m["before"]:print("already original")
    else:print("CONFLICT");sys.exit(4)
