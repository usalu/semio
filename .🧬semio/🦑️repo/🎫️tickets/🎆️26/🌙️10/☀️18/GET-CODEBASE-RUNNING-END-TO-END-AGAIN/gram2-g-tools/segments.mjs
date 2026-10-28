// [gram2-g] prototype of the segmented fact index: owners hold lists of shared row arrays; the result equals createFactIndexV1 of the expanded lists. exported: createSegmentedFactIndexV1(ownerSegments, {rowKey, compare})
/** ownerSegments: Map<name, Row[][]> (arrays shared by reference between owners) */
export function createSegmentedFactIndexV1(ownerSegments,{rowKey,compare}){
 const owners=[...ownerSegments.keys()].sort(compare),rows=[],byKey=new Map(),byObject=new Map(),empty=[];
 const arrayId=new Map(),arrays=[],arrayIds=[],arrayOwners=[];   // distinct arrays: object -> index, row ids (unique, first-appearance order), owner ids (ascending)
 const idsOf=array=>{let at=arrayId.get(array);if(at!==undefined)return at;at=arrays.length;arrayId.set(array,at);arrays.push(array);
  const ids=[],seen=new Set();
  for(const row of array){let id=byObject.get(row);if(id===undefined){const key=rowKey(row);id=byKey.get(key);if(id===undefined){id=rows.length;byKey.set(key,id);rows.push(row);}byObject.set(row,id);}if(!seen.has(id)){seen.add(id);ids.push(id);}}
  arrayIds.push(Uint32Array.from(ids));arrayOwners.push([]);return at;};
 const ownerArrays=[];
 owners.forEach((name,owner)=>{const mine=[],seenArray=new Set();let any=false;
  for(const array of ownerSegments.get(name)){const at=idsOf(array);if(arrayIds[at].length)any=true;if(!seenArray.has(at)){seenArray.add(at);mine.push(at);arrayOwners[at].push(owner);}}
  if(!any)empty.push({r:[],o:[owner]});ownerArrays.push(mine);});
 // row -> arrays containing it
 const first=new Uint32Array(rows.length+1);for(const ids of arrayIds)for(const id of ids)first[id+1]++;
 for(let id=0;id<rows.length;id++)first[id+1]+=first[id];
 const holders=new Uint32Array(first[rows.length]),fill=first.slice(0,rows.length);
 arrayIds.forEach((ids,at)=>{for(const id of ids)holders[fill[id]++]=at;});
 // pre-classes by the set of arrays; then merge by owner set
 const pre=new Map();
 for(let id=0;id<rows.length;id++){const key=holders.subarray(first[id],first[id+1]).join(",");let group=pre.get(key);if(!group){group={arrays:Array.from(holders.subarray(first[id],first[id+1])),r:[]};pre.set(key,group);}group.r.push(id);}
 const merged=new Map(),classes=[];
 for(const group of pre.values()){
  let o;if(group.arrays.length===1)o=arrayOwners[group.arrays[0]];else{const set=new Set();for(const at of group.arrays)for(const owner of arrayOwners[at])set.add(owner);o=[...set].sort((a,b)=>a-b);}
  const key=o.join(","),known=merged.get(key);
  if(known){known.r.push(...group.r);}else{const cls={r:group.r.slice(),o:o.slice()};merged.set(key,cls);classes.push(cls);}
 }
 for(const cls of classes)cls.r.sort((a,b)=>a-b);
 classes.sort((a,b)=>a.r[0]-b.r[0]);
 return{version:1,owners,rows,classes:[...classes,...empty]};
}
