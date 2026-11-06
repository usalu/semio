import { readFileSync, writeFileSync } from 'node:fs';
const path='🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs';
let body=readFileSync(path,'utf8');
const old='let turn=control.with_construction_recipient(&mut construction_loan,|control|Ok::<_,ValueError>(control.receive_shared(&mut input,&mut output,SharedIssuer::owned(),&mut frame))).unwrap();';
const replacement='let turn=funded(&mut control,|control|control.with_construction_recipient(&mut construction_loan,|control|Ok::<_,ValueError>(control.receive_shared(&mut input,&mut output,SharedIssuer::owned(),&mut frame))).unwrap());';
if(!body.includes(old))throw new Error('original admission law missing');
body=body.replace(old,replacement);
writeFileSync(path,body);
