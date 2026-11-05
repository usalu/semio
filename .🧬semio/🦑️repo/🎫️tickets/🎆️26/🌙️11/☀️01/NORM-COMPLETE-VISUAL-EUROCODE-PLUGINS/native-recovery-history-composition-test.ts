import fs from 'node:fs';
const p='🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🧪️tests/🔬️unit/🧩️composition-and-retained-decode/🦀️.rs';let s=fs.readFileSync(p,'utf8');
function replace(a:string,b:string){if(!s.includes(a))throw Error('Missing source '+a.slice(0,100));s=s.replace(a,b);}
replace('    cancelled: std::cell::Cell<bool>,','    cancelled: std::cell::Cell<bool>,\n    work: std::cell::Cell<u64>,');
replace('cancelled: std::cell::Cell::new(false) }','cancelled: std::cell::Cell::new(false), work: std::cell::Cell::new(1_000_000) }');
replace('fn paid_turn(decode:', 'struct TurnWork<\'a>(&\'a FlaggedTurn);\nimpl semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient for TurnWork<\'_>{fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|work|work<=self.0.work.get().min(4096)))}}\nfn settle_work(turn:&FlaggedTurn,work:u64){assert!(work<=turn.work.get().min(4096));turn.work.set(turn.work.get()-work)}\n\nfn paid_turn(decode:');
replace('let step = decode.step(bytes, turn, grant, history).expect("advance retained decode");','let outcome = decode.step(bytes, turn, grant, history, &mut TurnWork(turn));\n    settle_work(turn,outcome.work);\n    if turn.should_yield()||turn.is_cancelled(){assert_eq!(outcome.work,0)}\n    let step=outcome.result.expect("advance retained decode");');
replace('fn close_all(decode: &mut RetainedHistoryDecode, bytes: &[u8], grant: RetainedCloneGrant)','fn close_all(decode: &mut RetainedHistoryDecode, bytes: &[u8], grant: RetainedCloneGrant, turn:&FlaggedTurn)');
replace('let step = decode.close_step(bytes, &RetainedHistoryUninterrupted, grant).expect("close retained decode");','let outcome = decode.close_step(bytes, &RetainedHistoryUninterrupted, grant, &mut TurnWork(turn));\n        settle_work(turn,outcome.work);\n        let step=outcome.result.expect("close retained decode");');
s=s.replaceAll('close_all(&mut decode, &bytes, grant);','close_all(&mut decode, &bytes, grant, &turn);');
s=s.replaceAll('let (_, step) = settle(&mut decode, &bytes, &FlaggedTurn::new(), grant, &mut history);','let turn=FlaggedTurn::new();\n    let (_, step) = settle(&mut decode, &bytes, &turn, grant, &mut history);');
fs.writeFileSync(p,s);
