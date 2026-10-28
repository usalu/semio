import{createHash}from"node:crypto";
import{join}from"node:path";
import{graphQueueRequestedAtV1,graphQueueYieldsV1,yieldGraphQueueV1}from"../🛌️yield/🟨️.mjs";
import{GENERATION_POLICY_V1,canonicalJsonV1,compareCodePointsV1,createGenerationStoreV1,rowKeyV1}from"../🟨️.mjs";
import{createGraphPhaseClockV1}from"../../⏱️phases/🟨️.mjs";
import{decodeVerifiedScopeV1}from"../🔭️coverage/🟨️.mjs";
import{readPinnedGenerationV1}from"../🧷️pin/🟨️.mjs";
import{CUSTODY_POLICY_V1}from"../🔐️custody/🟨️.mjs";

/** 🗣️ The bilingual texts of the graph generation; the schema file declares the same table and a law keeps both equal. */
export const GENERATION_MESSAGES_V1=Object.freeze({
 en:Object.freeze({
  current:"graph: serving generation {generation} without verification (graph policy current)",
  verified:"graph: generation {generation} verified, {checked} facts checked",
  waiting:"graph: waiting for the graph build of a peer ({seconds} s in the queue, generation {generation} was the newest when this command queued)",
  covered:"graph: generation {generation} was sealed by a peer after this command began and already covers it, nothing is checked again",
  pinned:"graph: generation {generation} pinned by the invoking build serves this command ({count} projects), no queue and no check of the live tree",
  building:"graph: refreshing {dirty} projects (generation {generation})",
  published:"graph: generation {generation} published, {computed} projects refreshed in {seconds} s",
  indexed:"graph: fact index of generation {generation}: {rows} facts of {owners} projects, {expanded} MB expanded ({percent} % of the {limit} MB limit), {packed} MB packed",
  swept:"graph: {removed} superseded observation blocks removed ({megabytes} MB, {headers} records kept their blocks)",
  closure:"graph: {count} projects of the closure verified, no other project read",
  cold:"graph: no usable generation ({reason}), building the first one",
  stale:"graph: serving generation {generation} built by another implementation (graph policy current)",
  planned:"graph: {count} projects planned by a child process that has exited",
  summary:"graph: {rows} stale facts ({kinds}) leave {projects} projects stale",
  reason:"graph: {project} is stale: {why}",
  reasonStamp:"the inference protocol of {inferrer} changed from {recorded} to {current}",
  reasonInput:"the input file {path} changed",
  reasonListing:"the entries of the directory {path} changed",
  reasonData:"the declared data file {path} changed",
  reasonAnalysis:"the source analysis of {path} changed",
  reasonFact:"the recorded {kind} fact of {path} changed",
  reasonDerived:"the derived result of {path} changed",
  cutoff:"graph: {questions} derived results asked again after {rows} stale facts: {held} unchanged, {changed} changed",
  reasonMembership:"the workspace file {path} was {change}",
  reasonMore:"graph: {count} more stale inputs are not listed",
  withheldDependency:"graph: {project} is withheld: {source} depends on projects that this graph snapshot does not contain ({target}), created or removed while the graph was built; run the command again to include them, or remove them from the native sources of {project}",
  withheldSource:"graph: {project} is withheld: {source} does not parse (sha256 {sha}); fix that file and run the command again",
  changeAdded:"added",
  changeRemoved:"removed",
  changeChanged:"changed",
  merged:"graph: generation {generation} published, {computed} projects refreshed in {seconds} s and merged onto the generation a peer published meanwhile ({theirs} projects of peers kept)",
 }),
 de:Object.freeze({
  current:"Graph: Generation {generation} wird ohne Prüfung geliefert (Graph-Richtlinie current)",
  verified:"Graph: Generation {generation} geprüft, {checked} Fakten kontrolliert",
  waiting:"Graph: wartet auf den Graph-Aufbau eines anderen Prozesses ({seconds} s in der Warteschlange, Generation {generation} war beim Einreihen die neueste)",
  covered:"Graph: Generation {generation} wurde von einem anderen Prozess nach dem Beginn dieses Befehls versiegelt und deckt ihn bereits ab, nichts wird erneut geprüft",
  pinned:"Graph: die vom aufrufenden Build angeheftete Generation {generation} bedient diesen Befehl ({count} Projekte), ohne Warteschlange und ohne Prüfung des Arbeitsbaums",
  building:"Graph: {dirty} Projekte werden erneuert (Generation {generation})",
  published:"Graph: Generation {generation} veröffentlicht, {computed} Projekte in {seconds} s erneuert",
  indexed:"Graph: Faktenindex der Generation {generation}: {rows} Fakten von {owners} Projekten, {expanded} MB entpackt ({percent} % der Grenze von {limit} MB), {packed} MB gepackt",
  swept:"Graph: {removed} überholte Beobachtungsblöcke entfernt ({megabytes} MB, {headers} Aufzeichnungen behalten ihre Blöcke)",
  closure:"Graph: {count} Projekte der Hülle geprüft, kein weiteres Projekt gelesen",
  cold:"Graph: keine brauchbare Generation ({reason}), die erste wird aufgebaut",
  stale:"Graph: Generation {generation} stammt von einer anderen Implementierung und wird geliefert (Graph-Richtlinie current)",
  planned:"Graph: {count} Projekte von einem beendeten Kindprozess geplant",
  summary:"Graph: {rows} veraltete Fakten ({kinds}) lassen {projects} Projekte veralten",
  reason:"Graph: {project} ist veraltet: {why}",
  reasonStamp:"das Inferenzprotokoll von {inferrer} wechselte von {recorded} zu {current}",
  reasonInput:"die Eingabedatei {path} hat sich geändert",
  reasonListing:"die Einträge des Verzeichnisses {path} haben sich geändert",
  reasonData:"die deklarierte Datendatei {path} hat sich geändert",
  reasonAnalysis:"die Quellanalyse von {path} hat sich geändert",
  reasonFact:"der aufgezeichnete Fakt {kind} von {path} hat sich geändert",
  reasonDerived:"das abgeleitete Ergebnis von {path} hat sich geändert",
  cutoff:"Graph: {questions} abgeleitete Ergebnisse nach {rows} veralteten Fakten erneut abgefragt: {held} unverändert, {changed} verändert",
  reasonMembership:"die Arbeitsbereichsdatei {path} wurde {change}",
  reasonMore:"Graph: {count} weitere veraltete Eingaben werden nicht aufgelistet",
  withheldDependency:"Graph: {project} wird zurückgehalten: {source} hängt von Projekten ab, die dieser Graph-Schnappschuss nicht enthält ({target}); sie wurden während des Graph-Aufbaus angelegt oder entfernt; den Befehl erneut ausführen, um sie aufzunehmen, oder sie aus den nativen Quellen von {project} entfernen",
  withheldSource:"Graph: {project} wird zurückgehalten: {source} lässt sich nicht lesen (sha256 {sha}); die Datei korrigieren und den Befehl erneut ausführen",
  changeAdded:"hinzugefügt",
  changeRemoved:"entfernt",
  changeChanged:"geändert",
  merged:"Graph: Generation {generation} veröffentlicht, {computed} Projekte in {seconds} s erneuert und auf die Generation eines anderen Prozesses zusammengeführt ({theirs} Projekte anderer Prozesse übernommen)",
 }),
});

