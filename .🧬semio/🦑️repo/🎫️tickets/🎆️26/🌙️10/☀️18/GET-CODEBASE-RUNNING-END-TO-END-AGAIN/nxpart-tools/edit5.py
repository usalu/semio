B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/"
def load(p): return open(p,encoding='utf8').read()
def save(p,s): open(p,'w',encoding='utf8').write(s)
def rep(s,old,new,count=1):
    c=s.count(old)
    if c!=count: raise SystemExit(f"expected {count} got {c}: {old[:100]}")
    return s.replace(old,new)
P9="../../../../../../../../../"
p=B+"🟦️.ts";s=load(p)
s=rep(s,'import {openNativePlatformStorageV1,requireNativePlatformImageCloseV1,withNativePlatformChildStorageV1,withNativePlatformMaintenanceChildStorageV1,withNativePlatformDisposableChildStorageV1,requireNativeStoragePathV1,type NativeImageStorageV1,type NativeDisposableImageV1} from','import {openNativePlatformStorageV1,requireNativePlatformImageCloseV1,withNativePlatformChildStorageV1,withNativePlatformFundedChildStorageV1,retireNativePlatformFundedChildStorageV1,withNativePlatformMaintenanceChildStorageV1,withNativePlatformDisposableChildStorageV1,requireNativeStoragePathV1,type NativeImageStorageV1,type NativeDisposableImageV1} from')
s=rep(s,'import parentSchema from"./📂️parents/🧬️schema/🔣️.json"with{type:"json"};','import parentSchema from"./📂️parents/🧬️schema/🔣️.json"with{type:"json"};\nimport type{NativeStorageChildFundingV1}from"'+P9+'🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🟦️.ts";\nimport type{NativeStorageReservationV1}from"'+P9+'🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";\nimport type{ScriptInvocation as FundedScriptInvocation}from"'+P9+'🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts";')
anchor='/** 🧺️ Retains the exact private partition through a pre-funded disposable child and terminal settlement. */'
new='''/** 🏦️ Enrolls a parent-funded child, whose charge persists in the retained partition journal, under the privately retained actual partition image. */
export function withRepositoryCachePartitionFundedChildStorageV1<T>(partition:NativeImageStorageV1,input:Parameters<typeof openNativePlatformStorageV1>[0],funding:NativeStorageChildFundingV1,body:(child:NativeImageStorageV1,reservation:NativeStorageReservationV1,actualControl:ResourceLeaseControl)=>Promise<T>):Promise<T>{const options=Object.freeze({...input});return withRepositoryPartitionOperationV1(partition,options.control,(image,control)=>withNativePlatformFundedChildStorageV1(image,Object.freeze({...options,control}),funding,body));}
/** ♻️ Retires one persisted parent-funded child and discharges its charge under the privately retained actual partition image. */
export function retireRepositoryCachePartitionFundedChildStorageV1(partition:NativeImageStorageV1,reservation:NativeStorageReservationV1,reservationControl:ResourceLeaseControl,control:ResourceLeaseControl,invocation:FundedScriptInvocation):Promise<void>{return withRepositoryPartitionOperationV1(partition,control,(image,actual)=>retireNativePlatformFundedChildStorageV1(image,reservation,reservationControl,actual,invocation));}
'''
s=rep(s,anchor,new+anchor)
save(p,s)
p=B+"🏪️store/🟦️.ts";s=load(p)
s=rep(s,'import {repositoryCachePartitionRootV1 as imageRoot,openRepositoryCachePartitionV1,withRepositoryCachePartitionDisposableChildStorageV1,withRepositoryCachePartitionChildStorageV1,withRepositoryCachePartitionMaintenanceChildStorageV1} from "../🟦️.ts";','import {repositoryCachePartitionRootV1 as imageRoot,openRepositoryCachePartitionV1,withRepositoryCachePartitionDisposableChildStorageV1,withRepositoryCachePartitionChildStorageV1,withRepositoryCachePartitionFundedChildStorageV1,retireRepositoryCachePartitionFundedChildStorageV1,withRepositoryCachePartitionMaintenanceChildStorageV1} from "../🟦️.ts";')
s=rep(s,'import type importedSchema from "./🧬️schema/🔣️.json";','import type importedSchema from "./🧬️schema/🔣️.json";\nimport type {NativeStorageChildFundingV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌱️birth/🟦️.ts";')
anchor='/** 🧷️ Enrolls only an existing private partition reader and retains its real image through accepted descendants. */'
new='''/** 🏦️ Retains an issued partition reader while its parent-funded child persists its charge in the partition journal. */
export function withRepositoryCacheStoreFundedChildStorageV1<T>(store:RepositoryCacheStoreV1,input:Parameters<typeof openNativePlatformStorageV1>[0],funding:NativeStorageChildFundingV1,body:(child:NativeImageStorageV1,reservation:NativeStorageReservationV1,actualControl:ResourceLeaseControl)=>Promise<T>):Promise<T>{
  const options=Object.freeze({...input}),owner=issuedStores.get(store);if(!owner||owner.store!==store||owner.kind!=="reader"||!owner.partition||typeof body!=="function"||funding.invocation!==store.invocation)throw Error("Exact issued cache funded child owner required");requireOwner(owner);requireIssuedResourceControlInvocationV1(options.control,store.invocation);requireIssuedResourceControlRefinementV1(store.control,options.control);const frame=consumerFrames.getStore();if(!frame||!retainedFrame(owner,frame))throw Error("Original cache funded child consumer frame required");
  return enrollConsumer(frame,options.control,actualControl=>{requireOwner(owner);return withRepositoryCachePartitionFundedChildStorageV1(store.image,Object.freeze({...options,control:actualControl}),funding,body);},owner);
}
/** ♻️ Retires one persisted funded child through an issued partition reader and its recovered parent charge. */
export function retireRepositoryCacheStoreFundedChildStorageV1(store:RepositoryCacheStoreV1,reservation:NativeStorageReservationV1,reservationControl:ResourceLeaseControl,control:ResourceLeaseControl):Promise<void>{
  const owner=issuedStores.get(store);if(!owner||owner.store!==store||owner.kind!=="reader"||!owner.partition)throw Error("Exact issued cache funded child retirement owner required");requireOwner(owner);requireIssuedResourceControlInvocationV1(control,store.invocation);requireIssuedResourceControlRefinementV1(store.control,control);const frame=consumerFrames.getStore();if(!frame||!retainedFrame(owner,frame))throw Error("Original cache funded child consumer frame required");
  return enrollConsumer(frame,control,actualControl=>{requireOwner(owner);return retireRepositoryCachePartitionFundedChildStorageV1(store.image,reservation,reservationControl,actualControl,store.invocation);},owner);
}

'''
s=rep(s,anchor,new+anchor)
save(p,s)
print("ok")
