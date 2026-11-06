import {readFileSync,writeFileSync} from 'node:fs';
const path='🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs';
let body=readFileSync(path,'utf8');
const old='control.with_construction_recipient(&mut loan,|control|Ok::<_,ValueError>(control.receive_arc(&mut input,&mut output,&mut owner))).unwrap()';
const replacement='funded(&mut control,|control|control.with_construction_recipient(&mut loan,|control|Ok::<_,ValueError>(control.receive_arc(&mut input,&mut output,&mut owner))).unwrap())';
const start=body.indexOf('let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../🏭️construction/🧪️tests/🧫️fixtures/effect-frame.json"))');
if(start<0)throw new Error('arc allocator law missing');
const tail=body.slice(start),parts=tail.split(old);if(parts.length!==4)throw new Error(`expected three original admissions, found ${parts.length-1}`);
body=body.slice(0,start)+parts.join(replacement);writeFileSync(path,body);
