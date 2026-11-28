import os,sys
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🟦️.ts"
s=open(P,encoding='utf8').read()
def rep(a,b,n=1):
    global s
    if s.count(a)!=n: sys.exit("anchor count %d (want %d): %s"%(s.count(a),n,a[:80]))
    s=s.replace(a,b)
rep('persistentChildContext=new AsyncLocalStorage<PersistentChildScope>();','persistentChildContext=new AsyncLocalStorage<PersistentChildScope>(),persistentOpenFrames=new WeakMap<object,PersistentChildScope>();')
rep('frame.options=Object.freeze({...options,control});return openPersistentNativeImageV1(frame.options);','frame.options=Object.freeze({...options,control});persistentOpenFrames.set(frame.options,frame);return openPersistentNativeImageV1(frame.options);',2)
rep('child=await openPersistentNativeImageV1(options);','persistentOpenFrames.set(options,frame);child=await openPersistentNativeImageV1(options);')
rep('admittedParent=persistentChildContext.getStore(),parent=admittedParent?.options===input?admittedParent:undefined;','admittedParent=persistentOpenFrames.get(input)??persistentChildContext.getStore(),parent=admittedParent?.options===input?admittedParent:undefined;')
rep('import{acceptResourceLeaseCleanupV1}from"../../../../../🔒️leases/🟦️.ts";','import{acceptResourceLeaseCleanupV1,holdsExclusiveInTaskV1}from"../../../../../🔒️leases/🟦️.ts";')
rep('}else if(existingOnly!=="maintenance"){receiver.guard=await persistentGuard(receiver,"shared",options.control,!existingOnly,undefined,false,preparationControl);','}else if(existingOnly!=="maintenance"){if(holdsExclusiveInTaskV1({directory:options.leaseDirectory,resource:"native-fixed-image:"+directory,control:options.control,signal:options.control.signal}))throw Error("Native image opened inside its own creation: the same task still holds "+directory+" exclusively and would wait for itself / Native-Abbild innerhalb der eigenen Erzeugung geöffnet: dieselbe Aufgabe hält "+directory+" noch exklusiv und würde auf sich selbst warten");receiver.guard=await persistentGuard(receiver,"shared",options.control,!existingOnly,undefined,false,preparationControl);')
if os.environ.get("H48_DRY"): print("dry ok"); sys.exit(0)
open(P+".h48tmp","w",encoding='utf8').write(s); os.replace(P+".h48tmp",P); print("ok")
