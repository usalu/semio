import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname}from"node:path";const t=dirname(import.meta.path),base="🧰️framework/🔨️modules/🧵️job/📬️outcome/";
function edit(p:string,f:(s:string)=>string){const s=readFileSync(p,"utf8"),n=f(s);if(n===s)return;const tmp=t+"/🗑️generated/release-callers-"+Math.random().toString(16).slice(2)+".rs";writeFileSync(tmp,n);if(readFileSync(p,"utf8")!==s)throw Error("CAS "+p);renameSync(tmp,p)}
const files=JSON.parse(readFileSync(t+"/🗑️generated/release-currency-files.json","utf8"))as string[];
for(const p of files)if(p.endsWith(".rs"))edit(p,s=>s.replaceAll(/OriginalReturnEscrowAmounts\{([^{}]*)\}/g,(all,body)=>/release_bytes|pub |^\s*Self\b/.test(body)?all:"OriginalReturnEscrowAmounts{"+body+",release_bytes:0}").replaceAll(/held_copy_bytes:([^,{}]*),held_fuel:/g,(all,v)=>"held_copy_bytes:"+v+",held_release_bytes:0,held_fuel:"));
edit(base+"📤️publication/🪙️escrow/🧪️tests/🦀️.rs",s=>s.replaceAll("bytes.len(),184","bytes.len(),208").replaceAll("bytes.len(),4400","bytes.len(),5168").replaceAll("index*136","index*160").replace("[101u64,1,202,303,48,1,4,1,1,8,1,0,0,0,0,0,0]","[101u64,1,202,303,48,1,4,1,1,8,1,0,0,0,0,0,0,0,0,0]"));
edit("🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/📤️receipt/🦀️.rs",s=>{
 s=s.replace("pub held_work:u64,pub redeemed_items:","pub held_work:u64,pub held_release:u64,pub redeemed_items:").replace("pub redeemed_work:u64,pub released_items:","pub redeemed_work:u64,pub redeemed_release:u64,pub released_items:").replace("pub released_work:u64}","pub released_work:u64,pub released_release:u64}");
 s=s.replaceAll("count*136","count*160").replaceAll("index*136","index*160").replace("[0u64;17]","[0u64;20]");
 s=s.replace("held_work:v[10],redeemed_items:v[11],redeemed_copy:v[12],redeemed_work:v[13],released_items:v[14],released_copy:v[15],released_work:v[16]","held_work:v[10],held_release:v[11],redeemed_items:v[12],redeemed_copy:v[13],redeemed_work:v[14],redeemed_release:v[15],released_items:v[16],released_copy:v[17],released_work:v[18],released_release:v[19]");
 s=s.replace("used!=row.held_work){return Err(invalid())}","used!=row.held_work)||row.redeemed_release.checked_add(row.released_release).is_none_or(|used|used>row.held_release||row.status==2&&used!=row.held_release){return Err(invalid())}");
 s=s.replace("(48..=4400)","(48..=5168)").replace("(escrow.length-48)%136","(escrow.length-48)%160");
 return s;
});
console.log("same-source wire reader and existing callers migrated");
