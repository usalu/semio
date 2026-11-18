import json,sys
FAM="🏗️build/📏️workspace-scale/🧬️schema/🔣️.json"
RETRY="🏗️build/🕸️graph/🔁retry/🧬️schema/🔣️.json"
AUTH="🏗️build/🧬️schema/📥️authority.json"
VALUE=9548
DROP=[("🏗️build/🧬️schema/🔣️.json","properties/admission/properties/selection/oneOf/1/properties/tasks","maxItems"),("🏗️build/🧬️schema/🚪️selection.json","oneOf/1/properties/tasks","maxItems")]
MEMBERS=[{"file":RETRY,"pointer":"$defs/SealPolicy/const","key":"maximumOwners","scale":"owners"},{"file":RETRY,"pointer":"$defs/Seal/properties/laterChangedOwners","key":"maxItems","scale":"owners"},{"file":AUTH,"pointer":"properties/seal/properties/laterChangedOwners","key":"maxItems","scale":"owners"}]
def swap(path,old,new):
    s=open(path,encoding="utf8").read()
    if s.count(old)!=1:sys.exit("anchor count %d in %s"%(s.count(old),path))
    open(path,"w",encoding="utf8").write(s.replace(old,new))
def apply(root,restore=False):
    pairs=[(RETRY,'"maximumOwners": 64','"maximumOwners": %d'%VALUE),(RETRY,'"maxItems": 64','"maxItems": %d'%VALUE),(AUTH,'"maxItems": 64','"maxItems": %d'%VALUE)]
    for f,a,b in pairs:swap(root+f,*(((b,a)) if restore else (a,b)))
    p=root+FAM;d=json.load(open(p,encoding="utf8"));fam=d["definitions"]["Family"]["const"]
    if restore:
        sys.exit("family restore is by sha from the backup copy")
    fam["exempt"]=[r for r in fam["exempt"] if (r["file"],r["pointer"],r["key"]) not in DROP]
    first=len(fam["members"]);fam["members"]+=MEMBERS;fam["equal"].append(list(range(first,first+len(MEMBERS))))
    open(p,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
if __name__=="__main__":apply(sys.argv[1])
