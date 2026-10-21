import{createHash}from"node:crypto";
import{GENERATION_POLICY_V1,canonicalJsonV1,compareCodePointsV1,createGenerationStoreV1,rowKeyV1}from"../🟨️.mjs";
import{createGraphPhaseClockV1}from"../../⏱️phases/🟨️.mjs";

/** 🗣️ The bilingual texts of the graph generation; the schema file declares the same table and a law keeps both equal. */
export const GENERATION_MESSAGES_V1=Object.freeze({
 en:Object.freeze({
  current:"graph: serving generation {generation} without verification (graph policy current)",
  verified:"graph: generation {generation} verified, {checked} facts checked",
  waiting:"graph: waiting for the graph build of a peer to finish",
  building:"graph: refreshing {dirty} projects (generation {generation})",
  published:"graph: generation {generation} published, {computed} projects refreshed in {seconds} s",
  closure:"graph: {count} projects of the closure verified, no other project read",
  cold:"graph: no usable generation ({reason}), building the first one",
  stale:"graph: serving generation {generation} built by another implementation (graph policy current)",
 }),
 de:Object.freeze({
  current:"Graph: Generation {generation} wird ohne Prüfung geliefert (Graph-Richtlinie current)",
  verified:"Graph: Generation {generation} geprüft, {checked} Fakten kontrolliert",
  waiting:"Graph: wartet auf den Graph-Aufbau eines anderen Prozesses",
  building:"Graph: {dirty} Projekte werden erneuert (Generation {generation})",
  published:"Graph: Generation {generation} veröffentlicht, {computed} Projekte in {seconds} s erneuert",
  closure:"Graph: {count} Projekte der Hülle geprüft, kein weiteres Projekt gelesen",
  cold:"Graph: keine brauchbare Generation ({reason}), die erste wird aufgebaut",
  stale:"Graph: Generation {generation} stammt von einer anderen Implementierung und wird geliefert (Graph-Richtlinie current)",
 }),
});

/** 🌐️ Picks the language of the process like every other progress text: dashboard locale, then the system locale, English first and German second. */
export function selectGenerationLanguageV1(environment){
 return["SEMIO_DASHBOARD_LOCALE","LC_ALL","LC_MESSAGES","LANG"].map(name=>environment[name]?.split(/[_.@-]/u)[0]?.toLowerCase()).find(value=>value==="en"||value==="de")??"en";
}

/** 🗣️ Renders one message in the language of the environment. */
export function renderGenerationMessageV1(key,fields,environment){
 const template=GENERATION_MESSAGES_V1[selectGenerationLanguageV1(environment)][key];
 if(template===undefined)throw Error("Closed generation message required: "+key);
 return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
}

/** 🚦️ Reads the graph policy from `--graph-policy fresh|current` (before `--`) or the environment; `--graph` is taken by the portable dry-run alias. Returns the policy, whether anything chose it and the remaining arguments. */
export function extractGraphPolicyV1(args,environment){
 let policy=environment.SEMIO_BUILD_GRAPH_POLICY??"fresh",explicit=environment.SEMIO_BUILD_GRAPH_POLICY!==undefined;const remaining=[];
 for(let index=0;index<args.length;index++){
  const argument=args[index];
  if(argument==="--"){remaining.push(...args.slice(index));break;}
  if(argument==="--graph-policy"){policy=args[++index]??"";explicit=true;continue;}
  if(argument.startsWith("--graph-policy=")){policy=argument.slice("--graph-policy=".length);explicit=true;continue;}
  remaining.push(argument);
 }
 if(!GENERATION_POLICY_V1.policies.includes(policy))throw Error("Graph policy fresh or current required: "+policy);
 return{policy,explicit,args:remaining};
}

/** 🧱️ Splits the facts of the loaded implementation by their consumers: a module no record can depend on is dropped, a module that only shapes the nodes of valid projects belongs to those projects, every other module to every project, every unavailable project and the environment; identical facts are kept once. */
export function scopeImplementationRowsV1(rows){
 const scopes=GENERATION_POLICY_V1.implementation,all=[],projects=[],seen=new Set();
 for(const row of rows){
  const key=rowKeyV1(row);if(seen.has(key)||scopes.none.some(prefix=>row.path.includes(prefix)))continue;seen.add(key);
  (scopes.projects.some(prefix=>row.path.includes(prefix))?projects:all).push(row);
 }
 return{all,projects};
}

/** 🪪️ The digest that stamps a generation: the loaded bytes of every module of the implementation, whoever consumes it. */
export function implementationDigestV1(rows){
 return createHash("sha256").update(canonicalJsonV1(rows.map(row=>[row.path,row.fact.sha256]).sort((a,b)=>compareCodePointsV1(a[0],b[0])||compareCodePointsV1(a[1],b[1])))).digest("hex");
}

/**
 * 🧩️ Shapes what one partial inference produced into the compute result of the store: every owner carries the facts of its own inference record plus the captured facts below its roots, edges go to their source owner, facts nobody consumed belong to the environment.
 * `workspaceRoot` makes row paths comparable (rows carry absolute paths); the root project never claims facts by path.
 */
