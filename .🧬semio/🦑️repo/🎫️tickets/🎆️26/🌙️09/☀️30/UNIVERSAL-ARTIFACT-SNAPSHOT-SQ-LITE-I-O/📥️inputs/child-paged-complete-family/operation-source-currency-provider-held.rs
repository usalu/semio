impl crate::value::ErasedSnapshotRetirement for OwnedOperationBytes{
    fn close_step(&mut self,grant:crate::value::retained_clone::RetainedCloneGrant)->Result<crate::value::retained_clone::RetainedCloneStep,crate::value::ValueError>{
        use crate::value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        let empty=RetainedCloneProgress::default();if self.closed{return Ok(RetainedCloneStep::Complete(empty))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}if grant.maximum_depth<1{return Err(crate::value::ValueError::literal(ValueRefusalKind::DepthLimit,"original operation source retirement requires admitted depth"))}
        let logical=self.len()!=0;
        if logical&&grant.maximum_copy_bytes==0{return Ok(RetainedCloneStep::Progress(empty))}
        let release=if logical||self.allocated_bytes()==0{0}else{self.next_close_byte_demand().map_err(|fault|crate::value::ValueError::new(fault.kind,fault.reason))?};
        if release>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        let step=self.close_one(1,if logical{grant.maximum_copy_bytes}else if release!=0{grant.maximum_release_bytes}else{1}).map_err(|fault|crate::value::ValueError::new(fault.kind,fault.reason))?;
        match step{OperationByteCloseStep::Complete=>Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..empty})),OperationByteCloseStep::Pending{released_items,released_bytes}=>Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:released_items,copied_bytes:usize::from(logical&&released_items!=0),released_bytes,..empty}))}
    }
    fn terminal_is_empty(&self)->bool{self.closed}
    fn next_copy_byte_demand(&self)->Result<usize,crate::value::ValueError>{Ok(usize::from(!self.closed&&self.len()!=0))}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,crate::value::ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,crate::value::ValueError>{if self.closed||self.len()!=0||self.allocated_bytes()==0{Ok(0)}else{self.next_close_byte_demand().map_err(|fault|crate::value::ValueError::new(fault.kind,fault.reason))}}
    fn next_depth_demand(&self)->Result<usize,crate::value::ValueError>{Ok(usize::from(!self.closed))}
}
