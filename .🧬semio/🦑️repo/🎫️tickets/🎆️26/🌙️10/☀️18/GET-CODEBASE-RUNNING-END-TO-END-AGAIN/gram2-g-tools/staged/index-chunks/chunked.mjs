// [gram2-g] chunked fact index: same output as createFactIndexV1, without expanding the (owner, row) pairs. Standalone form for the oracle tests; the batch pastes `createFactIndexChunkedBodyV1` into the generations module.
export function createFactIndexChunkedV1(ownerRows,{rowKeyV1,compareCodePointsV1}){
 const owners=[...ownerRows.keys()].sort(compareCodePointsV1),rows=[],byKey=new Map(),byObject=new Map(),empty=[],chunkIndex=new Map(),chunks=[],chunkOwners=[];let seen=new Int32Array(1<<16),scratch=new Uint32Array(1<<12);
 const intern=(from,to,owner)=>{
  const length=to-from;let hash=length;for(let at=from;at<to;at++)hash=(Math.imul(hash,0x9e3779b1)^scratch[at])>>>0;
  const bucket=chunkIndex.get(hash);let found=-1;
  if(bucket!==undefined)for(const candidate of typeof bucket==="number"?[bucket]:bucket){const ids=chunks[candidate];if(ids.length!==length)continue;let same=true;for(let at=0;at<length;at++)if(ids[at]!==scratch[from+at]){same=false;break;}if(same){found=candidate;break;}}
  if(found<0){found=chunks.length;chunks.push(scratch.slice(from,to));chunkOwners.push([]);if(bucket===undefined)chunkIndex.set(hash,found);else if(typeof bucket==="number")chunkIndex.set(hash,[bucket,found]);else bucket.push(found);}
  chunkOwners[found].push(owner);
 };
 owners.forEach((name,owner)=>{
  const list=ownerRows.get(name);if(scratch.length<list.length)scratch=new Uint32Array(Math.max(list.length,scratch.length*2));
  let count=0;
  for(const row of list){
   let id=byObject.get(row);
   if(id===undefined){const key=rowKeyV1(row);id=byKey.get(key);if(id===undefined){id=rows.length;byKey.set(key,id);rows.push(row);}byObject.set(row,id);}
   if(id>=seen.length){const grown=new Int32Array(Math.max(seen.length*2,id+1));grown.set(seen);seen=grown;}
   if(seen[id]!==owner+1){seen[id]=owner+1;scratch[count++]=id;}
  }
  if(!count){empty.push({r:[],o:[owner]});return;}
  let from=0;for(let at=0;at<count;at++)if(at===count-1||Math.imul(scratch[at],0x9e3779b1)>>>24===0){intern(from,at+1,owner);from=at+1;}
 });
 const first=new Uint32Array(rows.length+1);
 for(const ids of chunks)for(const id of ids)first[id+1]++;
 for(let id=0;id<rows.length;id++)first[id+1]+=first[id];
 const holders=new Uint32Array(first[rows.length]),fill=first.slice(0,rows.length);
 chunks.forEach((ids,chunk)=>{for(const id of ids)holders[fill[id]++]=chunk;});
 const pre=new Map(),groups=[];
 for(let id=0;id<rows.length;id++){
  const from=first[id],to=first[id+1];let hash=to-from;for(let at=from;at<to;at++)hash=(Math.imul(hash,0x9e3779b1)^holders[at])>>>0;
  let chain=pre.get(hash),group;if(chain===undefined)pre.set(hash,chain=[]);
  for(const candidate of chain){const other=candidate.span;if(other[1]-other[0]!==to-from)continue;let same=true;for(let at=0;at<to-from;at++)if(holders[other[0]+at]!==holders[from+at]){same=false;break;}if(same){group=candidate;break;}}
  if(group===undefined){group={r:[],span:[from,to]};chain.push(group);groups.push(group);}
  group.r.push(id);
 }
 const merged=new Map(),classes=[];
 for(const group of groups){
  const [from,to]=group.span;let o;
  if(to-from===1)o=chunkOwners[holders[from]];else{const set=new Set();for(let at=from;at<to;at++)for(const owner of chunkOwners[holders[at]])set.add(owner);o=[...set].sort((a,b)=>a-b);}
  let hash=o.length;for(const owner of o)hash=(Math.imul(hash,0x9e3779b1)^owner)>>>0;
  let chain=merged.get(hash),cls;if(chain===undefined)merged.set(hash,chain=[]);
  for(const candidate of chain){const other=candidate.o;if(other.length!==o.length)continue;let same=true;for(let at=0;at<o.length;at++)if(other[at]!==o[at]){same=false;break;}if(same){cls=candidate;break;}}
  if(cls===undefined){cls={r:group.r,o:o.slice()};chain.push(cls);classes.push(cls);}else for(const id of group.r)cls.r.push(id);
 }
 for(const cls of classes)if(cls.r.length>1)cls.r.sort((a,b)=>a-b);
 classes.sort((a,b)=>a.r[0]-b.r[0]);
 return{version:1,owners,rows,classes:[...classes.map(({r,o})=>({r,o})),...empty]};
}