export function shapeComputeResultV1({workspaceRoot,part,stubs,separator="/",implementation={all:[],projects:[]}}){
 const processed=new Set([...part.owners.map(owner=>owner.name),...part.edgeOnly.map(owner=>owner.name)]),edgesOf=new Map(),other=[];
 for(const edge of part.edges){if(processed.has(edge.source)){let list=edgesOf.get(edge.source);if(!list)edgesOf.set(edge.source,list=[]);list.push(edge);}else other.push(edge);}
 const prefix=root=>root==="."?undefined:workspaceRoot+separator+root.split("/").join(separator)+separator,prefixes=roots=>roots.filter(Boolean).map(prefix).filter(Boolean);
 const claimed=new Set(),owners=[];
 const claim=(base,roots)=>{
  const keys=new Set(base.map(rowKeyV1)),rows=[...base],list=prefixes(roots);
  if(list.length)for(const row of part.rows)if(list.some(entry=>row.path.startsWith(entry))){const key=rowKeyV1(row);if(!keys.has(key)){keys.add(key);rows.push(row);}}
  for(const key of keys)claimed.add(key);return rows;
 };
 for(const owner of part.owners)owners.push({name:owner.name,root:owner.root,kind:owner.unavailable?"unavailable":"project",node:owner.unavailable??owner.node,edges:edgesOf.get(owner.name)??[],targets:owner.unavailable?[]:Object.keys(owner.node?.targets??{}),...(owner.config?{config:owner.config}:{}),...(owner.nativeRoot?{nativeRoot:owner.nativeRoot}:{}),...(owner.tag?{tag:owner.tag}:{}),rows:[...claim(owner.nodeRows??[],[owner.root,owner.nativeRoot]),...implementation.all,...(owner.unavailable?[]:implementation.projects)]});
 for(const owner of part.edgeOnly){const stub=stubs[owner.name];owners.push({name:owner.name,partial:"edges",edges:edgesOf.get(owner.name)??[],rows:claim([],[stub.root,stub.nativeRoot])});}
 const environment=[...part.env.rows,...part.rows.filter(row=>!claimed.has(rowKeyV1(row))),...implementation.all];
 return{owners,env:{digest:part.env.digest,rows:environment},global:{version:1,externalNodes:part.externalNodes??{},edges:other,unavailable:part.parseUnavailable??[]},replaceTags:part.replaceTags??[]};
}

/** 📊️ Turns the store's progress events into the shared graph phase counters with their bilingual labels. */
export function createGenerationProgressV1(clock){
 const active=new Set();
 return async({phase,done,total})=>{
  if(done===0||!active.has(phase)){if(active.has(phase))await clock.end(phase);await clock.begin(phase,total);active.add(phase);}
  await clock.advance(phase,done);
  if(done>=total){await clock.end(phase);active.delete(phase);}
 };
}

const needsBuild=async()=>({built:false,published:false,needsBuild:true,computed:0});

/**
 * 🕰️ Loads the all-workspace graph (or the closure of some projects) from the newest sealed generation. A shared reader verifies and serves it without ever taking a writer; only a stale generation (and a fresh policy) enters the builder custody, where a holder that finds the work done by a peer serves that generation.
 * `host` binds the repository: `identity()`, `withShared(body)`, `withBuilder(body)`, `storeOptions(store, control)`, `membership(previous, signal)`, `discoverPart(spec, signal)`, `expand(projects, control)`, `readGraph(value)`, `clock(control)`, `say(key, fields)` and the constants `workspaceRoot`, `namedInputs`, `registryDigest`, `signal`.
 */
export async function loadGenerationGraphV1(host,{policy="fresh",closure}={}){
 const identity=await host.identity(),roots=closure?[...new Set(closure)]:undefined,rows=await host.implementationRows(),implementation=implementationDigestV1(rows),scoped=scopeImplementationRowsV1(rows);
 const attempt=async(builder,store,control)=>{
  const engine=createGenerationStoreV1(host.storeOptions(store,control)),progress=createGenerationProgressV1(host.clock(control));
  const compute=builder?async(spec,signal)=>shapeComputeResultV1({workspaceRoot:host.workspaceRoot,separator:host.separator,part:await host.discoverPart(spec,signal,control),stubs:spec.stubs,implementation:scoped}):async()=>{throw Error("A shared graph reader never computes");};
  const base={identity,implementation,policy:builder?"fresh":policy,membership:host.membership,compute,signal:host.signal,progress,builder:builder?body=>body():needsBuild};
  const started=Date.now(),result=roots?await engine.refreshClosure({...base,roots}):await engine.refresh(base);
  if(result.needsBuild)return{needsBuild:true};
  const assembled=await engine.assemble(result.current,{names:result.names&&[...result.names],compare:(a,b)=>a.localeCompare(b),signal:host.signal,progress});
  const projects=await host.expand(assembled.projects,control);
  const graph=host.readGraph({version:1,projects,dependencies:assembled.dependencies,namedInputs:host.namedInputs,externalNodes:assembled.externalNodes,...(assembled.unavailable.length?{unavailable:assembled.unavailable}:{}),admission:{selection:{kind:"all"},registry:host.registryDigest,closure:assembled.membership}});
  const generation=result.current.manifest.generation;
  if(result.served==="current")host.say(result.current.manifest.implementation===implementation?"current":"stale",{generation});
  else if(result.served==="built"&&result.published)host.say("published",{generation,computed:result.computed,seconds:Math.round((Date.now()-started)/1000)});
  else if(host.verbose)host.say(roots?"closure":"verified",roots?{count:result.names.size}:{generation,checked:result.plan?.checked??0});
  return{graph,result};
 };
 const first=await host.withShared((store,control)=>attempt(false,store,control));
 if(!first.needsBuild)return first.graph;
 return(await host.withBuilder((store,control)=>attempt(true,store,control))).graph;
}
