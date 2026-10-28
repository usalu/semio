"""Fifth increment of the nxret-g change (anchors are the live text after stages 1-4): the single-flight lease of the delegate, the background launcher of the collector and its wiring into the Nx backend's storage preparation. Usage: python3 stage5.py check|apply <root>; files are replaced by atomic rename, the first original of each is kept in `backup5/`."""
import hashlib,json,os,re,shutil,sys

HERE=os.path.dirname(os.path.abspath(__file__))
BACKUP=os.path.join(HERE,"backup5")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
COMMAND=f"{L}/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts"
LAUNCH=f"{L}/🏗️build/🧹️cache-gc/🚀️launch/🟦️.ts"
BACKEND=f"{L}/🏗️build/🔧️tools/🧶️nx/🔌️backend/🟦️.ts"
TEXTS=f"{L}/⚡️caching/💾️storage/🧾️ledger/📖️texts/🔣️.json"

COMMAND_PATCHES=[
 (' let pressure:boolean|undefined;\n',
  ' let pressure:boolean|undefined;\n const leases=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts");\n let single:Awaited<ReturnType<typeof leases.acquireResourceLease>>;\n try{single=await leases.acquireResourceLease({directory:leaseDirectory,resource:"repository-nx-tooling-collector:"+root,mode:"exclusive",signal,control:phaseControl,timeoutMs:0});}catch(error){if(/timed out/iu.test(String((error as Error).message)))return cacheTextV1(language,"nxTooling.running");throw error;}\n'),
 (' return formatNxToolingStoreBoundV1(language,await runNxToolingStoreBoundV1(dryRun,ports,policy)).join("\\n");\n}',
  ' try{return formatNxToolingStoreBoundV1(language,await runNxToolingStoreBoundV1(dryRun,ports,policy)).join("\\n");}finally{single.release();}\n}'),
]

LAUNCH_PATCHES=[
 ('import {cacheLedgerV1,cacheTextV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";',
  'import {cacheLedgerV1,cacheTextV1,type CacheLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";\nimport {nxToolingStoreBoundPolicyV1} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🟦️.ts";'),
 ('failed:"gc.products.launch.failed",intervalMs:PRODUCTS_COLLECTOR_INTERVAL_MS_V1});\n',
  'failed:"gc.products.launch.failed",intervalMs:PRODUCTS_COLLECTOR_INTERVAL_MS_V1});\nconst nxToolingTarget:CollectorTargetV1=Object.freeze({area:"nx-tooling",arguments:["--area=nx-tooling,nx-tmp"],name:"nx-tooling-collector",failed:"nxTooling.launch.failed",intervalMs:nxToolingStoreBoundPolicyV1().intervalMs});\n'),
 ('/** 🕸️ What any graph loader does about a pressed image',
  '/** 🧶️ Starts the Nx toolkit store collector as a detached, low-priority `repo cache gc --area=nx-tooling,nx-tmp` (output in `nx-tooling-collector.log` of the build area) below the soft free-space threshold the ledger declares for the tools image; single-flight (one launch per interval of the bound policy, and the collector itself takes a lease without waiting), never waits for it and never fails the writer. Returns the decision taken. */\nexport async function launchNxToolingCollectorV1(workspace:string,availableBytes:number,overrides:Partial<GraphCollectorPortsV1>={},ledger:CacheLedgerV1=cacheLedgerV1()):Promise<GraphCollectorDecisionV1>{return launchCollectorV1(workspace,availableBytes,nxToolingTarget,overrides,ledger);}\n\n/** 🧰️ What any Nx run does about a pressed tools image: it reads the free space of the mounted image (`undefined` means not mounted, which counts as free) and launches the single-flight background collector below the soft threshold. Never waits, never throws. */\nexport async function launchNxToolingCollectorIfPressedV1(workspace:string,overrides:Partial<GraphCollectorPortsV1>={},measure:(workspace:string)=>number|undefined=mountedToolsFreeBytesV1):Promise<GraphCollectorDecisionV1|"unmounted">{\n try{\n  const available=measure(workspace);\n  return available===undefined?"unmounted":await launchNxToolingCollectorV1(workspace,available,overrides);\n }catch{return "unmounted";}\n}\n\n/** 🕸️ What any graph loader does about a pressed image'),
 ('function mountedPartitionFreeBytesV1(workspace:string,partition:"graph"|"products"):number|undefined{','function mountedPartitionFreeBytesV1(workspace:string,partition:"graph"|"products"|"tools"):number|undefined{'),
 ('export const mountedProductsFreeBytesV1=(workspace:string):number|undefined=>mountedPartitionFreeBytesV1(workspace,"products");',
  'export const mountedProductsFreeBytesV1=(workspace:string):number|undefined=>mountedPartitionFreeBytesV1(workspace,"products");\n/** 🧰️ The free bytes of the mounted tools image of a workspace, undefined while the image is not mounted. */\nexport const mountedToolsFreeBytesV1=(workspace:string):number|undefined=>mountedPartitionFreeBytesV1(workspace,"tools");'),
]

