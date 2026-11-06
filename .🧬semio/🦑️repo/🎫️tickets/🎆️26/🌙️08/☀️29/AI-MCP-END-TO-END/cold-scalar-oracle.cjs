// Independent WebAssembly engine oracle for the neutral scalar calling convention.
const fs=require("fs");
const fixture=JSON.parse(fs.readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🎟️unit/plan.json","utf8"));
const leb=n=>{const a=[];do{let b=n&127;n>>>=7;if(n)b|=128;a.push(b)}while(n);return a};
const section=(id,bytes)=>[id,...leb(bytes.length),...bytes];
for(const row of fixture.cases){
 const n=row.arguments,params=Array(n).fill(0x7e);
 const type=[1,0x60,...leb(n),...params,1,0x7e],fun=[1,0],exp=[1,1,102,0,0];
 const body=[0,0x20,0];for(let i=1;i<n;i++)body.push(0x20,...leb(i),0x7c);body.push(0x0b);
 const wasm=new Uint8Array([0,97,115,109,1,0,0,0,...section(1,type),...section(3,fun),...section(7,exp),...section(10,[1,...leb(body.length),...body])]);
 const fn=new WebAssembly.Instance(new WebAssembly.Module(wasm)).exports.f;
 const args=Array.from({length:n},(_,i)=>BigInt(i+1));const expected=args.reduce((a,b)=>a+b,0n);
 if(fn(...args)!==expected)throw Error("wrong scalar forwarding for "+row.action);
}
const db=new (require("node:sqlite").DatabaseSync)(":memory:");try{const query=db.prepare("SELECT CASE WHEN ? IN ('publish-failure','read-failure','acknowledge-failure','acknowledge-failure-chunk','admit-actor','close-actor','close-jobs','close-actor-output') THEN 'infrastructure' WHEN ?='borrow-chunk' THEN NULL ELSE 'semantic' END AS purpose");for(const row of fixture.cases){if(query.get(row.action,row.action).purpose!==row.purpose)throw Error("wrong purpose for "+row.action)}}finally{db.close()}console.log("WebAssembly scalar ABI and SQLite purpose oracle passed "+fixture.cases.length+" cases");
