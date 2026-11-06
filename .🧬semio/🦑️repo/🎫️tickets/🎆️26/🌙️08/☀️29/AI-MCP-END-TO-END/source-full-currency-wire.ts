import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname}from"node:path";const t=dirname(import.meta.path);function edit(p:string,f:(s:string)=>string){const s=readFileSync(p,"utf8"),n=f(s),tmp=t+"/🗑️generated/full-currency-wire-"+Math.random().toString(16).slice(2);if(n===s)throw Error("unchanged "+p);writeFileSync(tmp,n);if(readFileSync(p,"utf8")!==s)throw Error("CAS");renameSync(tmp,p)}
edit("🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/📤️receipt/🦀️.rs",s=>{
 for(const c of ["held","redeemed","released"])s=s.replace("pub "+c+"_release:u64","pub "+c+"_release:u64,pub "+c+"_capacity:u64");
 s=s.replaceAll("count*160","count*184").replaceAll("index*160","index*184").replace("[0u64;20]","[0u64;23]");
 s=s.replace("held_release:v[11],redeemed_items:v[12],redeemed_copy:v[13],redeemed_work:v[14],redeemed_release:v[15],released_items:v[16],released_copy:v[17],released_work:v[18],released_release:v[19]","held_release:v[11],held_capacity:v[12],redeemed_items:v[13],redeemed_copy:v[14],redeemed_work:v[15],redeemed_release:v[16],redeemed_capacity:v[17],released_items:v[18],released_copy:v[19],released_work:v[20],released_release:v[21],released_capacity:v[22]");
 s=s.replace("used!=row.held_release){return Err(invalid())}","used!=row.held_release)||row.redeemed_capacity.checked_add(row.released_capacity).is_none_or(|used|used>row.held_capacity||row.status==2&&used!=row.held_capacity){return Err(invalid())}");
 s=s.replace("(48..=5168)","(48..=5936)").replace("(escrow.length-48)%160","(escrow.length-48)%184");
 s=s.replace("bytes=[0u8;208]","bytes=[0u8;232]").replace("let words=[1u64,2,3,4,0,1,1,1,0,0,0,12,0,0,0,7,0,0,0,5]","let words=[1u64,2,3,4,0,1,1,1,0,0,0,12,64,0,0,0,7,16,0,0,0,5,48]").replaceAll("48+19*8..56+19*8","48+21*8..56+21*8");
 const tail='bytes[48+21*8..56+21*8].copy_from_slice(&4u64.to_le_bytes());assert!(OriginalEscrowSnapshot::read(&bytes).is_err());';
 if(!s.includes(tail))throw Error("codec law");s=s.replace(tail,tail+'\n  bytes[48+21*8..56+21*8].copy_from_slice(&5u64.to_le_bytes());bytes[48+22*8..56+22*8].copy_from_slice(&49u64.to_le_bytes());assert!(OriginalEscrowSnapshot::read(&bytes).is_err());');
 return s;
});
edit("🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🪙️escrow/🧪️tests/🦀️.rs",s=>s.replaceAll("bytes.len(),208","bytes.len(),232").replaceAll("bytes.len(),5168","bytes.len(),5936").replaceAll("index*160","index*184").replace("[101u64,1,202,303,48,1,4,1,1,8,1,0,0,0,0,0,0,0,0,0]","[101u64,1,202,303,48,1,4,1,1,8,1,0,0,0,0,0,0,0,0,0,0,0,0]"));
const b="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📤️receipt/";
edit(b+"🧪️tests/shared.json",s=>{let v=JSON.parse(s);v.escrow.rows=v.escrow.rows.map((r:number[])=>[...r.slice(0,11),0,0,...r.slice(11,14),0,0,...r.slice(14,17),0,0]);return JSON.stringify(v,null,2)+"\n"});
edit(b+"🧬️schema/escrow.json",s=>{let v=JSON.parse(s);v.properties.rows.items.minItems=23;v.properties.rows.items.maxItems=23;return JSON.stringify(v)+"\n"});
edit(b+"🧪️tests/escrow.test.ts",s=>s.replaceAll("4400","5936").replaceAll("4536","6120").replaceAll("136","184").replace("[48,184,5936]","[48,232,5936]").replace("extent:320","extent:416").replace('SELECT ?+?<=? AND ?+?<=? AND ?+?<=? AS ok','SELECT ?+?<=? AND ?+?<=? AND ?+?<=? AND ?+?<=? AND ?+?<=? AS ok').replace("row[11],row[14],row[8],row[12],row[15],row[9],row[13],row[16],row[10]","row[13],row[18],row[8],row[14],row[19],row[9],row[15],row[20],row[10],row[16],row[21],row[11],row[17],row[22],row[12]"));
console.log("same full-currency schema and actor wire updated");
