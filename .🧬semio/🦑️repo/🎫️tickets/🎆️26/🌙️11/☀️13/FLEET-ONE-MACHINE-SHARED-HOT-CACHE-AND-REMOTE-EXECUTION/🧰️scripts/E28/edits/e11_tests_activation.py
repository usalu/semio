TL="⚡️caching/🚀️bootstrap/🛠️tools/"
def plan(p,L):
    t=L+TL+"🔗️activation/🧪️tests/🟦️.ts"
    p.sub(t,',authorizers:{accepted:true}','',4)
    p.sub(t,'JSON.stringify({version:1,digest:basename(previous),authorizers:{accepted:row.kind!=="invalid-receipt"}})','JSON.stringify({version:1,digest:row.kind==="invalid-receipt"?"f".repeat(64):basename(previous)})')
    p.sub(t,'{modulePath:join(selected,"node_modules")}','{modulePath:join(selected,"node_modules"),authorizer:join(root,".cache/b/t/m/nx-authorizers","e".repeat(64),"semio-authorizers.mjs")}',4)
    p.sub(t,'expect(path).toBe(selected);expect(phase).toBe(control);check();events.push("preflight");},nxAuthorizerReceiptHoldsV1:(_path:unknown,receipt:any)=>receipt?.accepted===true,','expect(path).toBe(join(root,".cache/b/t/m/nx-authorizers","e".repeat(64)));expect(phase).toBe(control);check();events.push("preflight");},nxAuthorizerBundleHoldsV1:()=>({}),')
    p.sub(t,'preflightNxAuthorizersV1:async()=>undefined,nxAuthorizerReceiptHoldsV1:(_path:unknown,receipt:any)=>receipt?.accepted===true,','preflightNxAuthorizersV1:async()=>undefined,nxAuthorizerBundleHoldsV1:()=>({}),')
    p.sub(t,'join,dirname,basename,relative,isAbsolute,process,STORE:','join,dirname,basename,relative,isAbsolute,process,machinePathV1:(root:string,...segments:string[])=>join(root,...segments),STORE:',2)
