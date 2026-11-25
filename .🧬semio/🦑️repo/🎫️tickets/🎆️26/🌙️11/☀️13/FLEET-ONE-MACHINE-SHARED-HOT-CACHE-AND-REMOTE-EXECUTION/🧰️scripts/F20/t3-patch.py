import sys
base="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/"
def sub(path,old,new):
    t=open(path,encoding="utf8").read()
    assert t.count(old)==1,(path,old[:60],t.count(old))
    open(path,"w",encoding="utf8").write(t.replace(old,new))
ts=base+"🟦️.ts"
sub(ts,'import {refuseBootstrapV1} from "../../📖️texts/🟦️.ts";','import {refuseBootstrapV1} from "../../📖️texts/🟦️.ts";\nimport {concurrentEditV1,retryOnConcurrentEditV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";')
old=' for(let attempt=1;;attempt++){try{return await walkNxAuthorizerSourcesV1(workspace,entry,signal,options,progress,attempt);}catch(error){signal.throwIfAborted();if(attempt>=membershipRetryV1.attempts||!membershipChangeV1(error))throw error;options.report?.("sources moved while they were read, walking the tool closure again ("+attempt+" of "+membershipRetryV1.attempts+")");}}\n'
new=''' let attempted=0;
 try{return await retryOnConcurrentEditV1("read",async attempt=>{attempted=attempt;try{return await walkNxAuthorizerSourcesV1(workspace,entry,signal,options,progress,attempt);}catch(error){signal.throwIfAborted();if(!membershipChangeV1(error))throw error;throw Object.assign(concurrentEditV1("vanished",(error as {path?:string}).path??entry,String((error as Error)?.message??error)),{membership:error});}},{signal,retryable:()=>attempted<membershipRetryV1.attempts,onRetry:event=>options.report?.("sources moved while they were read, walking the tool closure again ("+event.attempt+" of "+membershipRetryV1.attempts+") / Quellen änderten sich beim Lesen, der Werkzeugabschluss wird erneut durchlaufen ("+event.attempt+" von "+membershipRetryV1.attempts+")")});}
 catch(error){const held=error as {membership?:unknown;cause?:{membership?:unknown}};throw held.membership??held.cause?.membership??error;}
'''
sub(ts,old,new)
sc=base+"🧬️schema/🔣️.json"
sub(sc,'"const": { "attempts": 4 }, "description": "A path that vanishes, is replaced or changes between the moment a capture lists or resolves it and the moment it reads it is a membership change of the live tree, not a failure: the walk starts again from the entry (at most `attempts` walks in all)','"const": { "attempts": 6 }, "description": "A path that vanishes, is replaced or changes between the moment a capture lists or resolves it and the moment it reads it is a membership change of the live tree, not a failure: the walk starts again from the entry through `retryOnConcurrentEditV1` of the `read` profile, so it pauses 10, 40, 160, 640 and 2560 ms between walks while the replica settles (at most `attempts` walks in all)')
sub(base+"🧪️tests/🧭️ancestry/🟦️.ts","expect(attempts).toBe(4);","expect(attempts).toBe(6);")
