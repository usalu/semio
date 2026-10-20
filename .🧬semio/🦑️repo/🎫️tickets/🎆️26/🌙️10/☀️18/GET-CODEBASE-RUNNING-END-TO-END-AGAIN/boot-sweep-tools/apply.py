#!/usr/bin/env python3
"""Anchored apply of the staged boot-sweep set (reboot/crash recovery of the native storage partitions). usage: apply.py [--root DIR] [--write] | apply.py [--root DIR] --undo. Default is a dry run. Every anchor must occur exactly the declared number of times; new files must be absent or identical; the whole set is verified before the first write and rolled back on any failure; refuses while a dependency sync or `show projects` runs (live root only)."""
import hashlib,json,os,shutil,subprocess,sys
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import manifest as m
HERE=os.path.dirname(os.path.abspath(__file__))
args=sys.argv[1:]
write="--write" in args
undo="--undo" in args
LIVE="/Users/ueli/Documents/semio"
root=args[args.index("--root")+1] if "--root" in args else LIVE
live=os.path.realpath(root)==os.path.realpath(LIVE)
BK=HERE+"/../🗑️generated/boot-sweep/backup" if live else root.rstrip("/")+".backup"
S=m.S
JS=m.JS
HEAL='import{healNativeProcessBootV1,nativePartitionPublicationPendingV1,settleNativePartitionPublicationV1}from"%s🌐️platform/🥾️boot/🩹️recovery/🩺️heal/🟦️.ts";'
BIND_S='import{bindNativeBackingIdentityV1,bindNativeIdentitiesV1}from"./🌐️platform/🥾️boot/🩹️recovery/📋️plan/🟦️.ts";'
if live and write:
    ps=subprocess.run(["ps","-axo","command"],capture_output=True,text=True).stdout
    if any(("dependencies sync" in l or "show projects" in l) and "apply.py" not in l for l in ps.split("\n")):sys.exit("refused: dependency sync or show projects running")

if undo:
    index=json.load(open(BK+"/index.json",encoding="utf8"))
    for path,name in index.items():
        if name is None:
            if os.path.exists(path):
                os.remove(path);print("removed",path[len(root)+1:][-80:])
                folder=os.path.dirname(path)
                while folder!=root.rstrip("/"):
                    try:os.rmdir(folder)
                    except OSError:break
                    folder=os.path.dirname(folder)
        else:shutil.copyfile(BK+"/"+name,path);print("restored",path[len(root)+1:][-80:])
    sys.exit(0)

edits=[]
def target(rel,pairs):
    path=root+"/"+rel
    text=open(path,encoding="utf8").read()
    if all(new in text for _,new,_ in pairs):
        print("already applied:",rel[-60:]);return
    for old,new,count in pairs:
        have=text.count(old)
        if have!=count:sys.exit(f"anchor count {have} != {count}: {old[:100]!r} in {rel[-60:]}")
    out=text
    for old,new,_ in pairs:out=out.replace(old,new)
    edits.append((path,text,out))

def newfile(staged,rel):
    path=root+"/"+rel
    data=open(m.ST+"/"+staged,encoding="utf8").read()
    if os.path.exists(path):
        if open(path,encoding="utf8").read()!=data:sys.exit("new file exists with other content: "+rel[-70:])
        print("already present:",rel[-60:]);return
    edits.append((path,None,data))

for staged,rel in m.NEW.items():newfile(staged,rel)