/** 💬️ Renders one message in the language the caller resolved with the shared resolver. */
export function renderGenerationMessageV1(key,fields,language){
 const template=GENERATION_MESSAGES_V1[language]?.[key];
 if(template===undefined)throw Error("Closed generation message required: "+key);
 return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
}

/** 🚧️ The fields of the message that words one withheld project: the project, its declaring source, the projects it waits for and the start of the digest of the observed bytes. */
function withheldFieldsV1(row){return{project:row.project??row.root,source:row.source,target:row.missing?.join(", ")??"",sha:row.sha256.slice(0,12)};}

/** 🕸️ Words one withheld project in the language of the caller: the projects a test project waits for, or the source that does not parse. */
export function describeUnavailableProjectV1(row,language){return renderGenerationMessageV1(row.missing?"withheldDependency":"withheldSource",withheldFieldsV1(row),language);}

/**
 * 🌐️ Withholds from one graph snapshot every test project that depends on a project the snapshot does not hold: its row (declaring file from `configs`, digest of that file from `digest`, English reason, the missing names) joins `unavailable` under the project root and the project leaves `projects`.
 * Test inference reads project names from the live disk while the inventory is the snapshot of one run, so a project created or removed meanwhile is a dependency "not yet in this generation": only the test project waits and the next run finds the project. Returns the names withheld.
 */
