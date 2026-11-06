const fs=require('fs'),crypto=require('crypto'),p='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📤️receipt/🧪️tests/🦀️.rs',old=fs.readFileSync(p,'utf8');let s=old;
function all(a,b){if(!s.includes(a))throw Error('missing '+a);s=s.replaceAll(a,b)}
all('use super::*;','use super::*;\nfn escrow()->OriginalEscrowExtent{OriginalEscrowExtent{pointer:3000,length:16}}');
all('let base=OriginalActorFixedReceipt::read(&bytes).unwrap();','let (base,_)=OriginalActorFixedReceipt::read(&bytes).unwrap();');
all('receipt.produce(call)','receipt.produce(call,escrow())');all('OriginalActorFixedReceipt::read(wire).unwrap(),call','OriginalActorFixedReceipt::read(wire).unwrap(),(call,escrow())');
all('receipt.hand_back(pointer+1,144)','receipt.hand_back(pointer+1,152)');all('receipt.hand_back(pointer,144));','receipt.hand_back(pointer,152));');
all('receipt.produce(OriginalActorFixedCall{kind,..base})','receipt.produce(OriginalActorFixedCall{kind,..base},escrow())');
all('[0u8;136]','[0u8;144]');all('if let Ok(call)=result','if let Ok((call,escrow))=result');all('receipt.write(call)','receipt.write(call,escrow)');
all('let original=OriginalActorFixedReceipt::read(&bytes).unwrap();','let (original,_)=OriginalActorFixedReceipt::read(&bytes).unwrap();');all('receipt.produce(||','receipt.produce(escrow(),||');
for(const id of ['original','actual','excessive','maximum']){all('wire.write('+id+')','wire.write('+id+',escrow()).unwrap()');all('decoded.unwrap(),'+id,'decoded.unwrap(),('+id+',escrow())')}
const t=p+'.edit-'+crypto.randomBytes(4).toString('hex');fs.writeFileSync(t,s,{flag:'wx'});if(fs.readFileSync(p,'utf8')!==old){fs.unlinkSync(t);throw Error('concurrent test edit')}fs.renameSync(t,p);