target(S+"/🟦️.ts",[
 ('import { readNativeCreatorClaimRecordsV1 } from "./🌱️birth/🧾️claim-records/🟦️.ts";','import { readNativeCreatorClaimRecordsV1 } from "./🌱️birth/🧾️claim-records/🟦️.ts";\n'+BIND_S,1),
 ('JSON.stringify(namespaceIdentityV1(root,check))!==JSON.stringify(receipt.parent)||JSON.stringify(namespaceIdentityV1(receipt.directory,check,true))!==JSON.stringify(receipt.entry))throw Error("Exact recorded Native namespace identity required");}','bindNativeIdentitiesV1([receipt.parent,receipt.entry],[namespaceIdentityV1(root,check),namespaceIdentityV1(receipt.directory,check,true)])==="mismatch")throw Error("Exact recorded Native namespace identity required");}',1),
 ('JSON.stringify(namespaceIdentityV1(root,check))!==JSON.stringify(receipt.parent)||JSON.stringify(namespaceIdentityV1(receipt.directory,check,true))!==JSON.stringify(receipt.entry))throw Error("Exact owned empty Native namespace required");check();rmdirSync(receipt.directory);namespaceAbsentV1(receipt.directory,check);if(JSON.stringify(namespaceIdentityV1(root,check))!==JSON.stringify(receipt.parent))throw Error("Original Native namespace parent changed after removal");','bindNativeIdentitiesV1([receipt.parent,receipt.entry],[namespaceIdentityV1(root,check),namespaceIdentityV1(receipt.directory,check,true)])==="mismatch")throw Error("Exact owned empty Native namespace required");check();rmdirSync(receipt.directory);namespaceAbsentV1(receipt.directory,check);if(bindNativeIdentitiesV1([receipt.parent],[namespaceIdentityV1(root,check)])==="mismatch")throw Error("Original Native namespace parent changed after removal");',1),
 ('request.parentIdentity!==parent.identity||request.maximumPhysicalBytes>=allocation.bytes','bindNativeBackingIdentityV1(request.parentIdentity,parent.identity)==="mismatch"||request.maximumPhysicalBytes>=allocation.bytes',1),
 ('authored.maximumPhysicalBytes!==request.maximumPhysicalBytes||authored.dev!==file.dev||authored.ino!==file.ino','authored.maximumPhysicalBytes!==request.maximumPhysicalBytes||authored.ino!==file.ino',1),
 ('!same(last.parent,{dev:String(parentDirectory.dev),ino:String(parentDirectory.ino),mode:Number(parentDirectory.mode)})||!same(last.directory,{dev:String(folder.dev),ino:String(folder.ino),mode:Number(folder.mode)})||!same(last.root,underlying)','bindNativeIdentitiesV1([last.parent,last.directory],[{dev:String(parentDirectory.dev),ino:String(parentDirectory.ino),mode:Number(parentDirectory.mode)},{dev:String(folder.dev),ino:String(folder.ino),mode:Number(folder.mode)}])==="mismatch"||bindNativeIdentitiesV1([last.root],[underlying])==="mismatch"',1),
 ('scope.childRequest.parentIdentity!==source.backing.identity||scope.childRequest.allocation!==source.allocation.id','bindNativeBackingIdentityV1(scope.childRequest.parentIdentity,source.backing.identity)==="mismatch"||scope.childRequest.allocation!==source.allocation.id',1),
])

CLAIM_RECORDS=S+"/🌱️birth/🧾️claim-records/"+JS
target(CLAIM_RECORDS,[
 ('import exportedSchema from "../🧾️claim/🧬️schema/🔣️.json" with{type:"json"};','import exportedSchema from "../🧾️claim/🧬️schema/🔣️.json" with{type:"json"};\nimport{bindNativeIdentitiesV1}from"../../🌐️platform/🥾️boot/🩹️recovery/📋️plan/🟦️.ts";',1),
 ('export const same=(left:unknown,right:unknown)=>JSON.stringify(left)===JSON.stringify(right);','export const same=(left:unknown,right:unknown)=>JSON.stringify(left)===JSON.stringify(right);\n\n/** 🪪️ Accepts a recorded creator identity when its inode and mode are exact; device numbers are never persisted truth. */\nexport const stable=(recorded:Identity,current:Identity)=>bindNativeIdentitiesV1([recorded],[current])!=="mismatch";',1),
])
target(S+"/🌱️birth/🧾️claim/"+JS,[
 ('import { storageSchema, schema, protocol, same, readRequest, readNativeCreatorClaimRecordsV1 } from "../🧾️claim-records/🟦️.ts";','import { storageSchema, schema, protocol, same, stable, readRequest, readNativeCreatorClaimRecordsV1 } from "../🧾️claim-records/🟦️.ts";',1),
 ('!same(value.parent,originalParent)','!stable(value.parent,originalParent)',1),
 ('if(!same(await identity(parent,scope.check),originalParent))throw Error("Native creator parent changed")','if(!stable(await identity(parent,scope.check),originalParent))throw Error("Native creator parent changed")',1),
 ('if(!same(await identity(directory,scope.check),scope.record.directory))throw Error("Original Native creator directory changed")','if(!stable(await identity(directory,scope.check),scope.record.directory))throw Error("Original Native creator directory changed")',1),
 ('if(!same(await identity(root,scope.check),scope.record.root))throw Error("Original Native creator root changed")','if(!stable(await identity(root,scope.check),scope.record.root))throw Error("Original Native creator root changed")',1),
 ('if(!same(await identity(parent,authorize),scope.record.parent))throw Error("Original retired claim parent changed")','if(!stable(await identity(parent,authorize),scope.record.parent))throw Error("Original retired claim parent changed")',1),
 ('!same(await identity(parent,check),record.parent)||!same(await identity(directory,check),record.directory)||!same(underlying,record.root)','!stable(await identity(parent,check),record.parent)||!stable(await identity(directory,check),record.directory)||!stable(underlying,record.root)',1),
 ('if(!same(await identity(parent,removed),record.parent))throw Error("Recovered creator parent changed")','if(!stable(await identity(parent,removed),record.parent))throw Error("Recovered creator parent changed")',1),
])

