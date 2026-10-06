    /// 🧩️ A unique retained child source whose borrowed publications preserve every original operation octet.
    #[derive(Debug,PartialEq)]
    pub struct ChildEmit {
        pub owner:String,
        pub slot:String,
        pub child_id:String,
        pub ops:semio_framework_value::list::PagedList<::protocol::operation_bytes::OwnedOperationBytes,{isize::MAX as usize}>,
        pub op_schema:SchemaId,
        pub labels:semio_framework_value::list::PagedList<LocalizedLabel,{isize::MAX as usize}>,
        partial:Option<::protocol::operation_bytes::OwnedOperationBytes>,
        partial_label:Option<LocalizedLabel>,
    }
    impl ChildEmit {
        pub(crate) fn empty()->Self{Self{owner:String::new(),slot:String::new(),child_id:String::new(),ops:semio_framework_value::list::PagedList::empty(),op_schema:SchemaId(String::new()),labels:semio_framework_value::list::PagedList::empty(),partial:None,partial_label:None}}
        pub fn open(slot:impl Into<String>,child_id:impl Into<String>,_capacity:usize)->Self{
            Self{owner:String::new(),slot:slot.into(),child_id:child_id.into(),ops:semio_framework_value::list::PagedList::empty(),op_schema:SchemaId("child.empty".into()),labels:semio_framework_value::list::PagedList::empty(),partial:None,partial_label:None}
        }
        pub fn sources(&self)->&dyn ::protocol::operation_bytes::OperationSourceCollection{&self.ops}
        pub(crate) fn close_one(&mut self,maximum_items:usize,maximum_bytes:usize)->PluginCloseStep{
            use ::protocol::operation_bytes::OperationByteCloseStep;
            if maximum_items==0||maximum_bytes==0{return PluginCloseStep::Pending{released_items:0,released_bytes:0};}
            let index=self.ops.len().checked_sub(1);
            let source=match self.partial.as_mut(){Some(source)=>Some(source),None=>index.and_then(|index|self.ops.get_mut(index))};
            if let Some(source)=source{
                return match source.close_one(1,maximum_bytes){
                    Ok(OperationByteCloseStep::Pending{released_items,released_bytes})=>PluginCloseStep::Pending{released_items,released_bytes},
                    Ok(OperationByteCloseStep::Complete)=>{if self.partial.is_some(){self.partial.take();}else{self.ops.pop();}PluginCloseStep::Pending{released_items:1,released_bytes:0}},
                    Err(_)=>PluginCloseStep::Pending{released_items:0,released_bytes:0},
                };
            }
            if !self.ops.terminal_is_empty(){
                return match self.ops.release_empty_page(maximum_bytes){Ok(step)=>PluginCloseStep::Pending{released_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes},Err(_)=>PluginCloseStep::Pending{released_items:0,released_bytes:0}};
            }
            let index=self.labels.len().checked_sub(1);
            let label=match self.partial_label.as_mut(){Some(label)=>Some(label),None=>index.and_then(|index|self.labels.get_mut(index))};
            if let Some(label)=label{
                return match label.close_owned_cell_one(maximum_bytes){Ok(Some(released_bytes))=>PluginCloseStep::Pending{released_items:1,released_bytes},Ok(None)=>{if self.partial_label.is_some(){self.partial_label.take();}else{self.labels.pop();}PluginCloseStep::Pending{released_items:1,released_bytes:0}},Err(_)=>PluginCloseStep::Pending{released_items:0,released_bytes:0}};
            }
            if !self.labels.terminal_is_empty(){
                return match self.labels.release_empty_page(maximum_bytes){Ok(step)=>PluginCloseStep::Pending{released_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes},Err(_)=>PluginCloseStep::Pending{released_items:0,released_bytes:0}};
            }
            for text in [&mut self.op_schema.0,&mut self.child_id,&mut self.slot,&mut self.owner]{
                let bytes=text.capacity();if bytes==0{continue;}if bytes>maximum_bytes{return PluginCloseStep::Pending{released_items:0,released_bytes:0};}
                *text=String::new();return PluginCloseStep::Pending{released_items:1,released_bytes:bytes};
            }
            PluginCloseStep::Complete
        }
        fn return_one<const N:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<N>,maximum_items:usize,maximum_bytes:usize)->Result<PluginCloseStep,semio_framework_value::ValueError>{
            use ::protocol::operation_bytes::OperationByteReturnStep;
            let pending=|progressed|PluginCloseStep::Pending{released_items:usize::from(progressed),released_bytes:0};
            if maximum_items==0||maximum_bytes==0{return Ok(pending(false));}
            let index=self.ops.len().checked_sub(1);
            let source=match self.partial.as_mut(){Some(source)=>Some(source),None=>index.and_then(|index|self.ops.get_mut(index))};
            if let Some(source)=source{
                return match source.return_one(parent,1,maximum_bytes)?{OperationByteReturnStep::Pending{returned_items,..}=>Ok(pending(returned_items!=0)),OperationByteReturnStep::Complete=>{if self.partial.is_some(){self.partial.take();}else{self.ops.pop();}Ok(pending(true))}};
            }
            if !self.ops.terminal_is_empty(){return self.ops.return_empty_page(parent,1).map(|step|pending(step.progressed));}
            let index=self.labels.len().checked_sub(1);
            let label=match self.partial_label.as_mut(){Some(label)=>Some(label),None=>index.and_then(|index|self.labels.get_mut(index))};
            if let Some(label)=label{return match label.return_owned_cell_one(parent,1)?{Some(progressed)=>Ok(pending(progressed)),None=>{if self.partial_label.is_some(){self.partial_label.take();}else{self.labels.pop();}Ok(pending(true))}};}
            if !self.labels.terminal_is_empty(){return self.labels.return_empty_page(parent,1).map(|step|pending(step.progressed));}
            for text in [&mut self.op_schema.0,&mut self.child_id,&mut self.slot,&mut self.owner]{if text.capacity()!=0{return parent.return_text(text,1).map(pending);}}
            Ok(PluginCloseStep::Complete)
        }
        pub(crate) fn next_close_byte_demand(&self)->usize{
            let source=self.partial.as_ref().or_else(||self.ops.len().checked_sub(1).and_then(|index|self.ops.get(index)));
            if let Some(source)=source{return source.next_close_byte_demand().unwrap_or(1);}
            if !self.ops.terminal_is_empty(){return self.ops.next_release_allocation_bytes().unwrap_or(1);}
            let label=self.partial_label.as_ref().or_else(||self.labels.len().checked_sub(1).and_then(|index|self.labels.get(index)));
            if let Some(label)=label{return label.next_owned_close_byte_demand().max(1);}
            if !self.labels.terminal_is_empty(){return self.labels.next_release_allocation_bytes().unwrap_or(1);}
            [&self.op_schema.0,&self.child_id,&self.slot,&self.owner].into_iter().find_map(|text|(text.capacity()!=0).then_some(text.capacity())).unwrap_or(0)
        }
        pub fn push<S,M>(&mut self,operation:&M,options:&::protocol::codec::PackEncodeOptions,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Option<SchemaId>,::protocol::ProtocolError>
        where M:protocol::SemanticMutation<S>+::protocol::OpBinary{
            use ::protocol::operation_bytes::OwnedOperationBytes;
            if self.partial.is_some(){return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"refused child prefix retains its original partial source").into());}
            let maximum_payload=usize::try_from(options.limits.max_file_len).map_err(|_|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit,"child operation source exceeds addressable authority"))?;
            let maximum_alloc=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX).min(control.maximum_bytes());
            let source=OwnedOperationBytes::try_new(maximum_payload,maximum_alloc).map_err(|error|semio_framework_value::ValueError::new(error.kind,error.reason))?;
            self.partial=Some(source);
            ::protocol::OpBinary::encode_op_into(operation,options,self.partial.as_mut().expect("retained source"),control)?;
            while !self.ops.has_reserved_slot(){
                let required=self.ops.next_allocation_bytes().map_err(semio_framework_value::ValueError::from)?;
                if required>4096{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit,"child operation collection page exceeds production grant").into());}
                control.charge(required)?;
                let step=self.ops.reserve_one(4096).map_err(|error|semio_framework_value::ValueError::from(error.refusal()))?;
                if !step.progressed{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"child operation collection admitted no backing").into());}
            }
            while !self.labels.has_reserved_slot(){
                let required=self.labels.next_allocation_bytes().map_err(semio_framework_value::ValueError::from)?;
                if required>4096{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit,"child label collection page exceeds production grant").into());}
                control.charge(required)?;
                let step=self.labels.reserve_one(4096).map_err(|error|semio_framework_value::ValueError::from(error.refusal()))?;
                if !step.progressed{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"child label collection admitted no backing").into());}
            }
            self.partial_label=Some(protocol::SemanticMutation::label(operation));
            let retired_schema=if self.ops.is_empty(){let semantics=protocol::SemanticMutation::semantics(operation);Some(std::mem::replace(&mut self.op_schema,SchemaId(format!("{}.{}",semantics.entity,semantics.kind))))}else{None};
            let source=self.partial.take().expect("successful source retained");
            if let Err(source)=self.ops.push_reserved(source){self.partial=Some(source);return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"child operation collection lost admitted slot").into());}
            let label=self.partial_label.take().expect("retained semantic label");
            if let Err(label)=self.labels.push_reserved(label){self.partial_label=Some(label);return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"child label collection lost admitted slot").into());}
            Ok(retired_schema)
        }
    }
