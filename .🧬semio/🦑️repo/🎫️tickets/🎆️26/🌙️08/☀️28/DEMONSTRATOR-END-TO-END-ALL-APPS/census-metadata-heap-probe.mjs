import{getHeapStatistics}from"node:v8";
function probe(){const count=262144,before=getHeapStatistics(),rows=new Map(),candidates=new Map(),references=new Set(),names=[];
for(let index=0;index<count;index++){const id=index.toString(16).padStart(12,"0"),path=Buffer.from("/"+"🧰".repeat(121)+id).toString(),stamp=Array(7).fill("18446744073709551615").join(":")+id;const row={path,stamp,kind:"generation-fragment",bytes:536870912,allocatedBytes:536870912,modified:1792578000000,hash:id.padStart(64,"f")};rows.set(path,row);candidates.set(path,row);references.add(row.hash);names.push(id);}
const peak=getHeapStatistics();return Object.freeze({engine:"Node V8",count:rows.size,pathUnits:rows.keys().next().value.length,referenceCount:references.size,candidateCount:candidates.size,nameCount:names.length,before,peak,heapDeltaBytes:peak.used_heap_size-before.used_heap_size});}
export default probe();
