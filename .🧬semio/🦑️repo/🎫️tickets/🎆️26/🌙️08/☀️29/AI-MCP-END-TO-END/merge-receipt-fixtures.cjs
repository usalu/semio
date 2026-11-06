const fs=require('fs'),p='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📤️receipt/🧪️tests/';
const life=fs.readFileSync(p+'lifecycle.json','utf8'),escrow=fs.readFileSync(p+'escrow.json','utf8'),combined=JSON.stringify({lifecycle:JSON.parse(life),escrow:JSON.parse(escrow)})+'\n';fs.writeFileSync(p+'shared.json',combined,{flag:'wx'});
function edit(name,from,to){const path=p+name,old=fs.readFileSync(path,'utf8');if(old.split(from).length!==2)throw Error('anchor changed: '+path);const tmp=path+'.fixture.tmp';fs.writeFileSync(tmp,old.replace(from,to),{flag:'wx'});if(fs.readFileSync(path,'utf8')!==old){fs.unlinkSync(tmp);throw Error('concurrent edit: '+path)}fs.renameSync(tmp,path)}
edit('lifecycle.test.ts','import fixture from"./lifecycle.json";','import shared from"./shared.json";const fixture=shared.lifecycle;');
edit('escrow.test.ts','import fixture from"./escrow.json";','import shared from"./shared.json";const fixture=shared.escrow;');
edit('🦀️.rs','serde_json::from_str(include_str!("escrow.json")).unwrap();','serde_json::from_str::<serde_json::Value>(include_str!("shared.json")).unwrap()["escrow"].clone();');
edit('🦀️.rs','serde_json::from_str(include_str!("lifecycle.json")).unwrap();','serde_json::from_str::<serde_json::Value>(include_str!("shared.json")).unwrap()["lifecycle"].clone();');
if(fs.readFileSync(p+'lifecycle.json','utf8')!==life||fs.readFileSync(p+'escrow.json','utf8')!==escrow)throw Error('fixture changed during merge');fs.unlinkSync(p+'lifecycle.json');fs.unlinkSync(p+'escrow.json');
