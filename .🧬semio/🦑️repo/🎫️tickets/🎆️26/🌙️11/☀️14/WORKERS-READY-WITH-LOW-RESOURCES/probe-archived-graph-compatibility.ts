import {createHash} from "node:crypto";
import {brotliDecompressSync} from "node:zlib";
import {isAbsolute,join,relative,sep} from "node:path";
import {pathToFileURL} from "node:url";

const workspace=process.cwd(),build=pathToFileURL(join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build")+"/");
const {withImplementationPinV1}=await import(new URL("📥️inputs/🔏️revision/🟨️.mjs",build).href);
await withImplementationPinV1(async()=>{
 const [{GENERATION_POLICY_V1:policy,createNodeFilePortV1,readGenerationPointerV1,readGenerationManifestV1,openPackedFactIndexV1,readGenerationFragmentV1},{loadedGraphImplementationRowsV1},{buildGraphGenerationIdentityV1},{implementationDigestV1},{readBuildOwnerRegistryV1},producer]=await Promise.all([
  import(new URL("🕸️graph/🕰️generations/🟨️.mjs",build).href),
  import(new URL("🕸️graph/🔐️authority/🟦️.ts",build).href),
  import(new URL("🕸️graph/🏷️retention/🟦️.ts",build).href),
  import(new URL("🕸️graph/🕰️generations/🔌️load/🟨️.mjs",build).href),
  import(new URL("🕸️graph/🚪️admission/🟦️.ts",build).href),
  import(new URL("🕸️graph/📥️inference/🟨️.mjs",build).href),
 ]);
 const fs=createNodeFilePortV1(),root=join(workspace,".cache","b","g","m","generations"),pointerPath=join(root,policy.pointer),before=await fs.read(pointerPath);
 if(!before)throw Error("Missing graph generation pointer");
 const current=readGenerationPointerV1(before),key=current.manifest??current.parent;
 if(!key)throw Error("No retained manifest");
 const bytes=await fs.read(join(root,policy.manifests,key+".json"));
 if(!bytes?.length||bytes.length>policy.maximumManifestBytes||createHash("sha256").update(bytes).digest("hex")!==key)throw Error("Retained manifest is not digest-verified");
 const retained=readGenerationManifestV1(bytes);
 await producer.libraryBootstrap;
 const registry=readBuildOwnerRegistryV1(JSON.parse(producer.readBuildOwnerRegistrySourceV1(workspace))),identity=buildGraphGenerationIdentityV1(registry);
 const inputRows=[];
 for(const row of [...loadedGraphImplementationRowsV1,...producer.loadedImplementationRowsV1]){
  const inside=relative(workspace,row.path);
  if(inside===""||inside.startsWith("..")||isAbsolute(inside))continue;
  inputRows.push({...row,path:inside.split(sep).join("/")});
 }
 const rows=producer.classifyImplementationRowsV1(inputRows),implementation=implementationDigestV1(rows),after=await fs.read(pointerPath);
 if(!after||!before.equals(after))throw Error("Generation pointer moved during comparison");
 if(process.argv.includes("--facts")){
  const packed=await fs.read(join(root,policy.indexes,retained.factIndex+".facts.br"));
  if(!packed?.length||packed.length>policy.maximumIndexBytes||createHash("sha256").update(packed).digest("hex")!==retained.factIndex)throw Error("Retained index is not digest-verified");
  const index=openPackedFactIndexV1(packed,policy.maximumIndexBytes,"implementation-audit"),kindPath=(kind:string,path:string)=>JSON.stringify([kind,path]),valueOf=(row:any)=>row.kind==="stamp"?row.fact:row.fact.sha256;
  const wanted=new Set(rows.map((row:any)=>kindPath(row.kind,row.path))),versions=new Map<string,Set<unknown>>();
  for(let id=0;id<index.rows.length;id++){
   const kind=index.rowKindOf(id);
   if(kind!=="digest"&&kind!=="stamp")continue;
   const key=kindPath(kind,index.textOf(index.rowPathIdOf(id)));
   if(!wanted.has(key))continue;
   const values=versions.get(key)??new Set<unknown>();
   values.add(valueOf(index.rows[id]));
   versions.set(key,values);
  }
  const changed=rows.flatMap((row:any)=>{
   const values=versions.get(kindPath(row.kind,row.path));
   return values?.has(valueOf(row))?[]:[{kind:row.kind,path:row.path,scope:row.scope,previous:[...values??[]],current:valueOf(row)}];
  });
  const same=await fs.read(pointerPath);
  if(!same||!before.equals(same))throw Error("Generation pointer moved during index comparison");
  const reconstructed=rows.flatMap((row:any)=>{
   const values=versions.get(kindPath(row.kind,row.path));
   if(values?.has(valueOf(row)))return[row];
   if(values?.size!==1)return[];
   const value=[...values][0];
   return[{...row,fact:row.kind==="stamp"?value:{...row.fact,sha256:value}}];
  });
  const roster=Object.values(retained.owners),unavailable=roster.filter(owner=>owner.kind==="unavailable").length;
  process.stdout.write("[DEBUG] "+JSON.stringify({archiveRows:index.rows.length,archiveIndexBackingBytes:index.stats().bytes,roster:roster.length,unavailable,currentRows:rows.length,reconstructedRows:reconstructed.length,reconstructedImplementation:implementationDigestV1(reconstructed),changed})+"\n");
  index.drop();
 }
 if(process.argv.includes("--withheld")){
  const unavailable=Object.entries(retained.owners).filter(([,owner])=>owner.kind==="unavailable");
  if(unavailable.length>64)throw Error("Withheld diagnostic owner bound");
  const counts:Record<string,number>={};
  for(const [name,owner] of unavailable){
   const packed=await fs.read(join(root,policy.fragments,owner.fragment+".json.br"));
   if(!packed?.length||packed.length>policy.maximumFragmentBytes||createHash("sha256").update(packed).digest("hex")!==owner.fragment)throw Error("Withheld fragment digest mismatch");
   const value=readGenerationFragmentV1(JSON.parse(brotliDecompressSync(packed,{maxOutputLength:policy.maximumFragmentBytes}).toString("utf8")));
   if(value.name!==name||value.kind!==owner.kind||value.root!==owner.root)throw Error("Withheld fragment authority mismatch");
   const reason=String(value.node?.reason??"");
   const kind=reason.includes("Native dependency has no build project owner")||reason.includes("Canonical native dependency owner required")||reason.includes("Native execution contract requires a native project owner")?"registry":/farm children could not deliver/u.test(reason)?"farm-refusal":/memory|headroom|MiB|MB|capacity/iu.test(reason)?"resource":/parse/iu.test(reason)?"parse":Array.isArray(value.node?.missing)?"dependency":"other";
   counts[kind]=(counts[kind]??0)+1;
  }
  const same=await fs.read(pointerPath);
  if(!same||!before.equals(same))throw Error("Generation pointer moved during withheld comparison");
  process.stdout.write("[DEBUG] "+JSON.stringify({withheld:unavailable.length,classes:counts})+"\n");
 }
 if(process.argv.includes("--fleet")){
  const {fleetOriginsV1,remoteGenerationExchangeV1}=await import(new URL("🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts",build).href),origins=fleetOriginsV1(workspace);
  const exchange=remoteGenerationExchangeV1({workspace,identity:async()=>identity,environment:process.env,serving:false});
  const look=await exchange?.look({root,fs,implementation,requireComplete:true,signal:AbortSignal.timeout(15000)});
  process.stdout.write("[DEBUG] "+JSON.stringify({fleetConfigured:exchange!==undefined,origins:origins.all.length,look})+"\n");
 }
 process.stdout.write("[DEBUG] "+JSON.stringify({generation:current.generation,reason:current.reason??null,archive:retained.generation,archivedIdentity:retained.identity,currentIdentity:identity,archivedImplementation:retained.implementation,currentImplementation:implementation,identityMatches:retained.identity===identity,implementationMatches:retained.implementation===implementation,currentRows:rows.length})+"\n");
});
