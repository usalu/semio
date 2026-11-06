import {readFileSync,writeFileSync} from 'node:fs';
const file='🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs';
const text=readFileSync(file,'utf8');
const before='SealedShared::admit(input.take().unwrap(),SharedIssuer::owned(),grant);let result=match turn.result{Ok((owner,progress))=>{*actual=Some(progress.retained_capacity_bytes);';
const after='SealedShared::admit(input.take().unwrap(),SharedIssuer::owned(),grant,actual);let result=match turn.result{Ok((owner,progress))=>{';
if(!text.includes(before))throw new Error('shared law changed');
writeFileSync(file,text.replace(before,after));