export function withholdDanglingTestProjectsV1({projects,configs,unavailable,dangling,digest}){
 for(const{source,root,missing}of dangling){
  const file=configs[source]??root;
  unavailable.set(root,{root,project:source,source:file,sha256:digest(file),reason:renderGenerationMessageV1("withheldDependency",{project:source,source:file,target:missing.join(", ")},"en").slice(0,2048),missing:[...missing]});
  delete projects[source];
 }
 return dangling.map(row=>row.source);
}

/** 📣️ Says, once per build, which projects the partial inference withheld and what repairs each of them. */
export function announceWithheldProjectsV1(host,part){for(const owner of part.owners)if(owner.unavailable)host.say(owner.unavailable.missing?"withheldDependency":"withheldSource",withheldFieldsV1(owner.unavailable));}

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

/** 🧱️ Splits the facts of the loaded implementation by their consumers: a data file nobody covers belongs to every project and the environment, the stamp of an inferrer of scope `all` or `environment` belongs to the environment, and the stamp of a project inferrer belongs to nobody here because every project record names exactly the stamps it took part in. */
export function scopeImplementationRowsV1(rows){
 const all=[],environment=[],seen=new Set();
 for(const{scope,...row}of rows){
  const key=rowKeyV1(row);if(seen.has(key))continue;seen.add(key);
  if(row.kind!=="stamp")all.push(row);else if(scope==="environment"||scope==="all")environment.push(row);
 }
 return{all,environment};
}

/** 🪪️ The digest that stamps a generation: the declared stamp of every inferrer and the bytes of every data file the implementation reads, never the bytes of code. */
export function implementationDigestV1(rows){
 return createHash("sha256").update(canonicalJsonV1(rows.map(row=>[row.kind,row.path,row.kind==="stamp"?row.fact:row.fact.sha256]).sort((a,b)=>compareCodePointsV1(a[1],b[1])||compareCodePointsV1(String(a[2]),String(b[2]))))).digest("hex");
}

/** 🪆️ The tasks an orchestrating target launches as nested runs (`metadata.semio.execution` of kind `orchestration`): the nested runs read the generation the build pins, so a slice needs their closure too. */
const orchestrationChildrenV1=operation=>{const execution=operation?.metadata?.semio?.execution;return execution?.kind==="orchestration"&&Array.isArray(execution.children)?execution.children.map(child=>({project:child.project,target:child.target})):[];};

/** 🪶️ The part of a fragment that {@link sliceClosureV1} reads: the targets of its edges, per target the input references and `projects` rows of its inputs, its `dependsOn` and its declared orchestration children, and the same of its named input groups. A closure of hundreds of projects keeps this instead of every parsed node while it is computed. */
export function sliceViewV1(fragment){
 const reads=inputs=>(Array.isArray(inputs)?inputs:[]).flatMap(input=>typeof input==="string"?input.startsWith("{")||input.startsWith("!")?[]:[input]:input&&typeof input==="object"&&(typeof input.input==="string"||input.projects!==undefined)?[Object.fromEntries(["input","projects","dependencies"].filter(key=>input[key]!==undefined).map(key=>[key,input[key]]))]:[]);
 const targets={},named=fragment.node?.namedInputs;
 for(const[name,operation]of Object.entries(fragment.node?.targets??{})){const children=orchestrationChildrenV1(operation);targets[name]={inputs:reads(operation?.inputs),dependsOn:operation?.dependsOn,...(children.length?{metadata:{semio:{execution:{kind:"orchestration",children}}}}:{})};}
 return{edges:fragment.edges.map(edge=>({target:edge.target})),node:named&&typeof named==="object"?{targets,namedInputs:Object.fromEntries(Object.entries(named).map(([group,inputs])=>[group,reads(inputs)]))}:{targets}};
}

/**
 * 🍰️ The owners a selected scope needs, as a fixpoint over the generation: every admitted owner admits the owners it imports (its dependency edges, transitively); only the active targets follow the owners their `inputs` name as `projects`, through their named input groups too (`group`, `^group` over the dependency edges, the group a `projects` row names on that owner), their `dependsOn` (`^target` over the dependency edges, `project:target`, `target`, objects with `projects` and `dependencies`, `*` for the active target only) and their declared orchestration children (a nested run is served from the generation the outer build pins, so its tasks belong to the slice). Catalog provider owners join when an active owner lies below one of their consumer roots.
 * `owners` maps every owner name to its root, `external` names the external nodes; `fragment(name)` yields `{edges, node}`. A task, a root or an edge to a name that is neither an owner nor an external node refuses with `GENERATION_UNKNOWN_OWNER`: a slice is never silently incomplete.
 */
