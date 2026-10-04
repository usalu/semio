        /// ♻️ Keeps each preparation and prefix owned until its exact next allocation is returned.
        pub fn close_child_one(&mut self,maximum_items:usize,maximum_bytes:usize)->Option<PluginCloseStep>{
            if let Some(preparation)=self.child_preparations.front_mut(){
                if maximum_items==0||maximum_bytes==0{return Some(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
                let bytes=preparation.owner_cell_bytes();
                if preparation.terminal_is_empty(){
                    if bytes>maximum_bytes{return Some(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
                    self.child_preparations.pop_front();
                    return Some(PluginCloseStep::Pending{released_items:1,released_bytes:bytes});
                }
                preparation.begin_close();
                return Some(match preparation.close_step(maximum_items.min(1),maximum_bytes){
                    Ok(PluginCloseStep::Complete)=>PluginCloseStep::Pending{released_items:0,released_bytes:0},
                    Ok(step)=>step,
                    Err(_)=>PluginCloseStep::Blocked{reason:"child emission retains its exact typed retirement refusal"},
                });
            }
            if self.child_preparations.capacity()!=0{
                let bytes=self.child_preparations.capacity().checked_mul(std::mem::size_of::<ChildEmitPreparation>()).expect("allocated preparation queue layout");
                if maximum_items==0||bytes>maximum_bytes{return Some(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
                self.child_preparations=std::collections::VecDeque::new();
                return Some(PluginCloseStep::Pending{released_items:1,released_bytes:bytes});
            }
            if let Some(child)=self.child_emits.last_mut(){
                let step=child.close_one(maximum_items,maximum_bytes);
                if step==PluginCloseStep::Complete{
                    self.child_emits.pop();
                    return Some(PluginCloseStep::Pending{released_items:1,released_bytes:0});
                }
                return Some(step);
            }
            if self.child_emits.capacity()!=0{
                let bytes=self.child_emits.capacity().checked_mul(std::mem::size_of::<ChildEmit>()).expect("allocated child output queue layout");
                if maximum_items==0||bytes>maximum_bytes{return Some(PluginCloseStep::Pending{released_items:0,released_bytes:0});}
                self.child_emits=Vec::new();
                return Some(PluginCloseStep::Pending{released_items:1,released_bytes:bytes});
            }
            None
        }

