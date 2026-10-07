struct ImmutableSymbolSource{texts:Vec<String>}
impl semio_framework_dsl_record::native_encoding::FieldProjectionSource for ImmutableSymbolSource{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,ValueError>{
  if path.len()!=1{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"closed symbol source path"))}
  self.texts.get(path[0]).map(|text|semio_framework_dsl_record::native_encoding::FieldProjectionView::Text(text)).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"closed symbol source ordinal"))
 }
}
#[test]
fn borrowed_operation_symbols_preserve_first_source_and_all_paid_pages(){
 use super::{ProjectedSymbolScratch,SourceTextLocator,SourceTextKind};
 use semio_framework_value::NativeEncodeControl;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("@BORROWED_SYMBOL_FIXTURE@")).unwrap();
 let rows=fixture["symbols"].as_array().unwrap();let source=ImmutableSymbolSource{texts:rows.iter().map(|row|row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize)).collect()};
 let mut scratch=ProjectedSymbolScratch::empty();let mut accepted=|_|true;let mut control=NativeEncodeControl::new(fixture["positiveMaximumAllocationBytes"].as_u64().unwrap()as usize,&mut accepted);
 let(result,requested,released)=crate::test_allocation::observe_backing(||{
  for row in rows{scratch.note(&source,SourceTextLocator::new(&[row["source"].as_u64().unwrap()as usize],SourceTextKind::Text)?,row["forced"].as_bool().unwrap(),&mut control)?;}
  scratch.finish(&source,&mut control)
 });result.unwrap();assert_eq!(released,0);assert_eq!(requested,scratch.allocated_bytes());assert_eq!(control.owned_bytes(),requested);
 let expected=fixture["expectedSourceOrder"].as_array().unwrap();assert_eq!(scratch.len().unwrap(),expected.len());
 for(index,ordinal)in expected.iter().enumerate(){let original=&source.texts[ordinal.as_u64().unwrap()as usize];let text=scratch.locator(index).unwrap().resolve(&source).unwrap();assert_eq!(text,original);assert_eq!(text.as_ptr(),original.as_ptr());assert_eq!(scratch.index(&source,text,&mut control).unwrap(),Some(index as u64));}
 assert_eq!(scratch.index(&source,&source.texts[6],&mut control).unwrap(),None);
 let backing=scratch.allocated_bytes();for(items,bytes)in[(0,4096),(1,0)]{let(step,requested,released)=crate::test_allocation::observe_backing(||scratch.retire_one(items,bytes).unwrap());assert_eq!(step,(false,0,0));assert_eq!((requested,released),(0,0));assert_eq!(scratch.len().unwrap(),expected.len());assert_eq!(scratch.allocated_bytes(),backing);}
 let mut retired=0;for _ in 0..1000{let((progress,items,bytes),requested,released)=crate::test_allocation::observe_backing(||scratch.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,bytes);assert!(items<=1&&released<=4096);assert!(scratch.len().is_err());retired+=bytes;if !progress{break;}}
 assert_eq!(retired,backing);assert_eq!(scratch.allocated_bytes(),0);
 let mut scratch=ProjectedSymbolScratch::empty();let mut accepted=|_|true;let mut initial=NativeEncodeControl::new(4096,&mut accepted);
 let((kind,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let mut refusal=None;for row in rows{if let Err(error)=scratch.note(&source,SourceTextLocator::new(&[row["source"].as_u64().unwrap()as usize],SourceTextKind::Text).unwrap(),row["forced"].as_bool().unwrap(),&mut initial){refusal=Some(error);break;}}let error=refusal.expect("fixed4096 original symbol admission must refuse with paid backing retained");let result=(error.kind,error.message.capacity());drop(error);result});
 assert_eq!(kind,ValueRefusalKind::OwnershipLimit);assert_eq!(requested-diagnostic,scratch.allocated_bytes());assert_eq!(released,diagnostic);assert_eq!(initial.owned_bytes(),scratch.allocated_bytes());
 let backing=scratch.allocated_bytes();let mut retired=0;for _ in 0..1000{let((progress,items,bytes),requested,released)=crate::test_allocation::observe_backing(||scratch.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,bytes);assert!(items<=1&&released<=4096);retired+=bytes;if !progress{break;}}assert_eq!(retired,backing);assert_eq!(scratch.allocated_bytes(),0);
 let mut scratch=ProjectedSymbolScratch::empty();let mut fired=false;let mut cancel=|_|{if crate::test_allocation::observed_requested_bytes().is_some_and(|bytes|bytes>0){fired=true;false}else{true}};let mut canceled=NativeEncodeControl::new(65536,&mut cancel);
 let((kind,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let error=scratch.note(&source,SourceTextLocator::new(&[0],SourceTextKind::Text).unwrap(),false,&mut canceled).expect_err("actual post-backing cancellation");let result=(error.kind,error.message.capacity());drop(error);result});assert_eq!(kind,ValueRefusalKind::Canceled);assert_eq!(requested-diagnostic,scratch.allocated_bytes());assert_eq!(released,diagnostic);assert!(canceled.owned_bytes()>=scratch.allocated_bytes());drop(canceled);assert!(fired);
 let backing=scratch.allocated_bytes();let mut retired=0;for _ in 0..1000{let((progress,items,bytes),requested,released)=crate::test_allocation::observe_backing(||scratch.retire_one(1,4096).unwrap());assert_eq!(requested,0);assert_eq!(released,bytes);assert!(items<=1&&released<=4096);retired+=bytes;if !progress{break;}}assert_eq!(retired,backing);assert_eq!(scratch.allocated_bytes(),0);
 let mut scratch=ProjectedSymbolScratch::empty();let mut accepted=|_|true;let mut control=NativeEncodeControl::new(65536,&mut accepted);for row in rows{scratch.note(&source,SourceTextLocator::new(&[row["source"].as_u64().unwrap()as usize],SourceTextKind::Text).unwrap(),row["forced"].as_bool().unwrap(),&mut control).unwrap();}scratch.finish(&source,&mut control).unwrap();let backing=scratch.allocated_bytes();
 use semio_framework_value::retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep};let mut parent=ParentAllocationReturn::<64>::try_new(4096,65536).unwrap();
 for(items,bytes)in[(0,4096),(1,0)]{let(step,requested,released)=crate::test_allocation::observe_backing(||scratch.return_one(&mut parent,items,bytes).unwrap());assert_eq!(step,(false,0));assert_eq!((requested,released),(0,0));assert_eq!(scratch.len().unwrap(),expected.len());assert_eq!(scratch.allocated_bytes(),backing);assert!(parent.terminal_is_empty());}
 for _ in 0..1000{let((progress,returned),requested,released)=crate::test_allocation::observe_backing(||scratch.return_one(&mut parent,1,1).unwrap());assert_eq!((requested,released,returned),(0,0,0));assert_eq!(scratch.allocated_bytes(),backing);assert!(parent.terminal_is_empty());if !progress{break;}}
 let mut returned=0;for _ in 0..1000{let before=scratch.allocated_bytes();let((progress,bytes),requested,released)=crate::test_allocation::observe_backing(||scratch.return_one(&mut parent,1,4096).unwrap());assert_eq!((requested,released),(0,0));assert!(bytes<=4096);assert_eq!(before-scratch.allocated_bytes(),bytes);returned+=bytes;assert_eq!(parent.retained_bytes(),returned);assert!(scratch.len().is_err());if !progress{break;}}assert_eq!(returned,backing);assert_eq!(scratch.allocated_bytes(),0);
 let mut retired=0;for _ in 0..1000{let(step,requested,released)=crate::test_allocation::observe_backing(||parent.close_step(1,4096));assert_eq!(requested,0);match step{AllocationReturnStep::Complete=>{assert_eq!(released,0);break;},AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released<=4096);assert_eq!(released,released_bytes);retired+=released;}}}assert!(parent.terminal_is_empty());assert_eq!(retired,backing);
 for(row,text)in rows.iter().zip(&source.texts){assert_eq!(text.len(),row["text"].as_str().unwrap().len()*row["repeat"].as_u64().unwrap()as usize);}
 eprintln!("[DEBUG] actual borrowed symbols occurrences10 selected7 first-source pointers unchanged; fixed4096 refusal retains paid scratch; one-item4096 physical retirement exact");
}