export async function sliceClosureV1({owners,fragment,selection,roots=[],providers=[],external=[]}){
 const cache=new Map(),known=new Set(Object.keys(owners)),outside=new Set(external),admitted=new Set(),active=new Map(),queue=[];
 const get=async name=>{let row=cache.get(name);if(!row){row=await fragment(name);cache.set(name,row);}return row;};
 const unknown=name=>Object.assign(Error("Unknown project in the closure: "+name),{code:"GENERATION_UNKNOWN_OWNER"});
 const admit=async name=>{
  const todo=[name];
  while(todo.length){const at=todo.pop();if(admitted.has(at))continue;if(!known.has(at)){if(at===name||!outside.has(at))throw unknown(at);continue;}admitted.add(at);for(const edge of(await get(at)).edges)todo.push(edge.target);}
 };
 const task=async(name,target,optional)=>{
  if(!known.has(name))throw unknown(name);await admit(name);
  if(optional&&!(await get(name)).node?.targets?.[target])return;
  const set=active.get(name)??new Set();if(!set.has(target)){set.add(target);active.set(name,set);queue.push([name,target]);}
 };
 const groups=new Set(),imported=async name=>[...new Set((await get(name)).edges.map(edge=>edge.target))].filter(value=>known.has(value));
 const reach=async(name,inputs)=>{for(const input of Array.isArray(inputs)?inputs:[]){
  if(typeof input==="string"){const group=input.startsWith("^")?input.slice(1):input;for(const owner of input.startsWith("^")?await imported(name):[name]){const key=owner+"\0"+group,values=(await get(owner)).node?.namedInputs?.[group];if(!Array.isArray(values)||groups.has(key))continue;groups.add(key);await reach(owner,values);}}
  else if(input&&typeof input==="object"&&(typeof input.input==="string"||input.projects!==undefined))for(const owner of input.dependencies===true?await imported(name):input.projects===undefined?[name]:typeof input.projects==="string"?[input.projects]:Array.isArray(input.projects)?input.projects:[]){if(!known.has(owner))continue;await admit(owner);if(typeof input.input==="string")await reach(owner,[input.input]);}
 }};
 if(selection.kind==="tasks")for(const row of selection.tasks)await task(row.project,row.target,false);else{for(const name of roots)await admit(name);for(const name of roots)for(const target of selection.targets??[])await task(name,target,true);}
 while(queue.length){
  const[name,target]=queue.shift(),row=await get(name),operation=row.node?.targets?.[target];if(!operation)continue;
  const dependencies=[...new Set(row.edges.map(edge=>edge.target))].filter(value=>known.has(value));
  await reach(name,operation.inputs);
  for(const dependency of Array.isArray(operation.dependsOn)?operation.dependsOn:[]){
   if(typeof dependency==="string"){
    if(dependency.startsWith("^"))for(const value of dependencies)await task(value,dependency.slice(1),true);
    else if(dependency.includes(":")){const at=dependency.lastIndexOf(":");await task(dependency.slice(0,at),dependency.slice(at+1),false);}
    else await task(name,dependency,false);
   }else if(dependency&&typeof dependency==="object"&&typeof dependency.target==="string"){
    const projects=dependency.dependencies?dependencies:dependency.projects===undefined?[name]:dependency.projects==="*"?[...known]:typeof dependency.projects==="string"?[dependency.projects]:dependency.projects;
    for(const value of projects)await task(value,dependency.target,dependency.dependencies===true);
   }
  }
  for(const child of orchestrationChildrenV1(operation))await task(child.project,child.target,false);
 }
 const inside=(root,path)=>root==="."||path===root||path.startsWith(root+"/");
 for(const provider of providers){
  if(![...active.keys()].some(name=>provider.consumerRoots.some(root=>inside(root,owners[name]))))continue;
  let best;for(const[name,root]of Object.entries(owners))if(inside(root,provider.sourcePath)&&(best===undefined||root.length>owners[best].length))best=name;
  if(best!==undefined)await admit(best);
 }
 return admitted;
}

