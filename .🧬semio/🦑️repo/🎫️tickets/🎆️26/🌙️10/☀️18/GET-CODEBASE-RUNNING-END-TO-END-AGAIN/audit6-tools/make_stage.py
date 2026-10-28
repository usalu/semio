"""🏗️ Builds a flat staged batch from a mirror holding the change: base = the LIVE bytes now (absent = new file), new = the mirror bytes.
usage: python3 -I make_stage.py <batch> <mirror-root> "<one-line purpose>" <rel>...   -> staged/<batch>/{manifest.json,files/,base/,stage.py}"""
import hashlib,json,os,shutil,sys
here=os.path.dirname(os.path.abspath(__file__));C=os.path.dirname(here);live=os.path.dirname(C)
while not os.path.isdir(os.path.join(live,"🧰️framework")):live=os.path.dirname(live)
batch,mirror,purpose,rels=sys.argv[1],sys.argv[2],sys.argv[3],sys.argv[4:]
out=os.path.join(here,"staged",batch);shutil.rmtree(out,ignore_errors=True);os.makedirs(os.path.join(out,"files"));os.makedirs(os.path.join(out,"base"))
sha=lambda b:hashlib.sha256(b).hexdigest();manifest={}
for i,rel in enumerate(rels):
  ext=os.path.splitext(rel)[1];name=f"{i:02d}{ext}";new=open(os.path.join(mirror,rel),"rb").read();lp=os.path.join(live,rel)
  base=open(lp,"rb").read() if os.path.exists(lp) else None
  open(os.path.join(out,"files",name),"wb").write(new)
  if base is not None:open(os.path.join(out,"base",name),"wb").write(base)
  manifest[rel]={"file":name,"base":sha(base) if base is not None else None,"new":sha(new)}
json.dump(manifest,open(os.path.join(out,"manifest.json"),"w",encoding="utf8"),ensure_ascii=False,indent=1)
template=open(os.path.join(C,"xplat5-tools","stage.template.py"),encoding="utf8").read().split("\n")
template[1]=f'"""{purpose}'
open(os.path.join(out,"stage.py"),"w",encoding="utf8").write("\n".join(template))
print(out,len(rels),"files")
