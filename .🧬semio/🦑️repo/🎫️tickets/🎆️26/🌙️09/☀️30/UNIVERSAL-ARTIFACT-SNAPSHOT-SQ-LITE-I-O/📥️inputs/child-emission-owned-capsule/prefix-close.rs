        /// ♻️ Returns one complete physical allocation under its exact byte grant.
        pub(crate) fn close_one(&mut self, maximum_items:usize, maximum_bytes:usize)->PluginCloseStep{
            if maximum_items==0||maximum_bytes==0{return PluginCloseStep::Pending{released_items:0,released_bytes:0};}
            if let Some(op)=self.ops.last(){
                let bytes=op.capacity();
                if bytes>maximum_bytes{return PluginCloseStep::Pending{released_items:0,released_bytes:0};}
                self.ops.pop();
                return PluginCloseStep::Pending{released_items:1,released_bytes:bytes};
            }
            if self.ops.capacity()!=0{
                let bytes=self.ops.capacity().checked_mul(std::mem::size_of::<Vec<u8>>()).expect("allocated operation vector has a representable layout");
                if bytes>maximum_bytes{return PluginCloseStep::Pending{released_items:0,released_bytes:0};}
                self.ops=Vec::new();
                return PluginCloseStep::Pending{released_items:1,released_bytes:bytes};
            }
            if let Some(label)=self.labels.last_mut(){
                return match label.close_owned_cell_one(maximum_bytes){
                    Ok(Some(released_bytes))=>PluginCloseStep::Pending{released_items:1,released_bytes},
                    Ok(None)=>{self.labels.pop();PluginCloseStep::Pending{released_items:1,released_bytes:0}},
                    Err(_)=>PluginCloseStep::Pending{released_items:0,released_bytes:0},
                };
            }
            if self.labels.capacity()!=0{
                let bytes=self.labels.capacity().checked_mul(std::mem::size_of::<LocalizedLabel>()).expect("allocated label vector has a representable layout");
                if bytes>maximum_bytes{return PluginCloseStep::Pending{released_items:0,released_bytes:0};}
                self.labels=Vec::new();
                return PluginCloseStep::Pending{released_items:1,released_bytes:bytes};
            }
            for value in [&mut self.op_schema.0,&mut self.child_id,&mut self.slot]{
                let bytes=value.capacity();
                if bytes==0{continue;}
                if bytes>maximum_bytes{return PluginCloseStep::Pending{released_items:0,released_bytes:0};}
                *value=String::new();
                return PluginCloseStep::Pending{released_items:1,released_bytes:bytes};
            }
            PluginCloseStep::Complete
        }

        /// 📏️ A caller reserves the first complete allocation before requesting its return.
        pub(crate) fn next_close_byte_demand(&self)->usize{
            if let Some(op)=self.ops.last(){return op.capacity().max(1);}
            if self.ops.capacity()!=0{return self.ops.capacity().checked_mul(std::mem::size_of::<Vec<u8>>()).expect("operation vector layout");}
            if let Some(label)=self.labels.last(){return label.next_owned_close_byte_demand().max(1);}
            if self.labels.capacity()!=0{return self.labels.capacity().checked_mul(std::mem::size_of::<LocalizedLabel>()).expect("label vector layout");}
            [&self.op_schema.0,&self.child_id,&self.slot].into_iter().find_map(|value|(value.capacity()!=0).then_some(value.capacity())).unwrap_or(0)
        }