/** 🪦️ The previous owners a build still knows: an owner whose config was removed is dropped by the generation, so it must neither stay a kept stub of the inference (its edges would be refreshed for an owner that no longer exists) nor a member of the graph context. */
export function survivingStubsV1(stubs,removed){const gone=new Set(removed);return Object.fromEntries(Object.entries(stubs).filter(([name])=>!gone.has(name)));}

/** 🔢️ The edges of one owner in the order of their canonical text, so the fragment of an owner does not depend on the order in which its files were visited (a discovery of all owners and of some of them publish the same bytes). */
function orderedEdgesV1(edges){return edges.map(edge=>[canonicalJsonV1(edge),edge]).sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:0).map(row=>row[1]);}

/**
 * 🧩️ Shapes what one partial inference produced into the compute result of the store: every owner carries the facts of its own inference record plus the captured facts below its roots, edges go to their source owner, facts nobody consumed belong to the environment.
 * `workspaceRoot` makes row paths comparable (rows carry absolute paths); the root project never claims facts by path.
 */
export function shapeComputeResultV1({workspaceRoot,part,stubs,separator="/",implementation={all:[],environment:[]}}){
 const processed=new Set([...part.owners.map(owner=>owner.name),...part.edgeOnly.map(owner=>owner.name)]),edgesOf=new Map(),other=[];
 for(const edge of part.edges){if(processed.has(edge.source)){let list=edgesOf.get(edge.source);if(!list)edgesOf.set(edge.source,list=[]);list.push(edge);}else other.push(edge);}
 const prefix=root=>root==="."?undefined:workspaceRoot+separator+root.split("/").join(separator)+separator,prefixes=roots=>roots.filter(Boolean).map(prefix).filter(Boolean);
 const claimed=new Set(),owners=[],ordered=part.rows.map((row,at)=>[row.path,at]).sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:a[1]-b[1]);
 const under=entry=>{let low=0,high=ordered.length;while(low<high){const middle=(low+high)>>>1;if(ordered[middle][0]<entry)low=middle+1;else high=middle;}const found=[];for(;low<ordered.length&&ordered[low][0].startsWith(entry);low++)found.push(ordered[low][1]);return found;};
 const claimedRows=new WeakSet(),claimedLists=new WeakSet(),claim=(base,roots)=>{
  const list=prefixes(roots),found=list.length?[...new Set(list.flatMap(under))].sort((a,b)=>a-b):[];let rows=base;
  if(found.length){const wanted=new Map();for(const at of found){const row=part.rows[at],key=rowKeyV1(row);if(!wanted.has(key))wanted.set(key,row);}for(const row of base){if(!wanted.size)break;wanted.delete(rowKeyV1(row));}if(wanted.size){rows=[...base];for(const row of wanted.values())rows.push(row);}}
  if(!claimedLists.has(rows)){claimedLists.add(rows);for(const row of rows)if(!claimedRows.has(row)){claimedRows.add(row);claimed.add(rowKeyV1(row));}}return rows;
 };
 for(const owner of part.owners)owners.push({name:owner.name,root:owner.root,kind:owner.unavailable?"unavailable":"project",node:owner.unavailable??owner.node,edges:orderedEdgesV1(edgesOf.get(owner.name)??[]),targets:owner.unavailable?[]:Object.keys(owner.node?.targets??{}),...(owner.config?{config:owner.config}:{}),...(owner.nativeRoot?{nativeRoot:owner.nativeRoot}:{}),...(owner.tag?{tag:owner.tag}:{}),rows:[...claim(owner.nodeRows??[],[owner.root,owner.nativeRoot]),...implementation.all]});
 for(const owner of part.edgeOnly){const stub=stubs[owner.name];owners.push({name:owner.name,partial:"edges",...(owner.streamed?{}:{edges:orderedEdgesV1(edgesOf.get(owner.name)??[])}),rows:claim([],[stub.root,stub.nativeRoot])});}
 const environment=[...part.env.rows,...part.rows.filter(row=>!claimed.has(rowKeyV1(row))),...implementation.all,...implementation.environment];
 return{owners,...(part.reEdge?{reEdge:async(name,fragment)=>orderedEdgesV1(await part.reEdge(name,fragment))}:{}),...(part.derived?.length?{derived:part.derived}:{}),env:{digest:part.env.digest,rows:environment},global:{version:1,externalNodes:part.externalNodes??{},edges:other,unavailable:part.parseUnavailable??[]},replaceTags:part.replaceTags??[]};
}

