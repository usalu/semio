B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🎟️claims/"
def edit(path,pairs):
    s=open(B+path,encoding='utf8').read()
    for old,new in pairs:
        assert s.count(old)==1,(path,s.count(old),old[:70])
        s=s.replace(old,new)
    open(B+path,'w',encoding='utf8').write(s)
edit('🧬️schema/🔣️.json',[('"ownerPattern": "^[A-Za-z0-9._:@/+-]{1,160}$"','"ownerPattern": "^[A-Za-z0-9._:@/+-]{1,160}$",\n        "minimumLifetimeMilliseconds": 120000,\n        "maximumLifetimeMilliseconds": 1800000,\n        "lifetimeMultiple": 4,\n        "cooloffMilliseconds": 30000')])
edit('🟦️.ts',[
('type Claim={owner:string;grantedMs:number;untilMs:number;ttlMs:number;expectedMs:number|undefined};','type Claim={owner:string;grantedMs:number;untilMs:number;ttlMs:number;expectedMs:number|undefined};\nconst lifetimeOf=(claim:Claim):number=>Math.min(policy.maximumLifetimeMilliseconds,Math.max(policy.minimumLifetimeMilliseconds,(claim.expectedMs??0)*policy.lifetimeMultiple));'),
(' const id=(namespace:string,kind:string,key:string)=>`${namespace}\\u0000${kind}\\u0000${key}`;',' const id=(namespace:string,kind:string,key:string)=>`${namespace}\\u0000${kind}\\u0000${key}`,capped=new Map<string,{owner:string;untilMs:number}>();'),
('''   const address=id(namespace,kind,key),at=now(),current=live(address,at),ttl=clamp(request.ttlMilliseconds);
   if(current&&current.owner!==request.owner)''','''   const address=id(namespace,kind,key),at=now(),current=live(address,at),ttl=clamp(request.ttlMilliseconds),cooling=capped.get(address);
   if(cooling&&cooling.untilMs>at&&cooling.owner===request.owner)return {state:"held",by:"lifetime-cap",untilMilliseconds:cooling.untilMs};
   if(current&&current.owner===request.owner&&at-current.grantedMs>=lifetimeOf(current)){claims.delete(address);capped.set(address,{owner:current.owner,untilMs:at+policy.cooloffMilliseconds});wake(address,"lapsed");return {state:"held",by:"lifetime-cap",untilMilliseconds:at+policy.cooloffMilliseconds};}
   if(current&&current.owner!==request.owner)'''),
('''   const next:Claim={owner:request.owner,grantedMs:current?.grantedMs??at,untilMs:at+ttl,ttlMs:ttl,expectedMs:request.expectedMilliseconds??current?.expectedMs};''','''   const next:Claim={owner:request.owner,grantedMs:current?.grantedMs??at,untilMs:at+ttl,ttlMs:ttl,expectedMs:request.expectedMilliseconds??current?.expectedMs};
   next.untilMs=Math.min(next.untilMs,next.grantedMs+lifetimeOf(next));'''),
('''   if(!current||current.owner!==request.owner)return {state:"lost"};
   const ttl=clamp(request.ttlMilliseconds);current.untilMs=at+ttl;current.ttlMs=ttl;''','''   if(!current||current.owner!==request.owner)return {state:"lost"};
   if(at-current.grantedMs>=lifetimeOf(current)){claims.delete(address);capped.set(address,{owner:current.owner,untilMs:at+policy.cooloffMilliseconds});wake(address,"lapsed");return {state:"lost"};}
   const ttl=clamp(request.ttlMilliseconds);current.untilMs=Math.min(at+ttl,current.grantedMs+lifetimeOf(current));current.ttlMs=ttl;'''),
('export type ClaimRouteInputV1=Readonly<{request:Request;url:URL;table:ClaimTableV1;','export type ClaimRouteInputV1=Readonly<{scope?:string;request:Request;url:URL;table:ClaimTableV1;'),
(''' const raw=await input.readBody(4096);if(raw===undefined)return bad("Body too large");''',''' const raw=await input.readBody(4096);if(raw===undefined)return bad("Body too large or content hash missing");'''),
(''' if(!claimOwnerValidV1(body.owner))return bad("owner invalid");
 if(action==="release")return json(200,{state:input.table.release(namespace,kind,key,body.owner)});''',''' if(!claimOwnerValidV1(body.owner))return bad("owner invalid");
 const given=body.owner,owner=input.scope===undefined?given:`${input.scope}|${given}`,view=(row:Record<string,unknown>):Record<string,unknown>=>{const out={...row};for(const field of["owner","by"]){const value=out[field];if(typeof value!=="string"||input.scope===undefined)continue;const bar=value.indexOf("|");if(bar>0)out[field]=value===owner?given:value.slice(0,bar);}return out;};
 if(action==="release")return json(200,{state:input.table.release(namespace,kind,key,owner)});'''),
('''const outcome=input.table.renew(namespace,kind,key,{owner:body.owner,ttlMilliseconds:ttl});return json(outcome.state==="granted"?200:409,outcome);}''','''const outcome=input.table.renew(namespace,kind,key,{owner,ttlMilliseconds:ttl});return json(outcome.state==="granted"?200:409,view(outcome));}'''),
('''const outcome=input.table.claim(namespace,kind,key,{owner:body.owner,ttlMilliseconds:ttl,''','''const outcome=input.table.claim(namespace,kind,key,{owner,ttlMilliseconds:ttl,'''),
(''' return json(outcome.state==="held"?409:outcome.state==="full"?503:200,outcome);
}''',''' return json(outcome.state==="held"?409:outcome.state==="full"?503:200,view(outcome));
}'''),
])
print("claims ok")
