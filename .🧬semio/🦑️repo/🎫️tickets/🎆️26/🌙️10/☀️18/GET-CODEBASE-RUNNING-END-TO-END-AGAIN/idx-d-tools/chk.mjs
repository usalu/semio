import{readFileSync}from"node:fs";import{spawnSync}from"node:child_process";
const O=process.argv[2];const{encodeFactIndexV1,decodeFactIndexV1,canonicalJsonV1}=await import(O+"/🟨️.mjs");
const f=JSON.parse(readFileSync(O+"/🧪️tests/🔣️.json","utf8"));
const py=(cmd,input)=>spawnSync("python3",["-I",O+"/🧪️tests/🐍️oracle.py",cmd],{input});
for(const c of f.indexes){const js=encodeFactIndexV1(c.index,{maximumBytes:1<<26});const p=py("encode",JSON.stringify(c.index));const same=js.toString("hex")===p.stdout.toString();const d=decodeFactIndexV1(js);const eq=canonicalJsonV1(d)===canonicalJsonV1(c.index);const pd=py("decode",js.toString("hex"));console.log(c.id,"bytes",js.length,"py-same",same,"rt",eq,"pydec",pd.status,p.stderr.toString().slice(0,200));if(!same||!eq){console.log(js.toString("hex").slice(0,200));console.log(p.stdout.toString().slice(0,200));}}
for(const r of f.refusals){let threw=false;try{decodeFactIndexV1(Buffer.from(r.hex,"hex"))}catch(e){threw=true}const pd=py("decode",r.hex);console.log("refuse",r.id,threw,pd.status===3);}
