import os,sys,shutil,json
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/"
ST=P+"📦️artifacts/🏗️native-build/💾️storage/"
HERE=os.path.dirname(os.path.abspath(__file__))
APPLY="--apply" in sys.argv
CAP=lambda var,op:'requireNativeStorageRootCapabilityV1(%s,"%s");'%(var,op)
IMG='requireNativeImageEnvelopeV1(options);requireArtifactDescriptorClosureV1();const admitted=readNativeImageRetirementRequestV1(request)'
FIX='const platform=fixedPlatformV1();requireNativeImageEnvelopeV1(options);const admitted=readNativeImageRetirementRequestV1(request)'
edits=[
(ST+"🍎️image/🟦️.ts",[(IMG,CAP("options.directory","retire-image")+IMG,2)]),
(ST+"🧱️fixed/🟦️.ts",[(FIX,CAP("options.directory","retire-image")+FIX,2)]),
(ST+"🍎️image/🧩️shard/🟦️.ts",[
 ("):Promise<boolean>{\n return given.lock(async()=>{\n  const record=nativeImageShardFoldV1(","):Promise<boolean>{\n "+CAP("given.directory","retire-shard")+"return given.lock(async()=>{\n  const record=nativeImageShardFoldV1(",1),
 ("clean:boolean}>>{\n return given.lock(async()=>{\n  const retired:string[]=[]","clean:boolean}>>{\n "+CAP("given.directory","retire-shards")+"return given.lock(async()=>{\n  const retired:string[]=[]",1)]),
(ST+"🧹️whole/🟦️.ts",[("control:ResourceLeaseControl):Promise<void>{const scope=scopeOf(token);if(scope.options.directory!==directory)","control:ResourceLeaseControl):Promise<void>{"+CAP("directory","retire-whole-partition")+"const scope=scopeOf(token);if(scope.options.directory!==directory)",1)]),
(ST+"🧹️cleanup/🎛️command/🐚️shell/🟦️.ts",[
 ("export function removeEmptyNativeReceiverShellV1(directory:string,root:string):void{","export function removeEmptyNativeReceiverShellV1(directory:string,root:string):void{"+CAP("directory","remove-receiver-shell"),1),
 ("stamp:number=Date.now()):string|undefined{","stamp:number=Date.now()):string|undefined{"+CAP("root","quarantine-root"),1)]),
(ST+"🍎️image/🔁️detach/🟦️.ts",[("check:()=>void;ready?:()=>boolean}>):Promise<void>{\n for(let attempt=0;;attempt++){","check:()=>void;ready?:()=>boolean}>):Promise<void>{\n "+CAP("given.mountPoint","detach-image")+"for(let attempt=0;;attempt++){",1)]),
(ST+"🍎️image/🔗️attach/🟦️.ts",[("image:string):Promise<boolean>{\n let row=await rowOf(run,image),own=false;","image:string):Promise<boolean>{\n "+CAP("image","remove-volumes")+"let row=await rowOf(run,image),own=false;",1)]),
(ST+"🧹️cleanup/🎛️command/🟦️.ts",[("export function removeSettledNativeImageV1(owner:NativeImageCleanupOwnerV1):void{const scope=scopeOf(owner);","export function removeSettledNativeImageV1(owner:NativeImageCleanupOwnerV1):void{const scope=scopeOf(owner);"+CAP("scope.directory","remove-image"),1)]),
(ST+"🌐️platform/🟦️.ts",[("selection?:NativeStorageSelectionV1):Promise<boolean>{requireNativeImageEnvelopeV1(options);const admitted=readNativeImageRetirementRequestV1(request??","selection?:NativeStorageSelectionV1):Promise<boolean>{"+CAP("options.directory","retire-storage")+"requireNativeImageEnvelopeV1(options);const admitted=readNativeImageRetirementRequestV1(request??",1)]),
(P+"🧪️testing/🏖️sandbox/🖼️images/🟦️.ts",[("  try { real = realpathSync(real); } catch {}\n  if (!mayHoldDiskImageV1(real)) return [];","  try { real = realpathSync(real); } catch {}\n  requireNativeStorageRootCapabilityV1(real, \"detach-images\");\n  if (!mayHoldDiskImageV1(real)) return [];",1)]),
("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/🪦️retirement/🟦️.ts",[("requireIssuedResourceControlInvocationV1(control,original);const workspace=requirePrivateRepositoryWorkspaceV1(root);","requireIssuedResourceControlInvocationV1(control,original);const workspace=requirePrivateRepositoryWorkspaceV1(root);"+CAP("workspace","retire-private-repository"),1)]),
]
def rel(f):
    return os.path.relpath(ST+"🛂️root-capability/🟦️.ts",os.path.dirname(f))
def importline(f):
    if f.startswith(P+"🧪️testing"):
        return 'import { requireNativeStorageRootCapabilityV1 } from "%s";\n'%rel(f)
    return 'import{requireNativeStorageRootCapabilityV1}from"%s";\n'%rel(f)
ok=True;work=[]
for f,subs in edits:
    s=open(f,encoding="utf8").read()
    if "requireNativeStorageRootCapabilityV1" in s: print("already",f);continue
    for old,new,n in subs:
        c=s.count(old)
        if c!=n: print("ANCHOR",c,"!=",n,f,old[:70].replace("\n","|"));ok=False
        s=s.replace(old,new)
    s=importline(f)+s;work.append((f,s))
if os.path.exists(ST+"🛂️root-capability/🟦️.ts"): print("module already present")
if not ok: sys.exit(1)
print("anchors ok",len(work),"files")
if not APPLY: sys.exit(0)
for f,s in work:
    t=f+".h64tmp";open(t,"w",encoding="utf8").write(s);os.replace(t,f)
shutil.copytree(HERE+"/stage/🛂️root-capability",ST+"🛂️root-capability",dirs_exist_ok=True)
os.makedirs(ST+"🛂️root-capability/🧪️tests/🚪️entries",exist_ok=True)
shutil.copy(HERE+"/stage/entries-test/🟦️.ts",ST+"🛂️root-capability/🧪️tests/🚪️entries/🟦️.ts")
cp=P+"📖️texts/🔣️.json";s=open(cp,encoding="utf8").read()
if "storage.root-capability-refused" not in s:
    add=open(HERE+"/stage/texts-add.json",encoding="utf8").read()
    tail="\n  }\n}";i=s.rstrip().rfind(tail);n=s[:i]+add.replace("\n    }","\n    }",1)+tail+"\n"
    json.loads(n);open(cp+".h64tmp","w",encoding="utf8").write(n);os.replace(cp+".h64tmp",cp)
print("applied")