BACKEND_PATCHES=[
 ('await prepareNxGraphCacheStorageV1(root,invocation,phaseControl,snapshotBytes);\n',
  'await prepareNxGraphCacheStorageV1(root,invocation,phaseControl,snapshotBytes);\n  await (await import("../../../🧹️cache-gc/🚀️launch/🟦️.ts")).launchNxToolingCollectorIfPressedV1(root);\n'),
]

def texts_patch(text):
    if '"nxTooling.running"' in text:return text
    anchor='    "nxTooling.launch.failed":'
    if text.count(anchor)!=1:raise SystemExit("launch.failed anchor")
    entry='    "nxTooling.running": { "en": "Another Nx toolkit store collector is running", "de": "Ein anderer Sammler des Nx-Werkzeugspeichers läuft bereits" },\n'
    return text.replace(anchor,entry+anchor)

def patch_text(patches):
    def apply(text):
        for old,new in patches:
            if new!=old and text.count(new)==1:continue
            if text.count(old)!=1:raise SystemExit(f"anchor missing or ambiguous ({text.count(old)}x): {old[:90]}")
            text=text.replace(old,new)
        return text
    return apply

TARGETS=[(TEXTS,texts_patch),(LAUNCH,patch_text(LAUNCH_PATCHES)),(COMMAND,patch_text(COMMAND_PATCHES)),(BACKEND,patch_text(BACKEND_PATCHES))]

def remember(root,rel):
    os.makedirs(BACKUP,exist_ok=True)
    name=hashlib.sha1(rel.encode()).hexdigest()[:16]
    manifest=os.path.join(BACKUP,"manifest.tsv")
    known=set(line.split("\t")[0] for line in open(manifest,encoding="utf8")) if os.path.exists(manifest) else set()
    if name in known:return
    shutil.copy2(os.path.join(root,rel),os.path.join(BACKUP,name+".bak"))
    with open(manifest,"a",encoding="utf8") as handle:handle.write(f"{name}\t{rel}\n")

def main():
    mode,root=sys.argv[1],os.path.abspath(sys.argv[2])
    produced=[]
    for rel,transform in TARGETS:
        with open(os.path.join(root,rel),encoding="utf8") as handle:produced.append((rel,transform(handle.read())))
    for rel,text in produced:
        if rel.endswith(".json"):json.loads(text)
    if mode=="check":
        for rel,_ in produced:print("ok",rel[-60:])
        return
    for rel,text in produced:
        remember(root,rel)
        temporary=os.path.join(root,rel)+".nxret-apply"
        with open(temporary,"w",encoding="utf8") as handle:handle.write(text)
        os.replace(temporary,os.path.join(root,rel));print("applied",rel[-60:])

main()
