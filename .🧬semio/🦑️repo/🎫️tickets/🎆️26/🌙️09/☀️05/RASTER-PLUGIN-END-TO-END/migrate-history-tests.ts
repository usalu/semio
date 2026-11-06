import { readFileSync, writeFileSync } from "node:fs";
const changes = [
  {path:"🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🗃️history-ledger/♻️retirement/🦀️.rs", pairs:[
    ["let step=ledger.advance_retained_read(key,&factory,grant(d)).unwrap();", "let turn=ledger.advance_retained_read(key,&factory,grant(d));assert!(turn.work>0);let step=turn.result.unwrap();"],
    ["Some(ledger.retain_read(key,grant(d)).unwrap().0)", "{let turn=ledger.retain_read(key,grant(d));assert!(turn.work>0);Some(turn.result.unwrap().0)}"]
  ]},
  {path:"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧪️tests/🔬️unit/🦀️.rs", pairs:[
    ["let(step,allocated,released)=observed(||ledger.advance_retained_read(key,&factory,funded).unwrap());", "let(turn,allocated,released)=observed(||ledger.advance_retained_read(key,&factory,funded));assert!(turn.work>0);let step=turn.result.unwrap();"],
    ["let(mut root,_)=ledger.retain_read(key,funded).unwrap();", "let turn=ledger.retain_read(key,funded);assert!(turn.work>0);let(mut root,_)=turn.result.unwrap();"]
  ]}
];
for (const {path,pairs} of changes) { let text=readFileSync(path,"utf8"); for (const [a,b] of pairs) { if(!text.includes(a)) throw new Error(`missing expected edit in ${path}`); text=text.replaceAll(a,b); } writeFileSync(path,text); }
