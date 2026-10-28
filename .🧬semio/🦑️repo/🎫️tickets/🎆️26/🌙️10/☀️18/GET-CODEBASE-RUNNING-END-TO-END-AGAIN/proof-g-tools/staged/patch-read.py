"""🩹 Staged fix for `🧮️digest/📥️read/🟨️.mjs` (proof-g, 11:50): (1) `verifyObserved` ran INSIDE the tracked operation of the batch, so its concurrent-edit failure closed the Original ("Original process admission closed - first failure") and every retry of the `snapshot` layer failed again, the process exited 75; it now runs after the tracked operation. (2) `stableTreeProtocolV1().settleMilliseconds` is undefined (the settle window lives in `churnProtocolV1()`), the settle rule never applied. Apply after the freeze: python3 patch-read.py <path of the read module>; laws: `🧮️digest/🧪️tests/🔒️batch-snapshot` 2/2 (1/2 red before) and `🧮️digest/🧪️tests/🟦️.ts` 5/5."""
import sys
p=sys.argv[1]; t=open(p).read()
old_head="function exactSourceReadBatchOnceV1(root,control,run,inventoryRoots,store){return withRequestedIssuedResourceControlV1(control,0,async actual=>{"
new_head="async function exactSourceReadBatchOnceV1(root,control,run,inventoryRoots,store){const done=await withRequestedIssuedResourceControlV1(control,0,async actual=>{"
old_tail="check(scope);await verifyObserved(scope);requireCurrentExactSourceReadV1();return value;});}"
new_tail="check(scope);requireCurrentExactSourceReadV1();return{value,scope};});await verifyObserved(done.scope,control);requireCurrentExactSourceReadV1();return done.value;}"
assert t.count(old_head)==1 and t.count(old_tail)==1
t=t.replace(old_head,new_head).replace(old_tail,new_tail)
o2="async function verifyObserved(scope){"; n2="async function verifyObserved(scope,control){"
assert t.count(o2)==1; t=t.replace(o2,n2)
o3="await new Promise(accept=>setTimeout(accept,wait));check(scope);await look();"; n3="await new Promise(accept=>setTimeout(accept,wait));control?.signal?.throwIfAborted();await look();"
assert t.count(o3)==1; t=t.replace(o3,n3)
open(p,"w").write(t)
t=open(p).read()
a="import{concurrentEditV1,retryOnConcurrentEditV1,staleImplementationV1}from"
assert t.count(a)==1; t=t.replace(a,"import{churnProtocolV1,concurrentEditV1,retryOnConcurrentEditV1,staleImplementationV1}from")
b="import{stableTreeSettleTextV1,stableTreeProtocolV1}from"
assert t.count(b)==1; t=t.replace(b,"import{stableTreeSettleTextV1}from")
c="stableTreeProtocolV1().settleMilliseconds"
assert t.count(c)==1; t=t.replace(c,"churnProtocolV1().settleMilliseconds")
open(p,"w").write(t)