/** 🔍️ Names the exact input behind one stale reason of a plan: the stamp of an inferrer, an input file, a directory listing, a declared data file, a source analysis or a workspace file that appeared, vanished or changed. `current` maps stamp paths to the stamps this process carries. */
export function describeGenerationReasonV1(reason,{workspaceRoot,separator="/",current=new Map(),language}){
 const display=path=>path.startsWith(workspaceRoot+separator)?path.slice(workspaceRoot.length+separator.length).split(separator).join("/"):path;
 const project=reason.owners?.[0]??"@env";let key,fields;
 if(reason.change){key="reasonMembership";fields={path:display(reason.change.path),change:renderGenerationMessageV1({added:"changeAdded",removed:"changeRemoved"}[reason.change.kind]??"changeChanged",{},language)};}
 else{
  const{kind,path}=reason.row;
  if(kind==="stamp"){key="reasonStamp";fields={inferrer:path.replace(/^inferrer:/u,""),recorded:String(reason.row.fact),current:current.get(path)??"?"};}
  else if(kind==="file"){key="reasonInput";fields={path:display(path)};}
  else if(kind==="digest"||kind==="field"){key="reasonData";fields={path:display(path)};}
  else if(kind==="readdir"||kind==="readdirNames"||kind==="directories"){key="reasonListing";fields={path:display(path)};}
  else if(kind==="analysis"){key="reasonAnalysis";fields={path:display(path)};}
  else if(kind==="derived"){key="reasonDerived";let first=path;try{first=JSON.parse(path).roots?.[0]??path;}catch{}fields={path:display(String(first)).slice(0,160)};}
  else{key="reasonFact";fields={kind,path:display(path)};}
 }
 const why=renderGenerationMessageV1(key,fields,language);
 return{project,key,fields,why,text:renderGenerationMessageV1("reason",{project,why},language)};
}

