// [gram2-g] text transform of the observation module into the "row ids" variant (no permanent key strings): exported as transformObservationV1(text) so plugins and laws share one definition.
export const SLOT_BLOCK=`
/** 🔢️ The canonical row of one fact and the dense number that stands for it: equal (kind, path, fact) is one row and one number, found through a table of hashes (verified by comparison) instead of a permanent key string per row. */
let rowSlots=new WeakMap();const rowTable=new Map();let nextRowSlot=1;
const hashTextV1=(text,seed)=>{let hash=seed;for(let at=0;at<text.length;at++){hash^=text.charCodeAt(at);hash=Math.imul(hash,16777619);}return hash>>>0;};
const rowHashV1=(kind,path,factKey)=>hashTextV1(path,hashTextV1(kind,2166136261))*2097152+(hashTextV1(factKey,hashTextV1(path,0x811c9dc5))>>>11);
const sameRowV1=(row,kind,path,factKey)=>row.kind===kind&&row.path===path&&factKeyV1(row.fact)===factKey;
function lookupRowV1(kind,path,fact,make){
 const factKey=factKeyV1(fact),hash=rowHashV1(kind,path,factKey),bucket=rowTable.get(hash);
 if(bucket!==undefined){if(Array.isArray(bucket)){for(const row of bucket)if(sameRowV1(row,kind,path,factKey))return row;}else if(sameRowV1(bucket,kind,path,factKey))return bucket;}
 const row=make();rowSlots.set(row,nextRowSlot++);
 if(bucket===undefined)rowTable.set(hash,row);else if(Array.isArray(bucket))bucket.push(row);else rowTable.set(hash,[bucket,row]);
 return row;
}
function rowIdV1(row){let id=rowSlots.get(row);if(id===undefined){id=rowSlots.get(lookupRowV1(row.kind,row.path,row.fact,()=>row));rowSlots.set(row,id);}return id;}
function clearRowTableV1(){rowTable.clear();rowSlots=new WeakMap();}
`;
const swap=(text,from,to,what)=>{if(text.split(from).length!==2)throw Error("anchor "+what+" missing or ambiguous");return text.replace(from,to);};
export function transformObservationV1(text){
 let out=text;
 out=swap(out,"for(const row of rows)target.set(observationKeyV1(row),row);","for(const row of rows)target.set(rowIdV1(row),row);","retain");
 const start=out.indexOf("function canonicalObservationV1(kind,path,fact){"),end=out.indexOf("function recordRowV1(row){");
 if(start<0||end<start)throw Error("anchor canonical missing");
 out=out.slice(0,start)+"function canonicalObservationV1(kind,path,fact){return lookupRowV1(kind,path,fact,()=>({kind,path:graphStringInternerV1.share(path),fact}));}\n"+out.slice(end);
 out=swap(out,"const internedObservations=new Map(),INTERNED_OBSERVATIONS=4194304;","const INTERNED_OBSERVATIONS=4194304;"+SLOT_BLOCK,"intern");
 const a=out.indexOf("function internObservationsV1(rows){"),b=out.indexOf("\n",a);
 out=out.slice(0,a)+"function internObservationsV1(rows){return rows.map(row=>{const known=lookupRowV1(row.kind,row.path,row.fact,()=>{if(typeof row.path==='string')row.path=graphStringInternerV1.share(row.path);return row;});return known;});}"+out.slice(b);
 out=swap(out,"rows.every(row=>{const key=observationKeyV1(row),previous=checks.get(key);","rows.every(row=>{const key=rowIdV1(row),previous=checks.get(key);","shared checks");
 out=swap(out,"rows.every(row=>{const key=observationKeyV1(row),known=checks.get(key);","rows.every(row=>{const key=rowIdV1(row),known=checks.get(key);","nested checks");
 out=swap(out,"function observationKeyV1(row){let key=observationKeys.get(row);if(key===undefined){key=row.kind+\"\\0\"+row.path+\"\\0\"+factKeyV1(row.fact);observationKeys.set(row,key);}return key;}","function observationKeyV1(row){return row.kind+\"\\0\"+row.path+\"\\0\"+factKeyV1(row.fact);}","key");
 out=out.replaceAll("internedObservations.clear();","clearRowTableV1();");
 out=out.replaceAll("interned:internedObservations.size","interned:rowTable.size");
 return out;
}
