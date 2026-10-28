// @ts-nocheck
/** 🕰️ The fact index builder as it was before the chunked one (expanding every (owner, row) pair): the independent oracle the chunked builder must equal byte for byte. */
type Row={kind:string;path:string;fact:unknown};
export function legacyCreateFactIndexV1(ownerRows:ReadonlyMap<string,readonly Row[]>,{rowKeyV1,compareCodePointsV1}:{rowKeyV1:(row:Row)=>string;compareCodePointsV1:(a:string,b:string)=>number}){
 const owners=[...ownerRows.keys()].sort(compareCodePointsV1),rows=[],byKey=new Map(),byObject=new Map(),lists=[],empty=[];let seen=new Int32Array(1<<16);
 owners.forEach((name,owner)=>{
  const list=ownerRows.get(name),mine=new Uint32Array(list.length);let count=0;
  for(const row of list){
   let id=byObject.get(row);
   if(id===undefined){const key=rowKeyV1(row);id=byKey.get(key);if(id===undefined){id=rows.length;byKey.set(key,id);rows.push(row);}byObject.set(row,id);}
   if(id>=seen.length){const grown=new Int32Array(Math.max(seen.length*2,id+1));grown.set(seen);seen=grown;}
   if(seen[id]!==owner+1){seen[id]=owner+1;mine[count++]=id;}
  }
  if(!count)empty.push({r:[],o:[owner]});
  lists.push(mine.subarray(0,count));
 });
 const first=new Uint32Array(rows.length+1);
 for(const list of lists)for(const id of list)first[id+1]++;
 for(let id=0;id<rows.length;id++)first[id+1]+=first[id];
 const flat=new Uint32Array(first[rows.length]),fill=first.slice(0,rows.length);
 lists.forEach((list,owner)=>{for(const id of list)flat[fill[id]++]=owner;});
 const buckets=new Map(),classes=[];
 for(let id=0;id<rows.length;id++){
  const from=first[id],to=first[id+1];let hash=to-from;
  for(let at=from;at<to;at++)hash=(Math.imul(hash,0x9e3779b1)^flat[at])>>>0;
  let chain=buckets.get(hash),group;
  if(chain===undefined)buckets.set(hash,chain=[]);
  for(const candidate of chain){
   const other=candidate.span;if(other[1]-other[0]!==to-from)continue;
   let same=true;for(let at=0;at<to-from;at++)if(flat[other[0]+at]!==flat[from+at]){same=false;break;}
   if(same){group=candidate;break;}
  }
  if(group===undefined){group={r:[],o:Array.from(flat.subarray(from,to)),span:[from,to]};chain.push(group);classes.push(group);}
  group.r.push(id);
 }
 return{version:1,owners,rows,classes:[...classes.map(({r,o})=>({r,o})),...empty]};
}
