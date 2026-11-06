import {readFileSync,writeFileSync} from 'node:fs';
const file='🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs';let text=readFileSync(file,'utf8');
const changes:[string,string][]=[
 ['let mut frame=NativeReceivingEffect::default();let turn=control.with_construction_recipient','let mut frame=NativeSharedAdmission::default();let turn=control.with_construction_recipient'],
 ['assert_eq!(output.is_some(),case["born"].as_bool().unwrap());drop(construction_loan);','assert_eq!(frame.has_birth(),case["born"].as_bool().unwrap());assert_eq!(output.is_some(),case["output"].as_bool().unwrap());assert_eq!(input.is_some(),case["input"].as_bool().unwrap());if frame.has_birth(){let d=frame.close_demand().unwrap();let turn=frame.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..Default::default()});assert!(turn.result.is_ok());assert!(frame.terminal_is_empty())}drop(construction_loan);'],
];for(const [before,after] of changes){if(!text.includes(before))throw new Error(`preborn law changed: ${before}`);text=text.replace(before,after)}writeFileSync(file,text);
