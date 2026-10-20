import hashlib,json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__))
ROOT=sys.argv[1] if len(sys.argv)>1 and not sys.argv[1].startswith("--") else None
if ROOT is None:
    p=HERE
    while p!=os.path.dirname(p):
        if os.path.isfile(os.path.join(p,"bunfig.toml")) and os.path.isfile(os.path.join(p,"bun.lock")):ROOT=p;break
        p=os.path.dirname(p)
WRITE="--write" in sys.argv
O="🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/"
class Drift(Exception):pass
def load(rel):return open(os.path.join(ROOT,rel),encoding="utf8").read()
out={}
def once(text,old,new,label="edit"):
    if text.count(old)!=1:raise Drift("%s: anchor found %d times"%(label,text.count(old)))
    return text.replace(old,new)
MEDIA_DESC="Resource names that are a medium, not a resource: `prefix` followed by a directory path names an image, and a shared hold of it whose directory contains the lease directory a request uses is the store the lease lives in. It must be open before any lease can be taken, so it never takes part in ordering."
# schema
rel=O+"🧬️schema/🔣️.json";d=json.loads(load(rel))
if "media" not in d["definitions"]["Policy"]["properties"]:
    d["definitions"]["Policy"]["properties"]["media"]={"type":"array","description":MEDIA_DESC,"items":{"type":"object","additionalProperties":False,"required":["prefix"],"properties":{"prefix":{"type":"string","minLength":1,"maxLength":128}}}}
    d["definitions"]["Policy"]["required"].insert(d["definitions"]["Policy"]["required"].index("classes"),"media")
    out[rel]=json.dumps(d,ensure_ascii=False,indent=2)+"\n"
# policy
rel=O+"🔣️policy.json";d=json.loads(load(rel))
if "media" not in d:
    d["allowed"]=[row for row in d["allowed"] if not(row["from"]=="image" and row["to"]=="graph")]
    d2={}
    for k,v in d.items():
        if k=="classes":d2["media"]=[{"prefix":"native-fixed-image:"}]
        d2[k]=v
    out[rel]=json.dumps(d2,ensure_ascii=False,indent=2)+"\n"
# module
rel=O+"🟦️.ts";t=load(rel)
if "isLeaseMediumV1" not in t:
    t=once(t,"readonly fairness:Readonly<{queuedWithResourceLeases:boolean}>;","readonly fairness:Readonly<{queuedWithResourceLeases:boolean}>;","probe") if False else t
    t=once(t,"fairness:Readonly<{queuedWithResourceLeases:boolean}>;gate:","fairness:Readonly<{queuedWithResourceLeases:boolean}>;media:readonly Readonly<{prefix:string}>[];gate:","type")
    t=once(t,"const edgeAllowed=","""const slash=(path:string):string=>path.replaceAll("\\\\","/").replace(/\\/+$/u,"");
/** 💽️ Whether a hold is the medium of the request: a shared hold of an image (`media` prefix plus directory path) that contains the lease directory the request uses. The lease store is such an image; it is open before any lease exists in it, so it is the ground the leases stand on and never a rank to order against. */
export function isLeaseMediumV1(policy:LeaseOrderPolicyV1,held:HeldLeaseV1,request:Readonly<{directory:string}>):boolean{
 if(held.mode!=="shared")return false;
 return policy.media.some(medium=>held.resource.startsWith(medium.prefix)&&(slash(request.directory)+"/").startsWith(slash(held.resource.slice(medium.prefix.length))+"/"));
}

const edgeAllowed=""")
    t=once(t,"   const later=held.filter(row=>{const now=","   const later=held.filter(row=>!isLeaseMediumV1(policy,row,request)).filter(row=>{const now=")
    t=once(t,"const own=held.find(row=>row.directory===request.directory&&","const own=held.find(row=>!isLeaseMediumV1(policy,row,request)&&row.directory===request.directory&&")
    out[rel]=t
# fixture
rel=O+"🧪️tests/🧫️fixtures/🔣️.json";d=json.loads(load(rel));pol=d["policy"]
if "media" not in pol:
    pol["media"]=[{"prefix":"medium:"}]
    pol["classes"].append({"id":"image","rank":9,"scope":"generation","owner":"test","title":{"en":"Image","de":"Image"},"match":[{"kind":"prefix","value":"medium:"}]})
    h=lambda r,m="shared":{"directory":"/lease/data","resource":r,"mode":m,"atMs":500,"task":1}
    q=lambda r,m="exclusive",dr="/lease/data":{"directory":dr,"resource":r,"mode":m,"nonblocking":False,"bounded":False}
    d["order"]+= [
     {"id":"shared-medium-of-the-lease-directory-never-orders","held":[h("medium:/lease")],"request":q("outer:a"),"expect":None},
     {"id":"medium-also-matches-with-trailing-separator-and-backslashes","held":[h("medium:\\lease\\")],"request":q("outer:a"),"expect":None},
     {"id":"exclusive-medium-still-orders","held":[h("medium:/lease","exclusive")],"request":q("outer:a"),"expect":{"kind":"out-of-order","heldClass":"image"}},
     {"id":"shared-image-elsewhere-still-orders","held":[h("medium:/other")],"request":q("outer:a"),"expect":{"kind":"out-of-order","heldClass":"image"}},
     {"id":"prefix-of-a-sibling-directory-is-not-the-medium","held":[h("medium:/lease")],"request":q("outer:a","exclusive","/leased/data"),"expect":{"kind":"out-of-order","heldClass":"image"}}]
    d["selfDeadlock"]+= [{"id":"shared-medium-never-conflicts-with-its-own-resource","held":[{"directory":"/lease/data","resource":"medium:/lease","mode":"shared","atMs":1,"task":1}],"request":{"directory":"/lease/data","resource":"medium:/lease","mode":"exclusive","nonblocking":False,"bounded":False},"waitedMilliseconds":99999,"expect":None}]
    out[rel]=json.dumps(d,ensure_ascii=False,indent=1)+"\n"
# unit test: authored policy
rel=O+"🧪️tests/🟦️.ts";t=load(rel)
if "lease-store image" not in t:
    t=once(t,'for(const row of fixture.classification)','''test("the lease-store image never orders: held shared it is the medium, any other image or mode still orders",()=>{
 const live=readLeaseOrderPolicyV1(authored),at=(resource:string,mode:"shared"|"exclusive")=>[{directory:"/w/.cache/b/l/m/data",resource,mode,atMs:1,task:1}];
 const request={directory:"/w/.cache/b/l/m/data",resource:"repository-graph-facts:/w",mode:"exclusive" as const,nonblocking:false};
 expect(checkLeaseOrderV1(live,at("native-fixed-image:/w/.cache/b/l","shared"),request)).toBeUndefined();
 expect(checkLeaseOrderV1(live,at("native-fixed-image:/w/.cache/b/l","exclusive"),request)?.kind).toBe("out-of-order");
 expect(checkLeaseOrderV1(live,at("native-fixed-image:/w/.cache/b/d","shared"),request)?.kind).toBe("out-of-order");
 expect(live.allowed.some(edge=>edge.from==="image"&&edge.to==="graph")).toBe(false);
});

for(const row of fixture.classification)''')
    out[rel]=t
for rel,text in out.items():print("edit".ljust(8),rel)
if not out:print("already applied")
if WRITE:
    for rel,text in out.items():
        path=os.path.join(ROOT,rel);tmp=path+".conc-leases3.tmp";open(tmp,"w",encoding="utf8").write(text);os.replace(tmp,path)
    print("written")
