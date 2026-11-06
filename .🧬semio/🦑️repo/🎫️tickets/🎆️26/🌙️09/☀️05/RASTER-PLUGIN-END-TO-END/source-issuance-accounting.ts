import {readFileSync,writeFileSync} from "node:fs";
const root=process.argv[2]!;
function change(relative:string,edits:[string,string][]){const path=`${root}/${relative}`;let source=readFileSync(path,"utf8");for(const[before,after]of edits){if(!source.includes(before))throw new Error(`Missing input in ${relative}: ${before}`);source=source.replaceAll(before,after);}writeFileSync(path,source);}
change('🎟️admission/🧪️tests/🦀️.rs',[[ 'f["nullWork"].as_u64().unwrap()', 'f["nullWork"][phase].as_u64().unwrap()' ]]);
change('🎟️admission/🦀️.rs',[
 ['"owner"|"lease"=>2','"owner"|"lease"|"publication"=>2'],
 ['Ok(None)=>return progress(0,Default::default()),Err(e)=>return failure(e,0)};let payload=', 'Ok(None)=>return progress(1,Default::default()),Err(e)=>return failure(e,1)};let payload='],
 ['let(lease,p)=match turn.result{Ok(value)=>value,Err(_)=>return failure(ValueError::invariant("funded unique publication cannot refuse"),0)};', 'let work=1+turn.work;let(lease,p)=match turn.result{Ok(value)=>value,Err(_)=>return failure(ValueError::invariant("funded unique publication cannot refuse"),work)};'],
 ['complete(turn.work,p)},\n   _=>','complete(work,p)},\n   _=>']
]);
