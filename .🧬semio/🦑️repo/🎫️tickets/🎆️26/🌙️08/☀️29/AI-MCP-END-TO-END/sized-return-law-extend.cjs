const fs=require('fs'),f='🧰️framework/🔨️modules/🧵️job/⏱️context/📦️return/🎟️place/🧪️tests/🦀️.rs';const before=fs.readFileSync(f,'utf8');let s=before;
const a='if admitted{\n   assert!(place.record_sized_return(address+1,requested).is_err());';
const b=`if admitted{
   caller.turn(q,|cx|{let snapshot=semio_framework_job_context::testkit::return_budget_snapshot(cx);assert!(snapshot.conserved());assert_eq!(snapshot.held_copy_bytes,requested);Ok(())}).unwrap();
   let mut foreign=OriginalJobFixtureCaller::admit(OperationId(8899),Generation(4),stock,clock,10000,128);
   foreign.turn(q,|cx|{let progress=cx.retained_progress();assert!(place.settle_return(cx).is_err());assert_eq!(cx.retained_progress(),progress);Ok(())}).unwrap();foreign.retire(stock);
   assert!(place.record_sized_return(address+1,requested).is_err());`;
if(s.split(a).length!==2)throw Error('marker');s=s.replace(a,b);
const c='if row["captured"].as_bool().unwrap(){place.record_sized_return(address,requested).unwrap();}\n   let mut copied=0;';
const d='if row["captured"].as_bool().unwrap(){place.record_sized_return(address,requested).unwrap();}else{caller.cancel().cancel_now();}\n   let mut copied=0;';
if(s.split(c).length!==2)throw Error('marker');s=s.replace(c,d);if(fs.readFileSync(f,'utf8')!==before)throw Error('changed');let t=f+'.guest-law2.tmp';fs.writeFileSync(t,s);if(fs.readFileSync(f,'utf8')!==before){fs.unlinkSync(t);throw Error('changed before rename')}fs.renameSync(t,f);
