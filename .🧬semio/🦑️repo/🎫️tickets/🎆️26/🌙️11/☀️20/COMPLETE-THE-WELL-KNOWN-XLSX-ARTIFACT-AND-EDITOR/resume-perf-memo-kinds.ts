const path=process.argv[2];
const memo=JSON.parse(await Bun.file(path).text());
const kinds:Record<string,number>={},owner:Record<string,number>={};
for(const row of memo.observations)kinds[row.kind]=(kinds[row.kind]??0)+1;
let total=0,max=0;for(const o of memo.owners){total+=o.rows.length;max=Math.max(max,o.rows.length);for(const at of o.rows){const k=memo.observations[at].kind;owner[k]=(owner[k]??0)+1;}}
const nested=memo.observations.filter((r:any)=>r.kind==="analysis").reduce((n:number,r:any)=>n+(r.fact.nestedObservations?.length??0),0);
console.log(JSON.stringify({observations:memo.observations.length,owners:memo.owners.length,ownerRows:total,maxOwnerRows:max,kinds,ownerKinds:owner,nested,sample:memo.observations.slice(0,3)},null,1).slice(0,4000));
