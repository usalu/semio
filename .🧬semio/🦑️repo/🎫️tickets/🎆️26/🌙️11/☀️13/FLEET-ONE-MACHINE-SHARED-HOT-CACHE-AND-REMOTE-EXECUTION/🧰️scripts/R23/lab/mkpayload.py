import json,hashlib,os,shutil,sys
T="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R23"
base=json.load(open("base.json",encoding="utf8"));ROOT="/Users/Shared/semio/"
sha=lambda b:hashlib.sha256(b).hexdigest()
shutil.rmtree(T+"/new");os.makedirs(T+"/new")
files=[];n=0
for d,_,fs in sorted(os.walk("ov")):
    for f in sorted(fs):
        rel=os.path.join(d,f)[3:];data=open("ov/"+rel,"rb").read()
        if rel in base:
            if sha(data)==base[rel]:continue
            kind="edit";b=base[rel]
        else:
            if os.path.exists(ROOT+rel) and open(ROOT+rel,"rb").read()==data:continue
            if os.path.exists(ROOT+rel):sys.exit("new file exists live with other content: "+rel)
            kind="new";b=None
        n+=1;flat="%02d%s"%(n,os.path.splitext(f)[1]);open(T+"/new/"+flat,"wb").write(data)
        files.append({"dest":rel,"flat":flat,"kind":kind,"base":b,"new":sha(data)})
json.dump(files,open(T+"/payload.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
print(len(files),"files",sum(1 for x in files if x["kind"]=="edit"),"edited",sum(1 for x in files if x["kind"]=="new"),"new")
for x in files:print(x["kind"],x["dest"][len("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"):])
