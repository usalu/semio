test("retirement source capacity corpus is closed and agrees with independent decimal arithmetic",async()=>{
  const Decimal=(await import("decimal.js")).default.clone({precision:100});
  const local=join(root,owner,"♻️retirement"),fixture=JSON.parse(readFileSync(join(local,"🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(local,"🧬️schema/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
  expect(validate(fixture)).toBe(true);
  const cases=fixture.capacityAdmission.cases;expect(cases.length).toBe(12);expect(new Set(cases.map((row:any)=>row.id)).size).toBe(12);
  expect(new TextEncoder().encode(fixture.capacityAdmission.source).length).toBe(fixture.capacityAdmission.sourceBytes);
  const observations=[];
  for(const bits of [32,64]){
    const maximum=(1n<<BigInt(bits))-1n,signed=maximum>>1n;
    const number=(value:string)=>value==="usizeMax"?maximum:value==="isizeMax"?signed:BigInt(value);
    for(const row of cases){
      const source=number(row.sourceBytes),multiple=number(row.multiples),scaffold=number(row.scaffoldBytes),owned=number(row.ownedBytes),capacity=source*multiple+scaffold;
      const reference=new Decimal(source.toString()).mul(multiple.toString()).add(scaffold.toString());
      const accepted=capacity<=signed&&capacity>=owned,oracle=reference.lte(signed.toString())&&reference.gte(owned.toString());
      expect(reference.toFixed(0)).toBe(capacity.toString());expect(accepted).toBe(oracle);expect(accepted).toBe(row.accepted);if(accepted)expect(capacity.toString()).toBe(number(row.maximumBytes).toString());
      observations.push({bits,id:row.id,accepted,maximumBytes:accepted?capacity.toString():null});
    }
  }
  expect(observations.length).toBe(24);
  const hostile=[{...fixture,extra:true},{...fixture,capacityAdmission:{...fixture.capacityAdmission,extra:true}},{...fixture,leases:{...fixture.leases,extra:true}},{...fixture,continuation:{...fixture.continuation,extra:true}},{...fixture,capacityAdmission:{...fixture.capacityAdmission,cases:cases.map((row:any,index:number)=>index===0?{...row,accepted:"true"}:row)}},{...fixture,capacityAdmission:{...fixture.capacityAdmission,cases:cases.map((row:any,index:number)=>index===0?{...row,sourceBytes:"-1"}:row)}},{...fixture,capacityAdmission:{...fixture.capacityAdmission,cases:cases.map((row:any,index:number)=>index===0?{...row,maximumBytes:null}:row)}},{...fixture,capacityAdmission:{...fixture.capacityAdmission,cases:cases.map((row:any,index:number)=>index===3?{...row,maximumBytes:"4"}:row)}},{...fixture,leases:{...fixture.leases,orders:fixture.leases.orders.map((row:any,index:number)=>index===0?[0,0,2]:row)}}];
  for(const value of hostile)expect(validate(value)).toBe(false);
  for(const key of ["leases","capacityAdmission","continuation"]){const value={...fixture};delete value[key];expect(validate(value)).toBe(false);}
  console.log(`[DEBUG] retirement source capacity 24 portable word-width vectors agree with decimal.js; twelve closed-schema hostiles refused`);
});