target(S+"/🧱️fixed/"+JS,[
 ('import { readNativeImageRetirementFileV1 } from "../📄️retirement-file/🟦️.ts";','import { readNativeImageRetirementFileV1 } from "../📄️retirement-file/🟦️.ts";\n'+HEAL%"../",1),
 ('authored.dev!==file.dev||authored.ino!==file.ino||authored.size!==file.size||file.size>options.maximumPhysicalBytes)throw Error("Original fixed platform retirement identity changed")','lstatSync(directory).dev!==file.dev||authored.ino!==file.ino||authored.size!==file.size||file.size>options.maximumPhysicalBytes)throw Error("Original fixed platform retirement identity changed")',2),
 ('file.dev!==authored.dev||file.ino!==authored.ino||file.size!==authored.size||file.size>options.maximumPhysicalBytes||file.blocks*512>options.maximumPhysicalBytes||!mount.isDirectory()||mount.isSymbolicLink())throw Error("Exact persistent fixed backing required")','file.dev!==lstatSync(options.directory).dev||file.ino!==authored.ino||file.size!==authored.size||file.size>options.maximumPhysicalBytes||file.blocks*512>options.maximumPhysicalBytes||!mount.isDirectory()||mount.isSymbolicLink())throw Error("Exact persistent fixed backing required")',1),
 ('if(!existsSync(options.directory))return "absent";requireNativeImagePublicationSettledV1(options.directory);if(!existsSync(join(options.directory,"image.json")))return "unpublished";','if(!existsSync(options.directory))return "absent";await healNativeProcessBootV1({leaseDirectory:options.leaseDirectory,control:options.control});if(nativePartitionPublicationPendingV1(options.directory)){if(!exclusive)return "inactive";await settleNativePartitionPublicationV1({directory:options.directory,leaseDirectory:options.leaseDirectory,control:options.control});check();}requireNativeImagePublicationSettledV1(options.directory);if(!existsSync(join(options.directory,"image.json")))return "unpublished";',1),
])

target(S+"/🍎️image/"+JS,[
 ('physical.dev!==authored.dev||physical.ino!==authored.ino','lstatSync(directory).dev!==physical.dev||physical.ino!==authored.ino',2),
])

target(S+"/🧹️cleanup/🎛️command/"+JS,[
 ('import{readNativeBootSessionV1,nativeBootSessionV1,readNativeImageAncestryV1,requireNativeImageAncestryBindingV1}from"../../🌐️platform/🥾️boot/🟦️.ts";','import{readNativeBootSessionV1,nativeBootSessionV1,readNativeImageAncestryV1,requireNativeImageAncestryBindingV1}from"../../🌐️platform/🥾️boot/🟦️.ts";\n'+HEAL%"../../",1),
 ('await readNativeBootSessionV1(control,x.leaseDirectory);for(const[path,identity]of receiver.parents)exact(path,identity,true);if(!existsSync(directory))return"absent";requireNativeImagePublicationSettledV1(directory);','await readNativeBootSessionV1(control,x.leaseDirectory);await healNativeProcessBootV1({leaseDirectory:x.leaseDirectory,control});for(const[path,identity]of receiver.parents)exact(path,identity,true);if(!existsSync(directory))return"absent";if(nativePartitionPublicationPendingV1(directory)){if(receiver.stage==="reserved"&&!retirement)return"inactive";if(receiver.stage==="writer")await settleNativePartitionPublicationV1({directory,leaseDirectory:x.leaseDirectory,control});}requireNativeImagePublicationSettledV1(directory);',1),
])

for path,old,new in edits:print(("write " if write else "would write ")+path[len(root)+1:][-80:])
print(("applying" if write else "dry-run ok:"),len(edits),"targets")
if not write or not edits:sys.exit(0)
os.makedirs(BK,exist_ok=True)
index={}
for path,old,new in edits:
    if old is not None:
        name=hashlib.sha256(path.encode()).hexdigest()[:16]+".orig";shutil.copyfile(path,BK+"/"+name);index[path]=name
    else:index[path]=None
json.dump(index,open(BK+"/index.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
staged=[]
try:
    for path,old,new in edits:
        os.makedirs(os.path.dirname(path),exist_ok=True)
        open(path+".boot-sweep","w",encoding="utf8").write(new);staged.append(path)
    replaced=[]
    try:
        for path,old,new in edits:os.replace(path+".boot-sweep",path);replaced.append((path,old))
    except BaseException:
        for path,old in replaced:
            if old is None:os.remove(path)
            else:open(path,"w",encoding="utf8").write(old)
        raise
finally:
    for path in staged:
        if os.path.exists(path+".boot-sweep"):os.remove(path+".boot-sweep")
print("applied",len(edits),"targets; backups",BK)
