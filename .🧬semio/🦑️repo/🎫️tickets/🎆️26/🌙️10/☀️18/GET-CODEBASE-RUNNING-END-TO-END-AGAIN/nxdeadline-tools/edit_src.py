"""✂️ [nxdeadline] anchored source edits of batch `release`: edit_src.py <root>. Exit 1 when an anchor is lost (count != 1); prints one line per file."""
import pathlib
import sys

ROOT = pathlib.Path(sys.argv[1])
E = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement"
EDITS = {
    E + "/🔌️nx/🟦️.ts": [
        (
            "/** 🔓️ A bound namespace plus the one release of its graph lease: the caller releases it when its Nx run ended, inside the tooling store scope that was open when it bound, so the funded graph child never outlives that scope's settlement. */",
            "/** 🔓️ A bound namespace plus the one release of its graph lease: the caller may release it as soon as its Nx run ended, and the storage owner releases whatever is still held when its scope ends; the funded graph child is retained by the nx partition owner, never by the caller's nested scope, so binding inside a tooling or graph store scope cannot make that scope wait for the owner's release. A bind that is refused after it acquired the namespace releases it before the refusal returns. */",
        ),
        (
            "held.push(lease);if(watcherIdentity)await bindNxGraphNativeWatcherV1(lease,watcherIdentity);",
            "held.push(lease);try{if(watcherIdentity)await bindNxGraphNativeWatcherV1(lease,watcherIdentity);",
        ),
        (
            "await releaseNxGraphNamespacesV1([lease]);}});},actualPhase);",
            "await releaseNxGraphNamespacesV1([lease]);}});}catch(error){const index=held.indexOf(lease);if(index>=0)held.splice(index,1);await releaseNxGraphNamespacesV1([lease],{error});throw error;}},actualPhase);",
        ),
    ],
    E + "/🔌️nx/📦️graph/🟦️.ts": [
        (
            "import {withRepositoryCacheStoreV1,withRepositoryCacheStoreChildStorageV1,",
            "import {withRepositoryCacheStoreV1,withRepositoryCacheStoreOwnerFrameV1,withRepositoryCacheStoreChildStorageV1,",
        ),
        (
            "/** 🧷️ Retains a privately owned scoped child until its namespace releases and drains. */",
            "/** ⏏️ Settles like `work`, or with `forced()` as soon as `signal` aborts first; the abort listener never outlives the wait. */\n"
            "function untilAbortedV1<T>(work:Promise<T>,signal:AbortSignal,forced:()=>T):Promise<T>{return new Promise<T>((accept,refuse)=>{const stop=()=>{signal.removeEventListener(\"abort\",stop);accept(forced());};signal.addEventListener(\"abort\",stop,{once:true});if(signal.aborted)stop();work.then(value=>{signal.removeEventListener(\"abort\",stop);accept(value);},error=>{signal.removeEventListener(\"abort\",stop);refuse(error);});});}\n"
            "/** 🧷️ Retains a privately owned scoped child in its nx partition owner's consumer frame (not in the caller's possibly shorter nested scope) until its namespace releases and drains, or until its lifetime control aborts on a deadline or cancellation, which closes the image without waiting for a release. */",
        ),
        (
            "work=openNxGraphNativeChildV1(store,options,async(image,control)=>{admitted=true;accept(Object.freeze({image,control}));await released;});",
            "const parent=poolBases.get(store);if(!parent)throw Error(\"Actual nx partition owner required\");work=withRepositoryCacheStoreOwnerFrameV1(parent,()=>openNxGraphNativeChildV1(store,options,async(image,control)=>{admitted=true;accept(Object.freeze({image,control}));await untilAbortedV1(released,control.signal,()=>undefined);}));",
        ),
        (
            "releasing=(async()=>{const failures=[...await readers];",
            "releasing=(async()=>{const failures=[...await untilAbortedV1(readers,entry.control.signal,()=>[entry.control.signal.reason])];",
        ),
    ],
    E + "/📦️budget/📥️opening/🏪️store/🟦️.ts": [
        (
            "/** 🏦️ Retains an issued partition reader while its parent-funded child persists its charge in the partition journal. */",
            "/** 🏠️ Runs `body` in the nearest consumer frame of the store's own issued owner, so work that outlives the caller's nested scope of another owner (a retained child image) is drained and funded by its owner instead of by a scope that settles before the owner could release it. */\n"
            "export function withRepositoryCacheStoreOwnerFrameV1<T>(store:RepositoryCacheStoreV1,body:()=>Promise<T>):Promise<T>{const owner=issuedStores.get(store);if(!owner||owner.store!==store||typeof body!==\"function\")throw Error(\"Exact issued cache owner required\");requireOwner(owner);let frame=consumerFrames.getStore();while(frame&&frame.owner!==owner)frame=frame.parent;if(!frame)throw Error(\"Original cache owner frame required\");return consumerFrames.run(frame,body);}\n"
            "/** 🏦️ Retains an issued partition reader while its parent-funded child persists its charge in the partition journal. */",
        ),
    ],
}

failed = False
for rel, edits in EDITS.items():
    path = ROOT / rel
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            print(f"ANCHOR x{text.count(old)} {rel}: {old[:80]}")
            failed = True
            continue
        text = text.replace(old, new)
    if not failed:
        mode = path.stat().st_mode
        path.write_text(text, encoding="utf-8")
        path.chmod(mode & 0o7777)
        print("edited", rel)
sys.exit(1 if failed else 0)
