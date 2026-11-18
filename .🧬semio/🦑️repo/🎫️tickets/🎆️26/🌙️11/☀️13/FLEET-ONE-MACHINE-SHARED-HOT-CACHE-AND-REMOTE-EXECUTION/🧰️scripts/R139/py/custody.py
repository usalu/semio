import sys
p=sys.argv[1]
s=open(p,encoding="utf8").read()
def rep(old,new):
    global s
    assert s.count(old)==1,(old[:60],s.count(old))
    s=s.replace(old,new)
rep('import schema from"./🧬️schema/🔣️.json"with{type:"json"};','import{advanceProgressV1,askReasonV1,idleMillisecondsV1,isDeposedV1,holderSuffixV1,readAskV1,readHolderV1,readProgressV1,rememberDoorV1}from"../🪧️door-board/🟨️.mjs";\nimport schema from"./🧬️schema/🔣️.json"with{type:"json"};')
rep('return template.replace(/\\{([a-z]+)\\}/gu,(_token,name)=>String(fields[name]));','return template.replace(/\\{([a-z]+)\\}/gu,(_token,name)=>String(fields[name]??""));')
rep(' const required=schema.definitions.Claim.required;\n',' const required=schema.definitions.Claim.required,allowed=Object.keys(schema.definitions.Claim.properties);\n')
rep('for(const key of Object.keys(value))if(!required.includes(key))throw','for(const key of Object.keys(value))if(!allowed.includes(key))throw')
rep(' if(value.version!==1||typeof value.token','''
 if(value.holder!==undefined)try{readHolderV1(value.holder);}catch{throw fail("Closed custody holder required");}
 if(value.version!==1||typeof value.token''')
# createSliceCustodyV1
rep(' const directory=join(root,P.directory),pathOf=name=>join(directory,name+".json"),own=new Map();',' const directory=join(root,P.directory),pathOf=name=>join(directory,name+".json"),sidePath=(name,kind)=>join(directory,name+"."+kind+".json"),own=new Map(),SIDE=/^([a-f0-9]{24})\\.(progress|ask)\\.json$/u;')
rep(' const readOne=async name=>{',''' const readSide=async(name,kind,read)=>{const bytes=await fs.read(sidePath(name,kind));if(!bytes)return undefined;try{return read(JSON.parse(Buffer.from(bytes).toString("utf8")));}catch{return undefined;}};
 const writeSide=(name,kind,value)=>fs.write(sidePath(name,kind),Buffer.from(canonicalJsonV1(value)));
 const removeSides=name=>Promise.all(["progress","ask"].map(kind=>fs.remove(sidePath(name,kind)).catch(()=>{})));
 const readOne=async name=>{''')
old_live=s[s.index(' async function live(signal){'):s.index(' const admitted=async(signal,body)')]
new_live=''' async function live(signal){
  const rows=[],files=await fs.list(directory),sides=new Map(),named=new Set();
  for(const file of files){const found=SIDE.exec(file.name);if(found)sides.set(file.name,found[1]);else if(/^[a-f0-9]{24}\\.json$/u.test(file.name))named.add(file.name.slice(0,24));}
  for(const file of files){
   aborted(signal);
   if(!/^[a-f0-9]{24}\\.json$/u.test(file.name))continue;
   if(rows.length>=P.maximumClaims)throw fail("Bounded number of custody claims required","CUSTODY_FULL");
   const name=file.name.slice(0,24);
   const attach=async claim=>{const extra={};if(sides.has(name+".progress.json"))extra.progress=await readSide(name,"progress",readProgressV1);if(sides.has(name+".ask.json"))extra.ask=await readSide(name,"ask",readAskV1);return{...claim,...extra};};
   if(own.has(name)){rows.push(await attach(own.get(name)));continue;}
   const claim=await readOne(name);
   if(!claim){if(now()-file.modifiedMs>10*P.pollMilliseconds){await fs.remove(join(directory,file.name));await removeSides(name);}continue;}
   const probe=await lease.tryHold(resource.claim(name));
   if(!probe){rows.push(await attach(claim));continue;}
   try{await fs.remove(pathOf(name));await removeSides(name);}finally{probe.release();}
  }
  for(const[sideName,owner]of sides)if(!named.has(owner)&&now()-(files.find(file=>file.name===sideName)?.modifiedMs??now())>10*P.pollMilliseconds)await fs.remove(join(directory,sideName)).catch(()=>{});
  return rows;
 }

 const active=async signal=>{const at=now();return(await live(signal)).filter(row=>!isDeposedV1(row,at));};

'''
s=s.replace(old_live,new_live)
open(p,"w",encoding="utf8").write(s)
