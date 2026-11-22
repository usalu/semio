import sys
P=sys.argv[1].rstrip('/')+'/'
def edit(path,pairs):
    s=open(path).read()
    for a,b in pairs:
        assert s.count(a)==1,(path,a[:80],s.count(a))
        s=s.replace(a,b)
    open(path,'w').write(s)
edit(P+'🟦️.ts',[
('''/** 🚫️ Tells the fleet that an owner failed with a deterministic cause on this machine, so the other pullers leave it to the machine that runs the wave. Never throws. */
export async function publishOwnerPoolFailureV1(input:Readonly<{client:GraphFleetClientV1;owner:string;origin:string;recordDigest:string;revision:string;cause:string;now?:()=>number}>):Promise<boolean>{
 try{return await publishGraphEntryV1({client:input.client,state:ownerPoolFailureStateV1(input.recordDigest),owner:input.owner,documents:[{name:P.failureName,bytes:encode({version:1,revision:input.revision,cause:input.cause.slice(0,300),origin:input.origin,atMs:(input.now??Date.now)()})}]});}catch{return false;}
}''','''/** 🚫️ Tells the fleet that an owner failed with a deterministic cause on this machine, so the other pullers leave it to the machine that runs the wave. A live note of the same record state and revision is never written again: the same cause writes nothing, another cause is added to the note (`causes`, at most four, the first stays `cause`) while its origin and time stay those of the first failure, so the note neither flaps between machines nor outlives its first failure and every cause stays visible. Never throws. */
export async function publishOwnerPoolFailureV1(input:Readonly<{client:GraphFleetClientV1;owner:string;origin:string;recordDigest:string;revision:string;cause:string;now?:()=>number}>):Promise<boolean>{
 try{
  const now=input.now??Date.now,cause=input.cause.slice(0,300),live=await readOwnerPoolFailureV1(input.client,input.recordDigest,input.revision,now);
  if(live!==undefined&&live.causes.includes(cause))return true;
  const note=live===undefined?{cause,causes:[cause],origin:input.origin,atMs:now()}:{cause:live.cause,causes:[...live.causes,cause].slice(0,4),origin:live.origin,atMs:live.atMs};
  return await publishGraphEntryV1({client:input.client,state:ownerPoolFailureStateV1(input.recordDigest),owner:input.owner,documents:[{name:P.failureName,bytes:encode({version:1,revision:input.revision,...note})}]});
 }catch{return false;}
}'''),
('Promise<Readonly<{cause:string;origin:string;atMs:number}>|undefined>{','Promise<Readonly<{cause:string;causes:readonly string[];origin:string;atMs:number}>|undefined>{'),
('as {version:number;revision:string;cause:string;origin:string;atMs:number};\n  return note.version===1&&note.revision===revision&&now()-note.atMs<=P.failureMaximumAgeMilliseconds?note:undefined;','as {version:number;revision:string;cause:string;causes?:string[];origin:string;atMs:number};\n  return note.version===1&&note.revision===revision&&now()-note.atMs<=P.failureMaximumAgeMilliseconds?{cause:note.cause,causes:note.causes??[note.cause],origin:note.origin,atMs:note.atMs}:undefined;')])
edit(P+'🧬️schema/🔣️.json',[('''        "cause": {
          "type": "string",
          "maxLength": 300
        },
        "origin": {
          "type": "string"
        },
        "atMs": {
          "type": "integer",
          "minimum": 0
        }
      }
    }
  }''','''        "cause": {
          "type": "string",
          "maxLength": 300
        },
        "causes": {
          "description": "Every distinct cause the machines of the fleet reported while the note was live, the first one first (at most four); absent in a note with one cause.",
          "type": "array",
          "maxItems": 4,
          "items": { "type": "string", "maxLength": 300 }
        },
        "origin": {
          "type": "string"
        },
        "atMs": {
          "type": "integer",
          "minimum": 0
        }
      }
    }
  }''')])
edit(P+'🧪️tests/🟦️.ts',[('''  expect(await readOwnerPoolFailureV1(client,digest,"rev1",()=>Date.now()+P.failureMaximumAgeMilliseconds+1000)).toBeUndefined();
 }finally{await server.stop();}
});''','''  expect(await readOwnerPoolFailureV1(client,digest,"rev1",()=>Date.now()+P.failureMaximumAgeMilliseconds+1000)).toBeUndefined();
 }finally{await server.stop();}
});

test("a live failure note is not written again: the same cause writes nothing, another cause is added to it, the first origin and time stay and at most four causes are kept",async()=>{
 const {server,environment}=await world();
 try{
  const root=temp("n"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})}),digest=sha("n");let clock=1000;
  const publish=(origin:string,cause:string,revision="rev1")=>publishOwnerPoolFailureV1({client,owner:origin,origin,recordDigest:digest,revision,cause,now:()=>clock});
  const puts=async()=>((await (await fetch(server.url+"/status",{headers:{authorization:"Bearer w"}})).json()) as {counters:{ac:{puts:number}}}).counters.ac.puts;
  expect(await publish("g1-m01","boom A")).toBe(true);const first=await puts();
  clock=2000;expect(await publish("g1-m02","boom A")).toBe(true);expect(await puts(),"the same cause writes nothing").toBe(first);
  clock=3000;expect(await publish("g1-m02","boom B")).toBe(true);expect(await puts()).toBe(first+1);
  expect(await readOwnerPoolFailureV1(client,digest,"rev1",()=>3000)).toEqual({cause:"boom A",causes:["boom A","boom B"],origin:"g1-m01",atMs:1000});
  for(const cause of["boom C","boom D","boom E"]){clock+=10;expect(await publish("g1-m03",cause)).toBe(true);}
  expect((await readOwnerPoolFailureV1(client,digest,"rev1",()=>3100))?.causes).toEqual(["boom A","boom B","boom C","boom D"]);
  expect(await publish("g1-m01","other revision","rev2")).toBe(true);expect((await readOwnerPoolFailureV1(client,digest,"rev2",()=>3100))?.causes).toEqual(["other revision"]);
 }finally{await server.stop();}
});''')])
print("ok")
