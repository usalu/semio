
const base="🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list";
const cases=[{mode:0,name:"complete",receiveCalls:1,recoverCalls:0,childWork:1,interruptions:0},{mode:1,name:"pending",receiveCalls:1,recoverCalls:1,childWork:2,interruptions:0},{mode:2,name:"unwindBeforeReply",receiveCalls:1,recoverCalls:1,childWork:2,interruptions:1},{mode:3,name:"unwindAfterReply",receiveCalls:1,recoverCalls:0,childWork:1,interruptions:1}];
const law=await Bun.file(base+"/🧪️tests/borrowed.json").json();law.interruptions=cases;
const schema=await Bun.file(base+"/🧪️tests/borrowed-schema.json").json();schema.required.push("interruptions");schema.properties.interruptions={const:cases};
await Bun.write(base+"/🧪️tests/borrowed.json",JSON.stringify(law,null,2)+"\n");
await Bun.write(base+"/🧪️tests/borrowed-schema.json",JSON.stringify(schema,null,2)+"\n");
const p=base+"/🧪️tests/borrowed.test.ts";let s=await Bun.file(p).text();const old="expect(law.nativeLifecycle).toEqual";s=s.replace(old,'for(const row of law.interruptions)expect(db.query("select 1 receiveCalls,case when ? in (1,2) then 1 else 0 end recoverCalls,case when ? in (1,2) then 2 else 1 end childWork,case when ?>1 then 1 else 0 end interruptions").get(row.mode,row.mode,row.mode)).toEqual({receiveCalls:row.receiveCalls,recoverCalls:row.recoverCalls,childWork:row.childWork,interruptions:row.interruptions});'+old);await Bun.write(p,s);
