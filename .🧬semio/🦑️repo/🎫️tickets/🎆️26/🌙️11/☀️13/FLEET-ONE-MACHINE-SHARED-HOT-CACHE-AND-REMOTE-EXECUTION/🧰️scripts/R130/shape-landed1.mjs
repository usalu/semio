/** 🗃️ Whether rows are a row log list (counted, keyed, read in batches) and not an array of row objects. */
const lazyRowsV1=rows=>!Array.isArray(rows)&&typeof rows?.concat==="function"&&typeof rows.keys==="function";

/** 🧩️ Shapes what one partial inference produced into the compute result of the store: every owner carries the facts of its own inference record plus the captured facts below its roots, edges go to their source owner, facts nobody consumed belong to the environment.
 * `workspaceRoot` makes row paths comparable (rows carry absolute paths); the root project never claims facts by path. A part whose `rows` is a row log list is claimed in one streamed pass over the log (ids only, a batch of rows at a time); a part with an array of rows is claimed over the sorted array.
 */
export function shapeComputeResultV1({workspaceRoot,part,stubs,separator="/",implementation={all:[],environment:[]}}){
 const processed=new Set([...part.owners.map(owner=>owner.name),...part.edgeOnly.map(owner=>owner.name)]),edgesOf=new Map(),other=[];
 for(const edge of part.edges){if(processed.has(edge.source)){let list=edgesOf.get(edge.source);if(!list)edgesOf.set(edge.source,list=[]);list.push(edge);}else other.push(edge);}
 const prefix=root=>root==="."?undefined:workspaceRoot+separator+root.split("/").join(separator)+separator,prefixes=roots=>roots.filter(Boolean).map(prefix).filter(Boolean);
 const slots=[],claim=(base,roots)=>slots.push({base,prefixes:prefixes(roots)})-1;
 const joined=(rows,extra)=>lazyRowsV1(rows)?rows.concat(extra):[...rows,...extra];
 const owners=[],slotOf=[];
 for(const owner of part.owners){slotOf.push(claim(owner.nodeRows??[],[owner.root,owner.nativeRoot]));owners.push({name:owner.name,root:owner.root,kind:owner.unavailable?"unavailable":"project",node:owner.unavailable??owner.node,edges:orderedEdgesV1(edgesOf.get(owner.name)??[]),targets:owner.unavailable?[]:Object.keys(owner.node?.targets??{}),...(owner.config?{config:owner.config}:{}),...(owner.nativeRoot?{nativeRoot:owner.nativeRoot}:{}),...(owner.tag?{tag:owner.tag}:{}),...(owner.reads?{reads:owner.reads}:{})});}
 const edgeOnly=[];
 for(const owner of part.edgeOnly){const stub=stubs[owner.name];edgeOnly.push({name:owner.name,slot:claim([],[stub.root,stub.nativeRoot]),...(owner.streamed?{}:{edges:orderedEdgesV1(edgesOf.get(owner.name)??[]),...(part.readsOf?.(owner.name)?{reads:part.readsOf(owner.name)}:{})})});}
 const claims=lazyRowsV1(part.rows)&&part.rows.log!==undefined?claimStreamedV1({all:part.rows,slots,workspaceRoot,separator}):claimListedV1({all:part.rows,slots}),all=implementation.all;
 owners.forEach((owner,at)=>{owner.rows=joined(claims.rows[slotOf[at]],all);});
 for(const owner of edgeOnly){const{name,slot,...rest}=owner;owners.push({name,partial:"edges",...rest,rows:claims.rows[slot]});}
 const environment=claims.environment(part.env.rows,[...implementation.all,...implementation.environment]);
 return{owners,...(part.carried?.length?{carried:part.carried}:{}),...(part.readsOf?{readsOf:part.readsOf}:{}),...(part.reEdge?{reEdge:async(name,fragment)=>orderedEdgesV1(await part.reEdge(name,fragment))}:{}),...(part.derived?.length?{derived:part.derived}:{}),env:{digest:part.env.digest,rows:environment,...(part.env.parts?{parts:part.env.parts}:{})},global:{version:1,externalNodes:part.externalNodes??{},edges:other,unavailable:part.parseUnavailable??[]},replaceTags:part.replaceTags??[],...(Number.isSafeInteger(part.inferred)?{inferred:part.inferred}:{}),...(part.phases?{phases:part.phases}:{}),...(part.dispose?{dispose:part.dispose}:{})};
}

