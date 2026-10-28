"""Third and fourth increment of the nxret-g change (anchors are the live text after stage.py and stage2.py): the cache-gc delegate never queues for the whole-image exclusive scope. A queued wait that runs out is a failed resource loan, which closes the admission of the whole `repo cache gc` process, so a busy tools image must be a plain, immediate `busy` verdict instead; the background launcher asks again every interval. The fourth part adds the bounded idle polling: the delegate hands the runner a try-lock probe of the whole-image scope and an abortable sleep. Usage: python3 stage3.py check|apply <root>."""
import hashlib,os,shutil,sys
HERE=os.path.dirname(os.path.abspath(__file__))
BACKUP=os.path.join(HERE,"backup3")
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
COMMAND=f"{L}/🏗️build/🧹️cache-gc/⌨️command/🟦️.ts"
PATCHES=[
 ('const{withRepositoryCacheStoreMaintenanceV1,withRepositoryCacheStoreQueuedMaintenanceV1}=await import(','const{withRepositoryCacheStoreMaintenanceV1}=await import('),
 (',request={version:1 as const,mode:"count-bound" as const,identities,limit},wait=(await pressed())?policy.maximumWaitUnderPressureMs:policy.maximumWaitMs;',',request={version:1 as const,mode:"count-bound" as const,identities,limit};'),
 ('const result=await(wait>0?withRepositoryCacheStoreQueuedMaintenanceV1(root,"tools",invocation,phaseControl,{version:1,mode:"queued",maximumWaitMilliseconds:wait},scope):withRepositoryCacheStoreMaintenanceV1(root,"tools",invocation,phaseControl,scope));','const result=await withRepositoryCacheStoreMaintenanceV1(root,"tools",invocation,phaseControl,scope);'),
 ('{probeNxToolingHeldV1}=await import(','{probeNxToolingHeldV1,probeNxToolingImageIdleV1}=await import('),
 ('freeBytes:free,...dryRun?{}:{retire}};','freeBytes:free,idle:()=>probeNxToolingImageIdleV1(leaseDirectory,join(root,budgetPolicy.partitions.tools.directory),signal),sleep:(milliseconds:number)=>new Promise<void>((done,fail)=>{const timer=setTimeout(done,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);fail(signal.reason);},{once:true});}),...dryRun?{}:{retire}};'),
 ('inside the whole-image exclusive scope, which never waits for running commands unless the image is pressed by the machine budget or its free space; a dry run only plans. */','inside the whole-image exclusive scope, which never waits for running commands (a busy image is a plain verdict, the next run asks again); a dry run only plans. */'),
]
def patch_text(text):
    for old,new in PATCHES:
        if new!=old and text.count(new)==1:continue
        if text.count(old)!=1:raise SystemExit(f"anchor missing or ambiguous ({text.count(old)}x): {old[:90]}")
        text=text.replace(old,new)
    return text
def main():
    mode,root=sys.argv[1],os.path.abspath(sys.argv[2])
    with open(os.path.join(root,COMMAND),encoding="utf8") as handle:text=patch_text(handle.read())
    if mode=="check":print("ok");return
    os.makedirs(BACKUP,exist_ok=True)
    name=hashlib.sha1(COMMAND.encode()).hexdigest()[:16]
    if not os.path.exists(os.path.join(BACKUP,name+".bak")):shutil.copy2(os.path.join(root,COMMAND),os.path.join(BACKUP,name+".bak"))
    temporary=os.path.join(root,COMMAND)+".nxret-apply"
    with open(temporary,"w",encoding="utf8") as handle:handle.write(text)
    os.replace(temporary,os.path.join(root,COMMAND));print("applied",COMMAND[-50:])
main()
