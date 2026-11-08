import sys
sys.path.insert(0,'.')
D="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛬️decode/"
P=D+"🏭️construction/📦️backing/🧪️tests/🦀️.rs"
tail=open('/tmp/claude-501/backing-old-tail.rs',encoding='utf-8').read()
def rep(s,old,new,n=1):
    assert s.count(old)==n,(s.count(old),old[:90]);return s.replace(old,new)
t=tail
t=rep(t,"semio_framework_value_testing_allocator::observe_retirement_allocations","semio_framework_value_testing_allocator::observe_retirement_allocations",t.count("semio_framework_value_testing_allocator::observe_retirement_allocations"))
# layout test
t=rep(t,"let(value,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||allocate_empty_vec::<()>(count).unwrap());","let mut marker=None;let(value,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||allocate_empty_vec::<()>(count,&mut marker).unwrap());")
t=rep(t,"let(value,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||allocate_empty_vec::<u8>(count).unwrap());","let mut marker=None;let(value,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||allocate_empty_vec::<u8>(count,&mut marker).unwrap());")
t=rep(t,"let(value,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||allocate_empty_vec::<u64>(count).unwrap());","let mut marker=None;let(value,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||allocate_empty_vec::<u64>(count,&mut marker).unwrap());")
# crate:: paths
t=t.replace("crate::native_decoding::","").replace("crate::","")
# test 3 refused post
old3="let mut authority=OriginalConstructionAuthority::new(Construction{grant:bank,submitted:Default::default(),refuse:true});let mut loan=WorkLoan(&fuel);let mut work_issuer=NativeDecodeWorkIssuer::new();let mut native_work_loan=work_issuer.loan(&mut loan);let mut output=None;let mut frame=NativeReceivingEffect::default();let(turn,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||authority.with_loan(&mut (),|construction_loan|control.with_construction_recipient(construction_loan,|control|control.with_work_recipient(&mut native_work_loan,|control|Ok::<_,ValueError>(control.admit_vec::<u64>(&mut output,row[\"count\"].as_u64().unwrap()as usize,bank,&mut frame))).unwrap())).unwrap().unwrap());drop(native_work_loan);drop(work_issuer);drop(loan);"
new3="let mut authority=OriginalConstructionAuthority::new(Construction{grant:bank,submitted:Default::default(),refuse:true});let mut work=OriginalWorkAuthority::new(WorkLoan(fuel));let mut output=None;let mut frame=NativeReceivingEffect::default();let(turn,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||work.with_loan(&mut (),|work_loan|authority.with_loan(&mut (),|construction_loan|control.with_construction_recipient(construction_loan,|control|control.with_work_recipient(work_loan,|control|Ok::<_,ValueError>(control.admit_vec::<u64>(&mut output,row[\"count\"].as_u64().unwrap()as usize,bank,&mut frame))).unwrap())).unwrap().unwrap()).unwrap());"
t=rep(t,old3,new3)
# test 4 quote denial
old4="let mut loan=WorkLoan(&fuel);let mut work_issuer=NativeDecodeWorkIssuer::new();let mut native_work_loan=work_issuer.loan(&mut loan);let mut output=None;let mut frame=NativeReceivingEffect::default();let(turn,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||authority.with_loan(&mut (),|construction_loan|control.with_construction_recipient(construction_loan,|control|control.with_work_recipient(&mut native_work_loan,|control|Ok::<_,ValueError>(control.admit_vec::<u64>(&mut output,row[\"count\"].as_u64().unwrap()as usize,grant,&mut frame))).unwrap())).unwrap().unwrap());"
new4="let mut work=OriginalWorkAuthority::new(WorkLoan(fuel));let mut output=None;let mut frame=NativeReceivingEffect::default();let(turn,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||work.with_loan(&mut (),|work_loan|authority.with_loan(&mut (),|construction_loan|control.with_construction_recipient(construction_loan,|control|control.with_work_recipient(work_loan,|control|Ok::<_,ValueError>(control.admit_vec::<u64>(&mut output,row[\"count\"].as_u64().unwrap()as usize,grant,&mut frame))).unwrap())).unwrap().unwrap()).unwrap());"
t=rep(t,old4,new4)
# test 5
old5="let fuel=10;let mut loan=WorkLoan(&fuel);let mut work_issuer=NativeDecodeWorkIssuer::new();let mut native_work_loan=work_issuer.loan(&mut loan);let mut frame:NativeReceivingEffect<ValueError>=NativeReceivingEffect::default();let turn=authority.with_loan(&mut (),|construction_loan|control.with_construction_recipient(construction_loan,|control|control.with_work_recipient(&mut native_work_loan,|control|{"
new5="let mut work=OriginalWorkAuthority::new(WorkLoan(10));let mut frame:NativeReceivingEffect<ValueError>=NativeReceivingEffect::default();let turn=work.with_loan(&mut (),|work_loan|authority.with_loan(&mut (),|construction_loan|control.with_construction_recipient(construction_loan,|control|control.with_work_recipient(work_loan,|control|{"
t=rep(t,old5,new5)
t=rep(t,"let incorrect=row[\"incorrect\"].as_u64().unwrap();","")
t=rep(t,"RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:actual}","RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:u128::from(actual)}")
t=rep(t,"assert!(frame.settle_original_work(control,incorrect).is_err());assert_eq!(control.actual_work(),0);frame.settle_original_work(control,actual)?;frame.finish_allocation(control)?;frame.receipt(control)?;frame.take(control)}).unwrap())).unwrap().unwrap();assert_eq!(turn.work,actual);assert!(turn.result.is_ok());assert_eq!(control.actual_work(),actual);",
 "assert_eq!(control.actual_work(),0);assert!(frame.settle_original_work(control)?);frame.receipt(control)?;frame.take(control)}).unwrap())).unwrap().unwrap()).unwrap();assert_eq!(turn.work,u128::from(actual));assert!(turn.result.is_ok());assert_eq!(control.actual_work(),u128::from(actual));")
# test 6
old6="let mut events=0;let mut port=|_|{events+=1;Ok(())};let mut issuer=NativeDecodeAllocationIssuer::new();let mut loan=issuer.loan(&mut port);let mut observe=|_|true;let mut control=NativeDecodeControl::new_forwarded(bank.maximum_capacity_bytes,&mut observe,&mut loan);let mut frame=NativeReceivingEffect::default();"
new6="let mut events=0;let port=|_:NativeAllocationEvent|{events+=1;Ok(())};let mut authority=NativeAllocationAuthority::new(eff::Port(port));let mut observe=|_|true;authority.with_send_loan(&mut (),|loan|{let mut control=NativeDecodeControl::new_forwarded(bank.maximum_capacity_bytes,&mut observe,loan);let mut frame=NativeReceivingEffect::default();"
t=rep(t,old6,new6)
t=rep(t,"assert_eq!(control.owned_bytes(),0);drop(control);assert_eq!(events,0);close(empty.take().unwrap(),&mut bank,&mut fuel);close(zst.take().unwrap(),&mut bank,&mut fuel);}","assert_eq!(control.owned_bytes(),0);drop(control);close(empty.take().unwrap(),&mut bank,&mut fuel);close(zst.take().unwrap(),&mut bank,&mut fuel);}).unwrap();assert!(authority.into_port().is_ok());assert_eq!(events,0);}")
open(P,'a',encoding='utf-8').write(t)
print("ok")
