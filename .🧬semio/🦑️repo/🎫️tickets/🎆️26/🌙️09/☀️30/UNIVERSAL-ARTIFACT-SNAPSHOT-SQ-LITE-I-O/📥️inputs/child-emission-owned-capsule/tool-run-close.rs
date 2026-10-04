            if everything{
                if maximum_items==0||maximum_bytes==0{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                member.retired_emits.extend(member.emit.take());
                member.retired_emission_owners.extend(member.emission_owner.take());
                if !member.ops.is_empty()||member.ops.capacity()!=0{member.retired_ops.push_back(std::mem::take(&mut member.ops));}
            }
            if let Some(owner)=member.retired_emission_owners.front_mut(){
                if maximum_items==0||maximum_bytes==0{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                if owner.is_empty(){
                    let bytes=std::mem::size_of_val(owner.as_ref());
                    if bytes>maximum_bytes{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                    member.retired_emission_owners.pop_front();
                    return Ok(Some(PluginCloseStep::Pending{released_items:1,released_bytes:bytes}));
                }
                let step=match children.get(&(member.slot.clone(),member.child_id.clone())){
                    Some(child)=>child.member.visit_member(ToolRunMemberEmissionRetire{owner:owner.as_mut(),maximum_bytes})?,
                    None=>owner.close_step(1,maximum_bytes)?,
                };
                return Ok(Some(step));
            }
            if let Some(emit)=member.retired_emits.front_mut(){
                let step=emit.close_one(maximum_items.min(1),maximum_bytes);
                if step==PluginCloseStep::Complete{member.retired_emits.pop_front();return Ok(Some(PluginCloseStep::Pending{released_items:1,released_bytes:0}));}
                return Ok(Some(step));
            }
            if let Some(ops)=member.retired_ops.front_mut(){
                if maximum_items==0||maximum_bytes==0{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                if let Some(operation)=ops.last(){
                    let bytes=operation.capacity();
                    if bytes>maximum_bytes{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                    ops.pop();return Ok(Some(PluginCloseStep::Pending{released_items:1,released_bytes:bytes}));
                }
                let bytes=ops.capacity().checked_mul(std::mem::size_of::<Vec<u8>>()).expect("allocated member operation layout");
                if bytes>maximum_bytes{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                *ops=Vec::new();member.retired_ops.pop_front();return Ok(Some(PluginCloseStep::Pending{released_items:1,released_bytes:bytes}));
            }
            macro_rules! close_member_backing{
                ($field:ident,$item:ty)=>{
                    if member.$field.capacity()!=0{
                        let bytes=member.$field.capacity().checked_mul(std::mem::size_of::<$item>()).expect("allocated member retirement queue layout");
                        if maximum_items==0||bytes>maximum_bytes{return Ok(Some(PluginCloseStep::Pending{released_items:0,released_bytes:0}));}
                        member.$field=std::collections::VecDeque::new();return Ok(Some(PluginCloseStep::Pending{released_items:1,released_bytes:bytes}));
                    }
                };
            }
            close_member_backing!(retired_emission_owners,Box<dyn ToolRunMemberEmissionOwner>);
            close_member_backing!(retired_emits,ChildEmit);
            close_member_backing!(retired_ops,Vec<Vec<u8>>);
