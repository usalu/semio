from ph import *
E("semio-framework-value-edit","📇️index/🦀️.rs",'''fn added_index(&self,_:usize)->crate::RetainedWorkOutcome<usize>{unreachable!("key admission declares no added coordinates")}''','''fn added_index(&self,_:usize)->crate::RetainedWorkOutcome<usize>{crate::RetainedWorkOutcome{result:Err(invalid()),work:0}}''')
E("semio-framework-value-paged-list","🪢️concat/🦀️.rs",'self.chunks.push_reserved(original).unwrap_or_else(|_|unreachable!("paid chunk row remains exclusively reserved"));','self.chunks.push_reserved(original).map_err(|_|ValueError::invariant("paid chunk row remains exclusively reserved"))?;')
a="semio-framework-value-allocation-return"
E(a,"🔗️shared/🦀️.rs",'std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<T>()).expect("shared allocation layout").0.pad_to_align().size()}','std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<T>()).map_or(usize::MAX,|(layout,_)|layout.pad_to_align().size())}')
E(a,"🔗️shared/🦀️.rs",'Arc::get_mut(&mut storage).expect("new shared storage is unique").write(value);','let Some(slot)=Arc::get_mut(&mut storage) else{return RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::AllocationFailed,"new shared storage is not unique"),value)),work:1}};slot.write(value);')
if __name__=="__main__":build()
