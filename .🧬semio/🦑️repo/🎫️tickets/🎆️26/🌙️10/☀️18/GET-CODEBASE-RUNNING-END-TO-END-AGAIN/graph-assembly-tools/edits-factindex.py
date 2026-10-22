import sys,os
sys.path.insert(0,os.path.join(os.path.dirname(__file__),"..","graph-receipt-tools"))
from anchored import apply
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
old='''export function createFactIndexV1(ownerRows){
 const owners=[...ownerRows.keys()].sort(compareCodePointsV1),rows=[],ids=new Map(),consumers=[];
 owners.forEach((name,owner)=>{
  const seen=new Set();
  for(const row of ownerRows.get(name)){
   const key=rowKeyV1(row);let id=ids.get(key);if(id===undefined){id=rows.length;ids.set(key,id);rows.push(row);consumers.push([]);}
   if(!seen.has(id)){seen.add(id);consumers[id].push(owner);}
  }
 });
 const groups=new Map(),empty=[];
 consumers.forEach((list,id)=>{const key=list.join(",");let group=groups.get(key);if(!group){group={r:[],o:list};groups.set(key,group);}group.r.push(id);});
 owners.forEach((name,owner)=>{if(!ownerRows.get(name).length)empty.push({r:[],o:[owner]});});
 return{version:1,owners,rows,classes:[...groups.values(),...empty]};
}'''
new='''export function createFactIndexV1(ownerRows){
 const owners=[...ownerRows.keys()].sort(compareCodePointsV1),rows=[],byKey=new Map(),byObject=new Map(),lists=[],empty=[];
 owners.forEach((name,owner)=>{
  const list=ownerRows.get(name),mine=new Uint32Array(list.length),mark=new Set();let count=0;
  for(const row of list){
   let id=byObject.get(row);
   if(id===undefined){const key=rowKeyV1(row);id=byKey.get(key);if(id===undefined){id=rows.length;byKey.set(key,id);rows.push(row);}byObject.set(row,id);}
   if(!mark.has(id)){mark.add(id);mine[count++]=id;}
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
}'''
apply(L+"/🏗️build/🕸️graph/🕰️generations/🟨️.mjs",[(old,new)])