/** 📚️ Claims over an array of rows: the rows below the roots of each slot come from a path-sorted copy of the array; what no slot holds is the environment's. */
function claimListedV1({all,slots}){
 const claimed=new Set(),ordered=all.map((row,at)=>[row.path,at]).sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:a[1]-b[1]);
 const under=entry=>{let low=0,high=ordered.length;while(low<high){const middle=(low+high)>>>1;if(ordered[middle][0]<entry)low=middle+1;else high=middle;}const found=[];for(;low<ordered.length&&ordered[low][0].startsWith(entry);low++)found.push(ordered[low][1]);return found;};
 const keysOf=rows=>lazyRowsV1(rows)?rows.keys():(function*(){for(const row of rows)yield rowKeyV1(row);})(),joined=(rows,extra)=>lazyRowsV1(rows)?rows.concat(extra):[...rows,...extra];
 const claimedLists=new WeakSet(),claim=({base,prefixes:list})=>{
  const found=list.length?[...new Set(list.flatMap(under))].sort((a,b)=>a-b):[];let rows=base;
  if(found.length){const wanted=new Map();for(const at of found){const row=all[at],key=rowKeyV1(row);if(!wanted.has(key))wanted.set(key,row);}for(const key of keysOf(base)){if(!wanted.size)break;wanted.delete(key);}if(wanted.size)rows=joined(base,[...wanted.values()]);}
  if(!claimedLists.has(rows)){claimedLists.add(rows);for(const key of keysOf(rows))claimed.add(key);}return rows;
 };
 const rows=slots.map(claim);
 return{rows,environment:(own,extra)=>[...own,...all.filter(row=>!claimed.has(rowKeyV1(row))),...extra]};
}

/** 🌊️ Claims over a row log list in one pass: every row of the log is read once in batches, the slots whose roots lie above its path are found by its ancestor directories, and only row numbers are kept (a bitmap of the claimed numbers, the numbers a slot holds beyond its own list). Memory is a few bytes per row whatever the number of owners. */
function claimStreamedV1({all,slots,workspaceRoot,separator}){
 const log=all.log,start=workspaceRoot+separator,batch=2048;
 let bits=new Uint8Array((log.size()>>>3)+8);
 const mark=id=>{const byte=id>>>3;if(byte>=bits.length){const next=new Uint8Array(Math.max(byte+8,bits.length*2));next.set(bits);bits=next;}bits[byte]|=1<<(id&7);},marked=id=>(id>>>3)<bits.length&&(bits[id>>>3]&(1<<(id&7)))!==0;
 const idsOf=base=>lazyRowsV1(base)&&base.log===log&&!base.tails.length?base.ids:Uint32Array.from(lazyRowsV1(base)?base.rows():base,row=>log.id(row));
 const known=new Map(),sortedOf=base=>{let ids=known.get(base);if(ids===undefined){ids=base.length?Uint32Array.from(idsOf(base)).sort():new Uint32Array(0);known.set(base,ids);}return ids;};
 const has=(ids,id)=>{let low=0,high=ids.length;while(low<high){const middle=(low+high)>>>1;if(ids[middle]<id)low=middle+1;else high=middle;}return low<ids.length&&ids[low]===id;};
 const byPrefix=new Map(),sorted=slots.map(slot=>sortedOf(slot.base)),extras=slots.map(()=>[]);
 slots.forEach((slot,at)=>{for(const prefix of slot.prefixes){const list=byPrefix.get(prefix);if(list===undefined)byPrefix.set(prefix,[at]);else if(list[list.length-1]!==at)list.push(at);}});
 const visit=(id,row)=>{
  const path=row.path;if(!byPrefix.size||typeof path!=="string"||!path.startsWith(start))return;
  for(let at=start.length;;){const next=path.indexOf(separator,at);if(next<0)return;const hit=byPrefix.get(path.slice(0,next+1));if(hit!==undefined)for(const slot of hit)if(!has(sorted[slot],id)){const list=extras[slot];if(list[list.length-1]!==id)list.push(id);}at=next+1;}
 };
 const ids=all.ids;
 for(let at=0;at<ids.length;at+=batch){const slice=ids.subarray(at,Math.min(ids.length,at+batch)),rows=log.read(slice);for(let item=0;item<slice.length;item++)visit(slice[item],rows[item]);}
 const tailIds=[];for(const tail of all.tails)for(const row of tail){const id=log.id(row);tailIds.push(id);visit(id,row);}
 const marked0=new Set();for(const slot of slots){if(marked0.has(slot.base))continue;marked0.add(slot.base);for(const id of sortedOf(slot.base))mark(id);}
 for(const list of extras)for(const id of list)mark(id);
 const rows=slots.map((slot,at)=>extras[at].length===0?slot.base:log.list([...idsOf(slot.base),...extras[at]]));
 return{rows,environment:(own,extra)=>{
  const seen=new Set(),ids=[],add=id=>{if(!seen.has(id)){seen.add(id);ids.push(id);}};
  for(const row of own)add(log.id(row));
  for(const id of all.ids)if(!marked(id))add(id);
  for(const id of tailIds)if(!marked(id))add(id);
  return log.list(ids).concat(extra);
 }};
}