/** 📢️ Says why a build starts: the reason of a cold start, or the number of stale projects followed by the first stale inputs by name. */
export function announceGenerationBuildV1(host,spec,rows){
 if(spec.cold){host.say("cold",{reason:spec.coldReason??"empty"});return;}
 host.say("building",{dirty:spec.owners.length,generation:spec.generation??0});
 if(spec.staleKinds&&(spec.staleRows??0)>0)host.say("summary",{rows:spec.staleRows,kinds:Object.entries(spec.staleKinds).sort((x,y)=>y[1]-x[1]||(x[0]<y[0]?-1:1)).map(([kind,count])=>kind+" "+count).join(", "),projects:spec.owners.length});
 const current=new Map(rows.filter(row=>row.kind==="stamp").map(row=>[row.path,row.fact])),language=host.language;
 for(const reason of spec.reasons??[]){const row=describeGenerationReasonV1(reason,{workspaceRoot:host.workspaceRoot,separator:host.separator,current,language});host.say("reason",{project:row.project,why:row.why});}
 const listed=(spec.reasons??[]).filter(reason=>reason.row).length;
 if((spec.staleRows??0)>listed)host.say("reasonMore",{count:spec.staleRows-listed});
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

const needsBuild=async(_body,pending)=>({built:false,published:false,needsBuild:true,computed:0,pending});

/** 📦️ Expands fixed project/UTF8-byte groups once and preserves every caught consumer failure. */
export async function expandInChunksV1(projects,expand,{maximumProjects,maximumBytes}){
 const names=Object.keys(projects),result={};
 const pass=async group=>{
  const chunk={};for(const name of group)chunk[name]=projects[name];
  Object.assign(result,await expand(chunk));
 };
 let group=[],bytes=0;
 for(const name of names){const size=Buffer.byteLength(JSON.stringify(projects[name]));if(group.length&&(group.length>=maximumProjects||bytes+size>maximumBytes)){await pass(group);group=[];bytes=0;}group.push(name);bytes+=size;}
 if(group.length)await pass(group);
 return result;
}

/** 🌊️ The streaming twin of {@link expandInChunksV1}: the same fixed project and UTF8 byte groups, formed from `[name, node]` pairs as they arrive, so the raw nodes of a group are released as soon as the group is expanded and only the expanded projects accumulate. */
export async function expandStreamV1(entries,expand,{maximumProjects,maximumBytes}){
 const result={};
 const pass=async group=>{Object.assign(result,await expand(Object.fromEntries(group)));};
 let group=[],bytes=0;
 for await(const entry of entries){const size=Buffer.byteLength(JSON.stringify(entry[1]));if(group.length&&(group.length>=maximumProjects||bytes+size>maximumBytes)){await pass(group);group=[];bytes=0;}group.push(entry);bytes+=size;}
 if(group.length)await pass(group);
 return result;
}

/** 🕰️ Loads the all-workspace graph (a command whose front door offered a yield ticket and that only waits for a peer's build ends instead of waiting: see `../🛌️yield`), the closure of some projects, or the slice of a selected scope (tasks or literal owners) from the newest sealed generation; `lean` answers only project and target names. A shared reader verifies and serves it without ever taking a writer; only a stale or missing generation enters the builder custody, where a holder that finds the work done by a peer serves that generation.
 * `host` binds the repository: `release()` (optional: drops the in-process caches a construction filled, called once the graph is assembled so a process that goes on to run for hours keeps only the graph), `identity()`, `implementationRows()`, `withShared(body)`, `withBuilder(body)`, `storeOptions(store, control)`, `membership(previous, signal)`, `discoverPart(spec, signal)`, `expand(projects, control)`, `expansionLimits`, `readGraph(value)`, `readLean(value)`, `clock(control)`, `say(key, fields)` and the constants `workspaceRoot`, `namedInputs`, `registryDigest`, `signal`.
 */
export async function loadGenerationGraphV1(host,{policy="fresh",closure,selection,slice,lean,pin}={}){
 const requestedAt=graphQueueRequestedAtV1()??Date.now(),identity=await host.identity(),roots=slice?[...new Set(slice.roots)]:closure?[...new Set(closure)]:undefined,rows=pin===undefined?await host.implementationRows():[],implementation=implementationDigestV1(rows),scoped=scopeImplementationRowsV1(rows),limits=host.expansionLimits??{maximumProjects:32,maximumBytes:16777216};
 let pointer;
 const attempt=async(builder,store,control)=>{
  const options=host.storeOptions(store,control),engine=createGenerationStoreV1(options),progress=createGenerationProgressV1(host.clock(control));
  const compute=builder?async(spec,signal)=>{announceGenerationBuildV1(host,spec,rows);const part=await host.discoverPart(spec,signal,control);announceWithheldProjectsV1(host,part);await host.release?.();return shapeComputeResultV1({workspaceRoot:host.workspaceRoot,separator:host.separator,part,stubs:spec.stubs,implementation:scoped});}:async()=>{throw Error("A shared graph reader never computes");};
  pointer=join(options.root,GENERATION_POLICY_V1.pointer);
  const base={identity,implementation,policy:builder?"fresh":policy,membership:host.membership,compute,signal:host.signal,progress,requestedAt,builder:builder?body=>body(builder===true?undefined:builder):needsBuild};
  const closeFn=slice?async(current)=>{const global=await engine.readGlobal(current.manifest);return sliceClosureV1({owners:Object.fromEntries(Object.entries(current.manifest.owners).map(([name,row])=>[name,row.root])),external:Object.keys(global.externalNodes??{}),fragment:name=>engine.readFragment(current.manifest.owners[name]).then(value=>{if(!value)throw Object.assign(Error("Fragment of "+name+" is missing"),{code:"GENERATION_INCOMPLETE"});return sliceViewV1(value);}),selection:slice.selection,roots:slice.roots,providers:slice.providers??[]});}:undefined;
  const started=Date.now(),result=pin!==undefined?await readPinnedGenerationV1({engine,pin,identity,roots,closure:closeFn,signal:host.signal}):roots?await engine.refreshClosure({...base,roots,closure:closeFn}):await engine.refresh(base);
  engine.dropIndexes();if(typeof builder==="object")await builder.release();
  if(result.needsBuild)return{needsBuild:true,pending:result.pending,generation:(await engine.read(host.signal))?.pointer.generation??-1};
  const generation=result.current.manifest.generation;
  if(result.served==="current")host.say(result.current.manifest.implementation===implementation?"current":"stale",{generation});
  else if(result.served==="covered")host.say("covered",{generation});
  else if(result.served==="pinned")host.say("pinned",{generation,count:result.names?result.names.size:Object.keys(result.current.manifest.owners).length});
  else if(result.served==="built"&&result.published){host.say(result.merged===undefined?"published":"merged",{generation,computed:result.computed,seconds:Math.round((Date.now()-started)/1000),theirs:result.merged??0});if(result.index)host.say("indexed",{generation,rows:result.index.rows,owners:result.index.owners,expanded:(result.index.expandedBytes/1048576).toFixed(1),percent:(100*result.index.expandedBytes/GENERATION_POLICY_V1.maximumIndexBytes).toFixed(1),limit:GENERATION_POLICY_V1.maximumIndexBytes/1048576,packed:(result.index.packedBytes/1048576).toFixed(1)});}
  else if(host.verbose||lean)host.say(roots?"closure":"verified",roots?{count:result.names.size}:{generation,checked:result.plan?.checked??0});
  if(result.plan?.cutoff?.questions)host.say("cutoff",{questions:result.plan.cutoff.questions,rows:result.plan.stale??0,held:result.plan.cutoff.held,changed:result.plan.cutoff.changed});
  if(lean){const listing=await engine.catalog(result.current,{signal:host.signal});return{graph:host.readLean({version:1,lean:true,projects:Object.fromEntries(listing.projects.map(row=>[row.name,{name:row.name,root:row.root,targets:Object.fromEntries(row.targets.map(target=>[target,{}]))}])),dependencies:[],...(listing.unavailable.length?{unavailable:listing.unavailable}:{})}),result};}
  const stream=engine.assembleStream(result.current,{names:result.names&&[...result.names],compare:(a,b)=>a.localeCompare(b),signal:host.signal,progress});
  const projects=await expandStreamV1(stream.nodes(),chunk=>host.expand(chunk,control),limits),assembled=await stream.finish(),closureDigest=selection?createHash("sha256").update(canonicalJsonV1([assembled.membership,[...result.names].sort()])).digest("hex"):assembled.membership;
  const graph=host.readGraph({version:1,projects,dependencies:assembled.dependencies,namedInputs:host.namedInputs,externalNodes:assembled.externalNodes,...(assembled.unavailable.length?{unavailable:assembled.unavailable}:{}),admission:{selection:selection??{kind:"all"},registry:host.registryDigest,closure:closureDigest}});
  if(host.sealed){const manifest=result.current.manifest,owners=Object.keys(manifest.owners).sort(compareCodePointsV1);host.sealed({generation:manifest.generation,manifest:result.current.pointer.manifest,owners,covered:pin!==undefined?decodeVerifiedScopeV1(owners,pin.covered):result.names,atMs:pin!==undefined?pin.covered.atMs:requestedAt});}
  return{graph,result};
 };
 const first=await host.withShared((store,control)=>attempt(false,store,control));
 if(!first.needsBuild){await host.release?.();return first.graph;}
 const queued=Date.now();
 const request=first.pending?.dirty.length?{generation:first.pending.generation,dirty:first.pending.dirty,roots:first.pending.roots,structural:first.pending.structural,implementation}:undefined;
 if(host.prefetch&&request&&!await host.builderIdle?.(request)){host.carryClosures?.(true);try{await host.prefetch(request);}finally{host.carryClosures?.(false);}}
 let held=false;
 const notice=setInterval(()=>{if(held)return;host.say("waiting",{seconds:Math.round((Date.now()-queued)/1000),generation:first.generation});if(pointer!==undefined&&graphQueueYieldsV1({queuedMs:Date.now()-queued,yieldAfterMilliseconds:host.yieldAfterMilliseconds}))yieldGraphQueueV1({generation:first.generation,pointer,requestedAt,queuedMs:Date.now()-queued,exit:host.exitProcess});},host.waitNoticeMilliseconds??GENERATION_POLICY_V1.waitNoticeMilliseconds);notice.unref?.();
 let built,want=first.pending;try{for(let round=1;;round++){built=await host.withBuilder((store,control,claim)=>{held=true;return attempt(claim??true,store,control);},want);if(!built.needsBuild)break;held=false;want={...built.pending,structural:built.pending.structural||round>=CUSTODY_POLICY_V1.widenRounds};}}finally{clearInterval(notice);}
 await host.release?.();
 return built.graph;
}
