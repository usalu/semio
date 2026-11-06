import {readFileSync,writeFileSync} from 'node:fs';
const path='🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs';
let body=readFileSync(path,'utf8');
function replace(old:string,replacement:string){if(!body.includes(old))throw new Error('expected construction law missing');body=body.replace(old,replacement)}
replace('let mut control=NativeOwnedDecodeControl::new(&mut original,&mut loan);\n  let(first,birth_heap)=','let mut payer=ConstructionWork;let(first,birth_heap)=original.with_work_recipient(&mut payer,|original|{let mut control=NativeOwnedDecodeControl::new(original,&mut loan);\n  let(first,birth_heap)=');
replace('}drop(control);drop(loan);assert_eq!(recipient.calls,case["calls"]','}drop(control);Ok::<_,ValueError>((first,birth_heap))}).unwrap();drop(loan);assert_eq!(recipient.calls,case["calls"]');
replace('control.with_construction_recipient(&mut construction_loan,|control|->Result<(),OriginalReceivingPrimary>{assert_eq!(control.ledger_identity(),ledger);','funded(&mut control,|control|control.with_construction_recipient(&mut construction_loan,|control|->Result<(),OriginalReceivingPrimary>{assert_eq!(control.ledger_identity(),ledger);');
replace('Ok(())}).unwrap();drop(construction_loan);drop(foreign_loan);assert_eq!((recipient.calls,recipient.receipts),(4,4))','Ok(())}).unwrap());drop(construction_loan);drop(foreign_loan);assert_eq!((recipient.calls,recipient.receipts),(4,4))');
writeFileSync(path,body);
